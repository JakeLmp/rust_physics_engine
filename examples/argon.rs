use physics_engine::objects::point::{Point, StepType};
use physics_engine::physics::potential::LennardJones;
use physics_engine::physics::vector::Vector2D;

use uom::si::{
    acceleration::meter_per_second_squared,
    energy::electronvolt,
    f32::{Acceleration, Energy, Length, Mass, Velocity},
    length::angstrom,
    mass::dalton,
    velocity::atomic_unit_of_velocity,
};

use macroquad::prelude::*;

#[macroquad::main("Argon Gas Simulation")]
async fn main() {
    // Initialize argon atoms
    let mut points: Vec<Point> = Vec::new();
    for _i in 0..10 {
        points.push(Point::new(
            Vector2D {
                x: Length::new::<angstrom>(rand::gen_range(50.0, 100.0)),
                y: Length::new::<angstrom>(rand::gen_range(50.0, 100.0)),
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
        ));
    }

    // Lennard-Jones potential for Argon
    let potential = LennardJones {
        epsilon: Energy::new::<electronvolt>(0.0104),
        sigma: Length::new::<angstrom>(3.4),
    };

    loop {
        clear_background(BLACK);

        for i in 0..points.len() {
            let (left, right) = points.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();
            let others: Vec<&Point> = left.iter().chain(right.iter()).collect();

            current.draw_circle(None, None, WHITE);
            current.apply_potential(&potential, &others);
            current.step(Some(StepType::Verlet));
        }

        next_frame().await;
    }
}
