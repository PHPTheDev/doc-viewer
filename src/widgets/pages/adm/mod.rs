use crate::widgets::pages::pages::{Action, Page};
//use crate::widgets::pedidos::MessagePedidos;
use crate::Message;
use crate::data::state::State;
use iced::{Element, Length};
use iced::widget::{Column, button, column, container, keyed_column, pick_list, row, scrollable, shader, text};
// use crate::widgets::menu::main_menu;
// use crate::widgets::termo::termo;
use api_models::termos::models::{Setor, Status, Termo};
use crate::client::termos_ep::{filtro_termos};

#[derive(Debug, Clone)]
pub enum MessageAdm {
    Increment,
}

pub struct Adm {
    name: String,
    qtd_ped: i64,
}

impl Adm {
    pub fn new() -> Self {
        Self {
            name: String::from("Adm"),
            qtd_ped: 0,
        }
    }

}

impl Page for Adm {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn update(&mut self, message: Message, state: &mut State) -> Action {
        
        let Message::PagAdm(msg) = message else {
            return Action::None;
        };
        match msg {
            MessageAdm::Increment => {
                self.qtd_ped += 1;
                state.value += 1;
                Action::None
            }
            // Message::ChangeStatusTermo(val) => {
            //         state.value += val;
            //         Action::None
            // }
        } 
    }

    fn view(&self, state: &State) -> Element<'_, Message> {
            let oi = text("oi");
            let button = button(text("receba + 1")).on_press(MessageAdm::Increment);
            let valor = text(state.value);
            let outro = text(self.qtd_ped);
            // let ter = match state.filtered_data {
            //     Some(dt) => termo(&dt),
            //     None => termo(&state.data),
            // };                    
            // let con = container(ter);
            let content: Element<'_, MessageAdm> = column![oi,button,outro,valor].into();

            scrollable(content.map(Message::PagAdm)).width(Length::Fill).into()
    }
}
