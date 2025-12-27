//! Units module
//!
//! Defines units as types, storing only their abbreviation and each implementing
//! the `Unit` trait. This allows them to be used as a generic type with numeric
//! lab values, using the type system to ensure SI and conventional units aren't confused.

pub trait Unit {
    const ABBR: &'static str;
}

/*
 *
 *
 *              CONVENTIONAL UNITS
 *
 *
 */

/// Milliequivalents per liter (mEq/L).
pub struct MeqL;
impl Unit for MeqL {
    const ABBR: &'static str = "mEq/L";
}

/*
 *
 *
 *             SI UNITS
 *
 *
 */

/// Millimoles per liter (mmol/L).
pub struct MmolL;
impl Unit for MmolL {
    const ABBR: &'static str = "mmol/L";
}
