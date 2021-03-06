use std::collections::HashMap;
use serde_ds::ser;
use serde_bytes;
use datastore::{Blob, Value, Entity};

// Tests for simple value serialisation
#[test]
fn test_serialize_ints() {
    // Same expected result for all integers
    let expected = Value::from(14);

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
    let expected = Value::from(10.0);

    let res_f32 = ser::to_value(&(10.0 as f32)).expect("f32 serialization failed");
    assert_eq!(expected, res_f32);

    let res_f64 = ser::to_value(&(10.0 as f64)).expect("f64 serialization failed");
    assert_eq!(expected, res_f64);
}

#[test]
fn test_serialize_string() {
    let expected = Value::from("serialized string");

    let result_str = ser::to_value(&"serialized string").expect("String serialisation failed");
    assert_eq!(expected, result_str);

    let result_string =
        ser::to_value(&"serialized string".to_string()).expect("String serialisation failed");
    assert_eq!(expected, result_string);
}


#[test]
fn test_serialize_bool() {
    let expected = Value::from(true);
    let result = ser::to_value(&true).expect("bool serialization failed");
    assert_eq!(expected, result);
}

#[test]
fn test_serialize_unit() {
    let expected = Value::from(());

    let result_unit = ser::to_value(&()).expect("unit serialization failed");
    assert_eq!(expected, result_unit);

    #[derive(Serialize)]
    struct Foo;

    let result_unit_struct = ser::to_value(&Foo).expect("unit struct serialization failed");
    assert_eq!(expected, result_unit_struct);
}

#[test]
fn test_serialize_option() {
    let result_some =
        ser::to_value(&(Option::Some(4 as u8))).expect("Option::Some serialization failed");
    let expected_some = Value::from(4);
    assert_eq!(expected_some, result_some);

    let result_none =
        ser::to_value(&(Option::None as Option<u8>)).expect("Option::None serialization failed");
    let expected_none = Value::from(());
    assert_eq!(expected_none, result_none);
}

#[test]
fn test_serialize_bytes() {
    let input = serde_bytes::Bytes::new(b"foo");
    let result_bytes = ser::to_value(&input)
        .expect("bytes serialization failed");
    let expected = Value::from(Blob(vec!['f' as u8, 'o' as u8, 'o' as u8]));

    assert_eq!(expected, result_bytes);
}

#[test]
fn test_serialize_map() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value");

    let result = ser::to_value(&map).expect("HashMap serialization failed");

    let mut expected_properties = HashMap::new();
    expected_properties.insert(
        "key".to_string(),
        Value::from("value"),
    );

    let expected = Value::from(Entity { properties: expected_properties });

    assert_eq!(expected, result);
}

#[test]
fn test_serialize_seq() {
    let test_vec = vec!["hello", "rust"];
    let serialized = ser::to_value(&test_vec).expect("vector serialization failed");

    let expected = Value::from(vec![
        Value::from("hello"),
        Value::from("rust"),
    ]);


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

    let properties = hashmap! {
        "name".to_string() => Value::from("Rust"),
        "strongly_typed".to_string() => Value::from(true),
    };

    let expected = Value::from(Entity { properties });

    assert_eq!(expected, serialized);
}
