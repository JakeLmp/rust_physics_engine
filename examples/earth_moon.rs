use physics_engine::objects::point_mass::{PointMass, StepType};
use physics_engine::physics::potential::{Gravity, Potential};
use physics_engine::physics::vector::Vector2D;

use physics_engine::simulation::config::SimulationConfig;
use physics_engine::simulation::screen::Screen;
use physics_engine::simulation::units::{LengthUnit, MassUnit};
use uom::si::f32::Time;
use uom::si::{
    acceleration::meter_per_second_squared,
    f32::{Acceleration, Length, Mass, Velocity},
    length::meter,
    mass::kilogram,
    time::second,
    velocity::meter_per_second,
};

use macroquad::prelude::*;

#[macroquad::main("Earth-Moon System")]
async fn main() {
    // Simulation config for Earth-Moon system
    let config = SimulationConfig {
        time_step: Time::new::<second>(1000.0),
        length_unit: LengthUnit::Meter,
        mass_unit: MassUnit::Kilogram,
        pixels_per_length: 400.0 / 3.844e8, // scale to fit on screen
    };

    // Earth-Moon system parameters
    let earth_mass = Mass::new::<kilogram>(5.972e24); // Earth mass
    let moon_mass = Mass::new::<kilogram>(7.342e22); // Moon mass
    let earth_moon_distance = Length::new::<meter>(3.844e8); // Average distance

    // Moon's orbital velocity around Earth (approximately 1022 m/s)
    let moon_orbital_velocity = Velocity::new::<meter_per_second>(1022.0);

    let mut points: Vec<PointMass> = vec![
        // Earth at origin (we'll use a reference frame centered on Earth)
        PointMass::new(
            Vector2D {
                x: Length::new::<meter>(0.0),
                y: Length::new::<meter>(0.0),
            },
            Vector2D {
                x: Velocity::new::<meter_per_second>(0.0),
                y: Velocity::new::<meter_per_second>(0.0),
            },
            Vector2D {
                x: Acceleration::new::<meter_per_second_squared>(0.0),
                y: Acceleration::new::<meter_per_second_squared>(0.0),
            },
            earth_mass,
            config.time_step,
        ),
        // Moon
        PointMass::new(
            Vector2D {
                x: earth_moon_distance,
                y: Length::new::<meter>(0.0),
            },
            Vector2D {
                x: Velocity::new::<meter_per_second>(0.0),
                y: moon_orbital_velocity,
            },
            Vector2D {
                x: Acceleration::new::<meter_per_second_squared>(0.0),
                y: Acceleration::new::<meter_per_second_squared>(0.0),
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
            let others: Vec<&PointMass> = left.iter().chain(right.iter()).collect();

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

        next_frame().await;
    }
}
