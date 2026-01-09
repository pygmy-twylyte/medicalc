use medicalc::calculators::bmi;
use medicalc::calculators::cha2ds2_va::Cha2Ds2VA;
use medicalc::history::Years;
use medicalc::lab::vitals::{Height, Weight, WeightExt};
use medicalc::units::{Foot, Kg, Lb, Meter};

fn main() {
    println!("DEVELOPMENT TESTING");
    println!("Not part of the library.");

    let input_wt = 201.8.weight_lb();
    let weight_lb: Weight<Lb> = Weight::from(input_wt); /* can convert to itself via core From impl */
    let weight_kg: Weight<Kg> = Weight::from(weight_lb);
    println!("{weight_lb} → {weight_kg}");

    let input_ht = Height::<Meter>::from_ft_and_in(5, 7.0);
    let height_ft: Height<Foot> = Height::from(input_ht);
    let height_m: Height<Meter> = Height::from(height_ft);
    println!("{height_ft} → {height_m}");

    // bmi calculator gets same results regardless of which combination of units we pass in
    println!("{}", bmi(height_ft, weight_lb));
    println!("{}", bmi(height_m, weight_kg));
    println!("{}", bmi(height_ft, weight_kg));
    println!("{}", bmi(height_m, weight_lb));

    println!("\nCHA₂DS₂-VA test:\n");
    let stroke_risk_calculator = Cha2Ds2VA::new(Years(56.5)).has_htn().calculate();
    println!("Annual stroke risks:");
    println!(
        "{:.1}% without AC",
        stroke_risk_calculator.annual_cva_risk_no_oac().unwrap()
    );
    println!(
        "{:.1}% with AC",
        stroke_risk_calculator.annual_cva_risk_with_oac().unwrap()
    );
}
