pub mod termo {

//use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Termo {
	nome: String,
	data: String,
	qtd: u8,
	setor: Setor,
}

impl Termo {
	pub fn new() -> Self{
		Termo {
			nome: String::new(),
			data: String::new(),
			qtd: Default::default(),
			setor: Setor::TI,
		}
	}
	// add code here
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

}
