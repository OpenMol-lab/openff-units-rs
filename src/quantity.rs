use crate::{Result, Unit, UnitError, UnitInput};
use ndarray::{ArrayD, IxDyn};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

/// Numerical value carried by a [`Quantity`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Magnitude {
    Scalar(f64),
    Array(ArrayD<f64>),
}

impl From<f64> for Magnitude {
    fn from(value: f64) -> Self {
        Self::Scalar(value)
    }
}
impl From<f32> for Magnitude {
    fn from(value: f32) -> Self {
        Self::Scalar(value as f64)
    }
}
impl From<i32> for Magnitude {
    fn from(value: i32) -> Self {
        Self::Scalar(value as f64)
    }
}
impl From<i64> for Magnitude {
    fn from(value: i64) -> Self {
        Self::Scalar(value as f64)
    }
}
impl From<usize> for Magnitude {
    fn from(value: usize) -> Self {
        Self::Scalar(value as f64)
    }
}
impl From<Vec<f64>> for Magnitude {
    fn from(value: Vec<f64>) -> Self {
        Self::Array(
            ArrayD::from_shape_vec(IxDyn(&[value.len()]), value).expect("valid vector shape"),
        )
    }
}
impl From<Vec<i32>> for Magnitude {
    fn from(value: Vec<i32>) -> Self {
        Self::from(value.into_iter().map(f64::from).collect::<Vec<_>>())
    }
}
impl From<ArrayD<f64>> for Magnitude {
    fn from(value: ArrayD<f64>) -> Self {
        Self::Array(value)
    }
}

impl Magnitude {
    pub fn as_scalar(&self) -> Option<f64> {
        if let Self::Scalar(value) = self {
            Some(*value)
        } else {
            None
        }
    }
    pub fn as_array(&self) -> Option<&ArrayD<f64>> {
        if let Self::Array(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn map(&self, function: impl Fn(f64) -> f64 + Copy) -> Self {
        match self {
            Self::Scalar(value) => Self::Scalar(function(*value)),
            Self::Array(array) => Self::Array(array.mapv(function)),
        }
    }
}

/// A numerical magnitude associated with a unit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub magnitude: Magnitude,
    #[serde(rename = "units")]
    pub unit: Unit,
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        if !self.unit.is_compatible_with(&other.unit) {
            return false;
        }
        let converted = match convert_magnitude(&other.magnitude, &other.unit, &self.unit) {
            Ok(v) => v,
            Err(_) => return false,
        };
        magnitudes_close(&self.magnitude, &converted)
    }
}

impl Quantity {
    pub fn new<M, U>(magnitude: M, unit: U) -> Result<Self>
    where
        M: Into<Magnitude>,
        U: Into<UnitInput>,
    {
        let unit = resolve_unit(unit.into())?;
        Ok(Self {
            magnitude: magnitude.into(),
            unit,
        })
    }

    pub fn dimensionless<M: Into<Magnitude>>(magnitude: M) -> Self {
        Self::new(magnitude, "dimensionless").expect("dimensionless is built in")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Result<Self> {
        let value = value.trim();
        let split = value.find(char::is_whitespace).ok_or_else(|| {
            UnitError::Parse("quantity must contain a magnitude and unit".to_owned())
        })?;
        let magnitude = value[..split]
            .parse::<f64>()
            .map_err(|_| UnitError::Magnitude(value[..split].to_owned()))?;
        let expression = value[split..].trim().trim_start_matches('*').trim();
        Self::new(magnitude, expression)
    }

    pub fn value(&self) -> Result<f64> {
        self.magnitude
            .as_scalar()
            .ok_or_else(|| UnitError::Magnitude("quantity contains an array".to_owned()))
    }

    pub fn m(&self) -> &Magnitude {
        &self.magnitude
    }
    pub fn magnitude(&self) -> &Magnitude {
        &self.magnitude
    }
    pub fn u(&self) -> &Unit {
        &self.unit
    }
    pub fn units(&self) -> &Unit {
        &self.unit
    }

    pub fn to<U: Into<UnitInput>>(&self, target: U) -> Result<Self> {
        let target = resolve_unit(target.into())?;
        if !self.unit.is_compatible_with(&target) {
            return Err(UnitError::IncompatibleUnits(
                self.unit.to_string(),
                target.to_string(),
            ));
        }
        Ok(Self {
            magnitude: convert_magnitude(&self.magnitude, &self.unit, &target)?,
            unit: target,
        })
    }

    pub fn to_base_units(&self) -> Result<Self> {
        let target = crate::registry::unit().base_unit(self.unit.dimension())?;
        self.to(target)
    }

    pub fn m_as<U: Into<UnitInput>>(&self, target: U) -> Result<Magnitude> {
        Ok(self.to(target)?.magnitude)
    }
    pub fn is_compatible_with<U: Into<UnitInput>>(&self, target: U) -> bool {
        resolve_unit(target.into())
            .map(|u| self.unit.is_compatible_with(&u))
            .unwrap_or(false)
    }

    pub fn plus_minus<U: Into<UnitInput>>(&self, uncertainty: U) -> Result<Measurement> {
        let uncertainty = match uncertainty.into() {
            UnitInput::Unit(unit) => Self::new(1.0, unit)?,
            UnitInput::Expression(value) => Self::from_str(&value)?,
        };
        Ok(Measurement {
            value: self.clone(),
            uncertainty,
        })
    }
}

impl FromStr for Quantity {
    type Err = UnitError;
    fn from_str(value: &str) -> Result<Self> {
        Self::from_str(value)
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.magnitude {
            Magnitude::Scalar(value) => write!(formatter, "{value} {}", self.unit),
            Magnitude::Array(value) => write!(formatter, "{:?} {}", value, self.unit),
        }
    }
}

impl Add for Quantity {
    type Output = Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        let rhs = rhs.to(self.unit.clone())?;
        Ok(Self {
            magnitude: binary_magnitude(&self.magnitude, &rhs.magnitude, |a, b| a + b)?,
            unit: self.unit,
        })
    }
}

impl Sub for Quantity {
    type Output = Result<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = rhs.to(self.unit.clone())?;
        Ok(Self {
            magnitude: binary_magnitude(&self.magnitude, &rhs.magnitude, |a, b| a - b)?,
            unit: self.unit,
        })
    }
}

impl Mul for Quantity {
    type Output = Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(Self {
            magnitude: binary_magnitude(&self.magnitude, &rhs.magnitude, |a, b| a * b)?,
            unit: Unit::mul(&self.unit, &rhs.unit)?,
        })
    }
}

impl Div for Quantity {
    type Output = Result<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        Ok(Self {
            magnitude: binary_magnitude(&self.magnitude, &rhs.magnitude, |a, b| a / b)?,
            unit: Unit::div(&self.unit, &rhs.unit)?,
        })
    }
}

impl Neg for Quantity {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            magnitude: self.magnitude.map(|value| -value),
            unit: self.unit,
        }
    }
}

impl Mul<Unit> for f64 {
    type Output = Result<Quantity>;
    fn mul(self, rhs: Unit) -> Self::Output {
        Quantity::new(self, rhs)
    }
}
impl Mul<f64> for Unit {
    type Output = Result<Quantity>;
    fn mul(self, rhs: f64) -> Self::Output {
        Quantity::new(rhs, self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub value: Quantity,
    pub uncertainty: Quantity,
}

impl Measurement {
    pub fn new(value: Quantity, uncertainty: Quantity) -> Result<Self> {
        if !value.unit.is_compatible_with(&uncertainty.unit) {
            return Err(UnitError::IncompatibleUnits(
                value.unit.to_string(),
                uncertainty.unit.to_string(),
            ));
        }
        Ok(Self { value, uncertainty })
    }
    pub fn value(&self) -> &Quantity {
        &self.value
    }
    pub fn error(&self) -> &Quantity {
        &self.uncertainty
    }
}

fn resolve_unit(input: UnitInput) -> Result<Unit> {
    match input {
        UnitInput::Unit(unit) => Ok(unit),
        UnitInput::Expression(expression) => {
            if expression.trim() == "dimensionless" {
                return Ok(Unit::new("dimensionless", 1.0, crate::Dimension::NONE));
            }
            crate::registry::unit().parse(&expression)
        }
    }
}

fn convert_magnitude(value: &Magnitude, source: &Unit, target: &Unit) -> Result<Magnitude> {
    if !source.is_compatible_with(target) {
        return Err(UnitError::IncompatibleUnits(
            source.to_string(),
            target.to_string(),
        ));
    }
    let convert = |item: f64| target.convert_from_base(source.to_base_value(item));
    Ok(value.map(convert))
}

fn binary_magnitude(
    left: &Magnitude,
    right: &Magnitude,
    operation: impl Fn(f64, f64) -> f64 + Copy,
) -> Result<Magnitude> {
    match (left, right) {
        (Magnitude::Scalar(a), Magnitude::Scalar(b)) => Ok(Magnitude::Scalar(operation(*a, *b))),
        (Magnitude::Array(a), Magnitude::Array(b)) => {
            if a.shape() != b.shape() {
                return Err(UnitError::Magnitude("array shapes do not match".to_owned()));
            }
            Ok(Magnitude::Array(
                ndarray::Zip::from(a)
                    .and(b)
                    .map_collect(|x, y| operation(*x, *y)),
            ))
        }
        (Magnitude::Array(a), Magnitude::Scalar(b)) => {
            Ok(Magnitude::Array(a.mapv(|x| operation(x, *b))))
        }
        (Magnitude::Scalar(a), Magnitude::Array(b)) => {
            Ok(Magnitude::Array(b.mapv(|x| operation(*a, x))))
        }
    }
}

fn magnitudes_close(left: &Magnitude, right: &Magnitude) -> bool {
    match (left, right) {
        (Magnitude::Scalar(a), Magnitude::Scalar(b)) => {
            (*a - *b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.0)
        }
        (Magnitude::Array(a), Magnitude::Array(b)) => {
            a.shape() == b.shape()
                && a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| (*x - *y).abs() <= 1e-12 * x.abs().max(y.abs()).max(1.0))
        }
        _ => false,
    }
}
