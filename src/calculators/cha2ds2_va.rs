//! CHA₂DS₂-VA
//!
//! Stroke risk calculator in atrial fibrillation

// Let's see if scores like this work well in a builder-type pattern...

use crate::history::Years;

const CHADS_VA_STROKE_RISK_PCT_ANNUAL: [(f64, f64); 9] = [
    (0.5, 0.2),
    (1.5, 0.5),
    (2.9, 1.0),
    (5.1, 1.8),
    (7.3, 2.6),
    (11.2, 3.9),
    (15.5, 5.4),
    (14.7, 5.1),
    (19.5, 6.8),
];

#[derive(Debug, Clone, PartialEq)]
pub struct Cha2Ds2VA {
    age: Years,
    chf: bool,
    diabetes: bool,
    htn: bool,
    stroke: bool,
    vasc: bool,
    score: Option<u8>,
}
impl Cha2Ds2VA {
    pub fn new(age: Years) -> Self {
        Self {
            age,
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
    pub fn calculate(mut self) -> Self {
        let mut tally: u8 = match self.age {
            a if a.0 >= 75.0 => 2,
            a if a.0 >= 65.0 => 1,
            _ => 0,
        };
        tally = tally
            + [self.htn, self.chf, self.vasc, self.diabetes]
                .iter()
                .filter(|&p| *p)
                .count() as u8;
        if self.stroke {
            tally += 2;
        }
        self.score = Some(tally);
        self
    }
    pub fn score(&self) -> Option<u8> {
        self.score
    }
    pub fn annual_cva_risk_no_oac(&self) -> Option<f64> {
        if let Some(score) = self.score() {
            Some(CHADS_VA_STROKE_RISK_PCT_ANNUAL[score as usize].0)
        } else {
            None
        }
    }
    pub fn annual_cva_risk_with_oac(&self) -> Option<f64> {
        if let Some(score) = self.score() {
            Some(CHADS_VA_STROKE_RISK_PCT_ANNUAL[score as usize].1)
        } else {
            None
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn score_is_none_until_calculated() {
        let pre_calc = Cha2Ds2VA::new(Years(50.0));
        assert!(pre_calc.score().is_none());
        let calculated = pre_calc.calculate();
        assert!(calculated.score().is_some());
    }

    #[test]
    fn risks_are_none_until_calculated() {
        let pre_calc = Cha2Ds2VA::new(Years(50.0));
        assert!(pre_calc.annual_cva_risk_no_oac().is_none());
        assert!(pre_calc.annual_cva_risk_with_oac().is_none());
        let calculated = pre_calc.calculate();
        assert!(calculated.annual_cva_risk_no_oac().is_some());
        assert!(calculated.annual_cva_risk_with_oac().is_some());
    }

    #[test]
    fn healthy_under_65_scores_zero() {
        let cv_score = Cha2Ds2VA::new(Years(50.0)).calculate();
        assert_eq!(cv_score.score(), Some(0));
    }

    #[test]
    fn healthy_65_plus_scores_one() {
        let cv_score = Cha2Ds2VA::new(Years(65.0)).calculate();
        assert_eq!(Some(1), cv_score.score());
    }

    #[test]
    fn healthy_75_plus_scores_two() {
        let cv_score = Cha2Ds2VA::new(Years(75.0)).calculate();
        assert_eq!(Some(2), cv_score.score());
    }

    #[test]
    fn maximum_age_and_hx_scores_eight() {
        let cv_score = Cha2Ds2VA::new(Years(100.0))
            .has_chf()
            .has_diabetes()
            .has_htn()
            .has_stroke_hx()
            .has_vascular_hx()
            .calculate();
        assert_eq!(Some(8), cv_score.score());
    }
}
