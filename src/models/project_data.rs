use crate::models::azure_meta::AzureMetadata;
use crate::models::blob_data::BlobMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProjectData {
    pub meta: ProjectMetaData,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProjectMetaData {
    pub blob: BlobMetaData,
    pub images: Vec<String>,
}

#[cfg(feature = "base64")]
#[cfg(feature = "azure_storage_blobs")]
impl TryFrom<azure_storage_blobs::blob::Blob> for ProjectMetaData {
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
        let images = meta
            .deref()
            .iter()
            .filter_map(|(k, v)| match k.starts_with("PROJECT_IMAGE_") {
                true => Some(v.clone()),
                false => None,
            })
            .collect::<Vec<_>>();
        Ok(Self { blob, images })
    }
}

#[cfg(feature = "azure_core")]
#[cfg(feature = "base64")]
impl Into<azure_core::request_options::Metadata> for ProjectMetaData {
    fn into(self) -> azure_core::request_options::Metadata {
        use base64::engine::Engine;
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::GeneralPurposeConfig::default(),
        );
        let mut meta: azure_core::request_options::Metadata = (&self.blob).into();
        for (i, image) in self.images.into_iter().enumerate() {
            meta.insert(format!("PROJECT_IMAGE_{}", i), engine.encode(image));
        }

        meta
    }
}
