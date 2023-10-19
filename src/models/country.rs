use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

pub use strum::IntoEnumIterator as into_iter;

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, EnumIter, Display)]
pub enum Country {
    #[default]
    UnitedKingdom,
    Poland,
}

impl Country {
    pub fn key(&self) -> &str {
        match self {
            Self::UnitedKingdom => "en",
            Self::Poland => "pl",
        }
    }

    #[cfg(feature = "web_sys")]
    pub fn get_current() -> Self {
        use web_sys::window;
        for lang in window().unwrap().navigator().languages().to_vec() {
            let lang = lang.as_string().unwrap().to_lowercase();
            if lang.len() < 2 {
                continue;
            }
            if let Ok(country) = Self::try_from(&lang[..2]) {
                return country;
            }
        }
        Self::default()
    }
}

impl<'a> TryFrom<&'a str> for Country {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "pl" => Ok(Self::Poland),
            "en" => Ok(Self::UnitedKingdom),
            _ => Err(value),
        }
    }
}

impl TryFrom<String> for Country {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str()).map_err(|s| s.to_string())
    }
}

#[cfg(feature = "country_timeago")]
impl timeago::Language for Country {
    fn too_low(&self) -> &'static str {
        match self {
            Country::UnitedKingdom => timeago::languages::english::English.too_low(),
            Country::Poland => timeago::languages::polish::Polish.too_low(),
        }
    }

    fn too_high(&self) -> &'static str {
        match self {
            Country::UnitedKingdom => timeago::languages::english::English.too_high(),
            Country::Poland => timeago::languages::polish::Polish.too_high(),
        }
    }

    fn ago(&self) -> &'static str {
        match self {
            Country::UnitedKingdom => timeago::languages::english::English.ago(),
            Country::Poland => timeago::languages::polish::Polish.ago(),
        }
    }

    fn get_word(&self, tu: timeago::TimeUnit, x: u64) -> &'static str {
        match self {
            Country::UnitedKingdom => timeago::languages::english::English.get_word(tu, x),
            Country::Poland => timeago::languages::polish::Polish.get_word(tu, x),
        }
    }

    fn place_ago_before(&self) -> bool {
        match self {
            Country::UnitedKingdom => timeago::languages::english::English.place_ago_before(),
            Country::Poland => timeago::languages::polish::Polish.place_ago_before(),
        }
    }

    fn clone_boxed(&self) -> timeago::BoxedLanguage {
        match self {
            Country::UnitedKingdom => timeago::languages::english::English.clone_boxed(),
            Country::Poland => timeago::languages::polish::Polish.clone_boxed(),
        }
    }
}
