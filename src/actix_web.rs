use crate::from_str;
use actix_web::{error::ErrorBadRequest, Error, FromRequest, HttpRequest};
use futures::future::Ready;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Query<T>(pub T);

impl<T> FromRequest for Query<T>
where
    for<'de> T: Deserialize<'de>,
{
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let query = req.query_string();
        match from_str(query).map_err(ErrorBadRequest) {
            Ok(v) => futures::future::ready(Ok(Query(v))),
            Err(e) => futures::future::ready(Err(e)),
        }
    }
}

impl<T> Serialize for Query<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Query<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Query)
    }
}
