use crate::{Result, UnitError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Div, Mul};
use std::str::FromStr;

/// Exponents of the SI base dimensions plus OpenFF's two auxiliary dimensions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimension(pub [i32; 9]);

impl Dimension {
    pub const NONE: Self = Self([0; 9]);

    pub fn with(index: usize) -> Self {
        let mut value = [0; 9];
        value[index] = 1;
        Self(value)
    }

    pub fn combine(self, rhs: Self) -> Self {
        let mut value = [0; 9];
        for (out, (left, right)) in value.iter_mut().zip(self.0.into_iter().zip(rhs.0)) {
            *out = left + right;
        }
        Self(value)
    }

    pub fn scale(self, exponent: i32) -> Self {
        let mut value = [0; 9];
        for (out, item) in value.iter_mut().zip(self.0) {
            *out = item * exponent;
        }
        Self(value)
    }
}

/// A unit is represented by a scale to SI base units and dimension exponents.
#[derive(Clone, Debug)]
pub struct Unit {
    pub(crate) name: String,
    pub(crate) symbol: String,
    pub(crate) scale: f64,
    pub(crate) offset: f64,
    pub(crate) dimension: Dimension,
}

impl Unit {
    pub fn new(name: impl Into<String>, scale: f64, dimension: Dimension) -> Self {
        let name = name.into();
        Self {
            symbol: name.clone(),
            name,
            scale,
            offset: 0.0,
            dimension,
        }
    }

    pub(crate) fn with_symbol(
        name: impl Into<String>,
        symbol: impl Into<String>,
        scale: f64,
        offset: f64,
        dimension: Dimension,
    ) -> Self {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            scale,
            offset,
            dimension,
        }
    }

    pub fn parse(expression: &str) -> Result<Self> {
        crate::registry::unit().parse(expression)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    pub fn is_compatible_with(&self, other: &Unit) -> bool {
        self.dimension == other.dimension
    }

    /// Attach a scalar magnitude to this unit.
    pub fn quantity(
        &self,
        magnitude: impl Into<crate::Magnitude>,
    ) -> crate::Result<crate::Quantity> {
        crate::Quantity::new(magnitude, self.clone())
    }

    pub fn powi(&self, exponent: i32) -> Result<Self> {
        if self.offset != 0.0 && exponent != 1 {
            return Err(UnitError::OffsetUnit);
        }
        Ok(Self::with_symbol(
            format!("{} ** {}", self.name, exponent),
            format!("{}**{}", self.symbol, exponent),
            self.scale.powi(exponent),
            if exponent == 1 { self.offset } else { 0.0 },
            self.dimension.scale(exponent),
        ))
    }

    pub fn mul(&self, other: &Unit) -> Result<Self> {
        if self.offset != 0.0 || other.offset != 0.0 {
            return Err(UnitError::OffsetUnit);
        }
        Ok(Self::with_symbol(
            format!("{} * {}", self.name, other.name),
            format!("{} * {}", self.symbol, other.symbol),
            self.scale * other.scale,
            0.0,
            self.dimension.combine(other.dimension),
        ))
    }

    pub fn div(&self, other: &Unit) -> Result<Self> {
        if self.offset != 0.0 || other.offset != 0.0 {
            return Err(UnitError::OffsetUnit);
        }
        Ok(Self::with_symbol(
            format!("{} / {}", self.name, other.name),
            format!("{} / {}", self.symbol, other.symbol),
            self.scale / other.scale,
            0.0,
            self.dimension.combine(other.dimension.scale(-1)),
        ))
    }

    pub(crate) fn to_base_value(&self, value: f64) -> f64 {
        (value + self.offset) * self.scale
    }

    pub(crate) fn convert_from_base(&self, value: f64) -> f64 {
        value / self.scale - self.offset
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.dimension == other.dimension
            && (self.scale - other.scale).abs()
                <= 1e-14 * self.scale.abs().max(other.scale.abs()).max(1.0)
            && (self.offset - other.offset).abs() <= 1e-14
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

impl FromStr for Unit {
    type Err = UnitError;

    fn from_str(value: &str) -> Result<Self> {
        Self::parse(value)
    }
}

impl Mul for Unit {
    type Output = Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        Unit::mul(&self, &rhs)
    }
}

impl Div for Unit {
    type Output = Result<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        Unit::div(&self, &rhs)
    }
}

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let expression = String::deserialize(deserializer)?;
        Self::parse(&expression).map_err(serde::de::Error::custom)
    }
}

/// Input accepted by [`crate::Quantity::new`] and conversion methods.
#[derive(Clone, Debug)]
pub enum UnitInput {
    Unit(Unit),
    Expression(String),
}

impl From<Unit> for UnitInput {
    fn from(value: Unit) -> Self {
        Self::Unit(value)
    }
}

impl From<&Unit> for UnitInput {
    fn from(value: &Unit) -> Self {
        Self::Unit(value.clone())
    }
}

impl From<&str> for UnitInput {
    fn from(value: &str) -> Self {
        Self::Expression(value.to_owned())
    }
}

impl From<String> for UnitInput {
    fn from(value: String) -> Self {
        Self::Expression(value)
    }
}

impl From<f64> for UnitInput {
    fn from(value: f64) -> Self {
        Self::Expression(value.to_string())
    }
}
