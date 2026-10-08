use sqlx::sqlite::SqlitePool;
use sqlx::Error;

pub async fn setPool(url: &str) -> Result<SqlitePool, Error> {
	let pool = sqlx::sqlite::SqlitePool::connect(url).await?;

	Ok(pool)
}

#[derive(Clone)]
pub struct ApiState {
	pub connection: Option<SqlitePool>,
}

impl ApiState {
	pub fn new() -> Self {
		Self {
			connection: None,  
		}
	}

	pub async fn db_connection(&mut self, url: &str) {
		let con = setPool(url).await;

		let res = match con {
			Ok(c) => Some(c),
			Err(_e) => None,
		};
		println!("receba");
		self.connection = res
	}


}