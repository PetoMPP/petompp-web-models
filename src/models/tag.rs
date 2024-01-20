use deref_derive::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Deref, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tag {
    pub tag: String,
}

#[derive(Debug, Default, Deref, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tags {
    tags: String,
}

impl IntoIterator for Tags {
    type Item = Tag;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags().into_iter()
    }
}

impl FromIterator<Tag> for Tags {
    fn from_iter<T: IntoIterator<Item = Tag>>(iter: T) -> Self {
        Self {
            tags: iter
                .into_iter()
                .map(|tag| tag.tag)
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}

impl From<Vec<Tag>> for Tags {
    fn from(tags: Vec<Tag>) -> Self {
        Self::from_iter(tags)
    }
}

#[cfg(feature = "azure_storage_blobs")]
use azure_storage_blobs::prelude::Tags as AzureTags;

#[cfg(feature = "azure_storage_blobs")]
impl From<AzureTags> for Tags {
    fn from(tags: AzureTags) -> Self {
        Self {
            tags: tags
                .into_iter()
                .filter_map(|t| match t.0.starts_with("BLOB_TAG_") {
                    true => Some(t.0[9..].to_string()),
                    false => None,
                })
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}

#[cfg(feature = "azure_storage_blobs")]
impl Into<AzureTags> for Tags {
    fn into(self) -> AzureTags {
        self.tags()
            .into_iter()
            .map(|tag| (format!("BLOB_TAG_{}", tag.tag), "".to_string()))
            .collect::<HashMap<_, _>>()
            .into()
    }
}

impl Tags {
    pub fn tags(&self) -> Vec<Tag> {
        if self.tags.is_empty() {
            return vec![];
        }
        self.tags
            .split(',')
            .map(|tag| Tag {
                tag: tag.to_string(),
            })
            .collect()
    }
}
