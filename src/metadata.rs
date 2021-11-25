use serde::{Deserialize, Serialize};
use arma_rs::{ArmaValue, ToArma, IntoArma, arma_value};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Metadata {
    Empty,
    Test(Test),
    Command(Command)
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
            Metadata::Test(t) => t.to_arma(),
            Metadata::Command(c) => c.to_arma(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct Test {
    pub foo: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct Command {
    pub player: Player,
    pub target: Option<Player>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct Player {
    pub discord_id: Option<String>,
    pub discord_mention: Option<String>,
    pub discord_name: Option<String>,
    pub steam_uid: String,
}
