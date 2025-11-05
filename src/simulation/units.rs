//! Unit definitions for config-level scaling.
//!
//! This module provides enums and helpers for working with physical units
//! (length and mass) using `uom`. Intended for use in configuration and
//! scaling of simulation parameters.

use uom::si::{
    f32::{Length, Mass},
    length::{angstrom, nanometer, picometer},
    mass::{dalton, kilogram},
};

#[derive(Clone, Copy)]
enum LengthUnit {
    Angstrom,
    Nanometer,
    Picometer,
}

impl LengthUnit {
    fn create(&self, value: f32) -> Length {
        match self {
            LengthUnit::Angstrom => Length::new::<angstrom>(value),
            LengthUnit::Nanometer => Length::new::<nanometer>(value),
            LengthUnit::Picometer => Length::new::<picometer>(value),
        }
    }

    fn get(&self, length: Length) -> f32 {
        match self {
            LengthUnit::Angstrom => length.get::<angstrom>(),
            LengthUnit::Nanometer => length.get::<nanometer>(),
            LengthUnit::Picometer => length.get::<picometer>(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            LengthUnit::Angstrom => "Å",
            LengthUnit::Nanometer => "nm",
            LengthUnit::Picometer => "pm",
        }
    }
}

#[derive(Clone, Copy)]
enum MassUnit {
    Dalton,
    Kilogram,
}

impl MassUnit {
    fn create(&self, value: f32) -> Mass {
        match self {
            MassUnit::Dalton => Mass::new::<dalton>(value),
            MassUnit::Kilogram => Mass::new::<kilogram>(value),
        }
    }

    fn get(&self, mass: Mass) -> f32 {
        match self {
            MassUnit::Dalton => mass.get::<dalton>(),
            MassUnit::Kilogram => mass.get::<kilogram>(),
        }
    }
}
