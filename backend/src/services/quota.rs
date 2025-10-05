use crate::db;
use crate::error::Result as AppResult;
use crate::config::Config;
use uuid::Uuid;

pub async fn check_quota(db_pool: &sqlx::PgPool, config: &Config, user_id: Uuid, tier: &str, job_kind: &str) -> Result<(), String> {
    // Determine today's jobs count for the user and job_kind
    let count = db::Job::get_user_jobs_today(db_pool, user_id, Some(job_kind))
        .await
        .map_err(|e| format!("DB error: {:?}", e))?;

    let limit = match (tier, job_kind) {
        ("free", "image") => config.quotas.free_tier_image_daily as i64,
        ("free", "video") => config.quotas.free_tier_video_daily as i64,
        ("free", _) => i64::MAX,
        ("pro", "video") => config.quotas.pro_tier_video_daily as i64,
        ("pro", _) => i64::MAX,
        _ => i64::MAX,
    };

    if count >= limit {
        return Err(format!("Daily quota exceeded ({}/{}).", count, limit));
    }

    Ok(())
}

/// Concurrent jobs check (counts active/running jobs) — enforce concurrent limit
pub async fn check_concurrent(db_pool: &sqlx::PgPool, config: &Config, user_id: Uuid, tier: &str) -> Result<(), String> {
    let active = db::Job::get_active_jobs_count(db_pool, user_id)
        .await
        .map_err(|e| format!("DB error: {:?}", e))?;

    let limit = match tier {
        "free" => config.quotas.free_tier_concurrent as i64,
        "pro" => config.quotas.pro_tier_concurrent as i64,
        _ => i64::MAX,
    };

    if active >= limit {
        return Err(format!("Concurrent job limit exceeded ({}/{}).", active, limit));
    }

    Ok(())
}
