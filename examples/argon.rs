use molecular_dynamics::{
    objects::{physical_object::PhysicalObject, point_mass::PointMass},
    physics::{potential::LennardJones, time_integration::StepType},
};
use physics_core::vector::Vector2D;
use visualization::simulation::{
    config::SimulationConfigBuilder,
    units::{LengthUnit, MassUnit},
};

use uom::si::{
    acceleration::meter_per_second_squared,
    energy::electronvolt,
    f64::{Acceleration, Energy, Length, Mass, Time, Velocity},
    length::angstrom,
    mass::dalton,
    time::femtosecond,
    velocity::atomic_unit_of_velocity,
};

use macroquad::prelude::*;

#[macroquad::main("Argon Gas Simulation")]
async fn main() {
    // Simulation config for Argon
    let config = SimulationConfigBuilder::default()
        .time_step(Time::new::<femtosecond>(1.0))
        .length_unit(LengthUnit::Angstrom)
        .mass_unit(MassUnit::Dalton)
        .pixels_per_length(4.0)
        .force_softening_epsilon(Some(LengthUnit::Angstrom.new(1e-3)))
        .build()
        .unwrap();

    // Initialize argon atoms
    let max_bound = 100.0;
    let mut points: Vec<Box<dyn PhysicalObject>> = Vec::new();
    for _i in 0..100 {
        points.push(Box::new(PointMass::new(
            Vector2D {
                x: Length::new::<angstrom>(rand::gen_range(-max_bound, max_bound)),
                y: Length::new::<angstrom>(rand::gen_range(-max_bound, max_bound)),
            },
            Vector2D {
                x: Velocity::new::<atomic_unit_of_velocity>(0.0),
                y: Velocity::new::<atomic_unit_of_velocity>(0.0),
            },
            Vector2D {
                x: Acceleration::new::<meter_per_second_squared>(0.0),
                y: Acceleration::new::<meter_per_second_squared>(0.0),
            },
            Mass::new::<dalton>(39.948),
            config.time_step,
        )));
    }

    // Lennard-Jones potential for Argon
    let potential = LennardJones {
        epsilon: Energy::new::<electronvolt>(0.0104),
        sigma: Length::new::<angstrom>(3.4),
    };

    let color = WHITE;

    loop {
        clear_background(BLACK);

        for i in 0..points.len() {
            let (left, right) = points.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();

            current.reset_forces();

            // Apply forces from objects before and after current
            for other in left.iter().chain(right.iter()) {
                current.apply_force(&potential, other.as_ref(), &config);
            }
        }

        for point in &mut points {
            point.step(Some(&StepType::Verlet), config.time_step);
            point.draw(&config, Some(10.), color);
        }

        next_frame().await;
    }
}
