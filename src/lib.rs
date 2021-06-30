pub mod data;
pub mod error;
pub mod metadata;

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
pub use data::Data;
pub use error::{Error, ErrorType};
use message_io::network::ResourceId;
pub use metadata::Metadata;
use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    resource_id: Option<i64>,

    #[serde(default, skip_serializing_if = "data_is_empty")]
    data: Data,

    #[serde(default, skip_serializing_if = "metadata_is_empty")]
    metadata: Metadata,

    #[serde(default, skip_serializing_if = "errors_is_empty")]
    errors: Vec<Error>,
}

fn data_is_empty(data: &Data) -> bool {
    matches!(data, Data::Empty(_))
}

fn metadata_is_empty(metadata: &Metadata) -> bool {
    matches!(metadata, Metadata::Empty(_))
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
            data: Data::Empty(Empty::new()),
            metadata: Metadata::Empty(Empty::new()),
            errors: Vec::new(),
        }
    }

    pub fn from_bytes<F>(data: Vec<u8>, server_key_getter: F) -> Result<Message, String>
    where
        F: Fn(&Vec<u8>) -> Option<Vec<u8>>,
    {
        let message = decrypt_message(data, server_key_getter)?;

        Ok(message)
    }

    pub fn set_resource(&mut self, resource_id: ResourceId) -> &Message {
        self.resource_id = Some(resource_id.adapter_id() as i64);
        self
    }

    pub fn set_data(&mut self, data: Data) -> &Message {
        self.data = data;
        self
    }

    pub fn add_error(
        &mut self,
        error_type: ErrorType,
        error_message: &'static str,
    ) -> &Message {
        let error = Error::new(error_type, error_message.into());
        self.errors.push(error);
        self
    }

    pub fn as_bytes<F>(&self, server_key_getter: F) -> Result<Vec<u8>, String>
    where
        F: Fn(&Vec<u8>) -> Option<Vec<u8>>,
    {
        let server_id = match self.server_id.clone() {
            Some(id) => id,
            None => return Err("Message does not have a server ID".into()),
        };

        encrypt_message(self, &server_id, server_key_getter)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Connect,
    Disconnect,
    UpdateKeys,
    Ping,
    Pong,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Empty {}
    }
}

impl Default for Empty {
    fn default() -> Self {
        Empty::new()
    }
}

////////////////////////////////////////////////////////////

fn encrypt_message<F>(
    message: &Message,
    server_id: &Vec<u8>,
    server_key_getter: F,
) -> Result<Vec<u8>, String>
where
    F: Fn(&Vec<u8>) -> Option<Vec<u8>>,
{
    // Find the server key so the message can be encrypted
    let server_key = match (server_key_getter)(server_id) {
        Some(key) => key,
        None => return Err(format!("Failed to retrieve server_key for {:?}", server_id)),
    };

    // Setup everything for encryption
    let encryption_key = Key::from_slice(&server_key[0..32]); // server_key has to be exactly 32 bytes
    let encryption_cipher = Aes256Gcm::new(encryption_key);
    let nonce_key: Vec<u8> = (0..12).map(|_| random::<u8>()).collect();
    let encryption_nonce = Nonce::from_slice(&nonce_key);

    // Serialize this message
    let message = match serde_json::to_vec(&message) {
        Ok(bytes) => bytes,
        Err(e) => return Err(e.to_string()),
    };

    // Encrypt the message
    let encrypted_message = match encryption_cipher.encrypt(encryption_nonce, message.as_ref()) {
        Ok(bytes) => bytes,
        Err(e) => return Err(e.to_string()),
    };

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
    let mut packet: Vec<u8> = vec![server_id.len() as u8];
    packet.extend(&*server_id);

    // Append the nonce length and itself to the packet
    packet.push(nonce_key.len() as u8);
    packet.extend(&*nonce_key);

    // Now add the encrypted message to the end. This completes the packet
    packet.extend(&*encrypted_message);

    Ok(packet)
}

fn decrypt_message<F>(bytes: Vec<u8>, server_key_getter: F) -> Result<Message, String>
where
    F: Fn(&Vec<u8>) -> Option<Vec<u8>>,
{
    // The first byte is the length of the server_id so we know how many bytes to extract
    let id_length = bytes[0] as usize;

    // Extract the server ID and convert to a vec
    let server_id = bytes[1..=id_length].to_vec();

    // Find the server key so the message can be decrypted
    let server_key = match (server_key_getter)(&server_id) {
        Some(key) => key,
        None => return Err(format!("Failed to retrieve server_key for {:?}", server_id)),
    };

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
    let server_key = &server_key[0..32]; // server_key has to be exactly 32 bytes
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
        Err(e) => return Err(format!(
            "Failed to deserialize. Reason: {:?}. Message: {:?}",
            e, decrypted_bytes
        )),
    };

    // Store the server id
    message.server_id = Some(server_id);

    Ok(message)
}

#[cfg(test)]
mod tests {
    use crate::data::ServerInitialization;

    use super::*;

    #[test]
    fn encrypt_and_decrypt_message() {
        let mut message = Message::new(Type::Connect);

        let server_init = ServerInitialization {
            server_name: "server_name".into(),
            price_per_object: 10.0,
            territory_lifetime: 10.0,
            territory_data: "[]".into(),
            server_start_time: chrono::Utc::now(),
        };

        let expected = server_init.clone();

        message.set_data(Data::ServerInitialization(server_init));

        let server_id = "testing".as_bytes().to_vec();
        let server_key = "12345678901234567890123456789012345678901234567890"
            .as_bytes()
            .to_vec();

        let encrypted_bytes =
            encrypt_message(&message, &server_id, |_| Some(server_key.to_owned()));
        assert!(encrypted_bytes.is_ok());

        let decrypted_message =
            decrypt_message(encrypted_bytes.unwrap(), |_| Some(server_key.to_owned()));
        assert!(decrypted_message.is_ok());

        let decrypted_message = decrypted_message.unwrap();
        assert_eq!(decrypted_message.message_type, Type::Connect);

        // Ensure it has a server ID
        assert!(decrypted_message.server_id.is_some());

        match decrypted_message.data {
            Data::ServerInitialization(data) => {
                assert_eq!(data.server_name, expected.server_name);
                assert_eq!(data.price_per_object as i64, expected.price_per_object as i64);
                assert_eq!(data.territory_lifetime as i64, expected.territory_lifetime as i64);
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
        assert_eq!(serialized_message, format!("{{\"id\":\"{}\",\"type\":\"connect\"}}", message.id));
    }

    #[test]
    fn test_data_is_empty() {
        let result = data_is_empty(&Data::Empty(Empty::new()));
        assert!(result);

        let server_init = ServerInitialization {
            server_name: "server_name".into(),
            price_per_object: 10.0,
            territory_lifetime: 10.0,
            territory_data: "[]".into(),
            server_start_time: chrono::Utc::now(),
        };

        let result = data_is_empty(&Data::ServerInitialization(server_init));
        assert!(!result);
    }

    #[test]
    fn test_metadata_is_empty() {
        let result = metadata_is_empty(&Metadata::Empty(Empty::new()));
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
        assert!(matches!(message.data, Data::Empty(_)));
        assert!(matches!(message.metadata, Metadata::Empty(_)));
        assert!(message.errors.is_empty());
    }
}
