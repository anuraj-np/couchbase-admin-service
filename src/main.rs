use axum::{
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, error, Level};
use tracing_subscriber;
use prometheus::{TextEncoder, Encoder};

mod config;
mod error;
mod middleware;
mod models;
mod routes;
mod services;

use config::Config;
use error::AppError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded successfully");

    // Initialize Couchbase service
    let couchbase_service = match services::CouchbaseService::new(&config) {
        Ok(service) => {
            info!("Couchbase service initialized successfully");
            service
        }
        Err(e) => {
            error!("Failed to initialize Couchbase service: {}", e);
            error!("Application will start but Couchbase operations will fail");
            // Create a dummy service or handle this differently
            return Err(e.into());
        }
    };

    // Build application routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))
        .route("/buckets", post(routes::buckets::create_bucket))
        .route("/buckets", get(routes::buckets::list_buckets))
        .route("/buckets/:bucket/scopes", post(routes::scopes::create_scope))
        .route("/buckets/:bucket/scopes", get(routes::scopes::list_scopes))
        .route(
            "/buckets/:bucket/scopes/:scope/collections",
            post(routes::collections::create_collection),
        )
        .route(
            "/buckets/:bucket/scopes/:scope/collections",
            get(routes::collections::list_collections),
        )
        .route("/users", post(routes::users::create_user))
        .route("/users", get(routes::users::list_users))
        .route("/users/:username", get(routes::users::get_user))
        .route("/users/:username", delete(routes::users::delete_user))
        .route("/users/:username/roles", put(routes::users::update_user_roles))
        .route("/users/:username/permissions", get(routes::users::get_user_permissions))
        .route("/roles", get(routes::users::get_available_roles))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(axum::middleware::from_fn(middleware::auth_middleware)),
        )
        .with_state(couchbase_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn metrics_handler() -> Result<String, AppError> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(String::from_utf8(buffer)?)
}
