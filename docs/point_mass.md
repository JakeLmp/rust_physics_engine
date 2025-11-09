# PointMass

The `PointMass` struct represents a single point-like object with mass, position, velocity, and acceleration. It is the basic building block for physics simulations in this engine.

> **Note:**
> `PointMass` implements the [`PhysicalObject`](physical_object.md) trait, which provides a common interface for simulation objects.
> This includes methods for resetting forces, applying forces, stepping the simulation, accessing position/velocity/acceleration/mass, and drawing.

## Initialization

```rust
use uom::si::{
    acceleration::meter_per_second_squared,
    f32::{Acceleration, Length, Mass, Velocity},
    length::meter,
    mass::kilogram,
    velocity::meter_per_second,
};
use physics::vector::Vector2D;
use objects::point_mass::{PointMass, StepType};

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
```

## Stepping

Advance the simulation using different integration methods:

```rust
for i in 1..=3 {
    point.step(Some(&StepType::Naive), time_step);
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

Draw the point mass as a circle on the screen:

```rust
use macroquad::prelude::RED;
use simulation::config::SimulationConfig;

// Draw the point with a custom scale and color
point.draw(&config, Some(5.0), RED);
```

## Fields

- `pos`: Position (`Vector2D<Length>`)
- `vel`: Velocity (`Vector2D<Velocity>`)
- `acc`: Acceleration (`Vector2D<Acceleration>`)
- `mass`: Mass (`Mass`)

## Methods

- `new(...)`: Create a new `PointMass`
- `step(...)`: Advance the state using a chosen integration method
- `draw(...)`: Render the point mass to the screen
- Trait methods from `PhysicalObject`:
  - `reset_forces(&mut self)`
  - `apply_force(&mut self, potential: &dyn Potential, other: &dyn PhysicalObject)`
  - `step(&mut self, step_type: Option<&StepType>, time_step: Time)`
  - `pos(&self)`, `vel(&self)`, `acc(&self)`, `mass(&self)`
  - `draw(&self, config: &SimulationConfig, scale: Option<f32>, color: Color)`

## See Also

- [`Cluster`](cluster.md)
- [`Vector2D`](vector.md)
