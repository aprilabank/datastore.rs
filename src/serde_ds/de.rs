use datastore::Value;
use serde::Deserialize;
use serde::de::{self, Visitor};
use serde_ds::{Result, Error};
use std;

pub struct Deserializer<'de> {
    input: &'de Value,
}

/*impl<'de> Deserializer<'de> {
    pub fn from_value(input: &'de Value) -> Self {
        Deserializer { input }
    }
}*/

pub fn from_value<'de, T>(input: &'de Value) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer { input };
    T::deserialize(&mut deserializer)
}

// Conditionally unwraps the i64 contained in a Datastore Integer value (if the value is of the
// correct type).
fn int_value<'de>(input: &'de Value) -> Result<i64> {
    match *input {
        Value::Integer { ref integer_value } => Ok(integer_value.0),
        _ => Err(Error::ExpectedType("integer")),
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.input {
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
        match *self.input {
            Value::Boolean { boolean_value } => visitor.visit_bool(boolean_value),
            _ => Err(Error::ExpectedType("bool")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = int_value(self.input)?;
        if i > (std::i8::MAX as i64) {
            Err(Error::IntegerSizeMismatch())
        } else {
            visitor.visit_i8(i as i8)
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = int_value(self.input)?;
        if i > (std::i16::MAX as i64) {
            Err(Error::IntegerSizeMismatch())
        } else {
            visitor.visit_i16(i as i16)
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = int_value(self.input)?;
        if i > (std::i32::MAX as i64) {
            Err(Error::IntegerSizeMismatch())
        } else {
            visitor.visit_i32(i as i32)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(int_value(self.input)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = int_value(self.input)?;
        if i > (std::u8::MAX as i64) {
            Err(Error::IntegerSizeMismatch())
        } else {
            visitor.visit_u8(i as u8)
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = int_value(self.input)?;
        if i > (std::u16::MAX as i64) {
            Err(Error::IntegerSizeMismatch())
        } else {
            visitor.visit_u16(i as u16)
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = int_value(self.input)?;
        if i > (std::u32::MAX as i64) {
            Err(Error::IntegerSizeMismatch())
        } else {
            visitor.visit_u32(i as u32)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = int_value(self.input)?;
        visitor.visit_u64(i as u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let f = match *self.input {
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
        match *self.input {
            Value::Double { double_value } => visitor.visit_f64(double_value),
            _ => Err(Error::ExpectedType("double")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.input {
            Value::String { ref string_value } => visitor.visit_str(string_value.as_ref()),
            _ => Err(Error::ExpectedType("string")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.input {
            Value::String { ref string_value } => visitor.visit_string(string_value.clone()),
            _ => Err(Error::ExpectedType("string")),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.input {
            Value::Blob { ref blob_value } => visitor.visit_bytes(blob_value.0.as_ref()),
            _ => Err(Error::ExpectedType("blob")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.input {
            Value::Blob { ref blob_value } => visitor.visit_byte_buf(blob_value.0.clone()),
            _ => Err(Error::ExpectedType("blob")),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.input {
            Value::Null { .. } => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.input {
            Value::Null { .. } => visitor.visit_unit(),
            _ => Err(Error::ExpectedType("null")),
        }
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
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
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!() // FOO? Can this be unit?
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedValueType("char"))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedValueType("tuple"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedValueType("tuple struct"))
    }
}
