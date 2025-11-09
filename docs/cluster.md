# Cluster

The `Cluster` struct represents a collection of physical objects (such as `PointMass`es) that interact with each other according to a specified potential (e.g., gravity). It is used to simulate groups of particles or bodies in a physics simulation.

## Usage

A `Cluster` is typically initialized with a set of bounds and a simulation configuration. It manages its objects and steps their states forward in time.

### Example

```rust
use physics::cluster::{Cluster, RectangularBounds};
use simulation::config::SimulationConfig;

// Define bounds for initialization
let bounds = RectangularBounds {
    x1: ...,
    x2: ...,
    y1: ...,
    y2: ...,
};

// Create a cluster
let cluster = Cluster::new(&config, &bounds);
```

## Main Methods

- **new(config, position_bounds):**
  Initializes a cluster with randomly positioned objects within the given bounds.

- **center_of_mass():**
  Computes the center of mass of all objects in the cluster.

- **step(config, potential, step_type):**
  Advances the simulation by one time step, applying forces between objects and updating their states.

## Fields

- **objects:**
  A vector containing all physical objects in the cluster.

## See Also

- [`PointMass`](point_mass.md)
- [`Vector2D`](vector.md)
