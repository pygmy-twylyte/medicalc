//! CHA₂DS₂-VASc Score Calculator
//!
//! For estimating stroke risk in atrial fibrillation. Takes gender into acccount where
//! CHADS-VA does not.
//!

use crate::history::{Gender, Years};

/// CHA₂DS₂-VASc to annual stroke risk table from Friberg (2012)
const ANNUAL_CVA_RISK_TABLE: [f64; 10] = [0.2, 0.6, 2.2, 3.2, 4.8, 7.2, 9.7, 11.2, 10.8, 12.2];

/// A CHA₂DS₂-VASc calculator.
#[derive(Debug, Clone, PartialEq)]
pub struct ChadsVasc {
    age: Years,
    sex: Gender,
    chf: bool,
    diabetes: bool,
    htn: bool,
    stroke: bool,
    vasc: bool,
    score: Option<u8>,
}
impl ChadsVasc /* builder / setters */ {
    pub fn new(age: Years, sex: Gender) -> Self {
        Self {
            age,
            sex,
            chf: false,
            diabetes: false,
            htn: false,
            stroke: false,
            vasc: false,
            score: None,
        }
    }
    pub fn has_chf(mut self) -> Self {
        self.chf = true;
        self
    }
    pub fn has_diabetes(mut self) -> Self {
        self.diabetes = true;
        self
    }
    pub fn has_htn(mut self) -> Self {
        self.htn = true;
        self
    }
    pub fn has_stroke_hx(mut self) -> Self {
        self.stroke = true;
        self
    }
    pub fn has_vascular_hx(mut self) -> Self {
        self.vasc = true;
        self
    }
}

impl ChadsVasc /* getters */ {
    pub fn age(&self) -> Years {
        self.age
    }
    pub fn gender(&self) -> Gender {
        self.sex
    }
    pub fn chf(&self) -> bool {
        self.chf
    }
    pub fn diabetes(&self) -> bool {
        self.diabetes
    }
    pub fn htn(&self) -> bool {
        self.htn
    }
    pub fn stroke(&self) -> bool {
        self.stroke
    }
    pub fn vasc(&self) -> bool {
        self.vasc
    }
    pub fn score(&self) -> Option<u8> {
        self.score
    }
}

impl ChadsVasc /* calculations */ {
    #[must_use]
    pub fn calculate(mut self) -> Self {
        let mut tally = match self.age.0 {
            age if age >= 75.0 => 2,
            age if age >= 65.0 => 1,
            _ => 0,
        };
        tally += if self.sex == Gender::Female { 1 } else { 0 };
        tally += [self.chf, self.diabetes, self.htn, self.vasc]
            .iter()
            .filter(|&rf| *rf)
            .count() as u8;
        tally += if self.stroke { 2 } else { 0 };
        self.score = Some(tally);
        self
    }

    pub fn annual_stroke_risk_pct(&self) -> Option<f64> {
        if let Some(score) = self.score {
            Some(ANNUAL_CVA_RISK_TABLE[score as usize])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        calculators::cha2ds2_vasc::ChadsVasc,
        history::{Gender, Years},
    };

    #[test]
    fn score_is_none_until_calculate_is_run() {
        let chads_vasc = ChadsVasc::new(Years(50.0), Gender::Male);
        assert!(chads_vasc.score().is_none());
        assert!(chads_vasc.calculate().score().is_some());
    }

    #[test]
    fn stroke_risk_pct_returns_none_until_calculate_run() {
        let chads_vasc = ChadsVasc::new(Years(50.0), Gender::Male);
        assert!(chads_vasc.annual_stroke_risk_pct().is_none());
        assert!(chads_vasc.calculate().annual_stroke_risk_pct().is_some());
    }

    #[test]
    fn healthy_male_under_65_scores_zero() {
        let chads_vasc = ChadsVasc::new(Years(64.0), Gender::Male).calculate();
        assert_eq!(Some(0), chads_vasc.score());
    }

    #[test]
    fn healthy_female_under_65_scores_one() {
        let chads_vasc = ChadsVasc::new(Years(64.9), Gender::Female).calculate();
        assert_eq!(Some(1), chads_vasc.score());
    }

    #[test]
    fn maximum_risk_factors_scores_nine() {
        let chads_vasc = ChadsVasc::new(Years(75.0), Gender::Female)
            .has_chf()
            .has_diabetes()
            .has_htn()
            .has_stroke_hx()
            .has_vascular_hx()
            .calculate();
        assert_eq!(Some(9), chads_vasc.score())
    }
}
