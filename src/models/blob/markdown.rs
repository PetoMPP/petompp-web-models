use crate::error::Error;
use crate::models::blob::blob_meta::BlobMetaData;
use crate::models::country::Country;
use deref_derive::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

pub struct MarkdownData {
    pub meta: MarkdownMeta,
    pub content: String,
}

/// The metadata of a markdown file.
/// Those are stored as "id/lang.md"
#[derive(Deref, DerefMut, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkdownMeta(pub BlobMetaData);

impl From<MarkdownMeta> for BlobMetaData {
    fn from(val: MarkdownMeta) -> Self {
        (*val).clone()
    }
}

impl MarkdownMeta {
    pub fn create_filename(id: &str, lang: Country) -> String {
        format!("{}/{}.md", id, lang.key())
    }

    pub fn empty(id: &str, lang: Country) -> Self {
        Self(BlobMetaData {
            filename: Self::create_filename(id, lang),
            content_type: "text/markdown".to_string(),
            content_language: Some(lang.key().to_string()),
            ..Default::default()
        })
    }

    pub fn id(&self) -> &str {
        self.filename.split_once('/').unwrap().0
    }

    pub fn lang(&self) -> Country {
        Country::try_from(
            self.filename
                .split_once('/')
                .unwrap()
                .1
                .split_once('.')
                .unwrap()
                .0,
        )
        .unwrap()
    }
}

impl TryFrom<BlobMetaData> for MarkdownMeta {
    type Error = Error;
    fn try_from(val: BlobMetaData) -> Result<Self, Self::Error> {
        let c = val
            .filename
            .split_once('/')
            .ok_or(Error::Database("File has no id!".to_string()))?
            .1
            .split_once('.')
            .ok_or("File has no lang!")
            .and_then(|(l, _)| Country::try_from(l))
            .map_err(|e| Error::Database(format!("\"{}\" is not a valid lang", e)))?;
        match &val.content_language {
            Some(_) => Ok(Self(val)),
            None => Ok(Self(BlobMetaData {
                content_language: Some(c.key().to_string()),
                ..val
            })),
        }
    }
}
