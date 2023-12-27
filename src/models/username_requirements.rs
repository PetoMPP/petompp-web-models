use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsernameRequirements {
    pub min_length: i32,
    pub max_length: i32,
    pub special_chars: String,
}

impl UsernameRequirements {
    pub fn validate(&self, name: &str) -> Result<(), ()> {
        if !(self.min_length..self.max_length).contains(&(name.len() as i32)) {
            return Err(());
        }

        let mut allowed = self.special_chars.chars();
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || allowed.any(|x| x == c))
        {
            return Err(());
        }
        Ok(())
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
