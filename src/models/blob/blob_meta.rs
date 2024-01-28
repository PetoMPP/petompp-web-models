use crate::error::Error;
use crate::models::tag::Tags;
use crate::W;
use chrono::{DateTime, Utc};
use deref_derive::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg_attr(feature = "rocket", derive(rocket::form::FromForm))]
pub struct BlobUpload {
    pub meta: BlobMetaDto,
    pub content: Vec<u8>,
}

#[cfg(feature = "web-sys")]
#[cfg(feature = "js-sys")]
impl From<&BlobUpload> for web_sys::FormData {
    fn from(value: &BlobUpload) -> Self {
        let form = web_sys::FormData::new().unwrap();
        form.append_with_str("meta.filename", &value.meta.filename)
            .unwrap();
        form.append_with_str("meta.tags.tags", &value.meta.tags.to_string())
            .unwrap();
        form.append_with_str("meta.content_type", &value.meta.content_type)
            .unwrap();
        if let Some(cl) = &value.meta.content_language {
            form.append_with_str("meta.content_language", cl).unwrap();
        }
        for (k, v) in &*value.meta.metadata {
            form.append_with_str(format!("meta.metadata.[{}]", &k).as_str(), v)
                .unwrap();
        }
        let bytes = js_sys::Array::new();
        bytes.push(&js_sys::Uint8Array::from(value.content.as_slice()));
        let blob = web_sys::Blob::new_with_u8_array_sequence(&bytes).unwrap();
        form.append_with_blob("content", &blob).unwrap();
        form
    }
}

#[cfg(feature = "web-sys")]
#[cfg(feature = "js-sys")]
#[cfg(feature = "wasm-bindgen-futures")]
impl BlobUpload {
    pub async fn from_file(value: &web_sys::File) -> Result<Self, Error> {
        use wasm_bindgen::JsCast;
        let filename = value.name();
        let content_type = value.type_();
        let metadata = AzureMetadata::default();
        let tags = Tags::default();
        let content_language = None;
        let content = wasm_bindgen_futures::JsFuture::from(value.array_buffer())
            .await
            .and_then(|d| d.dyn_into::<js_sys::ArrayBuffer>())
            .map(|d| js_sys::Uint8Array::new(&d).to_vec())
            .map_err(|e| Error::Database(format!("{:?}", e)))?;
        Ok(Self {
            meta: BlobMetaDto {
                filename,
                metadata,
                tags,
                content_type,
                content_language,
            },
            content,
        })
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::form::FromForm))]
#[derive(Clone)]
pub struct BlobMetaDto {
    pub filename: String,
    pub metadata: AzureMetadata,
    pub tags: Tags,
    pub content_type: String,
    pub content_language: Option<String>,
}

impl From<BlobMetaData> for BlobMetaDto {
    fn from(value: BlobMetaData) -> Self {
        Self {
            filename: value.filename,
            metadata: value.metadata,
            tags: value.tags,
            content_type: value.content_type,
            content_language: value.content_language,
        }
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::form::FromForm))]
#[derive(Deref, DerefMut, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureMetadata(HashMap<String, String>);

impl Default for AzureMetadata {
    fn default() -> Self {
        Self(
            vec![
                ("BLOB_TITLE".to_string(), Default::default()),
                ("BLOB_SUMMARY".to_string(), Default::default()),
            ]
            .into_iter()
            .collect(),
        )
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::form::FromForm))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobMetaData {
    pub filename: String,
    pub metadata: AzureMetadata,
    pub tags: Tags,
    pub created: W<DateTime<Utc>>,
    pub updated: W<DateTime<Utc>>,
    pub content_type: String,
    pub content_language: Option<String>,
    pub size: u64,
}

impl BlobMetaData {
    pub fn title(&self) -> &String {
        self.metadata.get("BLOB_TITLE").unwrap()
    }

    pub fn with_title(&self, title: String) -> Self {
        let mut meta = self.clone();
        meta.metadata.insert("BLOB_TITLE".to_string(), title);
        meta
    }

    pub fn summary(&self) -> &String {
        self.metadata.get("BLOB_SUMMARY").unwrap()
    }

    pub fn with_summary(&self, summary: String) -> Self {
        let mut meta = self.clone();
        meta.metadata.insert("BLOB_SUMMARY".to_string(), summary);
        meta
    }
}

#[cfg(feature = "base64")]
#[cfg(feature = "azure_storage_blobs")]
impl TryFrom<&azure_storage_blobs::blob::Blob> for BlobMetaData {
    type Error = crate::error::Error;

    fn try_from(value: &azure_storage_blobs::blob::Blob) -> Result<Self, Self::Error> {
        use crate::{error::Error, models::blob::base64_engine::Base64Engine};
        let engine = Base64Engine::default();
        let filename = value.name.clone();
        let content_type = value.properties.content_type.clone();
        let content_language = value.properties.content_language.clone();
        let size = value.properties.content_length;
        let tags = Tags::from(value.tags.clone().unwrap_or_default());
        let created = W(DateTime::from_timestamp(
            value.properties.creation_time.unix_timestamp(),
            0,
        )
        .unwrap());
        let updated = W(DateTime::from_timestamp(
            value.properties.last_modified.unix_timestamp(),
            0,
        )
        .unwrap());
        let metadata = AzureMetadata(
            value
                .metadata
                .clone()
                .ok_or(Error::Database("File has no metadata!".to_string()))?
                .into_iter()
                .filter_map(|(k, v)| Some((k.to_uppercase(), engine.decode_string(&v)?)))
                .collect::<HashMap<_, _>>(),
        );
        let _ = metadata
            .get("BLOB_TITLE")
            .ok_or(Error::Database("File has no title!".to_string()))?
            .clone();
        let _ = metadata
            .get("BLOB_SUMMARY")
            .ok_or(Error::Database("File has no summary!".to_string()))?
            .clone();
        Ok(Self {
            filename,
            metadata,
            tags,
            created,
            updated,
            content_type,
            content_language,
            size,
        })
    }
}

#[cfg(feature = "azure_core")]
#[cfg(feature = "base64")]
impl From<&AzureMetadata> for azure_core::request_options::Metadata {
    fn from(val: &AzureMetadata) -> Self {
        use crate::models::blob::base64_engine::Base64Engine;
        use base64::engine::Engine;
        let engine = Base64Engine::default();
        let mut meta = azure_core::request_options::Metadata::new();
        for (k, v) in val.iter() {
            meta.insert(k, engine.encode(v));
        }
        meta
    }
}
