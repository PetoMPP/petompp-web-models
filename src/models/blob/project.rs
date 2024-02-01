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

    pub fn splash(&self) -> Option<&String> {
        self.metadata.get("PROJECT_SPLASH")
    }

    pub fn set_splash(&mut self, splash: String) {
        self.metadata.insert("PROJECT_SPLASH".to_string(), splash);
    }
}

impl TryFrom<BlobMetaData> for ProjectMetaData {
    type Error = crate::error::Error;
    fn try_from(value: BlobMetaData) -> Result<Self, Self::Error> {
        Ok(Self(MarkdownMeta::try_from(value)?))
    }
}
