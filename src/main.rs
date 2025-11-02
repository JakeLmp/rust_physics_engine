mod objects;
mod physics;

use uom::si::{
    acceleration::meter_per_second_squared,
    energy::electronvolt,
    f32::{Acceleration, Energy, Length, Mass, Velocity},
    length::angstrom,
    mass::dalton,
    velocity::atomic_unit_of_velocity,
};

use macroquad::prelude::*;

use objects::point::{Point, StepType};
use physics::{potential::LennardJones, vector::Vector2D};

/// Calculate multiplier so pos.mag.value * scale ~ 1.
fn get_position_scale_multiplier(points: &[Point]) -> f32 {
    let max = points
        .iter()
        .map(|point| point.pos.mag().value)
        .fold(0.0_f32, f32::max);
    // let mag = pos.mag().value;
    if max > 0.0 {
        1. / (10_f32).powf(max.log10().floor())
    } else {
        1.
    }
}

/// Calculate multiplier so mass.value * scale ~ 1.
fn get_mass_scale_multiplier(mass: Mass) -> f32 {
    if mass.value > 0. {
        1. / (10_f32).powf(mass.value.log10().floor())
    } else {
        1.
    }
}

#[macroquad::main("Physics")]
async fn main() {
    // units for Argon gas
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
            // Mass::new::<dalton>(39.948),
            Mass::new::<dalton>(5.),
        ));
    }

    // potentials for Argon gas
    let potential = LennardJones {
        epsilon: Energy::new::<electronvolt>(0.0104),
        sigma: Length::new::<angstrom>(3.4),
    };

    let pos_multiplier: f32 = 200. * get_position_scale_multiplier(&points);
    let mass_multiplier: f32 = 2. * get_mass_scale_multiplier(points[0].mass);
    let color = WHITE;

    loop {
        clear_background(BLACK);

        for i in 0..points.len() {
            let (left, right) = points.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();
            let others: Vec<&Point> = left.iter().chain(right.iter()).collect();

            current.draw_circle(Some(pos_multiplier), Some(mass_multiplier), color);
            current.apply_potential(&potential, &others);
            current.step(Some(StepType::Verlet));
        }

        // println!("{:?}", points[0].pos);

        next_frame().await;
    }
}
