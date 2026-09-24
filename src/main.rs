use iced::Task;
use iced::widget::{text, column, button};
use iced::Element;
use reqwest::{Client, Body};
use api_models::termos::models::{ Status, Termo};



#[derive(Debug, Clone)]
enum Message {
    GetFetch,
    ChangeStatusTermo,
    //GetFetched(String),
}

#[derive(Default)]
struct State {
    text: String,
}


fn update(state: &mut State, message: Message) { //-> Task<Message> 
    match message {
        Message::GetFetch => {
            state.text = get_termos();


        }
        Message::ChangeStatusTermo => {
            atender_termo(1);
            state.text = get_termos();
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
async fn get_termos() -> String {
    let body = reqwest::get("http://127.0.0.1:3000/termos").await.unwrap().text().await.unwrap(); 
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
    column![
        button(text("pesquisar")).on_press(Message::GetFetch),
        text(state.text.clone()),
        button(text("Atender")).on_press(Message::ChangeStatusTermo)

    ]
    .into()

}


pub fn main() -> iced::Result {
    iced::run(update, view)
}
