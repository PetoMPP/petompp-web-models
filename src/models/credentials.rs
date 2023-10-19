use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Credentials {
    pub name: String,
    pub password: String,
}
