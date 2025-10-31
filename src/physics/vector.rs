//! 2D vector mathematics for physics calculations.
//!
//! This module provides a `Vector2D` struct with basic vector operations
//! including addition, subtraction, scalar multiplication, and magnitude calculation.

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use uom::si::f32::{Length, Ratio};

/// A 2D vector with x and y components.
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: Length,
    pub y: Length,
}

impl Vector2D {
    /// Returns the magnitude (length) of the vector.
    pub fn mag(&self) -> Length {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Implements vector addition for `Vector2D`.
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
impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

/// Implements vector subtraction for `Vector2D`.
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
impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

/// Implements scalar multiplication for `Vector2D`.
impl Mul<Ratio> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: Ratio) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Implements in-place scalar multiplication for `Vector2D`.
impl MulAssign<Ratio> for Vector2D {
    fn mul_assign(&mut self, scalar: Ratio) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }
}
