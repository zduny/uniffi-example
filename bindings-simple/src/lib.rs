use std::{sync::Mutex, fmt::Display};

/// Pet with a name.
pub struct Pet {
    /// Pet's name.
    pub name: String,
}

impl From<Pet> for simple::Pet {
    fn from(pet: Pet) -> Self {
        simple::Pet { name: pet.name }
    }
}

/// Create hello message to a `pet`.
pub fn hello(pet: Pet) -> String {
    simple::hello(pet.into())
}

/// Person with a name.
pub struct Person {
    inner: Mutex<simple::Person>,
}

impl Person {
    /// Create new person with [name].
    pub fn new(name: String) -> Self {
        Person { inner: Mutex::new(simple::Person::new(&name)) }
    }

    /// Set person name.
    pub fn set_name(&self, name: String) {
        self.inner.lock().unwrap().set_name(&name);
    }

    /// Get person's name.
    pub fn get_name(&self) -> String {
        self.inner.lock().unwrap().get_name().to_string()
    }
}


/// Add two integers together.
pub fn add(left: u64, right: u64) -> u64 {
    simple::add(left, right)
}

/// Test enum.
pub enum TestEnum {
    A,
    B,
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
