# Vector2D

The `Vector2D` struct provides a generic, unit-safe 2D vector type for physics calculations. It supports arithmetic operations, magnitude calculation, chaining, and is compatible with the [uom](https://docs.rs/uom/) units library.

## Usage

Create vectors with physical units:

```rust
use uom::si::f32::{Length, Ratio};
use uom::si::length::meter;
use uom::si::ratio::ratio;
use crate::physics::vector::Vector2D;

let v1 = Vector2D {
    x: Length::new::<meter>(3.0),
    y: Length::new::<meter>(4.0),
};
let v2 = Vector2D {
    x: Length::new::<meter>(1.0),
    y: Length::new::<meter>(2.0),
};
```

## Arithmetic

- **Zero vector:**
  Create a zero vector for any unit type:
  ```rust
  let z = Vector2D::<Length>::zero();
  ```

- **Addition and subtraction:**
  ```rust
  let v3 = v1 + v2;
  let v4 = v1 - v2;
  ```

- **Assignment operators:**
  ```rust
  let mut v = v1;
  v += v2;
  v -= v2;
  ```

- **Unary negation:**
  Reverse the direction of a vector:
  ```rust
  let neg_v1 = -v1;
  ```

- **Multiplication by scalar (both orderings):**
  You can multiply a vector by a scalar, or a scalar by a vector:
  ```rust
  let scalar = Ratio::new::<ratio>(2.5);
  let v_scaled = v1 * scalar;
  let v_scaled2 = scalar * v1;
  ```
  > **Note:**
  > Multiplying a scalar by a vector (e.g., `scalar * v1`) is only implemented for specific quantity types using a macro in the codebase.
  > If you need this for a not-yet-supported quantity, use the `impl_vector_mul!` macro in your code:
  > ```rust
  > impl_vector_mul!(YourQuantityType);
  > ```
  > This will enable scalar-vector multiplication for your quantity.

- **Division by scalar:**
  ```rust
  let v_div = v1 / scalar;
  ```

## Magnitude

Calculate the magnitude (length) of a vector:
```rust
let mag = v1.mag();
```

## Showcase

```rust
// Create some vectors
let v1 = Vector2D {
    x: Length::new::<meter>(3.0),
    y: Length::new::<meter>(4.0),
};
let v2 = Vector2D {
    x: Length::new::<meter>(1.0),
    y: Length::new::<meter>(2.0),
};
let scalar = Ratio::new::<ratio>(2.5);

println!("=== Vector2D Showcase ===");
println!(
    "v1: x = {} m, y = {} m",
    v1.x.get::<meter>(),
    v1.y.get::<meter>()
);
println!(
    "v2: x = {} m, y = {} m",
    v2.x.get::<meter>(),
    v2.y.get::<meter>()
);
println!("scalar: {}", scalar.get::<ratio>());
println!();

// Magnitude calculation
println!("=== Magnitude ===");
println!("v1.mag(): {} m", v1.mag().get::<meter>());
println!("v2.mag(): {} m", v2.mag().get::<meter>());
println!();

// Addition
println!("=== Addition ===");
let v3 = v1 + v2;
println!(
    "v1 + v2 = x = {} m, y = {} m",
    v3.x.get::<meter>(),
    v3.y.get::<meter>()
);
println!();

// AddAssign
println!("=== AddAssign ===");
let mut v4 = v1;
println!(
    "Before v4 += v2: x = {} m, y = {} m",
    v4.x.get::<meter>(),
    v4.y.get::<meter>()
);
v4 += v2;
println!(
    "After v4 += v2: x = {} m, y = {} m",
    v4.x.get::<meter>(),
    v4.y.get::<meter>()
);
println!();

// Subtraction
println!("=== Subtraction ===");
let v5 = v1 - v2;
println!(
    "v1 - v2 = x = {} m, y = {} m",
    v5.x.get::<meter>(),
    v5.y.get::<meter>()
);
println!();

// SubAssign
println!("=== SubAssign ===");
let mut v6 = v1;
println!(
    "Before v6 -= v2: x = {} m, y = {} m",
    v6.x.get::<meter>(),
    v6.y.get::<meter>()
);
v6 -= v2;
println!(
    "After v6 -= v2: x = {} m, y = {} m",
    v6.x.get::<meter>(),
    v6.y.get::<meter>()
);
println!();

// Multiplication by scalar (both orderings)
println!("=== Multiplication ===");
let v7 = v1 * scalar;
println!(
    "v1 * {} = x = {} m, y = {} m",
    scalar.get::<ratio>(),
    v7.x.get::<meter>(),
    v7.y.get::<meter>()
);
let v8 = scalar * v1;
println!(
    "{} * v1 = x = {} m, y = {} m",
    scalar.get::<ratio>(),
    v8.x.get::<meter>(),
    v8.y.get::<meter>()
);
println!();

// Division
println!("=== Division ===");
let v9 = v1 / scalar;
println!(
    "v1 / {} = x = {} m, y = {} m",
    scalar.get::<ratio>(),
    v9.x.get::<meter>(),
    v9.y.get::<meter>()
);
println!();

// Chaining operations
println!("=== Chained Operations ===");
let result =
    (v2 + Vector2D {
        x: Length::new::<meter>(1.0),
        y: Length::new::<meter>(1.0),
    }) * Ratio::new::<ratio>(3.0);
println!(
    "(v2 + (1,1)) * 3.0 = x = {} m, y = {} m",
    result.x.get::<meter>(),
    result.y.get::<meter>()
);
println!("Magnitude of result: {} m", result.mag().get::<meter>());
```

## Methods

- `zero()`: Returns a zero vector.
- `mag()`: Returns the magnitude of the vector.
- Arithmetic operators: `+`, `-`, `*`, `/` (with scalars and other vectors).
- Assignment operators: `+=`, `-=`.
- Unary negation: `-vector` reverses direction.
- Compatible with any unit type supported by `uom`.

## See also

- [`PointMass`](point_mass.md)
- [`Cluster`](cluster.md)
