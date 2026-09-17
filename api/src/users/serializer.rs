use pdfrs::builder::PdfBuilder;
use pdfrs::pdf_generator::PageLayout;
use axum::Json;
use crate::users::models::{Termo, termo_pdf};

pub async fn json(Json(payload): Json<Termo>) -> Vec<u8> {
    let connection = sqlite::open("teste.db").unwrap();
    let pdf = termo_pdf(&payload).await;
    let query = format!("INSERT INTO teste (nome,rf,setor,pdf) VALUES ('{}','{}','{}','{}','{:?}','{:?}')",payload.nome,payload.rf,payload.data, payload.qtd, payload.setor, pdf);    connection.execute(
"CREATE TABLE IF NOT EXISTS teste (
            id INTEGER PRIMARY KEY,
            nome TEXT,
            rf   TEXT, 
            data INTEGER, 
            qtd INTEGER,
            setor TEXT,
            pdf BLOB
            );").unwrap();
    connection.execute(query).unwrap();
    pdf
}



// async fn get_user(Path(user_id) : Path<Uuid>) -> Json<CreateUser> {
//     let user = find_user(user_id).await;
//     Json(user)
// }

// async fn find_user(user_id: usize) -> CreateUser {
//     let connection = sqlite::open("teste.db").unwrap();
//     let query = format!("SELECT * FROM teste WHERE id =");
// }
