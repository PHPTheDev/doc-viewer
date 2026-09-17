use serde::{Deserialize, Serialize};
use pdfrs::builder::PdfBuilder;
use pdfrs::pdf_generator::PageLayout;
use axum::Json;
use axum::response::IntoResponse;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub nome: String,
    pub rf: String,
    pub setor: String, 
}

// impl IntoResponse for CreateUser {
// 	fn into_response(self) -> Response<Body> { todo!() }
// }

pub async fn termo_pdf(i: &CreateUser) -> Vec<u8>  {
	PdfBuilder::new()
	.with_layout(PageLayout::landscape())
	.with_margins(72.0)
	.add_heading("termo",1)
	.add_paragraph(format!("nome={}, rf={}, setor={}",i.nome,i.rf,i.setor).as_str())
	.build_bytes()
	.unwrap()


}

pub async fn termoalize(data: &(&String, &String, &String)) -> Json<CreateUser> {
	
	let user = CreateUser {
		nome: (&data.0).to_string(),
		rf: (&data.1).to_string(),
		setor: (&data.2).to_string(),
	};

	Json(user)
}
