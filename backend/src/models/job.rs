// Job model stub

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub media_asset_ids: Vec<String>,
    pub job_type: String,
    pub parameters: serde_json::Value,
    pub status: String,
    pub progress_percent: i32,
    pub priority: i32,
    pub result_location: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}
