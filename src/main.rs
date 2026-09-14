use iced;
use iced::Task;
use iced::{Element, Length, Border, Shadow};
use iced::widget::{Row, Column, button, row, column, text, scrollable};
use tree_ds::prelude::*;
pub mod data;
pub mod saver;
use crate::saver::{save_as_json, get_from_json};
use crate::data::tree::build_tree;

 
const ROOT_PATH: &str = r"C:\Users\x558899\Documents\Termos de Folhas scaneadas";

#[derive(Default)]
struct App{
    value: i32,
    items: Vec<String>,

}

#[derive(Debug, Clone)]
enum Message{
    Increment,
    AddRItem,
    ItemPressed(usize),
}

fn dir_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_t, _e| button::Style {
        background: None,
        text_color: iced::Color::from_rgb(
            3.0 / 255.0,
            161.0 / 255.0,
            252.0 / 255.0,
        ),
        border: Border::default(),
        shadow: Shadow::default(),
        snap: true,
    }
}

impl App{



    fn new_tree() -> Column<'static, Message>{
        let (tree, root_id) = build_tree(&ROOT_PATH.to_string()).unwrap();

        tree.traverse(&root_id, TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .fold(column![], |col, item| {
                let node = tree.get_node_by_id(item).unwrap();
                let term = node.get_value().unwrap();
                let name = term.clone().unwrap().path;

                    if term.unwrap().dir {
                        col.push(button(text(name.clone()))    
                            .style(dir_button_style())
                            .on_press(Message::ItemPressed(1)))
                    } else {
                        col.push(button(text(name.clone())).on_press(Message::ItemPressed(1)))
                    }

            })
    }

    fn update(&mut self, message: Message) -> Task<Message>{
        match message{
            Message::Increment => {
                self.value += 1;
            },
            Message::AddRItem => {
                self.items.push(format!("Item {}", self.items.len() + 1));
            }
            Message::ItemPressed(index) => {
                // Handle individual dynamic widget events
                ();
            }
        }
        Task::none()
    }

    fn view(&self) -> Row<'_, Message> {

        let increment = button("+").on_press(Message::Increment);

        let counter = text(self.value);

        let children: Column<Message> = self.items
            .iter()
            .enumerate()
            .fold(column![], |col, (i, item)| {
                col.push(button(text(item.clone())).on_press(Message::ItemPressed(i)))
            });

       row![scrollable(Self::new_tree()), column![
            button("Add Widget").on_press(Message::AddRItem),
            increment,
            counter,
            children]

            ].width(Length::Fill)



        //let interface = column![increment, counter];
        //interface
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