use serde::{Deserialize, Serialize};
use arma_rs::{ArmaValue, ToArma, IntoArma};

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
