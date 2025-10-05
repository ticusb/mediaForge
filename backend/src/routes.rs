use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{auth, db, error::{AppError, Result}, AppState};

// ============================================================================
// Health Check
// ============================================================================

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "service": "MediaForge API"
    }))
}

// ============================================================================
// Authentication Routes
// ============================================================================

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<auth::RegisterRequest>,
) -> Result<Json<auth::AuthResponse>> {
    // Validate email format
    if !payload.email.contains('@') || payload.email.len() < 5 {
        return Err(AppError::BadRequest(
            "Invalid email format".to_string(),
        ));
    }

    // Validate password strength
    if payload.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Check if user exists
    if db::User::find_by_email(&state.db, &payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict(
            "Email already registered".to_string(),
        ));
    }

    // Hash password
    let password_hash = auth::hash_password(&payload.password)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?;

    // Create user (default to free tier)
    let user = db::User::create(&state.db, &payload.email, &password_hash, "free").await?;

    // Generate JWT
    let claims = auth::Claims::new(user.id, user.email.clone(), user.subscription_tier.clone());
    let token = claims
        .to_token(&state.config.jwt_secret)
        .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))?;

    tracing::info!("User registered: {} ({})", user.email, user.id);

    Ok(Json(auth::AuthResponse {
        token,
        user: auth::UserInfo {
            id: user.id.to_string(),
            email: user.email,
            tier: user.subscription_tier,
        },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<auth::LoginRequest>,
) -> Result<Json<auth::AuthResponse>> {
    // Find user
    let user = db::User::find_by_email(&state.db, &payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    let valid = auth::verify_password(&payload.password, &user.password_hash)
        .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Generate JWT
    let claims = auth::Claims::new(user.id, user.email.clone(), user.subscription_tier.clone());
    let token = claims
        .to_token(&state.config.jwt_secret)
        .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))?;

    tracing::info!("User logged in: {} ({})", user.email, user.id);

    Ok(Json(auth::AuthResponse {
        token,
        user: auth::UserInfo {
            id: user.id.to_string(),
            email: user.email,
            tier: user.subscription_tier,
        },
    }))
}

// ============================================================================
// Upload Route
// ============================================================================

#[derive(Serialize)]
pub struct UploadResponse {
    pub asset_id: String,
    pub filename: String,
    pub size: u64,
    pub location: String,
}

pub async fn upload(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid multipart data: {}", e)))?
    {
        if let Some(file_name) = field.file_name() {
            let file_name_owned = file_name.to_string();

            // Read file data
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::BadRequest(format!("Failed to read file: {}", e)))?;

            // Validate file
            validate_file(&file_name_owned, &data, &state.config)?;

            // Save to storage
            let location = state
                .storage
                .save_bytes(&data, &file_name_owned)
                .map_err(|e| AppError::Internal(format!("Failed to save file: {:?}", e)))?;

            // Create media asset record
            let asset = db::MediaAsset::create(
                &state.db,
                auth_user.id,
                &file_name_owned,
                &get_file_extension(&file_name_owned),
                data.len() as i64,
            )
            .await?;

            // Update asset with storage location
            db::MediaAsset::update_status(&state.db, asset.id, "uploaded", Some(&location))
                .await?;

            tracing::info!(
                "File uploaded: {} by user {} (asset: {})",
                file_name_owned,
                auth_user.email,
                asset.id
            );

            return Ok(Json(UploadResponse {
                asset_id: asset.id.to_string(),
                filename: file_name_owned,
                size: data.len() as u64,
                location,
            }));
        }
    }

    Err(AppError::BadRequest("No file provided".to_string()))
}

// ============================================================================
// Processing Routes
// ============================================================================

#[derive(Deserialize)]
pub struct ConvertRequest {
    pub asset_id: String,
    pub output_format: String,
    #[serde(default)]
    pub lut_location: Option<String>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
}

#[derive(Serialize)]
pub struct JobResponse {
    pub job_id: String,
    pub status: String,
}

pub async fn convert(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<ConvertRequest>,
) -> Result<Json<JobResponse>> {
    let asset_id = Uuid::parse_str(&payload.asset_id)
        .map_err(|_| AppError::BadRequest("Invalid asset ID".to_string()))?;

    // Verify asset ownership
    let asset = verify_asset_ownership(&state.db, asset_id, auth_user.id).await?;

    // Check quota
    check_quota(&state, &auth_user, "image").await?;

    // Create job
    let job = db::Job::create(
        &state.db,
        auth_user.id,
        vec![asset_id],
        "convert",
        json!({
            "output_format": payload.output_format,
            "lut_location": payload.lut_location,
            "width": payload.width,
            "height": payload.height,
        }),
        if auth_user.tier == "pro" { 10 } else { 0 },
    )
    .await?;

    // Enqueue job
    state
        .queue
        .enqueue(crate::services::JobMessage {
            job_id: job.id.to_string(),
            user_id: auth_user.id.to_string(),
            job_type: "convert".to_string(),
            media_location: asset.result_location.unwrap_or_default(),
        })
        .await
        .map_err(|_| AppError::ServiceUnavailable("Queue is full".to_string()))?;

    tracing::info!(
        "Conversion job {} queued for user {}",
        job.id,
        auth_user.email
    );

    Ok(Json(JobResponse {
        job_id: job.id.to_string(),
        status: "queued".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct RemoveBgRequest {
    pub asset_id: String,
    #[serde(default)]
    pub replace_color: Option<[u8; 3]>,
}

pub async fn remove_bg(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<RemoveBgRequest>,
) -> Result<Json<JobResponse>> {
    let asset_id = Uuid::parse_str(&payload.asset_id)
        .map_err(|_| AppError::BadRequest("Invalid asset ID".to_string()))?;

    let asset = verify_asset_ownership(&state.db, asset_id, auth_user.id).await?;

    // Check quota for video processing
    check_quota(&state, &auth_user, "video").await?;

    let job = db::Job::create(
        &state.db,
        auth_user.id,
        vec![asset_id],
        "remove_bg",
        json!({
            "replace_color": payload.replace_color,
        }),
        if auth_user.tier == "pro" { 10 } else { 0 },
    )
    .await?;

    state
        .queue
        .enqueue(crate::services::JobMessage {
            job_id: job.id.to_string(),
            user_id: auth_user.id.to_string(),
            job_type: "remove_bg".to_string(),
            media_location: asset.result_location.unwrap_or_default(),
        })
        .await
        .map_err(|_| AppError::ServiceUnavailable("Queue is full".to_string()))?;

    tracing::info!(
        "Background removal job {} queued for user {}",
        job.id,
        auth_user.email
    );

    Ok(Json(JobResponse {
        job_id: job.id.to_string(),
        status: "queued".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct ColorGradeRequest {
    pub asset_id: String,
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub lut_location: Option<String>,
    #[serde(default)]
    pub hue: Option<i32>,
    #[serde(default)]
    pub saturation: Option<i32>,
    #[serde(default)]
    pub brightness: Option<i32>,
    #[serde(default)]
    pub contrast: Option<i32>,
}

pub async fn color_grade(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<ColorGradeRequest>,
) -> Result<Json<JobResponse>> {
    let asset_id = Uuid::parse_str(&payload.asset_id)
        .map_err(|_| AppError::BadRequest("Invalid asset ID".to_string()))?;

    let asset = verify_asset_ownership(&state.db, asset_id, auth_user.id).await?;

    let job = db::Job::create(
        &state.db,
        auth_user.id,
        vec![asset_id],
        "color_grade",
        json!({
            "preset": payload.preset,
            "lut_location": payload.lut_location,
            "hue": payload.hue,
            "saturation": payload.saturation,
            "brightness": payload.brightness,
            "contrast": payload.contrast,
        }),
        if auth_user.tier == "pro" { 10 } else { 0 },
    )
    .await?;

    state
        .queue
        .enqueue(crate::services::JobMessage {
            job_id: job.id.to_string(),
            user_id: auth_user.id.to_string(),
            job_type: "color_grade".to_string(),
            media_location: asset.result_location.unwrap_or_default(),
        })
        .await
        .map_err(|_| AppError::ServiceUnavailable("Queue is full".to_string()))?;

    tracing::info!(
        "Color grading job {} queued for user {}",
        job.id,
        auth_user.email
    );

    Ok(Json(JobResponse {
        job_id: job.id.to_string(),
        status: "queued".to_string(),
    }))
}

// LUT upload endpoint: Accepts a single .cube file (<= configured size) and
// returns a temporary location or registers it for user. Minimal validation here.
pub async fn upload_lut(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid multipart data: {}", e)))?
    {
        if let Some(file_name_ref) = field.file_name() {
            let file_name = file_name_ref.to_string();
            let lower = file_name.to_lowercase();
            if !lower.ends_with(".cube") {
                return Err(AppError::BadRequest("Only .cube LUT files are supported".to_string()));
            }

            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::BadRequest(format!("Failed to read file: {}", e)))?;

            let max_bytes = state.config.processing.lut_max_size_mb * 1024 * 1024;
            if data.len() as u64 > max_bytes {
                return Err(AppError::PayloadTooLarge(format!(
                    "LUT file too large: {} MB (max {} MB)",
                    data.len() as u64 / (1024 * 1024),
                    max_bytes / (1024 * 1024)
                )));
            }

            // Save LUT to storage (using same storage adapter)
            let location = state
                .storage
                .save_bytes(&data, &file_name)
                .map_err(|e| AppError::Internal(format!("Failed to save LUT: {:?}", e)))?;

            tracing::info!("User {} uploaded LUT {}", auth_user.email, file_name);

            return Ok(Json(json!({"location": location})))
        }
    }

    Err(AppError::BadRequest("No LUT file provided".to_string()))
}

// ============================================================================
// Job Status Routes
// ============================================================================

#[derive(Serialize)]
pub struct JobStatusResponse {
    pub job_id: String,
    pub status: String,
    pub progress: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_url: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

pub async fn get_job_status(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<JobStatusResponse>> {
    let job_uuid = Uuid::parse_str(&job_id)
        .map_err(|_| AppError::BadRequest("Invalid job ID".to_string()))?;

    let job = db::Job::find_by_id(&state.db, job_uuid)
        .await?
        .ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;

    // Verify ownership
    if job.user_id != auth_user.id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    Ok(Json(JobStatusResponse {
        job_id: job.id.to_string(),
        status: job.status,
        progress: job.progress_percent as u32,
        result_url: job.result_location,
        created_at: job.created_at.to_rfc3339(),
        completed_at: job.completed_at.map(|t| t.to_rfc3339()),
    }))
}

pub async fn list_user_jobs(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<JobStatusResponse>>> {
    let jobs = sqlx::query_as::<_, db::Job>(
        "SELECT * FROM jobs WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50",
    )
    .bind(auth_user.id)
    .fetch_all(&state.db)
    .await?;

    let response: Vec<JobStatusResponse> = jobs
        .into_iter()
        .map(|job| JobStatusResponse {
            job_id: job.id.to_string(),
            status: job.status,
            progress: job.progress_percent as u32,
            result_url: job.result_location,
            created_at: job.created_at.to_rfc3339(),
            completed_at: job.completed_at.map(|t| t.to_rfc3339()),
        })
        .collect();

    Ok(Json(response))
}

pub async fn download_result(
    auth_user: auth::AuthUser,
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<impl axum::response::IntoResponse> {
    let job_uuid = Uuid::parse_str(&job_id)
        .map_err(|_| AppError::BadRequest("Invalid job ID".to_string()))?;

    let job = db::Job::find_by_id(&state.db, job_uuid)
        .await?
        .ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;

    // Verify ownership
    if job.user_id != auth_user.id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    if job.status != "completed" {
        return Err(AppError::BadRequest("Job not completed".to_string()));
    }

    let result_location = job
        .result_location
        .ok_or_else(|| AppError::NotFound("Result not found".to_string()))?;

    // Read file from storage
    let file_data = tokio::fs::read(&result_location)
        .await
        .map_err(|_| AppError::NotFound("File not found".to_string()))?;

    // Determine content type from filename
    let content_type = get_content_type(&result_location);
    let filename = result_location
        .split('/')
        .last()
        .unwrap_or("result");

    let disposition = format!("attachment; filename=\"{}\"", filename);
    
    Ok((
        axum::http::StatusCode::OK,
        [
            ("Content-Type", content_type.to_string()),
            ("Content-Disposition", disposition),
        ],
        file_data,
    ))
}
// ============================================================================
// Helper Functions
// ============================================================================

async fn verify_asset_ownership(
    db: &sqlx::PgPool,
    asset_id: Uuid,
    user_id: Uuid,
) -> Result<db::MediaAsset> {
    let asset = sqlx::query_as::<_, db::MediaAsset>("SELECT * FROM media_assets WHERE id = $1")
        .bind(asset_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Asset not found".to_string()))?;

    if asset.user_id != user_id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    Ok(asset)
}

async fn check_quota(state: &AppState, user: &auth::AuthUser, job_type: &str) -> Result<()> {
    // Use quota service for logic
    match crate::services::quota::check_quota(&state.db, &state.config, user.id, &user.tier, job_type).await {
        Ok(_) => (),
        Err(e) => return Err(AppError::QuotaExceeded(format!("{} Upgrade to Pro for more capacity.", e))),
    }

    match crate::services::quota::check_concurrent(&state.db, &state.config, user.id, &user.tier).await {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::QuotaExceeded(format!("{} Try again later.", e))),
    }
}

fn validate_file(
    filename: &str,
    data: &[u8],
    config: &crate::config::Config,
) -> Result<()> {
    let lower = filename.to_lowercase();
    let size = data.len() as u64;

    let is_image = lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".png")
        || lower.ends_with(".webp")
        || lower.ends_with(".gif")
        || lower.ends_with(".heic");

    let is_video = lower.ends_with(".mp4") 
        || lower.ends_with(".mov") 
        || lower.ends_with(".avi")
        || lower.ends_with(".webm");

    if !is_image && !is_video {
        return Err(AppError::BadRequest(
            "Unsupported file type. Supported: JPG, PNG, WEBP, GIF, HEIC, MP4, MOV, AVI, WEBM"
                .to_string(),
        ));
    }

    let max_size_bytes = if is_image {
        config.processing.max_image_size_mb * 1024 * 1024
    } else {
        config.processing.max_video_size_mb * 1024 * 1024
    };

    if size > max_size_bytes {
        return Err(AppError::PayloadTooLarge(format!(
            "File too large: {} MB (max {} MB)",
            size / (1024 * 1024),
            max_size_bytes / (1024 * 1024)
        )));
    }

    Ok(())
}

fn get_file_extension(filename: &str) -> String {
    filename
        .rsplit('.')
        .next()
        .unwrap_or("unknown")
        .to_lowercase()
}

fn get_content_type(filename: &str) -> &'static str {
    let lower = filename.to_lowercase();
    
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".mp4") {
        "video/mp4"
    } else if lower.ends_with(".mov") {
        "video/quicktime"
    } else if lower.ends_with(".avi") {
        "video/x-msvideo"
    } else if lower.ends_with(".webm") {
        "video/webm"
    } else {
        "application/octet-stream"
    }
}