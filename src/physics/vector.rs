//! 2D vector mathematics for physics calculations.
//!
//! This module provides a `Vector2D` struct with basic vector operations
//! including addition, subtraction, scalar multiplication, and magnitude calculation.

use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// A generic-typed 2D vector with x and y components.
#[derive(Debug, Clone, Copy)]
pub struct Vector2D<Q> {
    pub x: Q,
    pub y: Q,
}

impl<Q> Vector2D<Q>
where
    Q: Copy + uom::num_traits::Zero,
{
    /// Initialise a zero-vector
    pub fn zero() -> Self {
        Self {
            x: Q::zero(),
            y: Q::zero(),
        }
    }
}

impl<Q> Vector2D<Q>
where
    Q: Copy + Mul<Q, Output = Q> + Add<Output = Q> + uom::num_traits::Float,
{
    /// Returns the magnitude of the vector.
    pub fn mag(&self) -> Q {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Implements vector addition for `Vector2D`.
impl<Q> Add for Vector2D<Q>
where
    Q: Add<Output = Q> + Copy,
{
    type Output = Vector2D<Q>;

    fn add(self, other: Vector2D<Q>) -> Vector2D<Q> {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Implements in-place vector addition for `Vector2D`.
impl<Q> AddAssign for Vector2D<Q>
where
    Q: AddAssign + Copy,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Implements vector subtraction for `Vector2D`.
impl<Q> Sub for Vector2D<Q>
where
    Q: Sub<Output = Q> + Copy,
{
    type Output = Vector2D<Q>;

    fn sub(self, other: Vector2D<Q>) -> Vector2D<Q> {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Implements in-place vector subtraction for `Vector2D`.
impl<Q> SubAssign for Vector2D<Q>
where
    Q: SubAssign + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

/// Implements generic scalar multiplication with `uom::Quantity` for `Vector2D`
impl<Q, S, Out> Mul<S> for Vector2D<Q>
where
    Q: Mul<S, Output = Out> + Copy,
    S: Copy,
{
    type Output = Vector2D<Out>;

    fn mul(self, rhs: S) -> Vector2D<Out> {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Implements division by generic type for `Vector2D<Q>`
impl<Q, S, Out> Div<S> for Vector2D<Q>
where
    Q: Div<S, Output = Out> + Copy,
    S: Copy,
{
    type Output = Vector2D<Out>;

    fn div(self, rhs: S) -> Vector2D<Out> {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// Due to Rust orphan rule, we cannot implement multiplicatin with `Vector2D` for generic quantity.
/// To sort-of make this scalable, we use a macro to implement multiplication for specific types.
macro_rules! impl_vector_mul {
    ($quantity:ty) => {
        impl<Q, Out> Mul<Vector2D<Q>> for $quantity
        where
            Q: Mul<$quantity, Output = Out> + Copy,
        {
            type Output = Vector2D<Out>;

            fn mul(self, rhs: Vector2D<Q>) -> Vector2D<Out> {
                Vector2D {
                    x: rhs.x * self,
                    y: rhs.y * self,
                }
            }
        }
    };
}

// Add more quantities here as needed
use uom::si::f32::{Acceleration, Force, Length, Mass, Ratio, Time, Velocity};
impl_vector_mul!(Acceleration);
impl_vector_mul!(Force);
impl_vector_mul!(Length);
impl_vector_mul!(Mass);
impl_vector_mul!(Ratio);
impl_vector_mul!(Time);
impl_vector_mul!(Velocity);
