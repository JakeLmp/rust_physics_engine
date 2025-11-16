use molecular_dynamics::{
    objects::{physical_object::PhysicalObject, point_mass::PointMass},
    physics::{
        potential::{Gravity, Potential},
        time_integration::StepType,
    },
};
use physics_core::vector::Vector2D;
use visualization::simulation::{
    config::SimulationConfig,
    units::{LengthUnit, MassUnit},
};

use uom::si::{
    acceleration::meter_per_second_squared,
    f64::{Acceleration, Length, Mass, Time, Velocity},
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
        display_stats: false,
    };

    // Earth-Moon system parameters
    let earth_mass = Mass::new::<kilogram>(5.972e24); // Earth mass
    let moon_mass = Mass::new::<kilogram>(7.342e22); // Moon mass
    let earth_moon_distance = Length::new::<meter>(3.844e8); // Average distance

    // Moon's orbital velocity around Earth (approximately 1022 m/s)
    let moon_orbital_velocity = Velocity::new::<meter_per_second>(1022.0);

    // Earth at origin
    let mut earth = PointMass::new(
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
    );

    // Moon
    let mut moon = PointMass::new(
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
    );

    // Newtonian gravity potential
    let potential = Gravity::default();

    // Update algorithm
    let step_type = StepType::Verlet;

    loop {
        clear_background(BLACK);

        earth.reset_forces();
        earth.apply_force(&potential, &moon);
        moon.reset_forces();
        moon.apply_force(&potential, &earth);

        earth.step(Some(&step_type), config.time_step);
        moon.step(Some(&step_type), config.time_step);

        earth.draw(&config, Some(20.), BLUE);
        moon.draw(&config, Some(10.), WHITE);

        next_frame().await;
    }
}
