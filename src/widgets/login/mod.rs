use crate::Message;
use crate::data::state::State;
use iced::Task;
use iced::widget::{Column, button, column, container, keyed_column, pick_list, row, scrollable, shader, text, text_input};
// use crate::widgets::menu::main_menu;
// use crate::widgets::termo::termo;
use api_models::termos::models::{Setor, Status, Termo};
use crate::client::termos_ep::{filtro_termos};

#[derive(Debug, Clone)]
pub enum MessageLogin {
    TextChangeg(String),
    ButtonPressed,
}

pub struct Login {
    rf: String,
}

impl Login {
    pub fn new() -> Self {
        Self {
            rf: String::new(),
        }
    }

}

impl Login {
    pub fn update(&mut self, message: MessageLogin) -> Task<Message>{
        match message {
            MessageLogin::TextChangeg(val) => { 
                self.rf = val;
                Task::none()
            }
            MessageLogin::ButtonPressed => {
                Task::perform(State::set_state(), Message::Fetched)
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        column![
            text_input("RF", &self.rf).on_input(|s| Message::PagLogin(MessageLogin::TextChangeg(s))),
            button("Log in").on_press(Message::PagLogin(MessageLogin::ButtonPressed)),
        ].into()
    }
}
