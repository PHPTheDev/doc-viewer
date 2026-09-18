use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Termo {
	pub nome: String,
  pub rf: String,
	pub data: String,
	pub qtd: u8,
	pub setor: Setor,
}

impl Termo {
	pub fn new() -> Self{
		Termo {
			nome: String::new(),
      rf: String::new(),
			data: String::new(),
			qtd: Default::default(),
			setor: Setor::TI,
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct NodeTermo{
	pub termo: Termo,
	pub path: String,
	pub depth: usize,
	pub dir: bool
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
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

}
