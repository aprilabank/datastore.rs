use serde::ser::{self, Serialize};
use error::{Error, Result};
use datastore::Value;

pub struct Serializer {}

pub fn to_value<T: Serialize>(value: &T) -> Value {
    unimplemented!()
}
