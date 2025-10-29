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
    println!();

    // Create some vectors
    let mut v1 = Vector2D { x: 3.0, y: 4.0 };
    let v2 = Vector2D { x: 1.0, y: 2.0 };
    let scalar = 2.5;

    println!("=== Vector2D Showcase ===");
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("scalar: {}", scalar);
    println!();

    // Magnitude calculation
    println!("=== Magnitude ===");
    println!("v1.mag(): {}", v1.mag());
    println!("v2.mag(): {}", v2.mag());
    println!();

    // Addition
    println!("=== Addition ===");
    let v3 = v1 + v2;
    println!("v1 + v2 = {:?}", v3);

    // AddAssign
    println!("=== AddAssign ===");
    let mut v4 = v1;
    println!("Before v4 += v2: {:?}", v4);
    v4 += v2;
    println!("After v4 += v2: {:?}", v4);
    println!();

    // Subtraction
    println!("=== Subtraction ===");
    let v5 = v1 - v2;
    println!("v1 - v2 = {:?}", v5);

    // SubAssign
    println!("=== SubAssign ===");
    let mut v6 = v1;
    println!("Before v6 -= v2: {:?}", v6);
    v6 -= v2;
    println!("After v6 -= v2: {:?}", v6);
    println!();

    // Multiplication by scalar
    println!("=== Multiplication ===");
    let v7 = v1 * scalar;
    println!("v1 * {} = {:?}", scalar, v7);

    // MulAssign
    println!("=== MulAssign ===");
    println!("Before v1 *= {}: {:?}", scalar, v1);
    v1 *= scalar;
    println!("After v1 *= {}: {:?}", scalar, v1);
    println!();

    // Chaining operations
    println!("=== Chained Operations ===");
    let result = (v2 + Vector2D { x: 1.0, y: 1.0 }) * 3.0;
    println!("(v2 + (1,1)) * 3.0 = {:?}", result);
    println!("Magnitude of result: {}", result.mag());
}
