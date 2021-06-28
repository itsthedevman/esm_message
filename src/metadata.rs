use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Metadata {
    Empty(crate::Empty)
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata::Empty(crate::Empty::new())
    }
}
