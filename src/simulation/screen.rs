use macroquad::prelude::{Color, Vec2, WHITE, draw_circle, draw_text, screen_height, screen_width};
use uom::si::Quantity;
use uom::si::f64::Length;

use crate::objects::point_mass::PointMass;
use crate::physics::vector::Vector2D;
use crate::simulation::config::SimulationConfig;

pub struct Screen;

pub enum ScreenPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Screen {
    /// Convert world coordinates to screen coordinates using given config
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn world_to_screen(pos: &Vector2D<Length>, config: &SimulationConfig) -> Vec2 {
        let x = config.length_unit.get(pos.x) * config.pixels_per_length;
        let y = config.length_unit.get(pos.y) * config.pixels_per_length;

        Vec2::new(
            screen_width() / 2.0 + x as f32,
            screen_height() / 2.0 + y as f32,
        )
    }

    /// Draw a point as a circle on the screen
    #[allow(clippy::cast_possible_truncation)]
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

    /// Display statistics about the sim
    pub fn display_stats(
        stats: &[(&str, &f32)],
        screen_pos: ScreenPosition,
        padding: Option<f32>,
        line_height: Option<f32>,
        text_color: Option<macroquad::color::Color>,
    ) {
        let pad = padding.unwrap_or(20.0);
        let line_h = line_height.unwrap_or(20.0);
        let color = text_color.unwrap_or(WHITE);

        let mut x = 0.0;
        let mut y = 0.0;
        match screen_pos {
            ScreenPosition::Top => {
                x = screen_width() / 2.0;
                y = pad;
            }
            ScreenPosition::Bottom => {
                x = screen_width() / 2.0;
                y = screen_height() - pad - (stats.len() as f32 * line_h);
            }
            ScreenPosition::Left => {
                x = pad;
                y = screen_height() / 2.0 - (stats.len() as f32 * line_h) / 2.0;
            }
            ScreenPosition::Right => {
                x = screen_width() - pad;
                y = screen_height() / 2.0 - (stats.len() as f32 * line_h) / 2.0;
            }
            ScreenPosition::TopLeft => {
                x = pad;
                y = pad;
            }
            ScreenPosition::TopRight => {
                x = screen_width() - pad;
                y = pad;
            }
            ScreenPosition::BottomLeft => {
                x = pad;
                y = screen_height() - pad - (stats.len() as f32 * line_h);
            }
            ScreenPosition::BottomRight => {
                x = screen_width() - pad;
                y = screen_height() - pad - (stats.len() as f32 * line_h);
            }
        }

        for (i, (name, value)) in stats.iter().enumerate() {
            let text = format!("{name}: {value}");
            draw_text(&text, x, y + i as f32 * line_h, 20.0, color);
        }
    }

    /// Get screen center
    #[must_use]
    pub fn center() -> Vec2 {
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
    }

    /// Get screen dimensions
    #[must_use]
    pub fn dimensions() -> Vec2 {
        Vec2::new(screen_width(), screen_height())
    }
}
