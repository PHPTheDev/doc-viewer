// use crate::Message;
// use api_models::termos::models::Termo;
// use iced::Element;
// use iced::widget::{column, row,text, button};
//
// pub fn termo(termos: &Vec<Termo>) -> Element<'_, Message> {
//       termos
//           .iter()
//           .fold(row![], |col, termo| {
//               let termo = column![
//                     text(format!("nome: {}" ,termo.nome.clone())),
//                     text(termo.rf.clone()),
//                     text(termo.user),
//                     text(termo.data.clone()),
//                     text(termo.status.as_str()),
//                     text(termo.setor.as_str().clone()),
//                     text(termo.qtd),
//                     button(text("Atender")).on_press(Message::ChangeStatusTermo(termo.id))
//               ].padding(6);
//               col.push(termo)
//           }).into()
// }
