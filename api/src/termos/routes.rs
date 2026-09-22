use sqlite::State;
use axum::Json;
use axum::extract::Path;
use crate::termos::models::{ Setor, Termo, termo_pdf};

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
        let user = statement.read::<i64, _>("user").unwrap();

        let termo = Termo {
		        nome: nome,
		        rf: rf,
            	data: data,
		        setor: setor,
            	qtd: qtd,
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
            data: statement.read::<String, _>("rf").unwrap(),
            qtd: statement.read::<i64, _>("rf").unwrap(),
            user: statement.read::<i64, _>("user").unwrap(),

        };

    };
        Json(rec)   

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
        let user = statement.read::<i64, _>("user").unwrap();

        let termo = Termo {
                nome: nome,
                rf: rf,
                data: data,
                setor: setor,
                qtd: qtd,
                user: user,
        };

        new.push(termo)
    };

    Json(new)



}

