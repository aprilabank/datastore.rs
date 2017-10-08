use std::fmt::Display;
use serde::ser::{self, Serialize};
use serde_ds::error::{Error, Result};
use datastore::{Value, Entity};

pub struct Serializer {}

/*impl Serializer {
    fn get_value(&self) -> Result<Value> {
        match *self {
            Some(v) => Ok(v),
            None    => Err(error::Error::SomeError())
        }
    }
}*/

/*pub fn to_value<T: Serialize>(value: &T) -> Result<Value> {
    let serializer = Serializer{};
    value.serialize(&serializer)?
}*/

impl <'a> ser::Serializer for &'a Serializer {
    type Ok = Entity;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_some<T: ? Sized>(self, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ? Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ? Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }

    fn collect_str<T: ? Sized>(self, value: &T) -> Result<Self::Ok> where
        T: Display {
        unimplemented!()
    }
}

impl <'a> ser::SerializeMap for &'a Serializer {
    type Ok = Entity;
    type Error = Error;

    fn serialize_key<T: ? Sized>(&mut self, key: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_value<T: ? Sized>(&mut self, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTuple for &'a Serializer {
    type Ok = Entity;
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
    type Ok = Entity;
    type Error = Error;

    fn serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a Serializer {
    type Ok = Entity;
    type Error = Error;

    fn serialize_field<T: ? Sized>(&mut self, key: &'static str, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a Serializer {
    type Ok = Entity;
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
    type Ok = Entity;
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
    type Ok = Entity;
    type Error = Error;

    fn serialize_field<T: ? Sized>(&mut self, value: &T) -> Result<()> where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}
