use pdfrs::builder::PdfBuilder;
use pdfrs::pdf_generator::PageLayout;
use axum::Json;
use crate::termos::models::{Termo, termo_pdf};

pub async fn json(Json(payload): Json<Termo>) -> Vec<u8> {
    let connection = sqlite::open("teste.db").unwrap();
    let pdf = termo_pdf(&payload).await;
    let query = format!("INSERT INTO teste (nome,rf,data,qtd,setor,pdf,user) 
        VALUES ('{}','{}','{}','{}','{:?}','{:?}', '{}')",
        payload.nome,
        payload.rf,
        payload.data, 
        payload.qtd, 
        payload.setor, 
        pdf,
        payload.user
    );    
    connection.execute(
        "CREATE TABLE IF NOT EXISTS teste (
            id INTEGER PRIMARY KEY,
            nome TEXT,
            rf   TEXT, 
            data INTEGER, 
            qtd INTEGER,
            setor TEXT,
            pdf BLOB,
            user INTEGER,
            FOREIGN KEY(user) REFERENCES users(rf)
            );"
    ).unwrap();
    connection.execute(query).unwrap();
    pdf
}