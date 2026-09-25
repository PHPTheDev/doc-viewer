use iced::Task;
use iced::widget::{Column, text, column, button};
use iced::Element;
use reqwest::{Client, Body};
use api_models::termos::models::{ Status, Termo};



#[derive(Debug, Clone)]
enum Message {
    GetFetch,
    ChangeStatusTermo(i64),
    //GetFetched(String),
}

#[derive(Default)]
struct State {
    data: Vec<Termo>,
}


fn update(state: &mut State, message: Message) { //-> Task<Message> 
    match message {
        Message::GetFetch => {
            state.data = get_termos();


        }
        Message::ChangeStatusTermo(val) => {
            atender_termo(val);
            state.data = get_termos();
        }//Task::perform(
                //get_termos(),
                //Message::GetFetched
            //),
        //Message::GetFetched(texto) => {
         //   state.text = texto;

          //  Task::none()

        //}



    }

}

#[tokio::main(worker_threads = 10)]
async fn get_termos() -> Vec<Termo> {
    let body = reqwest::get("http://127.0.0.1:3000/termos").await.unwrap().json::<Vec<Termo>>().await.unwrap();
    print!("{:?}", body.clone()); 
    body
}

#[tokio::main(worker_threads = 10)]
async fn atender_termo(id: i64) {
    let url = format!("http://127.0.0.1:3000/termos/{}/status", id);
    let cli = Client::new();
    let mut new = Termo::new();
    new.status = Status::ATENDIDO;
    new.user = 1234;
    let _ = cli.put(url).json(&new).send().await.unwrap();
    println!("this works?");
}


fn view(state: &State) -> Element<'_, Message> {
    let button = button(text("pesquisar")).on_press(Message::GetFetch);
    let dt = iter_data(state);

        let interface = column![button, dt].into();
        interface

}

fn iter_data(state: &State) -> Element<'_, Message> {
    state.data
        .iter()
        .fold(column![], |col, item| {
                let counter = 1;
                col.push(column![
                    text(format!("nome: {}" ,item.nome.clone())),
                    text(item.rf.clone()),
                    text(item.status.as_str().clone()),
                    text(item.qtd.clone()),
                    button(text("Atender")).on_press(Message::ChangeStatusTermo(item.id))
            ])

        }).into()
}


pub fn main() -> iced::Result {
    iced::run(update, view)
}
