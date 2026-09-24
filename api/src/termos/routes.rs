use sqlite::{Connection, State};
use axum::Json;
use axum::response::Response;
use axum::extract::Path;
use pdfrs::pdf_generator::PageLayout;
use pdfrs::builder::PdfBuilder;
//use crate::termos::models::{ Setor, Status, Termo, termo_pdf};
use api_models::termos::models::{Setor, Status, Termo};

pub async fn termo_pdf(i: &Termo) -> Vec<u8>  {
    PdfBuilder::new()
    .with_layout(PageLayout::landscape())
    .with_margins(72.0)
    .add_heading("termo",1)
    .add_paragraph(format!("nome={}, rf={}, setor={:?}",i.nome,i.rf,i.setor).as_str())
    .build_bytes()
    .unwrap()


}


//#[axum::debug_handler]
pub async fn get_termos() -> Json<Vec<Termo>> {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("SELECT * FROM teste");
    let mut statement = connection.prepare(query).unwrap();
    let mut new = vec![];
    while let Ok(State::Row) = statement.next() {
        let nome = statement.read::<String, _>("nome").unwrap();
        let rf = statement.read::<String, _>("rf").unwrap();
        let setor = Setor::parse(statement.read::<String, _>("setor").unwrap());      
        let data = statement.read::<String, _>("rf").unwrap();
        let qtd = statement.read::<i64, _>("rf").unwrap();
        let status = Status::parse(statement.read::<String, _>("status").unwrap());
        let user = statement.read::<i64, _>("user").unwrap();

        let termo = Termo {
		        nome: nome,
		        rf: rf,
            data: data,
		        setor: setor,
            qtd: qtd,
            status: status,
            user: user,
		};

        new.push(termo)
    };

    Json(new)

}

pub async fn get_termo(Path(user_id): Path<i64>) -> Json<Termo> {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("SELECT * FROM teste WHERE id = ?");
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user_id)).unwrap();
    let mut rec = Termo::default();

    while let Ok(State::Row) = statement.next() {
        println!("nome = {}", statement.read::<String, _>("nome").unwrap());
        println!("nome = {}", statement.read::<String, _>("rf").unwrap());
        println!("nome = {}", statement.read::<String, _>("setor").unwrap());
        rec = Termo {
            nome: statement.read::<String, _>("nome").unwrap(),
            rf: statement.read::<String, _>("rf").unwrap(),
            setor: Setor::parse(statement.read::<String, _>("setor").unwrap()),
            data: statement.read::<String, _>("data").unwrap(),
            qtd: statement.read::<i64, _>("qtd").unwrap(),
            status: Status::parse(statement.read::<String, _>("status").unwrap()),
            user: statement.read::<i64, _>("user").unwrap(),

        };

    };
        Json(rec)   

}

pub async fn put_termo(Path(user_id): Path<i64>, Json(payload): Json<Termo>) {
    let status = payload.status.as_str();
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("UPDATE teste SET status = '{}' FROM teste as t WHERE t.id = {}", status, user_id);
    // let mut statement = connection.prepare(query).unwrap();
    //statement.bind((1, status)).unwrap();


    connection.execute(query).unwrap();

        Response::builder()
        .status(200)
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap();

}

pub async fn delete_termo(Path(user_id): Path<i64>) {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("DELETE FROM teste WHERE id = {}", user_id);
    connection.execute(query).unwrap();

    Response::builder()
    .status(200)
    .header("X-Custom-Foo", "Bar")
    .body(())
    .unwrap();
}

pub async fn filtrar_setor(Path(setor): Path<String>) -> Json<Vec<Termo>> {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("SELECT * FROM teste WHERE setor = ?");
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1,setor.as_str())).unwrap();
    let mut new = vec![];
    while let Ok(State::Row) = statement.next() {
        let nome = statement.read::<String, _>("nome").unwrap();
        let rf = statement.read::<String, _>("rf").unwrap();
        let setor = Setor::parse(statement.read::<String, _>("setor").unwrap());      
        let data = statement.read::<String, _>("rf").unwrap();
        let qtd = statement.read::<i64, _>("rf").unwrap();
        let status = Status::parse(statement.read::<String, _>("status").unwrap());
        let user = statement.read::<i64, _>("user").unwrap();

        let termo = Termo {
                nome: nome,
                rf: rf,
                data: data,
                setor: setor,
                qtd: qtd,
                status: status,
                user: user,
        };

        new.push(termo)
    };

    Json(new)



}

