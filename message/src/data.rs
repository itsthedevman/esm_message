use std::collections::HashMap;

use crate::NumberString;
use arma_rs::{FromArma, IntoArma, Value as ArmaValue};
use chrono::{DateTime, Utc};
use message_proc::ImplIntoArma;
use serde::{Deserialize, Serialize};

/// Attempts to retrieve a reference to the data. Panicking if the internal data does not match the provided type.
/// Usage:
///     retrieve_data!(&message, Init)
#[macro_export]
macro_rules! retrieve_data {
    ($enum:expr, $module:ident::$type:ident) => {{
        let data = match &$enum {
            $module::$type(ref v) => v.clone(),
            data => panic!(
                "Unexpected type {:?}. Expected: {}.",
                data,
                stringify!($type)
            ),
        };

        data
    }};
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Data {
    Empty,
    Test(Test),

    // Init
    Init(Init),
    PostInit(PostInit),

    // Query
    Query(Query),
    QueryResult(QueryResult),

    // From Client
    SendToChannel(SendToChannel),

    // Arma
    Reward(Reward),
    Sqf(Sqf),
    SqfResult(SqfResult),
}

impl Default for Data {
    fn default() -> Self {
        Data::Empty
    }
}

impl IntoArma for Data {
    fn to_arma(&self) -> ArmaValue {
        match self {
            Data::Empty => ArmaValue::Null,
            Data::Test(t) => t.to_arma(),
            Data::Init(i) => i.to_arma(),
            Data::PostInit(pi) => pi.to_arma(),
            Data::Query(q) => q.to_arma(),
            Data::QueryResult(qr) => qr.to_arma(),
            Data::Reward(r) => r.to_arma(),
            Data::SendToChannel(d) => d.to_arma(),
            Data::Sqf(s) => s.to_arma(),
            Data::SqfResult(s) => s.to_arma(),
        }
    }
}

impl FromArma for Data {
    fn from_arma(string: String) -> Result<Self, String> {
        crate::parser::Parser::from_arma(string)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Test {
    pub foo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Init {
    pub extension_version: String,
    pub price_per_object: NumberString,
    pub server_name: String,
    pub server_start_time: DateTime<Utc>,
    pub territory_data: String,
    pub territory_lifetime: NumberString,
    pub vg_enabled: bool,
    pub vg_max_sizes: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct PostInit {
    pub extdb_path: String,
    pub gambling_modifier: NumberString,
    pub gambling_payout: NumberString,
    pub gambling_randomizer_max: NumberString,
    pub gambling_randomizer_mid: NumberString,
    pub gambling_randomizer_min: NumberString,
    pub gambling_win_chance: NumberString,
    pub logging_add_player_to_territory: bool,
    pub logging_channel_id: String,
    pub logging_demote_player: bool,
    pub logging_exec: bool,
    pub logging_gamble: bool,
    pub logging_modify_player: bool,
    pub logging_pay_territory: bool,
    pub logging_promote_player: bool,
    pub logging_remove_player_from_territory: bool,
    pub logging_reward: bool,
    pub logging_transfer: bool,
    pub logging_upgrade_territory: bool,
    pub max_payment_count: NumberString,
    pub territory_admins: Vec<String>,
    pub territory_payment_tax: NumberString,
    pub territory_upgrade_tax: NumberString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Reward {
    pub items: Option<HashMap<String, NumberString>>,
    pub locker_poptabs: Option<NumberString>,
    pub player_poptabs: Option<NumberString>,
    pub respect: Option<NumberString>,
    pub vehicles: Option<Vec<HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Sqf {
    pub execute_on: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct SqfResult {
    pub result: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Event {
    pub event_type: String,
    pub triggered_at: DateTime<Utc>,
}

// territory
//   - territory_id: Returns a single territory that matches this ID
// territories:
//   - uid: Returns any territories the target uid is a part of
//   - (no arguments): Lists all territories
// player_info_account_only
// leaderboard
// leaderboard_deaths
// leaderboard_score
// restore
// reset_player
// reset_all
// get_territory_id_from_hash
// set_custom_territory_id
// get_hash_from_id
// get_payment_count
// increment_payment_counter
// reset_payment_counter
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct Query {
    pub arguments: HashMap<String, String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct QueryResult {
    pub results: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ImplIntoArma)]
pub struct SendToChannel {
    pub id: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data, metadata, Message, Metadata, Type};

    #[test]
    fn test_retrieve_data() {
        let mut message = Message::new(Type::Test);
        message.data = Data::Test(data::Test {
            foo: "testing".into(),
        });
        message.metadata = Metadata::Test(metadata::Test {
            foo: "testing".into(),
        });

        let result = retrieve_data!(&message.data, Data::Test);
        assert_eq!(result.foo, String::from("testing"));

        let result = retrieve_data!(&message.metadata, Metadata::Test);
        assert_eq!(result.foo, String::from("testing"));
    }

    #[test]
    fn test_from_arma() {
        let data = vec!["test".to_arma(), vec![vec!["foo"], vec!["bar"]].to_arma()]
            .to_arma()
            .to_string();

        match Data::from_arma(data) {
            Ok(d) => match d {
                Data::Test(t) => assert_eq!(t.foo, String::from("bar")),
                t => panic!("Failed parse: {t:?}"),
            },
            Err(e) => panic!("{e}"),
        }
    }
}
