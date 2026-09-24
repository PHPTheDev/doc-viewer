//#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::string::String;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub rf: i64,
    pub senha: String,
    pub is_admin: u8,
}

impl User {
    fn new() -> User {
        User {
            rf: Default::default(),
            senha: String::new(),
            is_admin: Default::default(),
        }   // add code here
    }
}

