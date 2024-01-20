use crate::models::azure_meta::AzureMetadata;
use crate::models::blob_data::BlobMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BlogData {
    pub meta: BlogMetaData,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BlogMetaData {
    pub blob: BlobMetaData,
    pub project: Option<String>,
    pub image: String,
}

#[cfg(feature = "base64")]
#[cfg(feature = "azure_storage_blobs")]
impl TryFrom<azure_storage_blobs::blob::Blob> for BlogMetaData {
    type Error = crate::error::Error;

    fn try_from(value: azure_storage_blobs::blob::Blob) -> Result<Self, Self::Error> {
        let blob = BlobMetaData::try_from(&value)?;
        let meta: AzureMetadata = value
            .metadata
            .ok_or(crate::error::Error::Database(
                "File has no metadata!".to_string(),
            ))?
            .clone()
            .into();
        let project = meta.deref().get("BLOG_PROJECT").map(|s| s.to_string());
        let image = meta.deref().get("BLOG_IMAGE").cloned().unwrap_or_default();
        Ok(Self {
            blob,
            project,
            image,
        })
    }
}

#[cfg(feature = "azure_core")]
#[cfg(feature = "base64")]
impl Into<azure_core::request_options::Metadata> for BlogMetaData {
    fn into(self) -> azure_core::request_options::Metadata {
        use base64::engine::Engine;
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::GeneralPurposeConfig::default(),
        );
        let mut meta: azure_core::request_options::Metadata = (&self.blob).into();
        if let Some(project) = self.project {
            meta.insert(
                "BLOG_PROJECT".to_string(),
                engine.encode(project.as_bytes()),
            );
        }
        meta.insert(
            "BLOG_IMAGE".to_string(),
            engine.encode(self.image.as_bytes()),
        );

        meta
    }
}
