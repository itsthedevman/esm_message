use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Data {
    Empty(crate::Empty),
    Init(Init),
    PostInit(PostInit),
}

impl Default for Data {
    fn default() -> Self {
        Data::Empty(crate::Empty::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Init {
    pub server_name: String,
    pub price_per_object: f32,
    pub territory_lifetime: f32,
    pub territory_data: String,
    pub server_start_time: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostInit {
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
    pub territory_payment_tax: i64,
    pub territory_upgrade_tax: i64,
    pub territory_admins: Vec<String>,
    pub reward_player_poptabs: i64,
    pub reward_locker_poptabs: i64,
    pub reward_respect: i64,
    pub reward_items: HashMap<String, i64>, // For now
}
