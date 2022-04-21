use arma_rs::{FromArma, IntoArma, Value as ArmaValue};
use message_proc::ImplIntoArma;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Metadata {
    Empty,
    Test(Test),
    Command(Command),
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata::Empty
    }
}

impl IntoArma for Metadata {
    fn to_arma(&self) -> ArmaValue {
        match self {
            Metadata::Empty => ArmaValue::Null,
            Metadata::Test(t) => t.to_arma(),
            Metadata::Command(c) => c.to_arma(),
        }
    }
}

impl FromArma for Metadata {
    fn from_arma(input: String) -> Result<Self, String> {
        crate::parser::Parser::from_arma(&input)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Test {
    pub foo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Command {
    pub player: Player,
    pub target: Option<Player>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Player {
    pub discord_id: Option<String>,
    pub discord_mention: Option<String>,
    pub discord_name: Option<String>,
    pub steam_uid: String,
}
