use medicalc::lab::blood::glucose::SerumGlucoseExt;
use medicalc::lab::blood::sodium::SerumSodiumExt;

fn main() {
    println!("DEVELOPMENT TESTING");
    println!("Not part of the library.");

    println!("Sodium Corrections for Glucose");
    let na_measured = 126.0.na_serum_meq();

    let gluvals = vec![
        100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0,
    ];
    for val in &gluvals {
        let glucose = val.glu_serum_mg_dl();
        let na_corrected = medicalc::calculators::correct_na_for_glucose(na_measured, glucose);
        println!("{} -> {}", glucose, na_corrected);
    }

    // now try it with mixed SI and conventional units
    println!();
    println!("Now again, giving mixed SI and conventional units to the function...");
    let gluvals_mmol: Vec<_> = gluvals.iter().map(|mgdl| mgdl / 18.0).collect();
    for val in &gluvals_mmol {
        let glucose = val.glu_serum_mmol_l();
        let na_corrected = medicalc::calculators::correct_na_for_glucose(na_measured, glucose);
        println!("{} -> {}", glucose, na_corrected);
    }
}
