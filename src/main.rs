use medicalc::calculators::bmi;
use medicalc::calculators::cha2ds2_va::Cha2Ds2VA;
use medicalc::calculators::cha2ds2_vasc::ChadsVasc;
use medicalc::history::{Gender, Years};
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

    println!("\nCHA₂DS₂-VA(Sc) tests:\n");
    let chads_vasc = ChadsVasc::new(Years(68.0), Gender::Female)
        .has_diabetes()
        .has_htn()
        .calculate();
    let chads_va = Cha2Ds2VA::from(chads_vasc).calculate();

    println!("Annual stroke risks:");
    println!(
        "Without AC:\n\t{:.1}% CHA₂DS₂-VASc\n\t{:.1}% CHA₂DS₂-VA\n",
        chads_vasc.annual_stroke_risk_pct().unwrap(),
        chads_va.annual_cva_risk_no_oac().unwrap()
    );
    println!(
        "{:.1}% with AC",
        chads_va.annual_cva_risk_with_oac().unwrap()
    );
}
