use serde::{Deserialize, Serialize};
use arma_rs::{ArmaValue, ToArma, arma_value};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Metadata {
    Empty,
    Test(Test),
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata::Empty
    }
}

impl ToArma for Metadata {
    fn to_arma(&self) -> ArmaValue {
        match self {
            Metadata::Empty => arma_value!({}),
            md => md.to_arma()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Test {
    pub foo: String
}

impl ToArma for Test {
    fn to_arma(&self) -> ArmaValue {
        arma_value!({ "foo": self.foo })
    }
}
