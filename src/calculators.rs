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
    history::{Gender, Years},
    lab::{
        blood::{creatinine::Creatinine, glucose::Glucose, sodium::Sodium},
        gfr::Gfr,
    },
    units::{GfrUnit, MgdL, creatinine::CreatinineUnit, glucose::GlucoseUnit, sodium::SodiumUnit},
};

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
    let second_term = (1.0f64.min(ratio)).powf(alpha);
    let third_term = (1.0f64.max(ratio)).powf(-1.200);
    let fourth_term = 0.9938f64.powf(age.0);
    let egfr = 142.0 * second_term * third_term * fourth_term * sex_mult;
    Gfr::from(egfr)
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
}
