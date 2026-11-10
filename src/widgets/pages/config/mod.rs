use iced::{Element, Theme, Length, Task};
use iced::widget::{Column, button, column, container, keyed_column, pick_list, row, scrollable, shader, text};
use api_models::termos::models::{Setor, Status, Termo};

use crate::widgets::pages::pages::{Action, Page};
use crate::Message;
use crate::data::state::State;
use crate::client::termos_ep::{filtro_termos};
use crate::widgets::theme::string_from_theme;
use crate::data::user::savedSettings::SavedSettings;

#[derive(Debug, Clone)]
pub enum MessageConfig {
    ToggleTheme(Theme),
}

pub struct Config {
	name: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
        	name: String::from("Config")
        } 
    }
}

impl Page for Config {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn update(&mut self, message: Message, state: &mut State) -> Action {
        
        let Message::PagConfig(msg) = message else {
            return Action::None;
        };

        match msg {
            MessageConfig::ToggleTheme(theme) => {
                state.settings.settings.theme = theme;
                
                Action::Command(Task::perform(
                    SavedSettings {
                            theme: string_from_theme(&state.settings.settings.theme),
                        }
                        .save(state.settings.user.clone()),
                Message::Saved))
            }
        } 
    }

    fn view(&self, state: &State) -> Element<'_, Message> {

			let setores = [Theme::Light, Theme::Dark];

            let pick = pick_list(setores, Some(state.settings.settings.theme.clone()), MessageConfig::ToggleTheme);

            let content: Element<'_, MessageConfig> = column![pick].into();

            scrollable(content.map(Message::PagConfig)).width(Length::Fill).into()
    }
}
