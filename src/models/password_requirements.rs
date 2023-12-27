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

impl PasswordRequirements {
    pub fn validate(&self, password: &str) -> Result<(), ()> {
        if password.len() < self.min_length as usize {
            return Err(());
        }
        #[allow(clippy::type_complexity)]
        let checks: Vec<Box<dyn Fn(&str) -> bool>> = vec![
            Box::new(|s: &str| self.numbers && s.chars().any(|c| c.is_numeric())),
            Box::new(|s: &str| self.uppercase && s.chars().any(|c| c.is_uppercase())),
            Box::new(|s: &str| self.lowercase && s.chars().any(|c| c.is_lowercase())),
            Box::new(|s: &str| self.special && s.chars().any(|c| !c.is_alphanumeric())),
        ];

        let mut passed = 0;
        for check in checks {
            if check(password) {
                passed += 1;
            }
            if passed >= self.passes_required as usize {
                return Ok(());
            }
        }
        Err(())
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
