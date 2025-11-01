mod objects;
mod physics;

use uom::si::{
    acceleration::meter_per_second_squared,
    f32::{Acceleration, Length, Mass, Velocity},
    length::meter,
    mass::kilogram,
    velocity::meter_per_second,
};

use macroquad::prelude::*;

use objects::point::{Point, StepType};
use physics::vector::Vector2D;

#[macroquad::main("Physics")]
async fn main() {
    let mut points: Vec<Point> = Vec::new();
    for _i in 0..10 {
        points.push(Point::new(
            Vector2D {
                x: Length::new::<meter>(rand::gen_range(0.0, 800.0)),
                y: Length::new::<meter>(rand::gen_range(0.0, 600.0)),
            },
            Vector2D {
                x: Velocity::new::<meter_per_second>(0.0),
                y: Velocity::new::<meter_per_second>(0.0),
            },
            Vector2D {
                x: Acceleration::new::<meter_per_second_squared>(rand::gen_range(-1.0, 1.0)),
                y: Acceleration::new::<meter_per_second_squared>(rand::gen_range(-1.0, 1.0)),
            },
            Mass::new::<kilogram>(1.0),
        ));
    }

    let mass_multiplier: f32 = 20.;
    let color = WHITE;

    loop {
        clear_background(BLACK);

        for p in points.iter_mut() {
            p.draw_circle(mass_multiplier, color);
            p.step(Some(StepType::VelocityVerlet));
        }

        next_frame().await;
    }
}
