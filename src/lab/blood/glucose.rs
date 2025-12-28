//! Glucose Module
//!
//! SI units = mmol/L
//! Conventional units = mg/dL
//! 18 mg/dL glucose = 1 mmol/L glucose

use std::marker::PhantomData;

use crate::{
    constants::{GLU_MGDL_TO_MMOLL, GLU_MMOLL_TO_MGDL},
    lab::{NumericRanged, RangeThreshold, ResultRange},
    units::{MgdL, MmolL, Unit},
};

const GLU_SERUM_THRESHOLDS_MGDL: RangeThreshold = RangeThreshold {
    crit_low: 60.0,
    low_norm: 85.0,
    norm_hi: 125.0,
    hi_crit: 200.0,
};

const GLU_SERUM_THRESHOLDS_MMOLL: RangeThreshold = RangeThreshold {
    crit_low: GLU_SERUM_THRESHOLDS_MGDL.crit_low * GLU_MGDL_TO_MMOLL,
    low_norm: GLU_SERUM_THRESHOLDS_MGDL.low_norm * GLU_MGDL_TO_MMOLL,
    norm_hi: GLU_SERUM_THRESHOLDS_MGDL.norm_hi * GLU_MGDL_TO_MMOLL,
    hi_crit: GLU_SERUM_THRESHOLDS_MGDL.hi_crit * GLU_MGDL_TO_MMOLL,
};

//
// Type and inherent methods
//

/// A serum glucose measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Glucose<U: Unit> {
    value: f64,
    _ghost: PhantomData<U>,
}
impl<U: Unit> Glucose<U> {
    pub fn value(&self) -> f64 {
        self.value
    }
}
impl<U: Unit> std::fmt::Display for Glucose<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Glucose ({:.1} {})", self.value(), U::ABBR)
    }
}

//
// Convenience constructors from f64 values
//

/// Defines methods for creating `Glucose<U>` directly from f64 values.
pub trait SerumGlucoseExt {
    fn glu_serum_mg_dl(self) -> Glucose<MgdL>;
    fn glu_serum_mmol_l(self) -> Glucose<MmolL>;
}

impl SerumGlucoseExt for f64 {
    fn glu_serum_mg_dl(self) -> Glucose<MgdL> {
        Glucose::from(self)
    }
    fn glu_serum_mmol_l(self) -> Glucose<MmolL> {
        Glucose::from(self)
    }
}

//
// NumericRanged<U> impls
//

// conventional units
impl NumericRanged<MgdL> for Glucose<MgdL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        match self.value {
            val if val < GLU_SERUM_THRESHOLDS_MGDL.crit_low => ResultRange::CriticalLow,
            val if val < GLU_SERUM_THRESHOLDS_MGDL.low_norm => ResultRange::Low,
            val if val < GLU_SERUM_THRESHOLDS_MGDL.norm_hi => ResultRange::Normal,
            val if val < GLU_SERUM_THRESHOLDS_MGDL.hi_crit => ResultRange::High,
            _ => ResultRange::CriticalHigh,
        }
    }
}
// SI units
impl NumericRanged<MmolL> for Glucose<MmolL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        match self.value {
            val if val < GLU_SERUM_THRESHOLDS_MMOLL.crit_low => ResultRange::CriticalLow,
            val if val < GLU_SERUM_THRESHOLDS_MMOLL.low_norm => ResultRange::Low,
            val if val < GLU_SERUM_THRESHOLDS_MMOLL.norm_hi => ResultRange::Normal,
            val if val < GLU_SERUM_THRESHOLDS_MMOLL.hi_crit => ResultRange::High,
            _ => ResultRange::CriticalHigh,
        }
    }
}

//
// From impls
//

// construction from f64s
impl From<f64> for Glucose<MgdL> {
    fn from(value: f64) -> Self {
        Glucose {
            value,
            _ghost: PhantomData,
        }
    }
}
impl From<f64> for Glucose<MmolL> {
    fn from(value: f64) -> Self {
        Glucose {
            value,
            _ghost: PhantomData,
        }
    }
}

// conversions from one unit type to another
impl From<Glucose<MmolL>> for Glucose<MgdL> {
    fn from(glucose: Glucose<MmolL>) -> Self {
        Glucose {
            value: glucose.value() * GLU_MMOLL_TO_MGDL,
            _ghost: PhantomData,
        }
    }
}
impl From<Glucose<MgdL>> for Glucose<MmolL> {
    fn from(glucose: Glucose<MgdL>) -> Self {
        Glucose {
            value: glucose.value() * GLU_MGDL_TO_MMOLL,
            _ghost: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glucose_unit_conversions_are_correct() {
        let glucose_mmol = 1.0.glu_serum_mmol_l();
        let glucose_mgdl = 18.0.glu_serum_mg_dl();

        assert_eq!(glucose_mmol.value(), 1.0);
        assert_eq!(glucose_mgdl.value(), 18.0);

        let converted_mmol: Glucose<MmolL> = Glucose::from(glucose_mgdl);
        let converted_mgdl: Glucose<MgdL> = Glucose::from(glucose_mmol);

        assert_eq!(converted_mmol.value(), 1.0);
        assert_eq!(converted_mgdl.value(), 18.0);
    }

    #[test]
    fn glucose_result_ranges_correct_mmol_l() {
        assert_eq!(3.0.glu_serum_mmol_l().range(), ResultRange::CriticalLow);
        assert_eq!(4.0.glu_serum_mmol_l().range(), ResultRange::Low);
        assert_eq!(5.0.glu_serum_mmol_l().range(), ResultRange::Normal);
        assert_eq!(7.0.glu_serum_mmol_l().range(), ResultRange::High);
        assert_eq!(12.0.glu_serum_mmol_l().range(), ResultRange::CriticalHigh);
    }

    #[test]
    fn glucose_result_ranges_correct_mg_dl() {
        assert_eq!(50.0.glu_serum_mg_dl().range(), ResultRange::CriticalLow);
        assert_eq!(75.0.glu_serum_mg_dl().range(), ResultRange::Low);
        assert_eq!(100.0.glu_serum_mg_dl().range(), ResultRange::Normal);
        assert_eq!(125.0.glu_serum_mg_dl().range(), ResultRange::High);
        assert_eq!(300.0.glu_serum_mg_dl().range(), ResultRange::CriticalHigh);
    }
}
