use arma_rs::{ArmaValue, ToArma, arma_value};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Attempts to retrieve a reference to the data. Panicking if the internal data does not match the provided type.
/// Usage:
///     retrieve_data!(&message, Init)
#[macro_export]
macro_rules! retrieve_data {
    ($message:expr, $data_type:ident) => {{
        let data = match &$message.data {
            Data::$data_type(ref v) => v,
            data => panic!("Unexpected data type {:?}. Expected: {}.", data, stringify!($data_type))
        };

        data
    }};
}

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

impl ToArma for Data {
    fn to_arma(&self) -> ArmaValue {
        match self {
            Data::Empty(_) => arma_value!({}),
            d => d.to_arma()
        }
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
impl ToArma for Init {
    fn to_arma(&self) -> ArmaValue {
        arma_value!({
            "server_name": self.server_name,
            "price_per_object": self.price_per_object,
            "territory_lifetime": self.territory_lifetime,
            "territory_data": self.territory_data,
            "server_start_time": self.server_start_time
        })
    }
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
    // For now
    // pub reward_player_poptabs: i64,
    // pub reward_locker_poptabs: i64,
    // pub reward_respect: i64,
    // pub reward_items: HashMap<String, i64>,
}

impl ToArma for PostInit {
    fn to_arma(&self) -> ArmaValue {
        arma_value!({
            "extdb_path": self.extdb_path,
            "gambling_modifier": self.gambling_modifier,
            "gambling_payout": self.gambling_payout,
            "gambling_randomizer_max": self.gambling_randomizer_max,
            "gambling_randomizer_mid": self.gambling_randomizer_mid,
            "gambling_randomizer_min": self.gambling_randomizer_min,
            "gambling_win_chance": self.gambling_win_chance,
            "logging_add_player_to_territory": self.logging_add_player_to_territory,
            "logging_demote_player": self.logging_demote_player,
            "logging_exec": self.logging_exec,
            "logging_gamble": self.logging_gamble,
            "logging_modify_player": self.logging_modify_player,
            "logging_pay_territory": self.logging_pay_territory,
            "logging_promote_player": self.logging_promote_player,
            "logging_remove_player_from_territory": self.logging_remove_player_from_territory,
            "logging_reward": self.logging_reward,
            "logging_transfer": self.logging_transfer,
            "logging_upgrade_territory": self.logging_upgrade_territory,
            "max_payment_count": self.max_payment_count,
            "territory_payment_tax": self.territory_payment_tax,
            "territory_upgrade_tax": self.territory_upgrade_tax,
            "territory_admins": self.territory_admins
        })
    }
}
