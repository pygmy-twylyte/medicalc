//! BilirubinUnit module
//!
//! Describes units typically used in reporting serum bilirubin values and conversions
//! between them (µmol/L and mg/dL)
//!
//!

use crate::{
    constants::{SBILI_MGDL_TO_UMOLL, SBILI_UMOLL_TO_MGDL},
    units::{MgdL, UmolL},
};

use super::Unit;

pub trait BilirubinUnit: Unit {
    fn to_umoll(value: f64) -> f64;
    fn from_umoll(value: f64) -> f64;
}

impl BilirubinUnit for MgdL {
    fn to_umoll(value: f64) -> f64 {
        value * SBILI_MGDL_TO_UMOLL
    }

    fn from_umoll(value: f64) -> f64 {
        value * SBILI_UMOLL_TO_MGDL
    }
}

impl BilirubinUnit for UmolL {
    fn to_umoll(value: f64) -> f64 {
        value
    }

    fn from_umoll(value: f64) -> f64 {
        value
    }
}
