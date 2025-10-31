use crate::physics::vector::Vector2D;
use uom::si::f32::Mass;

#[derive(Debug)]
pub struct Point {
    pub pos: Vector2D,
    pub vel: Vector2D,
    pub acc: Vector2D,
    pub mass: Mass,
}

impl Point {
    pub fn step(&self) {
        todo!("Implement Verlet or similar");
    }
}
