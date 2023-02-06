/// Person with a name.
pub struct Person {
    /// Person's name.
    pub name: String,
}

impl From<Person> for simple::Person {
    fn from(person: Person) -> Self {
        simple::Person { name: person.name }
    }
}

/// Create hello message to a `person`.
pub fn hello(person: Person) -> String {
    simple::hello(person.into())
}

/// Add two integers together.
pub fn add(left: u64, right: u64) -> u64 {
    simple::add(left, right)
}

uniffi::include_scaffolding!("simple");
