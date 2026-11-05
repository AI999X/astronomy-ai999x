//! Astronomy's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model computes:
//! bars over named stars, planets and plasmas, orbits drawn from the elements the model returns, sky charts,
//! Hertzsprung-Russell diagrams, a Hubble diagram, spectra and scales. Every number drawn is the model's answer or a
//! further solve of it (docs/FIGURE_PARITY.md); a figure that cannot be drawn for the inputs is left out.

use crate::figs_common as c;
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::f64::consts::PI;

const AU: f64 = 1.495978707e11;
const PC: f64 = 3.085677581491367e16;
const LSUN: f64 = 3.828e26;
const RSUN: f64 = 6.957e8;
const DEG: f64 = PI / 180.0;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "combined_magnitude" => vec![c::family_std(s, "curve-2", "m1", "m", "m2", 0.1, 10.0, true)],
        "saha_hydrogen_ionization" => vec![c::family_std(s, "curve-2", "T", "x", "n", 0.5, 1.5, false)],
        "alfven_speed" => vec![plasma_bars(s, "v_A", "Alfvén speeds in six plasmas", "Alfvén speed (m/s)", false)],
        "debye_length" => vec![plasma_bars(s, "lambda_D", "Debye lengths in six plasmas", "Debye length (m)", false)],
        "debye_number" => vec![plasma_bars(s, "N_D", "Particles in a Debye sphere in six plasmas", "particles per Debye sphere", false)],
        "gyrofrequency" => vec![plasma_bars(s, "f_c", "Electron and proton gyrofrequencies in six plasmas", "gyrofrequency (Hz)", true)],
        "gyroradius" => vec![plasma_bars(s, "r_L", "Gyroradii in six plasmas", "gyroradius (m)", false)],
        "inertial_length" => vec![plasma_bars(s, "d", "Electron and ion inertial lengths in six plasmas", "inertial length (m)", true)],
        "lower_hybrid_frequency" => vec![plasma_bars(s, "f_lh", "Lower hybrid frequencies in six plasmas", "frequency (Hz)", false)],
        "magnetic_pressure" => vec![plasma_bars(s, "p_B", "Magnetic pressures in six plasmas", "magnetic pressure (Pa)", false)],
        "plasma_beta" => vec![plasma_bars(s, "beta", "Plasma beta in six plasmas", "plasma beta", false)],
        "plasma_frequency" => vec![plasma_bars(s, "f_p", "Electron and ion plasma frequencies in six plasmas", "plasma frequency (Hz)", true)],
        "upper_hybrid_frequency" => vec![plasma_bars(s, "f_uh", "Upper hybrid frequencies in six plasmas", "frequency (Hz)", false)],
        "angular_size" => vec![named_bars(s, "theta_arcsec", "Angular sizes across the sky", "angular diameter (″)", &[
            ("Sun", vec![("D", 1.3927e9), ("d", AU)]), ("Moon", vec![("D", 3.4748e6), ("d", 3.844e8)]),
            ("Andromeda", vec![("D", 46.56e3 * PC), ("d", 765e3 * PC)]), ("Venus", vec![("D", 1.2104e7), ("d", 0.277 * AU)]),
            ("Jupiter", vec![("D", 1.3982e8), ("d", 4.2 * AU)]), ("Betelg.", vec![("D", 2.0 * 764.0 * RSUN), ("d", 168.0 * PC)])],
            "Angular diameters from size and distance (stated: the Sun 1.3927e9 m at 1 au, the Moon 3.4748e6 m at 3.844e8 m, \
             the Andromeda galaxy 46.56 kpc at 765 kpc, Venus 1.2104e7 m at 0.277 au, Jupiter 1.3982e8 m at 4.2 au, \
             Betelgeuse 764 solar radii in radius at 168 pc), in arcseconds.", "″")],
        "atmospheric_scale_height" => vec![named_bars(s, "H_km", "Scale heights of planetary atmospheres", "scale height (km)", &[
            ("Venus", vec![("T", 737.0), ("mu", 0.04345), ("g", 8.87)]), ("Earth", vec![("T", 288.0), ("mu", 0.02897), ("g", 9.81)]),
            ("Mars", vec![("T", 210.0), ("mu", 0.04334), ("g", 3.71)]), ("Jupiter", vec![("T", 165.0), ("mu", 0.00222), ("g", 24.79)]),
            ("Titan", vec![("T", 94.0), ("mu", 0.028), ("g", 1.352)])],
            "Atmospheric scale heights, the height over which the pressure falls by a factor e, from temperature, mean \
             molar mass and surface gravity (stated: Venus 737 K, 43.45 g/mol, 8.87 m/s²; Earth 288 K, 28.97 g/mol, 9.81 \
             m/s²; Mars 210 K, 43.34 g/mol, 3.71 m/s²; Jupiter 165 K, 2.22 g/mol, 24.79 m/s²; Titan 94 K, 28 g/mol, 1.352 \
             m/s²).", " km")],
        "distance_unit_conversion" => vec![named_bars(s, "d_ly", "Distances in light-years", "distance (light-years)", &[
            ("Moon", vec![("d", 3.844e8)]), ("Sun", vec![("d", AU)]), ("Neptune", vec![("d", 30.07 * AU)]),
            ("Proxima", vec![("d", 1.302 * PC)]), ("Gal. centre", vec![("d", 8.18e3 * PC)]), ("Andromeda", vec![("d", 765e3 * PC)])],
            "Distances of six objects (stated in metres: the Moon 3.844e8, the Sun 1 au, Neptune 30.07 au, Proxima Centauri \
             1.302 pc, the galactic centre 8.18 kpc, Andromeda 765 kpc), each converted by the model; bars in light-years.", " ly")],
        "light_gathering_power" => vec![named_bars(s, "area_ratio", "Light gathered compared with the eye", "light-gathering ratio", &[
            ("binoculars", vec![("D1", 0.05), ("D2", 0.007)]), ("200 mm", vec![("D1", 0.2), ("D2", 0.007)]),
            ("Hubble", vec![("D1", 2.4), ("D2", 0.007)]), ("VLT", vec![("D1", 8.2), ("D2", 0.007)]), ("ELT", vec![("D1", 39.0), ("D2", 0.007)])],
            "How much more light five apertures collect than a 7 mm dark-adapted eye (stated: binoculars 50 mm, a 200 mm \
             telescope, Hubble 2.4 m, the VLT 8.2 m, the ELT 39 m), and the given aperture against its own reference.", "×")],
        "linear_size_from_angle" => vec![named_bars(s, "D_km", "What one angle spans at different distances", "diameter (km)", &[
            ("Moon", vec![("d", 3.844e8)]), ("Sun", vec![("d", AU)]), ("Proxima", vec![("d", 1.302 * PC)]),
            ("Gal. centre", vec![("d", 8.18e3 * PC)]), ("Andromeda", vec![("d", 765e3 * PC)])],
            "What the given angle spans at five distances (stated: the Moon 3.844e8 m, the Sun 1 au, Proxima Centauri 1.302 \
             pc, the galactic centre 8.18 kpc, Andromeda 765 kpc; Euclidean) and at the given distance: the linear size the \
             model returns.", " km")],
        "planet_equilibrium_temperature" => vec![named_bars(s, "T_eq", "Equilibrium temperatures of the planets", "equilibrium temperature (K)", &[
            ("Merc.", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", 0.387 * AU), ("A", 0.088)]),
            ("Venus", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", 0.723 * AU), ("A", 0.76)]),
            ("Earth", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", AU), ("A", 0.306)]),
            ("Mars", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", 1.524 * AU), ("A", 0.25)]),
            ("Jup.", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", 5.203 * AU), ("A", 0.503)]),
            ("Sat.", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", 9.537 * AU), ("A", 0.342)]),
            ("Uran.", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", 19.19 * AU), ("A", 0.3)]),
            ("Nep.", vec![("T_star", 5772.0), ("R_star", RSUN), ("a", 30.07 * AU), ("A", 0.29)])],
            "Equilibrium temperatures of the eight planets for the Sun (5772 K, radius 6.957e8 m) from their mean distances \
             and Bond albedos (stated), each computed by the model (no-atmosphere values, without any greenhouse warming).", " K")],
        "stellar_radius" => vec![named_bars(s, "R_Rsun", "Radii of stars from luminosity and temperature", "radius (solar radii)", &[
            ("Sun", vec![("L", LSUN), ("T", 5772.0)]), ("Sirius B", vec![("L", 0.056 * LSUN), ("T", 25200.0)]),
            ("Proxima", vec![("L", 0.00155 * LSUN), ("T", 3042.0)]), ("Vega", vec![("L", 40.1 * LSUN), ("T", 9602.0)]),
            ("Arcturus", vec![("L", 170.0 * LSUN), ("T", 4286.0)]), ("Rigel", vec![("L", 120000.0 * LSUN), ("T", 12100.0)]),
            ("Betelg.", vec![("L", 126000.0 * LSUN), ("T", 3600.0)])],
            "Radii returned from luminosity and effective temperature (stated: the Sun 1 solar luminosity, 5772 K; Sirius B \
             0.056, 25200 K; Proxima 0.00155, 3042 K; Vega 40.1, 9602 K; Arcturus 170, 4286 K; Rigel 120000, 12100 K; \
             Betelgeuse 126000, 3600 K; one solar luminosity = 3.828e26 W), in solar radii.", " R☉")],
        "absolute_magnitude" => vec![magnitude_ladder(s)],
        "altaz_from_hadec" => vec![sky_paths(s)],
        "apsis_speeds" => vec![apsis_orbit(s)],
        "binary_star_masses" => vec![binary_orbit(s)],
        "camera_pixel_scale" => vec![fields_of_view(s)],
        "cosmological_distances" => vec![hubble(s)],
        "culmination_altitude" => vec![meridian(s)],
        "effective_temperature" => vec![hr_stars(s)],
        "stellar_luminosity" => vec![hr_radii(s)],
        "hohmann_transfer" => vec![hohmann(s)],
        "julian_date" => vec![timeline(s)],
        "kepler_position_at_time" => vec![kepler_orbit(s)],
        "luminosity_from_bolometric_magnitude" => vec![luminosity_stems(s)],
        "magnitude_difference_from_flux_ratio" => vec![ratio_scale(s)],
        "orbit_from_state" => vec![launch_orbits(s)],
        "position_angle" => vec![companions(s)],
        "redshift_from_wavelengths" => vec![shifted_lines(s)],
        "time_of_flight" => vec![equal_sweeps(s)],
        "transit_depth" => vec![transits(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
fn f(x: f64) -> Input {
    Input::Float(x)
}

fn int(n: i64) -> Input {
    Input::Int(n)
}

fn txt(t: &str) -> Input {
    Input::Text(t.to_string())
}

fn wi(s: &Solved, set: &[(&str, Input)], y: &str) -> Option<f64> {
    let a = s.with_input(set)?;
    k::num_of(&a, y)
}

fn oi(s: &Solved, key: &str, set: &[(&str, Input)], y: &str) -> Option<f64> {
    let a = s.solve_other(key, set)?;
    k::num_of(&a, y)
}

fn fig(s: &Solved, kind: &str, title: &str, cap: &str, cv: &Canvas) -> Option<Figure> {
    Some(s.figure(kind, title, cap, cv.body(), cv.h))
}

fn takes(s: &Solved, sym: &str) -> bool {
    s.p.procedure(s.key).map(|p| p.inputs.iter().any(|i| i.symbol == sym)).unwrap_or(false)
}

#[allow(clippy::too_many_arguments)]
fn chart(s: &Solved, kind: &str, title: &str, cap: &str, series: Vec<(String, Vec<(f64, f64)>)>, xlab: &str,
         ylab: &str, header: &str, marker: Option<(f64, f64)>, mlabel: &str, log_x: bool) -> Option<Figure> {
    let series: Vec<(String, Vec<(f64, f64)>)> = series.into_iter().filter(|x| x.1.len() >= 2).collect();
    if series.is_empty() {
        return None;
    }
    let body = c::fit_right(&k::lines(&series, xlab, ylab, header, marker, mlabel, log_x));
    Some(s.chart(kind, title, cap, &body))
}

/// Bars over named cases (each the given inputs with some replaced, SI), the given inputs as the last bar.
fn named_bars(s: &Solved, y: &str, title: &str, ylab: &str, cases: &[(&str, Vec<(&str, f64)>)], intro: &str, u: &str)
              -> Option<Figure> {
    let mut labels = vec![];
    let mut vals = vec![];
    for (name, set) in cases {
        if let Some(v) = c::at(s, set, y) {
            labels.push(name.to_string());
            vals.push(v);
        }
    }
    let mine = s.out(y)?;
    labels.push("given".into());
    vals.push(mine);
    if vals.len() < 3 || vals.iter().any(|v| !(v.is_finite())) {
        return None;
    }
    let marks: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}{u}", k::num(*v))).collect();
    let body = c::fit_right(&k::bars(&labels, &vals, &[vals.len() - 1], "", ylab, &format!("given: {}{u}", k::num(mine))));
    Some(s.chart("bars", title, &format!("{intro} Each bar is a solve of the model; the last is the given inputs. \
                                          Marked: {}.", marks.join(", ")), &body))
}

/// Six reference plasmas: density (1/m³), temperature (K), field (T).
const PLASMAS: [(&str, f64, f64, f64); 6] = [("wind", 5e6, 1e5, 5e-9), ("corona", 1e15, 1e6, 1e-3),
    ("ionos.", 1e12, 1000.0, 5e-5), ("tail", 3e5, 5e7, 1e-8), ("ISM", 1e5, 8000.0, 5e-10),
    ("tokamak", 1e20, 1e8, 5.0)];

fn plasma_bars(s: &Solved, y: &str, title: &str, ylab: &str, species: bool) -> Option<Figure> {
    let mine = s.out(y)?;
    let uses: Vec<&str> = ["n", "T", "B"].into_iter().filter(|x| takes(s, x)).collect();
    if uses.is_empty() {
        return None;
    }
    let mut labels: Vec<String> = PLASMAS.iter().map(|p| p.0.to_string()).collect();
    labels.push("given".into());
    let solve_case = |pl: &(&str, f64, f64, f64), part: Option<&str>| -> Option<f64> {
        let mut set: Vec<(&str, Input)> = vec![];
        for u in &uses {
            set.push((u, f(match *u { "n" => pl.1, "T" => pl.2, _ => pl.3 })));
        }
        if let Some(pt) = part {
            set.push(("particle", txt(pt)));
        }
        wi(s, &set, y)
    };
    let body;
    let mut marks = vec![];
    if species {
        let mut groups = vec![];
        for pt in ["electron", "proton"] {
            let mut v: Vec<Option<f64>> = PLASMAS.iter().map(|pl| solve_case(pl, Some(pt))).collect();
            let given = if s.txt("particle").unwrap_or_else(|| "electron".into()) == pt { Some(mine) } else { None };
            v.push(given);
            for (l, x) in labels.iter().zip(&v) {
                if let Some(x) = x {
                    marks.push(format!("{l} ({pt}) {}", k::num(*x)));
                }
            }
            groups.push((pt.to_string(), v));
        }
        body = c::fit_right(&k::bar_groups(&labels, &groups, "", ylab, &format!("given: {}", k::num(mine))));
    } else {
        let mut vals = vec![];
        for pl in &PLASMAS {
            vals.push(solve_case(pl, None)?);
        }
        vals.push(mine);
        for (l, x) in labels.iter().zip(&vals) {
            marks.push(format!("{l} {}", k::num(*x)));
        }
        body = c::fit_right(&k::bars(&labels, &vals, &[6], "", ylab, &format!("given: {}", k::num(mine))));
    }
    Some(s.chart("bars", title, &format!(
        "{} for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona \
         1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, \
         5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the \
         quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. \
         Marked: {}.", ylab.split(" (").next().unwrap_or(ylab), marks.join(", ")), &body))
}

/// A log-log scatter canvas (the Hertzsprung-Russell plane: temperature decreasing to the right when `rev_x`).
struct LogFrame {
    x: (f64, f64),
    y: (f64, f64),
    rev_x: bool,
}

impl LogFrame {
    fn px(&self, v: f64) -> f64 {
        let t = (v.log10() - self.x.0.log10()) / (self.x.1.log10() - self.x.0.log10());
        if self.rev_x { 470.0 - t * 380.0 } else { 90.0 + t * 380.0 }
    }
    fn py(&self, v: f64) -> f64 {
        let t = (v.log10() - self.y.0.log10()) / (self.y.1.log10() - self.y.0.log10());
        350.0 - t * 300.0
    }
    fn draw(&self, cv: &mut Canvas, xlab: &str, ylab: &str) {
        cv.raw("<rect class=\"axp-f\" x=\"90\" y=\"50\" width=\"380\" height=\"300\"/>");
        for (lo, hi, is_x) in [(self.x.0, self.x.1, true), (self.y.0, self.y.1, false)] {
            let (e0, e1) = (lo.log10().ceil() as i32, hi.log10().floor() as i32);
            let step = (((e1 - e0) as f64) / 6.0).ceil().max(1.0) as usize;
            for e in (e0..=e1).step_by(step) {
                let v = 10f64.powi(e);
                if is_x {
                    let x = self.px(v);
                    cv.raw(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"50\" x2=\"{x:.1}\" y2=\"350\"/>"));
                    cv.name(x, 368.0, "middle", &k::num(v));
                } else {
                    let y = self.py(v);
                    cv.raw(&format!("<line class=\"axp-g\" x1=\"90\" y1=\"{y:.1}\" x2=\"470\" y2=\"{y:.1}\"/>"));
                    cv.name(84.0, y + 4.0, "end", &k::num(v));
                }
            }
        }
        cv.name(280.0, 392.0, "middle", xlab);
        cv.name(16.0, 40.0, "start", ylab);
    }
}

// ------------------------------------------------------------------------------------------------ stars and magnitudes
fn magnitude_ladder(s: &Solved) -> Option<Figure> {
    let (m0, mm) = (c::inp(s, "m")?, s.out("M")?);
    let stars = [("Sirius", -1.46, 2.64), ("Vega", 0.03, 7.68), ("Arcturus", -0.05, 11.26), ("Polaris", 1.98, 133.0),
                 ("Rigel", 0.13, 264.0), ("Deneb", 1.25, 802.0)];
    let mut labels = vec![];
    let mut app = vec![];
    let mut abs = vec![];
    let mut marks = vec![];
    for (n, m, d) in stars {
        let mut set = vec![("m", f(m)), ("d", f(d * PC))];
        if takes(s, "A") {
            set.push(("A", f(0.0)));
        }
        let a = wi(s, &set, "M")?;
        labels.push(n.to_string());
        app.push(Some(m));
        abs.push(Some(a));
        marks.push(format!("{n} m = {}, M = {}", k::num(m), k::num(a)));
    }
    labels.push("given".into());
    app.push(Some(m0));
    abs.push(Some(mm));
    let body = c::fit_right(&k::bar_groups(&labels, &[("apparent m".into(), app), ("absolute M".into(), abs)], "",
                                           "magnitude (smaller is brighter)", &format!("given: M = {}", k::num(mm))));
    Some(s.chart("ladder", "Apparent and absolute magnitudes of bright stars", &format!(
        "Apparent magnitudes and distances of six bright stars and of the given star (stated: Sirius m = -1.46 at 2.64 pc, \
         Vega 0.03 at 7.68 pc, Arcturus -0.05 at 11.26 pc, Polaris 1.98 at 133 pc, Rigel 0.13 at 264 pc, Deneb 1.25 at 802 \
         pc; no extinction) and the absolute magnitude the model returns for each, the magnitude the star would show from \
         10 pc. Marked: {}; given M = {}.", marks.join("; "), k::num(mm)), &body))
}

fn luminosity_stems(s: &Solved) -> Option<Figure> {
    let (m0, l0) = (c::inp(s, "M_bol")?, s.out("L_Lsun")?);
    let mut labels = vec![];
    let mut vals = vec![];
    let mut hl = vec![];
    for m in -10..=15 {
        vals.push(wi(s, &[("M_bol", f(m as f64))], "L_Lsun")?);
        labels.push(m.to_string());
        if (m as f64 - m0).abs() < 0.5 {
            hl.push(labels.len() - 1);
        }
    }
    let body = c::fit_right(&k::bars(&labels, &vals, &hl, "absolute bolometric magnitude", "luminosity (solar units)",
                                     &format!("given: {} solar luminosities", k::num(l0))));
    Some(s.chart("stems", "Luminosity against absolute bolometric magnitude", &format!(
        "Luminosity in solar units for every whole absolute bolometric magnitude from -10 to 15 (26 solves of the model): \
         each step of 5 magnitudes is a factor of exactly 100. Marked: -10 → {}, 0 → {}, 15 → {}; the given M = {} gives \
         {} solar luminosities.", k::num(vals[0]), k::num(vals[10]), k::num(vals[25]), k::num(m0), k::num(l0)), &body))
}

fn ratio_scale(s: &Solved) -> Option<Figure> {
    let (r0, dm0) = (c::inp(s, "ratio")?, s.out("dm")?);
    if !(r0 > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(200.0);
    let (lo, hi) = (r0.log10().min(0.0).floor(), r0.log10().max(10.0).ceil());
    let px = c::lin(lo, hi, 40.0, 480.0);
    cv.line((40.0, 100.0), (480.0, 100.0));
    let mut marks = vec![];
    let step = ((hi - lo) / 10.0).ceil().max(1.0) as usize;
    for e in (lo as i32..=hi as i32).step_by(step) {
        let r = 10f64.powi(e);
        let dm = wi(s, &[("ratio", f(r))], "dm")?;
        let x = px(e as f64);
        cv.line((x, 92.0), (x, 108.0));
        cv.name(x, 84.0, "middle", &k::num(r));
        cv.value(x, 128.0, "middle", &k::num(k::snap(dm, 1.0)));
        marks.push(format!("{} → {}", k::num(r), k::num(k::snap(dm, 1.0))));
    }
    let x0 = px(r0.log10());
    cv.raw(&format!("<circle class=\"axp-m\" cx=\"{x0:.1}\" cy=\"100\" r=\"6\"/>"));
    cv.bold(x0, 160.0, "middle", &format!("given ratio {}: Δm = {}", k::num(r0), k::num(dm0)));
    cv.name(16.0, 24.0, "start", "flux ratio F1/F2 (above), magnitude difference m1 - m2 (below)");
    fig(s, "scale", "A ruler of flux ratios and magnitude differences", &format!(
        "A ruler of flux ratios, one per tick, and the magnitude difference the model returns under each: a factor 10 in \
         flux is 2.5 magnitudes. {}. Marked: the given ratio {}, Δm = {}.", marks.join(", "), k::num(r0), k::num(dm0)), &cv)
}

fn hr_stars(s: &Solved) -> Option<Figure> {
    let (l0, r0, t0) = (c::inp(s, "L")?, c::inp(s, "R")?, s.out("T")?);
    let stars = [("Sun", 1.0, 1.0), ("Sirius A", 25.4, 1.711), ("Sirius B", 0.056, 0.0084), ("Vega", 40.1, 2.36),
                 ("Proxima", 0.00155, 0.154), ("Betelgeuse", 126000.0, 764.0), ("Rigel", 120000.0, 78.9),
                 ("Aldebaran", 439.0, 45.1), ("Arcturus", 170.0, 25.4)];
    let mut pts = vec![];
    for (n, l, r) in stars {
        pts.push((n.to_string(), c::at(s, &[("L", l * LSUN), ("R", r * RSUN)], "T")?, l));
    }
    pts.push(("given".into(), t0, l0 / LSUN));
    let (tlo, thi) = pts.iter().fold((f64::INFINITY, 0.0f64), |a, p| (a.0.min(p.1), a.1.max(p.1)));
    let (llo, lhi) = pts.iter().fold((f64::INFINITY, 0.0f64), |a, p| (a.0.min(p.2), a.1.max(p.2)));
    if !(tlo > 0.0 && llo > 0.0) {
        return None;
    }
    let fr = LogFrame { x: (tlo / 1.5, thi * 1.5), y: (llo / 5.0, lhi * 5.0), rev_x: true };
    let mut cv = Canvas::new(400.0);
    fr.draw(&mut cv, "effective temperature (K), hotter to the left", "luminosity (solar units)");
    for (i, (n, t, l)) in pts.iter().enumerate() {
        let (x, y) = (fr.px(*t), fr.py(*l));
        let last = i + 1 == pts.len();
        cv.raw(&format!("<circle class=\"{}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{}\"/>", if last { "axp-m" } else { "axp-d" },
                        if last { 6 } else { 4 }));
        cv.name(x + 7.0, y - 5.0, "start", n);
    }
    cv.bold(16.0, 24.0, "start", &format!("given: T = {} K", k::num(t0)));
    fig(s, "hr", "Stars on the Hertzsprung-Russell diagram", &format!(
        "A Hertzsprung-Russell diagram (temperature decreasing to the right): nine stars placed at the effective \
         temperature the model returns from their luminosity and radius (stated, in solar units of 3.828e26 W and \
         6.957e8 m), and the given star ({} W, {}): {}.", k::num(l0), k::q(r0, "m"),
        pts.iter().map(|(n, t, _)| format!("{n} {} K", k::num(*t))).collect::<Vec<_>>().join(", ")), &cv)
}

fn hr_radii(s: &Solved) -> Option<Figure> {
    let (r0, t0, l0) = (c::inp(s, "R")?, c::inp(s, "T")?, s.out("L_Lsun")?);
    let ts = k::span(2300.0, 50000.0, 25, true);
    let fr = LogFrame { x: (2000.0, 60000.0), y: (1e-7, 1e9), rev_x: true };
    let mut cv = Canvas::new(400.0);
    fr.draw(&mut cv, "effective temperature (K), hotter to the left", "luminosity (solar units)");
    for r in [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 1000.0] {
        let mut p = vec![];
        for t in &ts {
            if let Some(l) = c::at(s, &[("R", r * RSUN), ("T", *t)], "L_Lsun") {
                if l > fr.y.0 && l < fr.y.1 {
                    p.push((fr.px(*t), fr.py(l)));
                }
            }
        }
        if p.len() >= 2 {
            c::poly(&mut cv, &p, "axp-c4", false);
            let e = p[p.len() - 1];
            cv.name(e.0 + 4.0, e.1 + 4.0, "start", &format!("{} R☉", k::num(r)));
        }
    }
    if l0 > fr.y.0 && l0 < fr.y.1 && t0 > fr.x.0 && t0 < fr.x.1 {
        cv.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", fr.px(t0), fr.py(l0)));
    }
    let sun = c::at(s, &[("R", RSUN), ("T", 5772.0)], "L_Lsun")?;
    cv.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\"/>", fr.px(5772.0), fr.py(sun)));
    cv.name(fr.px(5772.0) + 6.0, fr.py(sun) + 14.0, "start", "Sun");
    cv.bold(16.0, 24.0, "start", &format!("given: L = {} solar luminosities", k::num(l0)));
    fig(s, "hr", "Lines of equal radius on the Hertzsprung-Russell plane", &format!(
        "Lines of equal radius on the Hertzsprung-Russell plane: the luminosity the model returns for radii 0.001 to 1000 \
         solar radii at 25 temperatures from 2300 to 50000 K each (temperature decreasing to the right). Marked: one \
         solar radius at 5772 K gives {} solar luminosities; the given R = {}, T = {} K gives {} (marked when it falls in \
         the drawn range).", k::num(sun), k::q(r0, "m"), k::num(t0), k::num(l0)), &cv)
}

fn hubble(s: &Solved) -> Option<Figure> {
    let (z0, mu0) = (c::inp(s, "z")?, s.out("mu")?);
    let om0 = c::inp(s, "Om")?;
    let zs = k::span(0.01, z0.max(1.0) * 1.2, 41, true);
    let mut series = vec![];
    for om in [0.3, om0] {
        let p: Vec<(f64, f64)> = zs.iter().filter_map(|z| wi(s, &[("z", f(*z)), ("Om", f(om))], "mu").map(|y| (*z, y))).collect();
        series.push((format!("Ωm = {}", k::num(om)), p));
        if (om0 - 0.3).abs() < 1e-9 {
            break;
        }
    }
    chart(s, "hubble", "A Hubble diagram: distance modulus against redshift", &format!(
        "A Hubble diagram: distance modulus against redshift (41 redshifts from 0.01 to {}, H0 = {} km/s/Mpc), each a \
         solve of the model for a flat universe with Ωm = 0.3 and with the given Ωm = {}. Marked: the given z = {}, μ = {}.",
        k::num(zs[40]), k::num(c::inp(s, "H0")?), k::num(om0), k::num(z0), k::num(mu0)),
          series, "z, redshift", "μ, distance modulus", &format!("μ = {}", k::num(mu0)), Some((z0, mu0)),
          &k::num(mu0), true)
}

fn shifted_lines(s: &Solved) -> Option<Figure> {
    let (lo, lr, z) = (c::inp(s, "lam_obs")?, c::inp(s, "lam_rest")?, s.out("z")?);
    if !(lr > 0.0 && lo > 0.0) {
        return None;
    }
    let ratio = lo / lr;
    let lines = [("Hα", 656.28e-9), ("Hβ", 486.13e-9), ("Hγ", 434.05e-9), ("Hδ", 410.17e-9)];
    let mut zs = vec![];
    for (_, l) in lines {
        zs.push(wi(s, &[("lam_obs", f(l * ratio)), ("lam_rest", f(l))], "z")?);
    }
    let all: Vec<f64> = lines.iter().map(|l| l.1).chain(lines.iter().map(|l| l.1 * ratio)).collect();
    let (a, b) = all.iter().fold((f64::INFINITY, 0.0f64), |m, x| (m.0.min(*x), m.1.max(*x)));
    let px = c::lin(a * 0.95, b * 1.05, 40.0, 480.0);
    let mut cv = Canvas::new(230.0);
    for (row, (y, name)) in [(70.0, "at rest"), (150.0, "observed")].iter().enumerate() {
        cv.raw(&format!("<rect class=\"axp-ka\" x=\"40\" y=\"{}\" width=\"440\" height=\"40\"/>", y - 20.0));
        cv.name(40.0, y - 26.0, "start", name);
        for (i, (n, l)) in lines.iter().enumerate() {
            let x = px(if row == 0 { *l } else { l * ratio });
            cv.raw(&format!("<path class=\"axp-c\" d=\"M{x:.1},{:.1} L{x:.1},{:.1}\"/>", y - 20.0, y + 20.0));
            if row == 1 || i == 0 {
                cv.name(x, y + 34.0, "middle", n);
            }
        }
    }
    cv.bold(16.0, 24.0, "start", &format!("z = {}, v = {} km/s", k::num(z), k::num(s.out("v")? / 1000.0)));
    fig(s, "spectrum", "Spectral lines shifted by one redshift", &format!(
        "Four Balmer lines of hydrogen at their rest wavelengths (656.28, 486.13, 434.05, 410.17 nm) and as observed, each \
         observed wavelength {} times the rest one (the given ratio); the model returns the same redshift for every pair, \
         z = {}, and a recession speed of {} km/s. Every line shifts by the same factor, so redder lines move farther.",
        k::num(ratio), zs.iter().map(|x| k::num(*x)).collect::<Vec<_>>().join(", "), k::num(s.out("v")? / 1000.0)), &cv)
}

fn timeline(s: &Solved) -> Option<Figure> {
    let jd0 = s.out("JD")?;
    let dates = [("Gregorian calendar", 1583, 1, 1, 0), ("MJD day 0", 1858, 11, 17, 0), ("1900 January 0.5", 1900, 1, 1, 12),
                 ("Unix epoch", 1970, 1, 1, 0), ("J2000.0", 2000, 1, 1, 12)];
    let mut pts = vec![];
    for (n, y, m, d, h) in dates {
        let mut set = vec![("year", int(y)), ("month", int(m)), ("day", int(d)), ("hour", int(h))];
        if takes(s, "minute") {
            set.push(("minute", int(0)));
        }
        if takes(s, "second") {
            set.push(("second", f(0.0)));
        }
        let a = s.with_input(&set)?;
        pts.push((n.to_string(), format!("{y}-{m:02}-{d:02} {h}h"), k::num_of(&a, "JD")?, k::num_of(&a, "MJD")?));
    }
    pts.push(("given".into(), format!("{}-{:02}-{:02}", c::inp(s, "year")? as i64, c::inp(s, "month")? as i64,
                                      c::inp(s, "day")? as i64), jd0, s.out("MJD")?));
    let (lo, hi) = pts.iter().fold((f64::INFINITY, 0.0f64), |a, p| (a.0.min(p.2), a.1.max(p.2)));
    let px = c::lin(lo - (hi - lo) * 0.05, hi + (hi - lo) * 0.05, 40.0, 480.0);
    let mut cv = Canvas::new(300.0);
    cv.line((30.0, 160.0), (490.0, 160.0));
    for (i, (n, d, jd, _)) in pts.iter().enumerate() {
        let x = px(*jd);
        let last = i + 1 == pts.len();
        cv.raw(&format!("<circle class=\"{}\" cx=\"{x:.1}\" cy=\"160\" r=\"{}\"/>", if last { "axp-m" } else { "axp-d" },
                        if last { 6 } else { 4 }));
        let up = i % 2 == 0;
        let y = if up { 56.0 + 30.0 * (i / 2) as f64 } else { 200.0 + 30.0 * (i / 2) as f64 };
        cv.dashed((x, 160.0), (x, if up { y + 6.0 } else { y - 18.0 }));
        cv.name(x, y - 12.0, "middle", n);
        cv.value(x, y + 2.0, "middle", &format!("JD {}", k::num(*jd)));
        let _ = d;
    }
    cv.bold(16.0, 24.0, "start", &format!("given: JD {} (MJD {})", jd0, k::num(s.out("MJD")?)));
    fig(s, "timeline", "Dates on the Julian day count", &format!(
        "{} dates on the Julian day count, the given one among them (dates as inputs, UT), each a solve of the model: {}.",
        pts.len(), pts.iter().map(|(n, d, jd, mjd)| format!("{n} {d} → JD {} (MJD {})", jd, k::num(*mjd)))
            .collect::<Vec<_>>().join("; ")), &cv)
}

// ------------------------------------------------------------------------------------------------ orbits
/// An ellipse with one focus at `fo` (pixels), semi-major axis a and eccentricity e (both in pixels/unit), the
/// periapsis to the right.
fn focus_ellipse(cv: &mut Canvas, fo: (f64, f64), a: f64, e: f64, class: &str) {
    let b = a * (1.0 - e * e).max(0.0).sqrt();
    c::ellipse(cv, (fo.0 - a * e, fo.1), a, b, class);
}

fn apsis_orbit(s: &Solved) -> Option<Figure> {
    let (rp, ra, m) = (c::inp(s, "rp")?, c::inp(s, "ra")?, c::inp(s, "M")?);
    let (vp, va, a, e, p) = (s.out("vp")?, s.out("va")?, s.out("a")?, s.out("e")?, s.out("P")?);
    let sc = (400.0 / (2.0 * a)).min(150.0 / (a * (1.0 - e * e).max(1e-9).sqrt()));
    let mut cv = Canvas::new(360.0);
    let fo = (60.0 + ra * sc, 200.0);
    focus_ellipse(&mut cv, fo, a * sc, e, "axp-c");
    c::disc(&mut cv, fo, 8.0, true);
    let vs = 90.0 / vp.max(va);
    let peri = (fo.0 + rp * sc, fo.1);
    let apo = (fo.0 - ra * sc, fo.1);
    c::varrow(&mut cv, peri, (peri.0, peri.1 - vp * vs), "axp-c2");
    c::varrow(&mut cv, apo, (apo.0, apo.1 + va * vs), "axp-c2");
    cv.value(peri.0 - 6.0, peri.1 - vp * vs - 8.0, "end", &format!("vp = {}", c::qv(vp, "m/s")));
    cv.value(apo.0 + 6.0, apo.1 + va * vs + 16.0, "start", &format!("va = {}", c::qv(va, "m/s")));
    cv.dashed(apo, peri);
    cv.bold(16.0, 24.0, "start", &format!("a = {}, e = {}, P = {}", k::q(a, "m"), k::num(e), k::q(p, "s")));
    fig(s, "orbit", "An orbit between periapsis and apoapsis", &format!(
        "An orbit around a mass M = {} kg from periapsis {} to apoapsis {}, drawn to scale; velocity arrows to a common \
         scale. The model returns vp = {}, va = {} (ratio {}, the inverse ratio of the distances), a = {}, e = {}, P = {}.",
        k::num(m), k::q(rp, "m"), k::q(ra, "m"), c::qv(vp, "m/s"), c::qv(va, "m/s"), k::num(vp / va), k::q(a, "m"), k::num(e),
        k::q(p, "s")), &cv)
}

fn binary_orbit(s: &Solved) -> Option<Figure> {
    let (p, a1, a2) = (c::inp(s, "P")?, c::inp(s, "a1")?, c::inp(s, "a2")?);
    let (m1, m2, mt) = (s.out("M1_Msun")?, s.out("M2_Msun")?, s.out("M_total_Msun")?);
    if !(a1 > 0.0 && a2 > 0.0) {
        return None;
    }
    let sc = 170.0 / a1.max(a2);
    let mut cv = Canvas::new(380.0);
    let cm = (260.0, 200.0);
    c::ellipse(&mut cv, cm, a1 * sc, a1 * sc, "axp-c");
    c::ellipse(&mut cv, cm, a2 * sc, a2 * sc, "axp-c2");
    cv.raw(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", cm.0 - 6.0, cm.1, cm.0 + 6.0, cm.1));
    cv.raw(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", cm.0, cm.1 - 6.0, cm.0, cm.1 + 6.0));
    let r1 = 5.0 + 9.0 * m1 / mt;
    let r2 = 5.0 + 9.0 * m2 / mt;
    let s1 = (cm.0 + a1 * sc, cm.1);
    let s2 = (cm.0 - a2 * sc, cm.1);
    c::disc(&mut cv, s1, r1, true);
    c::disc(&mut cv, s2, r2, true);
    cv.dashed(s2, s1);
    cv.value(s1.0 + r1 + 4.0, s1.1 - 8.0, "start", &format!("star 1: {} M☉", k::num(m1)));
    cv.value(s2.0 - r2 - 4.0, s2.1 - 8.0, "end", &format!("star 2: {} M☉", k::num(m2)));
    cv.bold(16.0, 24.0, "start", &format!("total {} M☉, period {}", k::num(mt), k::q(p, "s")));
    fig(s, "orbit", "A binary star about its centre of mass", &format!(
        "A binary star drawn to scale about its centre of mass (the cross), taking circular orbits: period {}, orbit \
         radii {} (star 1) and {} (star 2). The model returns a total mass of {} solar masses, split inversely to the \
         orbit radii: M1 = {}, M2 = {} solar masses; the heavier star moves on the smaller circle.", k::q(p, "s"),
        k::q(a1, "m"), k::q(a2, "m"), k::num(mt), k::num(m1), k::num(m2)), &cv)
}

fn hohmann(s: &Solved) -> Option<Figure> {
    let (r1, r2) = (c::inp(s, "r1")?, c::inp(s, "r2")?);
    let (dv1, dv2, dv, tt, at) = (s.out("dv1")?, s.out("dv2")?, s.out("dv")?, s.out("t_transfer")?, s.out("a_transfer")?);
    let e = ((r2 - r1) / (r2 + r1)).abs();
    let sc = 170.0 / r1.max(r2);
    let mut cv = Canvas::new(420.0);
    let ctr = (230.0, 220.0);
    c::disc(&mut cv, ctr, 8.0, true);
    c::ellipse(&mut cv, ctr, r1 * sc, r1 * sc, "axp-c4");
    c::ellipse(&mut cv, ctr, r2 * sc, r2 * sc, "axp-c4");
    // the transfer half ellipse: periapsis at r1 (right), apoapsis at r2 (left)
    let (rp, ra) = (r1.min(r2), r1.max(r2));
    let b = at * (1.0 - e * e).sqrt();
    let cx = ctr.0 - (at - rp) * sc;
    let half: Vec<(f64, f64)> = (0..=60).map(|i| {
        let t = PI * i as f64 / 60.0;
        (cx + at * sc * t.cos(), ctr.1 - b * sc * t.sin())
    }).collect();
    c::poly(&mut cv, &half, "axp-c", false);
    cv.dot((ctr.0 + rp * sc, ctr.1));
    cv.dot((ctr.0 - ra * sc, ctr.1));
    cv.value(ctr.0 + rp * sc + 6.0, ctr.1 + 16.0, "start", &format!("Δv1 = {}", c::qv(dv1, "m/s")));
    cv.value(ctr.0 - ra * sc + 6.0, ctr.1 + 16.0, "start", &format!("Δv2 = {}", c::qv(dv2, "m/s")));
    cv.bold(16.0, 24.0, "start", &format!("total Δv = {}, transfer time {}", c::qv(dv, "m/s"), k::q(tt, "s")));
    fig(s, "orbit", "A Hohmann transfer between circular orbits", &format!(
        "The Hohmann transfer from a circular orbit of r1 = {} to one of r2 = {}, computed by the model: transfer ellipse \
         a = {} (returned), e = {} (derived from the two radii), drawn as its half orbit; burns Δv1 = {} and Δv2 = {}, \
         total {}, transfer time {}.", k::q(r1, "m"), k::q(r2, "m"), k::q(at, "m"), k::num(e), c::qv(dv1, "m/s"),
        c::qv(dv2, "m/s"), c::qv(dv, "m/s"), k::q(tt, "s")), &cv)
}

fn kepler_orbit(s: &Solved) -> Option<Figure> {
    let (a, e, t0) = (c::inp(s, "a")?, c::inp(s, "e")?, c::inp(s, "t")?);
    let p = s.out("P")?;
    if !(p > 0.0 && a > 0.0) {
        return None;
    }
    let mut path = vec![];
    let n = 121;
    for i in 0..n {
        let t = p * i as f64 / (n - 1) as f64;
        let b = s.with(&[("t", t)])?;
        path.push((k::num_of(&b, "nu")? * DEG, k::num_of(&b, "r")?));
    }
    let sc = 180.0 / (a * (1.0 + e));
    let fo = (300.0, 210.0);
    let xy = |nu: f64, r: f64| (fo.0 + r * sc * nu.cos(), fo.1 - r * sc * nu.sin());
    let mut cv = Canvas::new(420.0);
    let pts: Vec<(f64, f64)> = path.iter().map(|(nu, r)| xy(*nu, *r)).collect();
    c::poly(&mut cv, &pts, "axp-c", false);
    c::disc(&mut cv, fo, 8.0, true);
    for i in 0..16 {
        let t = p * i as f64 / 16.0;
        let b = s.with(&[("t", t)])?;
        let q = xy(k::num_of(&b, "nu")? * DEG, k::num_of(&b, "r")?);
        cv.dot(q);
    }
    let now = xy(s.out("nu")? * DEG, s.out("r")?);
    cv.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", now.0, now.1));
    cv.dashed(fo, now);
    cv.bold(16.0, 24.0, "start", &format!("at t = {}: ν = {}°, r = {}", k::q(t0, "s"), k::num(s.out("nu")?), k::q(s.out("r")?, "m")));
    cv.value(16.0, 44.0, "start", &format!("v = {}, P = {}", c::qv(s.out("v")?, "m/s"), k::q(p, "s")));
    fig(s, "orbit", "Positions on an elliptical orbit at equal times", &format!(
        "An orbit with a = {} and e = {}: 121 positions (true anomaly and distance) solved by the model from Kepler's \
         equation over one period P = {}, and 16 positions at equal time steps (dots): equal areas swept, bunched near \
         apoapsis. Marked: the given time {}, ν = {}°, r = {}, speed {}.", k::q(a, "m"), k::num(e), k::q(p, "s"),
        k::q(t0, "s"), k::num(s.out("nu")?), k::q(s.out("r")?, "m"), c::qv(s.out("v")?, "m/s")), &cv)
}

fn launch_orbits(s: &Solved) -> Option<Figure> {
    let (r, v) = (c::inp(s, "r")?, c::inp(s, "v")?);
    let mut cv = Canvas::new(400.0);
    let fo = (300.0, 200.0);
    let mut orbits = vec![];
    for fct in [0.8, 1.0, 1.2, 1.3] {
        let Some(b) = s.with(&[("v", v * fct)]) else { continue };
        let (Some(a), Some(e), Some(rp), Some(ra)) = (k::num_of(&b, "a"), k::num_of(&b, "e"), k::num_of(&b, "rp"), k::num_of(&b, "ra")) else { continue };
        if a > 0.0 && e < 1.0 {
            orbits.push((fct, a, e, rp, ra));
        }
    }
    if orbits.is_empty() {
        return None;
    }
    let ext = orbits.iter().map(|o| o.4).fold(r, f64::max);
    let sc = 240.0 / ext;
    for (i, (fct, a, e, rp, _)) in orbits.iter().enumerate() {
        // the launch point is on the +x side: periapsis there when the launch is at periapsis, else apoapsis
        let at_peri = (rp - r).abs() <= (orbits[i].4 - r).abs();
        let a_px = a * sc;
        let b_px = a_px * (1.0 - e * e).sqrt();
        let cx = if at_peri { fo.0 - a_px * e } else { fo.0 + a_px * e };
        c::ellipse(&mut cv, (cx, fo.1), a_px, b_px, ["axp-c3", "axp-c", "axp-c2", "axp-c4"][i % 4]);
        cv.value(16.0, 48.0 + 18.0 * i as f64, "start", &format!("{} v: a = {}, e = {}", k::num(*fct), k::q(*a, "m"), k::num(*e)));
    }
    c::disc(&mut cv, fo, 7.0, true);
    cv.dot((fo.0 + r * sc, fo.1));
    c::varrow(&mut cv, (fo.0 + r * sc, fo.1), (fo.0 + r * sc, fo.1 - 50.0), "axp-k");
    cv.bold(16.0, 24.0, "start", &format!("launched at r = {} with v = {} and its multiples", k::q(r, "m"), c::qv(v, "m/s")));
    fig(s, "orbit", "Orbits launched from one point at different speeds", &format!(
        "Orbits launched from the same point r = {} at 0.8, 1, 1.2 and 1.3 times the given speed {} (the given flight-path \
         angle), each ellipse drawn from the semi-major axis and eccentricity the model returns (unbound ones left out): {}.",
        k::q(r, "m"), c::qv(v, "m/s"), orbits.iter().map(|(fct, a, e, rp, ra)| format!(
            "{} v: a = {}, e = {}, periapsis {}, apoapsis {}", k::num(*fct), k::q(*a, "m"), k::num(*e), k::q(*rp, "m"),
            k::q(*ra, "m"))).collect::<Vec<_>>().join("; ")), &cv)
}

fn equal_sweeps(s: &Solved) -> Option<Figure> {
    let (a, e) = (c::inp(s, "a")?, c::inp(s, "e")?);
    let dt = s.out("dt")?;
    let p = s.out("P")?;
    let t_peri = s.with(&[("nu1", -45.0 * DEG), ("nu2", 45.0 * DEG)]).and_then(|b| k::num_of(&b, "dt"))?;
    let t_apo = s.with(&[("nu1", 135.0 * DEG), ("nu2", 225.0 * DEG)]).and_then(|b| k::num_of(&b, "dt"))?;
    let sc = 160.0 / (a * (1.0 + e));
    let fo = (300.0, 200.0);
    let rr = |nu: f64| a * (1.0 - e * e) / (1.0 + e * nu.cos());
    let mut cv = Canvas::new(380.0);
    for (lo, hi, class) in [(-45.0, 45.0, "axp-s"), (135.0, 225.0, "axp-s2")] {
        let mut w = vec![fo];
        for i in 0..=30 {
            let nu = (lo + (hi - lo) * i as f64 / 30.0) * DEG;
            w.push((fo.0 + rr(nu) * sc * nu.cos(), fo.1 - rr(nu) * sc * nu.sin()));
        }
        c::area(&mut cv, &w, class);
    }
    focus_ellipse(&mut cv, fo, a * sc, e, "axp-c");
    c::disc(&mut cv, fo, 6.0, true);
    cv.value(fo.0 + rr(0.0) * sc + 8.0, fo.1 + 4.0, "start", &k::q(t_peri, "s"));
    cv.value(fo.0 - rr(PI) * sc - 8.0, fo.1 + 4.0, "end", &k::q(t_apo, "s"));
    cv.bold(16.0, 24.0, "start", &format!("period {}; given sweep {}", k::q(p, "s"), k::q(dt, "s")));
    fig(s, "orbit", "Equal angles, unequal times: Kepler's second law", &format!(
        "Equal 90° sweeps of true anomaly on an orbit with a = {} and e = {}: from -45° to 45° about periapsis the model \
         returns {}, from 135° to 225° about apoapsis {}, of a period of {}; the shaded areas take times in proportion to \
         their areas (Kepler's second law; the shapes are drawn from the conic r = a(1 - e²)/(1 + e cos ν)). The given \
         sweep takes {}.", k::q(a, "m"), k::num(e), k::q(t_peri, "s"), k::q(t_apo, "s"), k::q(p, "s"), k::q(dt, "s")), &cv)
}

// ------------------------------------------------------------------------------------------------ the sky
fn sky_paths(s: &Solved) -> Option<Figure> {
    let (h0, dec0, lat) = (c::inp(s, "H")?, c::inp(s, "dec")?, c::inp(s, "lat")?);
    let (alt0, az0) = (s.out("alt_deg")?, s.out("az_deg")?);
    let mut cv = Canvas::new(440.0);
    let ctr = (230.0, 230.0);
    let rp = 180.0;
    c::ellipse(&mut cv, ctr, rp, rp, "axp-k");
    c::ellipse(&mut cv, ctr, rp * 2.0 / 3.0, rp * 2.0 / 3.0, "axp-c4");
    c::ellipse(&mut cv, ctr, rp / 3.0, rp / 3.0, "axp-c4");
    // north up, east to the left (looking up at the sky)
    let xy = |alt: f64, az: f64| {
        let r = rp * (90.0 - alt) / 90.0;
        (ctr.0 - r * (az * DEG).sin(), ctr.1 - r * (az * DEG).cos())
    };
    for (t, az) in [("N", 0.0), ("E", 90.0), ("S", 180.0), ("W", 270.0)] {
        let p = xy(-8.0, az);
        cv.bold(p.0, p.1 + 5.0, "middle", t);
    }
    let decs = [60.0, 20.0, 0.0, -30.0];
    let mut notes = vec![];
    for (i, d) in decs.iter().enumerate() {
        let mut seg = vec![];
        let mut top: Option<(f64, f64)> = None;
        for j in 0..=60 {
            let h = -PI + 2.0 * PI * j as f64 / 60.0;
            let Some(b) = s.with(&[("H", h), ("dec", d * DEG)]) else { continue };
            let (Some(al), Some(az)) = (k::num_of(&b, "alt_deg"), k::num_of(&b, "az_deg")) else { continue };
            if al >= 0.0 {
                seg.push(xy(al, az));
                if top.map(|t| al > t.0).unwrap_or(true) {
                    top = Some((al, az));
                }
            } else if seg.len() >= 2 {
                c::poly(&mut cv, &seg, ["axp-c3", "axp-c", "axp-c2", "axp-c4"][i], false);
                seg.clear();
            } else {
                seg.clear();
            }
        }
        if seg.len() >= 2 {
            c::poly(&mut cv, &seg, ["axp-c3", "axp-c", "axp-c2", "axp-c4"][i], false);
        }
        if let Some((al, az)) = top {
            notes.push(format!("δ = {}°: highest at altitude {}°, azimuth {}°", k::num(*d), k::num(al), k::num(az)));
            cv.value(430.0, 60.0 + 20.0 * i as f64, "start", &format!("δ = {}°", k::num(*d)));
        }
    }
    let p = xy(alt0.max(0.0), az0);
    if alt0 >= 0.0 {
        cv.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", p.0, p.1));
    }
    cv.bold(16.0, 24.0, "start", &format!("latitude {}°", k::num(lat / DEG)));
    cv.value(16.0, 430.0, "start", &format!("given: alt {}°, az {}° (H = {}°, δ = {}°)", k::num(alt0), k::num(az0),
                                           k::num(h0 / DEG), k::num(dec0 / DEG)));
    fig(s, "polar", "Daily paths of stars across the sky", &format!(
        "Daily paths across the sky of stars of four declinations seen from latitude {}° (azimuth around the rim, north \
         up and east to the left, zenith distance outward; 61 hour angles each converted by the model to altitude and \
         azimuth, only the part above the horizon drawn): {}. Marked: the given position, altitude {}°, azimuth {}°.",
        k::num(lat / DEG), notes.join("; "), k::num(alt0), k::num(az0)), &cv)
}

fn meridian(s: &Solved) -> Option<Figure> {
    let (dec0, lat) = (c::inp(s, "dec")?, c::inp(s, "lat")?);
    let latd = lat / DEG;
    let mut cv = Canvas::new(380.0);
    let ctr = (260.0, 210.0);
    let rp = 150.0;
    c::ellipse(&mut cv, ctr, rp, rp, "axp-c4");
    cv.line((ctr.0 - rp - 20.0, ctr.1), (ctr.0 + rp + 20.0, ctr.1));
    cv.name(ctr.0 - rp - 24.0, ctr.1 + 4.0, "end", "N");
    cv.name(ctr.0 + rp + 24.0, ctr.1 + 4.0, "start", "S");
    // angles measured anticlockwise from the southern horizon (to the right)
    let at = |ang: f64| (ctr.0 + rp * (ang * DEG).cos(), ctr.1 - rp * (ang * DEG).sin());
    let pole = at(180.0 - latd);
    cv.line(ctr, pole);
    cv.name(pole.0 - 6.0, pole.1 - 6.0, "end", "celestial pole");
    let eq_ = at(90.0 - latd);
    cv.dashed(at(270.0 - latd), eq_);
    cv.name(eq_.0 + 6.0, eq_.1 - 4.0, "start", "equator");
    let mut decs: Vec<f64> = vec![dec0 / DEG, 60.0, -10.0];
    decs.dedup_by(|a, b| (*a - *b).abs() < 1.0);
    let mut notes = vec![];
    for (i, d) in decs.iter().enumerate() {
        let b = s.with(&[("dec", d * DEG)])?;
        let (up, lo) = (k::num_of(&b, "alt_upper_deg")?, k::num_of(&b, "alt_lower_deg")?);
        let au = if *d <= latd { up } else { 180.0 - up };
        let al = 180.0 - lo;
        let (p, q) = (at(au), at(al));
        let class = if i == 0 { "axp-c" } else { "axp-c3" };
        cv.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p.0, p.1, q.0, q.1));
        cv.dot(p);
        cv.dot(q);
        cv.value(p.0 + 6.0, p.1 - 6.0, "start", &format!("{}°", k::num(up)));
        notes.push(format!("δ = {}°: upper {}°, lower {}°", k::num(*d), k::num(up), k::num(lo)));
    }
    cv.bold(16.0, 24.0, "start", &format!("the meridian seen edge-on from latitude {}°", k::num(latd)));
    for (i, n) in notes.iter().enumerate() {
        cv.value(16.0, 300.0 + 20.0 * i as f64, "start", n);
    }
    fig(s, "sky", "Culminations on the meridian", &format!(
        "The meridian seen edge-on from latitude {}°: horizon from north to south, the celestial pole at the latitude's \
         altitude, the celestial equator (dashed) at right angles, and stars whose daily circles appear as chords from \
         upper to lower culmination, the given declination among them (the culmination altitudes the model returns place \
         the ends): {}.", k::num(latd), notes.join("; ")), &cv)
}

fn companions(s: &Solved) -> Option<Figure> {
    let (ra1, dec1) = (c::inp(s, "ra1")?, c::inp(s, "dec1")?);
    let pa0 = s.out("pa_deg")?;
    let offs = [(0.0, 10.0), (8.0, 6.0), (10.0, 0.0), (6.0, -8.0), (-6.0, -8.0), (-10.0, 2.0)];
    let mut cv = Canvas::new(440.0);
    let ctr = (240.0, 230.0);
    let sc = 15.0;
    c::disc(&mut cv, ctr, 8.0, true);
    c::varrow(&mut cv, ctr, (ctr.0, ctr.1 - 190.0), "axp-c4");
    cv.name(ctr.0 + 6.0, ctr.1 - 180.0, "start", "N");
    c::varrow(&mut cv, ctr, (ctr.0 - 190.0, ctr.1), "axp-c4");
    cv.name(ctr.0 - 186.0, ctr.1 - 8.0, "start", "E");
    let cosd = dec1.cos().max(1e-6);
    let mut notes = vec![];
    for (e, n) in offs {
        let (ra2, dec2) = (ra1 + e / 3600.0 * DEG / cosd, dec1 + n / 3600.0 * DEG);
        let Some(b) = s.with(&[("ra2", ra2), ("dec2", dec2)]) else { continue };
        let Some(pa) = k::num_of(&b, "pa_deg") else { continue };
        let p = (ctr.0 - e * sc, ctr.1 - n * sc);
        cv.dashed(ctr, p);
        cv.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", p.0, p.1));
        cv.value(p.0 + 8.0, p.1 - 6.0, "start", &format!("{}°", k::num(pa)));
        notes.push(format!("({e}″ E, {n}″ N) → {}°", k::num(pa)));
    }
    if notes.is_empty() {
        return None;
    }
    cv.bold(16.0, 24.0, "start", &format!("given: position angle {}°", k::num(pa0)));
    fig(s, "sky", "Position angles of companions around a star", &format!(
        "Position angles of six companions around a star at the given first position, drawn as on the sky with north up \
         and east to the left; each companion's offset in arcseconds east and north is stated (the right-ascension offset \
         divided by cos δ). The model measures each angle from north through east: {}. The given pair: {}°.",
        notes.join("; "), k::num(pa0)), &cv)
}

fn fields_of_view(s: &Solved) -> Option<Figure> {
    let (f0, p, scale0) = (c::inp(s, "f")?, c::inp(s, "p")?, s.out("pixel_scale_arcsec")?);
    let n = c::inp(s, "N")?;
    let moon = oi(s, "angular_size", &[("D", f(3.4748e6)), ("d", f(3.844e8))], "theta_arcsec")? / 60.0;
    let mut fov = vec![];
    for ff in [0.8 * f0, f0, 2.0 * f0, 5.6 * f0] {
        let b = s.with(&[("f", ff)])?;
        fov.push((ff, k::num_of(&b, "fov_arcmin")?, k::num_of(&b, "pixel_scale_arcsec")?));
    }
    let big = fov.iter().map(|x| x.1).fold(moon, f64::max);
    let sc = 300.0 / big;
    let mut cv = Canvas::new(440.0);
    let ctr = (200.0, 240.0);
    c::disc(&mut cv, ctr, (moon / 2.0 * sc).max(2.0), true);
    for (i, (ff, fv, _)) in fov.iter().enumerate() {
        let h = fv * sc / 2.0;
        cv.raw(&format!("<rect class=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\"/>",
                        ["axp-c3", "axp-c", "axp-c2", "axp-c4"][i], ctr.0 - h, ctr.1 - h, 2.0 * h, 2.0 * h));
        cv.value(370.0, 80.0 + 22.0 * i as f64, "start", &format!("f = {}: {}′", k::q(*ff, "m"), k::num(*fv)));
    }
    cv.name(370.0, 180.0, "start", &format!("Moon: {}′", k::num(moon)));
    cv.bold(16.0, 24.0, "start", &format!("{} pixels of {}: {}″ per pixel at f = {}", k::num(n), k::q(p, "m"), k::num(scale0), k::q(f0, "m")));
    fig(s, "field-of-view", "Fields of view of one camera behind four telescopes", &format!(
        "Fields of view of one camera ({} pixels of {}) behind telescopes of focal length 0.8, 1, 2 and 5.6 times the given \
         {}, drawn to scale around the full Moon, whose angular diameter ({}′) the model's angular-size calculation \
         returns for 3.4748e6 m at 3.844e8 m. Marked: {}.", k::num(n), k::q(p, "m"), k::q(f0, "m"), k::num(moon),
        fov.iter().map(|(ff, fv, ps)| format!("f = {} → {}′ field, {}″ per pixel", k::q(*ff, "m"), k::num(*fv), k::num(*ps)))
            .collect::<Vec<_>>().join("; ")), &cv)
}

fn transits(s: &Solved) -> Option<Figure> {
    let (rp0, rs0) = (c::inp(s, "Rp")?, c::inp(s, "Rs")?);
    let d0 = s.out("depth_ppm")?;
    let b0 = if takes(s, "b") { c::inp(s, "b").unwrap_or(0.0) } else { 0.0 };
    let cases = [("Earth", 6.371e6, RSUN, 0.0), ("Neptune", 2.4622e7, RSUN, 0.0), ("Jupiter", 7.1492e7, RSUN, 0.0),
                 ("Jupiter, b = 0.95", 7.1492e7, RSUN, 0.95), ("given", rp0, rs0, b0)];
    let mut cv = Canvas::new(230.0);
    let mut notes = vec![];
    for (i, (n, rp, rs, b)) in cases.iter().enumerate() {
        let mut set = vec![("Rp", *rp), ("Rs", *rs)];
        if takes(s, "b") {
            set.push(("b", *b));
        }
        let d = if i == 4 { d0 } else { c::at(s, &set, "depth_ppm")? };
        let cx = 56.0 + 102.0 * i as f64;
        let r = 42.0;
        c::disc(&mut cv, (cx, 110.0), r, true);
        let pr = (rp / rs * r).max(1.2);
        let py = 110.0 + b * r;
        cv.dashed((cx - r - 4.0, py), (cx + r + 4.0, py));
        cv.raw(&format!("<circle class=\"axp-kd\" cx=\"{cx:.1}\" cy=\"{py:.1}\" r=\"{pr:.1}\"/>"));
        cv.name(cx, 172.0, "middle", n);
        cv.value(cx, 190.0, "middle", &format!("{} ppm", k::num(d)));
        notes.push(format!("{n} {} ppm", k::num(d)));
    }
    cv.bold(16.0, 24.0, "start", "planets crossing stars, to scale (each star drawn the same size)");
    fig(s, "sketch", "Planets crossing a star and the dimming they cause", &format!(
        "Planets crossing a Sun-sized star (radius 6.957e8 m), drawn to scale: Earth (6.371e6 m), Neptune (2.4622e7 m) \
         and Jupiter (7.1492e7 m) through the centre, Jupiter on a chord with impact parameter 0.95 (stated; dashed: the \
         path), and the given planet across its star, drawn with its star at the same size. Marked: the fractional \
         dimming the model returns, {}.", notes.join(", ")), &cv)
}
