use super::country::Country;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ResourceData {
    pub key: String,
    pub en: Option<String>,
    pub pl: Option<String>,
}

impl ResourceData {
    pub fn new_from_lang(key: impl Into<String>, lang: &Country, value: impl Into<String>) -> Self {
        match lang {
            Country::UnitedKingdom => Self {
                key: key.into(),
                en: Some(value.into()),
                pl: None,
            },
            Country::Poland => Self {
                key: key.into(),
                en: None,
                pl: Some(value.into()),
            },
        }
    }
}
