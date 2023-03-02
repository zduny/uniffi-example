use std::sync::Arc;

/// A scalar (1D vector) or 2/3/4-dimensional vector.
pub enum Vector {
    Scalar(vectors::Scalar),
    Vector2(vectors::Vector2),
    Vector3(vectors::Vector3),
    Vector4(vectors::Vector4),
}

impl Vector {
    pub fn scalar(s: f64) -> Self {
        Vector::Scalar(vectors::Scalar(s))
    }

    pub fn vec2(x: f64, y: f64) -> Self {
        Vector::Vector2(vectors::Vector2 { x, y })
    }

    pub fn vec3(x: f64, y: f64, z: f64) -> Self {
        Vector::Vector3(vectors::Vector3 { x, y, z })
    }

    pub fn vec4(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vector::Vector4(vectors::Vector4 { x, y, z, w })
    }

    pub fn norm(&self) -> f64 {
        vectors::Vector::norm(&self)
    }
}

impl vectors::Vector for &Vector {
    /// Vector's length.
    fn norm(&self) -> f64 {
        match self {
            Vector::Scalar(inner) => inner.norm(),
            Vector::Vector2(inner) => inner.norm(),
            Vector::Vector3(inner) => inner.norm(),
            Vector::Vector4(inner) => inner.norm(),
        }
    }
}

/// Result of comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonResult {
    /// Vectors are perfectly equal.
    Equal,

    /// Vectors lengths are close (absolute value of difference is smaller than [EPSILON]).
    Close,

    /// Left vector has greater length.
    LeftGreater,

    /// Right vector has greater length.
    RightGreater,
}

impl From<vectors::ComparisonResult> for ComparisonResult {
    fn from(value: vectors::ComparisonResult) -> Self {
        match value {
            vectors::ComparisonResult::Equal => ComparisonResult::Equal,
            vectors::ComparisonResult::Close => ComparisonResult::Close,
            vectors::ComparisonResult::LeftGreater => ComparisonResult::LeftGreater,
            vectors::ComparisonResult::RightGreater => ComparisonResult::RightGreater,
        }
    }
}

/// Compare vectors lengths.
pub fn compare_length(left: Arc<Vector>, right: Arc<Vector>) -> ComparisonResult {
    vectors::compare_length(left.as_ref(), right.as_ref()).into()
}

uniffi::include_scaffolding!("vectors");
