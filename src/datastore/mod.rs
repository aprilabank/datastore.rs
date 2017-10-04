use std::collections::HashMap;
use serde::de::Error;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use base64;

#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartitionId {
    project_id: String,
    namespace_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PathElement {
    Id { kind: String, id: String },
    Name { kind: String, name: String },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Key {
    partition_id: PartitionId,
    path: Vec<PathElement>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ArrayValue {
    values: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LatLng {
    latitude: f64,
    longitude: f64,
}

/// This newtype around a byte vector provides base64-based (de-)serialisation for use in Datastore.
#[derive(Debug, PartialEq)]
pub struct Blob(Vec<u8>);

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        let bytes: &[u8] = self.0.as_ref();
        let encoded = base64::encode(bytes);
        serializer.serialize_str(encoded.as_ref())
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let str = String::deserialize(deserializer)?;
        base64::decode(&str)
            .map(|vec| { Blob(vec) })
            .map_err(|e| { D::Error::custom(format!("base64-decoding failed: {:?}", e)) })
    }
}

/// This newtype around a 64-bit signed integer provides string-based (de-)serialisation for
/// use in Datastore.
#[derive(Debug, PartialEq)]
pub struct Int(i64);

impl Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let string_rep = format!("{}", self.0);
        serializer.serialize_str(string_rep.as_str())
    }
}

impl<'de> Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let int_str: String = String::deserialize(deserializer)?;
        int_str.parse()
            .map(|i| { Int(i) })
            .map_err(|e| { D::Error::custom(format!("could not parse int: {:?}", e)) })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Value {
    Null { null_value: () },
    Boolean { boolean_value: bool },
    Integer { integer_value: Int },
    Double { double_value: f64 },
    Array { array_value: ArrayValue },
    GeoPoint { geo_point_value: LatLng },
    EntityValue { entity_value: Entity },
    KeyValue { key_value: Key },
    Blob { blob_value: Blob },
    // TODO: Timestamp { timestamp_value: ? },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Entity {
    // Key must be present for all non-nested entities.
    // TODO: Encode as type.
    key: Option<Key>,
    properties: HashMap<String, Value>,
}
