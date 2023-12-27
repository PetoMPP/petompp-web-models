use rocket::serde::{Deserialize, Serialize};
use crate::models::password_requirements::PasswordRequirements;
use crate::models::username_requirements::UsernameRequirements;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserSettingsDto {
    name_min_length: Option<i32>,
    name_max_length: Option<i32>,
    name_special_characters: Option<String>,
    password_min_length: Option<i32>,
    password_needed_checks: Option<i32>,
    password_check_numbers: Option<bool>,
    password_check_uppercase: Option<bool>,
    password_check_lowercase: Option<bool>,
    password_check_special_characters: Option<bool>,
}

impl From<(UsernameRequirements, PasswordRequirements)> for UserSettingsDto {
    fn from(value: (UsernameRequirements, PasswordRequirements)) -> Self {
        let (username_requirements, password_requirements) = value;
        Self {
            name_min_length: Some(username_requirements.min_length),
            name_max_length: Some(username_requirements.max_length),
            name_special_characters: Some(username_requirements.special_chars),
            password_min_length: Some(password_requirements.min_length),
            password_needed_checks: Some(password_requirements.passes_required),
            password_check_numbers: Some(password_requirements.numbers),
            password_check_uppercase: Some(password_requirements.uppercase),
            password_check_lowercase: Some(password_requirements.lowercase),
            password_check_special_characters: Some(password_requirements.special)
        }
    }
}
