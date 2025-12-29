//! Serum creatinine module
//!

use std::marker::PhantomData;

use crate::{
    constants::SCR_MGDL_TO_UMOLL,
    lab::{NumericRanged, RangeThreshold, ResultRange, select_range},
    units::{MgdL, UmolL, Unit},
};

/// Default thresholds for lab alert ranges for serum creatinine, in mg/dL.
const SCR_THRESHOLDS_MG_DL: RangeThreshold = RangeThreshold {
    crit_low: 0.6,
    low_norm: 0.9,
    norm_hi: 1.4,
    hi_crit: 3.0,
};

/// Default thresholds for lab alert ranges for serum creatinine, in µmol/L
const SCR_THRESHOLDS_UMOL_L: RangeThreshold = RangeThreshold {
    crit_low: SCR_THRESHOLDS_MG_DL.crit_low * SCR_MGDL_TO_UMOLL,
    low_norm: SCR_THRESHOLDS_MG_DL.low_norm * SCR_MGDL_TO_UMOLL,
    norm_hi: SCR_THRESHOLDS_MG_DL.norm_hi * SCR_MGDL_TO_UMOLL,
    hi_crit: SCR_THRESHOLDS_MG_DL.hi_crit * SCR_MGDL_TO_UMOLL,
};

/*
 *               Type and inherent methods
 */

/// A serum creatinine measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Creatinine<U: Unit> {
    value: f64,
    _ghost: PhantomData<U>,
}
impl<U: Unit> Creatinine<U> {
    pub fn value(&self) -> f64 {
        self.value
    }
}
impl<U: Unit> std::fmt::Display for Creatinine<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Creatinine ({:.1} {})", self.value, U::ABBR)
    }
}

/*
 *         Convenience constructors from f64 values
 */

/// Defines convenience constructors for serum creatinine measurements from f64 values.
pub trait CreatinineExt {
    fn cr_serum_mg_dl(self) -> Creatinine<MgdL>;
    fn cr_serum_umol_l(self) -> Creatinine<UmolL>;
}
impl CreatinineExt for f64 {
    fn cr_serum_mg_dl(self) -> Creatinine<MgdL> {
        Creatinine::from(self)
    }

    fn cr_serum_umol_l(self) -> Creatinine<UmolL> {
        Creatinine::from(self)
    }
}

/*
 *          FROM impls
 */

// creation from f64
impl From<f64> for Creatinine<MgdL> {
    fn from(value: f64) -> Self {
        Creatinine {
            value,
            _ghost: PhantomData,
        }
    }
}
impl From<f64> for Creatinine<UmolL> {
    fn from(value: f64) -> Self {
        Creatinine {
            value,
            _ghost: PhantomData,
        }
    }
}

// conversion between mg/dL and umol/L types
impl From<Creatinine<UmolL>> for Creatinine<MgdL> {
    fn from(scr: Creatinine<UmolL>) -> Self {
        Creatinine {
            value: scr.value / SCR_MGDL_TO_UMOLL,
            _ghost: PhantomData,
        }
    }
}
impl From<Creatinine<MgdL>> for Creatinine<UmolL> {
    fn from(scr: Creatinine<MgdL>) -> Self {
        Creatinine {
            value: scr.value * SCR_MGDL_TO_UMOLL,
            _ghost: PhantomData,
        }
    }
}

/*
 *      NumericRanged impls
 */

impl NumericRanged<MgdL> for Creatinine<MgdL> {
    fn value(&self) -> f64 {
        self.value()
    }

    fn range(&self) -> ResultRange {
        select_range(self.value, &SCR_THRESHOLDS_MG_DL)
    }
}
impl NumericRanged<UmolL> for Creatinine<UmolL> {
    fn value(&self) -> f64 {
        self.value()
    }

    fn range(&self) -> ResultRange {
        select_range(self.value, &SCR_THRESHOLDS_UMOL_L)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-6, "{} !~= {}", a, b);
    }

    #[test]
    fn creatinine_unit_conversions_round_trip() {
        let mg_dl = Creatinine::<MgdL>::from(1.2);
        let as_umol: Creatinine<UmolL> = Creatinine::from(mg_dl);
        approx_eq(as_umol.value(), 1.2 * SCR_MGDL_TO_UMOLL);

        let back_to_mg_dl: Creatinine<MgdL> = Creatinine::from(as_umol);
        approx_eq(back_to_mg_dl.value(), 1.2);
    }

    #[test]
    fn creatinine_ranges_in_mg_dl_are_selected_correctly() {
        assert_eq!(
            Creatinine::<MgdL>::from(0.4).range(),
            ResultRange::CriticalLow
        );
        assert_eq!(Creatinine::<MgdL>::from(0.8).range(), ResultRange::Low);
        assert_eq!(Creatinine::<MgdL>::from(1.1).range(), ResultRange::Normal);
        assert_eq!(Creatinine::<MgdL>::from(2.0).range(), ResultRange::High);
        assert_eq!(
            Creatinine::<MgdL>::from(4.0).range(),
            ResultRange::CriticalHigh
        );
    }

    #[test]
    fn creatinine_ranges_in_umol_l_match_thresholds() {
        let factor = SCR_MGDL_TO_UMOLL;
        assert_eq!(
            Creatinine::<UmolL>::from(0.5 * factor).range(),
            ResultRange::CriticalLow
        );
        assert_eq!(
            Creatinine::<UmolL>::from(0.8 * factor).range(),
            ResultRange::Low
        );
        assert_eq!(
            Creatinine::<UmolL>::from(1.1 * factor).range(),
            ResultRange::Normal
        );
        assert_eq!(
            Creatinine::<UmolL>::from(2.0 * factor).range(),
            ResultRange::High
        );
        assert_eq!(
            Creatinine::<UmolL>::from(4.0 * factor).range(),
            ResultRange::CriticalHigh
        );
    }
}
