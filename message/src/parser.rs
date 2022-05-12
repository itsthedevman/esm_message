use serde::de::DeserializeOwned;
use serde_json::Value as JSONValue;

pub struct Parser {}

impl Parser {
    pub fn from_arma<T: DeserializeOwned>(input: &str) -> Result<T, String> {
        let input = input.replace("\"\"", "\\\"");

        let input: JSONValue = match serde_json::from_str(&input) {
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
    if !input
        .iter()
        .all(|i| i.is_array() && i.as_array().unwrap().len() == 2)
    {
        return Err(format!("[esm_message::parser::convert_arma_array_to_object] Input must consist of key/value pairs. Input: {input:?}"));
    }

    let mut object = serde_json::map::Map::new();
    for pair in input {
        let pair = match pair.as_array() {
            Some(a) => a,
            None => return Err(format!("[esm_message::parser::convert_arma_array_to_object] Failed to convert key/value pair. Pair: {pair:?}")),
        };

        let key = match pair.get(0) {
            Some(k) => match k.as_str() {
                Some(k) => k,
                None => return Err(format!("[esm_message::parser::convert_arma_array_to_object] Failed to convert key to string. Pair: {pair:?}"))
            },
            None => return Err(format!("[esm_message::parser::convert_arma_array_to_object] Failed to extract key from {pair:?}"))
        };

        let value = match pair.get(1) {
            Some(v) => v,
            None => return Err(format!("[esm_message::parser::convert_arma_array_to_object] Failed to extract value from {pair:?}"))
        };

        object.insert(key.to_string(), validate_content(value));
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
            json!(["key_1", "value_1"]),
            json!(["key_2", 2_i32]),
            json!(["key_3", true]),
            json!(["key_4", vec![json!(["sub_key_1", "sub_value_1"])]])
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
            json!(["type", "test"]),
            json!(["content", json!([json!(["foo", "bar"])])])
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

    #[test]
    fn it_handles_escaped_strings() {
        let input = "[[\"type\",\"sqf_result\"],[\"content\",[[\"result\",\"[[\"\"key_1\"\",\"\"value_1\"\"],[\"\"key_2\"\",true],[\"\"key_3\"\",[[\"\"key_4\"\",false],[\"\"key_5\"\",[[\"\"key_6\"\",6],[\"\"key_7\"\",<null>]]]]]]\"]]]]";

        let result: Result<Data, String> = Parser::from_arma(input);

        assert_eq!(
            result.unwrap(),
            Data::SqfResult(data::SqfResult {
                result: Some("[[\"key_1\",\"value_1\"],[\"key_2\",true],[\"key_3\",[[\"key_4\",false],[\"key_5\",[[\"key_6\",6],[\"key_7\",<null>]]]]]]".to_string())
            })
        );
    }
}
