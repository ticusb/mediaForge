mod services;

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, Multipart},
    http::StatusCode,
    Extension,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use services::{LocalStorage, Storage, Queue, JobMessage, start_worker_with_status};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Instantiate local storage (development default)
    let storage = LocalStorage::new("./data/uploads");
    let shared: Arc<dyn Storage> = Arc::new(storage);

    // Instantiate in-memory queue and worker
    let (queue, rx) = Queue::new(100);
    let queue_arc = Arc::new(queue);
    
    // Start worker with status tracking
    let statuses = queue_arc.get_statuses_handle();
    start_worker_with_status(rx, shared.clone(), statuses);

    let app = Router::new()
        .route("/api/upload", post(upload_handler))
        .route("/api/convert", post(convert_handler))
        .route("/api/status/:jobId", get(status_handler))
        .layer(Extension(shared))
        .layer(Extension(queue_arc))
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([
                    hyper::Method::GET,
                    hyper::Method::POST,
                    hyper::Method::OPTIONS,
                ])
                .allow_headers(tower_http::cors::Any),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to port 8080");
    
    tracing::info!("Server listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

// Custom error type for better error handling
#[derive(Debug)]
enum AppError {
    FileTooLarge(String),
    InvalidFileType,
    NoFileProvided,
    StorageError,
    QueueFull,
    InvalidMultipart,
    ReadError,
    JobNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::FileTooLarge(msg) => (StatusCode::PAYLOAD_TOO_LARGE, msg),
            AppError::InvalidFileType => (
                StatusCode::BAD_REQUEST,
                "Unsupported file type. Supported: JPG, PNG, WEBP, GIF, HEIC, MP4, MOV, AVI".to_string(),
            ),
            AppError::NoFileProvided => (StatusCode::BAD_REQUEST, "No file provided".to_string()),
            AppError::StorageError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save file to storage".to_string(),
            ),
            AppError::QueueFull => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Processing queue is full, please try again later".to_string(),
            ),
            AppError::InvalidMultipart => (
                StatusCode::BAD_REQUEST,
                "Invalid multipart data".to_string(),
            ),
            AppError::ReadError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read file data".to_string(),
            ),
            AppError::JobNotFound => (StatusCode::NOT_FOUND, "Job not found".to_string()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

// Response types
#[derive(Serialize)]
struct UploadResponse {
    #[serde(rename = "jobId")]
    job_id: String,
    status: String,
    location: String,
}

#[derive(Serialize)]
struct ConvertResponse {
    #[serde(rename = "jobId")]
    job_id: String,
    status: String,
}

#[derive(Serialize)]
struct StatusResponse {
    #[serde(rename = "jobId")]
    job_id: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

// File validation constants
const MAX_IMAGE_SIZE: u64 = 5 * 1024 * 1024; // 5MB
const MAX_VIDEO_SIZE: u64 = 50 * 1024 * 1024; // 50MB

async fn upload_handler(
    Extension(storage): Extension<Arc<dyn Storage>>,
    Extension(queue): Extension<Arc<Queue>>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::InvalidMultipart)?
    {
        if let Some(file_name) = field.file_name() {
            let file_name_owned = file_name.to_string();
            
            // Read file data
            let data = field.bytes().await.map_err(|_| AppError::ReadError)?;

            // Validate file type and size
            validate_file(&file_name_owned, &data)?;

            // Save to storage
            let location = storage
                .save_bytes(&data, &file_name_owned)
                .map_err(|_| AppError::StorageError)?;

            // Create job
            let job_id = Uuid::new_v4().to_string();
            let job_message = JobMessage {
                job_id: job_id.clone(),
                job_type: "remove_bg".to_string(),
                media_location: location.clone(),
            };

            // Enqueue job
            queue
                .enqueue(job_message)
                .await
                .map_err(|_| AppError::QueueFull)?;

            tracing::info!("File uploaded and job queued: {} (job_id: {})", file_name_owned, job_id);

            return Ok(Json(UploadResponse {
                job_id,
                status: "queued".to_string(),
                location,
            }));
        }
    }

    Err(AppError::NoFileProvided)
}

fn validate_file(file_name: &str, data: &Bytes) -> Result<(), AppError> {
    let lower = file_name.to_lowercase();
    let size = data.len() as u64;

    let is_image = lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".png")
        || lower.ends_with(".webp")
        || lower.ends_with(".gif")
        || lower.ends_with(".heic");

    let is_video = lower.ends_with(".mp4")
        || lower.ends_with(".mov")
        || lower.ends_with(".avi");

    if !is_image && !is_video {
        return Err(AppError::InvalidFileType);
    }

    if is_image && size > MAX_IMAGE_SIZE {
        return Err(AppError::FileTooLarge(format!(
            "Image too large: {} bytes (max {}MB)",
            size,
            MAX_IMAGE_SIZE / 1024 / 1024
        )));
    }

    if is_video && size > MAX_VIDEO_SIZE {
        return Err(AppError::FileTooLarge(format!(
            "Video too large: {} bytes (max {}MB)",
            size,
            MAX_VIDEO_SIZE / 1024 / 1024
        )));
    }

    Ok(())
}

async fn convert_handler(
    Extension(queue): Extension<Arc<Queue>>,
) -> Result<Json<ConvertResponse>, AppError> {
    let job_id = Uuid::new_v4().to_string();
    
    // Example: queue a conversion job
    let job_message = JobMessage {
        job_id: job_id.clone(),
        job_type: "convert".to_string(),
        media_location: "".to_string(), // Would need actual media location
    };

    queue
        .enqueue(job_message)
        .await
        .map_err(|_| AppError::QueueFull)?;

    tracing::info!("Convert job queued: {}", job_id);

    Ok(Json(ConvertResponse {
        job_id,
        status: "queued".to_string(),
    }))
}

async fn status_handler(
    Path(job_id): Path<String>,
    Extension(queue): Extension<Arc<Queue>>,
) -> Result<Json<StatusResponse>, AppError> {
    use services::JobStatus;
    
    // Get job status from queue
    let job_status = queue
        .get_status(&job_id)
        .await
        .ok_or(AppError::JobNotFound)?;

    let response = match job_status {
        JobStatus::Queued => StatusResponse {
            job_id,
            status: "queued".to_string(),
            progress: Some(0),
            result_url: None,
            error: None,
        },
        JobStatus::Processing { progress } => StatusResponse {
            job_id,
            status: "processing".to_string(),
            progress: Some(progress),
            result_url: None,
            error: None,
        },
        JobStatus::Completed { result_url } => StatusResponse {
            job_id,
            status: "completed".to_string(),
            progress: Some(100),
            result_url: Some(result_url),
            error: None,
        },
        JobStatus::Failed { error } => StatusResponse {
            job_id,
            status: "failed".to_string(),
            progress: None,
            result_url: None,
            error: Some(error),
        },
    };

    Ok(Json(response))
}