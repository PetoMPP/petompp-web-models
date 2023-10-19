use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ResourceData {
    pub key: String,
    pub en: Option<String>,
    pub pl: Option<String>,
}

impl ResourceData {
    pub fn new_from_lang(
        key: impl Into<String>,
        lang: &str,
        value: impl Into<String>,
    ) -> Option<Self> {
        match lang {
            "en" => Some(Self {
                key: key.into(),
                en: Some(value.into()),
                pl: None,
            }),
            "pl" => Some(Self {
                key: key.into(),
                en: None,
                pl: Some(value.into()),
            }),
            _ => None,
        }
    }
}
