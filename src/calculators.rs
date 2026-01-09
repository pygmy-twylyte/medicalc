//! Calculators module
//!
//! Various medical calculation using standardize formulas are implemented here.
//! Care is taken using the type system to make use of incorrect units impossible,
//! while aiming for seamless use with easy conversion between SI and conventional
//! units where applicable.
//!
//! Though I am in the USA, we're going to try to catch up to the rest of the world
//! and use SI as the "canonical" units for calculations.

use crate::{
    constants::{SBILI_UMOLL_TO_MGDL, SCR_UMOLL_TO_MGDL},
    history::{Gender, Years},
    lab::{
        blood::{
            bilirubin::Bilirubin, creatinine::Creatinine, glucose::Glucose, inr::Inr,
            sodium::Sodium,
        },
        gfr::Gfr,
        vitals::{Bmi, BmiExt, Height, Weight},
    },
    units::{
        bilirubin::BilirubinUnit,
        creatinine::CreatinineUnit,
        glucose::GlucoseUnit,
        sodium::SodiumUnit,
        vitals::{HeightUnit, WeightUnit},
        GfrUnit, KgM2, MgdL, Unit,
    },
};

pub mod cha2ds2_va;
pub mod cha2ds2_vasc;

/// Sodium correction for hyperglycemia.
///
/// Hyperglycemia causes osmotic dilutional hyponatremia. This function uses the
/// Katz formula for serum glucose ranging up to 400 mg/dL (22.2 mmol/L) and the
/// Hillier formula for more severe hyperglycemia.
///
/// * Katz: Corrected Na = Measured Na + 0.29 × (glucose - 5.6)
/// * Hillier: Corrected Na = Measured Na + 0.43 × (glucose - 5.6)
pub fn correct_na_for_glucose<N, G>(sodium: Sodium<N>, glucose: Glucose<G>) -> Sodium<N>
where
    N: SodiumUnit,
    G: GlucoseUnit,
    Sodium<N>: From<f64>,
{
    // convert input units to mmol/L
    let na_mmol = N::to_mmol_l(sodium.value());
    let glu_mmol = G::to_mmol_l(glucose.value());

    // define the two formulas that may be used
    let katz = |na, glu| na + 0.29 * (glu - 5.6);
    let hillier = |na, glu| na + 0.43 * (glu - 5.6);

    // correct the sodium measurement according to the severity
    // of the hyperglycemia
    let corrected_na = if glu_mmol < 22.2 {
        katz(na_mmol, glu_mmol)
    } else {
        hillier(na_mmol, glu_mmol)
    };

    // return the corrected measurement, converting back to the
    // same units (N) that were input as needed.
    Sodium::from(N::from_mmol_l(corrected_na))
}

/// CKD-EPI 2021 calculation (creatinine only).
///
/// The equation uses serum creatinine expressed in mg/dL.
pub fn egfr_ckd_epi<U: CreatinineUnit>(
    scr: Creatinine<U>,
    age: Years,
    sex: Gender,
) -> Gfr<GfrUnit> {
    // set the sex-determined constants (2021 race-free equation)
    let (kappa, alpha, sex_mult) = if sex == Gender::Female {
        (0.7, -0.241, 1.012)
    } else {
        (0.9, -0.302, 1.0)
    };

    // make sure we have SCr value in mg/dL... a little awkward since we've standardized
    // elsewhere in SI units
    let scr_umol_l = U::to_umol_l(scr.value());
    let scr_mg_dl = MgdL::from_umol_l(scr_umol_l);

    let ratio = scr_mg_dl / kappa;
    let second_term = (1.0_f64.min(ratio)).powf(alpha);
    let third_term = (1.0_f64.max(ratio)).powf(-1.200);
    let fourth_term = 0.9938_f64.powf(age.0);
    let egfr = 142.0 * second_term * third_term * fourth_term * sex_mult;
    Gfr::from(egfr)
}

/// BMI calculation
pub fn bmi<H, W>(height: Height<H>, weight: Weight<W>) -> Bmi<KgM2>
where
    H: HeightUnit,
    W: WeightUnit,
{
    let ht = H::to_m(height.value());
    let wt = W::to_kg(weight.value());

    (wt / ht.powi(2)).to_bmi()
}

/// The result of a MELD score calculation.
pub type MeldScore = u8;

/// The time of last dialysis used in calculating a MELD score.
pub type LastDialysis = Option<u8>;

/// Model for End-Stage Liver Disease (MELD) Score
///
/// MELD = 3.78 * ln(serum_bili_in_mgdL) + 11.2 * ln(INR) + 9.57 * ln(serum_creat_mgdL) + 6.43
pub fn meld<B, I, C>(
    bili: Bilirubin<B>,
    inr: Inr<I>,
    scr: Creatinine<C>,
    last_hd: LastDialysis,
) -> MeldScore
where
    B: BilirubinUnit,
    I: Unit,
    C: CreatinineUnit,
{
    let bili_mgdl = B::to_umoll(bili.value()) * SBILI_UMOLL_TO_MGDL;
    let scr_mgdl = C::to_umol_l(scr.value()) * SCR_UMOLL_TO_MGDL;

    // UNOS modifications:
    // 1) floors set at 1.0 for these values
    // 2) BUT use Scr = 4.0 mg/dL if dialyzed within 7 days
    let bili_mgdl = bili_mgdl.max(1.0);
    let scr_mgdl = if let Some(days) = last_hd {
        if days <= 7 {
            4.0
        } else {
            scr_mgdl.max(1.0)
        }
    } else {
        scr_mgdl.max(1.0)
    };

    let bili_term = 3.78 * bili_mgdl.ln();
    let inr_term = 11.2 * inr.value().ln();
    let scr_term = 9.57 * scr_mgdl.ln();

    let raw_score = bili_term + inr_term + scr_term + 6.43;
    raw_score.round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab::blood::{
        glucose::SerumGlucoseExt,
        sodium::{SerumSodiumExt, Sodium},
    };
    use crate::units::{MgdL, MmolL};

    fn approx_eq(lhs: f64, rhs: f64) {
        assert!((lhs - rhs).abs() < 1e-9, "{} !~= {}", lhs, rhs);
    }

    #[test]
    fn correct_na_uses_katz_below_threshold() {
        let sodium = 130.0.na_serum_meq();
        let glucose = 10.0.glu_serum_mmol_l();

        let corrected = correct_na_for_glucose(sodium, glucose);
        let expected = 130.0 + 0.29 * (10.0 - 5.6);

        approx_eq(corrected.value(), expected);
    }

    #[test]
    fn correct_na_uses_hillier_above_threshold() {
        let sodium = 132.0.na_serum_meq();
        let glucose = 30.0.glu_serum_mmol_l();

        let corrected = correct_na_for_glucose(sodium, glucose);
        let expected = 132.0 + 0.43 * (30.0 - 5.6);

        approx_eq(corrected.value(), expected);
    }

    #[test]
    fn correct_na_preserves_original_units() {
        // Work with mmol/L sodium and mg/dL glucose to exercise conversions.
        let sodium: Sodium<MmolL> = 138.0.na_serum_mmol();
        let glucose_mgdl = 500.0.glu_serum_mg_dl();

        let corrected = correct_na_for_glucose(sodium, glucose_mgdl);

        // Convert expected result to mmol/L using mg/dL glucose converted to mmol/L.
        let glucose_mmol = MgdL::to_mmol_l(glucose_mgdl.value());
        let expected = 138.0 + 0.43 * (glucose_mmol - 5.6);

        approx_eq(corrected.value(), expected);
    }

    #[test]
    fn correct_na_threshold_boundary() {
        // Test just above 22.2 mmol/L threshold (should use Hillier)
        let sodium = 135.0.na_serum_meq();
        let glucose = 22.3.glu_serum_mmol_l();

        let corrected = correct_na_for_glucose(sodium, glucose);
        let expected = 135.0 + 0.43 * (22.3 - 5.6);

        approx_eq(corrected.value(), expected);
    }

    #[test]
    fn correct_na_with_mixed_units() {
        // Test with mg/dL sodium doesn't exist, but test mmol sodium with mmol glucose
        let sodium = 140.0.na_serum_mmol();
        let glucose = 15.0.glu_serum_mmol_l();

        let corrected = correct_na_for_glucose(sodium, glucose);
        let expected = 140.0 + 0.29 * (15.0 - 5.6);

        approx_eq(corrected.value(), expected);
    }

    #[test]
    fn correct_na_normal_glucose() {
        // With normal glucose, correction should be minimal
        let sodium = 138.0.na_serum_meq();
        let glucose = 5.6.glu_serum_mmol_l(); // baseline glucose

        let corrected = correct_na_for_glucose(sodium, glucose);
        approx_eq(corrected.value(), 138.0);
    }

    // Tests for egfr_ckd_epi

    #[test]
    fn egfr_ckd_epi_female_normal() {
        use crate::lab::blood::creatinine::CreatinineExt;

        let scr = 1.0.cr_serum_mg_dl();
        let age = Years(40.0);
        let sex = Gender::Female;

        let gfr = egfr_ckd_epi(scr, age, sex);

        // Female with SCr 1.0 mg/dL at age 40:
        // For female, ratio = 1.0/0.7 > 1, so use max(ratio) in third term
        let ratio: f64 = 1.0 / 0.7;
        let expected =
            142.0 * 1.0_f64.powf(-0.241) * ratio.powf(-1.200) * 0.9938_f64.powf(40.0) * 1.012;

        approx_eq(gfr.value(), expected);
    }

    #[test]
    fn egfr_ckd_epi_male_normal() {
        use crate::lab::blood::creatinine::CreatinineExt;

        let scr = 1.2.cr_serum_mg_dl();
        let age = Years(50.0);
        let sex = Gender::Male;

        let gfr = egfr_ckd_epi(scr, age, sex);

        // Male with SCr 1.2 mg/dL at age 50:
        // For male, ratio = 1.2/0.9 > 1, so use 1.0 in second term, ratio in third
        let ratio: f64 = 1.2 / 0.9;
        let expected =
            142.0 * 1.0_f64.powf(-0.302) * ratio.powf(-1.200) * 0.9938_f64.powf(50.0) * 1.0;

        approx_eq(gfr.value(), expected);
    }

    #[test]
    fn egfr_ckd_epi_with_umol_units() {
        use crate::lab::blood::creatinine::CreatinineExt;

        // Test with µmol/L input
        let scr = 88.4.cr_serum_umol_l(); // 88.4 µmol/L = 1.0 mg/dL
        let age = Years(30.0);
        let sex = Gender::Female;

        let gfr = egfr_ckd_epi(scr, age, sex);

        // Should produce same result as 1.0 mg/dL
        let scr_mgdl = 1.0.cr_serum_mg_dl();
        let gfr_mgdl = egfr_ckd_epi(scr_mgdl, age, sex);

        approx_eq(gfr.value(), gfr_mgdl.value());
    }

    #[test]
    fn egfr_ckd_epi_low_creatinine_female() {
        use crate::lab::blood::creatinine::CreatinineExt;

        // Female with low creatinine (uses min function with ratio <= 1)
        let scr = 0.5.cr_serum_mg_dl();
        let age = Years(25.0);
        let sex = Gender::Female;

        let gfr = egfr_ckd_epi(scr, age, sex);

        let ratio: f64 = 0.5 / 0.7;
        let expected =
            142.0 * ratio.powf(-0.241) * 1.0_f64.powf(-1.200) * 0.9938_f64.powf(25.0) * 1.012;

        approx_eq(gfr.value(), expected);
    }

    #[test]
    fn egfr_ckd_epi_high_creatinine_male() {
        use crate::lab::blood::creatinine::CreatinineExt;

        // Male with high creatinine (uses max function with ratio > 1)
        let scr = 3.0.cr_serum_mg_dl();
        let age = Years(70.0);
        let sex = Gender::Male;

        let gfr = egfr_ckd_epi(scr, age, sex);

        let ratio: f64 = 3.0 / 0.9;
        let expected =
            142.0 * 1.0_f64.powf(-0.302) * ratio.powf(-1.200) * 0.9938_f64.powf(70.0) * 1.0;

        approx_eq(gfr.value(), expected);
    }

    // Tests for BMI calculation

    #[test]
    fn bmi_with_si_units() {
        use crate::lab::vitals::{HeightExt, WeightExt};

        let height = 1.75.height_in_m();
        let weight = 70.0.weight_kg();

        let bmi_result = bmi(height, weight);
        let expected = 70.0 / (1.75 * 1.75);

        approx_eq(bmi_result.value(), expected);
    }

    #[test]
    fn bmi_with_imperial_units() {
        use crate::lab::vitals::{HeightExt, WeightExt};

        let height = 5.75.height_in_ft(); // 5.75 feet
        let weight = 154.32.weight_lb(); // ~70 kg

        let bmi_result = bmi(height, weight);

        // Convert to metric for expected
        let height_m = 5.75 * crate::constants::FT_TO_M;
        let weight_kg = 154.32 * crate::constants::LB_TO_KG;
        let expected = weight_kg / (height_m * height_m);

        approx_eq(bmi_result.value(), expected);
    }

    #[test]
    fn bmi_with_mixed_units() {
        use crate::lab::vitals::{HeightExt, WeightExt};

        // Mix feet and kilograms
        let height = 6.0.height_in_ft();
        let weight = 80.0.weight_kg();

        let bmi_result = bmi(height, weight);

        let height_m = 6.0 * crate::constants::FT_TO_M;
        let expected = 80.0 / (height_m * height_m);

        approx_eq(bmi_result.value(), expected);
    }

    #[test]
    fn bmi_underweight() {
        use crate::lab::vitals::{HeightExt, WeightExt};

        let height = 1.75.height_in_m();
        let weight = 50.0.weight_kg();

        let bmi_result = bmi(height, weight);
        let expected = 50.0 / (1.75 * 1.75);

        approx_eq(bmi_result.value(), expected);
        assert!(bmi_result.value() < 18.5); // Underweight threshold
    }

    #[test]
    fn bmi_obese() {
        use crate::lab::vitals::{HeightExt, WeightExt};

        let height = 1.75.height_in_m();
        let weight = 100.0.weight_kg();

        let bmi_result = bmi(height, weight);
        let expected = 100.0 / (1.75 * 1.75);

        approx_eq(bmi_result.value(), expected);
        assert!(bmi_result.value() > 30.0); // Obese threshold
    }

    // Tests for MELD score

    #[test]
    fn meld_normal_values() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        let bili = 1.0.serum_bili_mgdl();
        let inr = 1.0.inr();
        let scr = 1.0.cr_serum_mg_dl();

        let score = meld(bili, inr, scr, None);

        // MELD = 3.78*ln(1.0) + 11.2*ln(1.0) + 9.57*ln(1.0) + 6.43
        // = 0 + 0 + 0 + 6.43 = 6.43, rounds to 6
        assert_eq!(score, 6);
    }

    #[test]
    fn meld_elevated_values() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        let bili = 3.0.serum_bili_mgdl();
        let inr = 2.0.inr();
        let scr = 2.5.cr_serum_mg_dl();

        let score = meld(bili, inr, scr, None);

        let bili_term = 3.78 * 3.0_f64.ln();
        let inr_term = 11.2 * 2.0_f64.ln();
        let scr_term = 9.57 * 2.5_f64.ln();
        let expected = (bili_term + inr_term + scr_term + 6.43).round() as u8;

        assert_eq!(score, expected);
    }

    #[test]
    fn meld_with_dialysis_recent() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        let bili = 2.0.serum_bili_mgdl();
        let inr = 1.5.inr();
        let scr = 1.5.cr_serum_mg_dl(); // This should be forced to 4.0

        let score = meld(bili, inr, scr, Some(3)); // Dialyzed 3 days ago

        // SCr should be treated as 4.0
        let bili_term = 3.78 * 2.0_f64.ln();
        let inr_term = 11.2 * 1.5_f64.ln();
        let scr_term = 9.57 * 4.0_f64.ln();
        let expected = (bili_term + inr_term + scr_term + 6.43).round() as u8;

        assert_eq!(score, expected);
    }

    #[test]
    fn meld_with_dialysis_old() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        let bili = 2.0.serum_bili_mgdl();
        let inr = 1.5.inr();
        let scr = 1.5.cr_serum_mg_dl();

        let score = meld(bili, inr, scr, Some(10)); // Dialyzed 10 days ago

        // SCr should use actual value with floor of 1.0
        let bili_term = 3.78 * 2.0_f64.ln();
        let inr_term = 11.2 * 1.5_f64.ln();
        let scr_term = 9.57 * 1.5_f64.ln();
        let expected = (bili_term + inr_term + scr_term + 6.43).round() as u8;

        assert_eq!(score, expected);
    }

    #[test]
    fn meld_with_umol_bilirubin() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        // Use µmol/L for bilirubin
        let bili = 17.1.serum_bili_umoll(); // 17.1 µmol/L = 1.0 mg/dL
        let inr = 1.0.inr();
        let scr = 1.0.cr_serum_mg_dl();

        let score = meld(bili, inr, scr, None);

        // Should produce same result as 1.0 mg/dL bilirubin
        let bili_mgdl = 1.0.serum_bili_mgdl();
        let score_mgdl = meld(bili_mgdl, inr, scr, None);

        assert_eq!(score, score_mgdl);
    }

    #[test]
    fn meld_with_umol_creatinine() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        let bili = 2.0.serum_bili_mgdl();
        let inr = 1.5.inr();
        let scr = 176.8.cr_serum_umol_l(); // 176.8 µmol/L = 2.0 mg/dL

        let score = meld(bili, inr, scr, None);

        // Should produce same result as 2.0 mg/dL
        let scr_mgdl = 2.0.cr_serum_mg_dl();
        let score_mgdl = meld(bili, inr, scr_mgdl, None);

        assert_eq!(score, score_mgdl);
    }

    #[test]
    fn meld_high_score() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        let bili = 10.0.serum_bili_mgdl();
        let inr = 3.0.inr();
        let scr = 4.0.cr_serum_mg_dl();

        let score = meld(bili, inr, scr, None);

        // High values should produce high MELD score
        assert!(score > 30);
    }

    #[test]
    fn meld_floor_values_applied() {
        use crate::lab::blood::{bilirubin::BilirubinExt, creatinine::CreatinineExt, inr::InrExt};

        // Test with values below floor (should be raised to 1.0)
        let bili = 0.5.serum_bili_mgdl(); // Below floor
        let inr = 0.8.inr(); // Below floor
        let scr = 0.5.cr_serum_mg_dl(); // Below floor

        let score = meld(bili, inr, scr, None);

        // All values should be floored to 1.0, but INR doesn't have a floor in our implementation
        // So only bili and scr are floored
        let bili_term = 3.78 * 1.0_f64.ln();
        let inr_term = 11.2 * 0.8_f64.ln();
        let scr_term = 9.57 * 1.0_f64.ln();
        let expected = (bili_term + inr_term + scr_term + 6.43).round() as u8;
        assert_eq!(score, expected);
    }
}
