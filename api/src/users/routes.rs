use sqlite::State;
use axum::Json;
use axum::extract::Path;
use crate::users::models::{CreateUser, termo_pdf, termoalize};

#[axum::debug_handler]
pub async fn get_termos() -> Json<Vec<CreateUser>> {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("SELECT * FROM teste");
    let mut statement = connection.prepare(query).unwrap();
    let mut new = vec![];
    while let Ok(State::Row) = statement.next() {
        let nome = statement.read::<String, _>("nome").unwrap();
        let rf = statement.read::<String, _>("rf").unwrap();
        let setor = statement.read::<String, _>("setor").unwrap();
        //let rec = termoalize(&(&nome, &rf, &setor)).await;
        let user = CreateUser {
		nome: nome,
		rf: rf,
		setor: setor
			};

        new.push(user)
    };

    Json(new)

}

pub async fn get_termo(Path(user_id): Path<i64>) -> Json<CreateUser> {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("SELECT * FROM teste WHERE id = ?");
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user_id)).unwrap();
    let mut rec = CreateUser {
        nome: String::new(),
        rf: String::new(),
        setor: String::new(),
    };
    while let Ok(State::Row) = statement.next() {
        println!("nome = {}", statement.read::<String, _>("nome").unwrap());
        println!("nome = {}", statement.read::<String, _>("rf").unwrap());
        println!("nome = {}", statement.read::<String, _>("setor").unwrap());
        rec = CreateUser {
            nome: statement.read::<String, _>("nome").unwrap(),
            rf: statement.read::<String, _>("rf").unwrap(),
            setor: statement.read::<String, _>("setor").unwrap(),
        };

    };
        Json(rec)   

}