# PhysicalObject Trait

The `PhysicalObject` trait defines a common interface for heterogeneous 2D physics objects. This trait enables collections of different object types (circles, rectangles, etc.) to interact within the same simulation.

## Purpose

This trait is designed for **2D physics** scenarios where you need:
- Different types of objects (circles, rectangles, polygons, etc.)
- Heterogeneous collections (e.g., `Vec<Box<dyn PhysicalObject>>`)
- Polymorphic behavior (different draw methods, collision responses)

> **Note:** For molecular dynamics simulations with homogeneous particles,
> use `PointMass` directly without this trait (see [`point_mass.md`](point_mass.md)).

## Trait Definition

```rust
pub trait PhysicalObject {
    // Force management
    fn reset_forces(&mut self);
    fn apply_force(
        &mut self,
        potential: &dyn Potential,
        other: &dyn PhysicalObject,
        config: &SimulationConfig,
    );

    // Time integration
    fn step(&mut self, step_type: Option<&StepType>, time_step: Time);

    // Getters
    fn pos(&self) -> Vector2D<Length>;
    fn vel(&self) -> Vector2D<Velocity>;
    fn acc(&self) -> Vector2D<Acceleration>;
    fn mass(&self) -> Mass;

    // Setters
    fn set_pos(&mut self, new_value: Vector2D<Length>);
    fn set_vel(&mut self, new_value: Vector2D<Velocity>);
    fn set_acc(&mut self, new_value: Vector2D<Acceleration>);
    fn set_mass(&mut self, new_value: Mass);

    // Rendering
    fn draw(&self, config: &SimulationConfig, scale: Option<f32>, color: Color);
}
```

## Example: Heterogeneous Game Physics

```rust
use engine::objects::physical_object::PhysicalObject;
use molecular_dynamics::physics::potential::Gravity;

// Collection of different object types
let mut objects: Vec<Box<dyn PhysicalObject>> = vec![
    Box::new(Circle::new(...)),
    Box::new(Rectangle::new(...)),
    Box::new(Polygon::new(...)),
];

// Update all objects polymorphically
for obj in &mut objects {
    obj.reset_forces();
}

for i in 0..objects.len() {
    for j in (i+1)..objects.len() {
        // Apply forces between different object types
        objects[i].apply_force(&gravity, objects[j].as_ref(), &config);
    }
}

for obj in &mut objects {
    obj.step(Some(&StepType::VelocityVerlet), time_step);
    obj.draw(&config, None, RED);
}
```

## See Also

- [`PointMass`](point_mass.md) - The concrete particle type for MD simulations
- [`Vector2D`](vector.md) - Vector mathematics for physics
