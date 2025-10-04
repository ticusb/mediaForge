// User model stub

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub subscription_tier: String, // "free" or "pro"
    pub daily_quota: i32,
    pub concurrent_jobs_allowed: i32,
    pub created_at: String,
}
