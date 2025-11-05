use physics_engine::objects::point::{Point, StepType};
use physics_engine::physics::potential::LennardJones;
use physics_engine::physics::vector::Vector2D;

use physics_engine::simulation::config::SimulationConfig;
use physics_engine::simulation::screen::Screen;
use physics_engine::simulation::units::{LengthUnit, MassUnit};
use uom::si::f32::Time;
use uom::si::mass::kilogram;
use uom::si::{
    acceleration::meter_per_second_squared,
    energy::electronvolt,
    f32::{Acceleration, Energy, Length, Mass, Velocity},
    length::angstrom,
    mass::dalton,
    time::femtosecond,
    velocity::atomic_unit_of_velocity,
};

use macroquad::prelude::*;

#[macroquad::main("Argon Gas Simulation")]
async fn main() {
    // Simulation config for Argon
    let config = SimulationConfig {
        time_step: Time::new::<femtosecond>(100.0),
        length_unit: LengthUnit::Angstrom,
        mass_unit: MassUnit::Dalton,
        pixels_per_length: 4.0, // 4 pixels per angstrom
    };

    // Initialize argon atoms
    let max_bound = 100.0;
    let mut points: Vec<Point> = Vec::new();
    for _i in 0..100 {
        points.push(Point::new(
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
        ));
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
            let others: Vec<&Point> = left.iter().chain(right.iter()).collect();

            Screen::draw_point(
                current,
                &config,
                Some(15.0 / current.mass.get::<dalton>()),
                color,
            );
            current.apply_potential(&potential, &others);
            current.step(Some(StepType::Verlet), config.time_step);
        }

        next_frame().await;
    }
}
