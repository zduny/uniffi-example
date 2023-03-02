/// Simple error.
#[derive(Debug, thiserror::Error)]
pub enum SimpleError {
    #[error("invalid vector dimensions")]
    WrongDimensions,
}
