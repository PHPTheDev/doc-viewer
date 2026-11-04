use std::sync::Arc;

use crate::pool::ApiState;
use api_models::users::models::User;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use http::StatusCode;
use sqlx::Row;

pub async fn new_user(
    Extension(state): Extension<Arc<ApiState>>,
    Json(payload): Json<User>,
) -> Result<StatusCode, StatusCode> {
    let hash = Argon2::default()
        .hash_password(payload.senha.as_bytes())
        .unwrap()
        .to_string();

    let query = sqlx::query("INSERT INTO user (rf, senha, is_admin) VALUES ($1, $2, $3)")
        .bind(payload.rf)
        .bind(hash)
        .bind(payload.is_admin);

    let result = query.execute(&state.connection).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_user(
    Extension(state): Extension<Arc<ApiState>>,
    Path(user_id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let query = sqlx::query("DELETE FROM user WHERE rf = $1").bind(user_id);
    let result = query.execute(&state.connection).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn put_user(
    Extension(state): Extension<Arc<ApiState>>,
    Path(user_id): Path<i64>,
    Json(payload): Json<User>,
) -> Result<StatusCode, StatusCode> {
    let hash = Argon2::default()
        .hash_password(payload.senha.as_bytes())
        .unwrap()
        .to_string();

    let query = sqlx::query(
        "UPDATE user SET rf = $1, senha = $2, is_admin = $3 FROM user as u WHERE u.rf = $4",
    )
    .bind(payload.rf)
    .bind(hash)
    .bind(payload.is_admin)
    .bind(user_id);
    let result = query.execute(&state.connection).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[axum::debug_handler]
pub async fn check_user(
    Extension(state): Extension<Arc<ApiState>>,
    Path(rf): Path<i64>,
    Json(sen): Json<String>,
) -> impl IntoResponse {
    let query = sqlx::query("SELECT * FROM user WHERE rf = $1").bind(rf);
    let fetch = query.fetch_one(&state.connection).await;

    let result = match fetch {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let senha = result.get::<String, _>("senha");
    let password_hash = PasswordHash::new(&senha).unwrap();

    match Argon2::default().verify_password(sen.as_bytes(), &password_hash) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
