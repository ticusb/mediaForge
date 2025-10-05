// backend/src/services/worker.rs
// Background job worker with database integration

use tokio::sync::mpsc::Receiver;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{db, config};
use super::queue::{JobMessage, JobStatus};
use super::processing::ImageProcessor;
use super::Storage;

pub fn start_worker(
    mut rx: Receiver<JobMessage>,
    storage: Arc<dyn Storage>,
    db_pool: sqlx::PgPool,
    statuses: Arc<Mutex<HashMap<String, JobStatus>>>,
    config: config::Config,
) {
    tokio::spawn(async move {
        let processor = ImageProcessor::new(config.processing.model_path.clone())
            .expect("Failed to initialize image processor");

        tracing::info!("Worker started and ready to process jobs");

        while let Some(job) = rx.recv().await {
            tracing::info!("Worker processing job {} (type: {})", job.job_id, job.job_type);

            // Update status to processing
            {
                let mut s = statuses.lock().await;
                s.insert(job.job_id.clone(), JobStatus::Processing { progress: 0 });
            }

            let job_uuid = match Uuid::parse_str(&job.job_id) {
                Ok(id) => id,
                Err(e) => {
                    tracing::error!("Invalid job UUID {}: {}", job.job_id, e);
                    continue;
                }
            };

            // Update database
            if let Err(e) = db::Job::update_progress(&db_pool, job_uuid, "processing", 0).await {
                tracing::error!("Failed to update job status: {:?}", e);
            }

            // Process job based on type
            let result = match job.job_type.as_str() {
                "remove_bg" => {
                    process_background_removal(
                        &job,
                        &db_pool,
                        &storage,
                        &processor,
                        &statuses,
                    ).await
                }
                "convert" => {
                    process_conversion(
                        &job,
                        &db_pool,
                        &storage,
                        &processor,
                        &statuses,
                    ).await
                }
                "color_grade" => {
                    process_color_grade(
                        &job,
                        &db_pool,
                        &storage,
                        &processor,
                        &statuses,
                    ).await
                }
                _ => {
                    tracing::error!("Unknown job type: {}", job.job_type);
                    Err("Unknown job type".to_string())
                }
            };

            // Update final status
            match result {
                Ok(result_location) => {
                    let mut s = statuses.lock().await;
                    s.insert(
                        job.job_id.clone(),
                        JobStatus::Completed {
                            result_url: result_location.clone(),
                        },
                    );
                    drop(s);

                    if let Err(e) = db::Job::complete(&db_pool, job_uuid, &result_location).await {
                        tracing::error!("Failed to mark job as complete: {:?}", e);
                    }

                    tracing::info!("Job {} completed successfully", job.job_id);
                }
                Err(error) => {
                    let mut s = statuses.lock().await;
                    s.insert(
                        job.job_id.clone(),
                        JobStatus::Failed {
                            error: error.clone(),
                        },
                    );
                    drop(s);

                    if let Err(e) = db::Job::fail(&db_pool, job_uuid, &error).await {
                        tracing::error!("Failed to mark job as failed: {:?}", e);
                    }

                    tracing::error!("Job {} failed: {}", job.job_id, error);
                }
            }
        }

        tracing::info!("Worker exiting - channel closed");
    });
}

async fn process_background_removal(
    job: &JobMessage,
    db_pool: &sqlx::PgPool,
    storage: &Arc<dyn Storage>,
    processor: &ImageProcessor,
    statuses: &Arc<Mutex<HashMap<String, JobStatus>>>,
) -> Result<String, String> {
    // Get job details from database
    let job_uuid = Uuid::parse_str(&job.job_id).map_err(|e| e.to_string())?;
    let job_record = db::Job::find_by_id(db_pool, job_uuid)
        .await
        .map_err(|e| format!("Failed to fetch job: {:?}", e))?
        .ok_or("Job not found")?;

    // Get media asset IDs
    let asset_ids: Vec<String> = serde_json::from_value(job_record.media_asset_ids)
        .map_err(|e| format!("Invalid asset IDs: {}", e))?;

    if asset_ids.is_empty() {
        return Err("No assets in job".to_string());
    }

    let asset_id = Uuid::parse_str(&asset_ids[0]).map_err(|e| e.to_string())?;

    // Get asset location from database
    let asset = sqlx::query_as::<_, db::MediaAsset>(
        "SELECT * FROM media_assets WHERE id = $1"
    )
    .bind(asset_id)
    .fetch_optional(db_pool)
    .await
    .map_err(|e| format!("Failed to fetch asset: {:?}", e))?
    .ok_or("Asset not found")?;

    let input_path = std::path::PathBuf::from(&asset.result_location.unwrap_or(asset.original_filename.clone()));
    let output_filename = format!("processed_{}.png", job.job_id);
    let output_path = std::env::temp_dir().join(&output_filename);

    // Update progress
    update_progress(statuses, &job.job_id, 20).await;

    // Check if we should replace background
    let replace_color: Option<[u8; 3]> = job_record
        .parameters
        .get("replace_color")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    // Process image or video
    let lower = input_path.to_string_lossy().to_lowercase();
    let is_video = lower.ends_with(".mp4") || lower.ends_with(".mov") || lower.ends_with(".avi") || lower.ends_with(".webm");

    if is_video {
        // For MVP, extract first frame and remove background on it
        processor
            .remove_background_from_video(&input_path, &output_path)
            .map_err(|e| format!("Background removal failed (video): {:?}", e))?;
    } else {
        if let Some(color) = replace_color {
            processor
                .replace_background(&input_path, &output_path, color)
                .map_err(|e| format!("Background replacement failed: {:?}", e))?;
        } else {
            processor
                .remove_background(&input_path, &output_path)
                .map_err(|e| format!("Background removal failed: {:?}", e))?;
        }
    }

    update_progress(statuses, &job.job_id, 80).await;

    // Save result to storage
    let result_bytes = std::fs::read(&output_path)
        .map_err(|e| format!("Failed to read result: {}", e))?;

    let result_location = storage
        .save_bytes(&result_bytes, &output_filename)
        .map_err(|e| format!("Failed to save result: {:?}", e))?;

    // Cleanup temp file
    std::fs::remove_file(&output_path).ok();

    update_progress(statuses, &job.job_id, 100).await;

    Ok(result_location)
}

async fn process_conversion(
    job: &JobMessage,
    db_pool: &sqlx::PgPool,
    storage: &Arc<dyn Storage>,
    processor: &ImageProcessor,
    statuses: &Arc<Mutex<HashMap<String, JobStatus>>>,
) -> Result<String, String> {
    let job_uuid = Uuid::parse_str(&job.job_id).map_err(|e| e.to_string())?;
    let job_record = db::Job::find_by_id(db_pool, job_uuid)
        .await
        .map_err(|e| format!("Failed to fetch job: {:?}", e))?
        .ok_or("Job not found")?;

    let asset_ids: Vec<String> = serde_json::from_value(job_record.media_asset_ids)
        .map_err(|e| format!("Invalid asset IDs: {}", e))?;

    let asset_id = Uuid::parse_str(&asset_ids[0]).map_err(|e| e.to_string())?;

    let asset = sqlx::query_as::<_, db::MediaAsset>(
        "SELECT * FROM media_assets WHERE id = $1"
    )
    .bind(asset_id)
    .fetch_optional(db_pool)
    .await
    .map_err(|e| format!("Failed to fetch asset: {:?}", e))?
    .ok_or("Asset not found")?;

    let input_path = std::path::PathBuf::from(&asset.result_location.unwrap_or(asset.original_filename.clone()));

    // Get conversion parameters
    let output_format: String = job_record
        .parameters
        .get("output_format")
        .and_then(|v| v.as_str())
        .unwrap_or("png")
        .to_string();

    let width: Option<u32> = job_record
        .parameters
        .get("width")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

    let height: Option<u32> = job_record
        .parameters
        .get("height")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

    let output_filename = format!("converted_{}.{}", job.job_id, output_format);
    let output_path = std::env::temp_dir().join(&output_filename);

    update_progress(statuses, &job.job_id, 30).await;

    // Convert image
    processor
        .convert_format(&input_path, &output_path, width, height)
        .map_err(|e| format!("Conversion failed: {:?}", e))?;

    update_progress(statuses, &job.job_id, 80).await;

    // Save result
    let result_bytes = std::fs::read(&output_path)
        .map_err(|e| format!("Failed to read result: {}", e))?;

    let result_location = storage
        .save_bytes(&result_bytes, &output_filename)
        .map_err(|e| format!("Failed to save result: {:?}", e))?;

    std::fs::remove_file(&output_path).ok();

    update_progress(statuses, &job.job_id, 100).await;

    Ok(result_location)
}

async fn process_color_grade(
    job: &JobMessage,
    db_pool: &sqlx::PgPool,
    storage: &Arc<dyn Storage>,
    processor: &ImageProcessor,
    statuses: &Arc<Mutex<HashMap<String, JobStatus>>>,
) -> Result<String, String> {
    let job_uuid = Uuid::parse_str(&job.job_id).map_err(|e| e.to_string())?;
    let job_record = db::Job::find_by_id(db_pool, job_uuid)
        .await
        .map_err(|e| format!("Failed to fetch job: {:?}", e))?
        .ok_or("Job not found")?;

    let asset_ids: Vec<String> = serde_json::from_value(job_record.media_asset_ids)
        .map_err(|e| format!("Invalid asset IDs: {}", e))?;

    let asset_id = Uuid::parse_str(&asset_ids[0]).map_err(|e| e.to_string())?;

    let asset = sqlx::query_as::<_, db::MediaAsset>(
        "SELECT * FROM media_assets WHERE id = $1"
    )
    .bind(asset_id)
    .fetch_optional(db_pool)
    .await
    .map_err(|e| format!("Failed to fetch asset: {:?}", e))?
    .ok_or("Asset not found")?;

    let input_path = std::path::PathBuf::from(&asset.result_location.unwrap_or(asset.original_filename.clone()));

    let output_filename = format!("graded_{}.png", job.job_id);
    let output_path = std::env::temp_dir().join(&output_filename);

    update_progress(statuses, &job.job_id, 20).await;

    // Check for preset or manual adjustments
    if let Some(lut_loc) = job_record.parameters.get("lut_location").and_then(|v| v.as_str()) {
        // Apply LUT (if present)
        processor
            .apply_lut(&input_path, &output_path, lut_loc)
            .map_err(|e| format!("LUT application failed: {:?}", e))?;
    } else if let Some(preset) = job_record.parameters.get("preset").and_then(|v| v.as_str()) {
        processor
            .apply_preset(&input_path, &output_path, preset)
            .map_err(|e| format!("Preset application failed: {:?}", e))?;
    } else {
        let hue = job_record.parameters.get("hue").and_then(|v| v.as_i64()).map(|v| v as i32);
        let saturation = job_record.parameters.get("saturation").and_then(|v| v.as_i64()).map(|v| v as i32);
        let brightness = job_record.parameters.get("brightness").and_then(|v| v.as_i64()).map(|v| v as i32);
        let contrast = job_record.parameters.get("contrast").and_then(|v| v.as_i64()).map(|v| v as i32);

        processor
            .color_grade(&input_path, &output_path, hue, saturation, brightness, contrast)
            .map_err(|e| format!("Color grading failed: {:?}", e))?;
    }

    update_progress(statuses, &job.job_id, 80).await;

    // Save result
    let result_bytes = std::fs::read(&output_path)
        .map_err(|e| format!("Failed to read result: {}", e))?;

    let result_location = storage
        .save_bytes(&result_bytes, &output_filename)
        .map_err(|e| format!("Failed to save result: {:?}", e))?;

    std::fs::remove_file(&output_path).ok();

    update_progress(statuses, &job.job_id, 100).await;

    Ok(result_location)
}

async fn update_progress(
    statuses: &Arc<Mutex<HashMap<String, JobStatus>>>,
    job_id: &str,
    progress: u32,
) {
    let mut s = statuses.lock().await;
    s.insert(
        job_id.to_string(),
        JobStatus::Processing { progress },
    );
}