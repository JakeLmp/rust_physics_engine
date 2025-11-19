use derive_builder::Builder;
use uom::si::f64::Time;

use macroquad::prelude::*;

use crate::simulation::{
    screen::{Screen, ScreenPosition},
    units::{LengthUnit, MassUnit},
};

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

impl SimulationConfig {
    pub fn simulation_setup(&self) {
        set_fullscreen(self.init_fullscreen);
    }

    // pub fn loop_setup(&self) {
    //     if config.display_stats {
    //         Screen::display_stats(
    //             &[("time", &(passed_time.value as f32))],
    //             ScreenPosition::TopRight,
    //             None,
    //             None,
    //             None,
    //             None,
    //         );
    //     }
    // }
}
