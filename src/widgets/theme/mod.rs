use iced::Theme;

pub fn theme_from_string(st: String) -> Theme {
	match st {
        st if st == "Light".to_string() => Theme::Light,
        st if st == "Dark".to_string() => Theme::Dark,
        _ => Theme::Light,
    }
}

pub fn string_from_theme(theme: &Theme) -> String {
	match theme {
            Theme::Light => String::from("Light"),
            Theme::Dark => String::from("Dark"),
            _ => "Light".to_string(),
        }
}