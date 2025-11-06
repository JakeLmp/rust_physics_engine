use macroquad::prelude::rand;
use uom::si::{
    acceleration::meter_per_second_squared,
    f32::{Acceleration, Length, Mass, Velocity},
    length::angstrom,
    mass::dalton,
    velocity::atomic_unit_of_velocity,
};

use crate::{
    objects::point_mass::{PhysicalObject, PointMass},
    physics::vector::Vector2D,
    simulation::config::SimulationConfig,
};

/// A cluster of objects, e.g. PointMasses
pub struct Cluster {
    /// The objects in question
    /// objects must implement the PhysicalObject trait
    pub objects: Vec<Box<dyn PhysicalObject>>,
}

pub struct RectangularBounds {
    x1: Length,
    x2: Length,
    y1: Length,
    y2: Length,
}

impl Cluster {
    fn new(config: &SimulationConfig, position_bounds: RectangularBounds) -> Self {
        // Initialize argon atoms
        let mut objects: Vec<Box<dyn PhysicalObject>> = Vec::new();
        for _i in 0..100 {
            objects.push(Box::new(PointMass::new(
                Vector2D {
                    // use config here to get the right units
                    x: Length::new::<angstrom>(rand::gen_range(
                        -config.length_unit.get(position_bounds.x1),
                        config.length_unit.get(position_bounds.x2),
                    )),
                    // also here
                    y: Length::new::<angstrom>(rand::gen_range(
                        -config.length_unit.get(position_bounds.y1),
                        config.length_unit.get(position_bounds.y2),
                    )),
                },
                Vector2D {
                    x: Velocity::new::<atomic_unit_of_velocity>(0.0), // and here
                    y: Velocity::new::<atomic_unit_of_velocity>(0.0), // and here
                },
                Vector2D {
                    x: Acceleration::new::<meter_per_second_squared>(0.0), // and here too
                    y: Acceleration::new::<meter_per_second_squared>(0.0), // also here
                },
                Mass::new::<dalton>(39.948), // don't forget here
                config.time_step,
            )));
        }

        Self { objects: objects }
    }

    fn center_of_mass(&self) -> Vector2D<Length> {
        // and lastly here
        let mut com = Vector2D::<Length>::zero() * Mass::new::<dalton>(0.0);
        let mut total_mass = Mass::new::<dalton>(0.0);
        for object in &self.objects {
            com += object.pos() * object.mass();
        }
        com / total_mass
    }
}
