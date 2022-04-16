use serde::de::DeserializeOwned;
use serde_json::{json, Value as JSONValue};

pub struct Parser {}

impl Parser {
    pub fn from_arma<T: DeserializeOwned>(input: String) -> Result<T, String> {
        let input: JSONValue = match serde_json::from_str(&input) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!(
                    "[esm_message::parser::from_arma] Failed to convert input into JSONValue. Reason: {e}. Input: {input:?}"
                ))
            }
        };

        let input = match input.as_array() {
            Some(a) => a,
            None => {
                return Err(format!(
                    "[esm_message::parser::from_arma] Input is not of type Array. Input: {input:?}"
                ))
            }
        };

        let input_type = match input.get(0) {
            Some(v) => match v.as_str() {
                Some(v) => v,
                None => {
                    return Err(format!(
                        "[esm_message::parser::from_arma] Failed to extract string from {v:?}"
                    ))
                }
            },
            None => {
                return Err(format!("[esm_message::parser::from_arma] Failed to extract the \"type\" (index 0) from {input:?}"))
            }
        };

        let input_content = match input.get(1) {
            Some(v) => validate_content(v),
            None => {
                return Err(format!(
                    "[esm_message::parser::from_arma] Failed to extract the \"content\" (index 1) from {input:?}"
                ))
            }
        };

        // Convert to JSON, this allows us to deserialize it as an actual type. It also validates the data more
        let json = json!({ "type": input_type, "content": input_content });
        let json = match serde_json::to_string(&json) {
            Ok(j) => j,
            Err(e) => return Err(format!("[esm_message::parser::from_arma] Failed to convert final json to string. Reason: {e}. Input: \"{json}\"")),
        };

        let output: T = match serde_json::from_str(&json) {
            Ok(t) => t,
            Err(e) => return Err(format!("[esm_message::parser::from_arma] Failed to convert to Data. Reason: {e}. Input: \"{json}\" ")),
        };

        Ok(output)
    }
}

fn validate_content(input: &JSONValue) -> JSONValue {
    match input {
        JSONValue::Array(a) => match convert_arma_array_to_object(a) {
            Ok(v) => v,
            Err(_) => input.to_owned(),
        },
        _ => input.to_owned(),
    }
}

fn convert_arma_array_to_object(input: &Vec<JSONValue>) -> Result<JSONValue, String> {
    if input.len() != 2 || !input.iter().all(|i| i.is_array()) {
        return Err(format!("[esm_message::parser::convert_arma_array_to_object] Input must contain exactly 2 Arrays. Input: {input:?}"));
    }

    let key_array = match input.get(0) {
        Some(a) => a.as_array().unwrap(),
        None => {
            return Err(format!("[esm_message::parser::convert_arma_array_to_object] Failed to extract key array from {input:?}"));
        }
    };

    let value_array = match input.get(1) {
        Some(a) => a.as_array().unwrap(),
        None => {
            return Err(format!("[esm_message::parser::convert_arma_array_to_object] Failed to extract value array from {input:?}"));
        }
    };

    if !key_array.iter().all(|i| i.is_string()) {
        return Err(format!("[esm_message::parser::convert_arma_array_to_object] All elements must be Strings in key array {key_array:?}"));
    }

    if key_array.len() < value_array.len() {
        return Err(format!("[esm_message::parser::convert_arma_array_to_object] Missing keys! # of keys: {}, # of values: {}", key_array.len(), value_array.len()));
    }

    let mut object = serde_json::map::Map::new();
    for (index, key) in key_array.iter().enumerate() {
        let value = value_array.get(index).unwrap_or(&JSONValue::Null);

        object.insert(key.as_str().unwrap().to_string(), validate_content(value));
    }

    Ok(JSONValue::Object(object))
}
