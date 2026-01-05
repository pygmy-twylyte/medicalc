//! Constants module
//!
//! A central place for constants used in medical calculations.

/// Multiply by this factor to convert glucose mmol/L to mg/dL.
pub const GLU_MMOLL_TO_MGDL: f64 = 18.0;

/// Multiply by this factor to convert glucose mg/dL to mmol/L.
pub const GLU_MGDL_TO_MMOLL: f64 = GLU_MMOLL_TO_MGDL.recip();

/// Multiply by this factor to convert creatinine mg/dL to umol/L
pub const SCR_MGDL_TO_UMOLL: f64 = 88.4;

/// Multiply by this factor to convert creatinine umol/L
pub const SCR_UMOLL_TO_MGDL: f64 = SCR_MGDL_TO_UMOLL.recip();

/// Multiply by this factor to convert pounds to kilograms.
pub const LB_TO_KG: f64 = 0.45359237;

/// Multiply by this factor to convert kilograms to pounds.
pub const KG_TO_LB: f64 = LB_TO_KG.recip();

/// Multiply by this factor to convert feet to meters.
pub const FT_TO_M: f64 = 0.3048;

/// Multiply by this factor to convert meters to feet.
pub const M_TO_FT: f64 = FT_TO_M.recip();

/// Multiply by this factor to convert mg/dL bilirubin to µmol/L
pub const SBILI_MGDL_TO_UMOLL: f64 = 17.1;

/// Multiply by this factor to convert µmol/L bilirubin to mg/dL
pub const SBILI_UMOLL_TO_MGDL: f64 = SBILI_MGDL_TO_UMOLL.recip();
