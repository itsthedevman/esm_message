use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Data {
    Empty,
    ServerInitialization(ServerInitialization)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerInitialization {
    pub server_name: String,
    pub price_per_object: i64,
    pub territory_lifetime: i64,
    pub territory_data: String,
    pub server_start_time: DateTime<Utc>
}
