use physics_engine::objects::point::{Point, StepType};
use physics_engine::physics::potential::{Gravity, Potential};
use physics_engine::physics::vector::Vector2D;

use uom::si::{
    acceleration::meter_per_second_squared,
    f32::{Acceleration, Length, Mass, Velocity},
    length::meter,
    mass::kilogram,
    velocity::meter_per_second,
};

use macroquad::prelude::*;

#[macroquad::main("Earth-Moon System")]
async fn main() {
    // Earth-Moon system parameters
    let earth_mass = Mass::new::<kilogram>(5.972e24); // Earth mass
    let moon_mass = Mass::new::<kilogram>(7.342e22); // Moon mass
    let earth_moon_distance = Length::new::<meter>(3.844e8); // Average distance

    // Moon's orbital velocity around Earth (approximately 1022 m/s)
    let moon_orbital_velocity = Velocity::new::<meter_per_second>(1022.0);

    let mut points: Vec<Point> = vec![
        // Earth at origin (we'll use a reference frame centered on Earth)
        Point::new(
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
        ),
        // Moon
        Point::new(
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
        ),
    ];

    // Newtonian gravity potential
    let potential = Gravity::default();

    // Scale for visualization
    let pos_multiplier = 400.0 / earth_moon_distance.value;
    // Scale masses to get reasonable visual sizes (Earth ~15 pixels, Moon ~5 pixels)
    let mass_multiplier = 15.0 / earth_mass.value;

    loop {
        clear_background(BLACK);

        for i in 0..points.len() {
            let (left, right) = points.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();
            let others: Vec<&Point> = left.iter().chain(right.iter()).collect();

            let color = if i == 0 { BLUE } else { WHITE };
            current.draw_circle(Some(pos_multiplier), Some(mass_multiplier), color);

            current.apply_potential(&potential, &others);
            current.step(Some(StepType::Verlet));
        }

        next_frame().await;
    }
}
