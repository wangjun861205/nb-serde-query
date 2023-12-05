pub mod error;
pub mod utils;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct MyStruct {
        ids: Array<String>,
    }

    #[test]
    fn test_serde() {
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
}
