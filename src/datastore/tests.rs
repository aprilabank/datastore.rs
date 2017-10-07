use datastore::*;
use serde_json;
use chrono::{TimeZone, Utc};
use std::fs::File;
use std::path::{Path, PathBuf};

#[test]
fn test_path_element_serialisation() {
    let path_element = PathElement::Id {
        kind: "TestKind".to_string(),
        id: "foo".to_string(),
    };
    let serialised = serde_json::to_string(&path_element).expect("Serialisation failed");
    let json = r#"{"kind":"TestKind","id":"foo"}"#.to_string();
    assert_eq!(json, serialised, "Path element should have serialised correctly");

    let deserialised: PathElement = serde_json::from_str(json.as_ref()).expect("Deserialisation failed");
    assert_eq!(path_element, deserialised, "Deserialised element should match initial value");
}

#[test]
fn test_blob_serialisation() {
    let blob = Blob(vec![82, 117, 115, 116, 33]);
    let expected = "\"UnVzdCE=\"".to_string();

    let serialised = serde_json::to_string(&blob).expect("Serialisation failed");
    assert_eq!(expected, serialised, "Serialisation should match expected value");

    let deserialised: Blob = serde_json::from_str(serialised.as_str()).expect("Deserialisation failed");
    assert_eq!(blob, deserialised, "Deserialised blob should match initial value");
}

#[test]
fn test_int_serialisation() {
    let int = Int(1337); // Sometimes a cliche is required.
    let expected = "\"1337\"".to_string();

    let serialised = serde_json::to_string(&int).expect("Serialisation failed");
    assert_eq!(expected, serialised, "Serialisation should match expected value");

    let deserialised: Int = serde_json::from_str(serialised.as_str()).expect("Deserialisation failed");
    assert_eq!(int, deserialised, "Deserialised int shoudl match initial value");
}

#[test]
fn test_entity_deserialisation() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/entity-test.json");
    let file = File::open(d.as_ref() as &Path)
        .expect("Could not open test file");
    let deserialised: Entity = serde_json::from_reader(file).expect("Deserialisation failed");

    let expected_time =
        Utc.ymd(2017, 09, 21).and_hms(05, 41, 33);

    let expected_cl_options = hashmap!(
        "scoringStatus".to_string() => Value::String { string_value: "Accepted".to_string() },
    );

    let expected_products = Value::Array {
        array_value: ArrayValue {
            values: vec![ Value::String { string_value: "creditline".to_string() } ],
        },
    };

    let properties = hashmap!(
        "email".to_string() => Value::String { string_value: "mags@mag".to_string() },
        "companyCountry".to_string() => Value::String { string_value: "NO".to_string() },
        "status".to_string() => Value::EntityValue {
            entity_value: Entity { properties: expected_cl_options },
        },
        "signingId".to_string() => Value::Null { null_value: () },
        "created".to_string() => Value::Timestamp { timestamp_value: expected_time },
        "availableProducts".to_string() => expected_products,
    );

    let expected = Entity { properties };
    assert_eq!(expected, deserialised, "Deserialised value should match expectations");
}
