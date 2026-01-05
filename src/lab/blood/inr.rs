//! INR module
//!
//! INR is a unitless measure, a ratio of a a sample's prothrombin time to that
//! of a control / normal. Still, we define an InrUnit for this in the units module
//! so it's consistent with the form of other measured lab values that do have units.

use std::marker::PhantomData;

use crate::units::{InrUnit, Unit};

/// An INR (international normalized ration for prothrombin time) measurement.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Inr<U: Unit> {
    value: f64,
    _unit: PhantomData<U>,
}

//
//      Getter / Display impls
//
impl<U: Unit> Inr<U> {
    /// Returns the numeric value of the INR measurement.
    pub fn value(&self) -> f64 {
        self.value
    }
}
impl<U: Unit> std::fmt::Display for Inr<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INR ({})", self.value)
    }
}

//
//      InrExt to make INR directly from f64
//
pub trait InrExt {
    fn inr(self) -> Inr<InrUnit>;
}
impl InrExt for f64 {
    fn inr(self) -> Inr<InrUnit> {
        Inr {
            value: self,
            _unit: PhantomData,
        }
    }
}

//
//      From impls
//
impl<U: Unit> From<f64> for Inr<U> {
    fn from(value: f64) -> Self {
        Inr {
            value,
            _unit: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inr_construction_from_f64() {
        let inr_value = 1.5.inr();
        assert_eq!(inr_value.value(), 1.5);
    }

    #[test]
    fn inr_from_trait() {
        let inr_value: Inr<InrUnit> = Inr::from(2.0);
        assert_eq!(inr_value.value(), 2.0);
    }

    #[test]
    fn inr_display_format() {
        let inr_value = 1.8.inr();
        let display_string = format!("{}", inr_value);
        assert!(display_string.contains("1.8"));
        assert!(display_string.contains("INR"));
    }

    #[test]
    fn inr_normal_value() {
        let inr_normal = 1.0.inr();
        assert_eq!(inr_normal.value(), 1.0);
    }

    #[test]
    fn inr_therapeutic_range() {
        // Typical therapeutic range for warfarin is 2.0-3.0
        let inr_low = 2.0.inr();
        let inr_high = 3.0.inr();

        assert_eq!(inr_low.value(), 2.0);
        assert_eq!(inr_high.value(), 3.0);
    }

    #[test]
    fn inr_elevated() {
        let inr_elevated = 5.0.inr();
        assert_eq!(inr_elevated.value(), 5.0);
        assert!(inr_elevated.value() > 3.0); // Above therapeutic range
    }

    #[test]
    fn inr_subtherapeutic() {
        let inr_low = 1.5.inr();
        assert_eq!(inr_low.value(), 1.5);
        assert!(inr_low.value() < 2.0); // Below therapeutic range
    }

    #[test]
    fn inr_fractional_values() {
        let inr = 2.37.inr();
        assert_eq!(inr.value(), 2.37);
    }

    #[test]
    fn inr_copy_clone() {
        let inr1 = 2.5.inr();
        let inr2 = inr1; // Copy
        let inr3 = inr1.clone(); // Clone

        assert_eq!(inr1.value(), inr2.value());
        assert_eq!(inr1.value(), inr3.value());
    }

    #[test]
    fn inr_debug_format() {
        let inr = 1.9.inr();
        let debug_string = format!("{:?}", inr);
        assert!(debug_string.contains("Inr"));
    }

    #[test]
    fn inr_equality() {
        let inr1 = 2.2.inr();
        let inr2 = 2.2.inr();
        let inr3 = 2.3.inr();

        assert_eq!(inr1, inr2);
        assert_ne!(inr1, inr3);
    }
}
