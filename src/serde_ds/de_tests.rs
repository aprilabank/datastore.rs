use std;
use datastore::{Value, Int};
use serde_ds::de;
use serde_ds::Error;

#[test]
fn test_deserialize_ints() {
    let input = Value::Integer { integer_value: Int::from(42) };

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
