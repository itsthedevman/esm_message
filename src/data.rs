use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Data {
    Empty(crate::Empty),
    ServerInitialization(ServerInitialization)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerInitialization {
    pub server_name: String,
    pub price_per_object: f32,
    pub territory_lifetime: f32,
    pub territory_data: String,
    pub server_start_time: DateTime<Utc>
}
