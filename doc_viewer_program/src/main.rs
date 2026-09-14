use iced;
use iced::Task;
use iced::widget::{Column, button, column, text};
pub mod data;
pub mod saver;
use crate::saver::{save_as_json, get_from_json};
use crate::data::tree::build_tree;

 
const ROOT_PATH: &str = r"C:\Users\x558899\Documents\Termos de Folhas scaneadas";

#[derive(Default)]
struct App{
    value: i32,
}

#[derive(Debug, Clone)]
enum Message{
    Increment,
}

impl App{

    fn update(&mut self, message: Message) -> Task<Message>{
        match message{
            Message::Increment => {
                self.value += 1;
            },
        }
        Task::none()
    }

    fn view(&self) -> Column<'_, Message> {
        let increment = button("+").on_press(Message::Increment);

        let counter = text(self.value);

        let interface = column![increment, counter];
        interface
    } 
}



fn main() -> iced::Result {
    iced::run(App::update, App::view)
    //match build_tree(&ROOT_PATH.to_string()) {
    //    Ok((tree, _root_id)) => save_as_json(tree, "p1.json"),
    //    Err(e) => eprintln!("Failed to walk directory: {e}"),
    //}
    //get_from_json().expect("nn foi");
}