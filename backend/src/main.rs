mod auth;
mod config;
mod db;
mod services;
mod routes;

use axum::{
    middleware,
    routing::{get, post},
    Router,
    Extension,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub storage: Arc<dyn services::Storage>,
    pub queue: Arc<services::Queue>,
    pub config: config::Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database pool
    let db = db::create_pool(&config.database_url).await?;
    tracing::info!("Database connected");

    // Run migrations
    db::run_migrations(&db).await?;
    tracing::info!("Migrations complete");

    // Initialize storage
    let storage: Arc<dyn services::Storage> = if config.storage.mode == "s3" {
        Arc::new(services::S3Storage::new(
            &config.storage.s3_bucket.clone().unwrap_or_default(),
            &config.storage.s3_endpoint.clone().unwrap_or_default(),
        ))
    } else {
        std::fs::create_dir_all(&config.storage.local_path)?;
        Arc::new(services::LocalStorage::new(&config.storage.local_path))
    };
    tracing::info!("Storage initialized: {}", config.storage.mode);

    // Initialize job queue
    let (queue, rx) = services::Queue::new(100);
    let queue = Arc::new(queue);

    // Start worker
    let statuses = queue.get_statuses_handle();
    services::start_worker(rx, storage.clone(), db.clone(), statuses, config.clone());
    tracing::info!("Worker started");

    // Create app state
    let state = AppState {
        db: db.clone(),
        storage: storage.clone(),
        queue: queue.clone(),
        config: config.clone(),
    };

    // Build router
    let app = Router::new()
        // Public routes
        .route("/api/health", get(routes::health))
        .route("/api/auth/register", post(routes::register))
        .route("/api/auth/login", post(routes::login))
        
        // Protected routes (require authentication)
        .route("/api/upload", post(routes::upload))
        .route("/api/convert", post(routes::convert))
        .route("/api/remove-bg", post(routes::remove_bg))
        .route("/api/color-grade", post(routes::color_grade))
        .route("/api/jobs/:job_id", get(routes::get_job_status))
        .route("/api/jobs", get(routes::list_user_jobs))
        .route("/api/download/:job_id", get(routes::download_result))
        .route_layer(middleware::from_fn_with_state(
            config.jwt_secret.clone(),
            auth::auth_middleware,
        ))
        
        // Add state and CORS
        .layer(Extension(state))
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

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("🚀 MediaForge server listening on {}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}