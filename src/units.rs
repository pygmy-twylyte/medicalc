//! Units module
//!
//! Defines units as types, storing only their abbreviation and each implementing
//! the `Unit` trait. This allows them to be used as a generic type with numeric
//! lab values, using the type system to ensure SI and conventional units aren't confused.

pub trait Unit {
    const ABBR: &'static str;
}

pub mod glucose;
pub mod sodium;

/// Milliequivalents per liter (mEq/L).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeqL;
impl Unit for MeqL {
    const ABBR: &'static str = "mEq/L";
}

/// Milligrams per deciliter (mg/dL).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MgdL;
impl Unit for MgdL {
    const ABBR: &'static str = "mg/dL";
}

/// Millimoles per liter (mmol/L).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MmolL;
impl Unit for MmolL {
    const ABBR: &'static str = "mmol/L";
}
