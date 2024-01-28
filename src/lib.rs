use chrono::{DateTime, Utc};
use deref_derive::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

pub mod error;
pub mod models;

#[derive(Deref, DerefMut, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct W<T>(T);

#[cfg(feature = "rocket")]
use rocket::{
    data::ToByteUnit,
    form::{DataField, ValueField},
};

#[cfg(feature = "rocket")]
#[rocket::async_trait]
impl<'v> rocket::form::FromFormField<'v> for W<DateTime<Utc>> {
    fn from_value(field: ValueField<'v>) -> rocket::form::Result<'v, Self> {
        Ok(serde_json::from_str(field.value)
            .map_err(|e| rocket::form::Error::validation(e.to_string()))?)
    }

    async fn from_data(field: DataField<'v, '_>) -> rocket::form::Result<'v, Self> {
        let limit = field
            .request
            .limits()
            .get("person")
            .unwrap_or(256.kibibytes());

        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);
        let value = String::from_utf8(bytes.into())
            .map_err(|_| rocket::form::Error::validation("invalid_str"))?;

        Ok(serde_json::from_str(&value)
            .map_err(|e| rocket::form::Error::validation(e.to_string()))?)
    }
}
