/// Pet with a name.
pub struct Pet {
    /// Pet's name.
    pub name: String,
}

/// Create hello message to a `pet`.
pub fn hello(pet: Pet) -> String {
    let name = pet.name;
    format!("Hello {name}!")
}

/// Person with a name.
pub struct Person {
    /// Person's name.
    pub name: String,
}

impl Person {
    /// Create new person with [name].
    pub fn new(name: &str) -> Self {
        Person {
            name: name.to_string(),
        }
    }

    /// Set person name.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Get person's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// Add two integers together.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(
            hello(Pet {
                name: "Milo".to_string()
            }),
            "Hello Milo!"
        );
    }

    #[test]
    fn test_get_name() {
        assert_eq!(Person::new("Daniel").get_name(), "Daniel");
    }

    #[test]
    fn test_set_name() {
        let mut person = Person::new("Daniel");
        person.set_name("Tom");
        assert_eq!(person.get_name(), "Tom");
    }

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
