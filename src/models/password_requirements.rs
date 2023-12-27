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
            Requirement::new("min_length", false, move |s: &&str| {
                s.len() >= min_length as usize
            })
        }];
        if self.numbers {
            reqs.push(Requirement::new("numbers", true, |s: &&str| {
                s.chars().any(|c| c.is_numeric())
            }));
        }
        if self.uppercase {
            reqs.push(Requirement::new("uppercase", true, |s: &&str| {
                s.chars().any(|c| c.is_uppercase())
            }));
        }
        if self.lowercase {
            reqs.push(Requirement::new("lowercase", true, |s: &&str| {
                s.chars().any(|c| c.is_lowercase())
            }));
        }
        if self.special {
            reqs.push(Requirement::new("special", true, |s: &&str| {
                s.chars().any(|c| !c.is_alphanumeric())
            }));
        }
        reqs
    }

    fn optional_required_count(&self) -> usize {
        self.passes_required as usize
    }
}

impl Default for PasswordRequirements {
    fn default() -> Self {
        Self {
            min_length: 8,
            passes_required: 3,
            numbers: true,
            uppercase: true,
            lowercase: true,
            special: true,
        }
    }
}
