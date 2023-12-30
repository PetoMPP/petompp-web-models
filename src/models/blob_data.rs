use crate::models::azure_meta::AzureMetadata;
use crate::models::country::Country;
use crate::models::tag::Tags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobData {
    pub meta: BlobMetaData,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobMetaData {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub tags: Tags,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub lang: Country,
}

#[cfg(feature = "base64")]
#[cfg(feature = "azure_storage_blobs")]
impl TryFrom<&azure_storage_blobs::blob::Blob> for BlobMetaData {
    type Error = crate::error::Error;

    fn try_from(value: &azure_storage_blobs::blob::Blob) -> Result<Self, Self::Error> {
        use crate::{error::Error, models::tag::Tag};
        let (id, lang) = value
            .name
            .split_once('/')
            .ok_or(Error::Database("File has no id!".to_string()))?;
        let id = id.to_string();
        let lang = Country::try_from(
            lang.split_once('.')
                .ok_or(Error::Database("File has no extension!".to_string()))?
                .0,
        )
        .map_err(|_| Error::Database("File has no valid lang!".to_string()))?;
        let meta: AzureMetadata = value
            .metadata
            .clone()
            .ok_or(Error::Database("File has no metadata!".to_string()))?
            .into();
        let title = meta
            .deref()
            .get("BLOB_TITLE")
            .ok_or(Error::Database("File has no title!".to_string()))?
            .clone();
        let summary = meta
            .deref()
            .get("BLOB_SUMMARY")
            .ok_or(Error::Database("File has no summary!".to_string()))?
            .clone();
        let tags = value
            .tags
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| match t.0.starts_with("BLOB_TAG_") {
                true => Some(Tag {
                    tag: t.0[9..].to_string(),
                }),
                false => None,
            })
            .collect::<Vec<_>>();
        let tags = Tags::from(tags);
        let created =
            DateTime::from_timestamp(value.properties.creation_time.unix_timestamp(), 0).unwrap();
        let updated =
            DateTime::from_timestamp(value.properties.last_modified.unix_timestamp(), 0).unwrap();
        Ok(Self {
            id,
            title,
            tags,
            created,
            updated,
            summary,
            lang,
        })
    }
}

#[cfg(feature = "azure_core")]
#[cfg(feature = "base64")]
impl Into<azure_core::request_options::Metadata> for &BlobMetaData {
    fn into(self) -> azure_core::request_options::Metadata {
        use base64::engine::Engine;
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::GeneralPurposeConfig::default(),
        );
        let mut meta = azure_core::request_options::Metadata::new();
        meta.insert("BLOG_TITLE", engine.encode(&self.title));
        meta.insert("BLOG_SUMMARY", engine.encode(&self.summary));
        meta
    }
}
