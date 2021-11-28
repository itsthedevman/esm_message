use std::collections::HashMap;

use arma_rs::{ArmaValue, ToArma, arma_value, IntoArma};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{NumberString, ArmaHashMap};

/// Attempts to retrieve a reference to the data. Panicking if the internal data does not match the provided type.
/// Usage:
///     retrieve_data!(&message, Init)
#[macro_export]
macro_rules! retrieve_data {
    ($enum:expr, $module:ident::$type:ident) => {{
        let data = match &$enum {
            $module::$type(ref v) => v.clone(),
            data => panic!("Unexpected type {:?}. Expected: {}.", data, stringify!($type))
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
    RewardResult(RewardResult),
    Sqf(Sqf),
    SqfResult(SqfResult),
}

impl Default for Data {
    fn default() -> Self {
        Data::Empty
    }
}

impl ToArma for Data {
    fn to_arma(&self) -> ArmaValue {
        match self {
            Data::Empty => arma_value!({}),
            Data::Test(t) => t.to_arma(),
            Data::Init(i) => i.to_arma(),
            Data::PostInit(pi) => pi.to_arma(),
            Data::Query(q) => q.to_arma(),
            Data::QueryResult(qr) => qr.to_arma(),
            Data::Reward(r) => r.to_arma(),
            Data::RewardResult(r) => r.to_arma(),
            Data::SendToChannel(d) => d.to_arma(),
            Data::Sqf(s) => s.to_arma(),
            Data::SqfResult(s) => s.to_arma(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct Test {
    pub foo: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct Reward {
    pub items: Vec<ArmaHashMap>,
    pub locker_poptabs: NumberString,
    pub player_poptabs: NumberString,
    pub respect: NumberString,
    pub vehicles: Vec<ArmaHashMap>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct RewardResult {
    pub receipt: ArmaHashMap,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct Sqf {
    pub execute_on: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct SqfResult {
    pub result: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct Query {
    pub arguments: HashMap<String, String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct QueryResult {
    pub results: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, IntoArma)]
pub struct SendToChannel {
    pub id: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Message, Metadata, Type, data, metadata};

    #[test]
    fn test_retrieve_data() {
        let mut message = Message::new(Type::Test);
        message.data = Data::Test(data::Test { foo: "testing".into() });
        message.metadata = Metadata::Test(metadata::Test { foo: "testing".into() });

        let result = retrieve_data!(&message.data, Data::Test);
        assert_eq!(result.foo, String::from("testing"));

        let result = retrieve_data!(&message.metadata, Metadata::Test);
        assert_eq!(result.foo, String::from("testing"));
    }
}
