use super::{MeqL, MmolL, Unit};

/// Describes a sodium measurement that can be converted to and from mmol/L units.
pub trait SodiumUnit: Unit {
    fn to_mmol_l(val: f64) -> f64;
    fn from_mmol_l(val: f64) -> f64;
}
impl SodiumUnit for MeqL {
    fn from_mmol_l(val: f64) -> f64 {
        val
    }
    fn to_mmol_l(val: f64) -> f64 {
        val
    }
}
impl SodiumUnit for MmolL {
    fn from_mmol_l(val: f64) -> f64 {
        val
    }
    fn to_mmol_l(val: f64) -> f64 {
        val
    }
}
