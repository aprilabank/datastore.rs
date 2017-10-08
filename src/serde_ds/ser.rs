use std::collections::HashMap;
use std::fmt::Display;
use serde::ser::{self, Serialize};
use serde_ds::error::{Error, Result};
use datastore::{Value, Entity, Int, Blob};

#[derive(Copy, Clone)]
pub struct Serializer;

pub fn to_value<T: Serialize>(value: &T) -> Result<Value> {
    let serializer = Serializer;
    value.serialize(&serializer)
}

impl<'a> ser::Serializer for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = MapSerializer<'a>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(Value::Boolean { boolean_value: v })
    }

    // All integer types are represented by the same type in Datastore.

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(Value::Integer { integer_value: Int(v as i64) })
    }

    // Likewise, all floating-point numbers map to the same type.

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(Value::Double { double_value: v as f64 })
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(Value::Double { double_value: v })
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(Value::String { string_value: v.to_string() })
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(Value::Blob { blob_value: Blob(v.to_vec()) })
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T: ? Sized>(self, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(Value::Null { null_value: () })
    }

    // Serialize struct with no data as null value - there is no data after all!
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    // Unit variants are enum members without extra fields. They are serialised simply as a string
    // value.
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok> where
        T: ? Sized + Serialize {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq((Some(len)))
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq((Some(len)))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let map_serializer = MapSerializer {
            ser: &self,
            map: HashMap::new(),
            key: Option::None,
        };

        Ok(map_serializer)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        let map_serializer = MapSerializer {
            ser: &self,
            map: HashMap::new(),
            key: Option::None,
        };

        Ok(map_serializer)
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
        Ok(self) // TODO: Presumably?
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        // Serialising a single character makes no sense in Datastore. Is that a string? An int?
        // Noone knows.
        Err(Error::UnsupportedValueType("char"))
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
        // It is unclear how to correctly tag tuple variants in Datastore at the moment.
        // Using internal tagging is strange: Does that mean that, if we serialise tuples as an
        // array value, the tag is the first element of the array? That seems silly.
        //
        // We also don't want to insert an extra entity in Datastore that contains a single array
        // value (the actual tuple) keyed with the tag, because that is cumbersome to work with.
        Err(Error::UnsupportedValueType("serde tuple variant"))
    }

    fn serialize_newtype_variant<T: ? Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        // The reasoning for not implementing this yet is exactly the same as us stated above for
        // tuple variant serialisation.
        Err(Error::UnsupportedValueType("serde newtype variant"))
    }
}

pub struct MapSerializer<'a> {
    ser: &'a Serializer,
    map: HashMap<String, Value>,
    key: Option<String>,
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()> where
        T: ? Sized + Serialize {
        let key_value: Value = key.serialize(self.ser)?;
        match key_value {
            Value::String { string_value } => {
                self.key = Option::Some(string_value);
                Ok(())
            }
            _ => Err(Error::UnsupportedKeyType()),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()> where
        T: ? Sized + Serialize {
        let serialized_value = value.serialize(self.ser)?;

        match self.key {
            // According to the Serde docs the following error should never be returned anyways as
            // serde guarantees that serialize_key is run first.
            None => Err(Error::SerialisationError("map key is missing".to_string())),
            Some(ref k) => {
                self.map.insert(k.clone(), serialized_value);
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Self::Ok> {
        let entity_value = Entity {
            properties: self.map,
        };

        Ok(Value::EntityValue { entity_value })
    }

    // TODO: Implement serialize_entry() to avoid the usage of the Option.
}

impl<'a> ser::SerializeStruct for MapSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()> where
        T: ? Sized + Serialize {
        let serialized_value = value.serialize(self.ser)?;
        self.map.insert(key.to_string(), serialized_value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let entity_value = Entity {
            properties: self.map,
        };

        Ok(Value::EntityValue { entity_value })
    }
}

impl<'a> ser::SerializeTuple for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeSeq for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ? Sized>(&mut self, key: &'static str, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ? Sized>(&mut self, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ? Sized>(&mut self, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}
