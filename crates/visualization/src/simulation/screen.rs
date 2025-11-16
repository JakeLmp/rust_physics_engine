use macroquad::prelude::{Vec2, WHITE, draw_text, screen_height, screen_width};
use macroquad::text::measure_text;

use uom::si::f64::Length;

use crate::simulation::config::SimulationConfig;
use physics_core::vector::Vector2D;

pub struct Screen;

#[allow(dead_code)]
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

    /// Display statistics about the sim
    pub fn display_stats(
        stats: &[(&str, &f32)],
        screen_pos: ScreenPosition,
        padding: Option<f32>,
        line_height: Option<f32>,
        font_size: Option<f32>,
        text_color: Option<macroquad::color::Color>,
    ) {
        let pad = padding.unwrap_or(20.0);
        let line_h = line_height.unwrap_or(20.0);
        let font_s = font_size.unwrap_or(20.0);
        let color = text_color.unwrap_or(WHITE);

        let x: f32;
        let y: f32;

        match screen_pos {
            ScreenPosition::Top => {
                x = 0.0;
                y = pad;
            }
            ScreenPosition::Bottom => {
                x = 0.0;
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
            let text_dimensions = measure_text(&text, None, font_s as u16, 1.0);

            let draw_x = match screen_pos {
                ScreenPosition::Top | ScreenPosition::Bottom => {
                    // Center horizontally
                    screen_width() / 2.0 - text_dimensions.width / 2.0
                }
                ScreenPosition::Right | ScreenPosition::TopRight | ScreenPosition::BottomRight => {
                    // Right align
                    x - text_dimensions.width
                }
                _ => x, // Left align
            };

            draw_text(&text, draw_x, y + i as f32 * line_h, font_s, color);
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
