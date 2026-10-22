use api_models::{Setor, Termo};
use reqwest::StatusCode;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

use crate::widgets::pages::pages::{Action, Page};
use crate::data::user::UserSettings;
use crate::data::user::savedSettings::SavedSettings;
use crate::data::settings::Settings;
use crate::{FetchError, client::termos_ep::get_termos};
use crate::widgets::theme::theme_from_string;

#[derive(Debug, Clone)]
pub struct State {
    pub data: Vec<Termo>,
    pub filtered_data: Option<Vec<Termo>>,
    pub filtro: Option<Setor>,
    pub value: i64,
    pub settings: UserSettings,
}

impl State {
    pub async fn set_state(user_name: String) -> Result<State, FetchError>{
        let termos = get_termos().await; 

        // termos = match termos {
        //     Ok(ter) => Ok(ter),
        //     Err(e) => return Err(FetchError::Request(e.status())), 
        // };

        let savedConfig = SavedSettings::load(user_name.clone()).await.unwrap();

        let userConfig = UserSettings {
            user: user_name,
            settings: Settings {
                theme: theme_from_string(savedConfig.theme),
            },
        };

        Ok(State {
            data: termos.unwrap(),
            filtered_data: None,
            filtro: Some(Setor::All),
            settings: userConfig,
            value: 0,
        })
    }

    pub async fn re_fetch(&mut self) {
       let novos_termos = get_termos().await;
       // if let Err(e) = novos_termos {
       //     if e.status().is_some() {
       //         return println!("status code: {:?}", e.status().unwrap());
       //     } else {
       //       return println!("esse erro aq: {:?}", e);
       //     }
       //  }

       self.data = novos_termos.unwrap();
    }

}


