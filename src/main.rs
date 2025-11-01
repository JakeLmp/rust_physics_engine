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
    let mut p = Point::new(
        Vector2D {
            x: Length::new::<meter>(100.0),
            y: Length::new::<meter>(100.0),
        },
        Vector2D {
            x: Velocity::new::<meter_per_second>(1.0),
            y: Velocity::new::<meter_per_second>(0.0),
        },
        Vector2D {
            x: Acceleration::new::<meter_per_second_squared>(0.0),
            y: Acceleration::new::<meter_per_second_squared>(0.01),
        },
        Mass::new::<kilogram>(1.0),
    );

    let mass_multiplier: f32 = 20.;

    loop {
        clear_background(BLACK);

        p.draw_circle(mass_multiplier, WHITE);
        p.step(Some(StepType::Naive));

        next_frame().await;
    }
}
