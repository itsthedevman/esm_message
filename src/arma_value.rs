// Pulled from https://github.com/BrettMayson/arma-rs/blob/expirement/core/src/to_arma.rs
// Needed to add more to it

#[macro_export]
macro_rules! arma_value {
    // Ruby hash syntax (would've preferred json syntax)
    ({ $($key:expr => $value:expr),* }) => {{
        ArmaValue::HashMap(
            vec![
                $( ($key.to_arma(), $value.to_arma()) ),*
            ]
        )
    }};

    // Array
    ([ $($val:expr),* ]) => {
        ArmaValue::Array(
            vec![$($val.to_arma()),*]
        )
    };

    // String, number
    ($val:literal) => {
        $val.to_arma()
    };

    // Nil
    (nil) => {
        ArmaValue::Nil
    };

    (null) => {
        ArmaValue::Nil
    };
}

#[derive(Clone)]
pub enum ArmaValue {
    Nil,
    Number(f32),
    Array(Vec<ArmaValue>),
    Boolean(bool),
    String(String),
    HashMap(Vec<(ArmaValue, ArmaValue)>),
}

impl std::fmt::Display for ArmaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => write!(f, "{}", n.to_string()),
            Self::Array(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Boolean(b) => write!(f, "{}", b.to_string()),

            // Because Arma strings are quoted twice in a string
            Self::String(s) => write!(f, "\"\"{}\"\"", s.to_string().replace("\"", "\"\"")),
            Self::HashMap(h) => write!(
                f,
                "[{}]",
                h.iter()
                    .map(|(k, v)| format!("[{}, {}]", k.to_string(), v.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

pub trait ToArma {
    fn to_arma(&self) -> ArmaValue;
}

impl ToArma for ArmaValue {
    fn to_arma(&self) -> ArmaValue {
        self.to_owned()
    }
}

impl<T: ToArma> ToArma for Vec<T> {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Array(self.iter().map(|x| x.to_arma()).collect::<Vec<ArmaValue>>())
    }
}

impl<T: ToArma> ToArma for (T, T) {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Array(vec![self.0.to_arma(), self.1.to_arma()])
    }
}

impl ToArma for String {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::String(self.to_string())
    }
}
impl ToArma for &'static str {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::String(self.to_string())
    }
}

impl ToArma for u8 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u16 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u64 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u128 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}

impl ToArma for i8 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i16 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i64 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i128 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}

impl ToArma for f32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for f64 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}

impl ToArma for bool {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Boolean(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_converts_bool() {
        assert_eq!(format!("{}", ArmaValue::Boolean(true)), "true");
        assert_eq!(format!("{}", ArmaValue::Boolean(false)), "false");
    }

    #[test]
    fn test_it_converts_hash_map() {
        let vec = vec![
            (ArmaValue::String("key".into()), ArmaValue::String("value".into())),
            (ArmaValue::String("key2".into()), ArmaValue::Boolean(true))
        ];

        assert_eq!(format!("{}", ArmaValue::HashMap(vec)), r#"[[""key"", ""value""], [""key2"", true]]"#);
    }

    #[test]
    fn test_macro_arma_value() {
        // Hashmap
        let int = 55;
        let result = arma_value!({
            "string" => "world",
            "int" => int,
            "bool" => false,
            "array" => vec![1, 2, 3],
            "hash" => arma_value!({ "true" => true, "false" => false })
        });

        assert_eq!(
            result.to_string(),
            r#"[[""string"", ""world""], [""int"", 55], [""bool"", false], [""array"", [1, 2, 3]], [""hash"", [[""true"", true], [""false"", false]]]]"#
        );

        // Array
        let result = arma_value!(["string", int, false, vec![1,2,3], arma_value!({ "true" => true, "false" => false })]);
        assert_eq!(
            result.to_string(),
            r#"[""string"", 55, false, [1, 2, 3], [[""true"", true], [""false"", false]]]"#
        );

        // String
        let result = arma_value!("test");
        assert_eq!(result.to_string(), "\"\"test\"\"");

        // Number
        let result = arma_value!(55);
        assert_eq!(result.to_string(), "55");

        // Boolean
        let result = arma_value!(true);
        assert_eq!(result.to_string(), "true");

        // Nil/Null
        let result = arma_value!(nil);
        assert_eq!(result.to_string(), "nil");

        let result = arma_value!(null);
        assert_eq!(result.to_string(), "nil");
    }
}
