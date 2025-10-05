use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
    pub storage: StorageConfig,
    pub quotas: QuotaConfig,
    pub processing: ProcessingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub mode: String, // "local" or "s3"
    pub local_path: String,
    pub s3_endpoint: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_access_key: Option<String>,
    pub s3_secret_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuotaConfig {
    pub free_tier_image_daily: u32,
    pub free_tier_video_daily: u32,
    pub free_tier_concurrent: u32,
    pub pro_tier_video_daily: u32,
    pub pro_tier_concurrent: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessingConfig {
    pub max_image_size_mb: u64,
    pub max_video_size_mb: u64,
    pub max_video_duration_seconds: u32,
    pub model_path: String,
    pub temp_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET")?,
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            storage: StorageConfig {
                mode: env::var("STORAGE_MODE").unwrap_or_else(|_| "local".to_string()),
                local_path: env::var("LOCAL_STORAGE_PATH")
                    .unwrap_or_else(|_| "./data/uploads".to_string()),
                s3_endpoint: env::var("S3_ENDPOINT").ok(),
                s3_bucket: env::var("S3_BUCKET").ok(),
                s3_access_key: env::var("S3_ACCESS_KEY").ok(),
                s3_secret_key: env::var("S3_SECRET_KEY").ok(),
            },
            quotas: QuotaConfig {
                free_tier_image_daily: env::var("FREE_TIER_IMAGE_DAILY")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?,
                free_tier_video_daily: env::var("FREE_TIER_VIDEO_DAILY")
                    .unwrap_or_else(|_| "3".to_string())
                    .parse()?,
                free_tier_concurrent: env::var("FREE_TIER_CONCURRENT")
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()?,
                pro_tier_video_daily: env::var("PRO_TIER_VIDEO_DAILY")
                    .unwrap_or_else(|_| "50".to_string())
                    .parse()?,
                pro_tier_concurrent: env::var("PRO_TIER_CONCURRENT")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()?,
            },
            processing: ProcessingConfig {
                max_image_size_mb: env::var("MAX_IMAGE_SIZE_MB")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()?,
                max_video_size_mb: env::var("MAX_VIDEO_SIZE_MB")
                    .unwrap_or_else(|_| "50".to_string())
                    .parse()?,
                max_video_duration_seconds: env::var("MAX_VIDEO_DURATION_SECONDS")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?,
                model_path: env::var("MODEL_PATH")
                    .unwrap_or_else(|_| "./models/u2net.onnx".to_string()),
                temp_dir: env::var("TEMP_DIR")
                    .unwrap_or_else(|_| "./data/temp".to_string()),
            },
        })
    }
}