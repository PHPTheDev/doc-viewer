use api_models::termos::models::{Setor, Status, Termo};
use iced::Element;
use iced::Task;
use iced::widget::{Column, button, column, container, keyed_column, pick_list, scrollable, text};
use reqwest::{Body, Client};

mod client;
use crate::client::termos_ep::{atender_termo, filtro_termos, get_termos};

#[derive(Debug, Clone)]
enum Message {
    PageA(MessageA),
}

trait Page {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>>;
    fn view(&self) -> iced::Element<'_, Message>;
}

struct MyApp {
    page: Box<dyn Page>,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                page: Box::new(PageA::new()),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        let page = self.page.update(message);
        if let Some(p) = page {
            self.page = p;
        }
    }

    fn view(&self) -> iced::Element<Message> {
        self.page.view()
    }
}

#[derive(Debug, Clone)]
enum MessageA {
    GetFetch,
    ChangeStatusTermo(i64),
    GetNew(Setor),
    //GetFetched(String),
}

#[derive(Default, /*PartialEq,*/ Clone)]
struct PageA {
    data: Vec<Termo>,
    filtro: Option<Setor>,
}

impl PageA {
    fn new() -> Self {
        Self {
            data: Vec::<Termo>::new(),
            filtro: None,
        }
    }

    fn iter_data(&self) -> Element<'_, Message> {
        self.data
            .iter()
            .fold(column![], |col, item| {
                let counter = 1;
                col.push(column![
                    text(format!("nome: {}", item.nome.clone())),
                    text(item.rf.clone()),
                    text(item.status.as_str().clone()),
                    text(item.qtd.clone()),
                    button(text("Atender"))
                        .on_press(Message::PageA(MessageA::ChangeStatusTermo(item.id)))
                ])
            })
            .into()
    }
}

impl Page for PageA {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        //-> Task<Message>
        if let Message::PageA(msg) = message {
            match msg {
                MessageA::GetFetch => {
                    self.data = get_termos();
                }
                MessageA::ChangeStatusTermo(val) => {
                    atender_termo(val);
                    self.data = get_termos();
                }
                MessageA::GetNew(setor) => {
                    self.filtro = Some(setor.clone());
                    self.data = filtro_termos(setor);
                }
            }
        }
        None
    }

    fn view(&self) -> Element<'_, Message> {
        let button = button(text("pesquisar")).on_press(Message::PageA(MessageA::GetFetch));
        let dt = self.iter_data();
        let setores = [Setor::FISC, Setor::SUG];
        let termos_get = keyed_column(self.data.iter().enumerate().map(|(i, termo)| {
            (
                i,
                text!("Nome: {0} Setor: {1}", termo.nome, termo.setor).into(),
            )
        }));

        let container = container(termos_get)
            .style(container::rounded_box)
            .height(400)
            .width(400);
        //let receba = pick_list(state.filtro, Some(&setores), Setor::to_string).on_select(Message::GetNew).padding(5);

        let interface = scrollable(column![button, /*receba,*/ dt, container]).into();
        interface
    }
}

pub fn main() -> iced::Result {
    iced::application(MyApp::new, MyApp::update, MyApp::view).run()
}

