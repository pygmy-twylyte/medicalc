//! (e)GFR module
//!
//! GFR is pretty universally reported in mL/min/1.73 m² -- a unit used
//! for nothing else.

use crate::units::{GfrUnit, Unit};
use std::marker::PhantomData;

/*
 * GFR type and inherent methods
 */

/// The result of a GFR calculation.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gfr<U: Unit> {
    value: f64,
    _ghost: PhantomData<U>,
}
impl<U: Unit> Gfr<U> {
    pub fn value(&self) -> f64 {
        self.value
    }
}

/*
 *  Extension trait to construct from an f64
 */
pub trait GfrExt {
    fn to_gfr(value: f64) -> Gfr<GfrUnit> {
        Gfr {
            value,
            _ghost: PhantomData,
        }
    }
}

/*
 * Trait impls
 */

impl<U: Unit> std::fmt::Display for Gfr<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GFR ({:.0} {})", self.value.round(), U::ABBR)
    }
}

impl From<f64> for Gfr<GfrUnit> {
    fn from(value: f64) -> Self {
        Gfr {
            value,
            _ghost: PhantomData,
        }
    }
}
