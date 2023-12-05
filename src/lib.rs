pub mod error;
pub mod utils;

use crate::error::Error;
use base64::prelude::*;
use serde::{
    de::DeserializeOwned,
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

pub fn to_string<T>(value: &T) -> Result<String, Error>
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
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            is_first: true,
            output: String::new(),
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

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
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
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
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
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
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
}
