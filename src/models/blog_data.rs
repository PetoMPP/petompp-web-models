use super::tag::Tags;
use crate::models::country::Country;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogMetaData {
    pub id: String,
    pub title: String,
    pub tags: Tags,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub image: Option<String>,
    pub summary: String,
    pub lang: Country,
}

impl BlogMetaData {
    pub fn filename(&self) -> String {
        format!("blog/{}/{}.md", self.id, self.lang.key())
    }
}

#[cfg(feature = "azure_storage_blobs")]
use crate::{error::Error, models::tag::Tag};
#[cfg(feature = "azure_storage_blobs")]
use std::collections::HashMap;

#[cfg(feature = "azure_storage_blobs")]
impl TryFrom<azure_storage_blobs::blob::Blob> for BlogMetaData {
    type Error = Error;

    fn try_from(value: azure_storage_blobs::blob::Blob) -> Result<Self, Self::Error> {
        let (id, lang) = value
            .name
            .split_once('/')
            .ok_or(Error::DatabaseError("File has no id!".to_string()))?;
        let id = id.to_string();
        let lang = Country::try_from(
            lang.split_once('.')
                .ok_or(Error::DatabaseError("File has no extension!".to_string()))?
                .0,
        )
        .map_err(|_| Error::DatabaseError("File has no valid lang!".to_string()))?;
        let meta = value
            .metadata
            .ok_or(Error::DatabaseError("File has no metadata!".to_string()))?
            .clone()
            .into_iter()
            .map(|(k, v)| (k.to_uppercase(), v)) // Azure storage blobs metadata is case insensitive
            .collect::<HashMap<_, _>>();
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
        Ok(BlogMetaData {
            id,
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
