use molecular_dynamics::{
    handler::SimulationHandler,
    physics::{potential::LennardJones, time_integration::StepType},
    point_mass::PointMass,
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

#[macroquad::main("Argon Gas Large - SimulationHandler")]
async fn main() {
    // Simulation config for Argon
    let config = SimulationConfigBuilder::default()
        .time_step(Time::new::<femtosecond>(1.0))
        .length_unit(LengthUnit::Angstrom)
        .mass_unit(MassUnit::Dalton)
        .pixels_per_length(0.4)
        .time_steps_per_frame(Some(10))
        .build()
        .unwrap();

    // Initialize argon atoms
    let max_bound = 1000.0;
    let mut points: Vec<Box<PointMass>> = Vec::new();
    for _i in 0..1000 {
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

    // Create simulation handler
    let mut handler = SimulationHandler::new(points);

    // Lennard-Jones potential for Argon
    let potential = LennardJones {
        epsilon: Energy::new::<electronvolt>(0.0104),
        sigma: Length::new::<angstrom>(3.4),
    };

    let color = WHITE;

    loop {
        // Run multiple physics steps
        handler.step_physics(
            &config,
            &potential,
            config.time_step,
            StepType::VelocityVerlet,
        );

        clear_background(BLACK);

        // Sync back to objects for rendering
        handler.sync_to_points();

        // Draw all objects
        for obj in &handler.points {
            obj.draw(&config, Some(1.), color);
        }

        next_frame().await;
    }
}
