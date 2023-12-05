use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub cause: Option<Box<dyn StdError>>,
}

impl Error {
    pub fn new(message: impl Into<String>, cause: Option<Box<dyn StdError>>) -> Self {
        Error {
            message: message.into(),
            cause,
        }
    }
}

impl DeError for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error {
            message: msg.to_string(),
            cause: None,
        }
    }
}

impl SerError for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error {
            message: msg.to_string(),
            cause: None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cause) = &self.cause {
            return write!(f, "{}: {}", self.message, cause);
        }
        write!(f, "{}", self.message)
    }
}

impl StdError for Error {}
