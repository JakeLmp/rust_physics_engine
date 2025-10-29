mod objects;

use objects::vector::Vector2D;

fn main() {
    let v = Vector2D { x: 10.0, y: 10.0 };

    println!("Vector: {:?}", v);
    println!("Magnitude: {:?}", v.mag());
}
