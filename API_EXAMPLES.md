# Medicalc API Usage Examples

This document provides practical examples of using the medicalc library for medical calculations.

## Table of Contents
- [Sodium Correction for Hyperglycemia](#sodium-correction-for-hyperglycemia)
- [eGFR Calculation (CKD-EPI 2021)](#egfr-calculation-ckd-epi-2021)
- [BMI Calculation](#bmi-calculation)
- [MELD Score](#meld-score)
- [Working with Units](#working-with-units)

## Sodium Correction for Hyperglycemia

Correct sodium for hyperglycemia using either Katz (glucose < 400 mg/dL) or Hillier (glucose ≥ 400 mg/dL) formulas.

```rust
use medicalc::calculators::correct_na_for_glucose;
use medicalc::lab::blood::{sodium::SerumSodiumExt, glucose::SerumGlucoseExt};

// Example 1: Using SI units (mmol/L)
let sodium = 130.0.na_serum_mmol();
let glucose = 25.0.glu_serum_mmol_l();
let corrected = correct_na_for_glucose(sodium, glucose);
println!("Corrected sodium: {}", corrected); // Returns in mmol/L

// Example 2: Using conventional units (mEq/L and mg/dL)
let sodium = 132.0.na_serum_meq();
let glucose = 450.0.glu_serum_mg_dl();
let corrected = correct_na_for_glucose(sodium, glucose);
println!("Corrected sodium: {}", corrected); // Returns in mEq/L

// Example 3: Mixing units (library handles conversion)
let sodium = 135.0.na_serum_mmol();
let glucose = 400.0.glu_serum_mg_dl(); // mg/dL glucose
let corrected = correct_na_for_glucose(sodium, glucose);
println!("Corrected: {}", corrected); // Returns in mmol/L (same as input sodium)
```

**Key Points:**
- Return value has the same units as the input sodium
- Library automatically converts glucose to mmol/L for calculation
- Uses Katz formula for glucose < 22.2 mmol/L (400 mg/dL)
- Uses Hillier formula for glucose ≥ 22.2 mmol/L

## eGFR Calculation (CKD-EPI 2021)

Calculate estimated glomerular filtration rate using the race-free CKD-EPI 2021 equation.

```rust
use medicalc::calculators::egfr_ckd_epi;
use medicalc::lab::blood::creatinine::CreatinineExt;
use medicalc::history::{Gender, Years};

// Example 1: Female patient with mg/dL creatinine
let scr = 1.2.cr_serum_mg_dl();
let age = Years(65.0);
let sex = Gender::Female;
let gfr = egfr_ckd_epi(scr, age, sex);
println!("eGFR: {} mL/min/1.73m²", gfr.value());

// Example 2: Male patient with µmol/L creatinine
let scr = 150.0.cr_serum_umol_l(); // ~1.7 mg/dL
let age = Years(45.0);
let sex = Gender::Male;
let gfr = egfr_ckd_epi(scr, age, sex);
println!("eGFR: {}", gfr);

// Example 3: Interpret GFR stage
let gfr_value = gfr.value();
let stage = match gfr_value {
    x if x >= 90.0 => "Normal (Stage 1 if kidney damage present)",
    x if x >= 60.0 => "Stage 2: Mildly decreased",
    x if x >= 45.0 => "Stage 3a: Mild to moderate",
    x if x >= 30.0 => "Stage 3b: Moderate to severe",
    x if x >= 15.0 => "Stage 4: Severely decreased",
    _ => "Stage 5: Kidney failure",
};
println!("CKD Stage: {}", stage);
```

**Key Points:**
- Uses CKD-EPI 2021 race-free equation
- Accepts creatinine in mg/dL or µmol/L
- Returns GFR in standard units (mL/min/1.73m²)
- Age should be in years as `Years(f64)`

## BMI Calculation

Calculate Body Mass Index from height and weight.

```rust
use medicalc::calculators::bmi;
use medicalc::lab::vitals::{Height, HeightExt, WeightExt};

// Example 1: SI units (meters and kilograms)
let height = 1.75.height_in_m();
let weight = 70.0.weight_kg();
let bmi_result = bmi(height, weight);
println!("BMI: {} kg/m²", bmi_result.value());

// Example 2: Imperial units (feet and pounds)
let height = 5.9.height_in_ft(); // 5.9 feet
let weight = 154.0.weight_lb();
let bmi_result = bmi(height, weight);
println!("BMI: {}", bmi_result);

// Example 3: Mixed units (feet and kilograms)
let height = 6.0.height_in_ft();
let weight = 85.0.weight_kg();
let bmi_result = bmi(height, weight);
println!("BMI: {}", bmi_result);

// Example 4: Using feet and inches
let height = Height::from_ft_and_in(5, 10.0); // 5 feet 10 inches
let weight = 165.0.weight_lb();
let bmi_result = bmi(height, weight);

// Interpret BMI
let category = match bmi_result.value() {
    x if x < 18.5 => "Underweight",
    x if x < 25.0 => "Normal weight",
    x if x < 30.0 => "Overweight",
    x if x < 35.0 => "Class I Obesity",
    x if x < 40.0 => "Class II Obesity",
    _ => "Class III Obesity",
};
println!("Category: {}", category);
```

**Key Points:**
- Accepts any combination of height units (meters/feet) and weight units (kg/lb)
- Returns BMI in kg/m² (standard)
- `from_ft_and_in()` method for common US format

## MELD Score

Calculate Model for End-Stage Liver Disease score.

```rust
use medicalc::calculators::meld;
use medicalc::lab::blood::{
    bilirubin::BilirubinExt,
    creatinine::CreatinineExt,
    inr::InrExt,
};

// Example 1: Basic MELD calculation (mg/dL units)
let bili = 3.0.serum_bili_mgdl();
let inr = 2.0.inr();
let scr = 2.0.cr_serum_mg_dl();
let score = meld(bili, inr, scr, None);
println!("MELD score: {}", score);

// Example 2: With recent dialysis (forces SCr to 4.0)
let bili = 2.5.serum_bili_mgdl();
let inr = 1.8.inr();
let scr = 1.5.cr_serum_mg_dl(); // Will be treated as 4.0
let days_since_dialysis = Some(3); // Dialyzed 3 days ago
let score = meld(bili, inr, scr, days_since_dialysis);
println!("MELD score (with dialysis): {}", score);

// Example 3: Using SI units
let bili = 51.3.serum_bili_umoll(); // 51.3 µmol/L ≈ 3.0 mg/dL
let inr = 2.0.inr();
let scr = 176.8.cr_serum_umol_l(); // 176.8 µmol/L ≈ 2.0 mg/dL
let score = meld(bili, inr, scr, None);
println!("MELD score: {}", score);

// Example 4: Interpreting MELD score
let urgency = match score {
    x if x < 10 => "Low mortality risk (~2% 3-month)",
    x if x < 20 => "Moderate risk (~6-20% 3-month)",
    x if x < 30 => "High risk (~20-50% 3-month)",
    x if x < 40 => "Very high risk (~50-80% 3-month)",
    _ => "Extremely high risk (>80% 3-month)",
};
println!("Prognosis: {}", urgency);
```

**Key Points:**
- Returns MELD score as `u8` (rounded integer)
- UNOS modifications applied:
  - Floor values of 1.0 for bilirubin and creatinine
  - SCr = 4.0 if dialyzed within 7 days
- Accepts bilirubin in mg/dL or µmol/L
- Accepts creatinine in mg/dL or µmol/L

## Working with Units

### Unit Conversions

The library automatically handles unit conversions, but you can also convert explicitly:

```rust
use medicalc::lab::blood::glucose::Glucose;
use medicalc::units::{MgdL, MmolL};

// Create in one unit
let glucose_mgdl = 90.0.glu_serum_mg_dl();

// Convert to another unit
let glucose_mmol: Glucose<MmolL> = Glucose::from(glucose_mgdl);
println!("{} mg/dL = {} mmol/L", glucose_mgdl.value(), glucose_mmol.value());
// Output: 90 mg/dL = 5.0 mmol/L
```

### Checking Result Ranges

All lab values implement the `NumericRanged` trait:

```rust
use medicalc::lab::{NumericRanged, ResultRange};

let sodium = 128.0.na_serum_meq();
match sodium.range() {
    ResultRange::CriticalLow => println!("CRITICAL: Severe hyponatremia"),
    ResultRange::Low => println!("Mild hyponatremia"),
    ResultRange::Normal => println!("Normal sodium"),
    ResultRange::High => println!("Mild hypernatremia"),
    ResultRange::CriticalHigh => println!("CRITICAL: Severe hypernatremia"),
}

// Access value and units
println!("Value: {}", sodium.value());
println!("Units: {}", sodium.units());
```

### Display Formatting

All types implement `Display`:

```rust
let height = 1.75.height_in_m();
let weight = 70.0.weight_kg();
let bmi_result = bmi(height, weight);

println!("{}", height);     // "Height (1.8 m)"
println!("{}", weight);     // "Weight (70.0 kg)"
println!("{}", bmi_result); // "BMI (22.9 kg/m²)"
```

### Type Safety Example

The type system prevents unit confusion at compile time:

```rust
// This works - both parameters have correct types
let corrected = correct_na_for_glucose(sodium, glucose);

// This won't compile - can't pass a creatinine where glucose is expected
// let corrected = correct_na_for_glucose(sodium, creatinine); // ❌ Compile error

// This works - library handles unit conversion internally
let sodium_meq = 130.0.na_serum_meq();
let glucose_mmol = 10.0.glu_serum_mmol_l();
let corrected = correct_na_for_glucose(sodium_meq, glucose_mmol);
// Returns Sodium<MeqL> - same unit as input sodium
```

## Conversion Factors Reference

| Measurement | Conversion |
|------------|-----------|
| Glucose | 1 mmol/L = 18.0 mg/dL |
| Creatinine | 1 mg/dL = 88.4 µmol/L |
| Bilirubin | 1 mg/dL = 17.1 µmol/L |
| Sodium | 1 mEq/L = 1 mmol/L (monovalent) |
| Weight | 1 kg = 2.20462 lb |
| Height | 1 m = 3.28084 ft |

## Error Handling

Currently, the library doesn't perform bounds checking. Consider validating inputs:

```rust
fn calculate_with_validation(scr_value: f64, age_value: f64) -> Option<Gfr<GfrUnit>> {
    // Validate inputs
    if scr_value <= 0.0 || age_value <= 0.0 || age_value > 120.0 {
        return None;
    }
    
    let scr = scr_value.cr_serum_mg_dl();
    let age = Years(age_value);
    let sex = Gender::Male;
    
    Some(egfr_ckd_epi(scr, age, sex))
}
```

## Best Practices

1. **Use extension traits** for ergonomic construction: `.na_serum_meq()`, `.glu_serum_mg_dl()`, etc.
2. **Let the type system work for you** - it prevents unit confusion
3. **Display output uses appropriate precision** - `format!("{}", value)` for user display
4. **Access raw values** with `.value()` when you need the f64
5. **Check result ranges** with `.range()` for clinical decision support
6. **Mix units freely** - the library handles conversions automatically

## Complete Example: Patient Workup

```rust
use medicalc::calculators::{egfr_ckd_epi, bmi};
use medicalc::lab::blood::creatinine::CreatinineExt;
use medicalc::lab::vitals::{HeightExt, WeightExt};
use medicalc::history::{Gender, Years};

fn patient_workup() {
    // Patient demographics
    let age = Years(68.0);
    let sex = Gender::Female;
    
    // Vital signs
    let height = 1.63.height_in_m();
    let weight = 75.0.weight_kg();
    
    // Lab values
    let scr = 1.3.cr_serum_mg_dl();
    
    // Calculate metrics
    let bmi_result = bmi(height, weight);
    let gfr = egfr_ckd_epi(scr, age, sex);
    
    // Report
    println!("Patient Assessment:");
    println!("  Age: {:.0} years, Sex: {:?}", age.0, sex);
    println!("  {}", height);
    println!("  {}", weight);
    println!("  {}", bmi_result);
    println!("  {}", scr);
    println!("  {}", gfr);
    
    // Clinical interpretation
    if bmi_result.value() >= 30.0 {
        println!("  ⚠ BMI indicates obesity");
    }
    if gfr.value() < 60.0 {
        println!("  ⚠ eGFR indicates CKD stage 3 or higher");
    }
}
```

## Further Reading

- See `TEST_SUMMARY.md` for comprehensive test coverage details
- Check individual module documentation for detailed formula references
- Consult source code comments for clinical references and rationale