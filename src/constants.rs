//! Constants module
//!
//! A central place for constants used in medical calculations.

/// Multiply by this factor to convert glucose mmol/L to mg/dL.
pub const GLU_MMOLL_TO_MGDL: f64 = 18.0;

/// Multiply by this factor to convert glucose mg/dL to mmol/L.
pub const GLU_MGDL_TO_MMOLL: f64 = 1.0 / GLU_MMOLL_TO_MGDL;

/// Multiply by this factor to convert creatinine mg/dL to umol/L
pub const SCR_MGDL_TO_UMOLL: f64 = 88.4;
