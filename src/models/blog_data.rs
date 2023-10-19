use super::tag::Tags;
use crate::models::country::Country;
use crate::services::filename::FilenameService;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogMetaData {
    pub title: String,
    pub tags: Tags,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub image: Option<String>,
    pub summary: String,
    pub lang: Country,
}

impl BlogMetaData {
    pub fn filename(&self, filename_service: &FilenameService) -> String {
        self.created.format("%Y-%m-%d").to_string()
            + filename_service.sanitize(self.title.as_str()).as_str()
            + self.lang.key()
            + ".md"
    }
}
