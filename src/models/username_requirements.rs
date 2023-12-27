use crate::models::requirement::{Requirement, Requirements};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsernameRequirements {
    pub min_length: i32,
    pub max_length: i32,
    pub special_chars: String,
}

impl<'a> Requirements<&'a str> for UsernameRequirements {
    fn requirements(&self) -> Vec<Requirement<&'a str>> {
        vec![
            {
                let min_length = self.min_length;
                Requirement::new("min_length", false, move |s: &&str| {
                    s.len() >= min_length as usize
                })
            },
            {
                let max_length = self.max_length;
                Requirement::new("max_length", false, move |s: &&str| {
                    s.len() <= max_length as usize
                })
            },
            {
                let special_chars = self.special_chars.clone();
                Requirement::new("special_chars", false, move |s: &&str| {
                    let mut allowed = special_chars.chars();
                    s.chars()
                        .all(|c| c.is_alphanumeric() || allowed.any(|x| x == c))
                })
            },
        ]
    }

    fn optional_required_count(&self) -> usize {
        0
    }
}

impl Default for UsernameRequirements {
    fn default() -> Self {
        Self {
            min_length: 3,
            max_length: 28,
            special_chars: "-_.$@!#%^&*".to_string(),
        }
    }
}
