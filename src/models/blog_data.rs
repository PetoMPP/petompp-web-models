use crate::models::azure_meta::AzureMetadata;
use crate::models::blob_data::BlobMetaData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        let mut meta: azure_core::request_options::Metadata = (&self.blob).into();
        let tags = match self.project {
            Some(project) => vec![
                ("BLOG_PROJECT".to_string(), project),
                ("BLOG_IMAGE".to_string(), self.image),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
            None => vec![("BLOG_IMAGE".to_string(), self.image)]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        };
        let blog_meta = AzureMetadata::from(tags);
        for (k, v) in blog_meta.deref() {
            meta.insert(k, v.clone());
        }

        meta
    }
}
