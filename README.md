# 2D Physics Engine

A hobby project building a basic 2D physics engine in Rust for learning and experimentation.

## Features

Work in progress - building out core physics simulation and visualization.

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
- **pre-commit** with `cargo fmt` and `clippy` (pedantic mode)

Format and lint before committing:
```bash
cargo fmt
cargo clippy --all-targets --all-features -- -W clippy::pedantic
```

## Examples

See the [`docs/point.md`](docs/point.md) and [`docs/vector.md`](docs/vector.md) files for usage examples of the main types:

- **Point**: Creating, updating, and drawing a physics point object.
- **Vector2D**: Performing vector math for physics calculations.

These documentation pages include code snippets and explanations to help you get started with simulation and visualization.
