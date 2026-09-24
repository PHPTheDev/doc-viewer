use serde::{Serialize, Deserialize};
use http_body::Body;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
pub struct Termo {
	pub nome: String,
  pub rf: String,
	pub data: String,
	pub qtd: u8,
  pub status: Status,
	pub setor: Setor,
  pub user: i64,
}

impl Termo {
	pub fn new() -> Self{
		Termo {
			nome: String::new(),
      rf: String::new(),
			data: String::new(),
			qtd: Default::default(),
      status: Status::PENDENTE, 
			setor: Setor::TI,

			user: Default::default(),
		}
	}
}

// impl Default for Termo {
// 	fn default() -> Termo {
// 		Termo::new()
// 	}
// }

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
	pub fn new() -> Setor {
        Setor::TI
    }
}

impl Default for Setor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
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
