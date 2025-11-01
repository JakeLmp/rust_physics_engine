mod objects;
mod physics;

use uom::si::{
    acceleration::meter_per_second_squared,
    energy::{joule, kilojoule},
    f32::{Acceleration, Energy, Length, Mass, Velocity},
    length::{angstrom, meter},
    mass::kilogram,
    velocity::{atomic_unit_of_velocity, meter_per_second},
};

use macroquad::prelude::*;

use objects::point::{Point, StepType};
use physics::vector::Vector2D;

use crate::physics::potential::LennardJones;

#[macroquad::main("Physics")]
async fn main() {
    let mut points: Vec<Point> = Vec::new();
    for _i in 0..2 {
        points.push(Point::new(
            Vector2D {
                x: Length::new::<angstrom>(rand::gen_range(0.0, 100.0)),
                y: Length::new::<angstrom>(rand::gen_range(0.0, 100.0)),
            },
            Vector2D {
                x: Velocity::new::<atomic_unit_of_velocity>(0.0),
                y: Velocity::new::<atomic_unit_of_velocity>(0.0),
            },
            Vector2D {
                x: Acceleration::new::<meter_per_second_squared>(rand::gen_range(-1.0, 1.0)),
                y: Acceleration::new::<meter_per_second_squared>(rand::gen_range(-1.0, 1.0)),
            },
            Mass::new::<kilogram>(1.0),
        ));
    }

    println!("{:?}", points[0]);

    let potential: Box<dyn physics::potential::Potential> = Box::new(LennardJones {
        epsilon: Energy::new::<kilojoule>(1000.0),
        sigma: Length::new::<meter>(1.0),
    });

    let scale_multiplier: f32 = 10e10;
    let mass_multiplier: f32 = 10e10;
    let color = WHITE;

    loop {
        clear_background(BLACK);

        for i in 0..points.len() {
            let (left, right) = points.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();
            let others: Vec<&Point> = left.iter().chain(right.iter()).collect();

            current.draw_circle(Some(scale_multiplier), Some(mass_multiplier), color);
            current.apply_potential(&potential, &others);
            current.step(Some(StepType::Verlet));
        }

        next_frame().await;
    }
}
