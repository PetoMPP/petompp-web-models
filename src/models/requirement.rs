pub trait Requirements<T> {
    fn requirements(&self) -> Vec<Requirement<T>>;
    fn optional_required_count(&self) -> usize;
    fn validate<'a>(&'a self, value: &'a T) -> Result<(), Vec<&str>> {
        let mut errors = vec![];
        let mut opt_errors = vec![];
        let mut passed = 0;
        let reqs = self.requirements();
        for requirement in reqs.iter() {
            match requirement.optional {
                true => match (requirement.validator)(value) {
                    true => passed += 1,
                    false => opt_errors.push(requirement.name),
                },
                false => match (requirement.validator)(value) {
                    true => {},
                    false => errors.push(requirement.name),
                },
            }
        }
        if errors.is_empty() && passed >= self.optional_required_count().min(reqs.iter().filter(|r| r.optional).count()) {
            Ok(())
        } else {
            Err(errors.into_iter().chain(opt_errors).collect())
        }
    }
}

pub struct Requirement<'a, T> {
    pub name: &'a str,
    pub optional: bool,
    pub validator: Box<dyn Fn(&T) -> bool>,
}

impl<'a, T> Requirement<'a, T> {
    pub fn new(name: &'a str, optional: bool, validator: impl Fn(&T) -> bool + 'static) -> Self {
        Self {
            name,
            optional,
            validator: Box::new(validator),
        }
    }
}