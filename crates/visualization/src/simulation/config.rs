use derive_builder::Builder;
use uom::si::f64::Time;

use crate::simulation::units::{LengthUnit, MassUnit};

#[derive(Debug, Builder)]
pub struct SimulationConfig {
    /// uom::si::f32::Time object specifying time step between each simulation frame
    pub time_step: Time,
    /// The number of time steps to take each frame before rendering
    #[builder(default = None)]
    pub time_steps_per_frame: Option<u8>,
    /// Length unit used in drawing
    #[builder(default = "LengthUnit::Meter")]
    pub length_unit: LengthUnit,
    /// Mass unit used in drawing
    #[builder(default = "MassUnit::Kilogram")]
    pub mass_unit: MassUnit,
    /// Number of pixels per unit length
    #[builder(default = 1.0)]
    pub pixels_per_length: f64,
    /// Start simulation fullscreen
    #[builder(default = false)]
    pub init_fullscreen: bool,
    /// Whether to display stats
    #[builder(default = false)]
    pub display_stats: bool,
}
