use axum::{routing::post, Router};
use axum::extract::Json;
use serde::{Deserialize};


#[derive(Deserialize)]
struct CreateUser {
    nome: String,
    rf: String,
    setor: String, 
}

async fn json(Json(payload): Json<CreateUser>) {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("INSERT INTO teste (nome,rf,setor) VALUES ('{}','{}','{}')",payload.nome,payload.rf,payload.setor);
    connection.execute(
"CREATE TABLE IF NOT EXISTS teste (
            nome TEXT,
            rf   TEXT,
            setor TEXT
            );").unwrap();
    connection.execute(query).unwrap();
}


#[tokio::main]
async fn main() {

    let app = Router::new()
                      .route("/", post(json));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();



}