use std::collections;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde_ds::{de, ser};

// These tests perform roundtrip serialisation of a type and check whether "the same thing" came out
// at the other end.

fn test_roundtrip<'de, T>(value: T)
    where T: Debug + PartialEq + Serialize + Deserialize<'de>
{
    let serialized = ser::to_value(&value).expect("serialization failed");
    let deserialized: T = de::from_value(serialized).expect("deserialization failed");
    assert_eq!(value, deserialized, "value mismatch after serialization round-trip");
}

#[test]
fn test_unsigned_integers() {
    test_roundtrip(42 as u8);
    test_roundtrip(42 as u16);
    test_roundtrip(42 as u32);
    test_roundtrip(42 as u64);
}

#[test]
fn test_signed_integers() {
    test_roundtrip(-42 as i8);
    test_roundtrip(-42 as i16);
    test_roundtrip(-42 as i32);
    test_roundtrip(-42 as i64);
}

#[test]
fn test_string() {
    // &str is not tested because nothing can own the deserialised string value.
    test_roundtrip("test String".to_string());
}

#[test]
fn test_bool() {
    test_roundtrip(true);
    test_roundtrip(false);
}
#[test]
fn test_vectors() {
    test_roundtrip(vec!["1".to_string(), "2".to_string()]);
    test_roundtrip(vec![1, 2]);
    test_roundtrip(vec![true, false])
}

#[test]
fn test_optional() {
    test_roundtrip(Option::None as Option<u8>);
    test_roundtrip(Option::Some(42));
    test_roundtrip(Option::Some("foo".to_string()));
}

#[test]
fn test_maps() {
    test_roundtrip(collections::HashMap::<u8, u8>::new());
    test_roundtrip(hashmap!(
        "meaning".to_string() => 42,
        "other".to_string() => 13,
    ));
}

#[test]
fn test_struct() {
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    struct TestStruct {
        int_field: u8,
        string_field: String,
        bool_field: bool,
    }

    test_roundtrip(TestStruct {
        int_field: 42,
        string_field: String::from("foo"),
        bool_field: false,
    });
}

/*
#[test]
fn test_simple_enum() {
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    enum Colour {
        Red, Blue, Green,
    }

    test_roundtrip(Colour::Red);
    test_roundtrip(Colour::Blue);
    test_roundtrip(Colour::Green);
}
*/
