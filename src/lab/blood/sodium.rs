//! Sodium (plasma) module
//!
//! Chemistry 101: Sodium (Na+) is monovalent, making milliequivalents (mEq) and
//! millimoles (mmol) the same. So no conversion factor is needed to go from SI to
//! conventional units in this case.

use std::marker::PhantomData;

use crate::{
    lab::{NumericRanged, RangeThreshold, ResultRange},
    units::{MeqL, MmolL, Unit},
};

const NA_PLASMA_THRESHOLDS: RangeThreshold = RangeThreshold {
    crit_low: 130.0,
    low_norm: 135.0,
    norm_hi: 145.0,
    hi_crit: 150.0,
};

pub struct Sodium<U: Unit> {
    value: f64,
    _ghost: PhantomData<U>,
}

pub trait PlasmaSodiumExt {
    fn na_plasma_meq(self) -> Sodium<MeqL>;
    fn na_plasma_mmol(self) -> Sodium<MmolL>;
}

impl PlasmaSodiumExt for f64 {
    fn na_plasma_meq(self) -> Sodium<MeqL> {
        Sodium::from(self)
    }
    fn na_plasma_mmol(self) -> Sodium<MmolL> {
        Sodium::from(self)
    }
}

// Conventional Units (mEq / L)

impl NumericRanged for Sodium<MeqL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        match self.value {
            val if val < NA_PLASMA_THRESHOLDS.crit_low => ResultRange::CriticalLow,
            val if val < NA_PLASMA_THRESHOLDS.low_norm => ResultRange::Low,
            val if val < NA_PLASMA_THRESHOLDS.norm_hi => ResultRange::Normal,
            val if val < NA_PLASMA_THRESHOLDS.hi_crit => ResultRange::High,
            _ => ResultRange::CriticalHigh,
        }
    }
}

impl From<f64> for Sodium<MeqL> {
    fn from(value: f64) -> Self {
        Sodium {
            value,
            _ghost: PhantomData,
        }
    }
}

// SI units

impl NumericRanged for Sodium<MmolL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        match self.value {
            val if val < NA_PLASMA_THRESHOLDS.crit_low => ResultRange::CriticalLow,
            val if val < NA_PLASMA_THRESHOLDS.low_norm => ResultRange::Low,
            val if val < NA_PLASMA_THRESHOLDS.norm_hi => ResultRange::Normal,
            val if val < NA_PLASMA_THRESHOLDS.hi_crit => ResultRange::High,
            _ => ResultRange::CriticalHigh,
        }
    }
}

impl From<f64> for Sodium<MmolL> {
    fn from(value: f64) -> Self {
        Sodium {
            value,
            _ghost: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plasma_na_ranges_are_correct() {
        assert_eq!(110.0.na_plasma_meq().range(), ResultRange::CriticalLow);
        assert_eq!(130.0.na_plasma_mmol().range(), ResultRange::Low);
        assert_eq!(135.0.na_plasma_meq().range(), ResultRange::Normal);
        assert_eq!(144.9.na_plasma_mmol().range(), ResultRange::Normal);
        assert_eq!(148.0.na_plasma_meq().range(), ResultRange::High);
        assert_eq!(155.0.na_plasma_mmol().range(), ResultRange::CriticalHigh);
    }
}
