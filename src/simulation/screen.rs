use macroquad::prelude::{Color, Vec2, draw_circle, screen_height, screen_width};
use uom::si::f64::Length;

use crate::objects::point_mass::PointMass;
use crate::physics::vector::Vector2D;
use crate::simulation::config::SimulationConfig;

pub struct Screen;

impl Screen {
    /// Convert world coordinates to screen coordinates using given config
    pub fn world_to_screen(pos: &Vector2D<Length>, config: &SimulationConfig) -> Vec2 {
        let x = config.length_unit.get(pos.x) * config.pixels_per_length;
        let y = config.length_unit.get(pos.y) * config.pixels_per_length;

        Vec2::new(
            screen_width() / 2.0 + x as f32,
            screen_height() / 2.0 + y as f32,
        )
    }

    /// Draw a point as a circle on the screen
    pub fn draw_point(
        point: &PointMass,
        config: &SimulationConfig,
        radius_multiplier: Option<f32>,
        color: Color,
    ) {
        let screen_pos = Self::world_to_screen(&point.pos, config);
        let radius = config.mass_unit.get(point.mass) as f32 * radius_multiplier.unwrap_or(1.0);

        draw_circle(screen_pos.x, screen_pos.y, radius, color);
    }

    /// Get screen center
    pub fn center() -> Vec2 {
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
    }

    /// Get screen dimensions
    pub fn dimensions() -> Vec2 {
        Vec2::new(screen_width(), screen_height())
    }
}
