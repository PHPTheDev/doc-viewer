


#[derive(Debug, Clone)]
pub struct Settings {
	pub theme: iced::Theme,
}

impl Settings {
	pub fn new() -> Self {
		Self {
			theme: iced::Theme::Light,
		}
	}
}