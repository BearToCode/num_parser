//!
//! Contains contexts settings.
//!

use std::f64::consts;

use crate::{EvalResult, Value};

/// The number of decimal places shown.
///
/// ## Examples
/// ```
/// use num_parser::*;
///
/// let my_context = Context::new(
///     settings::Rounding::Round(4),
///     settings::AngleUnit::default(),
///     settings::DepthLimit::default()
/// );
///
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rounding {
    /// Round number to a specific decimal place.
    Round(u8),
    /// Disable rounding.
    NoRounding,
}

impl Rounding {
    /// Returns the rounding default value.
    pub fn default() -> Self {
        Rounding::Round(8)
    }
}

/// The angle unit to use.
///
/// ## Examples
/// ```
/// use num_parser::*;
///
/// let my_context = Context::new(
///     settings::Rounding::default(),
///     settings::AngleUnit::Degree, // Or Radian or Turn
///     settings::DepthLimit::default()
/// );
///
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AngleUnit {
    /// Measure angles in radians. A full turn is 2π.
    Radian,
    /// Measure angles in degrees. A full turn is 360°.
    Degree,
    /// Measure angles in turns. A full turn is 1.
    Turn,
}

impl AngleUnit {
    /// Returns the angle unit default value.
    pub fn default() -> Self {
        AngleUnit::Radian
    }

    /// Converts a value from an angle unit to another.
    pub fn convert_value(self, to: Self, value: Value) -> EvalResult<Value> {
        let as_radians = match self {
            Self::Radian => value,
            Self::Degree => value.div(Value::from(180))?.mul(Value::from(consts::PI))?,
            Self::Turn => value.div(Value::from(0.5))?.mul(Value::from(consts::PI))?,
        };

        Ok(match to {
            Self::Radian => as_radians,
            Self::Degree => as_radians
                .div(Value::from(consts::PI))?
                .mul(Value::from(180))?,
            Self::Turn => as_radians
                .div(Value::from(consts::PI))?
                .mul(Value::from(0.5))?,
        })
    }
}

/// The depth limit.
///
/// ## Examples
/// ```
/// use num_parser::*;
///
/// let my_context = Context::new(
///     settings::Rounding::default(),
///     settings::AngleUnit::default(),
///     settings::DepthLimit::Limit(10)
/// );
///
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DepthLimit {
    /// An iteration limit.
    Limit(u32),
    /// No limit.
    ///
    /// **WARNING**: disabling limit prevents recursion control and may cause
    /// the stack to overflow causing the program to panic.
    NoLimit,
}

impl DepthLimit {
    /// Return the depth limit default value.
    pub fn default() -> Self {
        DepthLimit::Limit(49)
    }
}
