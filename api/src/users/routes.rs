use std::sync::Arc;

use api_models::users::models::User;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use http::StatusCode;
use sqlx::Row;

use crate::pool::ApiState;

pub async fn new_user(
    Extension(state): Extension<Arc<ApiState>>,
    Json(payload): Json<User>,
) -> Result<StatusCode, StatusCode> {
    let query = sqlx::query("INSERT INTO user (rf, senha, is_admin) VALUES ($1, $2, $3)")
        .bind(payload.rf)
        .bind(payload.senha)
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
    let query = sqlx::query(
        "UPDATE user SET rf = $1, senha = $2, is_admin = $3 FROM user as u WHERE u.rf = $4",
    )
    .bind(payload.rf)
    .bind(payload.senha)
    .bind(payload.is_admin)
    .bind(user_id);
    let result = query.execute(&state.connection).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn check_user(
    Extension(state): Extension<Arc<ApiState>>,
    Json(sen): Json<String>,
    Path(rf): Path<i64>,
) -> impl IntoResponse {
    let query = sqlx::query("SELECT * FROM user WHERE rf = $1").bind(rf);
    let fetch = query.fetch_one(&state.connection).await;

    let result = match fetch {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let senha = result.get::<String, _>("senha");

    if senha == sen {
        //println!("certo");
        Ok(StatusCode::OK)
    } else {
        //println!("errado");
        Err(StatusCode::BAD_REQUEST)
    }
}
