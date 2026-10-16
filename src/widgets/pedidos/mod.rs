use crate::widgets::pages::Action;
use crate::{Message, widgets::pages::Page};
use crate::data::state::State;
use iced::{Element, Length};
use iced::widget::{Column, button, column, container, keyed_column, pick_list, row, scrollable, shader, text};
// use crate::widgets::menu::main_menu;
// use crate::widgets::termo::termo;
use api_models::termos::models::{Setor, Status, Termo};
use crate::client::termos_ep::{filtro_termos};

#[derive(Debug, Clone)]
pub enum MessagePedidos {
    ChangeStatusTermo(i64),
    FetchFilter(Setor),
}

pub struct Pedidos {
    name: String,
    qtd_ped: i64,
}

impl Pedidos {
    pub fn new() -> Self {
        Self {
            name: String::from("Pedidos"),
            qtd_ped: 0,
        }
    }

}

impl Page for Pedidos {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn update(&mut self, message: Message, state: &mut State) -> Action {
        
        let Message::PagPedidos(msg) = message else {
            return Action::None;
        };
        match msg {
            MessagePedidos::FetchFilter(filter) => {
                state.filtro = Some(filter);
                match filter {
                    Setor::All => {
                        state.filtered_data = None;
                        Action::None
                    }
                    _ => {
                        state.filtered_data = Some(filtro_termos(filter));
                        Action::None
                    }
                }
            }
            _ => Action::None
            // Message::ChangeStatusTermo(val) => {
            //         state.value += val;
            //         Action::None
            // }
        } 
    }

    fn view(&self, state: &State) -> Element<'_, Message> {
            let oi = text("oi");
            let button = button(text("receba + 1")).on_press(MessagePedidos::ChangeStatusTermo(1));
            let valor = text(state.value);
            // let ter = match state.filtered_data {
            //     Some(dt) => termo(&dt),
            //     None => termo(&state.data),
            // };                    
            // let con = container(ter);

            let setores = [Setor::All, Setor::FISC, Setor::SUG];


            let pick = pick_list(setores, state.filtro, MessagePedidos::FetchFilter);
    
            let content: Element<'_, MessagePedidos> = column![oi,button, pick,valor].into();

            scrollable(content.map(Message::PagPedidos)).width(Length::Fill).into()
    }
}
