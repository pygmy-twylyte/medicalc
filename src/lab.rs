use crate::units::Unit;

pub mod blood;

/// Trait shared by numeric lab values with defined normal and abnormal ranges
pub trait NumericRanged<U: Unit> {
    /// Obtain the numeric lab result.
    fn value(&self) -> f64;
    /// Get a descriptive category (high/low/critical) for a numeric lab result.
    fn range(&self) -> ResultRange;
    /// Get the units for this value
    fn units(&self) -> &'static str {
        U::ABBR
    }
}

/// Describes possible ranges for numeric results.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResultRange {
    CriticalLow,
    Low,
    Normal,
    High,
    CriticalHigh,
}

/// Holds range thresholds for numeric results.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeThreshold {
    pub crit_low: f64,
    pub low_norm: f64,
    pub norm_hi: f64,
    pub hi_crit: f64,
}

/// Determine an named range (e.g. normal or critical high) for a given value.
pub fn select_range(value: f64, thresholds: &RangeThreshold) -> ResultRange {
    match value {
        val if val <= thresholds.crit_low => ResultRange::CriticalLow,
        val if val <= thresholds.low_norm => ResultRange::Low,
        val if val <= thresholds.norm_hi => ResultRange::Normal,
        val if val <= thresholds.hi_crit => ResultRange::High,
        _ => ResultRange::CriticalHigh,
    }
}
