pub mod error;
use serde::de::SeqAccess;
use serde::{de::MapAccess, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;

#[derive(Debug, Clone)]
struct State {
    m: HashMap<String, Vec<String>>,
    curr_key: Option<String>,
}

#[derive(Debug, Clone)]
struct Deserializer {
    state: Rc<RefCell<State>>,
}

impl Deserializer {
    fn try_from_str(s: &str) -> Result<Self, Error> {
        let mut m = HashMap::new();
        for p in s.split('&').into_iter() {
            let mut iter = p.split('=');
            let key = iter
                .next()
                .ok_or(Error::new("invalid key value pair", None))?
                .to_string();
            let val = iter
                .next()
                .ok_or(Error::new("invalid key value pair", None))?
                .to_string();
            m.entry(key).or_insert(vec![]).push(val);
        }
        Ok(Self {
            state: Rc::new(RefCell::new(State { m, curr_key: None })),
        })
    }
}

impl<'de> SeqAccess<'de> for Deserializer {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self
            .state
            .borrow()
            .m
            .get(&self.state.borrow().curr_key.clone().unwrap())
            .is_none()
        {
            return Ok(None);
        }
        seed.deserialize(self.clone()).map(Some)
    }
}

impl<'de> MapAccess<'de> for Deserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.state.borrow().m.is_empty() {
            return Ok(None);
        }
        seed.deserialize(self.clone()).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.clone())
    }
}

impl<'de> serde::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
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
        let mut state = self.state.borrow_mut();
        let key = state
            .m
            .keys()
            .next()
            .ok_or(Error::new("empty map", None))?
            .to_owned();
        state.curr_key = Some(key.clone());
        visitor.visit_str(&key)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut state = self.state.borrow_mut();
        let curr_key = state
            .curr_key
            .clone()
            .ok_or(Error::new("empty key", None))?;

        let mut vals = state
            .m
            .remove(&curr_key)
            .ok_or(Error::new("empty value", None))?;
        if vals.is_empty() {
            return Err(Error::new("empty value", None));
        }
        let val = vals.remove(0);
        if !vals.is_empty() {
            state.m.insert(curr_key, vals);
        }
        return visitor.visit_str(&val);
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut state = self.state.borrow_mut();
        let curr_key = state
            .curr_key
            .clone()
            .ok_or(Error::new("empty key", None))?;

        let mut vals = state
            .m
            .remove(&curr_key)
            .ok_or(Error::new("empty value", None))?;
        if vals.is_empty() {
            return Err(Error::new("empty value", None));
        }
        let val = vals.remove(0);
        if !vals.is_empty() {
            state.m.insert(curr_key, vals);
        }
        return visitor.visit_string(val);
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut state = self.state.borrow_mut();
        let curr_key = state
            .curr_key
            .clone()
            .ok_or(Error::new("empty key", None))?;

        let mut vals = state
            .m
            .remove(&curr_key)
            .ok_or(Error::new("empty value", None))?;
        if vals.is_empty() {
            return Err(Error::new("empty value", None));
        }
        let val = vals.remove(0);
        if !vals.is_empty() {
            state.m.insert(curr_key, vals);
        }
        return visitor.visit_i32(
            val.parse()
                .map_err(|e| Error::new("invalid i32 literial", Some(Box::new(e))))?,
        );
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
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
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize)]
struct MyStruct {
    name: String,
    hobby: String,
    age: i32,
    ids: Vec<i32>,
}

fn main() {
    let s = "ids=1&ids=2&age=37&hobby=code&name=John";
    println!(
        "{:?}",
        MyStruct::deserialize(Deserializer::try_from_str(s).unwrap())
    );
}
