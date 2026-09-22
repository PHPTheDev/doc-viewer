use sqlite::State;
use axum::Json;
use axum::extract::Path;
use crate::users::models::User;

pub async fn new_user(Json(payload):Json<User>) {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("INSERT INTO users (rf, senha, is_admin) VALUES ('{}','{}', '{}')",payload.rf, payload.senha, payload.is_admin);
    connection.execute(
    "CREATE TABLE IF NOT EXISTS users (
            rf INTEGER PRIMARY KEY,
            senha TEXT,
            is_admin INTEGER 
            );").unwrap();
    connection.execute(query).unwrap();

}


pub async fn check_user(Json(payload):Json<User>) {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("SELECT * FROM users WHERE rf = ?");
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1,payload.rf)).unwrap();

    while let Ok(State::Row) = statement.next() {
        let senha = statement.read::<String, _>("senha").unwrap();

       if senha == payload.senha {
            println!("certo")
       }
       else{
            println!("errado");

        }

    }
}  




