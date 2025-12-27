pub mod blood;

/// Trait shared by numeric lab values with defined normal and abnormal ranges
pub trait NumericRanged {
    /// Obtain the numeric lab result.
    fn value(&self) -> f64;
    /// Get a descriptive category (high/low/critical) for a numeric lab result.
    fn range(&self) -> ResultRange;
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
