use crate::models::blob::blob_meta::BlobMetaData;
use crate::models::blob::markdown::MarkdownMeta;
use crate::models::country::Country;
use deref_derive::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProjectData {
    pub meta: ProjectMetaData,
    pub content: String,
}

#[derive(Deref, DerefMut, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMetaData(MarkdownMeta);

impl From<ProjectMetaData> for BlobMetaData {
    fn from(val: ProjectMetaData) -> Self {
        (**val).clone()
    }
}

impl ProjectMetaData {
    pub fn empty(id: &str, lang: Country) -> Self {
        Self(MarkdownMeta::empty(id, lang))
    }

    pub fn images(&self) -> Vec<&String> {
        let reg = regex::Regex::new(r"^PROJECT_IMAGE_(\d+)").unwrap();
        let mut images = self
            .metadata
            .iter()
            .filter_map(|(k, v)| reg.captures(k).map(|c| (c[1].parse::<usize>().unwrap(), v)))
            .collect::<Vec<_>>();
        images.sort_by(|(a, _), (b, _)| a.cmp(b));
        images.into_iter().map(|(_, v)| v).collect()
    }

    pub fn with_images(&self, images: Vec<String>) -> Self {
        let mut meta = (**self).clone();
        for (i, image) in images.into_iter().enumerate() {
            meta.metadata.insert(format!("PROJECT_IMAGE_{}", i), image);
        }
        Self(meta)
    }
}

impl TryFrom<BlobMetaData> for ProjectMetaData {
    type Error = crate::error::Error;
    fn try_from(value: BlobMetaData) -> Result<Self, Self::Error> {
        Ok(Self(MarkdownMeta::try_from(value)?))
    }
}
