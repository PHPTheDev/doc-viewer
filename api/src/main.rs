use axum::{routing::{post, get}, Router};
use http::{Request, Response, Method, header};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;

pub mod users;
pub mod termos;

//const app: Router = Router::new();

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any)
    .allow_headers(Any);


    let app = Router::new()
                      .route("/", post(termos::serializer::json))
                      .route("/termos/{id}", get(termos::routes::get_termo))
                      .route("/termos", get(termos::routes::get_termos))
                      .route("/newUser",post(users::routes::new_user))
                      .route("/checkUser",post(users::routes::check_user))
                      .route("/filtrar/{setor}",post(termos::routes::filtrar_setor))
                      .layer(ServiceBuilder::new().layer(cors));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();



}
