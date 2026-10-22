
use crate::data::settings::Settings;

pub mod savedSettings;

#[derive(Debug, Clone)]
pub struct UserSettings {
	pub user: String,
	pub settings: Settings,
}