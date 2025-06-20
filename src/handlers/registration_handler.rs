use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::{repositories::AuthRepository, utils};

#[derive(Deserialize)]
pub struct RegistrationRequest {
    email: String,
    password: String,
}

pub async fn registration_handler(
    Extension(auth_repository): Extension<Arc<AuthRepository>>,
    Json(payload): Json<RegistrationRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let result = auth_repository
        .create_user(&payload.email, &payload.password)
        .await;

    let user_id = match result {
        Ok(id) => id,
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    return (
                        StatusCode::CONFLICT,
                        Json(json!({"error": "User already exists"})),
                    );
                }
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal Server Error",
                    "data": null
                })),
            );
        }
    };

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
        StatusCode::CREATED,
        Json(json!({
            "error": null,
            "data": {
                "token": token,
            }
        })),
    )
}
