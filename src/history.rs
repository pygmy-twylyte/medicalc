//! History module
//!
//! This module contains types for historic factors that may come into play in various calculations,
//! Examples include age and gender.

/// Age in years.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Years(pub f64);

/// Closest physiologic gender.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Gender {
    Female,
    Male,
}
