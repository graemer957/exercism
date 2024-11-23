use rand::distributions::Uniform;
use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let new_name = Robot::generate_unique_name();
        USED_NAMES.with(|names| names.borrow_mut().remove(&self.name));
        self.name = new_name;
    }

    fn generate_unique_name() -> String {
        let mut rng = rand::thread_rng();
        let chars = Uniform::new_inclusive(b'A', b'Z');
        let mut generate_name = || {
            format!(
                "{}{}{:03}",
                chars.sample(&mut rng) as char,
                chars.sample(&mut rng) as char,
                &rng.gen_range(0..=999)
            )
        };

        USED_NAMES.with(|names| {
            let mut used = names.borrow_mut();

            loop {
                let name = generate_name();

                if !used.contains(&name) {
                    used.insert(name.clone());

                    return name;
                }
            }
        })
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self {
            name: Robot::generate_unique_name(),
        }
    }
}

impl Drop for Robot {
    fn drop(&mut self) {
        USED_NAMES.with(|names| names.borrow_mut().remove(&self.name));
    }
}
