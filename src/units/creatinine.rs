use super::{MgdL, UmolL, Unit};
use crate::constants::SCR_MGDL_TO_UMOLL;

/// Describes methods for converting serum creatinine values from mg/dL to µmol/L and back.
///
/// Conversion factor: 1 mg/dL = 88.4 µmol/L
pub trait CreatinineUnit: Unit {
    fn to_umol_l(value: f64) -> f64;
    fn from_umol_l(value: f64) -> f64;
}
impl CreatinineUnit for MgdL {
    fn to_umol_l(value: f64) -> f64 {
        value * SCR_MGDL_TO_UMOLL
    }
    fn from_umol_l(value: f64) -> f64 {
        value / SCR_MGDL_TO_UMOLL
    }
}
impl CreatinineUnit for UmolL {
    fn to_umol_l(value: f64) -> f64 {
        value
    }
    fn from_umol_l(value: f64) -> f64 {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mgdl_conversions_scale_by_constant() {
        let mgdl_value = 1.3;
        let as_umol = MgdL::to_umol_l(mgdl_value);
        assert!((as_umol - mgdl_value * SCR_MGDL_TO_UMOLL).abs() < f64::EPSILON);

        let back_to_mgdl = MgdL::from_umol_l(as_umol);
        assert!((back_to_mgdl - mgdl_value).abs() < f64::EPSILON);
    }

    #[test]
    fn umol_conversions_are_identity() {
        let value = 123.4;
        assert_eq!(UmolL::to_umol_l(value), value);
        assert_eq!(UmolL::from_umol_l(value), value);
    }
}
