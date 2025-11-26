# Simulation Architecture

This document describes how particle simulations are structured in the molecular dynamics crate.

## Overview

Molecular dynamics simulations work with collections of `PointMass` particles that interact via potentials.

## Basic Simulation Loop

```rust
use molecular_dynamics::point_mass::PointMass;
use molecular_dynamics::physics::potential::LennardJones;
use molecular_dynamics::physics::time_integration::StepType;

// Initialize particles
let mut particles: Vec<PointMass> = vec![
    PointMass::new(pos1, vel1, acc1, mass, time_step),
    PointMass::new(pos2, vel2, acc2, mass, time_step),
    // ... more particles
];

let lennard_jones = LennardJones::new(epsilon, sigma);

// Main simulation loop
loop {
    // 1. Reset forces
    for particle in &mut particles {
        particle.reset_forces();
    }

    // 2. Calculate pairwise forces
    for i in 0..particles.len() {
        for j in (i+1)..particles.len() {
            let (left, right) = particles.split_at_mut(j);
            left[i].apply_force(&lennard_jones, &right[0], &config);
        }
    }

    // 3. Integrate equations of motion
    for particle in &mut particles {
        particle.step(Some(&StepType::VelocityVerlet), time_step);
    }

    // 4. Render (optional)
    for particle in &particles {
        particle.draw(&config, None, RED);
    }
}
```

## See Also

- [`PointMass`](point_mass.md) - The particle struct
- [`Vector2D`](vector.md) - Vector mathematics
- [`PhysicalObject`](physical_object.md) - Game physics trait (different use case)
