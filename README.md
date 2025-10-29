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
git clone <your-repo-url>
cd <repo-name>

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
