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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mgdl_glucose_conversion_uses_defined_factors() {
        let mgdl = 90.0;
        let mmol = MgdL::to_mmol_l(mgdl);
        assert!((mmol - mgdl * GLU_MGDL_TO_MMOLL).abs() < f64::EPSILON);

        let back_to_mgdl = MgdL::from_mmol_l(mmol);
        assert!((back_to_mgdl - mgdl).abs() < f64::EPSILON);
    }

    #[test]
    fn mmol_glucose_conversion_is_identity() {
        let mmol = 5.0;
        assert_eq!(MmolL::to_mmol_l(mmol), mmol);
        assert_eq!(MmolL::from_mmol_l(mmol), mmol);
    }
}
