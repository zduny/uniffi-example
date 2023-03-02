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
