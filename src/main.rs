use std::collections::HashMap;
use iced::time::{self, Duration, Instant, milliseconds};
use iced::{Application, Element, Length, Program, Subscription, Task, Theme};
use iced::widget::{Column, button, column, row, container, keyed_column, pick_list, scrollable, text};
use reqwest::{Body, Client, StatusCode};
use api_models::termos::models::{Setor, Status, Termo};

mod client;
mod data;
mod widgets;
use crate::data::state::{State};
use crate::client::termos_ep::{get_termos, atender_termo, filtro_termos};
use crate::widgets::pages::adm::{MessageAdm, Adm};
use crate::widgets::pages::login::{MessageLogin, Login};
use crate::widgets::pages::pedidos::{MessagePedidos, Pedidos};
use crate::widgets::pages::config::{MessageConfig, Config};
use crate::widgets::pages::pages::{Action, Page};
use crate::widgets::menu::main_menu;
use crate::data::user::savedSettings::SaveError;

enum App{
    Login(Login),
    Main(Main),
}

struct Main {
    load: State,
    op: String,
    pub page: HashMap<String, Box<dyn Page>>,
}

#[derive(Debug, Clone)]
enum Message {
    Fetched(Result<State, FetchError>),
    ChangePage(String),
    ReFetch(Instant),
    Data(Result<Vec<Termo>, FetchError>),
    Saved(Result<(), SaveError>),


    PagPedidos(MessagePedidos),
    PagAdm(MessageAdm),
    PagLogin(MessageLogin),
    PagConfig(MessageConfig)
}

#[derive(Debug, Clone)]
enum FetchError {
    TimeOut,
    Request(Option<StatusCode>),
}

impl App {

    fn new() -> (Self, Task<Message>) {
        (
            Self::Login(Login::new()),
            Task::none()
        )
    } 

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            App::Login(log) => match message {
                    Message::PagLogin(msg) => log.update(msg),
                    Message::Fetched(Ok(state)) => {
                        *self = App::Main(Main {load: state, op: String::from("Pedidos"),page: HashMap::from([
                                (String::from("Pedidos"), Box::new(Pedidos::new()) as Box<dyn Page>),
                                (String::from("Adm"), Box::new(Adm::new()) as Box<dyn Page>),
                                (String::from("Config"), Box::new(Config::new()) as Box<dyn Page>)
                                ])
                        });
                        
                        Task::none()
                    } 
                    Message::Fetched(Err(e)) => {
                        println!("{:?}", e);
                        // *self = App::Main(Main {load: state, page: Box::new(Pedidos::new())});
                        Task::none()
                    }
                    _ => Task::none(),
                }, 
                App::Main(main) => main.update(message),
            }
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = match self {
            App::Main(_) =>  time::every(milliseconds(5000)).map(Message::ReFetch), 
            _ => Subscription::none(),
        };

        Subscription::batch(vec![tick])
    }

    fn view(&self) -> Element<'_, Message> {
        match self {
            App::Login(l) => l.view(),
            App::Main(m) => {
                let menu = column![main_menu(&m.op)];
                let pag = m.page.get(&m.op);
                let contents = pag.unwrap().view(&m.load);

                scrollable(column![menu, contents]).into()

            }
        }
    }

    fn theme(&self) -> iced::Theme {
        match self {
            App::Login(l) => iced::Theme::Light,
            App::Main(m) => {
                m.load.settings.settings.theme.clone()//.unwrap_or(iced::Theme::Light)
            } 
        }
    }
}

impl Main {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ReFetch(_) => {
                print!("tong");
                Task::none()
            }
            Message::Fetched(Ok(state)) => {
                self.load = state;
                Task::none()
            }
            Message::Fetched(Err(e)) => {
                println!("{e:?}");
                Task::none()
            }
            Message::ChangePage(p) => {
                self.op = p;
                Task::none()
            }
            Message::Saved(_result) => {
                Task::none()
            }

            other => {
                let page = self.page.get_mut(&self.op);

                let Some(pag) = page else {
                    return Task::none();
                };

                match pag.update(other, &mut self.load) {
                    Action::None => Task::none(),
                    Action::Command(task) => task,
                    _ => Task::none()
                }
            }
        }
    }
}


// fn iter_data(state: &State) -> Element<'_, Message> {
//     state.data
//         .iter()
//         .fold(column![], |col, item| {
//                 let counter = 1;
//                 col.push(column![
//                     text(format!("nome: {}" ,item.nome.clone())),
//                     text(item.rf.clone()),
//                     text(item.status.as_str().clone()),
//                     text(item.qtd.clone()),
//                     button(text("Atender")).on_press(Message::ChangeStatusTermo(item.id))
//             ])
//
//         }).into()
// }

fn application() -> Application<impl Program<Message = Message, Theme = Theme>> {
    iced::application(App::new, App::update, App::view)
        .title("receba")
        .subscription(App::subscription)
        .theme(App::theme)
}

pub fn main() -> iced::Result {
    application().run()
}

