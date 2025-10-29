mod objects;
mod physics;

use objects::point::Point;
use physics::vector::Vector2D;

fn main() {
    let v = Vector2D { x: 10.0, y: 10.0 };
    let p = Point {
        pos: Vector2D { x: 0., y: 0. },
        vel: Vector2D { x: 1., y: 0. },
        acc: Vector2D { x: 0., y: 0. },
    };

    println!("Vector: {:?}", v);
    println!("Magnitude: {:?}", v.mag());
    println!("Point: {:?}", p);
}
