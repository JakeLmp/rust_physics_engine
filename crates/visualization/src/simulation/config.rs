use derive_builder::Builder;
use physics_core::vector::Vector2D;
use uom::si::f64::{Force, Length, Time};

use macroquad::prelude::*;

use crate::simulation::{
    // screen::{Screen, ScreenPosition},
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

    /// If given, use a force-softening minimum distance
    #[builder(default = None)]
    pub force_softening_epsilon: Option<Length>,

    /// If given, use a maximum allowed force
    #[builder(default = None)]
    pub force_cap: Option<Force>,

    /// Boundary type
    #[builder(default = BoundaryType::Infinite)]
    pub boundary_type: BoundaryType,

    /// Pairwise interaction cutoff radius `r_c`.
    /// Pair forces where `r > r_c` are skipped
    #[builder(default = None)]
    pub pair_cutoff_radius: Option<Length>,
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

/// `BoundaryType` options. The boundary size
#[derive(Debug, Clone)]
pub enum BoundaryType {
    /// No boundaries
    Infinite,
    /// Periodic boundaries. Size given as half-width from origin.
    Periodic(Vector2D<Length>),
    /// Elastic collision with non-moving boundaries. Size given as half-width from origin.
    Elastic(Vector2D<Length>),
    /// Object is removed from system after passing boundary. Size given as half-width from origin.
    Open(Vector2D<Length>),
}

impl BoundaryType {
    /// Get the bounds `Vector2D<Length>`, if applicable
    pub fn bounds(&self) -> Option<Vector2D<Length>> {
        match self {
            BoundaryType::Infinite => None,
            BoundaryType::Periodic(bounds) => Some(*bounds),
            BoundaryType::Elastic(bounds) => Some(*bounds),
            BoundaryType::Open(bounds) => Some(*bounds),
        }
    }
}
