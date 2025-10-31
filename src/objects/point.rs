use crate::physics::vector::Vector2D;
use macroquad::prelude::draw_circle;
use uom::si::f32::Mass;

#[derive(Debug)]
pub struct Point {
    pub pos: Vector2D,
    pub vel: Vector2D,
    pub acc: Vector2D,
    pub mass: Mass,
}

pub enum StepType {
    ///
    Naive,
    ///
    Verlet,
}

impl Point {
    pub fn draw_circle(&self, mass_multiplier: f32, color: macroquad::prelude::Color) {
        draw_circle(
            self.pos.x.value,
            self.pos.y.value,
            self.mass.value * mass_multiplier,
            color,
        );
    }

    /// Update position parameters using different methods
    pub fn step(&mut self, step_type: Option<StepType>) {
        let step_type = step_type.unwrap_or(StepType::Naive);

        match step_type {
            StepType::Naive => {
                self.naive_step();
            }
            StepType::Verlet => {
                todo!();
            }
        }
    }

    /// Naive update method
    fn naive_step(&mut self) {
        self.pos += self.vel;
        self.vel += self.acc;
    }
}
