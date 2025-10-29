//! 2D vector mathematics for physics calculations.
//!
//! This module provides a `Vector2D` struct with basic vector operations
//! including addition, subtraction, scalar multiplication, and magnitude calculation.

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// A 2D vector with x and y components.
///
/// # Examples
///
/// ```
/// let v = Vector2D { x: 3.0, y: 4.0 };
/// assert_eq!(v.mag(), 5.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    /// The x-component of the vector
    pub x: f32,
    /// The y-component of the vector
    pub y: f32,
}

impl Vector2D {
    /// Returns the magnitude (length) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = Vector2D { x: 3.0, y: 4.0 };
    /// assert_eq!(v.mag(), 5.0);
    /// ```
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Implements vector addition for `Vector2D`.
///
/// # Examples
///
/// ```
/// let v1 = Vector2D { x: 1.0, y: 2.0 };
/// let v2 = Vector2D { x: 3.0, y: 4.0 };
/// let result = v1 + v2;
/// assert_eq!(result.x, 4.0);
/// assert_eq!(result.y, 6.0);
/// ```
impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Implements in-place vector addition for `Vector2D`.
///
/// # Examples
///
/// ```
/// let mut v1 = Vector2D { x: 1.0, y: 2.0 };
/// let v2 = Vector2D { x: 3.0, y: 4.0 };
/// v1 += v2;
/// assert_eq!(v1.x, 4.0);
/// assert_eq!(v1.y, 6.0);
/// ```
impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

/// Implements vector subtraction for `Vector2D`.
///
/// # Examples
///
/// ```
/// let v1 = Vector2D { x: 5.0, y: 7.0 };
/// let v2 = Vector2D { x: 2.0, y: 3.0 };
/// let result = v1 - v2;
/// assert_eq!(result.x, 3.0);
/// assert_eq!(result.y, 4.0);
/// ```
impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Implements in-place vector subtraction for `Vector2D`.
///
/// # Examples
///
/// ```
/// let mut v1 = Vector2D { x: 5.0, y: 7.0 };
/// let v2 = Vector2D { x: 2.0, y: 3.0 };
/// v1 -= v2;
/// assert_eq!(v1.x, 3.0);
/// assert_eq!(v1.y, 4.0);
/// ```
impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

/// Implements scalar multiplication for `Vector2D`.
///
/// # Examples
///
/// ```
/// let v = Vector2D { x: 2.0, y: 3.0 };
/// let result = v * 2.0;
/// assert_eq!(result.x, 4.0);
/// assert_eq!(result.y, 6.0);
/// ```
impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f32) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Implements in-place scalar multiplication for `Vector2D`.
///
/// # Examples
///
/// ```
/// let mut v = Vector2D { x: 2.0, y: 3.0 };
/// v *= 2.0;
/// assert_eq!(v.x, 4.0);
/// assert_eq!(v.y, 6.0);
/// ```
impl MulAssign<f32> for Vector2D {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }
}
