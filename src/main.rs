// Hexagonal architecture modules
mod domain;
mod application;
mod infrastructure;

// Legacy database module (still needed for setup_database function)
mod database;

use anyhow::Result;
use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use dotenvy::dotenv;
use std::env;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    database::setup_database,
    application::UserApplicationService,
    infrastructure::{PostgresUserRepository, create_routes},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_nexus=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get configuration from environment
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Database setup with optimized pool
    let pool = setup_database().await?;

    // Create repository adapter
    let user_repository = PostgresUserRepository::new(pool);
    
    // Create application service
    let user_app_service = UserApplicationService::new(user_repository);

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Build the application with middleware
    let app = create_routes(user_app_service).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(cors),
    );

    // Create listener with TCP optimizations for high load
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    
    tracing::info!("Server running on http://0.0.0.0:{}", port);
    tracing::info!("Connection pool: max={}, optimized for high-load scenarios", 100);

    // Start the server
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
