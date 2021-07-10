use serde::{Deserialize, Serialize};
use arma_rs::{ArmaValue, ToArma, arma_value};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Metadata {
    Empty(crate::Empty)
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata::Empty(crate::Empty::new())
    }
}

impl ToArma for Metadata {
    fn to_arma(&self) -> ArmaValue {
        match self {
            Metadata::Empty(_) => arma_value!({}),
            md => md.to_arma()
        }
    }
}
