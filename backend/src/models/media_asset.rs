// MediaAsset model stub

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: String,
    pub user_id: String,
    pub original_filename: String,
    pub format: String,
    pub size_bytes: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub status: String,
    pub result_location: Option<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}
