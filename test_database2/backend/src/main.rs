use axum::{routing::post, Router};
use axum::extract::Json;
use serde::{Deserialize};
use PdfDocument::termo_pdf;
use http_body_util::Full;
use http::{Request, Response, Method, header};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
pub mod PdfDocument;


#[derive(Deserialize)]
struct CreateUser {
    nome: String,
    rf: String,
    setor: String, 
}

async fn json(Json(payload): Json<CreateUser>) -> Vec<u8> {
    let connection = sqlite::open("teste.db").unwrap();
    let pdf = termo_pdf(&payload).await;
    let query = format!("INSERT INTO teste (nome,rf,setor,pdf) VALUES ('{}','{}','{}','{:?}')",payload.nome,payload.rf,payload.setor,pdf);
    connection.execute(
"CREATE TABLE IF NOT EXISTS teste (
            nome TEXT,
            rf   TEXT,
            setor TEXT,
            pdf BLOB
            );").unwrap();
    connection.execute(query).unwrap();
    pdf
}





#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);


    let app = Router::new()
                      .route("/", post(json))
                      .layer(cors);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();



}