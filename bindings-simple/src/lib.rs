use bindings_vectors::Vector;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

/// Simple error.
#[derive(Debug, thiserror::Error)]
pub enum SimpleError {
    #[error("invalid vector dimensions")]
    WrongDimensions,
}

/// Pet with a name.
///
/// Multiline description.
pub struct Pet {
    /// Pet's name.
    pub name: String,
}

impl From<Pet> for simple::Pet {
    fn from(pet: Pet) -> Self {
        simple::Pet { name: pet.name }
    }
}

/// Create hello message to a pet.
///
/// # Arguments
///
/// - `pet` - pet to create a message to.
///
/// # Returns
///
/// Hello message to a pet.
pub fn hello(pet: Pet) -> String {
    simple::hello(pet.into())
}

/// Person with a name.
pub struct Person {
    inner: Mutex<simple::Person>,
}

impl Person {
    /// Create new person with [name].
    ///
    /// Example of multiline comment.
    pub fn new(name: String, location: Arc<Vector>) -> Result<Self, SimpleError> {
        if let Vector::Vector2(location) = location.as_ref() {
            Ok(Person {
                inner: Mutex::new(simple::Person::new(&name, location.clone())),
            })
        } else {
            Err(SimpleError::WrongDimensions)
        }
    }

    /// Set person name.
    ///
    /// # Arguments
    ///
    /// - `name` - person's name.
    pub fn set_name(&self, name: String) {
        self.inner.lock().unwrap().set_name(&name);
    }

    /// Get person's name.
    ///
    /// Example of multiline comment.
    pub fn get_name(&self) -> String {
        self.inner.lock().unwrap().get_name().to_string()
    }

    /// Get person's location.
    pub fn get_location(&self) -> Arc<Vector> {
        Arc::new(Vector::Vector2(self.inner.lock().unwrap().get_location()))
    }
}

/// Add two integers together.
pub fn add(left: u64, right: u64) -> u64 {
    simple::add(left, right)
}

/// Test enum.
pub enum TestEnum {
    /// Letter `A`.
    A,

    /// Letter `B`.
    B,

    /// Letter `C`.
    C,
}

impl Display for TestEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestEnum::A => write!(f, "A"),
            TestEnum::B => write!(f, "B"),
            TestEnum::C => write!(f, "C"),
        }
    }
}

/// Convert enum to corresponding letter string.
pub fn test_enum_to_string(test_enum: TestEnum) -> String {
    test_enum.to_string()
}

uniffi::include_scaffolding!("simple");
