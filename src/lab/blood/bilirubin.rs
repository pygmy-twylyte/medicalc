//! Serum bilirubin module

use std::marker::PhantomData;

use crate::constants::{SBILI_MGDL_TO_UMOLL, SBILI_UMOLL_TO_MGDL};
use crate::lab::{select_range, NumericRanged, RangeThreshold, ResultRange};
use crate::units::{MgdL, UmolL, Unit};

pub const SERUM_BILI_RANGES_MGDL: RangeThreshold = RangeThreshold {
    crit_low: 0.2,
    low_norm: 0.5,
    norm_hi: 2.5,
    hi_crit: 10.0,
};

pub const SERUM_BILI_RANGES_UMOLL: RangeThreshold = RangeThreshold {
    crit_low: SERUM_BILI_RANGES_MGDL.crit_low * SBILI_MGDL_TO_UMOLL,
    low_norm: SERUM_BILI_RANGES_MGDL.low_norm * SBILI_MGDL_TO_UMOLL,
    norm_hi: SERUM_BILI_RANGES_MGDL.norm_hi * SBILI_MGDL_TO_UMOLL,
    hi_crit: SERUM_BILI_RANGES_MGDL.hi_crit * SBILI_MGDL_TO_UMOLL,
};

/// A serum bilirubin measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bilirubin<U: Unit> {
    value: f64,
    _unit: PhantomData<U>,
}
impl<U: Unit> Bilirubin<U> {
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl From<Bilirubin<MgdL>> for Bilirubin<UmolL> {
    fn from(bili_mgdl: Bilirubin<MgdL>) -> Self {
        Bilirubin {
            value: bili_mgdl.value * SBILI_MGDL_TO_UMOLL,
            _unit: PhantomData,
        }
    }
}
impl From<Bilirubin<UmolL>> for Bilirubin<MgdL> {
    fn from(bili_umoll: Bilirubin<UmolL>) -> Self {
        Bilirubin {
            value: bili_umoll.value * SBILI_UMOLL_TO_MGDL,
            _unit: PhantomData,
        }
    }
}

impl<U: Unit> std::fmt::Display for Bilirubin<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bilirubin ({} {})", self.value, U::ABBR)
    }
}

pub trait BilirubinExt {
    fn serum_bili_umoll(self) -> Bilirubin<UmolL>;
    fn serum_bili_mgdl(self) -> Bilirubin<MgdL>;
}
impl BilirubinExt for f64 {
    fn serum_bili_umoll(self) -> Bilirubin<UmolL> {
        Bilirubin {
            value: self,
            _unit: PhantomData,
        }
    }

    fn serum_bili_mgdl(self) -> Bilirubin<MgdL> {
        Bilirubin {
            value: self,
            _unit: PhantomData,
        }
    }
}

impl NumericRanged<UmolL> for Bilirubin<UmolL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        select_range(self.value, &SERUM_BILI_RANGES_UMOLL)
    }
}

impl NumericRanged<MgdL> for Bilirubin<MgdL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        select_range(self.value, &SERUM_BILI_RANGES_MGDL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-6, "{} !~= {}", a, b);
    }

    #[test]
    fn bilirubin_construction_from_f64() {
        let bili_mgdl = 1.5.serum_bili_mgdl();
        let bili_umoll = 25.65.serum_bili_umoll();

        approx_eq(bili_mgdl.value(), 1.5);
        approx_eq(bili_umoll.value(), 25.65);
    }

    #[test]
    fn bilirubin_unit_conversions() {
        let bili_mgdl = 1.0.serum_bili_mgdl();
        let bili_umoll: Bilirubin<UmolL> = Bilirubin::from(bili_mgdl);

        // 1 mg/dL = 17.1 µmol/L
        approx_eq(bili_umoll.value(), 17.1);

        // Convert back
        let back_to_mgdl: Bilirubin<MgdL> = Bilirubin::from(bili_umoll);
        approx_eq(back_to_mgdl.value(), 1.0);
    }

    #[test]
    fn bilirubin_conversions_round_trip() {
        let original = 2.5.serum_bili_mgdl();
        let as_umoll: Bilirubin<UmolL> = Bilirubin::from(original);
        let back: Bilirubin<MgdL> = Bilirubin::from(as_umoll);

        approx_eq(back.value(), original.value());
    }

    #[test]
    fn bilirubin_ranges_mgdl() {
        assert_eq!(0.1.serum_bili_mgdl().range(), ResultRange::CriticalLow);
        assert_eq!(0.3.serum_bili_mgdl().range(), ResultRange::Low);
        assert_eq!(1.0.serum_bili_mgdl().range(), ResultRange::Normal);
        assert_eq!(5.0.serum_bili_mgdl().range(), ResultRange::High);
        assert_eq!(15.0.serum_bili_mgdl().range(), ResultRange::CriticalHigh);
    }

    #[test]
    fn bilirubin_ranges_umoll() {
        // Test with µmol/L values
        assert_eq!(
            (0.2 * SBILI_MGDL_TO_UMOLL * 0.5).serum_bili_umoll().range(),
            ResultRange::CriticalLow
        );
        assert_eq!(
            (0.5 * SBILI_MGDL_TO_UMOLL * 0.8).serum_bili_umoll().range(),
            ResultRange::Low
        );
        assert_eq!(
            (1.0 * SBILI_MGDL_TO_UMOLL).serum_bili_umoll().range(),
            ResultRange::Normal
        );
        assert_eq!(
            (5.0 * SBILI_MGDL_TO_UMOLL).serum_bili_umoll().range(),
            ResultRange::High
        );
        assert_eq!(
            (15.0 * SBILI_MGDL_TO_UMOLL).serum_bili_umoll().range(),
            ResultRange::CriticalHigh
        );
    }

    #[test]
    fn bilirubin_display_format() {
        let bili_mgdl = 2.3.serum_bili_mgdl();
        let display_string = format!("{}", bili_mgdl);
        assert!(display_string.contains("2.3"));
        assert!(display_string.contains("mg/dL"));
    }

    #[test]
    fn bilirubin_range_boundaries() {
        // Test values at boundaries - select_range uses <= comparisons
        // so a value exactly at a threshold belongs to the lower range
        let just_below_crit_low = (SERUM_BILI_RANGES_MGDL.crit_low - 0.01).serum_bili_mgdl();
        assert_eq!(just_below_crit_low.range(), ResultRange::CriticalLow);

        let just_above_crit_low = (SERUM_BILI_RANGES_MGDL.crit_low + 0.01).serum_bili_mgdl();
        assert_eq!(just_above_crit_low.range(), ResultRange::Low);

        let just_above_low_norm = (SERUM_BILI_RANGES_MGDL.low_norm + 0.01).serum_bili_mgdl();
        assert_eq!(just_above_low_norm.range(), ResultRange::Normal);

        let just_above_norm_hi = (SERUM_BILI_RANGES_MGDL.norm_hi + 0.01).serum_bili_mgdl();
        assert_eq!(just_above_norm_hi.range(), ResultRange::High);

        let just_above_hi_crit = (SERUM_BILI_RANGES_MGDL.hi_crit + 0.01).serum_bili_mgdl();
        assert_eq!(just_above_hi_crit.range(), ResultRange::CriticalHigh);
    }

    #[test]
    fn bilirubin_numeric_ranged_trait() {
        let bili = 1.5.serum_bili_mgdl();

        // Test NumericRanged trait methods
        assert_eq!(bili.value(), 1.5);
        assert_eq!(bili.units(), "mg/dL");
        assert_eq!(bili.range(), ResultRange::Normal);
    }

    #[test]
    fn bilirubin_conversion_factor_accuracy() {
        // Verify conversion factors match constants
        let mgdl_val = 1.0;
        let umoll_val = mgdl_val * SBILI_MGDL_TO_UMOLL;

        approx_eq(umoll_val, 17.1);
        approx_eq(umoll_val * SBILI_UMOLL_TO_MGDL, mgdl_val);
    }
}
