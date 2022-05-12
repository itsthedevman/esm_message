pub mod data;
pub mod error;
pub mod metadata;
pub mod parser;

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use arma_rs::FromArma;
use message_io::network::ResourceId;
use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use data::*;
pub use error::*;
pub use metadata::*;

// Numbers in Arma are best stored as Strings when sending across the wire to avoid precision loss.
// Use this type for any numbers
pub type NumberString = String;

/*
    {
        id: "",
        type: "",
        resource_id: null,
        server_id: [],
        data: {
            type: "",
            content: {}
        },
        metadata: {
            type: "",
            content: {}
        },
        errors: []
    }
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: Uuid,

    #[serde(rename = "type")]
    pub message_type: Type,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<Vec<u8>>,

    // Only used between the server and the bot. Ignored between server/client
    #[serde(skip)]
    pub resource_id: Option<i64>,

    #[serde(default, skip_serializing_if = "data_is_empty")]
    pub data: Data,

    #[serde(default, skip_serializing_if = "metadata_is_empty")]
    pub metadata: Metadata,

    #[serde(default, skip_serializing_if = "errors_is_empty")]
    pub errors: Vec<Error>,
}

fn data_is_empty(data: &Data) -> bool {
    matches!(data, Data::Empty)
}

fn metadata_is_empty(metadata: &Metadata) -> bool {
    matches!(metadata, Metadata::Empty)
}

fn errors_is_empty(errors: &[Error]) -> bool {
    errors.is_empty()
}

impl Message {
    pub fn new(message_type: Type) -> Self {
        Message {
            id: Uuid::new_v4(),
            message_type,
            resource_id: None,
            server_id: None,
            data: Data::Empty,
            metadata: Metadata::Empty,
            errors: Vec::new(),
        }
    }

    pub fn set_resource(&mut self, resource_id: ResourceId) -> &Message {
        self.resource_id = Some(resource_id.adapter_id() as i64);
        self
    }

    pub fn add_error<S>(&mut self, error_type: ErrorType, error_message: S) -> &Message
    where
        S: Into<String>,
    {
        let error = Error::new(error_type, error_message.into());
        self.errors.push(error);
        self
    }

    pub fn from_bytes(data: Vec<u8>, key: &[u8]) -> Result<Message, String> {
        decrypt_message(data, key)
    }

    pub fn as_bytes(&self, key: &[u8]) -> Result<Vec<u8>, String> {
        encrypt_message(self, key)
    }

    //  [
    //      "id",
    //      "type",
    //      [
    //          ["type", "content"],
    //          ["data_type", [["key_1", "key_2"], ["value_1", "value_2"]]
    //      ],
    //      [
    //          ["type", "content"],
    //          ["metadata_type", [["key_1", "key_2"], ["value_1", "value_2"]]
    //      ],
    //      [
    //          [["type", "content"], ["code", "1"]],
    //          [["type", "content"], ["message", "This is an error"]]
    //      ]
    //  ]
    pub fn from_arma(
        id: String,
        message_type: String,
        data: String,
        metadata: String,
        errors: String,
    ) -> Result<Message, String> {
        // The message type has to be double quoted in order to parse
        let mut message = match serde_json::from_str(&format!("\"{}\"", message_type)) {
            Ok(t) => Self::new(t),
            Err(e) => {
                return Err(format!(
                    "\"{}\" is not a valid type. Error: {}",
                    message_type, e
                ))
            }
        };

        match Uuid::parse_str(&id) {
            Ok(uuid) => message.id = uuid,
            Err(e) => return Err(format!("Failed to extract ID from {:?}. {}", id, e)),
        };

        match Data::from_arma(data) {
            Ok(d) => message.data = d,
            Err(e) => return Err(e),
        };

        match Metadata::from_arma(metadata) {
            Ok(d) => message.metadata = d,
            Err(e) => return Err(e),
        };

        match Error::from_arma(errors) {
            Ok(e) => message.errors = e,
            Err(e) => return Err(e),
        };

        Ok(message)
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    // System message types
    Connect,
    Disconnect,
    Ping,
    Pong,
    Test,
    Error,
    Resume,
    Pause,

    ///////////////////////
    // Client message types
    ///////////////////////

    // Initialization events
    Init,

    // Client event. on_response is one example
    Event,

    // Database query.
    Query,

    // Execute a Arma function
    Arma,
}

////////////////////////////////////////////////////////////

#[allow(clippy::ptr_arg)]
fn encrypt_message(message: &Message, server_key: &[u8]) -> Result<Vec<u8>, String> {
    // Setup everything for encryption
    let encryption_key = Key::from_slice(&server_key[0..32]); // server_key has to be exactly 32 bytes
    let encryption_cipher = Aes256Gcm::new(encryption_key);
    let nonce_key: Vec<u8> = (0..12).map(|_| random::<u8>()).collect();
    let encryption_nonce = Nonce::from_slice(&nonce_key);

    /*
        Message (as bytes)
        [
            1 byte -> Size of server id (server_id_bytes)
            # of server_id_bytes -> The server id
            1 byte -> Size of Nonce (nonce_bytes)
            # of nonce_bytes -> The nonce
            rest -> The encrypted json
        ]
    */
    // Start the packet off with the id length and itself
    let server_id = message.server_id.clone().unwrap();
    let mut packet: Vec<u8> = vec![server_id.len() as u8];
    packet.extend(&*server_id);

    // Append the nonce length and itself to the packet
    packet.push(nonce_key.len() as u8);
    packet.extend(&*nonce_key);

    // Serialize this message
    let message_bytes = match serde_json::to_vec(&message) {
        Ok(bytes) => bytes,
        Err(e) => return Err(e.to_string()),
    };

    // Encrypt the message
    let encrypted_message =
        match encryption_cipher.encrypt(encryption_nonce, message_bytes.as_ref()) {
            Ok(bytes) => bytes,
            Err(e) => return Err(e.to_string()),
        };

    // Now add the encrypted message to the end. This completes the packet
    packet.extend(&*encrypted_message);

    Ok(packet)
}

fn decrypt_message(bytes: Vec<u8>, server_key: &[u8]) -> Result<Message, String> {
    // The first byte is the length of the server_id so we know how many bytes to extract
    let id_length = bytes[0] as usize;

    // Extract the server ID and convert to a vec
    let server_id = bytes[1..=id_length].to_vec();

    // Now to decrypt. First step, extract the nonce
    let nonce_offset = 1 + id_length;
    let nonce_size = bytes[nonce_offset] as usize;
    let nonce_offset = 1 + nonce_offset;
    let nonce = bytes[nonce_offset..(nonce_offset + nonce_size)].to_vec();
    let nonce = Nonce::from_slice(&nonce);

    // Next, extract the encrypted bytes
    let enc_offset = nonce_offset + nonce_size;
    let encrypted_bytes = bytes[enc_offset..].to_vec();

    // Build the cipher
    let server_key = &server_key[0..=31]; // server_key has to be exactly 32 bytes
    let key = Key::from_slice(server_key);
    let cipher = Aes256Gcm::new(key);

    // Decrypt! This also ensures the message has been encrypted using this server's key.
    let decrypted_bytes = match cipher.decrypt(nonce, encrypted_bytes.as_ref()) {
        Ok(message) => message,
        Err(e) => {
            return Err(format!("Failed to decrypt. Reason: {}", e));
        }
    };

    // And deserialize into a struct
    let mut message: Message = match serde_json::from_slice(&decrypted_bytes) {
        Ok(message) => message,
        Err(e) => {
            return Err(format!(
                "Failed to deserialize. Reason: {:?}. Message: {:#?}",
                e,
                String::from_utf8(decrypted_bytes.clone())
                    .unwrap_or(format!("Bytes: {:?}", decrypted_bytes))
            ))
        }
    };

    // Store the server id
    message.server_id = Some(server_id);

    Ok(message)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::data::Init;

    #[test]
    fn test_encrypt_and_decrypt_message() {
        let mut message = Message::new(Type::Connect);

        let server_init = Init {
            server_name: "server_name".into(),
            price_per_object: "10".into(),
            territory_lifetime: "7".into(),
            territory_data: "[]".into(),
            server_start_time: chrono::Utc::now(),
            extension_version: "2.0.0".into(),
            vg_enabled: false,
            vg_max_sizes: String::new(),
        };

        let expected = server_init.clone();

        let server_id = String::from("esm_testing");
        message.server_id = Some(server_id.as_bytes().to_vec());
        message.data = Data::Init(server_init);

        let server_key = format!(
            "{}-{}-{}-{}",
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4()
        );
        let server_key = server_key.as_bytes();

        let encrypted_bytes = encrypt_message(&message, &server_key);
        assert!(encrypted_bytes.is_ok());

        let decrypted_message = decrypt_message(encrypted_bytes.unwrap(), &server_key);
        assert!(decrypted_message.is_ok());

        let decrypted_message = decrypted_message.unwrap();
        assert_eq!(decrypted_message.message_type, Type::Connect);

        // Ensure it has a server ID
        assert!(decrypted_message.server_id.is_some());

        match decrypted_message.data {
            Data::Init(data) => {
                assert_eq!(data.server_name, expected.server_name);
                assert_eq!(data.price_per_object, expected.price_per_object);
                assert_eq!(data.territory_lifetime, expected.territory_lifetime);
                assert_eq!(data.territory_data, expected.territory_data);
            }
            _ => panic!("Invalid message data"),
        }
    }

    #[test]
    fn test_it_serializes_with_server_id() {
        let mut message = Message::new(Type::Connect);
        message.server_id = Some("some_server_id".as_bytes().to_vec());

        let serialized_message = serde_json::to_string(&message);
        assert!(serialized_message.is_ok());

        let serialized_message = serialized_message.unwrap();
        assert_eq!(serialized_message, format!("{{\"id\":\"{}\",\"type\":\"connect\",\"server_id\":[115,111,109,101,95,115,101,114,118,101,114,95,105,100]}}", message.id));
    }

    #[test]
    fn test_it_serializes_without_server_id() {
        let message = Message::new(Type::Connect);

        let serialized_message = serde_json::to_string(&message);
        assert!(serialized_message.is_ok());

        let serialized_message = serialized_message.unwrap();
        assert_eq!(
            serialized_message,
            format!("{{\"id\":\"{}\",\"type\":\"connect\"}}", message.id)
        );
    }

    #[test]
    fn test_data_is_empty() {
        let result = data_is_empty(&Data::Empty);
        assert!(result);

        let server_init = Init {
            server_name: "server_name".into(),
            price_per_object: "10".into(),
            territory_lifetime: "7".into(),
            territory_data: "[]".into(),
            server_start_time: chrono::Utc::now(),
            extension_version: "2.0.0".into(),
            vg_enabled: false,
            vg_max_sizes: String::new(),
        };

        let result = data_is_empty(&Data::Init(server_init));
        assert!(!result);
    }

    #[test]
    fn test_metadata_is_empty() {
        let result = metadata_is_empty(&Metadata::Empty);
        assert!(result);
    }

    #[test]
    fn test_errors_is_empty() {
        let result = errors_is_empty(&Vec::new());
        assert!(result);

        let error = Error::new(ErrorType::Code, "1".into());
        let result = errors_is_empty(&[error]);
        assert!(!result);
    }

    #[test]
    fn test_serializing_empty_message() {
        let message = Message::new(Type::Connect);
        let json = serde_json::to_string(&message).unwrap();

        let expected = format!("{{\"id\":\"{}\",\"type\":\"connect\"}}", message.id);
        assert_eq!(json, expected);
    }

    #[test]
    fn test_deserializing_empty_message() {
        let uuid = Uuid::new_v4();
        let input = format!("{{\"id\":\"{}\",\"type\":\"connect\"}}", uuid);
        let message: Message = serde_json::from_str(&input).unwrap();

        assert_eq!(message.id, uuid);
        assert!(matches!(message.data, Data::Empty));
        assert!(matches!(message.metadata, Metadata::Empty));
        assert!(message.errors.is_empty());
    }

    #[test]
    fn test_from_str() {
        use data::Data;

        let id = Uuid::new_v4();
        let mut expectation = Message::new(Type::Event);
        expectation.id = id;
        expectation.data = Data::Test(data::Test {
            foo: "tes\"ting".into(),
        });

        expectation.metadata = Metadata::Test(metadata::Test {
            foo: "\"testing2\" - \"\"nested\"\"".into(),
        });

        expectation.add_error(ErrorType::Message, "This is a message");
        expectation.add_error(ErrorType::Code, "CODING");

        let result = Message::from_arma(
            id.to_string(),
            "event".into(),
            json!([
                json!(["type", json!("test")]),
                json!(["content", json!([json!(["foo", "tes\"ting"])])])
            ])
            .to_string(),
            json!([
                json!(["type", json!("test")]),
                json!([
                    "content",
                    json!([json!(["foo", "\"testing2\" - \"\"nested\"\""])])
                ])
            ])
            .to_string(),
            json!([
                json!([
                    json!(["type", "message"]),
                    json!(["content", "This is a message"])
                ]),
                json!([json!(["type", "code"]), json!(["content", "CODING"])])
            ])
            .to_string(),
        )
        .unwrap();

        assert_eq!(result.id, expectation.id);
        assert_eq!(result.data, expectation.data);
        assert_eq!(result.metadata, expectation.metadata);
        assert_eq!(result.errors, expectation.errors);
    }
}
