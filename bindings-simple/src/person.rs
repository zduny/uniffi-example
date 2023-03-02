use std::sync::Mutex;

/// Person with a name.
pub struct Person {
    inner: Mutex<simple::Person>,
}

impl Person {
    /// Create new person with [name].
    ///
    /// Example of multiline comment.
    pub fn new(name: String) -> Self {
        Person {
            inner: Mutex::new(simple::Person::new(&name)),
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
}
