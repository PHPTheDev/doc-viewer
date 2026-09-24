use axum::extract::rejection::StringRejection;
use axum::Json;
use pdfrs::pdf_generator::PageLayout;
use pdfrs::builder::PdfBuilder;
use sqlite::ffi::SQLITE_DBCONFIG_ENABLE_TRIGGER;
use serde::{Deserialize, Serialize};
use std::string::String;

use crate::users::models::User;
//use crate::termos::parser::Parse;

#[derive(Serialize, Deserialize)]
pub struct Termo {
	pub nome: String,
    pub rf: String,
	pub data: String,
	pub qtd: i64,
	pub setor: Setor,
    pub status: Status,
    pub user: i64,
}

impl Default for Termo {
    fn default() -> Self {
        Self::new() 
    } 
}

impl Termo {
    fn new() -> Termo {
        Termo { nome: String::new(), rf: String::new(),data: String::new(), qtd: Default::default(), setor: Setor::TI, status: Status::PENDENTE ,user: Default::default(), }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Setor {
	TI,
	SUG,
	STM,
	SAS,
	SA,
	GL,
	GAB,
	CPO,
	STF,
	SUSL,
	UNAI,
	CAF,
	FISC,
}

impl Setor {
    pub fn as_str(&self) -> String{
        match self {
            Setor::FISC => String::from("Fisc"),
            Setor::SUG => String::from("Sugesp"),
            // Setor::TI
            // Setor::STM
            // Setor::SA
            // Setor::GL
            // Setor::SAS
            // Setor::GAB
            // Setor::CPO
            // setor
            _ => "setor invalido ou nn colocado".to_string(),
        }
    }

    pub fn parse(value: String) -> Self {
        match value {
            value if value == "Fisc".to_string() => Setor::FISC,
            value if value == "Sugesp".to_string() => Setor::SUG,
            _ => Setor::default(),
        }
    }

    pub fn new() -> Setor {
        Setor::TI
    }
}

impl Default for Setor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    PENDENTE,
    ATENDIDO,
    RECUSADO,
} 

impl Status {

    pub fn as_str(&self) -> &str{
        match self {
            Status::PENDENTE => "Pendente", 
            Status::ATENDIDO => "Atendido",
            Status::RECUSADO => "Recusado",
        }
    }

    pub fn parse(value: String) -> Self {
        match value {
            value if value == "Pendente".to_string() => Status::PENDENTE,
            value if value == "Atendido".to_string() => Status::ATENDIDO,
            value if value == "Recusado".to_string() => Status::RECUSADO,
            _ => Status::default(),
        }
    }

    pub fn new() -> Status {
        Status::PENDENTE
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn termo_pdf(i: &Termo) -> Vec<u8>  {
	PdfBuilder::new()
	.with_layout(PageLayout::landscape())
	.with_margins(72.0)
	.add_heading("termo",1)
	.add_paragraph(format!("nome={}, rf={}, setor={:?}",i.nome,i.rf,i.setor).as_str())
	.build_bytes()
	.unwrap()


}
