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
