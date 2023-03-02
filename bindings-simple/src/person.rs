use std::sync::{Mutex, Arc};

use bindings_vectors::Vector;

use crate::SimpleError;


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
