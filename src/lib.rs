//! A small, serialisable unit and quantity system for OpenFF projects.
//!
//! The public API mirrors the useful, non-Python-specific part of
//! `openff.units`: [`Unit`], [`Quantity`], [`Measurement`], and a process-wide
//! default registry available through [`unit`].

mod quantity;
mod registry;
mod unit;

pub mod data;
pub mod elements;
pub mod openmm;

pub use quantity::{Magnitude, Measurement, Quantity};
pub use registry::{DEFAULT_UNIT_REGISTRY, UnitRegistry, unit};
pub use unit::{Dimension, Unit, UnitInput};

pub type Result<T> = std::result::Result<T, UnitError>;

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum UnitError {
    #[error("unknown unit or constant `{0}`")]
    UnknownUnit(String),
    #[error("invalid unit expression: {0}")]
    Parse(String),
    #[error("incompatible units: `{0}` and `{1}`")]
    IncompatibleUnits(String, String),
    #[error("cannot apply an offset unit in a compound expression")]
    OffsetUnit,
    #[error("invalid magnitude: {0}")]
    Magnitude(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_conversion() {
        let q = Quantity::new(1.4, "angstrom").unwrap();
        let nm = q.to("nanometer").unwrap();
        assert!((nm.value().unwrap() - 0.14).abs() < 1e-12);
    }

    #[test]
    fn arithmetic_and_parsing() {
        let k = Quantity::from_str("10 kcal / mol / nm ** 2").unwrap();
        let k2 = Quantity::new(10.0, "kilocalorie / mole / nanometer^2").unwrap();
        assert!(k.is_compatible_with(k2.u()));
        assert!(
            (k.to("kilocalorie / mole / nanometer^2")
                .unwrap()
                .value()
                .unwrap()
                - 10.0)
                .abs()
                < 1e-12
        );
    }
}
