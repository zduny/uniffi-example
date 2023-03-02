use vectors::Vector2;

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

    /// Person's location.
    pub location: Vector2,
}

impl Person {
    /// Create new person with [name] and [location].
    pub fn new(name: &str, location: Vector2) -> Self {
        Person {
            name: name.to_string(),
            location,
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

    /// Get person's location.
    pub fn get_location(&self) -> Vector2 {
        self.location
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
        assert_eq!(
            Person::new("Daniel", Vector2 { x: 0.0, y: 0.0 }).get_name(),
            "Daniel"
        );
    }

    #[test]
    fn test_location() {
        assert_eq!(
            Person::new("Daniel", Vector2 { x: 0.0, y: 0.0 }).get_location(),
            Vector2 { x: 0.0, y: 0.0 }
        );
    }

    #[test]
    fn test_set_name() {
        let mut person = Person::new("Daniel", Vector2 { x: 0.0, y: 0.0 });
        person.set_name("Tom");
        assert_eq!(person.get_name(), "Tom");
    }

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
