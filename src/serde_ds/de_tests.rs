use std;
use datastore::{Value, Int};
use serde_ds::de;
use serde_ds::Error;

#[test]
fn test_deserialize_ints() {
    let input = Value::Integer { integer_value: Int(42) };

    // Signed integer types

    let res_i8: i8 = de::from_value(&input).expect("i8 deserialization failed");
    assert_eq!(42, res_i8);

    let res_i16: i16 = de::from_value(&input).expect("i16 deserialization failed");
    assert_eq!(42, res_i16);

    let res_i32: i32 = de::from_value(&input).expect("i32 deserialization failed");
    assert_eq!(42, res_i32);

    let res_i64: i64 = de::from_value(&input).expect("i64 deserialization failed");
    assert_eq!(42, res_i64);

    // Unsigned integer types

    let res_u8: u8 = de::from_value(&input).expect("u8 deserialization failed");
    assert_eq!(42, res_u8);

    let res_u16: u16 = de::from_value(&input).expect("u16 deserialization failed");
    assert_eq!(42, res_u16);

    let res_u32: u32 = de::from_value(&input).expect("u32 deserialization failed");
    assert_eq!(42, res_u32);

    let res_u64: u64 = de::from_value(&input).expect("u64 deserialization failed");
    assert_eq!(42, res_u64);
}

#[test]
fn test_integer_overflows() {
    let input = Value::Integer { integer_value: Int(std::i64::MAX) };
    let expected = Error::IntegerSizeMismatch();

    let res_i8 = de::from_value::<i8>(&input).unwrap_err();
    assert_eq!(expected, res_i8);

    let res_i16 = de::from_value::<i16>(&input).unwrap_err();
    assert_eq!(expected, res_i16);

    let res_i32 = de::from_value::<i32>(&input).unwrap_err();
    assert_eq!(expected, res_i32);

    de::from_value::<i64>(&input).expect("i64::MAX deserialization failed");

    // Unsigned integer types

    let res_u8 = de::from_value::<u8>(&input).unwrap_err();
    assert_eq!(expected, res_u8);

    let res_u16 = de::from_value::<u16>(&input).unwrap_err();
    assert_eq!(expected, res_u16);

    let res_u32 = de::from_value::<u32>(&input).unwrap_err();
    assert_eq!(expected, res_u32);

    de::from_value::<u64>(&input).expect("u64 deserialization of i64::MAX failed");
}

#[test]
fn test_deserialize_floats() {
    let input = Value::Double { double_value: 10.0 };

    let res_f32: f32 = de::from_value(&input).expect("f32 deserialization failed");
    assert_eq!(10.0, res_f32);

    let res_f64: f64 = de::from_value(&input).expect("f64 deserialization failed");
    assert_eq!(10.0, res_f64);
}

#[test]
fn test_float_overflow() {
    let input = Value::Double { double_value: std::f64::MAX };

    let res_f32 = de::from_value::<f32>(&input).unwrap_err();
    assert_eq!(Error::DoubleSizeMismatch(), res_f32);

    de::from_value::<f64>(&input).expect("f64::MAX deserialization failed");
}
