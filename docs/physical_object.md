# PhysicalObject Trait

The `PhysicalObject` trait defines the interface for any object that participates in the physics simulation. Objects implementing this trait can interact via forces, update their state, and be rendered.

## Trait Methods

- `reset_forces(&mut self)`: Reset the object's accumulated forces (typically sets acceleration to zero).
- `apply_force(&mut self, potential: &dyn Potential, other: &dyn PhysicalObject)`: Apply a force from another object using a specified potential.
- `step(&mut self, step_type: Option<&StepType>, time_step: Time)`: Advance the object's state by one time step using the chosen integration method.
- `pos(&self)`, `vel(&self)`, `acc(&self)`, `mass(&self)`: Getters for position, velocity, acceleration, and mass.
- `draw(&self, config: &SimulationConfig, scale: Option<f32>, color: Color)`: Render the object.

Any struct implementing `PhysicalObject` can be used in clusters and simulations.

---

# PointMass Implementation

The `PointMass` struct is a simple implementation of `PhysicalObject`, representing a simple point-like object with position, velocity, acceleration, and mass.

## Example

```rust
use uom::si::{
    acceleration::meter_per_second_squared,
    f32::{Acceleration, Length, Mass, Velocity},
    length::meter,
    mass::kilogram,
    velocity::meter_per_second,
};
use physics::vector::Vector2D;
use objects::point_mass::{PointMass, StepType, PhysicalObject};

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

// Advance simulation
point.step(Some(&StepType::Naive), time_step);

// Draw the point mass
point.draw(&config, Some(5.0), RED);
```

See also: [`Cluster`](cluster.md), [`Vector2D`](vector.md)
