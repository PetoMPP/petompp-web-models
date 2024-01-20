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
        use crate::{error::Error, models::azure_meta::AzureMetadata};
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
            .get("BLOB_TITLE")
            .ok_or(Error::Database("File has no title!".to_string()))?
            .clone();
        let summary = meta
            .get("BLOB_SUMMARY")
            .ok_or(Error::Database("File has no summary!".to_string()))?
            .clone();
        let tags = Tags::from(value.tags.clone().unwrap_or_default());
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
impl From<&BlobMetaData> for azure_core::request_options::Metadata {
    fn from(val: &BlobMetaData) -> Self {
        use base64::engine::Engine;
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::GeneralPurposeConfig::default(),
        );
        let mut meta = azure_core::request_options::Metadata::new();
        meta.insert("BLOB_TITLE", engine.encode(&val.title));
        meta.insert("BLOB_SUMMARY", engine.encode(&val.summary));
        meta
    }
}
