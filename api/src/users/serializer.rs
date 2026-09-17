use pdfrs::builder::PdfBuilder;
use pdfrs::pdf_generator::PageLayout;
use crate::users::models::{CreateUser, termo_pdf};
use axum::Json;

pub async fn json(Json(payload): Json<CreateUser>) -> Vec<u8> {
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

