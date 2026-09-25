use sqlite::State;
use axum::Json;
use axum::extract::Path;
//use crate::users::models::User;
use api_models::users::models::User;
use axum::response::Response;

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

pub async fn delete_user(Path(user_id):Path<i64>) {
    let con = sqlite::open("teste.db").unwrap();
    let query = format!("DELETE FROM users WHERE rf = {}", user_id);
    con.execute(query).unwrap();

    Response::builder()
    .status(200)
    .header("X-Custom-Foo", "Bar")
    .body(())
    .unwrap();
}

pub async fn put_user(Path(user_id):Path<i64>, Json(payload):Json<User>){
    let con = sqlite::open("teste.db").unwrap();
    let query = format!("UPDATE users SET rf = {}, senha = '{}', is_admin = {} FROM users as u WHERE u.rf = {}", 
        payload.rf,
        payload.senha,
        payload.is_admin,
        user_id
    );
    con.execute(query).unwrap();

    Response::builder()
    .status(200)
    .header("X-Custom-Foo", "Bar")
    .body(())
    .unwrap();

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




