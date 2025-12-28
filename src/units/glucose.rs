use super::{MgdL, MmolL, Unit};
use crate::constants::{GLU_MGDL_TO_MMOLL, GLU_MMOLL_TO_MGDL};

/// Describes methods to convert glucose values to/from mmol/L for calculations.
pub trait GlucoseUnit: Unit {
    fn to_mmol_l(val: f64) -> f64;
    fn from_mmol_l(val: f64) -> f64;
}
impl GlucoseUnit for MgdL {
    fn to_mmol_l(val: f64) -> f64 {
        val * GLU_MGDL_TO_MMOLL
    }
    fn from_mmol_l(val: f64) -> f64 {
        val * GLU_MMOLL_TO_MGDL
    }
}
impl GlucoseUnit for MmolL {
    fn from_mmol_l(val: f64) -> f64 {
        val
    }
    fn to_mmol_l(val: f64) -> f64 {
        val
    }
}
