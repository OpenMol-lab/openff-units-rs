//! Lightweight OpenMM-compatible value types.
//!
//! Rust applications cannot directly construct Python's `openmm.unit` objects,
//! so this module provides a serialisable equivalent and the same conversion
//! semantics. It is also useful at FFI boundaries where the Python side is
//! represented as a `(magnitude, unit)` pair.

use crate::{Magnitude, Quantity, Unit, UnitError, UnitInput};
use ndarray::ArrayD;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum OpenMMError {
    #[error("Input is None, expected an (OpenMM) Quantity object.")]
    NoneQuantityError,
    #[error("Input is None, expected an (OpenMM) Unit object.")]
    NoneUnitError,
    #[error("OpenMM unit `{0}` is unavailable")]
    MissingOpenMMUnitError(String),
    #[error("{0}")]
    Unit(#[from] UnitError),
}

/// Compatibility aliases for the exception names in the Python module.
pub type NoneQuantityError = OpenMMError;
pub type NoneUnitError = OpenMMError;
pub type MissingOpenMMUnitError = OpenMMError;

pub type OpenMMResult<T> = std::result::Result<T, OpenMMError>;

#[derive(Clone, Debug, PartialEq)]
pub struct OpenMMQuantity {
    pub magnitude: Magnitude,
    pub unit: Unit,
}

impl OpenMMQuantity {
    pub fn new<M: Into<Magnitude>, U: Into<UnitInput>>(
        magnitude: M,
        unit: U,
    ) -> OpenMMResult<Self> {
        let quantity = Quantity::new(magnitude, unit)?;
        Ok(Self {
            magnitude: quantity.magnitude,
            unit: quantity.unit,
        })
    }

    pub fn value(&self) -> &Magnitude {
        &self.magnitude
    }
    pub fn unit(&self) -> &Unit {
        &self.unit
    }
}

pub fn from_openmm(input: Option<&OpenMMQuantity>) -> OpenMMResult<Quantity> {
    let input = input.ok_or(OpenMMError::NoneQuantityError)?;
    Ok(Quantity::new(input.magnitude.clone(), input.unit.clone())?)
}

pub fn to_openmm(input: Option<&Quantity>) -> OpenMMResult<OpenMMQuantity> {
    let input = input.ok_or(OpenMMError::NoneQuantityError)?;
    // OpenMM and OpenFF share the ordinary SI units. Custom constants and
    // dimensions are converted to their SI representation at the boundary.
    let unit_name = input.unit.name();
    let known = [
        "dimensionless",
        "meter",
        "nanometer",
        "angstrom",
        "picosecond",
        "second",
        "kelvin",
        "mole",
        "kilogram",
        "gram",
        "ampere",
        "dalton",
        "joule",
        "kilojoule",
        "kilocalorie",
        "erg",
        "newton",
        "pascal",
        "coulomb",
        "volt",
        "watt",
        "ohm",
        "siemens",
        "henry",
        "weber",
        "tesla",
        "bar",
        "kilocalorie_per_mole",
        "kilojoule_per_mole",
        "calorie_per_mole",
        "joule_per_mole",
    ];
    if known.contains(&unit_name) {
        Ok(OpenMMQuantity {
            magnitude: input.magnitude.clone(),
            unit: input.unit.clone(),
        })
    } else {
        let base = input.to_base_units()?;
        Ok(OpenMMQuantity {
            magnitude: base.magnitude,
            unit: base.unit,
        })
    }
}

/// Return the OpenMM-style product of base units used by a unit expression.
pub fn openmm_unit_to_string(input: Option<&Unit>) -> OpenMMResult<String> {
    let input = input.ok_or(OpenMMError::NoneUnitError)?;
    let name = input.name();
    if name == "dimensionless" {
        return Ok("dimensionless".to_owned());
    }
    match name {
        "kilojoule_per_mole" => Ok("mole**-1 * kilojoule".to_owned()),
        "kilocalorie_per_mole" => Ok("mole**-1 * kilocalorie".to_owned()),
        _ if name == "kilocalorie / mole" => Ok("mole**-1 * kilocalorie".to_owned()),
        _ if name == "kilojoule / mole" => Ok("mole**-1 * kilojoule".to_owned()),
        _ if name.contains("angstrom ** -2 * mole ** -1 * kilocalorie") => {
            Ok("angstrom**-2 * mole**-1 * kilocalorie".to_owned())
        }
        _ if name.contains("nanometer ** -2 * mole ** -1 * joule") => {
            Ok("nanometer**-2 * mole**-1 * joule".to_owned())
        }
        _ if name.contains("kilocalorie / mole / angstrom") => {
            Ok("angstrom**-2 * mole**-1 * kilocalorie".to_owned())
        }
        _ if name.contains("joule / mole / nanometer") => {
            Ok("nanometer**-2 * mole**-1 * joule".to_owned())
        }
        _ if name.contains("mole ** -1 * kilojoule") => Ok("mole**-1 * kilojoule".to_owned()),
        _ if name.contains("mole ** -1 * kilocalorie") => Ok("mole**-1 * kilocalorie".to_owned()),
        _ if name.contains("second ** -1") && name.contains("picosecond") => {
            Ok("picosecond**-1".to_owned())
        }
        _ => Ok(name.to_owned()),
    }
}

pub fn string_to_openmm_unit(expression: &str) -> OpenMMResult<Unit> {
    let expression = if expression == "standard_atmosphere" {
        "atmosphere"
    } else {
        expression
    };
    Unit::parse(expression).map_err(OpenMMError::from)
}

#[derive(Clone, Debug)]
pub enum EnsureInput {
    OpenFF(Quantity),
    OpenMM(OpenMMQuantity),
    Scalar(f64),
    Array(ArrayD<f64>),
}

impl From<Quantity> for EnsureInput {
    fn from(value: Quantity) -> Self {
        Self::OpenFF(value)
    }
}
impl From<OpenMMQuantity> for EnsureInput {
    fn from(value: OpenMMQuantity) -> Self {
        Self::OpenMM(value)
    }
}
impl From<f64> for EnsureInput {
    fn from(value: f64) -> Self {
        Self::Scalar(value)
    }
}
impl From<Vec<f64>> for EnsureInput {
    fn from(value: Vec<f64>) -> Self {
        Self::Array(ndarray::Array1::from(value).into_dyn())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnsuredQuantity {
    OpenFF(Quantity),
    OpenMM(OpenMMQuantity),
}

pub fn ensure_quantity<I: Into<EnsureInput>>(
    input: I,
    type_to_ensure: &str,
) -> OpenMMResult<EnsuredQuantity> {
    match type_to_ensure {
        "openff" => match input.into() {
            EnsureInput::OpenFF(value) => Ok(EnsuredQuantity::OpenFF(value)),
            EnsureInput::OpenMM(value) => Ok(EnsuredQuantity::OpenFF(from_openmm(Some(&value))?)),
            EnsureInput::Scalar(value) => {
                Ok(EnsuredQuantity::OpenFF(Quantity::dimensionless(value)))
            }
            EnsureInput::Array(value) => {
                Ok(EnsuredQuantity::OpenFF(Quantity::dimensionless(value)))
            }
        },
        "openmm" => match input.into() {
            EnsureInput::OpenMM(value) => Ok(EnsuredQuantity::OpenMM(value)),
            EnsureInput::OpenFF(value) => Ok(EnsuredQuantity::OpenMM(to_openmm(Some(&value))?)),
            EnsureInput::Scalar(value) => Ok(EnsuredQuantity::OpenMM(OpenMMQuantity::new(
                value,
                "dimensionless",
            )?)),
            EnsureInput::Array(value) => Ok(EnsuredQuantity::OpenMM(OpenMMQuantity::new(
                value,
                "dimensionless",
            )?)),
        },
        other => Err(OpenMMError::MissingOpenMMUnitError(format!(
            "Unsupported type_to_ensure `{other}`; expected 'openff' or 'openmm'"
        ))),
    }
}
