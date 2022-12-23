use std::fmt;

pub struct Task {
    name: String,
    description: String,
}

impl Task {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", &self.name, &self.description)
    }
}
