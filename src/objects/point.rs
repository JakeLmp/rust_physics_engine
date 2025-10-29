use crate::physics::vector::Vector2D;

#[derive(Debug)]
pub struct Point {
    pub pos: Vector2D,
    pub vel: Vector2D,
    pub acc: Vector2D,
}

impl Point {
    pub fn step(&self) {
        todo!("Implement Verlet or similar");
    }
}
