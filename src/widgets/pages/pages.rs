use crate::Message;
use crate::data::state::State;
use iced::Task;

pub trait Page: Send + Sync {
    fn get_name(&self) -> String;
    fn update(&mut self, message: Message, state: &mut State) -> Action; 
    fn view(&self, state: &State) -> iced::Element<'_, Message>;
}

pub enum Action {
    Command(Task<Message>),
    GoTo(Box<dyn Page>),
    None,
}


