mod auth;
mod config;
mod db;
mod error;
mod routes;
mod services;

use anyhow::Context;
use axum::{middleware, routing::get, routing::post, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub storage: Arc<dyn services::Storage>,
    pub queue: Arc<services::Queue>,
    pub config: Arc<config::Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing with environment filter
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,media_processor_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 MediaForge Server Starting...");

    // Load configuration
    let config = config::Config::from_env()
        .context("Failed to load configuration from environment")?;
    tracing::info!("✓ Configuration loaded successfully");

    // Create database pool with retry logic
    let db = db::create_pool(&config.database_url)
        .await
        .context("Failed to create database connection pool")?;
    tracing::info!("✓ Database connection pool created");

    // Test database connection
    sqlx::query("SELECT 1")
        .execute(&db)
        .await
        .context("Failed to connect to database. Is PostgreSQL running?")?;
    tracing::info!("✓ Database connection verified");

    // Run migrations
    db::run_migrations(&db)
        .await
        .context("Failed to run database migrations")?;
    tracing::info!("✓ Database migrations completed");

    // Initialize storage
    let storage: Arc<dyn services::Storage> = if config.storage.mode == "s3" {
        let s3_storage = services::S3Storage::new(
            config
                .storage
                .s3_bucket
                .as_deref()
                .context("S3_BUCKET required when STORAGE_MODE=s3")?,
            config
                .storage
                .s3_endpoint
                .as_deref()
                .context("S3_ENDPOINT required when STORAGE_MODE=s3")?,
        );
        Arc::new(s3_storage)
    } else {
        std::fs::create_dir_all(&config.storage.local_path)
            .context("Failed to create local storage directory")?;
        Arc::new(services::LocalStorage::new(&config.storage.local_path))
    };
    tracing::info!("✓ Storage initialized: {}", config.storage.mode);

    // Create required directories
    std::fs::create_dir_all(&config.processing.temp_dir)
        .context("Failed to create temp directory")?;
    tracing::info!("✓ Temporary directory created");

    // Initialize job queue
    let (queue, rx) = services::Queue::new(100);
    let queue = Arc::new(queue);

    // Start worker
    let statuses = queue.get_statuses_handle();
    services::start_worker(
        rx,
        storage.clone(),
        db.clone(),
        statuses,
        config.clone(),
    );
    tracing::info!("✓ Background worker started");

    // Create app state
    let state = AppState {
        db: db.clone(),
        storage: storage.clone(),
        queue: queue.clone(),
        config: Arc::new(config.clone()),
    };

    // Build router
    let app = Router::new()
        // Health check (public)
        .route("/api/health", get(routes::health))
        // Authentication routes (public)
        .route("/api/auth/register", post(routes::register))
        .route("/api/auth/login", post(routes::login))
        // Protected routes
        .route("/api/upload", post(routes::upload))
        .route("/api/convert", post(routes::convert))
        .route("/api/remove-bg", post(routes::remove_bg))
        .route("/api/color-grade", post(routes::color_grade))
        .route("/api/jobs/:job_id", get(routes::get_job_status))
        .route("/api/jobs", get(routes::list_user_jobs))
        .route("/api/download/:job_id", get(routes::download_result))
        .layer(middleware::from_fn_with_state(
            config.jwt_secret.clone(),
            auth::auth_middleware,
        ))
        // Add state
        .with_state(state)
        // CORS
        .layer(
            CorsLayer::permissive()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([
                    hyper::Method::GET,
                    hyper::Method::POST,
                    hyper::Method::OPTIONS,
                ])
                .allow_headers(tower_http::cors::Any),
        );

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context(format!("Failed to bind to {}", addr))?;

    tracing::info!("🎉 MediaForge server listening on http://{}", addr);
    tracing::info!("📖 API Documentation: http://{}/api/health", addr);

    axum::serve(listener, app)
        .await
        .context("Server error")?;

    Ok(())
}