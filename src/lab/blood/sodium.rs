//! Sodium (serum) module
//!
//! Chemistry 101: Sodium (Na+) is monovalent, making milliequivalents (mEq) and
//! millimoles (mmoplasma_na_ranges_are_correctl) the same. So no conversion factor is needed to go from SI to
//! conventional units in this case.

use std::marker::PhantomData;

use crate::{
    lab::{NumericRanged, RangeThreshold, ResultRange},
    units::{MeqL, MmolL, Unit},
};

const NA_SERUM_THRESHOLDS: RangeThreshold = RangeThreshold {
    crit_low: 130.0,
    low_norm: 135.0,
    norm_hi: 145.0,
    hi_crit: 150.0,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sodium<U: Unit> {
    value: f64,
    _ghost: PhantomData<U>,
}

impl<U: Unit> Sodium<U> {
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl<U: Unit> std::fmt::Display for Sodium<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Na ({:.0} {})", self.value, U::ABBR)
    }
}

pub trait SerumSodiumExt {
    fn na_serum_meq(self) -> Sodium<MeqL>;
    fn na_serum_mmol(self) -> Sodium<MmolL>;
}

impl SerumSodiumExt for f64 {
    fn na_serum_meq(self) -> Sodium<MeqL> {
        Sodium::from(self)
    }
    fn na_serum_mmol(self) -> Sodium<MmolL> {
        Sodium::from(self)
    }
}

// Conventional Units (mEq / L)

impl NumericRanged<MeqL> for Sodium<MeqL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        match self.value {
            val if val < NA_SERUM_THRESHOLDS.crit_low => ResultRange::CriticalLow,
            val if val < NA_SERUM_THRESHOLDS.low_norm => ResultRange::Low,
            val if val < NA_SERUM_THRESHOLDS.norm_hi => ResultRange::Normal,
            val if val < NA_SERUM_THRESHOLDS.hi_crit => ResultRange::High,
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

impl From<Sodium<MmolL>> for Sodium<MeqL> {
    fn from(sodium: Sodium<MmolL>) -> Self {
        Self {
            value: sodium.value(),
            _ghost: PhantomData,
        }
    }
}

// SI units

impl NumericRanged<MmolL> for Sodium<MmolL> {
    fn value(&self) -> f64 {
        self.value
    }

    fn range(&self) -> ResultRange {
        match self.value {
            val if val < NA_SERUM_THRESHOLDS.crit_low => ResultRange::CriticalLow,
            val if val < NA_SERUM_THRESHOLDS.low_norm => ResultRange::Low,
            val if val < NA_SERUM_THRESHOLDS.norm_hi => ResultRange::Normal,
            val if val < NA_SERUM_THRESHOLDS.hi_crit => ResultRange::High,
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

impl From<Sodium<MeqL>> for Sodium<MmolL> {
    fn from(sodium: Sodium<MeqL>) -> Self {
        Self {
            value: sodium.value(),
            _ghost: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serum_na_ranges_are_correct() {
        assert_eq!(110.0.na_serum_meq().range(), ResultRange::CriticalLow);
        assert_eq!(130.0.na_serum_mmol().range(), ResultRange::Low);
        assert_eq!(135.0.na_serum_meq().range(), ResultRange::Normal);
        assert_eq!(144.9.na_serum_mmol().range(), ResultRange::Normal);
        assert_eq!(148.0.na_serum_meq().range(), ResultRange::High);
        assert_eq!(155.0.na_serum_mmol().range(), ResultRange::CriticalHigh);
    }
}
