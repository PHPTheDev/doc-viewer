use sqlite::State;
use axum::Json;
use axum::extract::Path;
use crate::users::models::{ User, Setor, Termo, termo_pdf};

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

 

        let user = Termo {
		        nome: nome,
		        rf: rf,
            data: data,
		        setor: setor,
            qtd: qtd,
			  };

        new.push(user)
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
            data: statement.read::<String, _>("rf").unwrap(),
            qtd: statement.read::<i64, _>("rf").unwrap(),

        };

    };
        Json(rec)   

}


pub async fn new_user(Json(payload):Json<User>) {
    let connection = sqlite::open("teste.db").unwrap();
    let query = format!("INSERT INTO users (rf, senha) VALUES ('{}','{}')",payload.rf, payload.senha);
    connection.execute(
    "CREATE TABLE IF NOT EXISTS users (
            rf INTEGER PRIMARY KEY,
            senha TEXT
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

 

        let user = Termo {
                nome: nome,
                rf: rf,
            data: data,
                setor: setor,
            qtd: qtd,
              };

        new.push(user)
    };

    Json(new)



}


