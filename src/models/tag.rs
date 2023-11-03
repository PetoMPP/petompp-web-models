use deref_derive::Deref;
use serde::{Deserialize, Serialize};

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
