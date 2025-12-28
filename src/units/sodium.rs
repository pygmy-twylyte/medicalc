use super::{MeqL, MmolL, Unit};

/// Describes a sodium measurement that can be converted to and from mmol/L units.
pub trait SodiumUnit: Unit {
    fn to_mmol_l(val: f64) -> f64;
    fn from_mmol_l(val: f64) -> f64;
}
impl SodiumUnit for MeqL {
    fn from_mmol_l(val: f64) -> f64 {
        val
    }
    fn to_mmol_l(val: f64) -> f64 {
        val
    }
}
impl SodiumUnit for MmolL {
    fn from_mmol_l(val: f64) -> f64 {
        val
    }
    fn to_mmol_l(val: f64) -> f64 {
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sodium_meq_and_mmol_are_equivalent() {
        let value = 138.0;
        assert_eq!(MeqL::to_mmol_l(value), value);
        assert_eq!(MeqL::from_mmol_l(value), value);

        assert_eq!(MmolL::to_mmol_l(value), value);
        assert_eq!(MmolL::from_mmol_l(value), value);
    }
}
