use iced::{Element, widget::{button,column, text_input}};
use sqlite;

#[derive (Debug, Clone)]
enum Message {
    ContentChanged(String),
    Send,
}


#[derive(Default)]
struct State {
    content: String,
}


fn view(state: &State) -> Element<'_,Message> {
    column![
        text_input("nome", &state.content).on_input(Message::ContentChanged),
        button("Enviar").on_press(Message::Send),
    ].into()
}


fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }

        Message::Send => {
            let connection = sqlite::open("gaster.db").unwrap();
            let mut query1 = String::from("INSERT INTO teste (nome) VALUES('')");
            query1.insert_str(33, &state.content.to_string());
            println!("{query1}");
            connection.execute(
            "CREATE TABLE IF NOT EXISTS teste (
            nome TEXT   
            );").unwrap();
            connection.execute(query1).unwrap();
        }

    }

}



pub fn main() -> iced::Result {
    iced::run(update, view)
}
