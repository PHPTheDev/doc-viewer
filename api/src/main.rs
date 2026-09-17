use axum::{routing::post, Router};
use http::{Request, Response, Method, header};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;

pub mod users;

//const app: Router = Router::new();

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any)
    .allow_headers(Any);


    let app = Router::new()
                      .route("/", post(users::serializer::json))
                      .layer(ServiceBuilder::new().layer(cors));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();



}
