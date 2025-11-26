# PointMass

The `PointMass` struct represents a single particle with mass, position, velocity, and acceleration. It is the fundamental building block for molecular dynamics (MD) simulations.

## Initialization

```rust
use uom::si::{
    acceleration::meter_per_second_squared,
    f64::{Acceleration, Length, Mass, Time, Velocity},
    length::meter,
    mass::kilogram,
    velocity::meter_per_second,
    time::second,
};
use physics_core::vector::Vector2D;
use molecular_dynamics::point_mass::PointMass;

let time_step = Time::new::<second>(0.01);

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
    time_step,
);
```

## Force Application

Particles interact via potentials (e.g., Lennard-Jones for molecular dynamics):

```rust
use molecular_dynamics::physics::potential::LennardJones;

// Reset accumulated forces
point.reset_forces();

// Apply pairwise forces from other particles
for other in &other_particles {
    point.apply_force(&lennard_jones, other, &config);
}
```

## Time Integration

Advance the simulation using different integration methods:

```rust
use molecular_dynamics::physics::time_integration::StepType;

// Naive Euler method (simple, less accurate)
point.step(Some(&StepType::Naive), time_step);

// Verlet method (better energy conservation)
point.step(Some(&StepType::Verlet), time_step);

// Velocity Verlet method (most common in MD)
point.step(Some(&StepType::VelocityVerlet), time_step);

// Default to Naive if not specified
point.step(None, time_step);
```

## Visualization

Draw the particle as a circle:

```rust
use macroquad::prelude::RED;
use visualization::simulation::config::SimulationConfig;

// Draw with automatic scaling based on mass
point.draw(&config, None, RED);

// Or with custom radius
point.draw(&config, Some(5.0), RED);
```

## Fields

- `pos`: Position (`Vector2D<Length>`)
- `vel`: Velocity (`Vector2D<Velocity>`)
- `acc`: Acceleration (`Vector2D<Acceleration>`)
- `mass`: Mass (`Mass`)
- `last_pos`: Previous position (used by Verlet integrators)
- `last_vel`: Previous velocity (used by Velocity Verlet)

## Methods

### Construction
- `new(pos, vel, acc, mass, time_step)` - Create a new particle

### Force Application
- `reset_forces(&mut self)` - Zero out acceleration
- `apply_force(&mut self, potential, other, config)` - Apply force from another particle

### Time Integration
- `step(&mut self, step_type, time_step)` - Advance state by one time step

### Accessors
- `pos(&self)`, `vel(&self)`, `acc(&self)`, `mass(&self)` - Getters
- `set_pos(&mut self, ...)`, `set_vel(...)`, `set_acc(...)`, `set_mass(...)` - Setters

### Visualization
- `draw(&self, config, scale, color)` - Render to screen

## See Also

- [`Vector2D`](vector.md) - The 2D vector type used for physics calculations
- [`PhysicalObject`](physical_object.md) - Game physics trait (in `engine` crate)
