use iced::Task;
use iced::{Element, Length, Border, Shadow, Color};
use iced::widget::{Row, Column, button, row, center, container, mouse_area, opaque, column, operation, space, stack,
    text, text_input, scrollable};
use tree_ds::prelude::TraversalStrategy;
pub mod data;
pub mod saver;
use crate::data::termo::Termo;
use crate::saver::{save_as_json, get_from_json};
use crate::data::tree::build_tree;

 
const ROOT_PATH: &str = r"C:\Users\x558899\Documents\Termos de Folhas scaneadas";

#[derive(Default)]
struct App<'a>{
    value: i32,
    items: Vec<String>,
    show_modal: bool,   
    lol: &'a str,

}

#[derive(Debug, Clone)]
enum Message{
    Increment,
    AddRItem,
    ItemPressed(usize),    
    ShowModal,
    HideModal,
}

fn dir_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_t, _e| button::Style {
        background: Some(iced::Background::from(iced::Color::from_rgb(
            238.0 / 255.0,
            242.0 / 255.0,
            118.0 / 255.0,
        ))),
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



impl<'a> App<'a>{



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

    async fn get_termos_db() -> Result<Vec<Termo>, reqwest::Error> {
         let termos = reqwest::get("http://httpbin.org/ip")
            .await?
            .json::<Vec<Termo>>()
            .await?;

        Ok(termos)
         
    }

    async fn get_termos() -> Column<'static, Message>{
        Self::get_termos_db().await
            .iter()
            .fold(column![], |col, item| {
                for termo in item {

                col.push(container(
                        column![
                            text(termo.nome.clone()),
                            text(termo.data.clone()),
                            button(text(termo.rf.clone())).on_press(Message::ItemPressed(1)),
                            text(termo.qtd.clone()),
                            text(termo.setor.clone()),
                        ]

                    ));
                };
                col
                

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
            Message::ShowModal => {
                self.show_modal = true;
                //operation::focus_next::<T>();
            }
            Message::HideModal => {
                self.hide_modal();
                //Task::none();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {

        let increment = button("+").on_press(Message::Increment);

        let counter = text(self.value);

        let children: Column<Message> = self.items
            .iter()
            .enumerate()
            .fold(column![], |col, (i, item)| {
                col.push(button(text(item.clone())).on_press(Message::ItemPressed(i)))
            });

       let interface = row![scrollable(Self::new_tree()), column![
            button("Add Widget").on_press(Message::AddRItem),
            center(button(text("Show Modal")).on_press(Message::ShowModal)),
            increment,
            counter,
            children,
            Self::get_termos()
       ]

            ].width(Length::Fill);

        if self.show_modal {
             let signup = container(
                column![
                    text("Sign Up").size(24),
                    column![
                        column![
                            text("Email").size(12),
                            text_input("", self.lol)
                                //.on_input(Message::Email)
                                //.on_submit(Message::Submit)
                                .padding(5),
                        ]
                        .spacing(5),
                        column![
                            text("Password").size(12),
                            text_input("", self.lol)
                                //.on_input(Message::Password)
                                //.on_submit(Message::Submit)
                                //.secure(true)
                                .padding(5),
                        ]
                        .spacing(5),
                        column![
                            text("Plan").size(12),
                            //pick_list(Some(self.plan), Plan::ALL, Plan::to_string)
                            //    .on_select(Message::Plan)
                            //    .padding(5),
                        ]
                        .spacing(5),
                        button(text("Submit")).on_press(Message::Increment),
                    ]
                    .spacing(10)
                ]
                .spacing(20),
            )
            .width(300)
            .padding(10)
            .style(container::rounded_box);



            modal(interface, signup, Message::HideModal)
        } else {
            interface.into()
        }

        //let interface = column![increment, counter];
        //interface
    }   

    fn hide_modal(&mut self) {
        self.show_modal = false;
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

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}

