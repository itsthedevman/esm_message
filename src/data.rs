use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Data {
    Empty(crate::Empty),
    ServerInitialization(ServerInitialization),
    ServerPostInitialization(ServerPostInitialization),
}

impl Default for Data {
    fn default() -> Self {
        Data::Empty(crate::Empty::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerInitialization {
    pub server_name: String,
    pub price_per_object: f32,
    pub territory_lifetime: f32,
    pub territory_data: String,
    pub server_start_time: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerPostInitialization {
    pub extdb_path: String,
    pub gambling_modifier: i64,
    pub gambling_payout: i64,
    pub gambling_randomizer_max: f64,
    pub gambling_randomizer_mid: f64,
    pub gambling_randomizer_min: f64,
    pub gambling_win_chance: i64,
    pub logging_add_player_to_territory: bool,
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
    pub max_payment_count: i64,
    pub server_id: String,
    pub taxes_territory_payment: i64,
    pub taxes_territory_upgrade: i64,
    pub territory_admins: Vec<String>,
}
