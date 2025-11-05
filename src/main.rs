mod objects;
mod physics;
mod simulation;

use uom::si::{
    acceleration::meter_per_second_squared,
    energy::electronvolt,
    f32::{Acceleration, Energy, Length, Mass, Time, Velocity},
    length::angstrom,
    mass::{dalton, kilogram},
    time::{day, femtosecond},
    velocity::{atomic_unit_of_velocity, kilometer_per_second, meter_per_second},
};

use macroquad::prelude::*;

use objects::point::{Point, StepType};
use physics::{
    potential::{Gravity, LennardJones, Potential},
    vector::Vector2D,
};
use simulation::config::*;
use simulation::screen::Screen;
use simulation::units::*;

#[macroquad::main("Physics")]
async fn main() {
    // Simulation config for Earth-Moon system
    let config = SimulationConfig {
        time_step: Time::new::<uom::si::time::second>(1000.0), // 1000 seconds per step
        length_unit: LengthUnit::Meter,
        mass_unit: MassUnit::Kilogram,
        pixels_per_length: 400.0 / 3.844e8, // scale to fit on screen
    };

    // Earth-Moon system parameters
    let earth_mass = Mass::new::<uom::si::mass::kilogram>(5.972e24);
    let moon_mass = Mass::new::<uom::si::mass::kilogram>(7.342e22);
    let earth_moon_distance = Length::new::<uom::si::length::meter>(3.844e8);
    let moon_orbital_velocity = Velocity::new::<uom::si::velocity::meter_per_second>(1022.0);

    let mut points: Vec<Point> = vec![
        // Earth at origin
        Point::new(
            Vector2D {
                x: Length::new::<uom::si::length::meter>(0.0),
                y: Length::new::<uom::si::length::meter>(0.0),
            },
            Vector2D {
                x: Velocity::new::<uom::si::velocity::meter_per_second>(0.0),
                y: Velocity::new::<uom::si::velocity::meter_per_second>(0.0),
            },
            Vector2D {
                x: Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(0.0),
                y: Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(0.0),
            },
            earth_mass,
            config.time_step,
        ),
        // Moon
        Point::new(
            Vector2D {
                x: earth_moon_distance,
                y: Length::new::<uom::si::length::meter>(0.0),
            },
            Vector2D {
                x: Velocity::new::<uom::si::velocity::meter_per_second>(0.0),
                y: moon_orbital_velocity,
            },
            Vector2D {
                x: Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(0.0),
                y: Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(0.0),
            },
            moon_mass,
            config.time_step,
        ),
    ];

    // Newtonian gravity potential
    let potential = Gravity::default();

    loop {
        clear_background(BLACK);

        for i in 0..points.len() {
            let (left, right) = points.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();
            let others: Vec<&Point> = left.iter().chain(right.iter()).collect();

            let color = if i == 0 { BLUE } else { WHITE };

            Screen::draw_point(
                current,
                &config,
                Some(15.0 / current.mass.get::<kilogram>()),
                color,
            );
            current.apply_potential(&potential, &others);
            current.step(Some(StepType::Verlet), config.time_step);
        }
        println!("{}", points[1]);
        // println!();

        next_frame().await;
    }
}
