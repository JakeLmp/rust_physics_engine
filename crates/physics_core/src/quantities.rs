use crate::vector::Vector2D;
use uom::{
    ConstZero,
    si::{
        energy::joule,
        f64::{
            Energy, HeatCapacity, Length, Mass, Momentum, Ratio, ThermodynamicTemperature, Velocity,
        },
        heat_capacity::{boltzmann_constant, joule_per_kelvin},
        mass::kilogram,
        ratio::ratio,
        thermodynamic_temperature::kelvin,
    },
};

/// Helper function:
/// Checks if input lists are of the same length
fn check_list_lengths<T, S>(l1: &[T], l2: &[S]) {
    if l1.len() != l2.len() {
        panic!(
            "Lengths of inputs ({:?}) and ({:?}) don't match",
            l1.len(),
            l2.len()
        )
    };
}

/// Calculates the center of mass for a system of particles:
///
/// ```text
/// R_cm = (Σ m_i * r_i) / (Σ m_i)
/// ```
///
/// where:
/// - `R_cm` is the center of mass position vector
/// - `m_i` is the mass of particle i
/// - `r_i` is the position vector of particle i
pub fn center_of_mass(positions: &[Vector2D<Length>], masses: &[Mass]) -> Vector2D<Length> {
    check_list_lengths(positions, masses);
    let (weighted_sum, total_mass) = positions.iter().zip(masses).fold(
        (
            Vector2D::<Length>::zero() * Mass::new::<kilogram>(0.0),
            Mass::new::<kilogram>(0.0),
        ),
        |(com, sum_mass), (pos, mass)| (com + mass * pos, sum_mass + *mass),
    );
    weighted_sum / total_mass
}

/// Calculates the mean velocity of a system of particles:
///
/// ```text
/// v_mean = (Σ v_i) / N
/// ```
///
/// where:
/// - `v_i` is the velocity vector of particle i
/// - `N` is the number of particles
pub fn mean_velocity(velocities: &[Vector2D<Velocity>]) -> Vector2D<Velocity> {
    Ratio::new::<ratio>((1 / velocities.len()) as f64)
        * velocities
            .iter()
            .fold(Vector2D::<Velocity>::zero(), |total_vel, vel| {
                total_vel + vel
            })
}

/// Calculates the mean momentum of a system of particles:
///
/// ```text
/// p_mean = (Σ m_i * v_i) / N
/// ```
///
/// where:
/// - `m_i` is the mass of particle i
/// - `v_i` is the velocity vector of particle i
/// - `N` is the number of particles
pub fn mean_momentum(velocities: &[Vector2D<Velocity>], masses: &[Mass]) -> Vector2D<Momentum> {
    check_list_lengths(velocities, masses);
    Ratio::new::<ratio>((1 / velocities.len()) as f64)
        * velocities
            .iter()
            .zip(masses)
            .fold(Vector2D::<Momentum>::zero(), |total_moment, (vel, mass)| {
                total_moment + mass * vel
            })
}

/// Calculates the total kinetic energy of a system of particles:
///
/// ```text
/// E_k = Σ (1/2 * m_i * |v_i|²)
/// ```
///
/// where:
/// - `m_i` is the mass of particle i
/// - `|v_i|²` is the squared magnitude of velocity (v_x² + v_y²)
pub fn kinetic_energy(velocities: &[Vector2D<Velocity>], masses: &[Mass]) -> Energy {
    check_list_lengths(velocities, masses);
    velocities
        .iter()
        .zip(masses)
        .fold(Energy::ZERO, |tot_en, (vel, mass)| {
            tot_en + *mass * (vel.x * vel.x + vel.y * vel.y) / Ratio::new::<ratio>(2.0)
        })
}

/// Calculates the mean kinetic energy of a system of particles:
///
/// ```text
/// E_k = (Σ (1/2 * m_i * |v_i|²)) / N
/// ```
///
/// where:
/// - `m_i` is the mass of particle i
/// - `|v_i|²` is the squared magnitude of velocity (v_x² + v_y²)
/// - `N` is the number of particles
pub fn mean_kinetic_energy(velocities: &[Vector2D<Velocity>], masses: &[Mass]) -> Energy {
    kinetic_energy(velocities, masses) / Ratio::new::<ratio>(velocities.len() as f64)
}

/// Calculates the temperature from the equipartition theorem:
///
/// ```text
/// T = (2 * E_k) / (g * k_B)
/// ```
///
/// where:
/// - `E_k` is the total kinetic energy
/// - `g` is the number of degrees of freedom (default: 3N)
/// - `k_B` is the Boltzmann constant
/// - `N` is the number of particles
pub fn temperature(
    velocities: &[Vector2D<Velocity>],
    masses: &[Mass],
    degrees_of_freedom: Option<usize>,
    boltz_const: Option<HeatCapacity>,
) -> ThermodynamicTemperature {
    check_list_lengths(velocities, masses);

    let g = degrees_of_freedom.unwrap_or(3 * velocities.len()) as f64;
    let kb: HeatCapacity = boltz_const.unwrap_or(HeatCapacity::new::<boltzmann_constant>(1.0));
    let ek: Energy = kinetic_energy(velocities, masses);

    let temp_value = 2.0 * ek.get::<joule>() / (g * kb.get::<joule_per_kelvin>());
    ThermodynamicTemperature::new::<kelvin>(temp_value)
}
