use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    // Controls how the error_message is treated
    #[serde(rename = "type")]
    error_type: ErrorType,

    #[serde(rename = "message")]
    error_message: String,
}

impl Error {
    pub fn new(error_type: ErrorType, error_message: String) -> Self {
        Error { error_type, error_message }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorType {
    // Treats the error_message as a locale error code.
    Code,

    // Treats the error_message as a normal string
    Message
}
