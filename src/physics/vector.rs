//! 2D vector mathematics for physics calculations.
//!
//! This module provides a `Vector2D` struct with basic vector operations
//! including addition, subtraction, scalar multiplication, and magnitude calculation.

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use uom::si::Quantity;
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

impl<D, U> Vector2D<Quantity<D, U, f64>>
where
    D: uom::si::Dimension + ?Sized,
    U: uom::si::Units<f64> + ?Sized,
    Quantity<D, U, f64>: Copy,
{
    /// Returns the magnitude of the vector.
    pub fn mag(self) -> Quantity<D, U, f64> {
        let x_val = self.x.value;
        let y_val = self.y.value;
        let magnitude = (x_val * x_val + y_val * y_val).sqrt();
        Quantity {
            dimension: std::marker::PhantomData,
            units: std::marker::PhantomData,
            value: magnitude,
        }
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

/// Implements unary negation operator for `Vector2D`
/// This allows reversing the vector's direction
impl<Q> Neg for Vector2D<Q>
where
    Q: Neg<Output = Q> + Copy,
{
    type Output = Vector2D<Q>;

    fn neg(self) -> Vector2D<Q> {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
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
use uom::si::f64::{Acceleration, Force, Length, Mass, Ratio, Time, Velocity};
impl_vector_mul!(Acceleration);
impl_vector_mul!(Force);
impl_vector_mul!(Length);
impl_vector_mul!(Mass);
impl_vector_mul!(Ratio);
impl_vector_mul!(Time);
impl_vector_mul!(Velocity);
