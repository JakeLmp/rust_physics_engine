# 2D Physics Engine

A hobby project building a basic 2D physics engine in Rust for learning and experimentation.

## Features

Work in progress - building out core physics simulation and visualization.

- Uses [uom](https://docs.rs/uom/) for compile-time unit verification
- Custom `Vector2D` type compatible with `uom` quantities

## Requirements

- Rust (latest stable)
- Python 3 (for pre-commit hooks)

## Setup

```bash
# Clone the repository
git clone git@github.com:JakeLmp/rust_physics_engine.git
cd rust_physics_engine

# Set up pre-commit hooks
python3 -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate
pip install -r requirements.txt
pre-commit install

# Build and run
cargo run
```

## Development

This project uses:
- **macroquad** for 2D rendering
- **uom** for type-safe physical units and quantities
- **pre-commit** with `cargo fmt` and `clippy` (pedantic mode)

Format and lint before committing:
```bash
cargo fmt
cargo clippy --all-targets --all-features -- -W clippy::pedantic
```

## Examples & Documentation

See the documentation files in the [`docs/`](docs/) folder for usage examples and explanations of the main types and traits:

- [`physical_object.md`](docs/physical_object.md): The `PhysicalObject` trait and its implementations.
- [`point_mass.md`](docs/point_mass.md): Creating and updating a physics point-mass object.
- [`cluster.md`](docs/cluster.md): Managing and simulating clusters of physical objects.
- [`vector.md`](docs/vector.md): Performing vector math for physics calculations.

These documentation pages include code snippets and explanations to help you get started with simulation and visualization.

You can also find runnable simulation examples in the `examples/` folder.

To run an example, use:
```bash
cargo run --example <example_name>
```
Replace `<example_name>` with the name of the example file (without the `.rs` extension).
