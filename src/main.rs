use iced::Task;
use iced::widget::{text, column, button};
use iced::Element;



#[derive(Debug, Clone)]
enum Message {
    GetFetch,
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


fn view(state: &State) -> Element<'_, Message> {
    column![
        button(text("pesquisar")).on_press(Message::GetFetch),
        text(state.text.clone()),

    ]
    .into()

}


pub fn main() -> iced::Result {
    iced::run(update, view)
}