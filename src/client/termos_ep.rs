use api_models::termos::models::{Setor, Status, Termo};
use reqwest::{Client, Body};

#[tokio::main(worker_threads = 10)]
pub async fn get_termos() -> Vec<Termo> {
    let body = reqwest::get("http://127.0.0.1:3000/termos").await.unwrap().json::<Vec<Termo>>().await.unwrap();
    print!("{:?}", body.clone()); 
    body
}

#[tokio::main(worker_threads = 10)]
pub async fn atender_termo(id: i64) {
    let url = format!("http://127.0.0.1:3000/termos/{}/status", id);
    let cli = Client::new();
    let mut new = Termo::new();
    new.status = Status::ATENDIDO;
    new.user = 1234;
    let _ = cli.put(url).json(&new).send().await.unwrap();
    println!("this works?");
}

#[tokio::main(worker_threads = 10)]
pub async fn filtro_termos(setor: Setor) -> Vec<Termo> {
	let url = format!("http://127.0.0.1:3000/termos/{}", setor.as_str());
    let body = reqwest::get(url).await.unwrap().json::<Vec<Termo>>().await.unwrap();
    print!("{:?}", body.clone()); 
    body
}



