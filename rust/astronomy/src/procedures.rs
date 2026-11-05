//! astronomy: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/astronomy/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "absolute_magnitude",
    "age_at_redshift",
    "airmass_extinction",
    "alfven_speed",
    "altaz_from_hadec",
    "angular_separation",
    "angular_size",
    "angular_size_at_redshift",
    "apparent_bolometric_magnitude",
    "apsis_speeds",
    "atmospheric_scale_height",
    "binary_star_masses",
    "bolometric_magnitude_from_luminosity",
    "calendar_date_from_julian_date",
    "camera_pixel_scale",
    "combined_magnitude",
    "comoving_volume",
    "cosmological_distances",
    "critical_density",
    "culmination_altitude",
    "debye_length",
    "debye_number",
    "distance_from_distance_modulus",
    "distance_from_flux",
    "distance_unit_conversion",
    "effective_temperature",
    "einstein_radius",
    "escape_delta_v",
    "flux_ratio_from_magnitudes",
    "free_fall_time",
    "gyrofrequency",
    "gyroradius",
    "hohmann_transfer",
    "inertial_length",
    "julian_date",
    "kepler_position_at_time",
    "light_gathering_power",
    "linear_size_from_angle",
    "lookback_time",
    "lower_hybrid_frequency",
    "luminosity_from_bolometric_magnitude",
    "magnetic_pressure",
    "magnitude_difference_from_flux_ratio",
    "orbit_from_state",
    "parallax_distance",
    "plane_change_delta_v",
    "planet_equilibrium_temperature",
    "plasma_beta",
    "plasma_frequency",
    "position_angle",
    "radial_velocity_semi_amplitude",
    "rayleigh_resolution",
    "recession_velocity_from_redshift",
    "redshift_at_lookback_time",
    "redshift_from_wavelengths",
    "rise_set_hour_angle",
    "saha_hydrogen_ionization",
    "schwarzschild_radius",
    "sidereal_time",
    "stellar_luminosity",
    "stellar_radius",
    "telescope_magnification",
    "thermal_speed",
    "time_of_flight",
    "transit_depth",
    "transit_duration",
    "upper_hybrid_frequency",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "absolute_magnitude" => {
            let r = f_absolute_magnitude(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_15(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "age_at_redshift" => {
            let r = f_age_at_z(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Om") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_51(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "airmass_extinction" => {
            let r = f_airmass(match v.get("zen") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("m") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_87(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "alfven_speed" => {
            let r = f_alfven_speed(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_115(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "altaz_from_hadec" => {
            let r = f_altaz_from_hadec(match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dec") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_153(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angular_separation" => {
            let r = f_angular_separation(match v.get("ra1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dec1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ra2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dec2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_181(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angular_size" => {
            let r = f_angular_size(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_209(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angular_size_at_redshift" => {
            let r = f_angular_size_at_z(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Om") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "apparent_bolometric_magnitude" => {
            let r = f_apparent_bolometric(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_281(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "apsis_speeds" => {
            let r = f_apsis_speeds(match v.get("rp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_311(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "atmospheric_scale_height" => {
            let r = f_scale_height(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_339(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binary_star_masses" => {
            let r = f_binary_masses(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_367(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bolometric_magnitude_from_luminosity" => {
            let r = f_bolometric_magnitude(match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_395(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "calendar_date_from_julian_date" => {
            let r = f_calendar_from_jd(match v.get("jd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "camera_pixel_scale" => {
            let r = f_camera_pixel_scale(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_459(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_magnitude" => {
            let r = f_combined_magnitude(match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_485(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "comoving_volume" => {
            let r = f_comoving_volume(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Om") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_525(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cosmological_distances" => {
            let r = f_cosmo_distances(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Om") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_565(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_density" => {
            let r = f_critical_density(match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Om") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_601(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "culmination_altitude" => {
            let r = f_culmination(match v.get("dec") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_636(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "debye_length" => {
            let r = f_debye_length(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_664(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "debye_number" => {
            let r = f_debye_number(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_692(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "distance_from_distance_modulus" => {
            let r = f_distance_from_modulus(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_721(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "distance_from_flux" => {
            let r = f_distance_from_flux(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_749(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "distance_unit_conversion" => {
            let r = f_distance_conversion(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_777(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "effective_temperature" => {
            let r = f_effective_temperature(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_805(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "einstein_radius" => {
            let r = f_einstein_radius(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_835(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "escape_delta_v" => {
            let r = f_escape_dv(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_inf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_865(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "flux_ratio_from_magnitudes" => {
            let r = f_flux_ratio(match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_891(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "free_fall_time" => {
            let r = f_free_fall_time(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_919(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gyrofrequency" => {
            let r = f_gyrofrequency(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("particle") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("electron")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_949(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gyroradius" => {
            let r = f_gyroradius(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_perp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("particle") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("electron")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_981(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hohmann_transfer" => {
            let r = f_hohmann(match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inertial_length" => {
            let r = f_inertial_length(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("particle") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("electron")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1043(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "julian_date" => {
            let r = f_julian_date(match v.get("year") { Some(x) => py::from_val(x), None => { return None } }, match v.get("month") { Some(x) => py::from_val(x), None => { return None } }, match v.get("day") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hour") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, match v.get("minute") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, match v.get("second") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1114(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kepler_position_at_time" => {
            let r = f_kepler_position(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1158(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "light_gathering_power" => {
            let r = f_light_gathering(match v.get("D1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1186(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_size_from_angle" => {
            let r = f_linear_size(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1216(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lookback_time" => {
            let r = f_lookback_time(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Om") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1254(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lower_hybrid_frequency" => {
            let r = f_lower_hybrid(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1283(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "luminosity_from_bolometric_magnitude" => {
            let r = f_luminosity_from_magnitude(match v.get("M_bol") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1310(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "magnetic_pressure" => {
            let r = f_magnetic_pressure(match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1338(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "magnitude_difference_from_flux_ratio" => {
            let r = f_magnitude_difference(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1366(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "orbit_from_state" => {
            let r = f_orbit_from_state(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1401(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallax_distance" => {
            let r = f_parallax_distance(match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1429(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plane_change_delta_v" => {
            let r = f_plane_change(match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("di") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1460(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "planet_equilibrium_temperature" => {
            let r = f_planet_equilibrium(match v.get("T_star") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_star") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1493(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plasma_beta" => {
            let r = f_plasma_beta(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1522(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plasma_frequency" => {
            let r = f_plasma_frequency(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("particle") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("electron")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1552(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "position_angle" => {
            let r = f_position_angle(match v.get("ra1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dec1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ra2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dec2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1584(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radial_velocity_semi_amplitude" => {
            let r = f_rv_semi_amplitude(match v.get("M_star") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("i") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1618(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rayleigh_resolution" => {
            let r = f_rayleigh_resolution(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1648(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "recession_velocity_from_redshift" => {
            let r = f_recession_velocity(match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1675(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "redshift_at_lookback_time" => {
            let r = f_redshift_at_lookback(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Om") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1721(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "redshift_from_wavelengths" => {
            let r = f_redshift_from_lines(match v.get("lam_obs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam_rest") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1749(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rise_set_hour_angle" => {
            let r = f_rise_set(match v.get("dec") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1785(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "saha_hydrogen_ionization" => {
            let r = f_saha_hydrogen(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1819(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "schwarzschild_radius" => {
            let r = f_schwarzschild_radius(match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1847(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sidereal_time" => {
            let r = f_sidereal_time(match v.get("jd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1877(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stellar_luminosity" => {
            let r = f_stellar_luminosity(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1905(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stellar_radius" => {
            let r = f_stellar_radius(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1933(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "telescope_magnification" => {
            let r = f_telescope_magnification(match v.get("f_obj") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f_eye") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("afov") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1967(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thermal_speed" => {
            let r = f_thermal_speed(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("convention") { Some(x) => py::from_val(x), None => { return None } }, match v.get("particle") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("electron")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1999(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "time_of_flight" => {
            let r = f_time_of_flight(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2037(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "transit_depth" => {
            let r = f_transit_depth(match v.get("Rp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2076(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "transit_duration" => {
            let r = f_transit_duration(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2117(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "upper_hybrid_frequency" => {
            let r = f_upper_hybrid(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2145(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_ARCSEC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(4.84813681109536e-06)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_AU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(149597870700.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(299792458.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_CHI_H() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.1787094174619686e-18)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_DEG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.017453292519943295)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EPS0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.8541878188e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_F0_BOL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.5180210026334008e-08)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.6743e-11)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_GYR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.15576e+16)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_H() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.62607015e-34)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_J11() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.8317059702075125)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.380649e-23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_L0_BOL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.0128e+28)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LSUN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.828e+26)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9460730472580800.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ME() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.1093837139e-31)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.67262192595e-27)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MPC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.085677581491367e+22)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MSUN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.988409870698051e+30)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MU0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.25663706127e-06)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_PC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.085677581491367e+16)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_QE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RGAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RSUN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(695700000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SIGMA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(5.6703744191844314e-08)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_YR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31557600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CONV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("most_probable"), Py::tuple(vec![Py::Float(2.0), Py::str("most_probable")])), (Py::str("rms"), Py::tuple(vec![Py::Float(3.0), Py::str("rms")])), (Py::str("mean"), Py::tuple(vec![Py::Float(2.5464790894703255), Py::str("mean_magnitude")]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__DAYS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Int(31i64), Py::Int(28i64), Py::Int(31i64), Py::Int(30i64), Py::Int(31i64), Py::Int(30i64), Py::Int(31i64), Py::Int(31i64), Py::Int(30i64), Py::Int(31i64), Py::Int(30i64), Py::Int(31i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PARTICLES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("electron"), Py::tuple(vec![Py::Float(9.1093837139e-31), Py::str("e-")])), (Py::str("proton"), Py::tuple(vec![Py::Float(1.67262192595e-27), Py::str("p+")]))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__altaz(mut v_H: Py, mut v_dec: Py, mut v_lat: Py) -> R {
    let mut v_alt: Py = Py::None;
    let mut v_az: Py = Py::None;
    let mut v_sa: Py = Py::None;
    v_sa = py::add(&py::mul(&py::math1("sin", &v_lat.clone())?, &py::math1("sin", &v_dec.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_lat.clone())?, &py::math1("cos", &v_dec.clone())?)?, &py::math1("cos", &v_H.clone())?)?)?;
    v_alt = py::math1("asin", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), v_sa.clone()], false)?], true)?)?;
    v_az = py::atan2(&py::mul(&py::neg(&py::math1("cos", &v_dec.clone())?)?, &py::math1("sin", &v_H.clone())?)?, &py::sub(&py::mul(&py::math1("sin", &v_dec.clone())?, &py::math1("cos", &v_lat.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_dec.clone())?, &py::math1("sin", &v_lat.clone())?)?, &py::math1("cos", &v_H.clone())?)?)?)?;
    return Ok(Py::tuple(vec![v_alt.clone(), py::modulo(&v_az.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_sky(mut v_dec: Py, mut v_lat: Py) -> R {
    if (py::not(&{ let t1765 = py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?; Py::Bool({ let t1766 = v_dec.clone(); if py::le(&t1765, &t1766)? { { let t1767 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t1766, &t1767)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the declination must lie between -90 and 90 degrees"))));
    }
    if (py::not(&{ let t1768 = py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?; Py::Bool({ let t1769 = v_lat.clone(); if py::le(&t1768, &t1769)? { { let t1770 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t1769, &t1770)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the latitude must lie between -90 and 90 degrees"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__comoving(mut v_z: Py, mut v_Om: Py) -> R {
    return Ok((if ({ let t555 = v_z.clone(); Py::Bool({ let t556 = Py::Int(0i64); if py::gt(&t555, &t556)? { true } else { false } }) }).truthy() { py::float(&ax_engine::pymp::call("quad", &[f__inv_E(v_Om.clone())?, Py::list(vec![Py::Int(0i64), v_z.clone()])], &[])?)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cosmo_check(mut v_H0: Py, mut v_Om: Py, mut v_z: Py) -> R {
    if (py::not(&{ let t1691 = Py::Int(1i64); Py::Bool({ let t1692 = v_H0.clone(); if py::le(&t1691, &t1692)? { { let t1693 = Py::Int(1000i64); if py::le(&t1692, &t1693)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("H0 must lie between 1 and 1000 km/s/Mpc"))));
    }
    if (py::not(&{ let t1694 = Py::Float(0.01); Py::Bool({ let t1695 = v_Om.clone(); if py::le(&t1694, &t1695)? { { let t1696 = Py::Int(1i64); if py::le(&t1695, &t1696)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Omega_m must lie between 0.01 and 1 (flat universe: Omega_Lambda = 1 - Omega_m)"))));
    }
    if ({ let t1702 = { let t1697 = v_z.clone(); Py::Bool({ let t1698 = Py::None; if !py::is(&t1697, &t1698) { true } else { false } }) }; if !t1702.truthy() { t1702 } else { py::not(&{ let t1699 = Py::Int(0i64); Py::Bool({ let t1700 = v_z.clone(); if py::le(&t1699, &t1700)? { { let t1701 = Py::Int(1100i64); if py::le(&t1700, &t1701)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the redshift must lie between 0 and 1100"))));
    }
    return Ok(py::truediv(&py::mul(&v_H0.clone(), &Py::Float(1000.0))?, &c_MPC()?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__doppler_beta(mut v_z: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::pow(&py::add(&Py::Int(1i64), &v_z.clone())?, &Py::Int(2i64))?;
    return Ok(py::truediv(&py::sub(&v_s.clone(), &Py::Int(1i64))?, &py::add(&v_s.clone(), &Py::Int(1i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inv_E(mut v_Om: Py) -> R {
    return Ok({ let v_Om = v_Om.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&Py::Int(1i64), &ax_engine::pymp::call("sqrt", &[py::add(&py::mul(&v_Om.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_x.clone())?, &Py::Int(3i64))?)?, &py::sub(&Py::Int(1i64), &v_Om.clone())?)?], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jd_exact(mut v_year: Py, mut v_month: Py, mut v_day: Py, mut v_hour: Py, mut v_minute: Py, mut v_second: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dim: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_whole: Py = Py::None;
    let mut v_y: Py = Py::None;
    if (py::not(&{ let t1072 = { let t1059 = v_year.clone(); Py::Bool({ let t1060 = Py::Int(1582i64); if py::gt(&t1059, &t1060)? { true } else { false } }) }; if t1072.truthy() { t1072 } else { { let t1071 = { let t1061 = v_year.clone(); Py::Bool({ let t1062 = Py::Int(1582i64); if py::eq(&t1061, &t1062) { true } else { false } }) }; if !t1071.truthy() { t1071 } else { { let t1070 = { let t1063 = v_month.clone(); Py::Bool({ let t1064 = Py::Int(10i64); if py::gt(&t1063, &t1064)? { true } else { false } }) }; if t1070.truthy() { t1070 } else { { let t1069 = { let t1065 = v_month.clone(); Py::Bool({ let t1066 = Py::Int(10i64); if py::eq(&t1065, &t1066) { true } else { false } }) }; if !t1069.truthy() { t1069 } else { { let t1067 = v_day.clone(); Py::Bool({ let t1068 = Py::Int(15i64); if py::ge(&t1067, &t1068)? { true } else { false } }) } } } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the date must be in the Gregorian calendar (on or after 1582-10-15)"))));
    }
    if ({ let t1073 = v_year.clone(); Py::Bool({ let t1074 = Py::Int(9999i64); if py::gt(&t1073, &t1074)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the year must be at most 9999"))));
    }
    if (py::not(&{ let t1075 = Py::Int(1i64); Py::Bool({ let t1076 = v_month.clone(); if py::le(&t1075, &t1076)? { { let t1077 = Py::Int(12i64); if py::le(&t1076, &t1077)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the month must be 1 to 12"))));
    }
    v_dim = (if ({ let t1088 = { let t1078 = v_month.clone(); Py::Bool({ let t1079 = Py::Int(2i64); if py::eq(&t1078, &t1079) { true } else { false } }) }; if !t1088.truthy() { t1088 } else { f__leap(v_year.clone())? } }).truthy() { Py::Int(29i64) } else { py::getitem(&c__DAYS()?, &py::sub(&v_month.clone(), &Py::Int(1i64))?)? });
    if (py::not(&{ let t1089 = Py::Int(1i64); Py::Bool({ let t1090 = v_day.clone(); if py::le(&t1089, &t1090)? { { let t1091 = v_dim.clone(); if py::le(&t1090, &t1091)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["day must be 1 to ".to_string(), py::format(&v_dim.clone(), "")?, " in that month".to_string()].concat()))));
    }
    if (py::not(&{ let t1102 = { let t1092 = Py::Int(0i64); Py::Bool({ let t1093 = v_hour.clone(); if py::le(&t1092, &t1093)? { { let t1094 = Py::Int(23i64); if py::le(&t1093, &t1094)? { true } else { false } } } else { false } }) }; if !t1102.truthy() { t1102 } else { { let t1101 = { let t1095 = Py::Int(0i64); Py::Bool({ let t1096 = v_minute.clone(); if py::le(&t1095, &t1096)? { { let t1097 = Py::Int(59i64); if py::le(&t1096, &t1097)? { true } else { false } } } else { false } }) }; if !t1101.truthy() { t1101 } else { { let t1098 = Py::Int(0i64); Py::Bool({ let t1099 = v_second.clone(); if py::le(&t1098, &t1099)? { { let t1100 = Py::Int(60i64); if py::lt(&t1099, &t1100)? { true } else { false } } } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time must be 0-23 h, 0-59 min, 0 <= s < 60"))));
    }
    let t1105 = ((if ({ let t1103 = v_month.clone(); Py::Bool({ let t1104 = Py::Int(2i64); if py::le(&t1103, &t1104)? { true } else { false } }) }).truthy() { Py::tuple(vec![py::sub(&v_year.clone(), &Py::Int(1i64))?, py::add(&v_month.clone(), &Py::Int(12i64))?]) } else { Py::tuple(vec![v_year.clone(), v_month.clone()]) })).items()?;
    if t1105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_y = t1105[0].clone();
    v_m = t1105[1].clone();
    v_a = py::floordiv(&v_y.clone(), &Py::Int(100i64))?;
    v_b = py::add(&py::sub(&Py::Int(2i64), &v_a.clone())?, &py::floordiv(&v_a.clone(), &Py::Int(4i64))?)?;
    v_whole = py::add(&py::add(&py::add(&py::floordiv(&py::mul(&Py::Int(1461i64), &py::add(&v_y.clone(), &Py::Int(4716i64))?)?, &Py::Int(4i64))?, &py::floordiv(&py::mul(&Py::Int(306001i64), &py::add(&v_m.clone(), &Py::Int(1i64))?)?, &Py::Int(10000i64))?)?, &v_day.clone())?, &v_b.clone())?;
    return Ok(py::add(&py::sub(&py::fraction(&[v_whole.clone()])?, &py::fraction(&[Py::Int(3049i64), Py::Int(2i64)])?)?, &py::truediv(&py::add(&py::fraction(&[py::add(&py::mul(&v_hour.clone(), &Py::Int(3600i64))?, &py::mul(&v_minute.clone(), &Py::Int(60i64))?)?])?, &py::fraction(&[v_second.clone()])?)?, &Py::Int(86400i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kepler_E(mut v_Mn: Py, mut v_e: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_En: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_step: Py = Py::None;
    let t1139 = (Py::tuple(vec![Py::Float(0.0), py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?])).items()?;
    if t1139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1139[0].clone();
    v_hi = t1139[1].clone();
    v_E = (if ({ let t1140 = v_e.clone(); Py::Bool({ let t1141 = Py::Float(0.8); if py::lt(&t1140, &t1141)? { true } else { false } }) }).truthy() { py::add(&v_Mn.clone(), &py::mul(&v_e.clone(), &py::math1("sin", &v_Mn.clone())?)?)? } else { Py::Float(std::f64::consts::PI) });
    for t1142 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t1142;
        v_f = py::sub(&py::sub(&v_E.clone(), &py::mul(&v_e.clone(), &py::math1("sin", &v_E.clone())?)?)?, &v_Mn.clone())?;
        if ({ let t1143 = v_f.clone(); Py::Bool({ let t1144 = Py::Int(0i64); if py::gt(&t1143, &t1144)? { true } else { false } }) }).truthy() {
            v_hi = v_E.clone();
        } else {
            v_lo = v_E.clone();
        }
        v_step = py::truediv(&v_f.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &py::math1("cos", &v_E.clone())?)?)?)?;
        v_En = py::sub(&v_E.clone(), &v_step.clone())?;
        if (py::not(&{ let t1145 = v_lo.clone(); Py::Bool({ let t1146 = v_En.clone(); if py::lt(&t1145, &t1146)? { { let t1147 = v_hi.clone(); if py::lt(&t1146, &t1147)? { true } else { false } } } else { false } }) })).truthy() {
            v_En = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        }
        if ({ let t1148 = py::abs(&py::sub(&v_En.clone(), &v_E.clone())?)?; Py::Bool({ let t1149 = py::mul(&Py::Float(4e-16), &py::minmax(&[Py::Float(1.0), py::abs(&v_E.clone())?], true)?)?; if py::le(&t1148, &t1149)? { true } else { false } }) }).truthy() {
            return Ok(v_En.clone());
        }
        v_E = v_En.clone();
    }
    return Ok(v_E.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__leap(mut v_y: Py) -> R {
    return Ok({ let t1087 = { let t1080 = py::modulo(&v_y.clone(), &Py::Int(4i64))?; Py::Bool({ let t1081 = Py::Int(0i64); if py::eq(&t1080, &t1081) { true } else { false } }) }; if !t1087.truthy() { t1087 } else { { let t1086 = { let t1082 = py::modulo(&v_y.clone(), &Py::Int(100i64))?; Py::Bool({ let t1083 = Py::Int(0i64); if py::ne(&t1082, &t1083) { true } else { false } }) }; if t1086.truthy() { t1086 } else { { let t1084 = py::modulo(&v_y.clone(), &Py::Int(400i64))?; Py::Bool({ let t1085 = Py::Int(0i64); if py::eq(&t1084, &t1085) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mass(mut v_particle: Py) -> R {
    if ({ let t1997 = v_particle.clone(); Py::Bool({ let t1998 = c__PARTICLES()?; if !py::contains(&t1998, &t1997)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the particle must be 'electron' or 'proton'"))));
    }
    return Ok(py::getitem(&py::getitem(&c__PARTICLES()?, &v_particle.clone())?, &Py::Int(0i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t2137 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2138 = (t2137).items()?;
if t2138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2138[0].clone();
v_v = t2138[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__overlap(mut v_k: Py, mut v_d: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    if ({ let t2062 = v_d.clone(); Py::Bool({ let t2063 = py::add(&Py::Int(1i64), &v_k.clone())?; if py::ge(&t2062, &t2063)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2064 = v_d.clone(); Py::Bool({ let t2065 = py::sub(&Py::Int(1i64), &v_k.clone())?; if py::le(&t2064, &t2065)? { true } else { false } }) }).truthy() {
        return Ok(py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_k.clone())?, &v_k.clone())?);
    }
    v_a1 = py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::sub(&py::add(&py::mul(&v_d.clone(), &v_d.clone())?, &py::mul(&v_k.clone(), &v_k.clone())?)?, &Py::Int(1i64))?, &py::mul(&py::mul(&Py::Int(2i64), &v_d.clone())?, &v_k.clone())?)?], false)?], true)?)?;
    v_a2 = py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::sub(&py::add(&py::mul(&v_d.clone(), &v_d.clone())?, &Py::Int(1i64))?, &py::mul(&v_k.clone(), &v_k.clone())?)?, &py::mul(&Py::Int(2i64), &v_d.clone())?)?], false)?], true)?)?;
    return Ok(py::sub(&py::add(&py::mul(&py::mul(&v_k.clone(), &v_k.clone())?, &v_a1.clone())?, &v_a2.clone())?, &py::mul(&Py::Float(0.5), &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::mul(&py::mul(&py::mul(&py::add(&py::add(&py::neg(&v_d.clone())?, &v_k.clone())?, &Py::Int(1i64))?, &py::sub(&py::add(&v_d.clone(), &v_k.clone())?, &Py::Int(1i64))?)?, &py::add(&py::sub(&v_d.clone(), &v_k.clone())?, &Py::Int(1i64))?)?, &py::add(&py::add(&v_d.clone(), &v_k.clone())?, &Py::Int(1i64))?)?], true)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t2133 in py::iter(&(v_vals.clone()))? {
        v_v = t2133;
        if (py::not(&{ let t2136 = py::math1("isfinite", &v_v.clone())?; if !t2136.truthy() { t2136 } else { { let t2134 = v_v.clone(); Py::Bool({ let t2135 = Py::Int(0i64); if py::gt(&t2134, &t2135)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t2139 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t2139.truthy() { t2139 } else { Py::Bool(matches!(v_x.clone(), Py::Str(_))) } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if ({ let t2142 = { let t2140 = v_x.clone(); Py::Bool({ let t2141 = Py::Int(0i64); if py::eq(&t2140, &t2141) { true } else { false } }) }; if t2142.truthy() { t2142 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t2143 = v_x.clone(); Py::Bool({ let t2144 = Py::Int(0i64); if py::eq(&t2143, &t2144) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t2151 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t2151.truthy() { t2151 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t2152 = v_a.clone(); Py::Bool({ let t2153 = v_b.clone(); if py::eq(&t2152, &t2153) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t2155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2154 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t2154.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2154[0].clone();
            v_b = t2154[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t2156 = v_d.clone(); Py::Bool({ let t2157 = Py::Int(0i64); if py::eq(&t2156, &t2157) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t2158 = v_s.clone(); Py::Bool({ let t2159 = Py::Int(0i64); if py::gt(&t2158, &t2159)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__saha_s(mut v_T: Py, mut v_n: Py) -> R {
    return Ok(py::truediv(&py::mul(&py::pow(&py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &c_ME()?)?, &c_KB()?)?, &v_T.clone())?, &py::pow(&c_H()?, &Py::Int(2i64))?)?, &Py::Float(1.5))?, &py::math1("exp", &py::truediv(&py::neg(&c_CHI_H()?)?, &py::mul(&c_KB()?, &v_T.clone())?)?)?)?, &v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_absolute_magnitude(mut v_m: Py, mut v_d: Py, mut v_A: Py) -> R {
    let mut v_mu: Py = Py::None;
    let _ = f__pos(Py::str("the distance"), Py::tuple(vec![v_d.clone()]))?;
    if ({ let t5 = v_A.clone(); Py::Bool({ let t6 = Py::Int(0i64); if py::lt(&t5, &t6)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the extinction A must not be negative"))));
    }
    v_mu = py::sub(&py::mul(&Py::Int(5i64), &py::math1("log10", &py::truediv(&v_d.clone(), &c_PC()?)?)?)?, &Py::Int(5i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("M"), py::sub(&py::sub(&v_m.clone(), &v_mu.clone())?, &v_A.clone())?), (Py::str("mu"), v_mu.clone()), (Py::str("d_pc"), py::truediv(&v_d.clone(), &c_PC()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_age_at_z(mut v_z: Py, mut v_H0: Py, mut v_Om: Py) -> R {
    let mut v_age: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_now: Py = Py::None;
    v_h = f__cosmo_check(v_H0.clone(), v_Om.clone(), v_z.clone())?;
    v_f = f__inv_E(v_Om.clone())?;
    v_g = { let v_H0 = v_H0.clone(); let v_Om = v_Om.clone(); let v_f = v_f.clone(); let v_h = v_h.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::call(&v_f.clone(), &[v_x.clone()])?, &py::add(&Py::Int(1i64), &v_x.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_age = py::truediv(&py::float(&ax_engine::pymp::call("quad", &[v_g.clone(), Py::list(vec![v_z.clone(), ax_engine::pymp::constant("inf")?])], &[])?)?, &v_h.clone())?;
    v_now = py::truediv(&py::float(&ax_engine::pymp::call("quad", &[v_g.clone(), Py::list(vec![Py::Int(0i64), ax_engine::pymp::constant("inf")?])], &[])?)?, &v_h.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("age"), v_age.clone()), (Py::str("age_Gyr"), py::truediv(&v_age.clone(), &c_GYR()?)?), (Py::str("age_now_Gyr"), py::truediv(&v_now.clone(), &c_GYR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_airmass(mut v_zen: Py, mut v_k: Py, mut v_m: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_out: Py = Py::None;
    if (py::not(&{ let t67 = Py::Int(0i64); Py::Bool({ let t68 = v_zen.clone(); if py::le(&t67, &t68)? { { let t69 = py::mul(&Py::Int(85i64), &c_DEG()?)?; if py::le(&t68, &t69)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the zenith angle must lie between 0 and 85 degrees (the plane-parallel model fails near the horizon)"))));
    }
    v_X = py::truediv(&Py::Int(1i64), &py::math1("cos", &v_zen.clone())?)?;
    v_out = Py::dict(vec![(Py::str("X"), v_X.clone()), (Py::str("alt_deg"), py::sub(&Py::Int(90i64), &py::truediv(&v_zen.clone(), &c_DEG()?)?)?)]);
    if ({ let t70 = v_k.clone(); Py::Bool({ let t71 = Py::None; if !py::is(&t70, &t71) { true } else { false } }) }).truthy() {
        if ({ let t72 = v_k.clone(); Py::Bool({ let t73 = Py::Int(0i64); if py::lt(&t72, &t73)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the extinction coefficient must not be negative"))));
        }
        py::setitem(&v_out.clone(), &Py::str("extinction"), py::mul(&v_k.clone(), &v_X.clone())?)?;
        if ({ let t74 = v_m.clone(); Py::Bool({ let t75 = Py::None; if !py::is(&t74, &t75) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &Py::str("m0"), py::sub(&v_m.clone(), &py::mul(&v_k.clone(), &v_X.clone())?)?)?;
        }
    } else {
        if ({ let t76 = v_m.clone(); Py::Bool({ let t77 = Py::None; if !py::is(&t76, &t77) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("an extinction-corrected magnitude needs the extinction coefficient k"))));
        }
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alfven_speed(mut v_B: Py, mut v_n: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the field and the density"), Py::tuple(vec![v_B.clone(), v_n.clone()]))?;
    v_v = py::truediv(&v_B.clone(), &py::math1("sqrt", &py::mul(&py::mul(&c_MU0()?, &v_n.clone())?, &py::add(&c_MP()?, &c_ME()?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_A"), v_v.clone()), (Py::str("v_A_km_s"), py::truediv(&v_v.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_altaz_from_hadec(mut v_H: Py, mut v_dec: Py, mut v_lat: Py) -> R {
    let mut v_alt: Py = Py::None;
    let mut v_alt_deg: Py = Py::None;
    let mut v_az: Py = Py::None;
    let mut v_az_deg: Py = Py::None;
    let _ = f__check_sky(v_dec.clone(), v_lat.clone())?;
    let t137 = (f__altaz(v_H.clone(), v_dec.clone(), v_lat.clone())?).items()?;
    if t137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_alt = t137[0].clone();
    v_az = t137[1].clone();
    if ({ let t138 = py::abs(&py::sub(&py::abs(&v_alt.clone())?, &py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?)?; Py::Bool({ let t139 = Py::Float(1e-09); if py::lt(&t138, &t139)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at the zenith or nadir the azimuth is undefined"))));
    }
    let t140 = (Py::tuple(vec![py::truediv(&v_az.clone(), &c_DEG()?)?, py::truediv(&v_alt.clone(), &c_DEG()?)?])).items()?;
    if t140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_az_deg = t140[0].clone();
    v_alt_deg = t140[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("alt_deg"), (if ({ let t149 = py::abs(&v_alt_deg.clone())?; Py::Bool({ let t150 = Py::Float(1e-10); if py::lt(&t149, &t150)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_alt_deg.clone() })), (Py::str("az_deg"), (if ({ let t151 = v_az_deg.clone(); Py::Bool({ let t152 = py::sub(&Py::Int(360i64), &Py::Float(1e-10))?; if py::ge(&t151, &t152)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_az_deg.clone() }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angular_separation(mut v_ra1: Py, mut v_dec1: Py, mut v_ra2: Py, mut v_dec2: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_dra: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_s: Py = Py::None;
    for t169 in py::iter(&(Py::tuple(vec![v_dec1.clone(), v_dec2.clone()])))? {
        v_d = t169;
        if (py::not(&{ let t170 = py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?; Py::Bool({ let t171 = v_d.clone(); if py::le(&t170, &t171)? { { let t172 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t171, &t172)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("declinations must lie between -90 and 90 degrees"))));
        }
    }
    v_dra = py::sub(&v_ra2.clone(), &v_ra1.clone())?;
    v_num = py::hypot(&[py::mul(&py::math1("cos", &v_dec2.clone())?, &py::math1("sin", &v_dra.clone())?)?, py::sub(&py::mul(&py::math1("cos", &v_dec1.clone())?, &py::math1("sin", &v_dec2.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_dec1.clone())?, &py::math1("cos", &v_dec2.clone())?)?, &py::math1("cos", &v_dra.clone())?)?)?])?;
    v_den = py::add(&py::mul(&py::math1("sin", &v_dec1.clone())?, &py::math1("sin", &v_dec2.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_dec1.clone())?, &py::math1("cos", &v_dec2.clone())?)?, &py::math1("cos", &v_dra.clone())?)?)?;
    v_s = py::atan2(&v_num.clone(), &v_den.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("sep_deg"), py::truediv(&v_s.clone(), &c_DEG()?)?), (Py::str("sep_arcsec"), py::truediv(&v_s.clone(), &c_ARCSEC()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angular_size(mut v_D: Py, mut v_d: Py) -> R {
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("the size and the distance"), Py::tuple(vec![v_D.clone(), v_d.clone()]))?;
    v_th = py::mul(&Py::Int(2i64), &py::math1("atan", &py::truediv(&v_D.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta"), v_th.clone()), (Py::str("theta_arcsec"), py::truediv(&v_th.clone(), &c_ARCSEC()?)?), (Py::str("theta_deg"), py::truediv(&v_th.clone(), &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angular_size_at_z(mut v_z: Py, mut v_H0: Py, mut v_Om: Py, mut v_D: Py) -> R {
    let mut v_DA: Py = Py::None;
    let mut v_h: Py = Py::None;
    v_h = f__cosmo_check(v_H0.clone(), v_Om.clone(), v_z.clone())?;
    if ({ let t237 = v_z.clone(); Py::Bool({ let t238 = Py::Int(0i64); if py::le(&t237, &t238)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the redshift must be positive"))));
    }
    let _ = f__pos(Py::str("the size"), Py::tuple(vec![v_D.clone()]))?;
    v_DA = py::truediv(&py::mul(&py::truediv(&c_C()?, &v_h.clone())?, &f__comoving(v_z.clone(), v_Om.clone())?)?, &py::add(&Py::Int(1i64), &v_z.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta_arcsec"), py::truediv(&py::truediv(&v_D.clone(), &v_DA.clone())?, &c_ARCSEC()?)?), (Py::str("scale_kpc_per_arcsec"), py::truediv(&py::mul(&v_DA.clone(), &c_ARCSEC()?)?, &py::mul(&Py::Float(1000.0), &c_PC()?)?)?), (Py::str("D_A_Mpc"), py::truediv(&v_DA.clone(), &c_MPC()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_apparent_bolometric(mut v_L: Py, mut v_d: Py) -> R {
    let mut v_F: Py = Py::None;
    let _ = f__pos(Py::str("the luminosity and the distance"), Py::tuple(vec![v_L.clone(), v_d.clone()]))?;
    v_F = py::truediv(&v_L.clone(), &py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_d.clone())?, &v_d.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("m_bol"), py::mul(&py::neg(&Py::Float(2.5))?, &py::math1("log10", &py::truediv(&v_F.clone(), &c_F0_BOL()?)?)?)?), (Py::str("F"), v_F.clone()), (Py::str("M_bol"), py::mul(&py::neg(&Py::Float(2.5))?, &py::math1("log10", &py::truediv(&v_L.clone(), &c_L0_BOL()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_apsis_speeds(mut v_rp: Py, mut v_ra: Py, mut v_M: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let _ = f__pos(Py::str("the periapsis and apoapsis distances and the central mass"), Py::tuple(vec![v_rp.clone(), v_ra.clone(), v_M.clone()]))?;
    if ({ let t301 = v_rp.clone(); Py::Bool({ let t302 = v_ra.clone(); if py::gt(&t301, &t302)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the periapsis distance must not exceed the apoapsis distance"))));
    }
    v_mu = py::mul(&c_G()?, &v_M.clone())?;
    v_a = py::truediv(&py::add(&v_rp.clone(), &v_ra.clone())?, &Py::Int(2i64))?;
    v_e = py::truediv(&py::sub(&v_ra.clone(), &v_rp.clone())?, &py::add(&v_ra.clone(), &v_rp.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("vp"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &v_ra.clone())?, &py::mul(&v_rp.clone(), &py::add(&v_rp.clone(), &v_ra.clone())?)?)?)?), (Py::str("va"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &v_rp.clone())?, &py::mul(&v_ra.clone(), &py::add(&v_rp.clone(), &v_ra.clone())?)?)?)?), (Py::str("a"), v_a.clone()), (Py::str("e"), v_e.clone()), (Py::str("P"), py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::math1("sqrt", &py::truediv(&py::pow(&v_a.clone(), &Py::Int(3i64))?, &v_mu.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_binary_masses(mut v_P: Py, mut v_a1: Py, mut v_a2: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("the period and the two semi-major axes"), Py::tuple(vec![v_P.clone(), v_a1.clone(), v_a2.clone()]))?;
    v_a = py::add(&v_a1.clone(), &v_a2.clone())?;
    v_M = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?, &py::mul(&py::mul(&c_G()?, &v_P.clone())?, &v_P.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("M_total"), v_M.clone()), (Py::str("M_total_Msun"), py::truediv(&v_M.clone(), &c_MSUN()?)?), (Py::str("M1_Msun"), py::truediv(&py::truediv(&py::mul(&v_M.clone(), &v_a2.clone())?, &v_a.clone())?, &c_MSUN()?)?), (Py::str("M2_Msun"), py::truediv(&py::truediv(&py::mul(&v_M.clone(), &v_a1.clone())?, &v_a.clone())?, &c_MSUN()?)?), (Py::str("a_au"), py::truediv(&v_a.clone(), &c_AU()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bolometric_magnitude(mut v_L: Py) -> R {
    let _ = f__pos(Py::str("the luminosity"), Py::tuple(vec![v_L.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("M_bol"), py::mul(&py::neg(&Py::Float(2.5))?, &py::math1("log10", &py::truediv(&v_L.clone(), &c_L0_BOL()?)?)?)?), (Py::str("L_Lsun"), py::truediv(&v_L.clone(), &c_LSUN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_calendar_from_jd(mut v_jd: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_Cc: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_day: Py = Py::None;
    let mut v_month: Py = Py::None;
    let mut v_year: Py = Py::None;
    if (py::not(&{ let t411 = Py::Float(2299160.5); Py::Bool({ let t412 = v_jd.clone(); if py::le(&t411, &t412)? { { let t413 = Py::Float(5373484.5); if py::le(&t412, &t413)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Julian date must lie in the Gregorian era (JD 2299160.5 to 5373484.5)"))));
    }
    v_J = py::add(&py::fraction(&[v_jd.clone()])?, &py::fraction(&[Py::Int(1i64), Py::Int(2i64)])?)?;
    v_Z = py::floordiv(&py::numerator(&v_J.clone())?, &py::denominator(&v_J.clone())?)?;
    v_F = py::sub(&v_J.clone(), &v_Z.clone())?;
    v_alpha = py::floordiv(&py::sub(&py::mul(&v_Z.clone(), &Py::Int(100i64))?, &Py::Int(186721625i64))?, &Py::Int(3652425i64))?;
    v_A = py::sub(&py::add(&py::add(&v_Z.clone(), &Py::Int(1i64))?, &v_alpha.clone())?, &py::floordiv(&v_alpha.clone(), &Py::Int(4i64))?)?;
    v_B = py::add(&v_A.clone(), &Py::Int(1524i64))?;
    v_Cc = py::floordiv(&py::sub(&py::mul(&v_B.clone(), &Py::Int(100i64))?, &Py::Int(12210i64))?, &Py::Int(36525i64))?;
    v_D = py::floordiv(&py::mul(&Py::Int(1461i64), &v_Cc.clone())?, &Py::Int(4i64))?;
    v_E = py::floordiv(&py::mul(&py::sub(&v_B.clone(), &v_D.clone())?, &Py::Int(10000i64))?, &Py::Int(306001i64))?;
    v_day = py::sub(&py::sub(&v_B.clone(), &v_D.clone())?, &py::floordiv(&py::mul(&Py::Int(306001i64), &v_E.clone())?, &Py::Int(10000i64))?)?;
    v_month = (if ({ let t414 = v_E.clone(); Py::Bool({ let t415 = Py::Int(14i64); if py::lt(&t414, &t415)? { true } else { false } }) }).truthy() { py::sub(&v_E.clone(), &Py::Int(1i64))? } else { py::sub(&v_E.clone(), &Py::Int(13i64))? });
    v_year = (if ({ let t416 = v_month.clone(); Py::Bool({ let t417 = Py::Int(2i64); if py::gt(&t416, &t417)? { true } else { false } }) }).truthy() { py::sub(&v_Cc.clone(), &Py::Int(4716i64))? } else { py::sub(&v_Cc.clone(), &Py::Int(4715i64))? });
    return Ok(f__out(Py::dict(vec![(Py::str("year"), v_year.clone()), (Py::str("month"), v_month.clone()), (Py::str("day"), v_day.clone()), (Py::str("hours"), py::float(&py::mul(&Py::Int(24i64), &v_F.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_camera_pixel_scale(mut v_f: Py, mut v_p: Py, mut v_N: Py) -> R {
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the focal length and the pixel size"), Py::tuple(vec![v_f.clone(), v_p.clone()]))?;
    v_out = Py::dict(vec![(Py::str("pixel_scale_arcsec"), py::truediv(&py::mul(&Py::Int(2i64), &py::math1("atan", &py::truediv(&v_p.clone(), &py::mul(&Py::Int(2i64), &v_f.clone())?)?)?)?, &c_ARCSEC()?)?)]);
    if ({ let t446 = v_N.clone(); Py::Bool({ let t447 = Py::None; if !py::is(&t446, &t447) { true } else { false } }) }).truthy() {
        if ({ let t448 = v_N.clone(); Py::Bool({ let t449 = Py::Int(1i64); if py::lt(&t448, &t449)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the number of pixels must be at least 1"))));
        }
        py::setitem(&v_out.clone(), &Py::str("fov_deg"), py::truediv(&py::mul(&Py::Int(2i64), &py::math1("atan", &py::truediv(&py::mul(&v_N.clone(), &v_p.clone())?, &py::mul(&Py::Int(2i64), &v_f.clone())?)?)?)?, &c_DEG()?)?)?;
        py::setitem(&v_out.clone(), &Py::str("fov_arcmin"), py::mul(&py::getitem(&v_out.clone(), &Py::str("fov_deg"))?, &Py::Int(60i64))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combined_magnitude(mut v_m1: Py, mut v_m2: Py) -> R {
    let mut v_lo: Py = Py::None;
    let mut v_total: Py = Py::None;
    if ({ let t475 = py::abs(&py::sub(&v_m1.clone(), &v_m2.clone())?)?; Py::Bool({ let t476 = Py::Int(100i64); if py::gt(&t475, &t476)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two magnitudes must differ by at most 100"))));
    }
    v_lo = py::minmax(&[v_m1.clone(), v_m2.clone()], false)?;
    v_total = py::sub(&v_lo.clone(), &py::mul(&Py::Float(2.5), &py::math1("log10", &py::add(&Py::Int(1i64), &py::pow(&Py::Int(10i64), &py::mul(&py::neg(&Py::Float(0.4))?, &py::abs(&py::sub(&v_m1.clone(), &v_m2.clone())?)?)?)?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), v_total.clone()), (Py::str("brightening"), py::sub(&v_lo.clone(), &v_total.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_comoving_volume(mut v_z: Py, mut v_H0: Py, mut v_Om: Py) -> R {
    let mut v_DC: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_h: Py = Py::None;
    v_h = f__cosmo_check(v_H0.clone(), v_Om.clone(), v_z.clone())?;
    if ({ let t513 = v_z.clone(); Py::Bool({ let t514 = Py::Int(0i64); if py::le(&t513, &t514)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the redshift must be positive"))));
    }
    v_DC = py::mul(&py::truediv(&c_C()?, &v_h.clone())?, &f__comoving(v_z.clone(), v_Om.clone())?)?;
    v_V = py::mul(&py::mul(&py::truediv(&Py::Int(4i64), &Py::Int(3i64))?, &Py::Float(std::f64::consts::PI))?, &py::pow(&v_DC.clone(), &Py::Int(3i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_Gpc3"), py::truediv(&v_V.clone(), &py::pow(&py::mul(&Py::Float(1000000000.0), &c_PC()?)?, &Py::Int(3i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cosmo_distances(mut v_z: Py, mut v_H0: Py, mut v_Om: Py) -> R {
    let mut v_DC: Py = Py::None;
    let mut v_DL: Py = Py::None;
    let mut v_h: Py = Py::None;
    v_h = f__cosmo_check(v_H0.clone(), v_Om.clone(), v_z.clone())?;
    if ({ let t553 = v_z.clone(); Py::Bool({ let t554 = Py::Int(0i64); if py::le(&t553, &t554)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the redshift must be positive"))));
    }
    v_DC = py::mul(&py::truediv(&c_C()?, &v_h.clone())?, &f__comoving(v_z.clone(), v_Om.clone())?)?;
    v_DL = py::mul(&py::add(&Py::Int(1i64), &v_z.clone())?, &v_DC.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D_C"), v_DC.clone()), (Py::str("D_C_Mpc"), py::truediv(&v_DC.clone(), &c_MPC()?)?), (Py::str("D_L_Mpc"), py::truediv(&v_DL.clone(), &c_MPC()?)?), (Py::str("D_A_Mpc"), py::truediv(&py::truediv(&v_DC.clone(), &py::add(&Py::Int(1i64), &v_z.clone())?)?, &c_MPC()?)?), (Py::str("mu"), py::mul(&Py::Int(5i64), &py::math1("log10", &py::truediv(&v_DL.clone(), &py::mul(&Py::Int(10i64), &c_PC()?)?)?)?)?), (Py::str("D_L"), v_DL.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_density(mut v_H0: Py, mut v_Om: Py, mut v_z: Py) -> R {
    let mut v_Hz: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_rho: Py = Py::None;
    v_h = f__cosmo_check(v_H0.clone(), v_Om.clone(), v_z.clone())?;
    v_Hz = py::mul(&v_h.clone(), &py::math1("sqrt", &py::sub(&py::add(&py::mul(&v_Om.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_z.clone())?, &Py::Int(3i64))?)?, &Py::Int(1i64))?, &v_Om.clone())?)?)?;
    v_rho = py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &v_Hz.clone())?, &v_Hz.clone())?, &py::mul(&py::mul(&Py::Int(8i64), &Py::Float(std::f64::consts::PI))?, &c_G()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("H_z"), py::truediv(&py::mul(&v_Hz.clone(), &c_MPC()?)?, &Py::Float(1000.0))?), (Py::str("rho_c"), v_rho.clone()), (Py::str("n_H"), py::truediv(&v_rho.clone(), &c_MP()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_culmination(mut v_dec: Py, mut v_lat: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_low: Py = Py::None;
    let mut v_up: Py = Py::None;
    let _ = f__check_sky(v_dec.clone(), v_lat.clone())?;
    let t623 = (Py::tuple(vec![py::truediv(&v_dec.clone(), &c_DEG()?)?, py::truediv(&v_lat.clone(), &c_DEG()?)?])).items()?;
    if t623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t623[0].clone();
    v_f = t623[1].clone();
    v_up = py::sub(&Py::Int(90i64), &py::abs(&py::sub(&v_f.clone(), &v_d.clone())?)?)?;
    v_low = py::sub(&py::abs(&py::add(&v_f.clone(), &v_d.clone())?)?, &Py::Int(90i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("alt_upper_deg"), v_up.clone()), (Py::str("alt_lower_deg"), v_low.clone()), (Py::str("circumpolar"), { let t632 = v_low.clone(); Py::Bool({ let t633 = Py::Int(0i64); if py::gt(&t632, &t633)? { true } else { false } }) }), (Py::str("never_rises"), { let t634 = v_up.clone(); Py::Bool({ let t635 = Py::Int(0i64); if py::lt(&t634, &t635)? { true } else { false } }) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_debye_length(mut v_T: Py, mut v_n: Py) -> R {
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("the temperature and the density"), Py::tuple(vec![v_T.clone(), v_n.clone()]))?;
    v_lam = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&c_EPS0()?, &c_KB()?)?, &v_T.clone())?, &py::mul(&py::mul(&v_n.clone(), &c_QE()?)?, &c_QE()?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_D"), v_lam.clone()), (Py::str("lambda_D_nm"), py::mul(&v_lam.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_debye_number(mut v_T: Py, mut v_n: Py) -> R {
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("the temperature and the density"), Py::tuple(vec![v_T.clone(), v_n.clone()]))?;
    v_lam = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&c_EPS0()?, &c_KB()?)?, &v_T.clone())?, &py::mul(&py::mul(&v_n.clone(), &c_QE()?)?, &c_QE()?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("N_D"), py::mul(&py::mul(&py::mul(&py::truediv(&Py::Int(4i64), &Py::Int(3i64))?, &Py::Float(std::f64::consts::PI))?, &v_n.clone())?, &py::pow(&v_lam.clone(), &Py::Int(3i64))?)?), (Py::str("lambda_D"), v_lam.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_conversion(mut v_d: Py) -> R {
    let _ = f__pos(Py::str("the distance"), Py::tuple(vec![v_d.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("d_m"), v_d.clone()), (Py::str("d_km"), py::truediv(&v_d.clone(), &Py::Float(1000.0))?), (Py::str("d_au"), py::truediv(&v_d.clone(), &c_AU()?)?), (Py::str("d_ly"), py::truediv(&v_d.clone(), &c_LY()?)?), (Py::str("d_pc"), py::truediv(&v_d.clone(), &c_PC()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_from_flux(mut v_L: Py, mut v_F: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the luminosity and the flux"), Py::tuple(vec![v_L.clone(), v_F.clone()]))?;
    v_d = py::math1("sqrt", &py::truediv(&v_L.clone(), &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_F.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("d_pc"), py::truediv(&v_d.clone(), &c_PC()?)?), (Py::str("d_ly"), py::truediv(&v_d.clone(), &c_LY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_from_modulus(mut v_m: Py, mut v_M: Py, mut v_A: Py) -> R {
    let mut v_d_pc: Py = Py::None;
    let mut v_mu: Py = Py::None;
    if ({ let t708 = v_A.clone(); Py::Bool({ let t709 = Py::Int(0i64); if py::lt(&t708, &t709)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the extinction A must not be negative"))));
    }
    v_mu = py::sub(&py::sub(&v_m.clone(), &v_M.clone())?, &v_A.clone())?;
    if (py::not(&{ let t710 = py::neg(&Py::Int(10i64))?; Py::Bool({ let t711 = v_mu.clone(); if py::lt(&t710, &t711)? { { let t712 = Py::Int(60i64); if py::lt(&t711, &t712)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the distance modulus must lie between -10 and 60 magnitudes"))));
    }
    v_d_pc = py::pow(&Py::Int(10i64), &py::add(&py::truediv(&v_mu.clone(), &Py::Int(5i64))?, &Py::Int(1i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), py::mul(&v_d_pc.clone(), &c_PC()?)?), (Py::str("d_pc"), v_d_pc.clone()), (Py::str("d_ly"), py::truediv(&py::mul(&v_d_pc.clone(), &c_PC()?)?, &c_LY()?)?), (Py::str("mu"), v_mu.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_effective_temperature(mut v_L: Py, mut v_R: Py) -> R {
    let mut v_T: Py = Py::None;
    let _ = f__pos(Py::str("the luminosity and the radius"), Py::tuple(vec![v_L.clone(), v_R.clone()]))?;
    v_T = py::pow(&py::truediv(&v_L.clone(), &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &c_SIGMA()?)?, &v_R.clone())?, &v_R.clone())?)?, &Py::Float(0.25))?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_einstein_radius(mut v_M: Py, mut v_D_l: Py, mut v_D_s: Py) -> R {
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("the mass and the two distances"), Py::tuple(vec![v_M.clone(), v_D_l.clone(), v_D_s.clone()]))?;
    if ({ let t825 = v_D_s.clone(); Py::Bool({ let t826 = v_D_l.clone(); if py::le(&t825, &t826)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the source must lie behind the lens (D_s > D_l)"))));
    }
    v_th = py::math1("sqrt", &py::truediv(&py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &c_G()?)?, &v_M.clone())?, &py::pow(&c_C()?, &Py::Int(2i64))?)?, &py::sub(&v_D_s.clone(), &v_D_l.clone())?)?, &py::mul(&v_D_l.clone(), &v_D_s.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta_E"), v_th.clone()), (Py::str("theta_E_arcsec"), py::truediv(&v_th.clone(), &c_ARCSEC()?)?), (Py::str("R_E"), py::mul(&v_th.clone(), &v_D_l.clone())?), (Py::str("R_E_au"), py::truediv(&py::mul(&v_th.clone(), &v_D_l.clone())?, &c_AU()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_escape_dv(mut v_M: Py, mut v_r: Py, mut v_v_inf: Py) -> R {
    let mut v_mu: Py = Py::None;
    let mut v_vc: Py = Py::None;
    let mut v_vd: Py = Py::None;
    let _ = f__pos(Py::str("the central mass and the orbit radius"), Py::tuple(vec![v_M.clone(), v_r.clone()]))?;
    if ({ let t855 = v_v_inf.clone(); Py::Bool({ let t856 = Py::Int(0i64); if py::lt(&t855, &t856)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the hyperbolic excess speed must not be negative"))));
    }
    v_mu = py::mul(&c_G()?, &v_M.clone())?;
    v_vc = py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_r.clone())?)?;
    v_vd = py::math1("sqrt", &py::add(&py::mul(&v_v_inf.clone(), &v_v_inf.clone())?, &py::truediv(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &v_r.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dv"), py::sub(&v_vd.clone(), &v_vc.clone())?), (Py::str("v_circ"), v_vc.clone()), (Py::str("v_depart"), v_vd.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flux_ratio(mut v_m1: Py, mut v_m2: Py) -> R {
    let mut v_dm: Py = Py::None;
    v_dm = py::sub(&v_m1.clone(), &v_m2.clone())?;
    if ({ let t881 = py::abs(&v_dm.clone())?; Py::Bool({ let t882 = Py::Int(100i64); if py::gt(&t881, &t882)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the magnitude difference must be at most 100"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ratio"), py::pow(&Py::Int(10i64), &py::mul(&py::neg(&Py::Float(0.4))?, &v_dm.clone())?)?), (Py::str("dm"), v_dm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_free_fall_time(mut v_rho: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("the density"), Py::tuple(vec![v_rho.clone()]))?;
    v_t = py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(3i64), &Py::Float(std::f64::consts::PI))?, &py::mul(&py::mul(&Py::Int(32i64), &c_G()?)?, &v_rho.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_ff"), v_t.clone()), (Py::str("t_ff_yr"), py::truediv(&v_t.clone(), &c_YR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gyrofrequency(mut v_B: Py, mut v_particle: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("the field"), Py::tuple(vec![v_B.clone()]))?;
    v_w = py::truediv(&py::mul(&c_QE()?, &v_B.clone())?, &f__mass(v_particle.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_c"), v_w.clone()), (Py::str("f_c"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("T_c"), py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_w.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gyroradius(mut v_B: Py, mut v_v_perp: Py, mut v_particle: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the field and the perpendicular speed"), Py::tuple(vec![v_B.clone(), v_v_perp.clone()]))?;
    if ({ let t969 = v_v_perp.clone(); Py::Bool({ let t970 = py::mul(&Py::Float(0.1), &c_C()?)?; if py::ge(&t969, &t970)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the speed must be well below c (v < 0.1 c) for the non-relativistic gyroradius"))));
    }
    v_r = py::truediv(&py::mul(&f__mass(v_particle.clone())?, &v_v_perp.clone())?, &py::mul(&c_QE()?, &v_B.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("r_L"), v_r.clone()), (Py::str("r_L_km"), py::truediv(&v_r.clone(), &Py::Float(1000.0))?), (Py::str("r_L_nm"), py::mul(&v_r.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hohmann(mut v_r1: Py, mut v_r2: Py, mut v_M: Py) -> R {
    let mut v_at: Py = Py::None;
    let mut v_dv1: Py = Py::None;
    let mut v_dv2: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_va: Py = Py::None;
    let mut v_vp: Py = Py::None;
    let _ = f__pos(Py::str("the two orbit radii and the central mass"), Py::tuple(vec![v_r1.clone(), v_r2.clone(), v_M.clone()]))?;
    if ({ let t1001 = py::abs(&py::sub(&v_r1.clone(), &v_r2.clone())?)?; Py::Bool({ let t1002 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_r1.clone(), v_r2.clone()], true)?)?; if py::le(&t1001, &t1002)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two orbits must have different radii"))));
    }
    v_mu = py::mul(&c_G()?, &v_M.clone())?;
    let t1003 = (Py::tuple(vec![py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_r1.clone())?)?, py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_r2.clone())?)?])).items()?;
    if t1003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t1003[0].clone();
    v_v2 = t1003[1].clone();
    v_vp = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &v_r2.clone())?, &py::mul(&v_r1.clone(), &py::add(&v_r1.clone(), &v_r2.clone())?)?)?)?;
    v_va = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &v_r1.clone())?, &py::mul(&v_r2.clone(), &py::add(&v_r1.clone(), &v_r2.clone())?)?)?)?;
    v_at = py::truediv(&py::add(&v_r1.clone(), &v_r2.clone())?, &Py::Int(2i64))?;
    v_t = py::mul(&Py::Float(std::f64::consts::PI), &py::math1("sqrt", &py::truediv(&py::pow(&v_at.clone(), &Py::Int(3i64))?, &v_mu.clone())?)?)?;
    let t1004 = (Py::tuple(vec![py::abs(&py::sub(&v_vp.clone(), &v_v1.clone())?)?, py::abs(&py::sub(&v_v2.clone(), &v_va.clone())?)?])).items()?;
    if t1004.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dv1 = t1004[0].clone();
    v_dv2 = t1004[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("dv1"), v_dv1.clone()), (Py::str("dv2"), v_dv2.clone()), (Py::str("dv"), py::add(&v_dv1.clone(), &v_dv2.clone())?), (Py::str("t_transfer"), v_t.clone()), (Py::str("t_days"), py::truediv(&v_t.clone(), &Py::Int(86400i64))?), (Py::str("a_transfer"), v_at.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inertial_length(mut v_n: Py, mut v_particle: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("the density"), Py::tuple(vec![v_n.clone()]))?;
    v_w = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_QE()?)?, &c_QE()?)?, &py::mul(&c_EPS0()?, &f__mass(v_particle.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), py::truediv(&c_C()?, &v_w.clone())?), (Py::str("d_km"), py::truediv(&py::truediv(&c_C()?, &v_w.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_julian_date(mut v_year: Py, mut v_month: Py, mut v_day: Py, mut v_hour: Py, mut v_minute: Py, mut v_second: Py) -> R {
    let mut v_jd: Py = Py::None;
    v_jd = f__jd_exact(py::int(&v_year.clone())?, py::int(&v_month.clone())?, py::int(&v_day.clone())?, py::int(&v_hour.clone())?, py::int(&v_minute.clone())?, v_second.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("JD"), py::float(&v_jd.clone())?), (Py::str("MJD"), py::float(&py::sub(&v_jd.clone(), &py::fraction(&[Py::Int(4800001i64), Py::Int(2i64)])?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kepler_position(mut v_M: Py, mut v_a: Py, mut v_e: Py, mut v_t: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_Mn: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the central mass and the semi-major axis"), Py::tuple(vec![v_M.clone(), v_a.clone()]))?;
    if (py::not(&{ let t1134 = Py::Int(0i64); Py::Bool({ let t1135 = v_e.clone(); if py::le(&t1134, &t1135)? { { let t1136 = Py::Float(0.99); if py::lt(&t1135, &t1136)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the eccentricity must satisfy 0 <= e < 0.99 (an elliptical orbit)"))));
    }
    if ({ let t1137 = v_t.clone(); Py::Bool({ let t1138 = Py::Int(0i64); if py::lt(&t1137, &t1138)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time since periapsis must not be negative"))));
    }
    v_mu = py::mul(&c_G()?, &v_M.clone())?;
    v_n = py::math1("sqrt", &py::truediv(&v_mu.clone(), &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?;
    v_P = py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_n.clone())?;
    v_Mn = py::fmod(&py::mul(&v_n.clone(), &v_t.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    v_E = f__kepler_E(v_Mn.clone(), v_e.clone())?;
    v_nu = py::modulo(&py::atan2(&py::mul(&py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_e.clone())?)?)?, &py::math1("sin", &v_E.clone())?)?, &py::sub(&py::math1("cos", &v_E.clone())?, &v_e.clone())?)?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    v_r = py::mul(&v_a.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &py::math1("cos", &v_E.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("mean_anomaly"), py::truediv(&v_Mn.clone(), &c_DEG()?)?), (Py::str("E"), py::truediv(&v_E.clone(), &c_DEG()?)?), (Py::str("nu"), py::truediv(&v_nu.clone(), &c_DEG()?)?), (Py::str("r"), v_r.clone()), (Py::str("v"), py::math1("sqrt", &py::mul(&v_mu.clone(), &py::sub(&py::truediv(&Py::Int(2i64), &v_r.clone())?, &py::truediv(&Py::Int(1i64), &v_a.clone())?)?)?)?), (Py::str("P"), v_P.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_light_gathering(mut v_D1: Py, mut v_D2: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the two apertures"), Py::tuple(vec![v_D1.clone(), v_D2.clone()]))?;
    v_r = py::pow(&py::truediv(&v_D1.clone(), &v_D2.clone())?, &Py::Int(2i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("area_ratio"), v_r.clone()), (Py::str("mag_gain"), py::mul(&Py::Float(2.5), &py::math1("log10", &v_r.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_size(mut v_theta: Py, mut v_d: Py) -> R {
    let mut v_D: Py = Py::None;
    let _ = f__pos(Py::str("the angle and the distance"), Py::tuple(vec![v_theta.clone(), v_d.clone()]))?;
    if ({ let t1206 = v_theta.clone(); Py::Bool({ let t1207 = Py::Float(std::f64::consts::PI); if py::ge(&t1206, &t1207)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angular size must be smaller than 180 degrees"))));
    }
    v_D = py::mul(&py::mul(&Py::Int(2i64), &v_d.clone())?, &py::math1("tan", &py::truediv(&v_theta.clone(), &Py::Int(2i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("D_km"), py::truediv(&v_D.clone(), &Py::Float(1000.0))?), (Py::str("D_au"), py::truediv(&v_D.clone(), &c_AU()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lookback_time(mut v_z: Py, mut v_H0: Py, mut v_Om: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_h = f__cosmo_check(v_H0.clone(), v_Om.clone(), v_z.clone())?;
    if ({ let t1244 = v_z.clone(); Py::Bool({ let t1245 = Py::Int(0i64); if py::le(&t1244, &t1245)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the redshift must be positive"))));
    }
    v_f = f__inv_E(v_Om.clone())?;
    v_t = py::truediv(&py::float(&ax_engine::pymp::call("quad", &[{ let v_H0 = v_H0.clone(); let v_Om = v_Om.clone(); let v_f = v_f.clone(); let v_h = v_h.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::call(&v_f.clone(), &[v_x.clone()])?, &py::add(&Py::Int(1i64), &v_x.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![Py::Int(0i64), v_z.clone()])], &[])?)?, &v_h.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_L"), v_t.clone()), (Py::str("t_L_Gyr"), py::truediv(&v_t.clone(), &c_GYR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lower_hybrid(mut v_B: Py, mut v_n: Py) -> R {
    let mut v_w: Py = Py::None;
    let mut v_wce: Py = Py::None;
    let mut v_wci: Py = Py::None;
    let mut v_wpi2: Py = Py::None;
    let _ = f__pos(Py::str("the field and the density"), Py::tuple(vec![v_B.clone(), v_n.clone()]))?;
    let t1274 = (Py::tuple(vec![py::truediv(&py::mul(&c_QE()?, &v_B.clone())?, &c_MP()?)?, py::truediv(&py::mul(&c_QE()?, &v_B.clone())?, &c_ME()?)?])).items()?;
    if t1274.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_wci = t1274[0].clone();
    v_wce = t1274[1].clone();
    v_wpi2 = py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_QE()?)?, &c_QE()?)?, &py::mul(&c_EPS0()?, &c_MP()?)?)?;
    v_w = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::truediv(&Py::Int(1i64), &py::mul(&v_wci.clone(), &v_wce.clone())?)?, &py::truediv(&Py::Int(1i64), &v_wpi2.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_lh"), v_w.clone()), (Py::str("f_lh"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_luminosity_from_magnitude(mut v_M_bol: Py) -> R {
    let mut v_L: Py = Py::None;
    if (py::not(&{ let t1299 = py::neg(&Py::Int(30i64))?; Py::Bool({ let t1300 = v_M_bol.clone(); if py::lt(&t1299, &t1300)? { { let t1301 = Py::Int(40i64); if py::lt(&t1300, &t1301)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the absolute bolometric magnitude must lie between -30 and 40"))));
    }
    v_L = py::mul(&c_L0_BOL()?, &py::pow(&Py::Int(10i64), &py::mul(&py::neg(&Py::Float(0.4))?, &v_M_bol.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("L_Lsun"), py::truediv(&v_L.clone(), &c_LSUN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_magnetic_pressure(mut v_B: Py) -> R {
    let mut v_p: Py = Py::None;
    let _ = f__pos(Py::str("the field"), Py::tuple(vec![v_B.clone()]))?;
    v_p = py::truediv(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&Py::Int(2i64), &c_MU0()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("p_B"), v_p.clone()), (Py::str("energy_density"), v_p.clone()), (Py::str("p_B_nPa"), py::mul(&v_p.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_magnitude_difference(mut v_ratio: Py) -> R {
    let _ = f__pos(Py::str("the flux ratio"), Py::tuple(vec![v_ratio.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("dm"), py::mul(&py::neg(&Py::Float(2.5))?, &py::math1("log10", &v_ratio.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_orbit_from_state(mut v_M: Py, mut v_r: Py, mut v_v: Py, mut v_gamma: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let _ = f__pos(Py::str("the central mass, the distance and the speed"), Py::tuple(vec![v_M.clone(), v_r.clone(), v_v.clone()]))?;
    if (py::not(&{ let t1386 = py::abs(&v_gamma.clone())?; Py::Bool({ let t1387 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t1386, &t1387)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the flight-path angle must lie strictly between -90 and 90 degrees"))));
    }
    v_mu = py::mul(&c_G()?, &v_M.clone())?;
    v_s = py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_v.clone())?, &v_v.clone())?, &v_mu.clone())?;
    if ({ let t1388 = v_s.clone(); Py::Bool({ let t1389 = Py::Int(2i64); if py::ge(&t1388, &t1389)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the orbit is not bound (the speed is at or above the escape speed)"))));
    }
    let t1390 = (Py::tuple(vec![py::math1("cos", &v_gamma.clone())?, py::math1("sin", &v_gamma.clone())?])).items()?;
    if t1390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1390[0].clone();
    v_sn = t1390[1].clone();
    v_a = py::truediv(&v_r.clone(), &py::sub(&Py::Int(2i64), &v_s.clone())?)?;
    v_e = py::hypot(&[py::sub(&py::mul(&py::mul(&v_s.clone(), &v_c.clone())?, &v_c.clone())?, &Py::Int(1i64))?, py::mul(&py::mul(&v_s.clone(), &v_c.clone())?, &v_sn.clone())?])?;
    v_e = (if ({ let t1391 = v_e.clone(); Py::Bool({ let t1392 = Py::Float(1e-12); if py::lt(&t1391, &t1392)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_e.clone() });
    return Ok(f__out(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("e"), v_e.clone()), (Py::str("rp"), py::mul(&v_a.clone(), &py::sub(&Py::Int(1i64), &v_e.clone())?)?), (Py::str("ra"), py::mul(&v_a.clone(), &py::add(&Py::Int(1i64), &v_e.clone())?)?), (Py::str("P"), py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::math1("sqrt", &py::truediv(&py::pow(&v_a.clone(), &Py::Int(3i64))?, &v_mu.clone())?)?)?), (Py::str("energy"), py::sub(&py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &Py::Int(2i64))?, &py::truediv(&v_mu.clone(), &v_r.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallax_distance(mut v_p: Py) -> R {
    let mut v_d_pc: Py = Py::None;
    let _ = f__pos(Py::str("the parallax"), Py::tuple(vec![v_p.clone()]))?;
    v_d_pc = py::truediv(&c_ARCSEC()?, &v_p.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), py::mul(&v_d_pc.clone(), &c_PC()?)?), (Py::str("d_pc"), v_d_pc.clone()), (Py::str("d_ly"), py::truediv(&py::mul(&v_d_pc.clone(), &c_PC()?)?, &c_LY()?)?), (Py::str("p_arcsec"), py::truediv(&v_p.clone(), &c_ARCSEC()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plane_change(mut v_v1: Py, mut v_v2: Py, mut v_di: Py) -> R {
    let mut v_dv: Py = Py::None;
    let _ = f__pos(Py::str("the two speeds"), Py::tuple(vec![v_v1.clone(), v_v2.clone()]))?;
    if (py::not(&{ let t1449 = Py::Int(0i64); Py::Bool({ let t1450 = v_di.clone(); if py::le(&t1449, &t1450)? { { let t1451 = Py::Float(std::f64::consts::PI); if py::le(&t1450, &t1451)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle between the velocities must lie between 0 and 180 degrees"))));
    }
    v_dv = py::math1("sqrt", &py::add(&py::pow(&py::sub(&v_v1.clone(), &v_v2.clone())?, &Py::Int(2i64))?, &py::mul(&py::mul(&py::mul(&Py::Int(4i64), &v_v1.clone())?, &v_v2.clone())?, &py::pow(&py::math1("sin", &py::truediv(&v_di.clone(), &Py::Int(2i64))?)?, &Py::Int(2i64))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dv"), v_dv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_planet_equilibrium(mut v_T_star: Py, mut v_R_star: Py, mut v_a: Py, mut v_A: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let _ = f__pos(Py::str("the stellar temperature, the stellar radius and the orbital distance"), Py::tuple(vec![v_T_star.clone(), v_R_star.clone(), v_a.clone()]))?;
    if (py::not(&{ let t1480 = Py::Int(0i64); Py::Bool({ let t1481 = v_A.clone(); if py::le(&t1480, &t1481)? { { let t1482 = Py::Int(1i64); if py::lt(&t1481, &t1482)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Bond albedo A must satisfy 0 <= A < 1"))));
    }
    if ({ let t1483 = v_a.clone(); Py::Bool({ let t1484 = v_R_star.clone(); if py::le(&t1483, &t1484)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the orbital distance must exceed the stellar radius"))));
    }
    v_T = py::mul(&py::mul(&v_T_star.clone(), &py::math1("sqrt", &py::truediv(&v_R_star.clone(), &py::mul(&Py::Int(2i64), &v_a.clone())?)?)?)?, &py::pow(&py::sub(&Py::Int(1i64), &v_A.clone())?, &Py::Float(0.25))?)?;
    v_S = py::mul(&py::mul(&c_SIGMA()?, &py::pow(&v_T_star.clone(), &Py::Int(4i64))?)?, &py::pow(&py::truediv(&v_R_star.clone(), &v_a.clone())?, &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_eq"), v_T.clone()), (Py::str("S"), v_S.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plasma_beta(mut v_T: Py, mut v_n: Py, mut v_B: Py) -> R {
    let mut v_pb: Py = Py::None;
    let mut v_pt: Py = Py::None;
    let _ = f__pos(Py::str("the temperature, the density and the field"), Py::tuple(vec![v_T.clone(), v_n.clone(), v_B.clone()]))?;
    let t1513 = (Py::tuple(vec![py::mul(&py::mul(&v_n.clone(), &c_KB()?)?, &v_T.clone())?, py::truediv(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&Py::Int(2i64), &c_MU0()?)?)?])).items()?;
    if t1513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pt = t1513[0].clone();
    v_pb = t1513[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("beta"), py::truediv(&v_pt.clone(), &v_pb.clone())?), (Py::str("p_thermal"), v_pt.clone()), (Py::str("p_magnetic"), v_pb.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plasma_frequency(mut v_n: Py, mut v_particle: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("the density"), Py::tuple(vec![v_n.clone()]))?;
    v_w = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_QE()?)?, &c_QE()?)?, &py::mul(&c_EPS0()?, &f__mass(v_particle.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_p"), v_w.clone()), (Py::str("f_p"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_position_angle(mut v_ra1: Py, mut v_dec1: Py, mut v_ra2: Py, mut v_dec2: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_dra: Py = Py::None;
    let mut v_pa: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    for t1568 in py::iter(&(Py::tuple(vec![v_dec1.clone(), v_dec2.clone()])))? {
        v_d = t1568;
        if (py::not(&{ let t1569 = py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?; Py::Bool({ let t1570 = v_d.clone(); if py::lt(&t1569, &t1570)? { { let t1571 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t1570, &t1571)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("declinations must lie strictly between -90 and 90 degrees (no position angle at a pole)"))));
        }
    }
    v_dra = py::sub(&v_ra2.clone(), &v_ra1.clone())?;
    v_y = py::mul(&py::math1("sin", &v_dra.clone())?, &py::math1("cos", &v_dec2.clone())?)?;
    v_x = py::sub(&py::mul(&py::math1("cos", &v_dec1.clone())?, &py::math1("sin", &v_dec2.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_dec1.clone())?, &py::math1("cos", &v_dec2.clone())?)?, &py::math1("cos", &v_dra.clone())?)?)?;
    if ({ let t1572 = py::hypot(&[v_x.clone(), v_y.clone()])?; Py::Bool({ let t1573 = Py::Float(1e-12); if py::lt(&t1572, &t1573)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two positions coincide (or are antipodal): no position angle"))));
    }
    v_pa = py::truediv(&py::modulo(&py::atan2(&v_y.clone(), &v_x.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &c_DEG()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("pa_deg"), (if ({ let t1582 = v_pa.clone(); Py::Bool({ let t1583 = py::sub(&Py::Int(360i64), &Py::Float(1e-10))?; if py::ge(&t1582, &t1583)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_pa.clone() }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rayleigh_resolution(mut v_lam: Py, mut v_D: Py) -> R {
    let mut v_th: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the wavelength and the aperture"), Py::tuple(vec![v_lam.clone(), v_D.clone()]))?;
    v_x = py::truediv(&py::mul(&c_J11()?, &v_lam.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?)?;
    if ({ let t1638 = v_x.clone(); Py::Bool({ let t1639 = Py::Float(0.5); if py::ge(&t1638, &t1639)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wavelength must be much smaller than the aperture (J11 lambda / (pi D) < 0.5)"))));
    }
    v_th = py::math1("asin", &v_x.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta"), v_th.clone()), (Py::str("theta_arcsec"), py::truediv(&v_th.clone(), &c_ARCSEC()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_recession_velocity(mut v_z: Py) -> R {
    let mut v_b: Py = Py::None;
    if (py::not(&{ let t1664 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1665 = v_z.clone(); if py::lt(&t1664, &t1665)? { { let t1666 = Py::Float(10000.0); if py::le(&t1665, &t1666)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the redshift must lie between -1 (excluded) and 10000"))));
    }
    v_b = f__doppler_beta(v_z.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), py::mul(&v_b.clone(), &c_C()?)?), (Py::str("beta"), v_b.clone()), (Py::str("v_cz"), py::mul(&c_C()?, &v_z.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_redshift_at_lookback(mut v_t: Py, mut v_H0: Py, mut v_Om: Py) -> R {
    let mut v_OL: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_h = f__cosmo_check(v_H0.clone(), v_Om.clone(), Py::None)?;
    let _ = f__pos(Py::str("the lookback time"), Py::tuple(vec![v_t.clone()]))?;
    if ({ let t1707 = v_Om.clone(); Py::Bool({ let t1708 = Py::Int(1i64); if py::eq(&t1707, &t1708) { true } else { false } }) }).truthy() {
        v_t0 = py::truediv(&Py::Int(2i64), &py::mul(&Py::Int(3i64), &v_h.clone())?)?;
        if ({ let t1709 = v_t.clone(); Py::Bool({ let t1710 = v_t0.clone(); if py::ge(&t1709, &t1710)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the lookback time must be shorter than the age of the universe"))));
        }
        v_z = py::sub(&py::pow(&py::truediv(&py::sub(&v_t0.clone(), &v_t.clone())?, &v_t0.clone())?, &py::truediv(&py::neg(&Py::Int(2i64))?, &Py::Int(3i64))?)?, &Py::Int(1i64))?;
    } else {
        v_OL = py::sub(&Py::Int(1i64), &v_Om.clone())?;
        v_q = py::math1("sqrt", &py::truediv(&v_OL.clone(), &v_Om.clone())?)?;
        v_k = py::mul(&py::mul(&Py::Float(1.5), &v_h.clone())?, &py::math1("sqrt", &v_OL.clone())?)?;
        v_t0 = py::truediv(&py::math1("asinh", &v_q.clone())?, &v_k.clone())?;
        if ({ let t1711 = v_t.clone(); Py::Bool({ let t1712 = v_t0.clone(); if py::ge(&t1711, &t1712)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the lookback time must be shorter than the age of the universe"))));
        }
        v_z = py::sub(&py::pow(&py::truediv(&v_q.clone(), &py::math1("sinh", &py::mul(&v_k.clone(), &py::sub(&v_t0.clone(), &v_t.clone())?)?)?)?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?, &Py::Int(1i64))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("age_now_Gyr"), py::truediv(&v_t0.clone(), &c_GYR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_redshift_from_lines(mut v_lam_obs: Py, mut v_lam_rest: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("the two wavelengths"), Py::tuple(vec![v_lam_obs.clone(), v_lam_rest.clone()]))?;
    v_z = py::sub(&py::truediv(&v_lam_obs.clone(), &v_lam_rest.clone())?, &Py::Int(1i64))?;
    v_b = f__doppler_beta(v_z.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("v"), py::mul(&v_b.clone(), &c_C()?)?), (Py::str("beta"), v_b.clone()), (Py::str("d_lambda"), py::sub(&v_lam_obs.clone(), &v_lam_rest.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rise_set(mut v_dec: Py, mut v_lat: Py) -> R {
    let mut v_H0: Py = Py::None;
    let mut v_az: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__check_sky(v_dec.clone(), v_lat.clone())?;
    v_x = py::mul(&py::neg(&py::math1("tan", &v_lat.clone())?)?, &py::math1("tan", &v_dec.clone())?)?;
    if ({ let t1776 = { let t1771 = py::abs(&v_lat.clone())?; Py::Bool({ let t1772 = py::sub(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &Py::Float(1e-12))?; if py::ge(&t1771, &t1772)? { true } else { false } }) }; if t1776.truthy() { t1776 } else { py::not(&{ let t1773 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1774 = v_x.clone(); if py::lt(&t1773, &t1774)? { { let t1775 = Py::Int(1i64); if py::lt(&t1774, &t1775)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the star does not rise and set at this latitude (it is circumpolar or never rises)"))));
    }
    v_H0 = py::math1("acos", &v_x.clone())?;
    v_az = py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::math1("sin", &v_dec.clone())?, &py::math1("cos", &v_lat.clone())?)?], false)?], true)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("H_set_h"), py::mul(&py::truediv(&v_H0.clone(), &Py::Float(std::f64::consts::PI))?, &Py::Int(12i64))?), (Py::str("time_up_sidereal_h"), py::mul(&py::truediv(&py::mul(&Py::Int(2i64), &v_H0.clone())?, &Py::Float(std::f64::consts::PI))?, &Py::Int(12i64))?), (Py::str("az_rise_deg"), py::truediv(&v_az.clone(), &c_DEG()?)?), (Py::str("az_set_deg"), py::sub(&Py::Int(360i64), &py::truediv(&v_az.clone(), &c_DEG()?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rv_semi_amplitude(mut v_M_star: Py, mut v_m_p: Py, mut v_P: Py, mut v_e: Py, mut v_i: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Mt: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("the masses and the period"), Py::tuple(vec![v_M_star.clone(), v_m_p.clone(), v_P.clone()]))?;
    if (py::not(&{ let t1604 = Py::Int(0i64); Py::Bool({ let t1605 = v_e.clone(); if py::le(&t1604, &t1605)? { { let t1606 = Py::Float(0.99); if py::lt(&t1605, &t1606)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the eccentricity must satisfy 0 <= e < 0.99"))));
    }
    if (py::not(&{ let t1607 = Py::Int(0i64); Py::Bool({ let t1608 = v_i.clone(); if py::lt(&t1607, &t1608)? { { let t1609 = Py::Float(std::f64::consts::PI); if py::le(&t1608, &t1609)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inclination must lie between 0 (excluded) and 180 degrees"))));
    }
    v_Mt = py::add(&v_M_star.clone(), &v_m_p.clone())?;
    v_K = py::truediv(&py::mul(&py::mul(&py::pow(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &c_G()?)?, &v_P.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?, &v_m_p.clone())?, &py::math1("sin", &v_i.clone())?)?, &py::mul(&py::pow(&v_Mt.clone(), &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_e.clone())?)?)?)?)?;
    v_a = py::pow(&py::truediv(&py::mul(&py::mul(&py::mul(&c_G()?, &v_Mt.clone())?, &v_P.clone())?, &v_P.clone())?, &py::mul(&Py::Int(4i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?)?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("a"), v_a.clone()), (Py::str("a_au"), py::truediv(&v_a.clone(), &c_AU()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_saha_hydrogen(mut v_T: Py, mut v_n: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the temperature and the density"), Py::tuple(vec![v_T.clone(), v_n.clone()]))?;
    if (py::not(&{ let t1805 = Py::Int(1000i64); Py::Bool({ let t1806 = v_T.clone(); if py::le(&t1805, &t1806)? { { let t1807 = Py::Float(1000000.0); if py::le(&t1806, &t1807)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature must lie between 1000 K and 1e6 K"))));
    }
    v_s = f__saha_s(v_T.clone(), v_n.clone())?;
    if (py::not(&{ let t1808 = Py::Float(1e-200); Py::Bool({ let t1809 = v_s.clone(); if py::lt(&t1808, &t1809)? { { let t1810 = Py::Float(100000000000000.0); if py::lt(&t1809, &t1810)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the hydrogen is completely neutral or completely ionized at these conditions (fraction not resolvable)"))));
    }
    v_x = py::truediv(&py::mul(&Py::Int(2i64), &v_s.clone())?, &py::add(&v_s.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_s.clone(), &v_s.clone())?, &py::mul(&Py::Int(4i64), &v_s.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("neutral"), py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &v_s.clone())?), (Py::str("n_e"), py::mul(&v_x.clone(), &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scale_height(mut v_T: Py, mut v_mu: Py, mut v_g: Py) -> R {
    let mut v_Hs: Py = Py::None;
    let _ = f__pos(Py::str("the temperature, the molar mass and the gravity"), Py::tuple(vec![v_T.clone(), v_mu.clone(), v_g.clone()]))?;
    v_Hs = py::truediv(&py::mul(&c_RGAS()?, &v_T.clone())?, &py::mul(&v_mu.clone(), &v_g.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("H"), v_Hs.clone()), (Py::str("H_km"), py::truediv(&v_Hs.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_schwarzschild_radius(mut v_M: Py) -> R {
    let mut v_rs: Py = Py::None;
    let _ = f__pos(Py::str("the mass"), Py::tuple(vec![v_M.clone()]))?;
    v_rs = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &c_G()?)?, &v_M.clone())?, &py::pow(&c_C()?, &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("rs"), v_rs.clone()), (Py::str("rs_km"), py::truediv(&v_rs.clone(), &Py::Float(1000.0))?), (Py::str("M_Msun"), py::truediv(&v_M.clone(), &c_MSUN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sidereal_time(mut v_jd: Py, mut v_lon: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_gmst: Py = Py::None;
    let mut v_lmst: Py = Py::None;
    let mut v_sec: Py = Py::None;
    let mut v_t: Py = Py::None;
    if (py::not(&{ let t1863 = Py::Float(2299160.5); Py::Bool({ let t1864 = v_jd.clone(); if py::le(&t1863, &t1864)? { { let t1865 = Py::Float(5373484.5); if py::le(&t1864, &t1865)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Julian date must lie between 2299160.5 and 5373484.5"))));
    }
    if (py::not(&{ let t1866 = py::neg(&Py::Float(std::f64::consts::PI))?; Py::Bool({ let t1867 = v_lon.clone(); if py::le(&t1866, &t1867)? { { let t1868 = py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?; if py::le(&t1867, &t1868)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the longitude must lie between -180 and 360 degrees (east positive)"))));
    }
    v_d = py::fraction(&[v_jd.clone()])?;
    v_t = py::truediv(&py::sub(&v_d.clone(), &Py::Int(2451545i64))?, &Py::Int(36525i64))?;
    v_f = py::mul(&Py::Int(86400i64), &py::sub(&v_d.clone(), &py::floordiv(&py::numerator(&v_d.clone())?, &py::denominator(&v_d.clone())?)?)?)?;
    v_sec = py::add(&py::add(&py::sub(&py::fraction(&[Py::Float(24110.54841)])?, &Py::Int(43200i64))?, &py::mul(&py::add(&py::fraction(&[Py::Float(8640184.812866)])?, &py::mul(&py::add(&py::fraction(&[Py::Float(0.093104)])?, &py::mul(&py::fraction(&[py::neg(&Py::Float(6.2e-06))?])?, &v_t.clone())?)?, &v_t.clone())?)?, &v_t.clone())?)?, &v_f.clone())?;
    v_gmst = py::truediv(&py::float(&py::modulo(&v_sec.clone(), &Py::Int(86400i64))?)?, &Py::Int(3600i64))?;
    v_lmst = py::modulo(&py::add(&v_gmst.clone(), &py::truediv(&py::truediv(&v_lon.clone(), &c_DEG()?)?, &Py::Int(15i64))?)?, &Py::Int(24i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("gmst_h"), v_gmst.clone()), (Py::str("lmst_h"), v_lmst.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stellar_luminosity(mut v_R: Py, mut v_T: Py) -> R {
    let mut v_L: Py = Py::None;
    let _ = f__pos(Py::str("the radius and the temperature"), Py::tuple(vec![v_R.clone(), v_T.clone()]))?;
    v_L = py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_R.clone())?, &v_R.clone())?, &c_SIGMA()?)?, &py::pow(&v_T.clone(), &Py::Int(4i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("L_Lsun"), py::truediv(&v_L.clone(), &c_LSUN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stellar_radius(mut v_L: Py, mut v_T: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("the luminosity and the temperature"), Py::tuple(vec![v_L.clone(), v_T.clone()]))?;
    v_R = py::truediv(&py::math1("sqrt", &py::truediv(&v_L.clone(), &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &c_SIGMA()?)?)?)?, &py::mul(&v_T.clone(), &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_Rsun"), py::truediv(&v_R.clone(), &c_RSUN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_telescope_magnification(mut v_f_obj: Py, mut v_f_eye: Py, mut v_D: Py, mut v_afov: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the focal lengths and the aperture"), Py::tuple(vec![v_f_obj.clone(), v_f_eye.clone(), v_D.clone()]))?;
    v_M = py::truediv(&v_f_obj.clone(), &v_f_eye.clone())?;
    v_out = Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("exit_pupil"), py::truediv(&v_D.clone(), &v_M.clone())?), (Py::str("exit_pupil_mm"), py::mul(&py::truediv(&v_D.clone(), &v_M.clone())?, &Py::Float(1000.0))?)]);
    if ({ let t1953 = v_afov.clone(); Py::Bool({ let t1954 = Py::None; if !py::is(&t1953, &t1954) { true } else { false } }) }).truthy() {
        if (py::not(&{ let t1955 = Py::Int(0i64); Py::Bool({ let t1956 = v_afov.clone(); if py::lt(&t1955, &t1956)? { { let t1957 = Py::Float(std::f64::consts::PI); if py::lt(&t1956, &t1957)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the apparent field of view must lie between 0 and 180 degrees"))));
        }
        py::setitem(&v_out.clone(), &Py::str("tfov_deg"), py::truediv(&py::truediv(&v_afov.clone(), &v_M.clone())?, &c_DEG()?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thermal_speed(mut v_T: Py, mut v_convention: Py, mut v_particle: Py) -> R {
    let _ = f__pos(Py::str("the temperature"), Py::tuple(vec![v_T.clone()]))?;
    if ({ let t1987 = v_convention.clone(); Py::Bool({ let t1988 = c__CONV()?; if !py::contains(&t1988, &t1987)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the convention must be most_probable, rms or mean"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("v_th"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&py::getitem(&py::getitem(&c__CONV()?, &v_convention.clone())?, &Py::Int(0i64))?, &c_KB()?)?, &v_T.clone())?, &f__mass(v_particle.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_time_of_flight(mut v_M: Py, mut v_a: Py, mut v_e: Py, mut v_nu1: Py, mut v_nu2: Py) -> R {
    let mut v_dM: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let _ = f__pos(Py::str("the central mass and the semi-major axis"), Py::tuple(vec![v_M.clone(), v_a.clone()]))?;
    if (py::not(&{ let t2019 = Py::Int(0i64); Py::Bool({ let t2020 = v_e.clone(); if py::le(&t2019, &t2020)? { { let t2021 = Py::Float(0.99); if py::lt(&t2020, &t2021)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the eccentricity must satisfy 0 <= e < 0.99 (an elliptical orbit)"))));
    }
    let t2022 = (Py::tuple(vec![py::modulo(&v_nu1.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, py::modulo(&v_nu2.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?])).items()?;
    if t2022.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t2022[0].clone();
    v_n2 = t2022[1].clone();
    if ({ let t2023 = py::abs(&py::sub(&v_n1.clone(), &v_n2.clone())?)?; Py::Bool({ let t2024 = Py::Float(1e-12); if py::lt(&t2023, &t2024)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two true anomalies must differ"))));
    }
    v_mu = py::mul(&c_G()?, &v_M.clone())?;
    v_n = py::math1("sqrt", &py::truediv(&v_mu.clone(), &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?;
    v_mean = { let v_M = v_M.clone(); let v_a = v_a.clone(); let v_e = v_e.clone(); let v_mean = v_mean.clone(); let v_mu = v_mu.clone(); let v_n = v_n.clone(); let v_n1 = v_n1.clone(); let v_n2 = v_n2.clone(); let v_nu1 = v_nu1.clone(); let v_nu2 = v_nu2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_nu: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_E: Py = Py::None;
            v_E = py::modulo(&py::atan2(&py::mul(&py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_e.clone())?)?)?, &py::math1("sin", &v_nu.clone())?)?, &py::add(&v_e.clone(), &py::math1("cos", &v_nu.clone())?)?)?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
            return Ok(py::sub(&v_E.clone(), &py::mul(&v_e.clone(), &py::math1("sin", &v_E.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_dM = py::sub(&py::call(&v_mean.clone(), &[v_n2.clone()])?, &py::call(&v_mean.clone(), &[v_n1.clone()])?)?;
    if ({ let t2025 = v_n2.clone(); Py::Bool({ let t2026 = v_n1.clone(); if py::lt(&t2025, &t2026)? { true } else { false } }) }).truthy() {
        v_dM = py::iadd(&v_dM, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    } else {
        if ({ let t2027 = v_dM.clone(); Py::Bool({ let t2028 = Py::Int(0i64); if py::lt(&t2027, &t2028)? { true } else { false } }) }).truthy() {
            v_dM = py::iadd(&v_dM, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("dt"), py::truediv(&v_dM.clone(), &v_n.clone())?), (Py::str("dt_days"), py::truediv(&py::truediv(&v_dM.clone(), &v_n.clone())?, &Py::Int(86400i64))?), (Py::str("P"), py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transit_depth(mut v_Rp: Py, mut v_Rs: Py, mut v_b: Py) -> R {
    let mut v_depth: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("the two radii"), Py::tuple(vec![v_Rp.clone(), v_Rs.clone()]))?;
    v_k = py::truediv(&v_Rp.clone(), &v_Rs.clone())?;
    if ({ let t2057 = v_k.clone(); Py::Bool({ let t2058 = Py::Int(1i64); if py::ge(&t2057, &t2058)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the planet must be smaller than the star"))));
    }
    if (py::not(&{ let t2059 = Py::Int(0i64); Py::Bool({ let t2060 = v_b.clone(); if py::le(&t2059, &t2060)? { { let t2061 = py::add(&Py::Int(1i64), &v_k.clone())?; if py::lt(&t2060, &t2061)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no transit: the impact parameter must satisfy 0 <= b < 1 + Rp/Rs"))));
    }
    v_depth = py::truediv(&f__overlap(v_k.clone(), v_b.clone())?, &Py::Float(std::f64::consts::PI))?;
    return Ok(f__out(Py::dict(vec![(Py::str("depth"), v_depth.clone()), (Py::str("depth_ppm"), py::mul(&v_depth.clone(), &Py::Float(1000000.0))?), (Py::str("k"), v_k.clone()), (Py::str("grazing"), { let t2074 = v_b.clone(); Py::Bool({ let t2075 = py::sub(&Py::Int(1i64), &v_k.clone())?; if py::gt(&t2074, &t2075)? { true } else { false } }) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transit_duration(mut v_P: Py, mut v_a: Py, mut v_Rs: Py, mut v_Rp: Py, mut v_i: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the period, the orbit radius and the two radii"), Py::tuple(vec![v_P.clone(), v_a.clone(), v_Rs.clone(), v_Rp.clone()]))?;
    if ({ let t2096 = v_Rp.clone(); Py::Bool({ let t2097 = v_Rs.clone(); if py::ge(&t2096, &t2097)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the planet must be smaller than the star"))));
    }
    if ({ let t2098 = v_a.clone(); Py::Bool({ let t2099 = py::add(&v_Rs.clone(), &v_Rp.clone())?; if py::le(&t2098, &t2099)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the orbit radius must exceed the sum of the two radii"))));
    }
    if (py::not(&{ let t2100 = Py::Int(0i64); Py::Bool({ let t2101 = v_i.clone(); if py::lt(&t2100, &t2101)? { { let t2102 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t2101, &t2102)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inclination must lie between 0 (excluded) and 90 degrees"))));
    }
    v_b = py::truediv(&py::mul(&v_a.clone(), &py::math1("cos", &v_i.clone())?)?, &v_Rs.clone())?;
    v_b = (if ({ let t2103 = v_b.clone(); Py::Bool({ let t2104 = Py::Float(1e-12); if py::lt(&t2103, &t2104)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_b.clone() });
    v_k = py::truediv(&v_Rp.clone(), &v_Rs.clone())?;
    if ({ let t2105 = v_b.clone(); Py::Bool({ let t2106 = py::add(&Py::Int(1i64), &v_k.clone())?; if py::ge(&t2105, &t2106)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the planet does not transit (impact parameter b >= 1 + Rp/Rs)"))));
    }
    v_out = Py::dict(vec![(Py::str("T14"), py::mul(&py::truediv(&v_P.clone(), &Py::Float(std::f64::consts::PI))?, &py::math1("asin", &py::truediv(&py::math1("sqrt", &py::sub(&py::pow(&py::add(&v_Rs.clone(), &v_Rp.clone())?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_b.clone(), &v_Rs.clone())?, &Py::Int(2i64))?)?)?, &py::mul(&v_a.clone(), &py::math1("sin", &v_i.clone())?)?)?)?)?)]);
    if ({ let t2107 = v_b.clone(); Py::Bool({ let t2108 = py::sub(&Py::Int(1i64), &v_k.clone())?; if py::lt(&t2107, &t2108)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("T23"), py::mul(&py::truediv(&v_P.clone(), &Py::Float(std::f64::consts::PI))?, &py::math1("asin", &py::truediv(&py::math1("sqrt", &py::sub(&py::pow(&py::sub(&v_Rs.clone(), &v_Rp.clone())?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_b.clone(), &v_Rs.clone())?, &Py::Int(2i64))?)?)?, &py::mul(&v_a.clone(), &py::math1("sin", &v_i.clone())?)?)?)?)?)?;
    }
    return Ok({ let mut v: Vec<Py> = vec![];  let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_out.clone())?; d.push((Py::str("T14_h"), py::truediv(&py::getitem(&v_out.clone(), &Py::str("T14"))?, &Py::Int(3600i64))?)); d.push((Py::str("b"), v_b.clone()));  if v.len() > 0 { return Err(PyErr::Type("_out() takes fewer positional arguments".into())) } f__out(Py::dict(d))? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_upper_hybrid(mut v_B: Py, mut v_n: Py) -> R {
    let mut v_w: Py = Py::None;
    let mut v_wc: Py = Py::None;
    let mut v_wp2: Py = Py::None;
    let _ = f__pos(Py::str("the field and the density"), Py::tuple(vec![v_B.clone(), v_n.clone()]))?;
    v_wp2 = py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_QE()?)?, &c_QE()?)?, &py::mul(&c_EPS0()?, &c_ME()?)?)?;
    v_wc = py::truediv(&py::mul(&c_QE()?, &v_B.clone())?, &c_ME()?)?;
    v_w = py::math1("sqrt", &py::add(&v_wp2.clone(), &py::mul(&v_wc.clone(), &v_wc.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_uh"), v_w.clone()), (Py::str("f_uh"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1013(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1015: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1014 = v_inputs.clone(); f_hohmann(py::getitem(&t1014, &Py::str("r1"))?, py::getitem(&t1014, &Py::str("r2"))?, py::getitem(&t1014, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1015 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1016 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1028 = (t1016).items()?;
        if t1028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1028[0].clone();
        v_v = t1028[1].clone();
        if ({ let t1017 = v_k.clone(); Py::Bool({ let t1018 = v_outputs.clone(); if !py::contains(&t1018, &t1017)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1043(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1045: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1044 = v_inputs.clone(); f_inertial_length(py::getitem(&t1044, &Py::str("n"))?, py::method(&t1044, "get", &[Py::str("particle"), Py::str("electron")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1045 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1046 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1058 = (t1046).items()?;
        if t1058.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1058[0].clone();
        v_v = t1058[1].clone();
        if ({ let t1047 = v_k.clone(); Py::Bool({ let t1048 = v_outputs.clone(); if !py::contains(&t1048, &t1047)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1114(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1116: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1115 = v_inputs.clone(); f_julian_date(py::getitem(&t1115, &Py::str("year"))?, py::getitem(&t1115, &Py::str("month"))?, py::getitem(&t1115, &Py::str("day"))?, py::method(&t1115, "get", &[Py::str("hour"), Py::Int(0i64)])?, py::method(&t1115, "get", &[Py::str("minute"), Py::Int(0i64)])?, py::method(&t1115, "get", &[Py::str("second"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1116 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1117 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1129 = (t1117).items()?;
        if t1129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1129[0].clone();
        v_v = t1129[1].clone();
        if ({ let t1118 = v_k.clone(); Py::Bool({ let t1119 = v_outputs.clone(); if !py::contains(&t1119, &t1118)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_115(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t117: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t116 = v_inputs.clone(); f_alfven_speed(py::getitem(&t116, &Py::str("B"))?, py::getitem(&t116, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t117 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t118 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t130 = (t118).items()?;
        if t130.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t130[0].clone();
        v_v = t130[1].clone();
        if ({ let t119 = v_k.clone(); Py::Bool({ let t120 = v_outputs.clone(); if !py::contains(&t120, &t119)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1158(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1160: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1159 = v_inputs.clone(); f_kepler_position(py::getitem(&t1159, &Py::str("M"))?, py::getitem(&t1159, &Py::str("a"))?, py::getitem(&t1159, &Py::str("e"))?, py::getitem(&t1159, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1160 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1161 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1173 = (t1161).items()?;
        if t1173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1173[0].clone();
        v_v = t1173[1].clone();
        if ({ let t1162 = v_k.clone(); Py::Bool({ let t1163 = v_outputs.clone(); if !py::contains(&t1163, &t1162)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1186(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1188: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1187 = v_inputs.clone(); f_light_gathering(py::getitem(&t1187, &Py::str("D1"))?, py::getitem(&t1187, &Py::str("D2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1188 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1189 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1201 = (t1189).items()?;
        if t1201.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1201[0].clone();
        v_v = t1201[1].clone();
        if ({ let t1190 = v_k.clone(); Py::Bool({ let t1191 = v_outputs.clone(); if !py::contains(&t1191, &t1190)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1216(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1218: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1217 = v_inputs.clone(); f_linear_size(py::getitem(&t1217, &Py::str("theta"))?, py::getitem(&t1217, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1218 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1219 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1231 = (t1219).items()?;
        if t1231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1231[0].clone();
        v_v = t1231[1].clone();
        if ({ let t1220 = v_k.clone(); Py::Bool({ let t1221 = v_outputs.clone(); if !py::contains(&t1221, &t1220)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1254(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1256: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1255 = v_inputs.clone(); f_lookback_time(py::getitem(&t1255, &Py::str("z"))?, py::getitem(&t1255, &Py::str("H0"))?, py::getitem(&t1255, &Py::str("Om"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1256 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1257 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1269 = (t1257).items()?;
        if t1269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1269[0].clone();
        v_v = t1269[1].clone();
        if ({ let t1258 = v_k.clone(); Py::Bool({ let t1259 = v_outputs.clone(); if !py::contains(&t1259, &t1258)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1283(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1285: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1284 = v_inputs.clone(); f_lower_hybrid(py::getitem(&t1284, &Py::str("B"))?, py::getitem(&t1284, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1285 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1286 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1298 = (t1286).items()?;
        if t1298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1298[0].clone();
        v_v = t1298[1].clone();
        if ({ let t1287 = v_k.clone(); Py::Bool({ let t1288 = v_outputs.clone(); if !py::contains(&t1288, &t1287)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1310(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1312: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1311 = v_inputs.clone(); f_luminosity_from_magnitude(py::getitem(&t1311, &Py::str("M_bol"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1312 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1313 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1325 = (t1313).items()?;
        if t1325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1325[0].clone();
        v_v = t1325[1].clone();
        if ({ let t1314 = v_k.clone(); Py::Bool({ let t1315 = v_outputs.clone(); if !py::contains(&t1315, &t1314)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1338(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1340: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1339 = v_inputs.clone(); f_magnetic_pressure(py::getitem(&t1339, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1340 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1341 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1353 = (t1341).items()?;
        if t1353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1353[0].clone();
        v_v = t1353[1].clone();
        if ({ let t1342 = v_k.clone(); Py::Bool({ let t1343 = v_outputs.clone(); if !py::contains(&t1343, &t1342)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1366(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1368: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1367 = v_inputs.clone(); f_magnitude_difference(py::getitem(&t1367, &Py::str("ratio"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1368 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1369 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1381 = (t1369).items()?;
        if t1381.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1381[0].clone();
        v_v = t1381[1].clone();
        if ({ let t1370 = v_k.clone(); Py::Bool({ let t1371 = v_outputs.clone(); if !py::contains(&t1371, &t1370)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1401(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1403: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1402 = v_inputs.clone(); f_orbit_from_state(py::getitem(&t1402, &Py::str("M"))?, py::getitem(&t1402, &Py::str("r"))?, py::getitem(&t1402, &Py::str("v"))?, py::method(&t1402, "get", &[Py::str("gamma"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1403 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1404 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1416 = (t1404).items()?;
        if t1416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1416[0].clone();
        v_v = t1416[1].clone();
        if ({ let t1405 = v_k.clone(); Py::Bool({ let t1406 = v_outputs.clone(); if !py::contains(&t1406, &t1405)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1429(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1431: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1430 = v_inputs.clone(); f_parallax_distance(py::getitem(&t1430, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1431 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1432 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1444 = (t1432).items()?;
        if t1444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1444[0].clone();
        v_v = t1444[1].clone();
        if ({ let t1433 = v_k.clone(); Py::Bool({ let t1434 = v_outputs.clone(); if !py::contains(&t1434, &t1433)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1460(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1462: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1461 = v_inputs.clone(); f_plane_change(py::getitem(&t1461, &Py::str("v1"))?, py::getitem(&t1461, &Py::str("v2"))?, py::getitem(&t1461, &Py::str("di"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1462 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1463 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1475 = (t1463).items()?;
        if t1475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1475[0].clone();
        v_v = t1475[1].clone();
        if ({ let t1464 = v_k.clone(); Py::Bool({ let t1465 = v_outputs.clone(); if !py::contains(&t1465, &t1464)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1493(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1494 = v_inputs.clone(); f_planet_equilibrium(py::getitem(&t1494, &Py::str("T_star"))?, py::getitem(&t1494, &Py::str("R_star"))?, py::getitem(&t1494, &Py::str("a"))?, py::method(&t1494, "get", &[Py::str("A"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1495 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1496 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1508 = (t1496).items()?;
        if t1508.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1508[0].clone();
        v_v = t1508[1].clone();
        if ({ let t1497 = v_k.clone(); Py::Bool({ let t1498 = v_outputs.clone(); if !py::contains(&t1498, &t1497)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_15(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t17: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t16 = v_inputs.clone(); f_absolute_magnitude(py::getitem(&t16, &Py::str("m"))?, py::getitem(&t16, &Py::str("d"))?, py::method(&t16, "get", &[Py::str("A"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t17 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t18 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t30 = (t18).items()?;
        if t30.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t30[0].clone();
        v_v = t30[1].clone();
        if ({ let t19 = v_k.clone(); Py::Bool({ let t20 = v_outputs.clone(); if !py::contains(&t20, &t19)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1522(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1524: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1523 = v_inputs.clone(); f_plasma_beta(py::getitem(&t1523, &Py::str("T"))?, py::getitem(&t1523, &Py::str("n"))?, py::getitem(&t1523, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1524 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1525 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1537 = (t1525).items()?;
        if t1537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1537[0].clone();
        v_v = t1537[1].clone();
        if ({ let t1526 = v_k.clone(); Py::Bool({ let t1527 = v_outputs.clone(); if !py::contains(&t1527, &t1526)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_153(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t154 = v_inputs.clone(); f_altaz_from_hadec(py::getitem(&t154, &Py::str("H"))?, py::getitem(&t154, &Py::str("dec"))?, py::getitem(&t154, &Py::str("lat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t156 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t168 = (t156).items()?;
        if t168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t168[0].clone();
        v_v = t168[1].clone();
        if ({ let t157 = v_k.clone(); Py::Bool({ let t158 = v_outputs.clone(); if !py::contains(&t158, &t157)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1552(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1554: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1553 = v_inputs.clone(); f_plasma_frequency(py::getitem(&t1553, &Py::str("n"))?, py::method(&t1553, "get", &[Py::str("particle"), Py::str("electron")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1554 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1555 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1567 = (t1555).items()?;
        if t1567.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1567[0].clone();
        v_v = t1567[1].clone();
        if ({ let t1556 = v_k.clone(); Py::Bool({ let t1557 = v_outputs.clone(); if !py::contains(&t1557, &t1556)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1584(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1586: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1585 = v_inputs.clone(); f_position_angle(py::getitem(&t1585, &Py::str("ra1"))?, py::getitem(&t1585, &Py::str("dec1"))?, py::getitem(&t1585, &Py::str("ra2"))?, py::getitem(&t1585, &Py::str("dec2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1586 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1587 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1599 = (t1587).items()?;
        if t1599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1599[0].clone();
        v_v = t1599[1].clone();
        if ({ let t1588 = v_k.clone(); Py::Bool({ let t1589 = v_outputs.clone(); if !py::contains(&t1589, &t1588)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1618(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1620: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1619 = v_inputs.clone(); f_rv_semi_amplitude(py::getitem(&t1619, &Py::str("M_star"))?, py::getitem(&t1619, &Py::str("m_p"))?, py::getitem(&t1619, &Py::str("P"))?, py::method(&t1619, "get", &[Py::str("e"), Py::Float(0.0)])?, py::method(&t1619, "get", &[Py::str("i"), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1620 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1621 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1633 = (t1621).items()?;
        if t1633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1633[0].clone();
        v_v = t1633[1].clone();
        if ({ let t1622 = v_k.clone(); Py::Bool({ let t1623 = v_outputs.clone(); if !py::contains(&t1623, &t1622)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1648(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1650: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1649 = v_inputs.clone(); f_rayleigh_resolution(py::getitem(&t1649, &Py::str("lam"))?, py::getitem(&t1649, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1650 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1651 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1663 = (t1651).items()?;
        if t1663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1663[0].clone();
        v_v = t1663[1].clone();
        if ({ let t1652 = v_k.clone(); Py::Bool({ let t1653 = v_outputs.clone(); if !py::contains(&t1653, &t1652)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1675(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1677: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1676 = v_inputs.clone(); f_recession_velocity(py::getitem(&t1676, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1677 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1678 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1690 = (t1678).items()?;
        if t1690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1690[0].clone();
        v_v = t1690[1].clone();
        if ({ let t1679 = v_k.clone(); Py::Bool({ let t1680 = v_outputs.clone(); if !py::contains(&t1680, &t1679)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1721(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1723: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1722 = v_inputs.clone(); f_redshift_at_lookback(py::getitem(&t1722, &Py::str("t"))?, py::getitem(&t1722, &Py::str("H0"))?, py::getitem(&t1722, &Py::str("Om"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1723 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1724 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1736 = (t1724).items()?;
        if t1736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1736[0].clone();
        v_v = t1736[1].clone();
        if ({ let t1725 = v_k.clone(); Py::Bool({ let t1726 = v_outputs.clone(); if !py::contains(&t1726, &t1725)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1749(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1751: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1750 = v_inputs.clone(); f_redshift_from_lines(py::getitem(&t1750, &Py::str("lam_obs"))?, py::getitem(&t1750, &Py::str("lam_rest"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1751 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1752 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1764 = (t1752).items()?;
        if t1764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1764[0].clone();
        v_v = t1764[1].clone();
        if ({ let t1753 = v_k.clone(); Py::Bool({ let t1754 = v_outputs.clone(); if !py::contains(&t1754, &t1753)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1785(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1787: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1786 = v_inputs.clone(); f_rise_set(py::getitem(&t1786, &Py::str("dec"))?, py::getitem(&t1786, &Py::str("lat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1787 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1788 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1800 = (t1788).items()?;
        if t1800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1800[0].clone();
        v_v = t1800[1].clone();
        if ({ let t1789 = v_k.clone(); Py::Bool({ let t1790 = v_outputs.clone(); if !py::contains(&t1790, &t1789)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_181(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t183: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t182 = v_inputs.clone(); f_angular_separation(py::getitem(&t182, &Py::str("ra1"))?, py::getitem(&t182, &Py::str("dec1"))?, py::getitem(&t182, &Py::str("ra2"))?, py::getitem(&t182, &Py::str("dec2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t183 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t184 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t196 = (t184).items()?;
        if t196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t196[0].clone();
        v_v = t196[1].clone();
        if ({ let t185 = v_k.clone(); Py::Bool({ let t186 = v_outputs.clone(); if !py::contains(&t186, &t185)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1819(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1821: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1820 = v_inputs.clone(); f_saha_hydrogen(py::getitem(&t1820, &Py::str("T"))?, py::getitem(&t1820, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1821 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1822 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1834 = (t1822).items()?;
        if t1834.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1834[0].clone();
        v_v = t1834[1].clone();
        if ({ let t1823 = v_k.clone(); Py::Bool({ let t1824 = v_outputs.clone(); if !py::contains(&t1824, &t1823)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1847(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1849: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1848 = v_inputs.clone(); f_schwarzschild_radius(py::getitem(&t1848, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1849 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1850 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1862 = (t1850).items()?;
        if t1862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1862[0].clone();
        v_v = t1862[1].clone();
        if ({ let t1851 = v_k.clone(); Py::Bool({ let t1852 = v_outputs.clone(); if !py::contains(&t1852, &t1851)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1877(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1879: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1878 = v_inputs.clone(); f_sidereal_time(py::getitem(&t1878, &Py::str("jd"))?, py::method(&t1878, "get", &[Py::str("lon"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1879 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1880 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1892 = (t1880).items()?;
        if t1892.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1892[0].clone();
        v_v = t1892[1].clone();
        if ({ let t1881 = v_k.clone(); Py::Bool({ let t1882 = v_outputs.clone(); if !py::contains(&t1882, &t1881)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1905(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1906 = v_inputs.clone(); f_stellar_luminosity(py::getitem(&t1906, &Py::str("R"))?, py::getitem(&t1906, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1907 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1908 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1920 = (t1908).items()?;
        if t1920.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1920[0].clone();
        v_v = t1920[1].clone();
        if ({ let t1909 = v_k.clone(); Py::Bool({ let t1910 = v_outputs.clone(); if !py::contains(&t1910, &t1909)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1933(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1935: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1934 = v_inputs.clone(); f_stellar_radius(py::getitem(&t1934, &Py::str("L"))?, py::getitem(&t1934, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1935 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1936 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1948 = (t1936).items()?;
        if t1948.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1948[0].clone();
        v_v = t1948[1].clone();
        if ({ let t1937 = v_k.clone(); Py::Bool({ let t1938 = v_outputs.clone(); if !py::contains(&t1938, &t1937)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1967(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1969: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1968 = v_inputs.clone(); f_telescope_magnification(py::getitem(&t1968, &Py::str("f_obj"))?, py::getitem(&t1968, &Py::str("f_eye"))?, py::getitem(&t1968, &Py::str("D"))?, py::method(&t1968, "get", &[Py::str("afov"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1969 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1970 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1982 = (t1970).items()?;
        if t1982.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1982[0].clone();
        v_v = t1982[1].clone();
        if ({ let t1971 = v_k.clone(); Py::Bool({ let t1972 = v_outputs.clone(); if !py::contains(&t1972, &t1971)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1999(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2001: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2000 = v_inputs.clone(); f_thermal_speed(py::getitem(&t2000, &Py::str("T"))?, py::getitem(&t2000, &Py::str("convention"))?, py::method(&t2000, "get", &[Py::str("particle"), Py::str("electron")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2001 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2002 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2014 = (t2002).items()?;
        if t2014.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2014[0].clone();
        v_v = t2014[1].clone();
        if ({ let t2003 = v_k.clone(); Py::Bool({ let t2004 = v_outputs.clone(); if !py::contains(&t2004, &t2003)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2037(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2039: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2038 = v_inputs.clone(); f_time_of_flight(py::getitem(&t2038, &Py::str("M"))?, py::getitem(&t2038, &Py::str("a"))?, py::getitem(&t2038, &Py::str("e"))?, py::getitem(&t2038, &Py::str("nu1"))?, py::getitem(&t2038, &Py::str("nu2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2039 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2040 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2052 = (t2040).items()?;
        if t2052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2052[0].clone();
        v_v = t2052[1].clone();
        if ({ let t2041 = v_k.clone(); Py::Bool({ let t2042 = v_outputs.clone(); if !py::contains(&t2042, &t2041)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2076(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2078: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2077 = v_inputs.clone(); f_transit_depth(py::getitem(&t2077, &Py::str("Rp"))?, py::getitem(&t2077, &Py::str("Rs"))?, py::method(&t2077, "get", &[Py::str("b"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2078 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2079 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2091 = (t2079).items()?;
        if t2091.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2091[0].clone();
        v_v = t2091[1].clone();
        if ({ let t2080 = v_k.clone(); Py::Bool({ let t2081 = v_outputs.clone(); if !py::contains(&t2081, &t2080)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_209(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t211: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t210 = v_inputs.clone(); f_angular_size(py::getitem(&t210, &Py::str("D"))?, py::getitem(&t210, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t211 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t212 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t224 = (t212).items()?;
        if t224.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t224[0].clone();
        v_v = t224[1].clone();
        if ({ let t213 = v_k.clone(); Py::Bool({ let t214 = v_outputs.clone(); if !py::contains(&t214, &t213)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2117(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2119: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2118 = v_inputs.clone(); f_transit_duration(py::getitem(&t2118, &Py::str("P"))?, py::getitem(&t2118, &Py::str("a"))?, py::getitem(&t2118, &Py::str("Rs"))?, py::getitem(&t2118, &Py::str("Rp"))?, py::getitem(&t2118, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2119 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2120 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2132 = (t2120).items()?;
        if t2132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2132[0].clone();
        v_v = t2132[1].clone();
        if ({ let t2121 = v_k.clone(); Py::Bool({ let t2122 = v_outputs.clone(); if !py::contains(&t2122, &t2121)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2145(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2147: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2146 = v_inputs.clone(); f_upper_hybrid(py::getitem(&t2146, &Py::str("B"))?, py::getitem(&t2146, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2147 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2148 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2160 = (t2148).items()?;
        if t2160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2160[0].clone();
        v_v = t2160[1].clone();
        if ({ let t2149 = v_k.clone(); Py::Bool({ let t2150 = v_outputs.clone(); if !py::contains(&t2150, &t2149)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t255: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t254 = v_inputs.clone(); f_angular_size_at_z(py::getitem(&t254, &Py::str("z"))?, py::getitem(&t254, &Py::str("H0"))?, py::getitem(&t254, &Py::str("Om"))?, py::getitem(&t254, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t255 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t256 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t268 = (t256).items()?;
        if t268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t268[0].clone();
        v_v = t268[1].clone();
        if ({ let t257 = v_k.clone(); Py::Bool({ let t258 = v_outputs.clone(); if !py::contains(&t258, &t257)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_281(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t283: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t282 = v_inputs.clone(); f_apparent_bolometric(py::getitem(&t282, &Py::str("L"))?, py::getitem(&t282, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t283 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t284 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t296 = (t284).items()?;
        if t296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t296[0].clone();
        v_v = t296[1].clone();
        if ({ let t285 = v_k.clone(); Py::Bool({ let t286 = v_outputs.clone(); if !py::contains(&t286, &t285)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_311(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t313: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t312 = v_inputs.clone(); f_apsis_speeds(py::getitem(&t312, &Py::str("rp"))?, py::getitem(&t312, &Py::str("ra"))?, py::getitem(&t312, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t313 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t314 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t326 = (t314).items()?;
        if t326.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t326[0].clone();
        v_v = t326[1].clone();
        if ({ let t315 = v_k.clone(); Py::Bool({ let t316 = v_outputs.clone(); if !py::contains(&t316, &t315)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_339(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t341: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t340 = v_inputs.clone(); f_scale_height(py::getitem(&t340, &Py::str("T"))?, py::getitem(&t340, &Py::str("mu"))?, py::getitem(&t340, &Py::str("g"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t341 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t342 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t354 = (t342).items()?;
        if t354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t354[0].clone();
        v_v = t354[1].clone();
        if ({ let t343 = v_k.clone(); Py::Bool({ let t344 = v_outputs.clone(); if !py::contains(&t344, &t343)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_367(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t369: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t368 = v_inputs.clone(); f_binary_masses(py::getitem(&t368, &Py::str("P"))?, py::getitem(&t368, &Py::str("a1"))?, py::getitem(&t368, &Py::str("a2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t369 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t370 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t382 = (t370).items()?;
        if t382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t382[0].clone();
        v_v = t382[1].clone();
        if ({ let t371 = v_k.clone(); Py::Bool({ let t372 = v_outputs.clone(); if !py::contains(&t372, &t371)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_395(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t397: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t396 = v_inputs.clone(); f_bolometric_magnitude(py::getitem(&t396, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t397 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t398 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t410 = (t398).items()?;
        if t410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t410[0].clone();
        v_v = t410[1].clone();
        if ({ let t399 = v_k.clone(); Py::Bool({ let t400 = v_outputs.clone(); if !py::contains(&t400, &t399)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t428: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t427 = v_inputs.clone(); f_calendar_from_jd(py::getitem(&t427, &Py::str("jd"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t428 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t429 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t441 = (t429).items()?;
        if t441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t441[0].clone();
        v_v = t441[1].clone();
        if ({ let t430 = v_k.clone(); Py::Bool({ let t431 = v_outputs.clone(); if !py::contains(&t431, &t430)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_459(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t461: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t460 = v_inputs.clone(); f_camera_pixel_scale(py::getitem(&t460, &Py::str("f"))?, py::getitem(&t460, &Py::str("p"))?, py::method(&t460, "get", &[Py::str("N"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t461 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t462 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t474 = (t462).items()?;
        if t474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t474[0].clone();
        v_v = t474[1].clone();
        if ({ let t463 = v_k.clone(); Py::Bool({ let t464 = v_outputs.clone(); if !py::contains(&t464, &t463)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_485(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t487: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t486 = v_inputs.clone(); f_combined_magnitude(py::getitem(&t486, &Py::str("m1"))?, py::getitem(&t486, &Py::str("m2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t487 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t488 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t500 = (t488).items()?;
        if t500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t500[0].clone();
        v_v = t500[1].clone();
        if ({ let t489 = v_k.clone(); Py::Bool({ let t490 = v_outputs.clone(); if !py::contains(&t490, &t489)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_51(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t53: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t52 = v_inputs.clone(); f_age_at_z(py::getitem(&t52, &Py::str("z"))?, py::getitem(&t52, &Py::str("H0"))?, py::getitem(&t52, &Py::str("Om"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t53 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t54 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t66 = (t54).items()?;
        if t66.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t66[0].clone();
        v_v = t66[1].clone();
        if ({ let t55 = v_k.clone(); Py::Bool({ let t56 = v_outputs.clone(); if !py::contains(&t56, &t55)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_525(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t527: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t526 = v_inputs.clone(); f_comoving_volume(py::getitem(&t526, &Py::str("z"))?, py::getitem(&t526, &Py::str("H0"))?, py::getitem(&t526, &Py::str("Om"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t527 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t528 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t540 = (t528).items()?;
        if t540.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t540[0].clone();
        v_v = t540[1].clone();
        if ({ let t529 = v_k.clone(); Py::Bool({ let t530 = v_outputs.clone(); if !py::contains(&t530, &t529)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_565(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t566 = v_inputs.clone(); f_cosmo_distances(py::getitem(&t566, &Py::str("z"))?, py::getitem(&t566, &Py::str("H0"))?, py::getitem(&t566, &Py::str("Om"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t567 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t568 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t580 = (t568).items()?;
        if t580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t580[0].clone();
        v_v = t580[1].clone();
        if ({ let t569 = v_k.clone(); Py::Bool({ let t570 = v_outputs.clone(); if !py::contains(&t570, &t569)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_601(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t603: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t602 = v_inputs.clone(); f_critical_density(py::getitem(&t602, &Py::str("H0"))?, py::getitem(&t602, &Py::str("Om"))?, py::method(&t602, "get", &[Py::str("z"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t603 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t604 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t616 = (t604).items()?;
        if t616.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t616[0].clone();
        v_v = t616[1].clone();
        if ({ let t605 = v_k.clone(); Py::Bool({ let t606 = v_outputs.clone(); if !py::contains(&t606, &t605)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_636(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t638: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t637 = v_inputs.clone(); f_culmination(py::getitem(&t637, &Py::str("dec"))?, py::getitem(&t637, &Py::str("lat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t638 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t639 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t651 = (t639).items()?;
        if t651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t651[0].clone();
        v_v = t651[1].clone();
        if ({ let t640 = v_k.clone(); Py::Bool({ let t641 = v_outputs.clone(); if !py::contains(&t641, &t640)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_664(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t666: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t665 = v_inputs.clone(); f_debye_length(py::getitem(&t665, &Py::str("T"))?, py::getitem(&t665, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t666 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t667 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t679 = (t667).items()?;
        if t679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t679[0].clone();
        v_v = t679[1].clone();
        if ({ let t668 = v_k.clone(); Py::Bool({ let t669 = v_outputs.clone(); if !py::contains(&t669, &t668)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_692(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t694: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t693 = v_inputs.clone(); f_debye_number(py::getitem(&t693, &Py::str("T"))?, py::getitem(&t693, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t694 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t695 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t707 = (t695).items()?;
        if t707.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t707[0].clone();
        v_v = t707[1].clone();
        if ({ let t696 = v_k.clone(); Py::Bool({ let t697 = v_outputs.clone(); if !py::contains(&t697, &t696)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_721(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t723: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t722 = v_inputs.clone(); f_distance_from_modulus(py::getitem(&t722, &Py::str("m"))?, py::getitem(&t722, &Py::str("M"))?, py::method(&t722, "get", &[Py::str("A"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t723 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t724 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t736 = (t724).items()?;
        if t736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t736[0].clone();
        v_v = t736[1].clone();
        if ({ let t725 = v_k.clone(); Py::Bool({ let t726 = v_outputs.clone(); if !py::contains(&t726, &t725)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_749(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t751: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t750 = v_inputs.clone(); f_distance_from_flux(py::getitem(&t750, &Py::str("L"))?, py::getitem(&t750, &Py::str("F"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t751 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t752 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t764 = (t752).items()?;
        if t764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t764[0].clone();
        v_v = t764[1].clone();
        if ({ let t753 = v_k.clone(); Py::Bool({ let t754 = v_outputs.clone(); if !py::contains(&t754, &t753)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_777(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t779: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t778 = v_inputs.clone(); f_distance_conversion(py::getitem(&t778, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t779 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t780 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t792 = (t780).items()?;
        if t792.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t792[0].clone();
        v_v = t792[1].clone();
        if ({ let t781 = v_k.clone(); Py::Bool({ let t782 = v_outputs.clone(); if !py::contains(&t782, &t781)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_805(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t806 = v_inputs.clone(); f_effective_temperature(py::getitem(&t806, &Py::str("L"))?, py::getitem(&t806, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t807 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t808 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t820 = (t808).items()?;
        if t820.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t820[0].clone();
        v_v = t820[1].clone();
        if ({ let t809 = v_k.clone(); Py::Bool({ let t810 = v_outputs.clone(); if !py::contains(&t810, &t809)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_835(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t837: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t836 = v_inputs.clone(); f_einstein_radius(py::getitem(&t836, &Py::str("M"))?, py::getitem(&t836, &Py::str("D_l"))?, py::getitem(&t836, &Py::str("D_s"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t837 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t838 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t850 = (t838).items()?;
        if t850.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t850[0].clone();
        v_v = t850[1].clone();
        if ({ let t839 = v_k.clone(); Py::Bool({ let t840 = v_outputs.clone(); if !py::contains(&t840, &t839)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_865(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t867: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t866 = v_inputs.clone(); f_escape_dv(py::getitem(&t866, &Py::str("M"))?, py::getitem(&t866, &Py::str("r"))?, py::method(&t866, "get", &[Py::str("v_inf"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t867 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t868 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t880 = (t868).items()?;
        if t880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t880[0].clone();
        v_v = t880[1].clone();
        if ({ let t869 = v_k.clone(); Py::Bool({ let t870 = v_outputs.clone(); if !py::contains(&t870, &t869)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_87(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t89: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t88 = v_inputs.clone(); f_airmass(py::getitem(&t88, &Py::str("zen"))?, py::method(&t88, "get", &[Py::str("k"), Py::None])?, py::method(&t88, "get", &[Py::str("m"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t89 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t90 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t102 = (t90).items()?;
        if t102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t102[0].clone();
        v_v = t102[1].clone();
        if ({ let t91 = v_k.clone(); Py::Bool({ let t92 = v_outputs.clone(); if !py::contains(&t92, &t91)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_891(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t893: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t892 = v_inputs.clone(); f_flux_ratio(py::getitem(&t892, &Py::str("m1"))?, py::getitem(&t892, &Py::str("m2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t893 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t894 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t906 = (t894).items()?;
        if t906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t906[0].clone();
        v_v = t906[1].clone();
        if ({ let t895 = v_k.clone(); Py::Bool({ let t896 = v_outputs.clone(); if !py::contains(&t896, &t895)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_919(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t921: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t920 = v_inputs.clone(); f_free_fall_time(py::getitem(&t920, &Py::str("rho"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t921 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t922 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t934 = (t922).items()?;
        if t934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t934[0].clone();
        v_v = t934[1].clone();
        if ({ let t923 = v_k.clone(); Py::Bool({ let t924 = v_outputs.clone(); if !py::contains(&t924, &t923)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_949(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t951: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t950 = v_inputs.clone(); f_gyrofrequency(py::getitem(&t950, &Py::str("B"))?, py::method(&t950, "get", &[Py::str("particle"), Py::str("electron")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t951 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t952 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t964 = (t952).items()?;
        if t964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t964[0].clone();
        v_v = t964[1].clone();
        if ({ let t953 = v_k.clone(); Py::Bool({ let t954 = v_outputs.clone(); if !py::contains(&t954, &t953)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_981(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t983: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t982 = v_inputs.clone(); f_gyroradius(py::getitem(&t982, &Py::str("B"))?, py::getitem(&t982, &Py::str("v_perp"))?, py::method(&t982, "get", &[Py::str("particle"), Py::str("electron")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t983 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t984 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t996 = (t984).items()?;
        if t996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t996[0].clone();
        v_v = t996[1].clone();
        if ({ let t985 = v_k.clone(); Py::Bool({ let t986 = v_outputs.clone(); if !py::contains(&t986, &t985)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
