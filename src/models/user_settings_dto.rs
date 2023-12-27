use crate::models::password_requirements::PasswordRequirements;
use crate::models::username_requirements::UsernameRequirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserSettingsDto {
    pub name_min_length: Option<i32>,
    pub name_max_length: Option<i32>,
    pub name_special_characters: Option<String>,
    pub password_min_length: Option<i32>,
    pub password_needed_checks: Option<i32>,
    pub password_check_numbers: Option<bool>,
    pub password_check_uppercase: Option<bool>,
    pub password_check_lowercase: Option<bool>,
    pub password_check_special_characters: Option<bool>,
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
            password_check_special_characters: Some(password_requirements.special),
        }
    }
}

impl TryInto<(UsernameRequirements, PasswordRequirements)> for UserSettingsDto {
    type Error = ();

    fn try_into(self) -> Result<(UsernameRequirements, PasswordRequirements), Self::Error> {
        Ok((
            UsernameRequirements {
                min_length: self.name_min_length.ok_or(())?,
                max_length: self.name_max_length.ok_or(())?,
                special_chars: self.name_special_characters.ok_or(())?,
            },
            PasswordRequirements {
                min_length: self.password_min_length.ok_or(())?,
                passes_required: self.password_needed_checks.ok_or(())?,
                numbers: self.password_check_numbers.ok_or(())?,
                uppercase: self.password_check_uppercase.ok_or(())?,
                lowercase: self.password_check_lowercase.ok_or(())?,
                special: self.password_check_special_characters.ok_or(())?,
            },
        ))
    }
}
