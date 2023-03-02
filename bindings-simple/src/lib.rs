mod error;
pub use error::SimpleError;

mod pet;
pub use pet::{Pet, hello};

mod person;
pub use person::Person;

mod test_enum;
pub use test_enum::*;

pub use bindings_vectors::Vector;

/// Add two integers together.
pub fn add(left: u64, right: u64) -> u64 {
    simple::add(left, right)
}

uniffi::include_scaffolding!("simple");
