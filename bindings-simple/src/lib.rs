mod pet;
pub use pet::*;

mod person;
pub use person::*;

mod test_enum;
pub use test_enum::*;

mod add;
pub use add::*;

uniffi::include_scaffolding!("simple");
