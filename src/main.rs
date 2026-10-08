use iced::Task;
use iced::widget::{Column, button, column, container, keyed_column, pick_list, scrollable, text};
use iced::Element;
use reqwest::{Client, Body};
use api_models::termos::models::{Setor, Status, Termo};

mod client;
use crate::client::termos_ep::{get_termos, atender_termo, filtro_termos};



#[derive(Debug, Clone)]
enum Message {
    GetFetch,
    ChangeStatusTermo(i64),
    GetNew(Setor),
    //GetFetched(String),
}

#[derive(Default, /*PartialEq,*/ Clone)]
struct State {
    data: Vec<Termo>,
    filtro: Option<Setor>,

}


fn update(state: &mut State, message: Message) { //-> Task<Message> 
    match message {
        Message::GetFetch => {
            state.data = get_termos();


        }
        Message::ChangeStatusTermo(val) => {
            atender_termo(val);
            state.data = get_termos();
        }
        Message::GetNew(setor) => {
            state.filtro = Some(setor.clone());
            state.data = filtro_termos(setor);
        }
        //Task::perform(
                //get_termos(),
                //Message::GetFetched
            //),
        //Message::GetFetched(texto) => {
         //   state.text = texto;

          //  Task::none()

        //}



    }



}

fn view(state: &State) -> Element<'_, Message> {
    let button = button(text("pesquisar")).on_press(Message::GetFetch);
    let dt = iter_data(state);
    let setores = [
        Setor::FISC, 
        Setor::SUG,
    ];
    let termos_get = 
    keyed_column(state.data.iter().enumerate().map(|(i,termo)| (i, text!("Nome: {0} Setor: {1}", termo.nome, termo.setor).into())));


   
    let container = container(termos_get).style(container::rounded_box).height(400).width(400);
    //let receba = pick_list(state.filtro, Some(&setores), Setor::to_string).on_select(Message::GetNew).padding(5);

        let interface = 
        scrollable(
            column![button, /*receba,*/ dt, container])
        .into();
        interface

}

fn iter_data(state: &State) -> Element<'_, Message> {
    state.data
        .iter()
        .fold(column![], |col, item| {
                let counter = 1;
                col.push(column![
                    text(format!("nome: {}" ,item.nome.clone())),
                    text(item.rf.clone()),
                    text(item.status.as_str().clone()),
                    text(item.qtd.clone()),
                    button(text("Atender")).on_press(Message::ChangeStatusTermo(item.id))
            ])

        }).into()
}


pub fn main() -> iced::Result {
    iced::run(update, view)
}