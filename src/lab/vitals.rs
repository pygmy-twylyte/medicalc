//! Vitals module
//!
//! Contains definitions for measurements of vital signs.

use std::marker::PhantomData;

use crate::{
    constants::{FT_TO_M, KG_TO_LB, LB_TO_KG, M_TO_FT},
    units::{Foot, Kg, KgM2, Lb, Meter, Unit},
};

/*
 *          Weight measurements
 */

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Weight<U: Unit> {
    value: f64,
    _ghost: PhantomData<U>,
}
impl<U: Unit> Weight<U> {
    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait WeightExt {
    fn weight_kg(self) -> Weight<Kg>;
    fn weight_lb(self) -> Weight<Lb>;
}
// construct weights from f64 vals
impl WeightExt for f64 {
    fn weight_kg(self) -> Weight<Kg> {
        Weight {
            value: self,
            _ghost: PhantomData,
        }
    }
    fn weight_lb(self) -> Weight<Lb> {
        Weight {
            value: self,
            _ghost: PhantomData,
        }
    }
}
// convert between weight units
impl From<Weight<Lb>> for Weight<Kg> {
    fn from(weight: Weight<Lb>) -> Self {
        Weight {
            value: weight.value * LB_TO_KG,
            _ghost: PhantomData,
        }
    }
}
impl From<Weight<Kg>> for Weight<Lb> {
    fn from(weight: Weight<Kg>) -> Self {
        Weight {
            value: weight.value * KG_TO_LB,
            _ghost: PhantomData,
        }
    }
}
// display impl
impl<U: Unit> std::fmt::Display for Weight<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Weight ({:.1} {})", self.value, U::ABBR)
    }
}

/*
 *      Height measurements
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height<U: Unit> {
    value: f64,
    _unit: PhantomData<U>,
}
impl<U: Unit> Height<U> {
    /// Get the raw value for the height.
    pub fn value(&self) -> f64 {
        self.value
    }
    /// Create a Height from feet and inches.
    pub fn from_ft_and_in(feet: u8, inches: f64) -> Height<Meter> {
        let total_ft = feet as f64 + inches / 12.0;
        Height {
            value: total_ft * FT_TO_M,
            _unit: PhantomData,
        }
    }
}
// construct from f64
pub trait HeightExt {
    fn height_in_m(self) -> Height<Meter>;
    fn height_in_ft(self) -> Height<Foot>;
}
impl HeightExt for f64 {
    fn height_in_ft(self) -> Height<Foot> {
        Height {
            value: self,
            _unit: PhantomData,
        }
    }
    fn height_in_m(self) -> Height<Meter> {
        Height {
            value: self,
            _unit: PhantomData,
        }
    }
}
// convert height between unit types
impl From<Height<Foot>> for Height<Meter> {
    fn from(other: Height<Foot>) -> Self {
        Height {
            value: other.value * FT_TO_M,
            _unit: PhantomData,
        }
    }
}
impl From<Height<Meter>> for Height<Foot> {
    fn from(other: Height<Meter>) -> Self {
        Height {
            value: other.value * M_TO_FT,
            _unit: PhantomData,
        }
    }
}
impl<U: Unit> std::fmt::Display for Height<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Height ({:.1} {})", self.value, U::ABBR)
    }
}

//
//      BMI Result / Value
//

pub struct Bmi<U: Unit> {
    value: f64,
    _units: PhantomData<U>,
}
impl<U: Unit> Bmi<U> {
    pub fn value(&self) -> f64 {
        self.value
    }
}
impl<U: Unit> std::fmt::Display for Bmi<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BMI ({:.1} {})", self.value, U::ABBR)
    }
}
pub trait BmiExt {
    fn to_bmi(self) -> Bmi<KgM2>;
}
impl BmiExt for f64 {
    fn to_bmi(self) -> Bmi<KgM2> {
        Bmi {
            value: self,
            _units: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-6, "{} !~= {}", a, b);
    }

    // Weight tests

    #[test]
    fn weight_construction_kg() {
        let weight = 70.0.weight_kg();
        approx_eq(weight.value(), 70.0);
    }

    #[test]
    fn weight_construction_lb() {
        let weight = 154.0.weight_lb();
        approx_eq(weight.value(), 154.0);
    }

    #[test]
    fn weight_kg_to_lb_conversion() {
        let weight_kg = 70.0.weight_kg();
        let weight_lb: Weight<Lb> = Weight::from(weight_kg);

        approx_eq(weight_lb.value(), 70.0 * KG_TO_LB);
    }

    #[test]
    fn weight_lb_to_kg_conversion() {
        let weight_lb = 154.32.weight_lb();
        let weight_kg: Weight<Kg> = Weight::from(weight_lb);

        approx_eq(weight_kg.value(), 154.32 * LB_TO_KG);
    }

    #[test]
    fn weight_round_trip_conversion() {
        let original = 80.0.weight_kg();
        let as_lb: Weight<Lb> = Weight::from(original);
        let back_to_kg: Weight<Kg> = Weight::from(as_lb);

        approx_eq(back_to_kg.value(), original.value());
    }

    #[test]
    fn weight_display_format() {
        let weight = 75.5.weight_kg();
        let display_string = format!("{}", weight);
        assert!(display_string.contains("75.5"));
        assert!(display_string.contains("kg"));
    }

    #[test]
    fn weight_common_values() {
        // Test some common weight conversions
        let kg_100 = 100.0.weight_kg();
        let lb_220: Weight<Lb> = Weight::from(kg_100);
        approx_eq(lb_220.value(), 220.4622621849);

        let lb_150 = 150.0.weight_lb();
        let kg_68: Weight<Kg> = Weight::from(lb_150);
        approx_eq(kg_68.value(), 68.0388555);
    }

    // Height tests

    #[test]
    fn height_construction_meters() {
        let height = 1.75.height_in_m();
        approx_eq(height.value(), 1.75);
    }

    #[test]
    fn height_construction_feet() {
        let height = 5.75.height_in_ft();
        approx_eq(height.value(), 5.75);
    }

    #[test]
    fn height_from_feet_and_inches() {
        let height = Height::<Meter>::from_ft_and_in(5, 10.0);

        // 5 feet 10 inches = 5 + 10/12 = 5.8333... feet
        let expected_ft = 5.0 + 10.0 / 12.0;
        let expected_m = expected_ft * FT_TO_M;

        approx_eq(height.value(), expected_m);
    }

    #[test]
    fn height_meters_to_feet_conversion() {
        let height_m = 1.8.height_in_m();
        let height_ft: Height<Foot> = Height::from(height_m);

        approx_eq(height_ft.value(), 1.8 * M_TO_FT);
    }

    #[test]
    fn height_feet_to_meters_conversion() {
        let height_ft = 6.0.height_in_ft();
        let height_m: Height<Meter> = Height::from(height_ft);

        approx_eq(height_m.value(), 6.0 * FT_TO_M);
    }

    #[test]
    fn height_round_trip_conversion() {
        let original = 1.70.height_in_m();
        let as_feet: Height<Foot> = Height::from(original);
        let back_to_m: Height<Meter> = Height::from(as_feet);

        approx_eq(back_to_m.value(), original.value());
    }

    #[test]
    fn height_display_format() {
        let height = 1.75.height_in_m();
        let display_string = format!("{}", height);
        assert!(display_string.contains("1.8") || display_string.contains("1.7"));
        assert!(display_string.contains("m"));
    }

    #[test]
    fn height_common_values() {
        // 6 feet should be approximately 1.829 meters
        let ft_6 = 6.0.height_in_ft();
        let m_182: Height<Meter> = Height::from(ft_6);
        approx_eq(m_182.value(), 1.8288);

        // 1.5 meters should be approximately 4.921 feet
        let m_150 = 1.5.height_in_m();
        let ft_492: Height<Foot> = Height::from(m_150);
        approx_eq(ft_492.value(), 4.92126);
    }

    #[test]
    fn height_feet_and_inches_typical_values() {
        // Test 5'9" (average male height in US)
        let height = Height::<Meter>::from_ft_and_in(5, 9.0);
        approx_eq(height.value(), 1.7526);

        // Test 5'4" (average female height in US)
        let height2 = Height::<Meter>::from_ft_and_in(5, 4.0);
        approx_eq(height2.value(), 1.6256);
    }

    // BMI tests

    #[test]
    fn bmi_construction() {
        let bmi = 25.0.to_bmi();
        approx_eq(bmi.value(), 25.0);
    }

    #[test]
    fn bmi_display_format() {
        let bmi = 23.5.to_bmi();
        let display_string = format!("{}", bmi);
        assert!(display_string.contains("23.5"));
        assert!(display_string.contains("kg/m²"));
    }

    #[test]
    fn bmi_underweight_threshold() {
        let bmi = 17.5.to_bmi();
        assert!(bmi.value() < 18.5);
    }

    #[test]
    fn bmi_normal_range() {
        let bmi_low = 18.5.to_bmi();
        let bmi_high = 24.9.to_bmi();

        assert!(bmi_low.value() >= 18.5);
        assert!(bmi_low.value() < 25.0);
        assert!(bmi_high.value() >= 18.5);
        assert!(bmi_high.value() < 25.0);
    }

    #[test]
    fn bmi_overweight_threshold() {
        let bmi = 27.0.to_bmi();
        assert!(bmi.value() >= 25.0);
        assert!(bmi.value() < 30.0);
    }

    #[test]
    fn bmi_obese_threshold() {
        let bmi = 32.0.to_bmi();
        assert!(bmi.value() >= 30.0);
    }

    // Conversion constant tests

    #[test]
    fn conversion_constants_reciprocal_relationship() {
        approx_eq(LB_TO_KG * KG_TO_LB, 1.0);
        approx_eq(FT_TO_M * M_TO_FT, 1.0);
    }

    #[test]
    fn conversion_constant_values() {
        // Verify known conversion factors
        approx_eq(LB_TO_KG, 0.45359237);
        approx_eq(FT_TO_M, 0.3048);
    }
}
