use iced::{Element, Center};
use iced::widget::{row, text, button};

use crate::Message;

pub fn main_menu(curr_op: &String) -> Element<'_, Message> {

    let menu_button = |label, op, curr_op| {
        let label = text(label);

        let button = button(label).style(if op == curr_op{
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::ChangePage(op)).padding(8)
    };

    row![
        menu_button("Pedidos", "Pedidos".to_string(), curr_op.clone()),
        menu_button("Adm", "Adm".to_string(), curr_op.clone()),
        menu_button("Config", "Config".to_string(), curr_op.clone()),
        // menu_button("Config", MenuOptions::Config, curr_op.clone()),
    ]
        .spacing(20)
        .align_y(Center)
        .into()
}
