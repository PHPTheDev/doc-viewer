use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::{ConnectOptions};


#[derive(Clone)]
pub struct ApiState {
	pub connection: SqlitePool,
}

impl ApiState {
	pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_options = SqliteConnectOptions::from_str("sqlite:data.db")?
        .create_if_missing(true)
        .disable_statement_logging()
        .to_owned();

        let connection = SqlitePoolOptions::new().connect_with(db_options).await?;


        Ok(Self {connection}) 


  }

}
