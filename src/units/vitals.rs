use crate::{
    constants::{FT_TO_M, KG_TO_LB, LB_TO_KG, M_TO_FT},
    units::{Foot, Kg, Lb, Meter},
};

use super::Unit;

/*
 *      Weight Units
 */

pub trait WeightUnit: Unit {
    fn to_kg(val: f64) -> f64;
    fn from_kg(val: f64) -> f64;
}
impl WeightUnit for Kg {
    fn to_kg(val: f64) -> f64 {
        val
    }
    fn from_kg(val: f64) -> f64 {
        val
    }
}
impl WeightUnit for Lb {
    fn from_kg(val: f64) -> f64 {
        val * KG_TO_LB
    }
    fn to_kg(val: f64) -> f64 {
        val * LB_TO_KG
    }
}

//
//      Height Units
//

pub trait HeightUnit: Unit {
    fn from_m(val: f64) -> f64;
    fn to_m(val: f64) -> f64;
}
impl HeightUnit for Meter {
    fn from_m(val: f64) -> f64 {
        val
    }
    fn to_m(val: f64) -> f64 {
        val
    }
}
impl HeightUnit for Foot {
    fn from_m(val: f64) -> f64 {
        val * M_TO_FT
    }
    fn to_m(val: f64) -> f64 {
        val * FT_TO_M
    }
}
