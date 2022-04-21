use serde::de::DeserializeOwned;
use serde_json::Value as JSONValue;

pub struct Parser {}

impl Parser {
    pub fn from_arma<T: DeserializeOwned>(input: &str) -> Result<T, String> {
        let input: JSONValue = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!(
                    "[esm_message::parser::from_arma] Failed to convert input into JSONValue. Reason: {e}. Input: {input:?}"
                ))
            }
        };

        let json = validate_content(&input);
        let json = match serde_json::to_string(&json) {
            Ok(j) => j,
            Err(e) => return Err(format!("[esm_message::parser::from_arma] Failed to convert to final JSON. Reason: {e}. Input: \"{input}\"")),
        };

        let output: T = match serde_json::from_str(&json) {
            Ok(t) => t,
            Err(e) => return Err(format!("[esm_message::parser::from_arma] Failed to convert to Data/Metadata. Reason: {e}. Input: \"{input}\" ")),
        };

        Ok(output)
    }
}

pub fn validate_content(input: &JSONValue) -> JSONValue {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data, Data};
    use arma_rs::IntoArma;
    use serde_json::json;

    #[test]
    fn it_converts_arma_hash_correctly() {
        let input = json!([
            json!(["key_1", "key_2", "key_3", "key_4"]),
            json!([
                "value_1",
                2_i32,
                true,
                vec![json!(["sub_key_1"]), json!(["sub_value_1"])]
            ])
        ]);

        let result = validate_content(&input);
        assert_eq!(
            result,
            json!({
                "key_1": json!("value_1"),
                "key_2": json!(2_i32),
                "key_3": json!(true),
                "key_4": json!({ "sub_key_1": "sub_value_1" })
            })
        )
    }

    #[test]
    fn it_converts_to_data_struct() {
        let input = json!([
            json!(["type", "content"]),
            json!(["test", json!([json!(["foo"]), json!(["bar"])])])
        ])
        .to_arma()
        .to_string();

        let result: Result<Data, String> = Parser::from_arma(&input);

        assert_eq!(
            result.unwrap(),
            Data::Test(data::Test {
                foo: "bar".to_string()
            })
        );
    }
}
