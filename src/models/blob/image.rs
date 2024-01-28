use crate::error::Error;
use crate::models::blob::blob_meta::BlobMetaData;
use deref_derive::Deref;
use serde::{Deserialize, Serialize};

#[derive(Deref, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMetaData(pub BlobMetaData);

impl From<ImageMetaData> for BlobMetaData {
    fn from(val: ImageMetaData) -> Self {
        (*val).clone()
    }
}

impl TryFrom<BlobMetaData> for ImageMetaData {
    type Error = Error;
    fn try_from(val: BlobMetaData) -> Result<Self, Self::Error> {
        if !val.content_type.starts_with("image") {
            return Err(Error::Database("File is not an image!".to_string()));
        }
        Ok(Self(val))
    }
}
