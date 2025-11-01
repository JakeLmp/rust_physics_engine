use macroquad::prelude::draw_circle;
use once_cell::sync::Lazy;
use uom::si::{
    f32::{Acceleration, Length, Mass, Ratio, Time, Velocity},
    ratio::ratio,
    time::millisecond,
};

use crate::physics::vector::Vector2D;

pub static TIME_STEP: Lazy<Time> = Lazy::new(|| Time::new::<millisecond>(100.));

#[derive(Debug, Clone)]
pub struct Point {
    pub pos: Vector2D<Length>,
    pub vel: Vector2D<Velocity>,
    pub acc: Vector2D<Acceleration>,
    pub mass: Mass,
    #[allow(dead_code)]
    last_pos: Vector2D<Length>,
    #[allow(dead_code)]
    last_vel: Vector2D<Velocity>,
}

/// Initialisation
impl Point {
    /// Initialise a new Point
    pub fn new(
        pos: Vector2D<Length>,
        vel: Vector2D<Velocity>,
        acc: Vector2D<Acceleration>,
        mass: Mass,
    ) -> Self {
        Self {
            pos,
            vel,
            acc,
            mass,
            last_pos: Self::init_last_pos(pos, vel, acc),
            last_vel: Vector2D::<Velocity>::zero(),
        }
    }

    // approximation for initial R_(k-1)
    fn init_last_pos(
        first_pos: Vector2D<Length>,
        first_vel: Vector2D<Velocity>,
        first_acc: Vector2D<Acceleration>,
    ) -> Vector2D<Length> {
        first_pos - *TIME_STEP * (first_vel + (*TIME_STEP * first_acc) / 2.)
    }
}

#[allow(dead_code)]
pub enum StepType {
    ///
    Naive,
    ///
    Verlet,
}

/// Update implementations
#[allow(dead_code)]
impl Point {
    /// Update position parameters using different methods
    pub fn step(&mut self, step_type: Option<StepType>) {
        let step_type = step_type.unwrap_or(StepType::Naive);

        match step_type {
            StepType::Naive => {
                self.naive_step();
            }
            StepType::Verlet => {
                self.verlet_step();
            }
        }
    }

    /// Naive update method
    fn naive_step(&mut self) {
        self.pos += *TIME_STEP * self.vel;
        self.vel += *TIME_STEP * self.acc;
    }

    /// Verlet update method
    fn verlet_step(&mut self) {
        // save before updating
        let prev_pos = self.last_pos;
        let current_pos = self.pos;

        // update pos
        self.pos = Ratio::new::<ratio>(2.) * self.pos - self.last_pos
            + *TIME_STEP * (*TIME_STEP * self.acc);

        // update vel
        self.vel = (self.pos - prev_pos) / (Ratio::new::<ratio>(2.) * *TIME_STEP);

        self.last_pos = current_pos;
    }
}

/// Drawing implementations
impl Point {
    pub fn draw_circle(&self, mass_multiplier: f32, color: macroquad::prelude::Color) {
        draw_circle(
            self.pos.x.value,
            self.pos.y.value,
            self.mass.value * mass_multiplier,
            color,
        );
    }
}
