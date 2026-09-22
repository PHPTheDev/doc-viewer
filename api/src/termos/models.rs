use axum::extract::rejection::StringRejection;
use axum::Json;
use pdfrs::pdf_generator::PageLayout;
use pdfrs::builder::PdfBuilder;
use sqlite::ffi::SQLITE_DBCONFIG_ENABLE_TRIGGER;
use serde::{Deserialize, Serialize};
use std::string::String;

use crate::users::models::User;

#[derive(Serialize, Deserialize)]
pub struct Termo {
	pub nome: String,
    pub rf: String,
	pub data: String,
	pub qtd: i64,
	pub setor: Setor,
    pub user: i64,
}

impl Default for Termo {
    fn default() -> Self {
        Self::new() 
    } 
}

impl Termo {
    fn new() -> Termo {
        Termo { nome: String::new(), rf: String::new(),data: String::new(), qtd: Default::default(), setor: Setor::TI, user: Default::default(), }
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
            val if val == "Fisc".to_string() => Setor::FISC,
            val if val == "Sugesp".to_string() => Setor::SUG,
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

pub async fn termo_pdf(i: &Termo) -> Vec<u8>  {
	PdfBuilder::new()
	.with_layout(PageLayout::landscape())
	.with_margins(72.0)
	.add_heading("termo",1)
	.add_paragraph(format!("nome={}, rf={}, setor={:?}",i.nome,i.rf,i.setor).as_str())
	.build_bytes()
	.unwrap()


}
