/// Implemented by structures with length.
pub trait Vector {
    /// Return vector's length.
    fn norm(&self) -> f64;
}

/// Scalar value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scalar(pub f64);

impl Vector for Scalar {
    fn norm(&self) -> f64 {
        self.0
    }
}

/// 2D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    /// Value of `x` coordinate.
    pub x: f64,

    /// Value of `y` coordinate.
    pub y: f64,
}

impl Vector for Vector2 {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// 3D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    /// Value of `x` coordinate.
    pub x: f64,

    /// Value of `y` coordinate.
    pub y: f64,

    /// Value of `z` coordinate.
    pub z: f64,
}

impl Vector for Vector3 {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// 4D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    /// Value of `x` coordinate.
    pub x: f64,

    /// Value of `y` coordinate.
    pub y: f64,

    /// Value of `z` coordinate.
    pub z: f64,

    /// Value of `w` coordinate.
    pub w: f64,
}

impl Vector for Vector4 {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}

/// Result of [compare_length] function.
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

pub const EPSILON: f64 = 0.00001;

pub fn compare_length<A, B>(left: A, right: B) -> ComparisonResult
where
    A: Vector,
    B: Vector,
{
    let left = left.norm();
    let right = right.norm();

    if left == right {
        ComparisonResult::Equal
    } else if (right - left).abs() < EPSILON {
        ComparisonResult::Close
    } else if left > right {
        ComparisonResult::LeftGreater
    } else {
        ComparisonResult::RightGreater
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        assert_eq!(Scalar(40.0).norm(), 40.0);
        assert_eq!(Vector2 { x: 3.0, y: 4.0 }.norm(), 5.0);
        assert_eq!(
            Vector3 {
                x: 3.0,
                y: 0.0,
                z: 4.0
            }
            .norm(),
            5.0
        );
        assert_eq!(
            Vector4 {
                x: 0.0,
                y: 4.0,
                z: 3.0,
                w: 0.0
            }
            .norm(),
            5.0
        );
    }

    #[test]
    fn test_compare_length() {
        assert_eq!(
            compare_length(Scalar(2.0), Scalar(2.0)),
            ComparisonResult::Equal
        );
        assert_eq!(
            compare_length(Vector2 { x: 3.0, y: 5.0 }, Vector2 { x: 3.0, y: 4.0 }),
            ComparisonResult::LeftGreater
        );
        assert_eq!(
            compare_length(
                Vector3 {
                    x: 3.0,
                    y: 0.0,
                    z: 4.0
                },
                Vector3 {
                    x: 3.0,
                    y: 1.0,
                    z: 4.0
                }
            ),
            ComparisonResult::RightGreater
        );
        assert_eq!(
            compare_length(Scalar(4.5), Scalar(4.500001)),
            ComparisonResult::Close
        );
    }
}
