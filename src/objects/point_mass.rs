use macroquad::color::Color;
use std::fmt;
use uom::si::{
    f64::{Acceleration, Length, Mass, Ratio, Time, Velocity},
    ratio::ratio,
};

use crate::{
    objects::physical_object::PhysicalObject,
    physics::{
        potential::Potential,
        time_integration::{NaiveStep, StepType, VelocityVerletStep, VerletStep},
        vector::Vector2D,
    },
    simulation::{config::SimulationConfig, screen::Screen},
};

#[derive(Debug, Clone)]
pub struct PointMass {
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
impl PointMass {
    /// Initialise a new `PointMass`
    #[must_use]
    pub fn new(
        pos: Vector2D<Length>,
        vel: Vector2D<Velocity>,
        acc: Vector2D<Acceleration>,
        mass: Mass,
        time_step: Time,
    ) -> Self {
        Self {
            pos,
            vel,
            acc,
            mass,
            last_pos: Self::init_last_pos(pos, vel, acc, time_step),
            last_vel: Vector2D::<Velocity>::zero(),
        }
    }

    // approximation for initial R_(-1) ~ R_0 - tau * (V_0 + (tau * G_0) / 2)
    fn init_last_pos(
        first_pos: Vector2D<Length>,
        first_vel: Vector2D<Velocity>,
        first_acc: Vector2D<Acceleration>,
        time_step: Time,
    ) -> Vector2D<Length> {
        first_pos - time_step * (first_vel + (time_step * first_acc) / 2.)
    }
}

#[allow(dead_code)]
impl PhysicalObject for PointMass {
    /// Reset force
    fn reset_forces(&mut self) {
        self.acc = Vector2D::<Acceleration>::zero();
    }

    /// Apply potential resulting from other object
    fn apply_force(&mut self, potential: &dyn Potential, other: &dyn PhysicalObject) {
        self.acc += potential.force(self, other) / self.mass;
    }

    /// Update position parameters using different methods
    fn step(&mut self, step_type: Option<&StepType>, time_step: Time) {
        let step_type = step_type.unwrap_or(&StepType::Naive);

        match step_type {
            StepType::Naive => {
                self.naive_step(time_step);
            }
            StepType::Verlet => {
                self.verlet_step(time_step);
            }
            StepType::VelocityVerlet => {
                self.velocity_verlet_step(time_step);
            }
        }
    }

    fn pos(&self) -> Vector2D<Length> {
        self.pos
    }
    fn vel(&self) -> Vector2D<Velocity> {
        self.vel
    }
    fn acc(&self) -> Vector2D<Acceleration> {
        self.acc
    }
    fn mass(&self) -> Mass {
        self.mass
    }

    /// Draws a circle to the Screen
    #[allow(clippy::cast_possible_truncation)]
    fn draw(&self, config: &SimulationConfig, scale: Option<f32>, color: Color) {
        Screen::draw_point(
            self,
            config,
            Some(scale.unwrap_or(15.0 / config.mass_unit.get(self.mass()) as f32)),
            color,
        );
    }
}

impl NaiveStep for PointMass {
    /// Naive update method
    fn naive_step(&mut self, time_step: Time) {
        self.pos += time_step * self.vel;
        self.vel += time_step * self.acc;
    }
}

impl VerletStep for PointMass {
    /// Verlet update method
    fn verlet_step(&mut self, time_step: Time) {
        // save before updating
        let prev_pos = self.last_pos;
        let current_pos = self.pos;

        // update pos
        self.pos =
            Ratio::new::<ratio>(2.) * self.pos - self.last_pos + time_step * (time_step * self.acc);

        // update vel
        self.vel = (self.pos - prev_pos) / (Ratio::new::<ratio>(2.) * time_step);

        self.last_pos = current_pos;
    }
}

impl VelocityVerletStep for PointMass {
    /// Velocity Verlet update method variant
    fn velocity_verlet_step(&mut self, time_step: Time) {
        // save before updating
        let current_pos = self.pos;
        let current_vel = self.vel;

        // update pos
        self.pos +=
            time_step * self.vel + (time_step / Ratio::new::<ratio>(2.0)) * (time_step * self.acc);

        // update vel
        self.vel += (time_step / Ratio::new::<ratio>(2.0)) * (Ratio::new::<ratio>(2.0) * self.acc); // this last term should actually be (G_(k+1) - G_k)

        self.last_pos = current_pos;
        self.last_vel = current_vel;
    }
}

impl fmt::Display for PointMass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PointMass {{ pos: {:?}, vel: {:?}, acc: {:?}, mass: {:?} }}",
            self.pos, self.vel, self.acc, self.mass
        )
    }
}
