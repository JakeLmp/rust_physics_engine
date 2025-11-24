use macroquad::color::Color;
use uom::si::f64::{Acceleration, Length, Mass, Time, Velocity};

use crate::physics::{potential::Potential, time_integration::StepType};
use physics_core::vector::Vector2D;
use visualization::simulation::config::SimulationConfig;

pub trait PhysicalObject {
    /// Method to reset acceleration before applying forces
    fn reset_forces(&mut self);
    /// Method to apply a force to the object
    fn apply_force(
        &mut self,
        potential: &dyn Potential,
        other: &dyn PhysicalObject,
        config: &SimulationConfig,
    );
    /// Method to apply a time step to the object
    fn step(&mut self, step_type: Option<&StepType>, time_step: Time);

    // Getter methods for pairwise interactions
    fn pos(&self) -> Vector2D<Length>;
    fn vel(&self) -> Vector2D<Velocity>;
    fn acc(&self) -> Vector2D<Acceleration>;
    fn mass(&self) -> Mass;

    // Setter methods
    fn set_pos(&mut self, new_value: Vector2D<Length>);
    fn set_vel(&mut self, new_value: Vector2D<Velocity>);
    fn set_acc(&mut self, new_value: Vector2D<Acceleration>);
    fn set_mass(&mut self, new_value: Mass);

    /// Method to draw the object to the Screen
    fn draw(&self, config: &SimulationConfig, scale: Option<f32>, color: Color);
}
