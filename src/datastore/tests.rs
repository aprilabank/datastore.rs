use datastore::*;
use serde_json;

fn test_project_id() -> String {
    "test-project".to_string()
}

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
