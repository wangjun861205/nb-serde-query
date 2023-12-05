pub mod error;
pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Array<T>(Vec<T>);

pub fn deserialize_array<'de, T, D>(deserializer: D) -> Result<Array<T>, D::Error>
where
    for<'d> T: Serialize + Deserialize<'d>,
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let v = serde_json::from_str::<Vec<T>>(&s).map_err(serde::de::Error::custom)?;
    Ok(Array(v))
}

pub fn serialize_array<T, S>(array: &Array<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: serde::Serializer,
{
    let s = serde_json::to_string(&array.0).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

pub fn deserialize_option_array<'de, T, D>(deserializer: D) -> Result<Option<Array<T>>, D::Error>
where
    for<'d> T: Serialize + Deserialize<'d>,
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }
    let v = serde_json::from_str::<Vec<T>>(&s).map_err(serde::de::Error::custom)?;
    Ok(Some(Array(v)))
}

pub fn serialize_option_array<T, S>(
    array: &Option<Array<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: serde::Serializer,
{
    match array {
        Some(array) => serialize_array(array, serializer),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    #[serde(
        deserialize_with = "deserialize_array",
        serialize_with = "serialize_array"
    )]
    ids: Array<String>,
}

fn main() {
    let s = r#"{"ids": "[\"1\", \"2\", \"3\", \"4\", \"5\"]"}"#;
    println!("{:?}", serde_json::from_str::<MyStruct>(s).unwrap());
    println!(
        "{:}",
        serde_json::to_string(&MyStruct {
            ids: Array(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string()
            ])
        })
        .unwrap()
    );
}
