//! Units module
//!
//! Defines units as types, storing only their abbreviation and each implementing
//! the `Unit` trait. This allows them to be used as a generic type with numeric
//! lab values, using the type system to ensure SI and conventional units aren't confused.

pub trait Unit {
    const ABBR: &'static str;
}

pub mod bilirubin;
pub mod creatinine;
pub mod glucose;
pub mod sodium;
pub mod vitals;

/// INR "Units" (actually unitless)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InrUnit;
impl Unit for InrUnit {
    const ABBR: &'static str = "INR";
}

/// GFR Units (mL/min/1.73m^2)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GfrUnit;
impl Unit for GfrUnit {
    const ABBR: &'static str = "mL/min/1.73m²";
}

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

/// Micromoles per liter (µmol/L)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UmolL;
impl Unit for UmolL {
    const ABBR: &'static str = "µmol/L";
}

/// Kilograms
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kg;
impl Unit for Kg {
    const ABBR: &'static str = "kg";
}

/// Pounds
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lb;
impl Unit for Lb {
    const ABBR: &'static str = "lb";
}

/// Meters
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meter;
impl Unit for Meter {
    const ABBR: &'static str = "m";
}

/// Feet
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Foot;
impl Unit for Foot {
    const ABBR: &'static str = "ft";
}

/// Kilograms per meter squared (for BMI)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KgM2;
impl Unit for KgM2 {
    const ABBR: &'static str = "kg/m²";
}
