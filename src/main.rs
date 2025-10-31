mod objects;
mod physics;

use uom::si::f32::{Length, Mass, Ratio};
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::ratio::ratio;

use macroquad::prelude::*;

use objects::point::Point;
use physics::vector::Vector2D;

#[macroquad::main("Physics")]
async fn main() {
    let mut p = Point {
        pos: Vector2D {
            x: Length::new::<meter>(100.0),
            y: Length::new::<meter>(100.0),
        },
        vel: Vector2D {
            x: Length::new::<meter>(1.0),
            y: Length::new::<meter>(0.0),
        },
        acc: Vector2D {
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.01),
        },
        mass: Mass::new::<kilogram>(1.0),
    };

    let mass_multiplier: f32 = 20.;

    loop {
        clear_background(BLACK);

        p.draw_circle(mass_multiplier, WHITE);
        p.step(None);

        next_frame().await;
    }
}
