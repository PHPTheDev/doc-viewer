use axum::Json;
use axum::response::Response;
use axum::extract::{State, Path};
use pdfrs::pdf_generator::PageLayout;
use pdfrs::builder::PdfBuilder;
use api_models::termos::models::{Setor, Status, Termo};
use http::StatusCode;
use sqlx::Row;

use crate::pool::ApiState;

// type Result<T> = Result<T, sqlx::Error>

pub async fn termo_pdf(i: &Termo) -> Vec<u8>  {
    PdfBuilder::new()
    .with_layout(PageLayout::landscape())
    .with_margins(72.0)
    .add_heading("termo",1)
    .add_paragraph(format!("nome={}, rf={}, setor={:?}",i.nome,i.rf,i.setor).as_str())
    .build_bytes()
    .unwrap()
}

pub async fn create_termo(State(state): State<ApiState>, Json(payload): Json<Termo>) -> Result<StatusCode, StatusCode> {

    let pdf = termo_pdf(&payload).await;
    let query = sqlx::query(
        "INSERT INTO teste (nome,rf,data,qtd,setor,status,pdf,user) 
        VALUES ($1, $2, $3, $4,'{:?}','{:?}','{:?}', '{}')"
    )
        .bind(payload.nome)
        .bind(payload.rf)   
        .bind(payload.data)
        .bind(payload.qtd)
        .bind(payload.setor.as_str())
        .bind(payload.status.as_str())
        .bind(pdf)
        .bind(payload.user);

    let result = query.execute(&state.connection.unwrap()).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(StatusCode::BAD_REQUEST), 
    }
    //pdf
}

#[axum::debug_handler]
pub async fn get_termos(State(state): State<ApiState>) -> Result<Json<Vec<Termo>>, StatusCode> {

    let query = sqlx::query("SELECT * FROM teste");

    let rows = query.fetch_all(&state.connection.unwrap()).await;

    let results  = match rows {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::NOT_FOUND), 
    };

    let termos: Vec<Termo> = results.iter().map(|row| {
        Termo {
            id: row.get::<i64, _>("id"),
            nome: row.get::<String, _>("nome"),
		    rf: row.get::<String, _>("rf"),
            data: row.get::<String, _>("data"),
		    setor: Setor::parse(row.get::<String, _>("setor")),
            qtd: row.get::<i64, _>("qtd"),
            status: Status::parse(row.get::<String, _>("status")),
            user: row.get::<i64, _>("user"),
		}
    }).collect();

    Ok(Json(termos))
}

pub async fn get_termo(State(state): State<ApiState>, Path(id): Path<i64>) -> Result<Json<Termo>, StatusCode> {

    let query = sqlx::query("SELECT * FROM teste WHERE id = $1").bind(id);
    
    // let mut rec = Termo::default();
    let rows = query.fetch_one(&state.connection.unwrap()).await;

    let row = match rows {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::NOT_FOUND), 
    };
        
    let rec = Termo {
        id: row.get::<i64, _>("id"),
        nome: row.get::<String, _>("nome"),
        rf: row.get::<String, _>("rf"),
        data: row.get::<String, _>("data"),
        setor: Setor::parse(row.get::<String, _>("setor")),
        qtd: row.get::<i64, _>("qtd"),
        status: Status::parse(row.get::<String, _>("status")),
        user: row.get::<i64, _>("user"),
    };
    Ok(Json(rec))   

}

pub async fn atende_termo(
    State(state): State<ApiState>, 
    Path(id): Path<i64>, 
    Json(status): Json<Status>
) -> Result<StatusCode, StatusCode>{

    let query = sqlx::query(
        "UPDATE teste SET status = $1 
        FROM teste as t 
        WHERE t.id = $2"
    )
        .bind(status.as_str())
        .bind(id);
    
    let result = query.execute(&state.connection.unwrap()).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(StatusCode::BAD_REQUEST), 
    }
}

pub async fn delete_termo(
    State(state): State<ApiState>, 
    Path(id): Path<i64>
) -> Result<StatusCode, StatusCode>{

    let query = sqlx::query(
        "DELETE FROM teste WHERE id = $1"
    )
        .bind(id);
    
    let result = query.execute(&state.connection.unwrap()).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(StatusCode::BAD_REQUEST), 
    }
}

pub async fn filtrar_setor(State(state): State<ApiState>, Path(setor): Path<String>) -> Result<Json<Vec<Termo>>, StatusCode> {
    let query = sqlx::query(
        "SELECT * FROM teste WHERE setor = $1"
    )
        .bind(setor);

    let rows = query.fetch_all(&state.connection.unwrap()).await;

    let results  = match rows {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::NOT_FOUND), 
    };

    let termos: Vec<Termo> = results.iter().map(|row| {
        Termo {
            id: row.get::<i64, _>("id"),
            nome: row.get::<String, _>("nome"),
            rf: row.get::<String, _>("rf"),
            data: row.get::<String, _>("data"),
            setor: Setor::parse(row.get::<String, _>("setor")),
            qtd: row.get::<i64, _>("qtd"),
            status: Status::parse(row.get::<String, _>("status")),
            user: row.get::<i64, _>("user"),
        }
    }).collect();

    Ok(Json(termos))
}

