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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-6, "{} !~= {}", a, b);
    }

    #[test]
    fn gfr_construction_from_f64() {
        let gfr: Gfr<GfrUnit> = Gfr::from(90.0);
        approx_eq(gfr.value(), 90.0);
    }

    #[test]
    fn gfr_value_getter() {
        let gfr: Gfr<GfrUnit> = Gfr::from(75.5);
        approx_eq(gfr.value(), 75.5);
    }

    #[test]
    fn gfr_display_format() {
        let gfr: Gfr<GfrUnit> = Gfr::from(85.7);
        let display_string = format!("{}", gfr);

        // Display should round to nearest integer
        assert!(display_string.contains("86"));
        assert!(display_string.contains("mL/min/1.73m²"));
    }

    #[test]
    fn gfr_display_rounds_correctly() {
        let gfr1: Gfr<GfrUnit> = Gfr::from(89.4);
        let display1 = format!("{}", gfr1);
        assert!(display1.contains("89"));

        let gfr2: Gfr<GfrUnit> = Gfr::from(89.5);
        let display2 = format!("{}", gfr2);
        assert!(display2.contains("90"));
    }

    #[test]
    fn gfr_normal_range() {
        // Normal GFR is > 90
        let gfr_normal: Gfr<GfrUnit> = Gfr::from(95.0);
        assert!(gfr_normal.value() > 90.0);
    }

    #[test]
    fn gfr_mild_reduction() {
        // Stage 2 CKD: 60-89
        let gfr: Gfr<GfrUnit> = Gfr::from(75.0);
        assert!(gfr.value() >= 60.0 && gfr.value() < 90.0);
    }

    #[test]
    fn gfr_moderate_reduction() {
        // Stage 3 CKD: 30-59
        let gfr: Gfr<GfrUnit> = Gfr::from(45.0);
        assert!(gfr.value() >= 30.0 && gfr.value() < 60.0);
    }

    #[test]
    fn gfr_severe_reduction() {
        // Stage 4 CKD: 15-29
        let gfr: Gfr<GfrUnit> = Gfr::from(20.0);
        assert!(gfr.value() >= 15.0 && gfr.value() < 30.0);
    }

    #[test]
    fn gfr_kidney_failure() {
        // Stage 5 CKD: < 15
        let gfr: Gfr<GfrUnit> = Gfr::from(10.0);
        assert!(gfr.value() < 15.0);
    }

    #[test]
    fn gfr_fractional_values() {
        let gfr: Gfr<GfrUnit> = Gfr::from(67.89);
        approx_eq(gfr.value(), 67.89);
    }

    #[test]
    fn gfr_high_values() {
        // Young healthy individuals can have GFR > 120
        let gfr: Gfr<GfrUnit> = Gfr::from(125.0);
        approx_eq(gfr.value(), 125.0);
    }

    #[test]
    fn gfr_low_values() {
        let gfr: Gfr<GfrUnit> = Gfr::from(5.0);
        approx_eq(gfr.value(), 5.0);
    }

    #[test]
    fn gfr_copy_trait() {
        let gfr1: Gfr<GfrUnit> = Gfr::from(80.0);
        let gfr2 = gfr1; // Copy

        approx_eq(gfr1.value(), 80.0);
        approx_eq(gfr2.value(), 80.0);
    }

    #[test]
    fn gfr_clone_trait() {
        let gfr1: Gfr<GfrUnit> = Gfr::from(70.0);
        let gfr2 = gfr1.clone();

        approx_eq(gfr1.value(), gfr2.value());
    }

    #[test]
    fn gfr_debug_format() {
        let gfr: Gfr<GfrUnit> = Gfr::from(88.0);
        let debug_string = format!("{:?}", gfr);
        assert!(debug_string.contains("Gfr"));
    }

    #[test]
    fn gfr_equality() {
        let gfr1: Gfr<GfrUnit> = Gfr::from(65.0);
        let gfr2: Gfr<GfrUnit> = Gfr::from(65.0);
        let gfr3: Gfr<GfrUnit> = Gfr::from(66.0);

        assert_eq!(gfr1, gfr2);
        assert_ne!(gfr1, gfr3);
    }
}
