use std::fmt::Display;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i32,
    pub name: String,
    pub role: RoleData,
    pub confirmed: bool,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum RoleData {
    User,
    Admin,
}

impl Display for RoleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
