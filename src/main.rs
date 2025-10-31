mod objects;
mod physics;

use uom::si::f32::{Length, Mass, Ratio};
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::ratio::ratio;

use objects::point::Point;
use physics::vector::Vector2D;

fn main() {
    let v = Vector2D {
        x: Length::new::<meter>(10.0),
        y: Length::new::<meter>(10.0),
    };
    let p = Point {
        pos: Vector2D {
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.0),
        },
        vel: Vector2D {
            x: Length::new::<meter>(1.0),
            y: Length::new::<meter>(0.0),
        },
        acc: Vector2D {
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.0),
        },
        mass: Mass::new::<kilogram>(1.0),
    };

    println!(
        "Vector: x = {} m, y = {} m",
        v.x.get::<meter>(),
        v.y.get::<meter>()
    );
    println!("Magnitude: {} m", v.mag().get::<meter>());
    println!(
        "Point: pos = ({}, {}), vel = ({}, {}), acc = ({}, {}), mass = {} kg",
        p.pos.x.get::<meter>(),
        p.pos.y.get::<meter>(),
        p.vel.x.get::<meter>(),
        p.vel.y.get::<meter>(),
        p.acc.x.get::<meter>(),
        p.acc.y.get::<meter>(),
        p.mass.get::<kilogram>()
    );
    println!();

    // Create some vectors
    let mut v1 = Vector2D {
        x: Length::new::<meter>(3.0),
        y: Length::new::<meter>(4.0),
    };
    let v2 = Vector2D {
        x: Length::new::<meter>(1.0),
        y: Length::new::<meter>(2.0),
    };
    let scalar = Ratio::new::<ratio>(2.5);

    println!("=== Vector2D Showcase ===");
    println!(
        "v1: x = {} m, y = {} m",
        v1.x.get::<meter>(),
        v1.y.get::<meter>()
    );
    println!(
        "v2: x = {} m, y = {} m",
        v2.x.get::<meter>(),
        v2.y.get::<meter>()
    );
    println!("scalar: {}", scalar.get::<ratio>());
    println!();

    // Magnitude calculation
    println!("=== Magnitude ===");
    println!("v1.mag(): {} m", v1.mag().get::<meter>());
    println!("v2.mag(): {} m", v2.mag().get::<meter>());
    println!();

    // Addition
    println!("=== Addition ===");
    let v3 = v1 + v2;
    println!(
        "v1 + v2 = x = {} m, y = {} m",
        v3.x.get::<meter>(),
        v3.y.get::<meter>()
    );

    // AddAssign
    println!("=== AddAssign ===");
    let mut v4 = v1;
    println!(
        "Before v4 += v2: x = {} m, y = {} m",
        v4.x.get::<meter>(),
        v4.y.get::<meter>()
    );
    v4 += v2;
    println!(
        "After v4 += v2: x = {} m, y = {} m",
        v4.x.get::<meter>(),
        v4.y.get::<meter>()
    );
    println!();

    // Subtraction
    println!("=== Subtraction ===");
    let v5 = v1 - v2;
    println!(
        "v1 - v2 = x = {} m, y = {} m",
        v5.x.get::<meter>(),
        v5.y.get::<meter>()
    );

    // SubAssign
    println!("=== SubAssign ===");
    let mut v6 = v1;
    println!(
        "Before v6 -= v2: x = {} m, y = {} m",
        v6.x.get::<meter>(),
        v6.y.get::<meter>()
    );
    v6 -= v2;
    println!(
        "After v6 -= v2: x = {} m, y = {} m",
        v6.x.get::<meter>(),
        v6.y.get::<meter>()
    );
    println!();

    // Multiplication by scalar
    println!("=== Multiplication ===");
    let v7 = v1 * scalar;
    println!(
        "v1 * {} = x = {} m, y = {} m",
        scalar.get::<ratio>(),
        v7.x.get::<meter>(),
        v7.y.get::<meter>()
    );

    // MulAssign
    println!("=== MulAssign ===");
    println!(
        "Before v1 *= {}: x = {} m, y = {} m",
        scalar.get::<ratio>(),
        v1.x.get::<meter>(),
        v1.y.get::<meter>()
    );
    v1 *= scalar;
    println!(
        "After v1 *= {}: x = {} m, y = {} m",
        scalar.get::<ratio>(),
        v1.x.get::<meter>(),
        v1.y.get::<meter>()
    );
    println!();

    // Chaining operations
    println!("=== Chained Operations ===");
    let result =
        (v2 + Vector2D {
            x: Length::new::<meter>(1.0),
            y: Length::new::<meter>(1.0),
        }) * Ratio::new::<ratio>(3.0);
    println!(
        "(v2 + (1,1)) * 3.0 = x = {} m, y = {} m",
        result.x.get::<meter>(),
        result.y.get::<meter>()
    );
    println!("Magnitude of result: {} m", result.mag().get::<meter>());
}
