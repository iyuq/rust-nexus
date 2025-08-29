use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    application::{UserApplicationService, CreateUserDto, UpdateUserDto, UserResponseDto, ApiResponse},
    domain::{UserError},
    infrastructure::PostgresUserRepository,
};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

// Concrete handlers for PostgresUserRepository
pub async fn create_user_concrete(
    State(app_service): State<UserApplicationService<PostgresUserRepository>>,
    Json(payload): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponseDto>>), (StatusCode, Json<ApiResponse<UserResponseDto>>)>
{
    match app_service.create_user(payload).await {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse::success(user)),
        )),
        Err(err) => Err(handle_user_error(err)),
    }
}

pub async fn get_user_concrete(
    State(app_service): State<UserApplicationService<PostgresUserRepository>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponseDto>>), (StatusCode, Json<ApiResponse<UserResponseDto>>)>
{
    match app_service.get_user_by_id(id).await {
        Ok(Some(user)) => Ok((
            StatusCode::OK,
            Json(ApiResponse::success(user)),
        )),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<UserResponseDto>::error("User not found".to_string())),
        )),
        Err(err) => Err(handle_user_error(err)),
    }
}

pub async fn update_user_concrete(
    State(app_service): State<UserApplicationService<PostgresUserRepository>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserDto>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponseDto>>), (StatusCode, Json<ApiResponse<UserResponseDto>>)>
{
    match app_service.update_user(id, payload).await {
        Ok(user) => Ok((
            StatusCode::OK,
            Json(ApiResponse::success(user)),
        )),
        Err(err) => Err(handle_user_error(err)),
    }
}

pub async fn delete_user_concrete(
    State(app_service): State<UserApplicationService<PostgresUserRepository>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<UserResponseDto>>)>
{
    match app_service.delete_user(id).await {
        Ok(()) => Ok((
            StatusCode::NO_CONTENT,
            Json(ApiResponse::success(())),
        )),
        Err(err) => Err(handle_user_error(err)),
    }
}

pub async fn get_users_concrete(
    State(app_service): State<UserApplicationService<PostgresUserRepository>>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<UserResponseDto>>>), (StatusCode, Json<ApiResponse<UserResponseDto>>)>
{
    match app_service.get_all_users(pagination.page, pagination.limit).await {
        Ok(users) => Ok((
            StatusCode::OK,
            Json(ApiResponse::success(users)),
        )),
        Err(err) => Err(handle_user_error(err)),
    }
}

fn handle_user_error(err: UserError) -> (StatusCode, Json<ApiResponse<UserResponseDto>>) {
    let (status, message) = match err {
        UserError::NotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
        UserError::EmailAlreadyExists => (StatusCode::CONFLICT, "Email already exists".to_string()),
        UserError::InvalidName(msg) => (StatusCode::BAD_REQUEST, format!("Invalid name: {}", msg)),
        UserError::InvalidEmail(msg) => (StatusCode::BAD_REQUEST, format!("Invalid email: {}", msg)),
    };
    
    (status, Json(ApiResponse::<UserResponseDto>::error(message)))
}
