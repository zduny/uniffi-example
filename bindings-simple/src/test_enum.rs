use std::fmt::Display;

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
