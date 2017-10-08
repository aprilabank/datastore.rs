use std::collections::HashMap;
use serde::de::Error;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use base64;
use chrono::{DateTime, Utc};

#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct PartitionId {
    project_id: String,
    namespace_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum PathElement {
    Id { kind: String, id: String },
    Name { kind: String, name: String },
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Key {
    partition_id: PartitionId,
    path: Vec<PathElement>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ArrayValue {
    values: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct LatLng {
    latitude: f64,
    longitude: f64,
}

/// This newtype around a byte vector provides base64-based (de-)serialisation for use in Datastore.
#[derive(Debug, PartialEq, Clone)]
pub struct Blob(pub Vec<u8>);

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
#[derive(Debug, PartialEq, Clone)]
pub struct Int(pub i64);

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

// Currently the many nested attributes are needed because of https://github.com/serde-rs/serde/issues/1061
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Value {
    Null {
        #[serde(rename = "nullValue")]
        null_value: ()
    },
    String {
        #[serde(rename = "stringValue")]
        string_value: String
    },
    Boolean {
        #[serde(rename = "booleanValue")]
        boolean_value: bool
    },
    Integer {
        #[serde(rename = "integerValue")]
        integer_value: Int
    },
    Double {
        #[serde(rename = "doubleValue")]
        double_value: f64
    },
    Array {
        #[serde(rename = "arrayValue")]
        array_value: ArrayValue
    },
    GeoPoint {
        #[serde(rename = "geoPointValue")]
        geo_point_value: LatLng
    },
    EntityValue {
        #[serde(rename = "entityValue")]
        entity_value: Entity
    },
    KeyValue {
        #[serde(rename = "keyValue")]
        key_value: Key
    },
    Blob {
        #[serde(rename = "blobValue")]
        blob_value: Blob
    },
    Timestamp {
        #[serde(rename = "timestampValue")]
        timestamp_value: DateTime<Utc>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Entity {
    pub properties: HashMap<String, Value>,
}
