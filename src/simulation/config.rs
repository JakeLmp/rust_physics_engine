struct SimulationConfig {
    length_unit: LengthUnit,
    mass_unit: MassUnit,
    pixels_per_length: f32,
}

impl SimulationConfig {
    fn world_to_screen(&self, pos: &Vector2D<Length>) -> Vec2 {
        let x = self.length_unit.get(pos.x) * self.pixels_per_length;
        let y = self.length_unit.get(pos.y) * self.pixels_per_length;

        Vec2::new(screen_width() / 2.0 + x, screen_height() / 2.0 + y)
    }
}

// Usage:
static CONFIG: SimulationConfig = SimulationConfig {
    length_unit: LengthUnit::Angstrom,
    mass_unit: MassUnit::Dalton,
    pixels_per_length: 50.0,
};

static X: Lazy<Length> = Lazy::new(|| CONFIG.length_unit.create(5.0));
