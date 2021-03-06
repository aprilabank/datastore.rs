use datastore::{Int, Value};
use serde::Deserialize;
use serde::de::{self, Visitor, MapAccess, DeserializeSeed, SeqAccess};
use serde_ds::{Result, Error};
use std;
use std::vec;
use std::collections::hash_map;

pub struct Deserializer {
    input: Value,
}

pub fn from_value<'de, T: Deserialize<'de>>(input: Value) -> Result<T> {
    let deserializer = Deserializer { input };
    T::deserialize(&deserializer)
}

fn int_value<'de>(input: &'de Value) -> Result<&'de Int> {
    match *input {
        Value::Integer { ref integer_value } => Ok(integer_value),
        _ => Err(Error::ExpectedType("integer"))
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::Null { .. } => self.deserialize_unit(visitor),
            Value::String { .. } => self.deserialize_string(visitor),
            Value::Integer { .. } => self.deserialize_i64(visitor),
            Value::Double { .. } => self.deserialize_f64(visitor),
            Value::Boolean { .. } => self.deserialize_bool(visitor),
            Value::Blob { .. } => self.deserialize_bytes(visitor),
            Value::Array { .. } => self.deserialize_seq(visitor),

            // Non-primitive types (entity, key, geo types etc.) don't have an obvious match.
            _ => Err(Error::NonSelfDescribingType()),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::Boolean { boolean_value } => visitor.visit_bool(boolean_value),
            _ => Err(Error::ExpectedType("bool")),
        }
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_i8(i)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_i16(i)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_i32(i)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_i64(i)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_u8(i)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_u16(i)
    }


    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_u32(i)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let i = int_value(&self.input)?.parse()?;
        visitor.visit_u64(i)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let f = match self.input {
            Value::Double { ref double_value } => Ok(*double_value),
            _ => Err(Error::ExpectedType("double")),
        }?;

        if f > (std::f32::MAX as f64) {
            Err(Error::DoubleSizeMismatch())
        } else {
            visitor.visit_f32(f as f32)
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::Double { double_value } => visitor.visit_f64(double_value),
            _ => Err(Error::ExpectedType("double")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::String { ref string_value } => visitor.visit_str(string_value.as_ref()),
            _ => Err(Error::ExpectedType("string")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::String { ref string_value } => visitor.visit_string(string_value.clone()),
            _ => Err(Error::ExpectedType("string")),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::Blob { ref blob_value } => visitor.visit_bytes(blob_value.0.as_ref()),
            _ => Err(Error::ExpectedType("blob")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::Blob { ref blob_value } => visitor.visit_byte_buf(blob_value.0.clone()),
            _ => Err(Error::ExpectedType("blob")),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::Null { .. } => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.input {
            Value::Null { .. } => visitor.visit_unit(),
            _ => Err(Error::ExpectedType("null")),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_seq(ArrayAccess::new(&self.input)?)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(EntityAccess::new(&self.input)?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_map(EntityAccess::new(&self.input)?)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::NotYetImplemented("enum deserialization"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        // TODO: What is this even for?
        Err(Error::NotYetImplemented("ignored value deserialization"))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::UnsupportedValueType("char"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::NotYetImplemented("tuple deserialization"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::UnsupportedValueType("tuple struct deserialization"))
    }
}

struct ArrayAccess {
    iter: vec::IntoIter<Value>,
}

impl ArrayAccess {
    fn new(v: &Value) -> Result<Self> {
        let array = match *v {
            Value::Array { ref array_value } => Ok(array_value.clone().values),
            _ => Err(Error::ExpectedType("array")),
        }?;

        let iter = array.into_iter();

        Ok(ArrayAccess{ iter })
    }
}

impl<'de> SeqAccess<'de> for ArrayAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>> where
        T: DeserializeSeed<'de> {
        let deserializer = match self.iter.next() {
            None => return Ok(None),
            Some(v) => Deserializer { input: v }
        };

        seed.deserialize(&deserializer).map(Some)
    }
}


struct EntityAccess {
    iter: hash_map::IntoIter<String, Value>,
    next_value: Option<Value>,
}

impl EntityAccess {
    fn new(v: &Value) -> Result<Self> {
        let entity = match *v {
            Value::EntityValue { ref entity_value } => Ok(entity_value),
            _ => Err(Error::ExpectedType("entity")),
        }?;

        let iter = entity.clone().properties.into_iter();

        Ok(EntityAccess {
            iter,
            next_value: None,
        })
    }
}

impl<'de> MapAccess<'de> for EntityAccess {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        let next = self.iter.next();
        match next {
            // No more elements in the map!
            None => Ok(None),

            Some((k, v)) => {
                // Keep the value around for the value-deserialization step.
                self.next_value = Option::Some(v);

                // Key needs to wrapped in a value unfortunately. This will change in a future
                // refactoring.
                let kv = Value::from(k);
                let key_deserializer = Deserializer { input: kv };
                seed.deserialize(&key_deserializer).map(Some)
            }
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        // Calling next_value_seed before next_key_seed is undefined behaviour in Serde and is
        // therefore allowed to panic:
        // https://docs.serde.rs/serde/de/trait.MapAccess.html#panics
        let input = self.next_value.clone().expect("next_value_seed called before next_key_seed");
        self.next_value = None;

        let val_deserializer = Deserializer { input };
        seed.deserialize(&val_deserializer)
    }
}
