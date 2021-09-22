/// Attempts to retrieve a reference from the provided enum. Panicking if the internal data does not match the provided type.
/// Usage:
///     retrieve!(&message.data, Data::Init)
///     retrieve!(&message.metadata, Metadata::Command)
#[macro_export]
macro_rules! retrieve {
    ($enum:expr, $module:ident::$type:ident) => {{
        let data = match &$enum {
            $module::$type(ref v) => v.clone(),
            data => panic!("Unexpected type {:?}. Expected: {}.", data, stringify!($type))
        };

        data
    }};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_retrieve() {
        let mut message = Message::new(Type::Test);
        message.data = Data::Test(data::Test { foo: "testing".into() });
        message.metadata = Metadata::Test(metadata::Test { foo: "testing".into() });

        let result = retrieve!(&message.data, Data::Test);
        assert_eq!(result.foo, String::from("testing"));

        let result = retrieve!(&message.metadata, Metadata::Test);
        assert_eq!(result.foo, String::from("testing"));
    }
}
