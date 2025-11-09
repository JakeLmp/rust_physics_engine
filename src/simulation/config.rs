use uom::si::f64::Time;

use crate::simulation::units::*;

pub struct SimulationConfig {
    pub time_step: Time,
    pub length_unit: LengthUnit,
    pub mass_unit: MassUnit,
    pub pixels_per_length: f64,
}
