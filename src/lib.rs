pub mod error;
pub mod utils;

use crate::error::Error;
use base64::prelude::*;
use serde::{
    de::{DeserializeOwned, MapAccess, SeqAccess},
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Deserialize, Serialize,
};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Array<T>(pub Vec<T>);

impl<T> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, T> Deserialize<'de> for Array<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let v = serde_json::from_str::<Vec<T>>(&s).map_err(serde::de::Error::custom)?;
        Ok(Array(v))
    }
}

pub fn to_string<T>(value: T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<T> Serialize for Array<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = serde_json::to_string(&self.0).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Default)]
pub struct Serializer {
    is_first: bool,
    output: String,
    curr_key: Option<String>,
    is_first_elem_of_seq: bool,
    is_first_elem_of_struct: bool,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            is_first: true,
            output: String::new(),
            curr_key: None,
            is_first_elem_of_seq: false,
            is_first_elem_of_struct: false,
        }
    }
}

impl SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_first {
            self.output.push('&');
        }
        self.is_first = false;
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output.push('=');
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if self.curr_key.is_none() {
            return Err(Error::new("empty key for sequence", None));
        }
        let key = self.curr_key.clone().unwrap();
        if self.is_first_elem_of_seq {
            while let Some(c) = self.output.pop() {
                if c == '&' {
                    break;
                }
            }
            self.is_first_elem_of_seq = false;
        }
        if !self.is_first {
            self.output.push('&');
        }
        self.output.push_str(&key);
        self.output.push('=');
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.is_first_elem_of_struct {
            while let Some(c) = self.output.pop() {
                if c == '&' {
                    break;
                }
            }
            self.is_first_elem_of_struct = false;
        }
        self.curr_key = Some(key.to_string());
        if !self.is_first {
            self.output.push('&');
        }
        self.is_first = false;
        key.serialize(&mut **self)?;
        self.output.push('=');
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.is_first {
            self.output.push('&');
        }
        self.is_first = false;
        key.serialize(&mut **self)?;
        self.output.push('=');
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl serde::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeMap = Self;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&BASE64_STANDARD.encode(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        while let Some(c) = self.output.pop() {
            if c == '&' {
                break;
            }
        }
        if self.output.is_empty() {
            self.is_first = true;
        }
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.is_first_elem_of_seq = true;
        Ok(self)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(v);
        Ok(())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.is_first_elem_of_struct = true;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

use serde::de::Visitor;
use std::collections::HashMap;
pub struct Deserializer {
    m: HashMap<String, Vec<String>>,
    curr_key: Option<String>,
    curr_val: Option<Vec<String>>,
    fields: Vec<String>,
}

impl Deserializer {
    pub fn try_from_str(s: &str) -> Result<Self, Error> {
        let m = s.split('&').map(|p| p.split('=')).try_fold(
            HashMap::new(),
            |mut m: HashMap<String, Vec<String>>, mut p| {
                let key = p.next().ok_or(Error::new("invalid key", None))?;
                let val = p.next().ok_or(Error::new("invalid value", None))?;
                if p.next().is_some() {
                    return Err(Error::new("invalid pair", None));
                }
                m.entry(key.to_string()).or_default().push(val.to_string());
                Ok(m)
            },
        )?;
        Ok(Self {
            m,
            curr_key: None,
            curr_val: None,
            fields: Vec::new(),
        })
    }
}

impl<'de> MapAccess<'de> for Deserializer {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if let Some(k) = self.fields.pop() {
            self.curr_key = Some(k);
            return seed.deserialize(self).map(Some);
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let k = self.curr_key.take().ok_or(Error::new("no key", None))?;
        if let Some(v) = self.m.remove(&k) {
            self.curr_val = Some(v);
            return seed.deserialize(self);
        }
        seed.deserialize(self)
    }
}

impl<'de> SeqAccess<'de> for Deserializer {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(vals) = &mut self.curr_val {
            if vals.is_empty() {
                return Ok(None);
            }
            let val = vals.remove(0);
            let mut next_deserializer = Deserializer {
                m: self.m.clone(),
                curr_key: None,
                curr_val: Some(vec![val]),
                fields: vec![],
            };
            return seed.deserialize(&mut next_deserializer).map(Some);
        }
        Ok(None)
    }
}

impl<'de> serde::Deserializer<'de> for &mut Deserializer {
    type Error = Error;

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut next_deserializer = Deserializer {
            m: self.m.clone(),
            curr_key: None,
            curr_val: None,
            fields: fields.iter().map(|s| s.to_string()).collect(),
        };
        next_deserializer.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(&self.curr_key.clone().ok_or(Error::new("no key", None))?)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(
            self.curr_val
                .take()
                .ok_or(Error::new("no bool value", None))?
                .first()
                .ok_or(Error::new("no bool value", None))?
                .parse()
                .map_err(|e| Error::new("invalid bool literial", Some(Box::new(e))))?,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(
            self.curr_val
                .take()
                .ok_or(Error::new("no i32 value", None))?
                .first()
                .ok_or(Error::new("no i32 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid i32 literial", Some(Box::new(e))))?,
        )
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(
            self.curr_val
                .take()
                .ok_or(Error::new("no string value", None))?
                .first()
                .ok_or(Error::new("no string value", None))?
                .clone(),
        )
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(val) = self.curr_val.take() {
            if val.is_empty() {
                return visitor.visit_none();
            }
            let mut next_deserializer = Deserializer {
                m: self.m.clone(),
                curr_key: None,
                curr_val: Some(val),
                fields: vec![],
            };
            return visitor.visit_some(&mut next_deserializer);
        }
        visitor.visit_none()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(
            self.curr_val
                .take()
                .ok_or(Error::new("no i8 value", None))?
                .first()
                .ok_or(Error::new("no i8 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid i8 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(
            self.curr_val
                .take()
                .ok_or(Error::new("no i16 value", None))?
                .first()
                .ok_or(Error::new("no i16 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid i16 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(
            self.curr_val
                .take()
                .ok_or(Error::new("no f32 value", None))?
                .first()
                .ok_or(Error::new("no f32 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid f32 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(
            self.curr_val
                .take()
                .ok_or(Error::new("no f64 value", None))?
                .first()
                .ok_or(Error::new("no f64 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid f64 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i128(
            self.curr_val
                .take()
                .ok_or(Error::new("no i128 value", None))?
                .first()
                .ok_or(Error::new("no i128 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid i128 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(
            self.curr_val
                .take()
                .ok_or(Error::new("no i64 value", None))?
                .first()
                .ok_or(Error::new("no i64 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid i64 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(
            self.curr_val
                .take()
                .ok_or(Error::new("no string value", None))?
                .first()
                .ok_or(Error::new("no string value", None))?,
        )
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(
            self.curr_val
                .take()
                .ok_or(Error::new("no u128 value", None))?
                .first()
                .ok_or(Error::new("no u128 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid u128 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(
            self.curr_val
                .take()
                .ok_or(Error::new("no u16 value", None))?
                .first()
                .ok_or(Error::new("no u16 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid u16 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(
            self.curr_val
                .take()
                .ok_or(Error::new("no u32 value", None))?
                .first()
                .ok_or(Error::new("no u32 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid u32 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(
            self.curr_val
                .take()
                .ok_or(Error::new("no u64 value", None))?
                .first()
                .ok_or(Error::new("no u64 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid u64 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(
            self.curr_val
                .take()
                .ok_or(Error::new("no u8 value", None))?
                .first()
                .ok_or(Error::new("no u8 value", None))?
                .parse()
                .map_err(|e| Error::new("invalid u8 literal", Some(Box::new(e))))?,
        )
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

pub fn from_str<T>(s: &str) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::try_from_str(s)?;
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Pagination {
        limit: i32,
        offset: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct MyStruct {
        name: String,
        age: i32,
        ids: Array<String>,
        #[serde(flatten)]
        pagination: Option<Pagination>,
    }

    #[test]
    fn test_serde() {
        let s = r#"{"ids": "[\"1\", \"2\", \"3\", \"4\", \"5\"]"}"#;
        println!("{:?}", serde_json::from_str::<MyStruct>(s).unwrap());
        println!(
            "{:}",
            serde_json::to_string(&MyStruct {
                name: "test".to_string(),
                age: 37,
                ids: Array(vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string()
                ]),
                pagination: None,
            })
            .unwrap()
        );
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct NormalVec {
        ids: Vec<String>,
    }

    #[test]
    fn test_serde_normal_vec() {
        let v = NormalVec {
            ids: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        };
        println!("{}", to_string(&v).unwrap());
    }

    #[test]
    fn test_serializer() {
        let s = MyStruct {
            name: "test".to_string(),
            age: 37,
            ids: Array(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            pagination: Some(Pagination {
                limit: 10,
                offset: 0,
            }),
        };
        println!("{}", to_string(&s).unwrap());
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Empty {
        a: Option<String>,
        b: Option<i32>,
        c: Option<Array<String>>,
    }

    #[test]
    fn test_serialize_empty() {
        let s = Empty {
            a: None,
            b: None,
            c: None,
        };
        println!("{}", to_string(&s).unwrap());
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct De {
        name: String,
        age: i32,
        pagination: Pagination,
        ids: Vec<i32>,
        hobbies: Option<Vec<String>>,
        op: Option<String>,
    }

    #[test]
    fn test_deserializer() {
        let mut deserializer = Deserializer::try_from_str(
            "age=37&name=test&offset=0&limit=10&ids=1&ids=2&op=some&hobbies=moto&hobbies=code",
        )
        .unwrap();
        let de = De::deserialize(&mut deserializer).unwrap();
        assert!(de.name == "test");
        assert!(de.age == 37);
        assert!(
            de.pagination
                == Pagination {
                    limit: 10,
                    offset: 0
                }
        );
        assert!(de.ids == vec![1, 2]);
        assert!(de.op == Some("some".into()));
        assert!(de.hobbies == Some(vec!["moto".into(), "code".into()]));
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Se {
        name: String,
        age: i32,
        pagination: Pagination,
        ids: Vec<i32>,
        hobbies: Option<Vec<String>>,
        op: Option<String>,
    }

    #[test]
    fn test_to_string() {
        let se = Se {
            name: "test".into(),
            age: 37,
            pagination: Pagination {
                limit: 10,
                offset: 0,
            },
            ids: vec![1, 2],
            hobbies: Some(vec!["moto".into(), "code".into()]),
            op: Some("some".into()),
        };
        let s = to_string(&se).unwrap();
        assert!(
            s == "name=test&age=37&limit=10&offset=0&ids=1&ids=2&hobbies=moto&hobbies=code&op=some"
        )
    }
}
