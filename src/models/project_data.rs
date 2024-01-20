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
        use crate::{error::Error, models::azure_meta::AzureMetadata};
        let blob = BlobMetaData::try_from(&value)?;
        let meta: AzureMetadata = value
            .metadata
            .ok_or(Error::Database("File has no metadata!".to_string()))?
            .clone()
            .into();
        let images = (*meta)
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
impl From<ProjectMetaData> for azure_core::request_options::Metadata {
    fn from(val: ProjectMetaData) -> Self {
        use base64::engine::Engine;
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::GeneralPurposeConfig::default(),
        );
        let mut meta: azure_core::request_options::Metadata = (&val.blob).into();
        for (i, image) in val.images.into_iter().enumerate() {
            meta.insert(format!("PROJECT_IMAGE_{}", i), engine.encode(image));
        }

        meta
    }
}
