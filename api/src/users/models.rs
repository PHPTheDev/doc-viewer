use serde::{Deserialize};
use pdfrs::builder::PdfBuilder;
use pdfrs::pdf_generator::PageLayout;

#[derive(Deserialize)]
pub struct CreateUser {
    pub nome: String,
    pub rf: String,
    pub setor: String, 
}

pub async fn termo_pdf(i: &CreateUser) -> Vec<u8>  {
	PdfBuilder::new()
	.with_layout(PageLayout::landscape())
	.with_margins(72.0)
	.add_heading("termo",1)
	.add_paragraph(format!("nome={}, rf={}, setor={}",i.nome,i.rf,i.setor).as_str())
	.build_bytes()
	.unwrap()


}

