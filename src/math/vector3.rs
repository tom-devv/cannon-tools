use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Creates a new 3D vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    /// Creates a zero vector (0.0, 0.0, 0.0).
    pub fn zero() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }

    /// Creates a vector with all components set to 1.0.
    pub fn one() -> Self {
        Vector3::new(1.0, 1.0, 1.0)
    }

    /// Calculates the squared Euclidean length (magnitude) of the vector.
    /// This is often preferred over `length()` when comparing distances,
    /// as it avoids the expensive square root operation.
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculates the Euclidean length (magnitude) of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns a new vector that is the normalized version of this vector.
    /// A normalized vector has a length of 1.0.
    /// Returns a zero vector if the original vector has zero length.
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 1e-10 {
            // Avoid division by zero for very small lengths
            *self / len
        } else {
            Vector3::zero()
        }
    }

    /// Calculates the dot product of this vector with another vector.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the cross product of this vector with another vector.
    pub fn cross(&self, other: &Self) -> Self {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns a new vector with each component being the minimum of
    /// the corresponding components of this and another vector.
    pub fn min(&self, other: &Self) -> Self {
        Vector3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Returns a new vector with each component being the maximum of
    /// the corresponding components of this and another vector.
    pub fn max(&self, other: &Self) -> Self {
        Vector3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Performs component-wise multiplication with another vector.
    pub fn component_mul(&self, other: &Self) -> Self {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

// --- Implementations of Standard Operators ---

// Vector addition (self + other)
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

// Vector subtraction (self - other)
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// Scalar multiplication (self * scalar)
impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Scalar multiplication (scalar * self) - requires separate impl
// This allows `5.0 * my_vector` instead of just `my_vector * 5.0`
impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        Vector3::new(self * vector.x, self * vector.y, self * vector.z)
    }
}

// Scalar division (self / scalar)
impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

// Unary negation (-self)
impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}
