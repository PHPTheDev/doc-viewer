use api_models::{Setor, Termo};
use reqwest::StatusCode;
use crate::widgets::pages::Page;

use crate::{FetchError, client::termos_ep::get_termos};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub data: Vec<Termo>,
    pub filtered_data: Option<Vec<Termo>>,
    pub filtro: Option<Setor>,
    pub value: i64,
}

impl State {
    pub async fn set_state() -> Result<State, FetchError>{
        let termos = get_termos().await; 

        // termos = match termos {
        //     Ok(ter) => Ok(ter),
        //     Err(e) => return Err(FetchError::Request(e.status())), 
        // };

        Ok(State {
            data: termos.unwrap(),
            filtered_data: None,
            filtro: Some(Setor::All),
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


