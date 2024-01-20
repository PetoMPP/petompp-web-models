use crate::models::requirement::{Requirement, Requirements};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PasswordRequirements {
    pub min_length: i32,
    pub passes_required: i32,
    pub numbers: bool,
    pub uppercase: bool,
    pub lowercase: bool,
    pub special: bool,
}

impl<'a> Requirements<&'a str> for PasswordRequirements {
    fn requirements(&self) -> Vec<Requirement<&'a str>> {
        let mut reqs = vec![{
            let min_length = self.min_length;
            Requirement::new("Password_MinLength", false, move |s: &&str| {
                s.len() >= min_length as usize
            })
        }];
        if self.numbers {
            reqs.push(Requirement::new(
                "Password_ContainsNumber",
                true,
                |s: &&str| s.chars().any(|c| c.is_numeric()),
            ));
        }
        if self.uppercase {
            reqs.push(Requirement::new(
                "Password_ContainsUppercase",
                true,
                |s: &&str| s.chars().any(|c| c.is_uppercase()),
            ));
        }
        if self.lowercase {
            reqs.push(Requirement::new(
                "Password_ContainsLowercase",
                true,
                |s: &&str| s.chars().any(|c| c.is_lowercase()),
            ));
        }
        if self.special {
            reqs.push(Requirement::new(
                "Password_ContainsSpecial",
                true,
                |s: &&str| s.chars().any(|c| !c.is_alphanumeric()),
            ));
        }
        reqs
    }

    fn optional_required_count(&self) -> usize {
        self.passes_required as usize
    }
}
