//! Astronomy: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p astronomy-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&astronomy_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "schwarzschild_radius", "inputs": {"M": "1.988409871e+30 kg"}}));
    println!();
    show(json!({"key": "age_at_redshift", "inputs": {"z": "3", "H0": "70", "Om": "1"}}));
    println!();
    show(json!({"key": "absolute_magnitude", "inputs": {"m": "8", "d": "3.085677581e+18 m", "A": "1"}}));
    println!();
    show(json!({"key": "radial_velocity_semi_amplitude", "inputs": {"M_star": "1.988409871e+30 kg", "m_p": "1.898124597e+27 kg", "P": "11.86 yr"}}));
    println!();
    show(json!({"key": "einstein_radius", "inputs": {"M": "9.942049353e+29 kg", "D_l": "1.234271033e+20 m", "D_s": "2.468542065e+20 m"}}));
    println!();
    show(json!({"key": "airmass_extinction", "inputs": {"zen": "60 deg"}}));
    println!();
    show(json!({"key": "plane_change_delta_v", "inputs": {"v1": "3 km/s", "v2": "4 km/s", "di": "90 deg"}}));
    println!();
    show(json!({"key": "planet_equilibrium_temperature", "inputs": {"T_star": "1000 K", "R_star": "2 m", "a": "4 m"}}));
    println!();
    show(json!({"key": "alfven_speed", "inputs": {"B": "5 nT", "n": "5 cm^-3"}}));
    println!();
    show(json!({"key": "altaz_from_hadec", "inputs": {"H": "0 deg", "dec": "0 deg", "lat": "45 deg"}}));
    println!();
    show(json!({"key": "bolometric_magnitude_from_luminosity", "inputs": {"L": "3.828e+26 W"}}));
    println!();
    show(json!({"key": "rayleigh_resolution", "inputs": {"lam": "550 nm", "D": "2.4 m"}}));
    println!();
    show(json!({"key": "calendar_date_from_julian_date", "inputs": {"jd": "2451545"}}));
    println!();
    show(json!({"key": "angular_size_at_redshift", "inputs": {"z": "3", "H0": "70", "Om": "1", "D": "3.085677581e+22 m"}}));
    println!();
    show(json!({"key": "distance_unit_conversion", "inputs": {"d": "3.085677581e+16 m"}}));
}
