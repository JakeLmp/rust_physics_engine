use uom::si::{
    f32::{Energy, Force, Length, Ratio},
    ratio::ratio,
};
use uom::typenum::{P6, P8, P12, P14};

use crate::{objects::point::Point, physics::vector::Vector2D};

pub trait Potential {
    /// Potential energy of a position in a one-dimensional potential
    fn energy(&self, position: Length) -> Energy;

    /// Potential energy of a position in a radially symmetric potential
    fn energy_radial(&self, position: Vector2D<Length>) -> Energy;

    /// Force exerted on point1 by point2
    fn force(&self, point1: &Point, point2: &Point) -> Vector2D<Force>;
}

// pub struct Gravity {
//     pub g: Acceleration,
// }

// impl Potential for Gravity {
//     fn energy(&self, position: Length) -> Energy {
//         todo!();
//     }

//     fn energy_radial(&self, position: Vector2D<Length>) -> Energy {
//         todo!();
//     }

//     fn force(&self, point1: &Point, point2: &Point) -> Vector2D<Force> {
//         todo!();
//     }
// }

/// The Lennard-Jones potential, commonly used in molecular dynamics
/// Typical value examples
/// Xenon: ε = 1.77 kJ/mol, σ = 4.10 Å
/// Argon: ε = 0.997 kJ/mol (or ε/k_B = 119.8 K), σ = 3.40 Å
pub struct LennardJones {
    pub epsilon: Energy,
    pub sigma: Length,
}

impl Potential for LennardJones {
    fn energy(&self, position: Length) -> Energy {
        Ratio::new::<ratio>(4.0)
            * self.epsilon
            * ((self.sigma / position).powi(P12::new()) - (self.sigma / position).powi(P6::new()))
    }

    fn energy_radial(&self, position: Vector2D<Length>) -> Energy {
        let r: Length = position.mag();
        self.energy(r)
    }

    fn force(&self, point1: &Point, point2: &Point) -> Vector2D<Force> {
        let r: Vector2D<Length> = point2.pos - point1.pos;
        let r_mag: Length = r.mag();

        r * (Ratio::new::<ratio>(48.) * self.epsilon) / (self.sigma * self.sigma)
            * ((self.sigma / r_mag).powi(P14::new())
                - Ratio::new::<ratio>(0.5) * (self.sigma / r_mag).powi(P8::new()))
    }
}
