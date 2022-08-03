//!
//! Contains contexts settings.
//!

/// The number of decimal places shown.
///
/// ## Examples
/// ```
/// use numcore::*;
///
/// let my_context = Context::new(settings::Rounding::Round(4));
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rounding {
    Round(u8),
    NoRounding,
}
