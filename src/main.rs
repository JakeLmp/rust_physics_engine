use molecular_dynamics::{
    handler::SimulationHandler,
    physics::{potential::LennardJones, time_integration::StepType},
    point_mass::PointMass,
};
use physics_core::vector::Vector2D;
use uom::si::{
    acceleration::meter_per_second_squared,
    energy::electronvolt,
    f64::{Acceleration, Energy, Length, Mass, Time, Velocity},
    length::angstrom,
    mass::dalton,
    time::femtosecond,
    velocity::atomic_unit_of_velocity,
};
use visualization::simulation::{
    config::{BoundaryKind, SimulationConfigBuilder},
    screen::{Screen, ScreenPosition},
    units::{LengthUnit, MassUnit},
};

use macroquad::prelude::*;

#[macroquad::main("Argon Gas Large - SimulationHandler")]
async fn main() {
    // Simulation config for Argon
    let max_bound = 100.0;
    let config = SimulationConfigBuilder::default()
        .time_step(Time::new::<femtosecond>(0.5))
        .length_unit(LengthUnit::Angstrom)
        .mass_unit(MassUnit::Dalton)
        .pixels_per_length(5.)
        .time_steps_per_frame(100)
        .boundary_type(BoundaryKind::Elastic(Vector2D {
            x: Length::new::<angstrom>(max_bound),
            y: Length::new::<angstrom>(max_bound),
        }))
        .pair_cutoff_radius(Some(Length::new::<angstrom>(max_bound)))
        .display_stats(true)
        .display_boundary(true)
        .build()
        .unwrap();

    let max_vel = 0.0;

    // Initialize argon atoms
    let mut points: Vec<PointMass> = Vec::new();
    for _i in 0..80 {
        points.push(PointMass::new(
            Vector2D {
                x: Length::new::<angstrom>(rand::gen_range(-max_bound, max_bound)),
                y: Length::new::<angstrom>(rand::gen_range(-max_bound, max_bound)),
            },
            Vector2D {
                x: Velocity::new::<atomic_unit_of_velocity>(rand::gen_range(-max_vel, max_vel)),
                y: Velocity::new::<atomic_unit_of_velocity>(rand::gen_range(-max_vel, max_vel)),
            },
            Vector2D {
                x: Acceleration::new::<meter_per_second_squared>(0.0),
                y: Acceleration::new::<meter_per_second_squared>(0.0),
            },
            Mass::new::<dalton>(39.948),
            config.time_step,
        ));
    }

    // Create simulation handler
    let mut handler = SimulationHandler::new(points);

    // Lennard-Jones potential for Argon
    let potential = LennardJones {
        epsilon: Energy::new::<electronvolt>(0.0104),
        sigma: Length::new::<angstrom>(3.4),
    };

    let color = WHITE;
    let mut elapsed_time = Time::new::<femtosecond>(0.0);

    loop {
        // Run multiple physics steps
        handler.step_physics(
            &config,
            &potential,
            config.time_step,
            StepType::VelocityVerlet,
        );

        elapsed_time += config.time_step * config.time_steps_per_frame as f64;

        clear_background(BLACK);

        // Sync back to objects for rendering
        handler.sync_to_points();

        // Draw all objects
        for obj in &handler.points {
            obj.draw(&config, Some(10.), color);
        }

        // Display stats
        if config.display_stats {
            let time_fs = elapsed_time.get::<femtosecond>() as f32;
            let particles = handler.points.len() as f32;
            Screen::display_stats(
                &[("Time (fs)", &time_fs), ("Particles", &particles)],
                ScreenPosition::TopLeft,
                None,
                None,
                None,
                None,
            );
        }

        // Draw boundaries
        Screen::draw_boundary(&config);

        next_frame().await;
    }
}
