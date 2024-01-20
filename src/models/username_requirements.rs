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
                let max_length = self.max_length;
                Requirement::new("Username_InvalidLength", false, move |s: &&str| {
                    (min_length..max_length).contains(&(s.len() as i32))
                })
            },
            {
                let special_chars = self.special_chars.clone();
                Requirement::new(
                    "Username_OnlyAlphanumericOrSelectedChars",
                    false,
                    move |s: &&str| {
                        let mut allowed = special_chars.chars();
                        s.chars()
                            .all(|c| c.is_alphanumeric() || allowed.any(|x| x == c))
                    },
                )
            },
        ]
    }

    fn optional_required_count(&self) -> usize {
        0
    }
}
