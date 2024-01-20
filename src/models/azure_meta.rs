use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use deref_derive::Deref;

#[derive(Deref, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AzureMetadata(HashMap<String, String>);

#[cfg(feature = "azure_core")]
impl From<AzureMetadata> for azure_core::request_options::Metadata {
    fn from(val: AzureMetadata) -> Self {
        let mut meta = azure_core::request_options::Metadata::new();
        for (k, v) in val.0 {
            meta.insert(k, v);
        }
        meta
    }
}

#[cfg(feature = "base64")]
impl From<HashMap<String, String>> for AzureMetadata {
    fn from(map: HashMap<String, String>) -> Self {
        use base64::engine::Engine;

        Self(
            map.into_iter()
                .map(|(k, v)| {
                    (
                        k.to_uppercase(),
                        String::from_utf8(
                            base64::engine::GeneralPurpose::new(
                                &base64::alphabet::STANDARD,
                                base64::engine::GeneralPurposeConfig::default(),
                            )
                            .decode(v)
                            .unwrap_or("Invalid base64".bytes().collect()),
                        )
                        .unwrap_or("Invalid utf8".to_string()),
                    )
                })
                .collect(),
        )
    }
}
