use macroquad::prelude::rand;
use uom::si::{
    f64::{Acceleration, Length, Mass, Velocity},
    mass::kilogram,
};

use crate::point_mass::PointMass;
use physics_core::vector::Vector2D;
use visualization::simulation::config::SimulationConfig;

/// A cluster of `PointMass`es
pub struct Cluster {
    /// The points in question
    pub points: Vec<Box<PointMass>>,
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
        no_of_points: u32,
        mass_of_points: Mass,
    ) -> Self {
        let mut points: Vec<Box<PointMass>> = Vec::new();
        for _i in 0..no_of_points {
            points.push(Box::new(PointMass::new(
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
                mass_of_points,
                config.time_step,
            )));
        }

        Self { points }
    }

    #[must_use]
    pub fn center_of_mass(&self) -> Vector2D<Length> {
        // Single pass-over to calculate weighted sum and total mass
        let (weighted_sum, total_mass) = self.points.iter().fold(
            (
                Vector2D::<Length>::zero() * Mass::new::<kilogram>(0.0),
                Mass::new::<kilogram>(0.0),
            ),
            |(com, total_mass), obj| (com + obj.pos() * obj.mass(), total_mass + obj.mass()),
        );
        weighted_sum / total_mass
    }
}
