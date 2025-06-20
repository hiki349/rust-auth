use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::{repositories::AuthRepository, utils};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login_handler(
    Extension(auth_repository): Extension<Arc<AuthRepository>>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let result = auth_repository.get_auth_user_by_email(&payload.email).await;

    let (user_id, password_hash) = match result {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Invalid credentials",
                    "data": null
                })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal Server Error",
                    "data": null
                })),
            );
        }
    };

    if payload.password != password_hash {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid credentials",
                "data": null
            })),
        );
    }

    let token = match utils::generate_jwt(user_id) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal Server Error",
                    "data": null
                })),
            );
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "error": null,
            "data": {
                "token": token
            }
        })),
    )
}
