# PointMass struct

## Initialization

```rust
use uom::si::f32::{Acceleration, Length, Mass, Velocity};
use uom::si::acceleration::meter_per_second_squared;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::velocity::meter_per_second;
use crate::physics::vector::Vector2D;
use crate::objects::point::{PointMass, StepType};

let mut point = PointMass::new(
    Vector2D {
        x: Length::new::<meter>(0.0),
        y: Length::new::<meter>(0.0),
    },
    Vector2D {
        x: Velocity::new::<meter_per_second>(1.0),
        y: Velocity::new::<meter_per_second>(0.5),
    },
    Vector2D {
        x: Acceleration::new::<meter_per_second_squared>(0.0),
        y: Acceleration::new::<meter_per_second_squared>(-9.81),
    },
    Mass::new::<kilogram>(1.5),
);

println!("=== PointMass Showcase ===");
println!(
    "Initial position: ({:.2} m, {:.2} m)",
    point.pos.x.get::<meter>(),
    point.pos.y.get::<meter>()
);
println!(
    "Initial velocity: ({:.2} m/s, {:.2} m/s)",
    point.vel.x.get::<meter_per_second>(),
    point.vel.y.get::<meter_per_second>()
);
println!(
    "Acceleration: ({:.2} m/s², {:.2} m/s²)",
    point.acc.x.get::<meter_per_second_squared>(),
    point.acc.y.get::<meter_per_second_squared>()
);
println!("Mass: {:.2} kg", point.mass.get::<kilogram>());
println!();

println!("=== Naive Step Simulation ===");
for i in 1..=3 {
    point.step(Some(StepType::Naive));
    println!(
        "Step {}: pos = ({:.2} m, {:.2} m), vel = ({:.2} m/s, {:.2} m/s)",
        i,
        point.pos.x.get::<meter>(),
        point.pos.y.get::<meter>(),
        point.vel.x.get::<meter_per_second>(),
        point.vel.y.get::<meter_per_second>()
    );
}
```

## Drawing

```rust
use macroquad::prelude::RED;
use crate::objects::point::PointMass;

// Assuming you have a point already created...
// Draw the point as a circle with radius = mass * mass_multiplier
point.draw_circle(5.0, RED);

// Using the point from the previous example,
// the radius will be: point.mass (1.5 kg) * 5.0 = 7.5 pixels
```
