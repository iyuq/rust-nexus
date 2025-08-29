use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    application::UserApplicationService,
    infrastructure::{PostgresUserRepository, web::handlers},
};

pub fn create_routes(
    app_service: UserApplicationService<PostgresUserRepository>,
) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/users", post(handlers::create_user_concrete))
        .route("/api/users", get(handlers::get_users_concrete))
        .route("/api/users/{id}", get(handlers::get_user_concrete))
        .route("/api/users/{id}", put(handlers::update_user_concrete))
        .route("/api/users/{id}", delete(handlers::delete_user_concrete))
        .with_state(app_service)
}

async fn health_check() -> &'static str {
    "OK"
}
