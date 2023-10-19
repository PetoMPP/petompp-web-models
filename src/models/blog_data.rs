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
            + "_"
            + self.lang.key()
            + ".md"
    }
}

#[cfg(feature = "azure_storage_blobs")]
use crate::{error::Error, models::tag::Tag};
#[cfg(feature = "azure_storage_blobs")]
impl TryFrom<azure_storage_blobs::blob::Blob> for BlogMetaData {
    type Error = Error;

    fn try_from(value: azure_storage_blobs::blob::Blob) -> Result<Self, Self::Error> {
        let meta = &value
            .metadata
            .ok_or(Error::DatabaseError("File has no metadata!".to_string()))?
            .clone();
        let title = meta
            .get("BLOG_TITLE")
            .ok_or(Error::DatabaseError("File has no title!".to_string()))?
            .clone();
        let summary = meta
            .get("BLOG_SUMMARY")
            .ok_or(Error::DatabaseError("File has no summary!".to_string()))?
            .clone();
        let image = meta.get("BLOG_IMAGE").cloned();
        let tags = value
            .tags
            .unwrap_or_default()
            .clone()
            .into_iter()
            .filter_map(|t| match t.0.starts_with("BLOG_TAG_") {
                true => Some(Tag {
                    tag: (t.0[9..].to_string()),
                }),
                false => None,
            })
            .collect::<Vec<_>>();
        let tags = Tags::from(tags);
        let created =
            DateTime::from_timestamp(value.properties.creation_time.unix_timestamp(), 0).unwrap();
        let updated =
            DateTime::from_timestamp(value.properties.last_modified.unix_timestamp(), 0).unwrap();
        let lang_start = value
            .name
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'_')
            .map(|(i, _)| i)
            .last()
            .ok_or(Error::DatabaseError("File has no language!".to_string()))?;
        let lang_end = value
            .name
            .chars()
            .enumerate()
            .skip(lang_start)
            .find(|(_, c)| c == &'.')
            .map(|(i, _)| i)
            .ok_or(Error::DatabaseError("File has no language!".to_string()))?;
        let lang = Country::try_from(&value.name[lang_start + 1..lang_end])
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        Ok(BlogMetaData {
            title,
            tags,
            created,
            updated,
            image,
            summary,
            lang,
        })
    }
}
