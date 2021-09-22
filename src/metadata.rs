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
            d => d.to_arma()
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
    pub steam_uid: String,
    pub discord_id: String,
    pub discord_name: String,
    pub discord_mention: String,
}
