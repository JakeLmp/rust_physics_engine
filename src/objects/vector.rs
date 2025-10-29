#[derive(Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
