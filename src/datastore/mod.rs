use std::collections::HashMap;
use serde::de::Error;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use base64;
use chrono::{DateTime, Utc};
use std::str::FromStr;
use std::convert::Into;

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
    pub values: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct LatLng {
    latitude: f64,
    longitude: f64,
}

/// This newtype around a byte vector provides base64-based (de-)serialisation for use in Datastore.
#[derive(Debug, PartialEq, Clone)]
pub struct Blob(pub Vec<u8>);

impl From<Vec<u8>> for Blob { fn from(v: Vec<u8>) -> Blob { Blob(v) } }

impl<'a> From<&'a [u8]> for Blob {
    fn from(v: &'a [u8]) -> Self {
        Blob(v.to_vec())
    }
}


impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes: &[u8] = self.0.as_ref();
        let encoded = base64::encode(bytes);
        serializer.serialize_str(encoded.as_ref())
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        base64::decode(&str).map(|vec| Blob(vec)).map_err(|e| {
            D::Error::custom(format!("base64-decoding failed: {:?}", e))
        })
    }
}

/// Datastore represents all integral types as string.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Int(String);

impl Int {
    // Delegate to the String::parse method.
    pub fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        self.0.parse()
    }
}

// Boilerplate for int -> Int conversions:
impl From<u8> for Int { fn from(v: u8) -> Self { Int(format!("{}", v)) } }

impl From<u16> for Int { fn from(v: u16) -> Self { Int(format!("{}", v)) } }

impl From<u32> for Int { fn from(v: u32) -> Self { Int(format!("{}", v)) } }

impl From<u64> for Int { fn from(v: u64) -> Self { Int(format!("{}", v)) } }

impl From<i8> for Int { fn from(v: i8) -> Self { Int(format!("{}", v)) } }

impl From<i16> for Int { fn from(v: i16) -> Self { Int(format!("{}", v)) } }

impl From<i32> for Int { fn from(v: i32) -> Self { Int(format!("{}", v)) } }

impl From<i64> for Int { fn from(v: i64) -> Self { Int(format!("{}", v)) } }

// Currently the many nested attributes are needed because of
// https://github.com/serde-rs/serde/issues/1061
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Value {
    Null {
        #[serde(rename = "nullValue")]
        null_value: (),
    },
    String {
        #[serde(rename = "stringValue")]
        string_value: String,
    },
    Boolean {
        #[serde(rename = "booleanValue")]
        boolean_value: bool,
    },
    Integer {
        #[serde(rename = "integerValue")]
        integer_value: Int,
    },
    Double {
        #[serde(rename = "doubleValue")]
        double_value: f64,
    },
    Array {
        #[serde(rename = "arrayValue")]
        array_value: ArrayValue,
    },
    GeoPoint {
        #[serde(rename = "geoPointValue")]
        geo_point_value: LatLng,
    },
    EntityValue {
        #[serde(rename = "entityValue")]
        entity_value: Entity,
    },
    KeyValue {
        #[serde(rename = "keyValue")]
        key_value: Key,
    },
    Blob {
        #[serde(rename = "blobValue")]
        blob_value: Blob,
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

// Lifestyle improvements via `From` instances for `Value`:

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::String { string_value: s.to_string() }
    }
}

impl From<String> for Value {
    fn from(string_value: String) -> Self {
        Value::String { string_value }
    }
}

impl From<bool> for Value {
    fn from(boolean_value: bool) -> Self {
        Value::Boolean { boolean_value }
    }
}

impl From<Entity> for Value {
    fn from(entity_value: Entity) -> Self {
        Value::EntityValue { entity_value }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null { null_value: () }
    }
}

impl From<f64> for Value {
    fn from(double_value: f64) -> Self {
        Value::Double { double_value }
    }
}

impl<T> From<T> for Value where T: Into<Int> {
    fn from(i: T) -> Self {
        Value::Integer { integer_value: i.into() }
    }
}

impl From<Vec<Value>> for Value {
    fn from(values: Vec<Value>) -> Self {
        let array_value = ArrayValue { values };
        Value::Array { array_value }
    }
}

impl From<Blob> for Value {
    fn from(blob_value: Blob) -> Self {
        Value::Blob { blob_value }
    }
}
