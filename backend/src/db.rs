// backend/src/db.rs
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    Connection(#[from] sqlx::Error),
    #[error("Record not found")]
    NotFound,
}

/// Create database connection pool
pub async fn create_pool(database_url: &str) -> Result<PgPool, DbError> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .connect(database_url)
        .await
        .map_err(DbError::Connection)
}

/// Run pending migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), DbError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| DbError::Connection(sqlx::Error::Migrate(Box::new(e))))
}

// Database models matching your schema

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub subscription_tier: String,
    pub daily_quota: i32,
    pub concurrent_jobs_allowed: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct MediaAsset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_filename: String,
    pub format: String,
    pub size_bytes: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub status: String,
    pub result_location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct Job {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_asset_ids: serde_json::Value,
    pub job_type: String,
    pub parameters: serde_json::Value,
    pub status: String,
    pub progress_percent: i32,
    pub priority: i32,
    pub result_location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

// Database operations

impl User {
    pub async fn create(
        pool: &PgPool,
        email: &str,
        password_hash: &str,
        tier: &str,
    ) -> Result<Self, DbError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, subscription_tier, daily_quota, concurrent_jobs_allowed)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(email)
        .bind(password_hash)
        .bind(tier)
        .bind(if tier == "free" { 10 } else { 999999 })
        .bind(if tier == "free" { 1 } else { 5 })
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, DbError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, DbError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}

impl MediaAsset {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        filename: &str,
        format: &str,
        size_bytes: i64,
    ) -> Result<Self, DbError> {
        let asset = sqlx::query_as::<_, MediaAsset>(
            r#"
            INSERT INTO media_assets 
            (id, user_id, original_filename, format, size_bytes, status, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(filename)
        .bind(format)
        .bind(size_bytes)
        .bind("uploaded")
        .bind(Utc::now())
        .bind(Utc::now() + chrono::Duration::hours(24))
        .fetch_one(pool)
        .await?;

        Ok(asset)
    }

    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        status: &str,
        result_location: Option<&str>,
    ) -> Result<(), DbError> {
        sqlx::query(
            "UPDATE media_assets SET status = $1, result_location = $2 WHERE id = $3"
        )
        .bind(status)
        .bind(result_location)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl Job {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        asset_ids: Vec<Uuid>,
        job_type: &str,
        parameters: serde_json::Value,
        priority: i32,
    ) -> Result<Self, DbError> {
        let job = sqlx::query_as::<_, Job>(
            r#"
            INSERT INTO jobs 
            (id, user_id, media_asset_ids, job_type, parameters, status, progress_percent, priority)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(serde_json::to_value(asset_ids).unwrap())
        .bind(job_type)
        .bind(parameters)
        .bind("queued")
        .bind(0)
        .bind(priority)
        .fetch_one(pool)
        .await?;

        Ok(job)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, DbError> {
        let job = sqlx::query_as::<_, Job>(
            "SELECT * FROM jobs WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(job)
    }

    pub async fn update_progress(
        pool: &PgPool,
        id: Uuid,
        status: &str,
        progress: i32,
    ) -> Result<(), DbError> {
        sqlx::query(
            "UPDATE jobs SET status = $1, progress_percent = $2 WHERE id = $3"
        )
        .bind(status)
        .bind(progress)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn complete(
        pool: &PgPool,
        id: Uuid,
        result_location: &str,
    ) -> Result<(), DbError> {
        sqlx::query(
            r#"
            UPDATE jobs 
            SET status = 'completed', progress_percent = 100, result_location = $1, completed_at = $2
            WHERE id = $3
            "#
        )
        .bind(result_location)
        .bind(Utc::now())
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn fail(
        pool: &PgPool,
        id: Uuid,
        error: &str,
    ) -> Result<(), DbError> {
        sqlx::query(
            "UPDATE jobs SET status = 'failed', parameters = jsonb_set(parameters, '{error}', $1) WHERE id = $2"
        )
        .bind(serde_json::to_value(error).unwrap())
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_jobs_today(
        pool: &PgPool,
        user_id: Uuid,
        job_type: Option<&str>,
    ) -> Result<i64, DbError> {
        let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
        
        let count = if let Some(jt) = job_type {
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM jobs WHERE user_id = $1 AND job_type = $2 AND created_at >= $3"
            )
            .bind(user_id)
            .bind(jt)
            .bind(today_start)
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM jobs WHERE user_id = $1 AND created_at >= $2"
            )
            .bind(user_id)
            .bind(today_start)
            .fetch_one(pool)
            .await?
        };

        Ok(count)
    }
}