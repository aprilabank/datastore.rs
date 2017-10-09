use std::collections::HashMap;
use serde::Serialize;
use serde_ds::ser;
use datastore::{Int, Blob, Value, Entity, ArrayValue};

// Tests for simple value serialisation
#[test]
fn test_serialize_ints() {
    // Same expected result for all integers
    let expected = Value::Integer { integer_value: Int(14) };

    // Test unsigned types
    let res_u8 = ser::to_value(&(14 as u8)).expect("u8 serialization failed");
    assert_eq!(expected, res_u8);

    let res_u16 = ser::to_value(&(14 as u16)).expect("u16 serialization failed");
    assert_eq!(expected, res_u16);

    let res_u32 = ser::to_value(&(14 as u32)).expect("u32 serialization failed");
    assert_eq!(expected, res_u32);

    let res_u64 = ser::to_value(&(14 as u64)).expect("u64 serialization failed");
    assert_eq!(expected, res_u64);

    // Test signed types
    let res_i8 = ser::to_value(&(14 as i8)).expect("i8 serialization failed");
    assert_eq!(expected, res_i8);

    let res_i16 = ser::to_value(&(14 as i16)).expect("i16 serialization failed");
    assert_eq!(expected, res_i16);

    let res_i32 = ser::to_value(&(14 as i32)).expect("i32 serialization failed");
    assert_eq!(expected, res_i32);

    let res_i64 = ser::to_value(&(14 as i64)).expect("i64 serialization failed");
    assert_eq!(expected, res_i64);
}

#[test]
fn test_serialize_floats() {
    let expected = Value::Double { double_value: 10.0 };

    let res_f32 = ser::to_value(&(10.0 as f32)).expect("f32 serialization failed");
    assert_eq!(expected, res_f32);

    let res_f64 = ser::to_value(&(10.0 as f64)).expect("f64 serialization failed");
    assert_eq!(expected, res_f64);
}

#[test]
fn test_serialize_string() {
    let expected = Value::String { string_value: "serialized string".to_string() };

    let result_str = ser::to_value(&"serialized string")
        .expect("String serialisation failed");
    assert_eq!(expected, result_str);

    let result_string = ser::to_value(&"serialized string".to_string())
        .expect("String serialisation failed");
    assert_eq!(expected, result_string);
}


#[test]
fn test_serialize_bool() {
    let expected = Value::Boolean { boolean_value: true };
    let result = ser::to_value(&true).expect("bool serialization failed");
    assert_eq!(expected, result);
}

#[test]
fn test_serialize_unit() {
    let expected = Value::Null { null_value: () };

    let result_unit = ser::to_value(&()).expect("unit serialization failed");
    assert_eq!(expected, result_unit);

    #[derive(Serialize)]
    struct Foo;

    let result_unit_struct = ser::to_value(&Foo).expect("unit struct serialization failed");
    assert_eq!(expected, result_unit_struct);
}

#[test]
fn test_serialize_option() {
    let result_some = ser::to_value(&(Option::Some(4 as u8))).expect("Option::Some serialization failed");
    let expected_some = Value::Integer { integer_value: Int(4) };
    assert_eq!(expected_some, result_some);

    let result_none = ser::to_value(&(Option::None as Option<u8>)).expect("Option::None serialization failed");
    let expected_none = Value::Null { null_value: () };
    assert_eq!(expected_none, result_none);
}

/*

Byte serialization test does not pass - gets handed to serialize_vec despite an implementation for
&[u8] - why?

Note: https://github.com/serde-rs/bytes

#[test]
fn test_serialize_bytes() {
    let result_bytes = ser::to_value(&"foo".as_bytes().to_vec())
        .expect("bytes serialization failed");
    let expected = Value::Blob { blob_value: Blob(vec!['f' as u8, 'o' as u8, 'o' as u8]) };

    assert_eq!(expected, result_bytes);
}
*/

#[test]
fn test_serialize_map() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value");

    let result = ser::to_value(&map).expect("HashMap serialization failed");

    let mut expected_properties = HashMap::new();
    expected_properties.insert(
        "key".to_string(),
        Value::String { string_value: "value".to_string() },
    );

    let expected = Value::EntityValue {
        entity_value: Entity {
            properties: expected_properties,
        },
    };

    assert_eq!(expected, result);
}

#[test]
fn test_serialize_seq() {
    let test_vec = vec!["hello", "rust"];
    let serialized = ser::to_value(&test_vec).expect("vector serialization failed");

    let expected = Value::Array {
        array_value: ArrayValue {
            values: vec![
                Value::String { string_value: "hello".to_string() },
                Value::String { string_value: "rust".to_string() },
            ]
        },
    };

    assert_eq!(expected, serialized);
}

#[test]
fn test_serialize_struct() {
    #[derive(Debug, Serialize)]
    struct Language<'a> {
        name: &'a str,
        strongly_typed: bool,
    };

    let rust = Language {
        name: "Rust",
        strongly_typed: true,
    };

    let serialized = ser::to_value(&rust).expect("struct serialization failed");

    let mut expected_properties = HashMap::new();
    expected_properties.insert(
        "name".to_string(),
        Value::String { string_value: "Rust".to_string() },
    );
    expected_properties.insert(
        "strongly_typed".to_string(),
        Value::Boolean { boolean_value: true },
    );

    let expected = Value::EntityValue {
        entity_value: Entity {
            properties: expected_properties,
        }
    };

    assert_eq!(expected, serialized);
}
