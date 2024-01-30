use crate::error::Error;
use crate::models::blob::blob_meta::BlobMetaData;
use crate::models::blob::markdown::MarkdownMeta;
use deref_derive::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use crate::models::country::Country;

#[derive(Deref, DerefMut, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogMetaData(MarkdownMeta);

impl BlogMetaData {
    pub fn empty(id: &str, lang: Country) -> Self {
        let mut meta = MarkdownMeta::empty(id, lang);
        meta.metadata
            .insert("BLOG_IMAGE".to_string(), Default::default());
        Self(meta)
    }
}

impl From<BlogMetaData> for BlobMetaData {
    fn from(val: BlogMetaData) -> Self {
        (**val).clone()
    }
}

impl BlogMetaData {
    pub fn project(&self) -> Option<&String> {
        self.metadata.get("BLOG_PROJECT")
    }

    pub fn set_project(&mut self, project: String) {
        self.metadata.insert("BLOG_PROJECT".to_string(), project);
    }

    pub fn image(&self) -> &String {
        self.metadata.get("BLOG_IMAGE").unwrap()
    }

    pub fn set_image(&mut self, image: String) {
        self.metadata.insert("BLOG_IMAGE".to_string(), image);
    }
}

impl TryFrom<BlobMetaData> for BlogMetaData {
    type Error = Error;

    fn try_from(value: BlobMetaData) -> Result<Self, Self::Error> {
        let _ = value
            .metadata
            .get("BLOG_IMAGE")
            .ok_or(Error::Database("File has no image!".to_string()))?;

        Ok(Self(MarkdownMeta::try_from(value)?))
    }
}
