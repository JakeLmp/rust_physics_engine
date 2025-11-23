use macroquad::prelude::rand;
use uom::si::{
    f64::{Acceleration, Length, Mass, Velocity},
    mass::kilogram,
};

use crate::{
    objects::{physical_object::PhysicalObject, point_mass::PointMass},
    physics::{potential::Potential, time_integration::StepType},
};
use physics_core::vector::Vector2D;
use visualization::simulation::config::SimulationConfig;

/// A cluster of objects, e.g. `PointMasses`
pub struct Cluster {
    /// The objects in question
    /// provided objects must implement the `PhysicalObject` trait
    pub objects: Vec<Box<dyn PhysicalObject>>,
}

#[derive(Debug)]
pub struct RectangularBounds {
    pub x1: Length,
    pub x2: Length,
    pub y1: Length,
    pub y2: Length,
}

impl Cluster {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(
        config: &SimulationConfig,
        position_bounds: &RectangularBounds,
        no_of_objects: u32,
        mass_of_objects: Mass,
    ) -> Self {
        let mut objects: Vec<Box<dyn PhysicalObject>> = Vec::new();
        for _i in 0..no_of_objects {
            objects.push(Box::new(PointMass::new(
                Vector2D {
                    x: config.length_unit.new(f64::from(rand::gen_range(
                        config.length_unit.get(position_bounds.x1) as f32,
                        config.length_unit.get(position_bounds.x2) as f32,
                    ))),
                    y: config.length_unit.new(f64::from(rand::gen_range(
                        config.length_unit.get(position_bounds.y1) as f32,
                        config.length_unit.get(position_bounds.y2) as f32,
                    ))),
                },
                // units don't matter here, as long as we're initialising with zero-vector
                Vector2D::<Velocity>::zero(),
                Vector2D::<Acceleration>::zero(),
                mass_of_objects,
                config.time_step,
            )));
        }

        Self { objects }
    }

    #[must_use]
    pub fn center_of_mass(&self) -> Vector2D<Length> {
        // Single pass-over to calculate weighted sum and total mass
        let (weighted_sum, total_mass) = self.objects.iter().fold(
            (
                Vector2D::<Length>::zero() * Mass::new::<kilogram>(0.0),
                Mass::new::<kilogram>(0.0),
            ),
            |(com, total_mass), obj| (com + obj.pos() * obj.mass(), total_mass + obj.mass()),
        );
        weighted_sum / total_mass
    }

    /// Steps the simulation forward by one time step.
    pub fn step(
        &mut self,
        config: &SimulationConfig,
        potential: &impl Potential,
        step_type: Option<&StepType>,
    ) {
        if self.objects.is_empty() {
            return;
        };

        for i in 0..self.objects.len() {
            let (left, right) = self.objects.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();

            current.reset_forces();

            // Apply forces from objects before and after current
            for other in left.iter().chain(right.iter()) {
                current.apply_force(potential, other.as_ref(), config);
            }
        }

        for object in &mut self.objects {
            object.step(step_type, config.time_step);
        }
    }
}
