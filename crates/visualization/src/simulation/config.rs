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
    // ===== Units =====
    /// Simulation time step
    pub time_step: Time,

    /// Length unit used in drawing
    #[builder(default = "LengthUnit::Meter")]
    pub length_unit: LengthUnit,

    /// Mass unit used in drawing
    #[builder(default = "MassUnit::Kilogram")]
    pub mass_unit: MassUnit,

    // ===== World-to-screen setup =====
    /// The number of time steps to take each frame before rendering
    #[builder(default = None)]
    pub time_steps_per_frame: Option<u8>,

    /// Number of pixels per unit length
    #[builder(default = 1.0)]
    pub pixels_per_length: f64,

    // ===== Visualisation options =====
    /// Start simulation fullscreen
    #[builder(default = false)]
    pub init_fullscreen: bool,

    /// Whether to display stats
    #[builder(default = false)]
    pub display_stats: bool,

    /// Whether to show the boundary of the simulation box
    #[builder(default = false)]
    pub display_boundary: bool,

    // ===== Simulation options =====
    /// If given, use a force-softening minimum distance
    #[builder(default = None)]
    pub force_softening_epsilon: Option<Length>,

    /// If given, use a maximum allowed force
    #[builder(default = None)]
    pub force_cap: Option<Force>,

    /// Boundary type
    #[builder(default = BoundaryKind::Infinite)]
    pub boundary_type: BoundaryKind,

    /// Pairwise interaction cutoff radius `r_c`.
    /// Pair forces where `r > r_c` are skipped
    #[builder(default = None)]
    pub pair_cutoff_radius: Option<Length>,

    /// Type of parallelism to use, if available.
    #[builder(default = ParallelComputationKind::SingleThread)]
    pub parallel_computation_kind: ParallelComputationKind,
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

/// `BoundaryType` options. The boundary size is given as half-width from the origin.
#[derive(Debug, Clone)]
pub enum BoundaryKind {
    /// No boundaries
    Infinite,
    /// Periodic boundaries.
    Periodic(Vector2D<Length>),
    /// Elastic collision with non-moving boundaries.
    Elastic(Vector2D<Length>),
    /// Object is removed from system after passing boundary.
    Open(Vector2D<Length>),
}

impl BoundaryKind {
    /// Get the bounds `Vector2D<Length>`, if applicable
    pub fn bounds(&self) -> Option<Vector2D<Length>> {
        match self {
            BoundaryKind::Infinite => None,
            BoundaryKind::Periodic(bounds) => Some(*bounds),
            BoundaryKind::Elastic(bounds) => Some(*bounds),
            BoundaryKind::Open(bounds) => Some(*bounds),
        }
    }
}

/// Options for parallelism.
#[derive(Debug, Clone)]
pub enum ParallelComputationKind {
    /// Single thread, no parallelism
    SingleThread,
    /// CPU multithreaded
    CPUMultiThread,
    // GPU multithreaded
    // GPUMultiThread,
}
