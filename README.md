# Physics Engine

A hobby project building and visualising physical simulations in Rust for learning and experimentation.

## Project Structure

This workspace contains several specialized crates:

```
crates/
├── physics_core/          # Core physics types (Vector2D, etc.)
├── molecular_dynamics/    # Particle simulations (MD)
├── visualization/         # Rendering and UI
└── engine/                # 2D physics engine (WIP)
```

### Crate Purposes

- **`physics_core`**: Fundamental types like `Vector2D` with unit-safe arithmetic
- **`molecular_dynamics`**: Molecular dynamics simulations with homogeneous particles (`PointMass`)
- **`visualization`**: Screen rendering, configuration, and UI using macroquad
- **`engine`**: 2D physics with heterogeneous objects via the `PhysicalObject` trait

## Features

- **Type-safe units**: Uses [uom](https://docs.rs/uom/) for compile-time dimensional analysis. Custom `Vector2D` struct implementing generic uom quantities.
- **Multiple integrators**: Naive Euler, Verlet, Velocity Verlet
- **Flexible potentials**: Lennard-Jones, gravity, custom potentials
- **Real-time visualization**: Interactive 2D rendering with macroquad

## Quick Start

### Requirements

- Rust (latest stable)
- Python 3 (for pre-commit hooks)

### Setup

```bash
# Clone the repository
git clone git@github.com:JakeLmp/rust_physics_engine.git
cd rust_physics_engine

# Set up pre-commit hooks
python3 -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate
pip install -r requirements.txt
pre-commit install
```

### Run Examples

```bash
# Argon atoms with Lennard-Jones potential
cargo run --example argon --release

# Large-scale molecular dynamics
cargo run --example argon_large --release

# Gravitational two-body problem
cargo run --example earth_moon --release
```

## Documentation

See the [`docs/md/`](docs/md/) folder for detailed guides:

- **[`vector.md`](docs/md/vector.md)** - Vector mathematics with unit safety
- **[`point_mass.md`](docs/md/point_mass.md)** - Particle physics for MD simulations
- **[`cluster.md`](docs/md/cluster.md)** - Simulation architecture and patterns
- **[`physical_object.md`](docs/md/physical_object.md)** - 2D physics trait (engine crate)

## Development

This project uses:
- **macroquad** for 2D rendering
- **uom** for type-safe physical units and quantities
- **pre-commit** with `cargo fmt` and `clippy` (pedantic mode)
