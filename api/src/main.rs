use axum::{
    Extension, Router,
    routing::{get, patch, post},
};
use http::Method;
use std::{error::Error, sync::Arc};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

mod pool;
mod termos;
mod users;
use crate::pool::ApiState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(ApiState::new().await?);

    sqlx::migrate!("./migrations")
        .run(&state.connection)
        .await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", post(termos::routes::create_termo))
        .route(
            "/termos/{id}",
            get(termos::routes::get_termo).delete(termos::routes::delete_termo),
        )
        .route("/termos", get(termos::routes::get_termos))
        .route("/termos/{id}/status", patch(termos::routes::atende_termo))
        .route("/termos/{setor}/setor", get(termos::routes::filtrar_setor))
        .route("/newUser", post(users::routes::new_user))
        //.route("/checkUser/{rf}", patch(users::routes::check_user))
        .route(
            "/users/{id}",
            patch(users::routes::put_user).delete(users::routes::delete_user),
        )
        //.route("/filtrar/{setor}",post(termos::routes::filtrar_setor))
        .layer(Extension(state))
        .layer(ServiceBuilder::new().layer(cors));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
