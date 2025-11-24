use uom::si::{
    ISQ, Quantity, SI,
    energy::electronvolt,
    f64::{Acceleration, Energy, Force, Length, Mass, Ratio, Time, Velocity},
    length::{angstrom, meter},
    mass::kilogram,
    ratio::ratio,
    time::second,
};
use uom::typenum::{N1, N2, P2, P3, P6, P8, P12, P14, Z0};
use visualization::simulation::config::SimulationConfig;

use crate::objects::physical_object::PhysicalObject;
use physics_core::vector::Vector2D;

// ----- HELPER FUNCTIONS -----

/// If configured, apply force softening to a distance magnitude
fn soften_distance(r_mag: Length, config: &SimulationConfig) -> Length {
    if let Some(epsilon) = config.force_softening_epsilon {
        r_mag + epsilon
    } else {
        r_mag
    }
}

/// If configured, apply force cap
fn cap_force(force: Vector2D<Force>, config: &SimulationConfig) -> Vector2D<Force> {
    if let Some(cap) = config.force_cap {
        let mag = force.mag();
        if mag > cap {
            return force * (cap / mag);
        }
    }
    force
}

// ----- TRAIT DEFINITION -----

#[allow(dead_code)]
pub trait Potential {
    /// Returns a new potential struct with default parameter values
    fn default() -> Self
    where
        Self: Sized;

    /// Potential energy between two points
    fn energy(&self, object1: &dyn PhysicalObject, object2: &dyn PhysicalObject) -> Energy;

    /// Force exerted on object1 by object2
    fn force(
        &self,
        object1: &dyn PhysicalObject,
        object2: &dyn PhysicalObject,
        config: &SimulationConfig,
    ) -> Vector2D<Force>;

    /// Compute pair-wise force from provided arrays
    fn force_from_arrays(
        &self,
        idx1: usize,
        idx2: usize,
        pos_arr: &Vec<Vector2D<Length>>,
        vel_arr: &Vec<Vector2D<Velocity>>,
        acc_arr: &Vec<Vector2D<Acceleration>>,
        mass_arr: &Vec<Mass>,
        config: &SimulationConfig,
    ) -> Vector2D<Force>;
}

// ----- GRAVITY POTENTIAL -----

// Define the type for G: m³/(kg·s²)
pub type GravitationalParameter = Quantity<
    ISQ<P3, N1, N2, Z0, Z0, Z0, Z0>, // L³·M⁻¹·T⁻²
    SI<f64>,
    f64,
>;

/// Newtonian Gravity potential.
/// Typical value for the Gravitational Constant is G = 6.67430×10⁻¹¹ m³·kg⁻¹·s⁻²
pub struct Gravity {
    pub big_g: GravitationalParameter,
}

impl Potential for Gravity {
    fn default() -> Self {
        Self {
            big_g: 6.67430e-11 * Length::new::<meter>(1.0).powi(P3::new())
                / (Mass::new::<kilogram>(1.0) * Time::new::<second>(1.0).powi(P2::new())),
        }
    }

    /// Gravitational potential energy: U = -G·m₁·m₂/r
    fn energy(&self, object1: &dyn PhysicalObject, object2: &dyn PhysicalObject) -> Energy {
        let r = object2.pos() - object1.pos();
        -self.big_g * object1.mass() * object2.mass() / r.mag()
    }

    /// Gravitational force: F = G·m₁·m₂·r̂/r²
    fn force(
        &self,
        object1: &dyn PhysicalObject,
        object2: &dyn PhysicalObject,
        config: &SimulationConfig,
    ) -> Vector2D<Force> {
        let r: Vector2D<Length> = object1.pos() - object2.pos();
        let r_mag = soften_distance(r.mag(), config);
        let r_hat: Vector2D<Ratio> = r / r_mag;

        let force: Vector2D<Force> =
            -r_hat * self.big_g * object1.mass() * object2.mass() / (r_mag * r_mag);

        cap_force(force, config)
    }

    /// Gravitational force: F = G·m₁·m₂·r̂/r²
    fn force_from_arrays(
        &self,
        idx1: usize,
        idx2: usize,
        pos_arr: &Vec<Vector2D<Length>>,
        _vel_arr: &Vec<Vector2D<Velocity>>,
        _acc_arr: &Vec<Vector2D<Acceleration>>,
        mass_arr: &Vec<Mass>,
        config: &SimulationConfig,
    ) -> Vector2D<Force> {
        let r: Vector2D<Length> = pos_arr[idx1] - pos_arr[idx2];
        let r_mag: Length = soften_distance(r.mag(), config);
        let r_hat: Vector2D<Ratio> = r / r_mag;

        let force = -r_hat * self.big_g * mass_arr[idx1] * mass_arr[idx2] / (r_mag * r_mag);

        cap_force(force, config)
    }
}

// ----- LENNARD-JONES POTENTIAL -----

/// The Lennard-Jones potential, commonly used in molecular dynamics
/// Typical value examples
/// Xenon: ε = 0.0184 eV, σ = 4.10 Å
/// Argon: ε = 0.0104 eV (or `ε/k_B` = 119.8 K), σ = 3.40 Å
pub struct LennardJones {
    pub epsilon: Energy,
    pub sigma: Length,
}

impl Potential for LennardJones {
    /// Returns `LennardJones` with parameters for Argon gas
    fn default() -> Self {
        Self {
            epsilon: Energy::new::<electronvolt>(0.0104),
            sigma: Length::new::<angstrom>(3.4),
        }
    }

    /// Lennard-Jones potential energy: U = 4ε[(σ/r)¹² - (σ/r)⁶]
    fn energy(&self, object1: &dyn PhysicalObject, object2: &dyn PhysicalObject) -> Energy {
        let r: Vector2D<Length> = object2.pos() - object1.pos();
        Ratio::new::<ratio>(4.0)
            * self.epsilon
            * ((self.sigma / r.mag()).powi(P12::new()) - (self.sigma / r.mag()).powi(P6::new()))
    }

    /// Lennard-Jones force: F = (48ε/σ²)·r·[(σ/r)¹⁴ - 0.5(σ/r)⁸]
    fn force(
        &self,
        object1: &dyn PhysicalObject,
        object2: &dyn PhysicalObject,
        config: &SimulationConfig,
    ) -> Vector2D<Force> {
        let r: Vector2D<Length> = object1.pos() - object2.pos();
        let r_mag = soften_distance(r.mag(), config);

        let force: Vector2D<Force> = r * (Ratio::new::<ratio>(48.) * self.epsilon)
            / (self.sigma * self.sigma)
            * ((self.sigma / r_mag).powi(P14::new())
                - Ratio::new::<ratio>(0.5) * (self.sigma / r_mag).powi(P8::new()));

        cap_force(force, config)
    }

    /// Lennard-Jones force: F = (48ε/σ²)·r·[(σ/r)¹⁴ - 0.5(σ/r)⁸]
    fn force_from_arrays(
        &self,
        idx1: usize,
        idx2: usize,
        pos_arr: &Vec<Vector2D<Length>>,
        _vel_arr: &Vec<Vector2D<Velocity>>,
        _acc_arr: &Vec<Vector2D<Acceleration>>,
        _mass_arr: &Vec<Mass>,
        config: &SimulationConfig,
    ) -> Vector2D<Force> {
        let r: Vector2D<Length> = pos_arr[idx1] - pos_arr[idx2];
        let r_mag: Length = soften_distance(r.mag(), config);

        let force = r * (Ratio::new::<ratio>(48.) * self.epsilon) / (self.sigma * self.sigma)
            * ((self.sigma / r_mag).powi(P14::new())
                - Ratio::new::<ratio>(0.5) * (self.sigma / r_mag).powi(P8::new()));

        cap_force(force, config)
    }
}
