use medicalc::calculators::bmi;
use medicalc::lab::vitals::{Height, Weight, WeightExt};
use medicalc::units::{Foot, Kg, Lb, Meter};

fn main() {
    println!("DEVELOPMENT TESTING");
    println!("Not part of the library.");

    let input_wt = 202.0.weight_lb();
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
}
