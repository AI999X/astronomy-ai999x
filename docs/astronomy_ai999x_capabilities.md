# astronomy-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `astronomy_ai999x.about()` for the same answer.

Every capability astronomy-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import astronomy_ai999x

result = astronomy_ai999x.solve('distance_unit_conversion', d='3.085677581491367e+16 m')
result.show()
# d_m = 3.08568e+16 m, d_km = 3.08568e+13, d_au = 206265, d_ly = 3.26157, d_pc = 1
```

## By topic

| topic | capabilities |
|---|---|
| Compact objects / black holes | [Schwarzschild radius](#schwarzschild_radius) |
| Cosmology / angular scale | [Angular size of an object at a redshift](#angular_size_at_redshift) |
| Cosmology / distances | [Cosmological distances at a redshift](#cosmological_distances) |
| Cosmology / Friedmann equation | [Hubble parameter and critical density](#critical_density) |
| Cosmology / redshift | [Radial velocity from a Doppler redshift](#recession_velocity_from_redshift), [Redshift and radial velocity from a spectral line](#redshift_from_wavelengths) |
| Cosmology / time | [Lookback time to a redshift](#lookback_time), [Age of the universe at a redshift](#age_at_redshift), [Redshift at a given lookback time](#redshift_at_lookback_time) |
| Cosmology / volumes | [Comoving volume out to a redshift](#comoving_volume) |
| Distances and magnitudes / absolute magnitude | [Absolute magnitude](#absolute_magnitude) |
| Distances and magnitudes / distance modulus | [Distance from the distance modulus](#distance_from_distance_modulus) |
| Distances and magnitudes / distance units | [Astronomical distance units](#distance_unit_conversion) |
| Distances and magnitudes / magnitude scale | [Flux ratio from a magnitude difference](#flux_ratio_from_magnitudes), [Magnitude difference from a flux ratio](#magnitude_difference_from_flux_ratio), [Combined magnitude of two stars](#combined_magnitude) |
| Distances and magnitudes / parallax | [Distance from parallax](#parallax_distance) |
| Distances and magnitudes / standard candles | [Distance from luminosity and flux (inverse-square law)](#distance_from_flux) |
| Exoplanets / radial velocity | [Radial-velocity semi-amplitude of a star](#radial_velocity_semi_amplitude) |
| Exoplanets / transits | [Transit depth](#transit_depth), [Transit duration](#transit_duration) |
| Gravitational lensing / point lens | [Einstein radius of a point-mass lens](#einstein_radius) |
| Observing / angular size | [Angular size](#angular_size), [Linear size from angular size](#linear_size_from_angle) |
| Observing / atmosphere | [Airmass and atmospheric extinction](#airmass_extinction) |
| Orbits / binary stars | [Masses of a visual binary star](#binary_star_masses) |
| Orbits / elliptical orbits | [Speeds at periapsis and apoapsis](#apsis_speeds) |
| Orbits / Kepler's equation | [Position on an elliptical orbit at a given time (Kepler's equation)](#kepler_position_at_time), [Time of flight between two points of an elliptical orbit](#time_of_flight) |
| Orbits / orbit determination | [Orbital elements from position and velocity](#orbit_from_state) |
| Orbits / orbital manoeuvres | [Hohmann transfer between circular orbits](#hohmann_transfer), [Departure burn from a circular orbit](#escape_delta_v), [Delta-v of a combined plane change](#plane_change_delta_v) |
| Planets / atmospheres | [Scale height of an isothermal atmosphere](#atmospheric_scale_height) |
| Planets / equilibrium temperature | [Equilibrium temperature of a planet](#planet_equilibrium_temperature) |
| Plasma physics / frequencies | [Plasma frequency](#plasma_frequency), [Upper hybrid frequency](#upper_hybrid_frequency), [Lower hybrid frequency](#lower_hybrid_frequency) |
| Plasma physics / lengths | [Debye length](#debye_length), [Inertial length (skin depth)](#inertial_length) |
| Plasma physics / particle motion | [Gyrofrequency (cyclotron frequency)](#gyrofrequency), [Gyroradius (Larmor radius)](#gyroradius) |
| Plasma physics / plasma parameter | [Number of particles in a Debye sphere](#debye_number) |
| Plasma physics / pressures | [Plasma beta](#plasma_beta), [Magnetic pressure](#magnetic_pressure) |
| Plasma physics / velocities | [Thermal speed of plasma particles](#thermal_speed) |
| Plasma physics / waves | [Alfven speed](#alfven_speed) |
| Positional astronomy / coordinates | [Altitude and azimuth from hour angle and declination](#altaz_from_hadec), [Angular separation of two sky positions](#angular_separation), [Position angle between two sky positions](#position_angle) |
| Positional astronomy / meridian transit | [Altitude of a star at culmination](#culmination_altitude) |
| Positional astronomy / rising and setting | [Rising and setting of a star](#rise_set_hour_angle) |
| Stars / bolometric magnitude | [Absolute bolometric magnitude from luminosity](#bolometric_magnitude_from_luminosity), [Luminosity from absolute bolometric magnitude](#luminosity_from_bolometric_magnitude), [Apparent bolometric magnitude and flux](#apparent_bolometric_magnitude) |
| Stars / effective temperature | [Effective temperature of a star](#effective_temperature) |
| Stars / luminosity | [Luminosity of a star (Stefan-Boltzmann)](#stellar_luminosity) |
| Stars / radius | [Radius of a star from luminosity and temperature](#stellar_radius) |
| Stars / star formation | [Free-fall time of a uniform cloud](#free_fall_time) |
| Stars / stellar atmospheres | [Ionization fraction of hydrogen (Saha equation)](#saha_hydrogen_ionization) |
| Telescopes / imaging | [Pixel scale and field of view of a telescope camera](#camera_pixel_scale) |
| Telescopes / light gathering | [Light-gathering power of two telescopes](#light_gathering_power) |
| Telescopes / magnification | [Telescope magnification, exit pupil and true field](#telescope_magnification) |
| Telescopes / resolution | [Diffraction limit of a telescope (Rayleigh criterion)](#rayleigh_resolution) |
| Time / calendars | [Julian date of a calendar date](#julian_date), [Calendar date from a Julian date](#calendar_date_from_julian_date) |
| Time / sidereal time | [Greenwich and local mean sidereal time](#sidereal_time) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| distance_unit_conversion | Astronomical distance units | d | d_m, d_km, d_au, d_ly, d_pc |
| parallax_distance | Distance from parallax | p | d, d_pc, d_ly, p_arcsec |
| distance_from_distance_modulus | Distance from the distance modulus | m, M, A | d, d_pc, d_ly, mu |
| absolute_magnitude | Absolute magnitude | m, d, A | M, mu, d_pc |
| flux_ratio_from_magnitudes | Flux ratio from a magnitude difference | m1, m2 | ratio, dm |
| magnitude_difference_from_flux_ratio | Magnitude difference from a flux ratio | ratio | dm |
| combined_magnitude | Combined magnitude of two stars | m1, m2 | m, brightening |
| bolometric_magnitude_from_luminosity | Absolute bolometric magnitude from luminosity | L | M_bol, L_Lsun |
| luminosity_from_bolometric_magnitude | Luminosity from absolute bolometric magnitude | M_bol | L, L_Lsun |
| apparent_bolometric_magnitude | Apparent bolometric magnitude and flux | L, d | m_bol, F, M_bol |
| angular_size | Angular size | D, d | theta, theta_arcsec, theta_deg |
| linear_size_from_angle | Linear size from angular size | theta, d | D, D_km, D_au |
| stellar_luminosity | Luminosity of a star (Stefan-Boltzmann) | R, T | L, L_Lsun |
| stellar_radius | Radius of a star from luminosity and temperature | L, T | R, R_Rsun |
| effective_temperature | Effective temperature of a star | L, R | T |
| planet_equilibrium_temperature | Equilibrium temperature of a planet | T_star, R_star, a, A | T_eq, S |
| saha_hydrogen_ionization | Ionization fraction of hydrogen (Saha equation) | T, n | x, neutral, n_e |
| schwarzschild_radius | Schwarzschild radius | M | rs, rs_km, M_Msun |
| atmospheric_scale_height | Scale height of an isothermal atmosphere | T, mu, g | H, H_km |
| free_fall_time | Free-fall time of a uniform cloud | rho | t_ff, t_ff_yr |
| binary_star_masses | Masses of a visual binary star | P, a1, a2 | M_total, M_total_Msun, M1_Msun, M2_Msun, a_au |
| hohmann_transfer | Hohmann transfer between circular orbits | r1, r2, M | dv1, dv2, dv, t_transfer, t_days, a_transfer |
| apsis_speeds | Speeds at periapsis and apoapsis | rp, ra, M | vp, va, a, e, P |
| escape_delta_v | Departure burn from a circular orbit | M, r, v_inf | dv, v_circ, v_depart |
| plane_change_delta_v | Delta-v of a combined plane change | v1, v2, di | dv |
| orbit_from_state | Orbital elements from position and velocity | M, r, v, gamma | a, e, rp, ra, P, energy |
| kepler_position_at_time | Position on an elliptical orbit at a given time (Kepler's equation) | M, a, e, t | mean_anomaly, E, nu, r, v, P |
| time_of_flight | Time of flight between two points of an elliptical orbit | M, a, e, nu1, nu2 | dt, dt_days, P |
| radial_velocity_semi_amplitude | Radial-velocity semi-amplitude of a star | M_star, m_p, P, e, i | K, a, a_au |
| transit_depth | Transit depth | Rp, Rs, b | depth, depth_ppm, k, grazing |
| transit_duration | Transit duration | P, a, Rs, Rp, i | T14, T14_h, T23, b |
| cosmological_distances | Cosmological distances at a redshift | z, H0, Om | D_C, D_C_Mpc, D_L_Mpc, D_A_Mpc, mu, D_L |
| angular_size_at_redshift | Angular size of an object at a redshift | z, H0, Om, D | theta_arcsec, scale_kpc_per_arcsec, D_A_Mpc |
| lookback_time | Lookback time to a redshift | z, H0, Om | t_L, t_L_Gyr |
| age_at_redshift | Age of the universe at a redshift | z, H0, Om | age, age_Gyr, age_now_Gyr |
| redshift_at_lookback_time | Redshift at a given lookback time | t, H0, Om | z, age_now_Gyr |
| comoving_volume | Comoving volume out to a redshift | z, H0, Om | V, V_Gpc3 |
| recession_velocity_from_redshift | Radial velocity from a Doppler redshift | z | v, beta, v_cz |
| redshift_from_wavelengths | Redshift and radial velocity from a spectral line | lam_obs, lam_rest | z, v, beta, d_lambda |
| rayleigh_resolution | Diffraction limit of a telescope (Rayleigh criterion) | lam, D | theta, theta_arcsec |
| telescope_magnification | Telescope magnification, exit pupil and true field | f_obj, f_eye, D, afov | M, exit_pupil, exit_pupil_mm, tfov_deg |
| light_gathering_power | Light-gathering power of two telescopes | D1, D2 | area_ratio, mag_gain |
| camera_pixel_scale | Pixel scale and field of view of a telescope camera | f, p, N | pixel_scale_arcsec, fov_deg, fov_arcmin |
| airmass_extinction | Airmass and atmospheric extinction | zen, k, m | X, alt_deg, extinction, m0 |
| altaz_from_hadec | Altitude and azimuth from hour angle and declination | H, dec, lat | alt_deg, az_deg |
| culmination_altitude | Altitude of a star at culmination | dec, lat | alt_upper_deg, alt_lower_deg, circumpolar, never_rises |
| rise_set_hour_angle | Rising and setting of a star | dec, lat | H_set_h, time_up_sidereal_h, az_rise_deg, az_set_deg |
| angular_separation | Angular separation of two sky positions | ra1, dec1, ra2, dec2 | sep_deg, sep_arcsec |
| position_angle | Position angle between two sky positions | ra1, dec1, ra2, dec2 | pa_deg |
| julian_date | Julian date of a calendar date | year, month, day, hour, minute, second | JD, MJD |
| calendar_date_from_julian_date | Calendar date from a Julian date | jd | year, month, day, hours |
| sidereal_time | Greenwich and local mean sidereal time | jd, lon | gmst_h, lmst_h |
| plasma_frequency | Plasma frequency | n, particle | omega_p, f_p |
| debye_length | Debye length | T, n | lambda_D, lambda_D_nm |
| debye_number | Number of particles in a Debye sphere | T, n | N_D, lambda_D |
| alfven_speed | Alfven speed | B, n | v_A, v_A_km_s |
| gyrofrequency | Gyrofrequency (cyclotron frequency) | B, particle | omega_c, f_c, T_c |
| gyroradius | Gyroradius (Larmor radius) | B, v_perp, particle | r_L, r_L_km, r_L_nm |
| thermal_speed | Thermal speed of plasma particles | T, convention, particle | v_th |
| plasma_beta | Plasma beta | T, n, B | beta, p_thermal, p_magnetic |
| magnetic_pressure | Magnetic pressure | B | p_B, energy_density, p_B_nPa |
| upper_hybrid_frequency | Upper hybrid frequency | B, n | omega_uh, f_uh |
| lower_hybrid_frequency | Lower hybrid frequency | B, n | omega_lh, f_lh |
| inertial_length | Inertial length (skin depth) | n, particle | d, d_km |
| einstein_radius | Einstein radius of a point-mass lens | M, D_l, D_s | theta_E, theta_E_arcsec, R_E, R_E_au |
| distance_from_flux | Distance from luminosity and flux (inverse-square law) | L, F | d, d_pc, d_ly |
| critical_density | Hubble parameter and critical density | H0, Om, z | H_z, rho_c, n_H |

<a id="distance_unit_conversion"></a>

### Astronomical distance units: `distance_unit_conversion`

A distance expressed in metres, kilometres, astronomical units, light-years and parsecs.

$$
1\,\mathrm{pc} = \frac{648000}{\pi}\,\mathrm{au},\quad 1\,\mathrm{ly} = c \times 365.25\,\mathrm{d}
$$

| input | meaning | condition |
|---|---|---|
| `d` | distance (m, km, au, ly, pc, kpc, Mpc ...) |  |

| output | meaning |
|---|---|
| `d_m` | distance in metres |
| `d_km` | distance in kilometres |
| `d_au` | distance in astronomical units |
| `d_ly` | distance in light-years |
| `d_pc` | distance in parsecs |

- **Method:** 1 $au = 149597870700$ m; 1 $pc = au 648000/pi$; 1 $ly = c 365.25$ days
- **Assumes:** $d > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Distances of six objects (stated in metres: the Moon 3](https://www.ai999x.com/?ax_fig=astronomy-ai999x/distance-unit-conversion-bars)

*Distances of six objects (stated in metres: the Moon 3.844e8, the Sun 1 au, Neptune 30.07 au, Proxima Centauri 1.302 pc, the galactic centre 8.18 kpc, Andromeda 765 kpc), each converted by the model; bars in light-years. Each bar is a solve of the model; the last is the given inputs. Marked: Moon 4.063e-08 ly, Sun 1.581e-05 ly, Neptune 0.0004755 ly, Proxima 4.247 ly, Gal. centre 2.668e+04 ly, Andromeda 2.495e+06 ly, given 4.24 ly. Your inputs: d = 4.01135e+16 m.*

![Distance in parsecs as d runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/distance-unit-conversion-live-1)

*Distance in parsecs as d runs from 2.407e+16 to 5.616e+16 m, every other input as you entered it (41 solutions). Marked: your d = 4.011e+16 m gives d_pc = 1.3. Your inputs: d = 4.01135e+16 m.*

```python
astronomy_ai999x.solve('distance_unit_conversion', d=3.085677581491367e+16)
# -> d_m = 3.08568e+16 m, d_km = 3.08568e+13, d_au = 206265, d_ly = 3.26156, d_pc = 1   (SOLVED)
```

```python
astronomy_ai999x.solve('distance_unit_conversion', d=149597870700.0)
# -> d_m = 1.49598e+11 m, d_km = 1.49598e+08, d_au = 1, d_ly = 1.58125e-05, d_pc = 4.84814e-06   (SOLVED)
```

```python
astronomy_ai999x.solve('distance_unit_conversion', d=9460730472580800.0)
# -> d_m = 9.46073e+15 m, d_km = 9.46073e+12, d_au = 63241.1, d_ly = 1, d_pc = 0.306601   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Distance conversion: d = 1 pc | SOLVED | d_m = 3.08568e+16 m, d_km = 3.08568e+13, d_au = 206265, d_ly = 3.26156, d_pc = 1 |
| Distance conversion: d = 1 ly | SOLVED | d_m = 9.46073e+15 m, d_km = 9.46073e+12, d_au = 63241.1, d_ly = 1, d_pc = 0.306601 |
| Distance conversion: d = -3 ly | INVALID_INPUT | Astronomical distance units: the distance must be a positive number. |
| Distance conversion: d = 5 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: astronomy_ai999x.templates(); eve |

<a id="parallax_distance"></a>

### Distance from parallax: `parallax_distance`

The distance of a star from its annual trigonometric parallax: d [pc] = $1 / p$ [arcsec].

$$
d\,[\mathrm{pc}] = \frac{1}{p\,[\mathrm{arcsec}]}
$$

| input | meaning | condition |
|---|---|---|
| `p` | annual parallax (arcsec, mas, rad, deg) |  |

| output | meaning |
|---|---|
| `d` | distance |
| `d_pc` | distance in parsecs |
| `d_ly` | distance in light-years |
| `p_arcsec` | parallax in arcseconds |

- **Method:** $d = 1/p$ with p in arcseconds gives parsecs
- **Assumes:** $p > 0$ (small-angle definition of the parsec)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Distance in parsecs as p runs from 2](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/parallax-distance-live-1.svg)

*Distance in parsecs as p runs from 2.234e-06 to 5.213e-06 rad, every other input as you entered it (41 solutions). Marked: your p = 3.723e-06 rad gives d_pc = 1.302. Your inputs: p = 3.72337e-06 rad.*

```python
astronomy_ai999x.solve('parallax_distance', p=4.84813681109536e-06)
# -> d = 3.08568e+16 m, d_pc = 1, d_ly = 3.26156, p_arcsec = 1   (SOLVED)
```

```python
astronomy_ai999x.solve('parallax_distance', p=4.84813681109536e-07)
# -> d = 3.08568e+17 m, d_pc = 10, d_ly = 32.6156, p_arcsec = 0.1   (SOLVED)
```

```python
astronomy_ai999x.solve('parallax_distance', p=3.7233690709212365e-06)
# -> d = 4.01781e+16 m, d_pc = 1.30208, d_ly = 4.24683, p_arcsec = 0.768   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Parallax distance: p = 0.1 arcsec | SOLVED | d = 3.08568e+17 m, d_pc = 10, d_ly = 32.6156, p_arcsec = 0.1 |
| Parallax distance: p = 2 mas | SOLVED | d = 1.54284e+19 m, d_pc = 500, d_ly = 1630.78, p_arcsec = 0.002 |
| Parallax distance: p = 0 arcsec | INVALID_INPUT | Distance from parallax: the parallax must be a positive number. |
| Parallax distance: | NEEDS_INPUT | parallax_distance needs p (annual parallax (arcsec, mas, rad, deg)). Example: solve('parallax_distance', p='4.8481368110 |

<a id="distance_from_distance_modulus"></a>

### Distance from the distance modulus: `distance_from_distance_modulus`

The distance of a star from its apparent and absolute magnitudes (and an optional extinction A): $m - M - A = 5$ log10($d / 10$ pc).

$$
d = 10^{(m - M - A)/5 + 1}\ \mathrm{pc}
$$

| input | meaning | condition |
|---|---|---|
| `m` | apparent magnitude |  |
| `M` | absolute magnitude |  |
| `A` | interstellar extinction in magnitudes (default 0) |  |

| output | meaning |
|---|---|
| `d` | distance |
| `d_pc` | distance in parsecs |
| `d_ly` | distance in light-years |
| `mu` | distance modulus m - M - A |

- **Method:** $d = 10$^((m - M - A)/5 + 1) pc
- **Assumes:** $A  \geq  0$; $-10 < m - M - A < 60$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Distance in parsecs as m runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/distance-from-distance-modulus-live-1)

*Distance in parsecs as m runs from 2.625 to 18.38, every other input as you entered it (41 solutions). Marked: your m = 10.5 gives d_pc = 1000. Your inputs: m = 10.5, M = 0.5.*

![Distance in parsecs as M runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/distance-from-distance-modulus-live-2)

*Distance in parsecs as M runs from 0.125 to 0.875, every other input as you entered it (29 solutions). Marked: your M = 0.5 gives d_pc = 1000. Your inputs: m = 10.5, M = 0.5.*

```python
astronomy_ai999x.solve('distance_from_distance_modulus', m=10.0, M=0.0)
# -> d = 3.08568e+19 m, d_pc = 1000, d_ly = 3261.56, mu = 10   (SOLVED)
```

```python
astronomy_ai999x.solve('distance_from_distance_modulus', m=15.0, M=5.0, A=0.0)
# -> d = 3.08568e+19 m, d_pc = 1000, d_ly = 3261.56, mu = 10   (SOLVED)
```

```python
astronomy_ai999x.solve('distance_from_distance_modulus', m=7.0, M=2.0, A=1.0)
# -> d = 1.94693e+18 m, d_pc = 63.0957, d_ly = 205.791, mu = 4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Distance from distance modulus: m = 10, M = 0 | SOLVED | d = 3.08568e+19 m, d_pc = 1000, d_ly = 3261.56, mu = 10 |
| Distance from distance modulus: m = 7, M = 2, A = 1 | SOLVED | d = 1.94693e+18 m, d_pc = 63.0957, d_ly = 205.791, mu = 4 |
| Distance from distance modulus: m = 7, M = 2, A = -1 | INVALID_INPUT | Distance from the distance modulus: the extinction A must not be negative. |
| Distance from distance modulus: m = 10 | NEEDS_INPUT | distance_from_distance_modulus needs M (absolute magnitude). Example: solve('distance_from_distance_modulus', m=10, M=0) |

<a id="absolute_magnitude"></a>

### Absolute magnitude: `absolute_magnitude`

The absolute magnitude of a star from its apparent magnitude and distance (and an optional extinction A).

$$
M = m - 5\log_{10}\frac{d}{10\,\mathrm{pc}} - A
$$

| input | meaning | condition |
|---|---|---|
| `m` | apparent magnitude |  |
| `d` | distance (pc, ly, m ...) |  |
| `A` | interstellar extinction in magnitudes (default 0) |  |

| output | meaning |
|---|---|
| `M` | absolute magnitude |
| `mu` | distance modulus 5 log10($d/10$ pc) |
| `d_pc` | distance in parsecs |

- **Method:** $M = m - 5$ log10($d / 10$ pc) - A
- **Assumes:** $d > 0$; $A  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Apparent magnitudes and distances of six bright stars and of the given star (stated: Sirius m = -1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/absolute-magnitude-ladder)

*Apparent magnitudes and distances of six bright stars and of the given star (stated: Sirius m = -1.46 at 2.64 pc, Vega 0.03 at 7.68 pc, Arcturus -0.05 at 11.26 pc, Polaris 1.98 at 133 pc, Rigel 0.13 at 264 pc, Deneb 1.25 at 802 pc; no extinction) and the absolute magnitude the model returns for each, the magnitude the star would show from 10 pc. Marked: Sirius m = -1.46, M = 1.432; Vega m = 0.03, M = 0.6032; Arcturus m = -0.05, M = -0.3077; Polaris m = 1.98, M = -3.639; Rigel m = 0.13, M = -6.978; Deneb m = 1.25, M = -8.271; given M = 0.6032. Your inputs: m = 0.03, d = 2.3698e+17 m.*

![Absolute magnitude as d runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/absolute-magnitude-live-1)

*Absolute magnitude as d runs from 1.422e+17 to 3.318e+17 m, every other input as you entered it (41 solutions). Marked: your d = 2.37e+17 m gives M = 0.6032. Your inputs: m = 0.03, d = 2.3698e+17 m.*

![Absolute magnitude as m runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/absolute-magnitude-live-2)

*Absolute magnitude as m runs from 0.0075 to 0.0525, every other input as you entered it (29 solutions). Marked: your m = 0.03 gives M = 0.6032. Your inputs: m = 0.03, d = 2.3698e+17 m.*

```python
astronomy_ai999x.solve('absolute_magnitude', m=12.0, d=3.085677581491367e+19)
# -> M = 2, mu = 10, d_pc = 1000   (SOLVED)
```

```python
astronomy_ai999x.solve('absolute_magnitude', m=7.0, d=3.0856775814913674e+18)
# -> M = 2, mu = 5, d_pc = 100   (SOLVED)
```

```python
astronomy_ai999x.solve('absolute_magnitude', m=8.0, d=3.0856775814913674e+18, A=1.0)
# -> M = 2, mu = 5, d_pc = 100   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Absolute magnitude: m = 8, d = 100 pc, A = 1 | SOLVED | M = 2, mu = 5, d_pc = 100 |
| Absolute magnitude: m = 6.5, d = 10 pc | SOLVED | M = 6.5, mu = 0, d_pc = 10 |
| Absolute magnitude: m = 6.5, d = 0 pc | INVALID_INPUT | Absolute magnitude: the distance must be a positive number. |
| Absolute magnitude: m = 6.5 | NEEDS_INPUT | absolute_magnitude needs d (distance (pc, ly, m ...)). Example: solve('absolute_magnitude', m=12, d='3.085677581491367e+ |

<a id="flux_ratio_from_magnitudes"></a>

### Flux ratio from a magnitude difference: `flux_ratio_from_magnitudes`

How many times brighter star 1 is than star 2: F1/F2 = 10^(-0.4 $(m_{1} - m_{2})$).

$$
\frac{F_1}{F_2} = 10^{-0.4\,(m_1 - m_2)}
$$

| input | meaning | condition |
|---|---|---|
| `m1` | magnitude of the first object |  |
| `m2` | magnitude of the second object |  |

| output | meaning |
|---|---|
| `ratio` | flux ratio $F_{1}/F_{2}$ |
| `dm` | magnitude difference $m_{1} - m_{2}$ |

- **Method:** Pogson's scale: F1/F2 = 10^(-0.4 $(m_{1} - m_{2})$)
- **Assumes:** |$m_{1} - m_{2}$| <= 100
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Flux ratio F1/F2 as m2 runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/flux-ratio-from-magnitudes-live-1)

*Flux ratio F1/F2 as m2 runs from 1.5 to 10.5, every other input as you entered it (41 solutions). Marked: your m2 = 6 gives ratio = 100. Your inputs: m1 = 1, m2 = 6.*

![Flux ratio F1/F2 as m1 runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/flux-ratio-from-magnitudes-live-2)

*Flux ratio F1/F2 as m1 runs from 0.25 to 1.75, every other input as you entered it (29 solutions). Marked: your m1 = 1 gives ratio = 100. Your inputs: m1 = 1, m2 = 6.*

```python
astronomy_ai999x.solve('flux_ratio_from_magnitudes', m1=1.0, m2=6.0)
# -> ratio = 100, dm = -5   (SOLVED)
```

```python
astronomy_ai999x.solve('flux_ratio_from_magnitudes', m1=0.0, m2=2.5)
# -> ratio = 10, dm = -2.5   (SOLVED)
```

```python
astronomy_ai999x.solve('flux_ratio_from_magnitudes', m1=4.0, m2=4.0)
# -> ratio = 1, dm = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Flux ratio: m1 = 1, m2 = 6 | SOLVED | ratio = 100, dm = -5 |
| Flux ratio: m1 = 0 mag, m2 = 5 mag | SOLVED | ratio = 100, dm = -5 |
| Flux ratio: m1 = 0, m2 = 500 | INVALID_INPUT | Flux ratio from a magnitude difference: the magnitude difference must be at most 100. |
| Flux ratio: m1 = 1 | NEEDS_INPUT | flux_ratio_from_magnitudes needs m2 (magnitude of the second object). Example: solve('flux_ratio_from_magnitudes', m1=1, |

<a id="magnitude_difference_from_flux_ratio"></a>

### Magnitude difference from a flux ratio: `magnitude_difference_from_flux_ratio`

The magnitude difference $m_{1} - m_{2}$ of two objects whose fluxes are in the ratio $F_{1}/F_{2}.$

$$
m_1 - m_2 = -2.5\log_{10}\frac{F_1}{F_2}
$$

| input | meaning | condition |
|---|---|---|
| `ratio` | flux ratio $F_{1}/F_{2}$ |  |

| output | meaning |
|---|---|
| `dm` | magnitude difference $m_{1} - m_{2}$ |

- **Method:** $m_{1} - m_{2} = -2.5$ log10(F1/F2)
- **Assumes:** $F_{1}/F_{2} > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A ruler of flux ratios, one per tick, and the magnitude difference the model returns under each: a factor 10 in flux is 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/magnitude-difference-from-flux-ratio-scale)

*A ruler of flux ratios, one per tick, and the magnitude difference the model returns under each: a factor 10 in flux is 2.5 magnitudes. 1 → 0, 10 → -2.5, 100 → -5, 1000 → -7.5, 1e+04 → -10, 1e+05 → -12.5, 1e+06 → -15, 1e+07 → -17.5, 1e+08 → -20, 1e+09 → -22.5, 1e+10 → -25. Marked: the given ratio 100, Δm = -5. Your inputs: ratio = 100.*

![Magnitude difference m1 - m2 as ratio runs from 25 to 175, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/magnitude-difference-from-flux-ratio-live-1)

*Magnitude difference m1 - m2 as ratio runs from 25 to 175, every other input as you entered it (41 solutions). Marked: your ratio = 100 gives dm = -5. Your inputs: ratio = 100.*

```python
astronomy_ai999x.solve('magnitude_difference_from_flux_ratio', ratio=100.0)
# -> dm = -5   (SOLVED)
```

```python
astronomy_ai999x.solve('magnitude_difference_from_flux_ratio', ratio=0.01)
# -> dm = 5   (SOLVED)
```

```python
astronomy_ai999x.solve('magnitude_difference_from_flux_ratio', ratio=10.0)
# -> dm = -2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Magnitude difference: ratio = 100 | SOLVED | dm = -5 |
| Magnitude difference from flux ratio: ratio = 0.01 | SOLVED | dm = 5 |
| Magnitude difference: ratio = -2 | INVALID_INPUT | Magnitude difference from a flux ratio: the flux ratio must be a positive number. |
| Magnitude difference: | NEEDS_INPUT | magnitude_difference_from_flux_ratio needs ratio (flux ratio F1/F2). Example: solve('magnitude_difference_from_flux_rati |

<a id="combined_magnitude"></a>

### Combined magnitude of two stars: `combined_magnitude`

The magnitude of two unresolved stars seen together (their fluxes add).

$$
m = -2.5\log_{10}\left(10^{-0.4 m_1} + 10^{-0.4 m_2}\right)
$$

| input | meaning | condition |
|---|---|---|
| `m1` | magnitude of the first star |  |
| `m2` | magnitude of the second star |  |

| output | meaning |
|---|---|
| `m` | combined magnitude |
| `brightening` | how much brighter than the brighter star alone (mag) |

- **Method:** fluxes add: $m = -2.5$ log10(10^(-0.4 $m_{1}) + 10^(-0.4$ m2))
- **Assumes:** |$m_{1} - m_{2}$| <= 100
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![m (combined magnitude) against m1 (magnitude of the first star) from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/combined-magnitude-curve-2)

*m (combined magnitude) against m1 (magnitude of the first star) from 0.2 to 20 on a logarithmic axis, one curve for each value of m2 (magnitude of the second star): 3, 1.5, 6; 41 points per curve, each a solve of the model. Marked: the given point, m1 = 2, m = 1.636. Your inputs: m1 = 2.0, m2 = 3.0.*

![Combined magnitude as m2 runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/combined-magnitude-live-1)

*Combined magnitude as m2 runs from 0.75 to 5.25, every other input as you entered it (41 solutions). Marked: your m2 = 3 gives m = 1.636. Your inputs: m1 = 2.0, m2 = 3.0.*

![Combined magnitude as m1 runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/combined-magnitude-live-2)

*Combined magnitude as m1 runs from 0.5 to 3.5, every other input as you entered it (29 solutions). Marked: your m1 = 2 gives m = 1.636. Your inputs: m1 = 2.0, m2 = 3.0.*

```python
astronomy_ai999x.solve('combined_magnitude', m1=5.0, m2=5.0)
# -> m = 4.24743, brightening = 0.752575   (SOLVED)
```

```python
astronomy_ai999x.solve('combined_magnitude', m1=1.0, m2=6.0)
# -> m = 0.989197, brightening = 0.0108034   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Combined magnitude: m1 = 5, m2 = 5 | SOLVED | m = 4.24743, brightening = 0.752575 |
| Combined magnitude: m1 = 1, m2 = 6 | SOLVED | m = 0.989197, brightening = 0.0108034 |
| Combined magnitude: m1 = 1, m2 = 600 | INVALID_INPUT | Combined magnitude of two stars: the two magnitudes must differ by at most 100. |
| Combined magnitude: m1 = 1 | NEEDS_INPUT | combined_magnitude needs m2 (magnitude of the second star). Example: solve('combined_magnitude', m1=5, m2=5). |

<a id="bolometric_magnitude_from_luminosity"></a>

### Absolute bolometric magnitude from luminosity, `bolometric_magnitude_from_luminosity`

The absolute bolometric magnitude of a star of luminosity L on the IAU 2015 scale (zero point $L_{0} = 3.0128e28$ W).

$$
M_\mathrm{bol} = -2.5\log_{10}\frac{L}{L_0}
$$

| input | meaning | condition |
|---|---|---|
| `L` | luminosity (W or Lsun) |  |

| output | meaning |
|---|---|
| `M_bol` | absolute bolometric magnitude |
| `L_Lsun` | luminosity in solar luminosities |

- **Method:** M_bol = -2.5 log10(L / L0), $L_{0} = 3.0128e28$ W (IAU 2015 B2)
- **Assumes:** $L > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Absolute bolometric magnitude as L runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/bolometric-magnitude-from-luminosity-live-1)

*Absolute bolometric magnitude as L runs from 2.297e+26 to 5.359e+26 W, every other input as you entered it (41 solutions). Marked: your L = 3.828e+26 W gives M_bol = 4.74. Your inputs: L = 3.828e+26 W.*

```python
astronomy_ai999x.solve('bolometric_magnitude_from_luminosity', L=3.0128e+28)
# -> M_bol = 0, L_Lsun = 78.7043   (SOLVED)
```

```python
astronomy_ai999x.solve('bolometric_magnitude_from_luminosity', L=3.0127999999999997e+30)
# -> M_bol = -5, L_Lsun = 7870.43   (SOLVED)
```

```python
astronomy_ai999x.solve('bolometric_magnitude_from_luminosity', L=3.828e+26)
# -> M_bol = 4.74, L_Lsun = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Absolute bolometric magnitude: L = 1 Lsun | SOLVED | M_bol = 4.74, L_Lsun = 1 |
| Absolute bolometric magnitude: L = 3.0128e30 W | SOLVED | M_bol = -5, L_Lsun = 7870.43 |
| Absolute bolometric magnitude: L = 0 W | INVALID_INPUT | Absolute bolometric magnitude from luminosity: the luminosity must be a positive number. |
| Absolute bolometric magnitude: | NEEDS_INPUT | bolometric_magnitude_from_luminosity needs L (luminosity (W or Lsun)). Example: solve('bolometric_magnitude_from_luminos |

<a id="luminosity_from_bolometric_magnitude"></a>

### Luminosity from absolute bolometric magnitude, `luminosity_from_bolometric_magnitude`

The luminosity of a star from its absolute bolometric magnitude on the IAU 2015 scale.

$$
L = L_0\,10^{-0.4 M_\mathrm{bol}}
$$

| input | meaning | condition |
|---|---|---|
| `M_bol` | absolute bolometric magnitude |  |

| output | meaning |
|---|---|
| `L` | luminosity |
| `L_Lsun` | luminosity in solar luminosities |

- **Method:** L = L0 * 10^(-0.4 M_bol), $L_{0} = 3.0128e28$ W
- **Assumes:** -30 < M_bol < 40
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Luminosity in solar units for every whole absolute bolometric magnitude from -10 to 15 (26 solves of the model): each step of 5 magnitudes is a factor of exactly 100](https://www.ai999x.com/?ax_fig=astronomy-ai999x/luminosity-from-bolometric-magnitude-stems)

*Luminosity in solar units for every whole absolute bolometric magnitude from -10 to 15 (26 solves of the model): each step of 5 magnitudes is a factor of exactly 100. Marked: -10 → 7.87e+05, 0 → 78.7, 15 → 7.87e-05; the given M = 4.74 gives 1 solar luminosities. Your inputs: M_bol = 4.74.*

![Luminosity in solar luminosities as M_bol runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/luminosity-from-bolometric-magnitude-live-1)

*Luminosity in solar luminosities as M_bol runs from 1.185 to 8.295, every other input as you entered it (41 solutions). Marked: your M_bol = 4.74 gives L_Lsun = 1. Your inputs: M_bol = 4.74.*

```python
astronomy_ai999x.solve('luminosity_from_bolometric_magnitude', M_bol=0.0)
# -> L = 3.0128e+28 W, L_Lsun = 78.7043   (SOLVED)
```

```python
astronomy_ai999x.solve('luminosity_from_bolometric_magnitude', M_bol=-5.0)
# -> L = 3.0128e+30 W, L_Lsun = 7870.43   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Luminosity from bolometric magnitude: M_bol = -5 | SOLVED | L = 3.0128e+30 W, L_Lsun = 7870.43 |
| Luminosity from bolometric magnitude: M_bol = 0 mag | SOLVED | L = 3.0128e+28 W, L_Lsun = 78.7043 |
| Luminosity from bolometric magnitude: M_bol = 99 | INVALID_INPUT | Luminosity from absolute bolometric magnitude: the absolute bolometric magnitude must lie between -30 and 40. |
| Luminosity from bolometric magnitude: | NEEDS_INPUT | luminosity_from_bolometric_magnitude needs M_bol (absolute bolometric magnitude). Example: solve('luminosity_from_bolome |

<a id="apparent_bolometric_magnitude"></a>

### Apparent bolometric magnitude and flux: `apparent_bolometric_magnitude`

The bolometric flux at the observer and the apparent bolometric magnitude of a star of luminosity L at distance d.

$$
F = \frac{L}{4\pi d^2},\quad m_\mathrm{bol} = -2.5\log_{10}\frac{F}{F_0}
$$

| input | meaning | condition |
|---|---|---|
| `L` | luminosity (W or Lsun) |  |
| `d` | distance (pc, ly, m ...) |  |

| output | meaning |
|---|---|
| `m_bol` | apparent bolometric magnitude |
| `F` | bolometric flux at the observer |
| `M_bol` | absolute bolometric magnitude |

- **Method:** F = L/(4 pi d^2), m_bol = -2.5 log10(F/F0), F0 = L0/(4 pi (10 pc)^2)
- **Assumes:** $L > 0$; $d > 0$; no extinction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Apparent bolometric magnitude as d runs from 8](https://www.ai999x.com/?ax_fig=astronomy-ai999x/apparent-bolometric-magnitude-live-1)

*Apparent bolometric magnitude as d runs from 8.976e+10 to 2.094e+11 m, every other input as you entered it (41 solutions). Marked: your d = 1.496e+11 m gives m_bol = -26.83. Your inputs: L = 3.828e+26 W, d = 1.49598e+11 m.*

![Apparent bolometric magnitude as L runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/apparent-bolometric-magnitude-live-2)

*Apparent bolometric magnitude as L runs from 2.297e+26 to 5.359e+26 W, every other input as you entered it (29 solutions). Marked: your L = 3.828e+26 W gives m_bol = -26.83. Your inputs: L = 3.828e+26 W, d = 1.49598e+11 m.*

```python
astronomy_ai999x.solve('apparent_bolometric_magnitude', L=3.0128e+28, d=3.0856775814913674e+18)
# -> m_bol = 5, F = 2.51802e-10 W/m^2, M_bol = 0   (SOLVED)
```

```python
astronomy_ai999x.solve('apparent_bolometric_magnitude', L=3.0127999999999997e+30, d=3.085677581491367e+19)
# -> m_bol = 5, F = 2.51802e-10 W/m^2, M_bol = -5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Apparent bolometric magnitude: L = 3.0128e28 W, d = 100 pc | SOLVED | m_bol = 5, F = 2.51802e-10 W/m^2, M_bol = 0 |
| Apparent bolometric magnitude: L = 3.0128e28 W, d = 1 kpc | SOLVED | m_bol = 10, F = 2.51802e-12 W/m^2, M_bol = 0 |
| Apparent bolometric magnitude: L = 1 Lsun, d = 0 pc | INVALID_INPUT | Apparent bolometric magnitude and flux: the luminosity and the distance must be a positive number. |
| Apparent bolometric magnitude: L = 1 Lsun | NEEDS_INPUT | apparent_bolometric_magnitude needs d (distance (pc, ly, m ...)). Example: solve('apparent_bolometric_magnitude', L='3.0 |

<a id="angular_size"></a>

### Angular size: `angular_size`

The angle subtended by an object of diameter D at distance d (exact: 2 atan(D / 2d)).

$$
\theta = 2\arctan\frac{D}{2d} \approx \frac{D}{d}
$$

| input | meaning | condition |
|---|---|---|
| `D` | diameter of the object |  |
| `d` | distance to the object |  |

| output | meaning |
|---|---|
| `theta` | angular diameter |
| `theta_arcsec` | angular diameter in arcseconds |
| `theta_deg` | angular diameter in degrees |

- **Method:** $\theta = 2$ atan(D / (2 d)); for D << d this is D/d radians
- **Assumes:** $D > 0$; $d > 0$ (distance to the object's centre plane)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Angular diameters from size and distance (stated: the Sun 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/angular-size-bars)

*Angular diameters from size and distance (stated: the Sun 1.3927e9 m at 1 au, the Moon 3.4748e6 m at 3.844e8 m, the Andromeda galaxy 46.56 kpc at 765 kpc, Venus 1.2104e7 m at 0.277 au, Jupiter 1.3982e8 m at 4.2 au, Betelgeuse 764 solar radii in radius at 168 pc), in arcseconds. Each bar is a solve of the model; the last is the given inputs. Marked: Sun 1920″, Moon 1865″, Andromeda 1.255e+04″, Venus 60.25″, Jupiter 45.9″, Betelg. 0.0423″, given 1865″. Your inputs: D = 3474.8 km, d = 384400 km.*

![Angular diameter in arcseconds as D runs from 2085 to 4865 km, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/angular-size-live-1)

*Angular diameter in arcseconds as D runs from 2085 to 4865 km, every other input as you entered it (41 solutions). Marked: your D = 3475 km gives θ_arcsec = 1865. Your inputs: D = 3474.8 km, d = 384400 km.*

![Angular diameter in arcseconds as d runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/angular-size-live-2)

*Angular diameter in arcseconds as d runs from 2.306e+05 to 5.382e+05 km, every other input as you entered it (29 solutions). Marked: your d = 3.844e+05 km gives θ_arcsec = 1865. Your inputs: D = 3474.8 km, d = 384400 km.*

```python
astronomy_ai999x.solve('angular_size', D=2.0, d=1.0)
# -> theta = 1.5708 rad, theta_arcsec = 324000, theta_deg = 90   (SOLVED)
```

```python
astronomy_ai999x.solve('angular_size', D=3.4641016151377544, d=1.0)
# -> theta = 2.0944 rad, theta_arcsec = 432000, theta_deg = 120   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Angular size: D = 2 m, d = 1 m | SOLVED | theta = 1.5708 rad, theta_arcsec = 324000, theta_deg = 90 |
| Angular size: D = 3474.8 km, d = 384400 km | SOLVED | theta = 0.00903948 rad, theta_arcsec = 1864.53, theta_deg = 0.517924 |
| Angular size: D = 2 m, d = -1 m | INVALID_INPUT | Angular size: the size and the distance must be a positive number. |
| Angular size: D = 2 m | NEEDS_INPUT | angular_size needs d (distance to the object). Example: solve('angular_size', D='2 m', d='1 m'). |

<a id="linear_size_from_angle"></a>

### Linear size from angular size: `linear_size_from_angle`

The true diameter of an object of angular size theta at distance d (exact: 2 d tan(theta/2)).

$$
D = 2d\tan\frac{\theta}{2} \approx \theta d
$$

| input | meaning | condition |
|---|---|---|
| `theta` | angular diameter (arcsec, arcmin, deg, rad) |  |
| `d` | distance to the object |  |

| output | meaning |
|---|---|
| `D` | diameter |
| `D_km` | diameter in kilometres |
| `D_au` | diameter in astronomical units |

- **Method:** $D = 2$ d tan(theta/2); for small angles $D = \theta$ d
- **Assumes:** $0 < \theta < 180$ degrees; $d > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![What the given angle spans at five distances (stated: the Moon 3](https://www.ai999x.com/?ax_fig=astronomy-ai999x/linear-size-from-angle-bars)

*What the given angle spans at five distances (stated: the Moon 3.844e8 m, the Sun 1 au, Proxima Centauri 1.302 pc, the galactic centre 8.18 kpc, Andromeda 765 kpc; Euclidean) and at the given distance: the linear size the model returns. Each bar is a solve of the model; the last is the given inputs. Marked: Moon 3533 km, Sun 1.375e+06 km, Proxima 3.693e+11 km, Gal. centre 2.32e+15 km, Andromeda 2.17e+17 km, given 3533 km. In SI units: D = 3.533e+06 m. Your inputs: theta = 0.00919207 rad, d = 384400 km.*

![Diameter as θ runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/linear-size-from-angle-live-1)

*Diameter as θ runs from 0.005515 to 0.01287 rad, every other input as you entered it (41 solutions). Marked: your θ = 0.009192 rad gives D = 3533 km. Your inputs: theta = 0.00919207 rad, d = 384400 km.*

![Diameter as d runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/linear-size-from-angle-live-2)

*Diameter as d runs from 2.306e+05 to 5.382e+05 km, every other input as you entered it (29 solutions). Marked: your d = 3.844e+05 km gives D = 3533 km. Your inputs: theta = 0.00919207 rad, d = 384400 km.*

```python
astronomy_ai999x.solve('linear_size_from_angle', theta=1.5707963267948966, d=1.0)
# -> D = 2 m, D_km = 0.002, D_au = 1.33692e-11   (SOLVED)
```

```python
astronomy_ai999x.solve('linear_size_from_angle', theta=2.0943951023931953, d=1.0)
# -> D = 3.4641 m, D_km = 0.0034641, D_au = 2.31561e-11   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Linear size: theta = 90 deg, d = 1 m | SOLVED | D = 2 m, D_km = 0.002, D_au = 1.33692e-11 |
| Linear size: theta = 1 arcsec, d = 1 pc | SOLVED | D = 1.49598e+11 m, D_km = 1.49598e+08, D_au = 1 |
| Linear size: theta = 200 deg, d = 1 m | INVALID_INPUT | Linear size from angular size: the angular size must be smaller than 180 degrees. |
| Linear size: theta = 1 arcsec | NEEDS_INPUT | linear_size_from_angle needs d (distance to the object). Example: solve('linear_size_from_angle', theta='1.5707963267948 |

<a id="stellar_luminosity"></a>

### Luminosity of a star (Stefan-Boltzmann): `stellar_luminosity`

The luminosity of a star of radius R radiating as a blackbody at effective temperature T: $L = 4$ pi $R^{2}$ sigma $T^{4.}$

$$
L = 4\pi R^2 \sigma T_\mathrm{eff}^4
$$

| input | meaning | condition |
|---|---|---|
| `R` | stellar radius (m, km, Rsun) |  |
| `T` | effective temperature |  |

| output | meaning |
|---|---|
| `L` | luminosity |
| `L_Lsun` | luminosity in solar luminosities |

- **Method:** $L = 4$ pi $R^{2}$ sigma $T^{4}$ with $\sigma = 2$ $pi^{5}$ k^4 / (15 $h^{3}$ c^2)
- **Assumes:** $R > 0$; $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Lines of equal radius on the Hertzsprung-Russell plane: the luminosity the model returns for radii 0](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/stellar-luminosity-hr.svg)

*Lines of equal radius on the Hertzsprung-Russell plane: the luminosity the model returns for radii 0.001 to 1000 solar radii at 25 temperatures from 2300 to 50000 K each (temperature decreasing to the right). Marked: one solar radius at 5772 K gives 1 solar luminosities; the given R = 695.7 Mm, T = 5772 K gives 1 (marked when it falls in the drawn range). In SI units: L = 3.828e+26 W. Your inputs: R = 6.957e+08 m, T = 5772 K.*

![Luminosity as T runs from 3463 to 8081 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/stellar-luminosity-live-1)

*Luminosity as T runs from 3463 to 8081 K, every other input as you entered it (41 solutions). Marked: your T = 5772 K gives L = 3.828e+26. Your inputs: R = 6.957e+08 m, T = 5772 K.*

![Luminosity as R runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/stellar-luminosity-live-2)

*Luminosity as R runs from 4.174e+08 to 9.74e+08 m, every other input as you entered it (29 solutions). Marked: your R = 6.957e+08 m gives L = 3.828e+26. Your inputs: R = 6.957e+08 m, T = 5772 K.*

```python
astronomy_ai999x.solve('stellar_luminosity', R=1.0, T=100.0)
# -> L = 71.256 W, L_Lsun = 1.86144e-25   (SOLVED)
```

```python
astronomy_ai999x.solve('stellar_luminosity', R=695700000.0, T=5772.0)
# -> L = 3.82799e+26 W, L_Lsun = 0.999998   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Stellar luminosity: R = 1 Rsun, T = 5772 K | SOLVED | L = 3.82799e+26 W, L_Lsun = 0.999998 |
| Stellar luminosity: R = 1 m, T = 100 K | SOLVED | L = 71.256 W, L_Lsun = 1.86144e-25 |
| Stellar luminosity: R = 1 Rsun, T = -5 K | INVALID_INPUT | Luminosity of a star (Stefan-Boltzmann): the radius and the temperature must be a positive number. |
| Stellar luminosity: R = 1 Rsun | NEEDS_INPUT | stellar_luminosity needs T (effective temperature). Example: solve('stellar_luminosity', R='1 m', T='100 K'). |

<a id="stellar_radius"></a>

### Radius of a star from luminosity and temperature, `stellar_radius`

The radius of a star from its luminosity and effective temperature: $R = sqrt$(L / (4 pi sigma T^4)).

$$
R = \sqrt{\frac{L}{4\pi\sigma T_\mathrm{eff}^4}}
$$

| input | meaning | condition |
|---|---|---|
| `L` | luminosity (W or Lsun) |  |
| `T` | effective temperature |  |

| output | meaning |
|---|---|
| `R` | stellar radius |
| `R_Rsun` | radius in solar radii |

- **Method:** $R = sqrt$(L / (4 pi sigma)) / $T^{2}$
- **Assumes:** $L > 0$; $T > 0$; blackbody emission
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Radii returned from luminosity and effective temperature (stated: the Sun 1 solar luminosity, 5772 K; Sirius B 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/stellar-radius-bars)

*Radii returned from luminosity and effective temperature (stated: the Sun 1 solar luminosity, 5772 K; Sirius B 0.056, 25200 K; Proxima 0.00155, 3042 K; Vega 40.1, 9602 K; Arcturus 170, 4286 K; Rigel 120000, 12100 K; Betelgeuse 126000, 3600 K; one solar luminosity = 3.828e26 W), in solar radii. Each bar is a solve of the model; the last is the given inputs. Marked: Sun 1 R☉, Sirius B 0.01241 R☉, Proxima 0.1417 R☉, Vega 2.288 R☉, Arcturus 23.65 R☉, Rigel 78.83 R☉, Betelg. 912.5 R☉, given 1 R☉. In SI units: R = 6.957e+08 m. Your inputs: L = 3.828e+26 W, T = 5772 K.*

![Stellar radius as T runs from 3463 to 8081 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/stellar-radius-live-1)

*Stellar radius as T runs from 3463 to 8081 K, every other input as you entered it (41 solutions). Marked: your T = 5772 K gives R = 6.957e+08. Your inputs: L = 3.828e+26 W, T = 5772 K.*

![Stellar radius as L runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/stellar-radius-live-2)

*Stellar radius as L runs from 2.297e+26 to 5.359e+26 W, every other input as you entered it (29 solutions). Marked: your L = 3.828e+26 W gives R = 6.957e+08. Your inputs: L = 3.828e+26 W, T = 5772 K.*

```python
astronomy_ai999x.solve('stellar_radius', L=71.25602647365321, T=100.0)
# -> R = 1 m, R_Rsun = 1.4374e-09   (SOLVED)
```

```python
astronomy_ai999x.solve('stellar_radius', L=28502410589.461285, T=100.0)
# -> R = 20000 m, R_Rsun = 2.8748e-05   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Stellar radius: L = 1 Lsun, T = 5772 K | SOLVED | R = 6.95701e+08 m, R_Rsun = 1 |
| Stellar radius: L = 1 Lsun, T = 0 K | INVALID_INPUT | Radius of a star from luminosity and temperature: the luminosity and the temperature must be a positive number. |
| Stellar radius: T = 5772 K | NEEDS_INPUT | stellar_radius needs L (luminosity (W or Lsun)). Example: solve('stellar_radius', L='71.25602647365321 W', T='100 K'). |

<a id="effective_temperature"></a>

### Effective temperature of a star: `effective_temperature`

The effective temperature of a star from its luminosity and radius: T = (L / (4 pi $R^{2}$ sigma))^$(1/4)$.

$$
T_\mathrm{eff} = \left(\frac{L}{4\pi R^2\sigma}\right)^{1/4}
$$

| input | meaning | condition |
|---|---|---|
| `L` | luminosity (W or Lsun) |  |
| `R` | stellar radius (m, km, Rsun) |  |

| output | meaning |
|---|---|
| `T` | effective temperature |

- **Method:** T = (L / (4 pi $R^{2}$ sigma))^$(1/4)$
- **Assumes:** $L > 0$; $R > 0$; blackbody emission
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A Hertzsprung-Russell diagram (temperature decreasing to the right): nine stars placed at the effective temperature the model returns from their luminosity and radius (stated, in solar units of 3](https://www.ai999x.com/?ax_fig=astronomy-ai999x/effective-temperature-hr)

*A Hertzsprung-Russell diagram (temperature decreasing to the right): nine stars placed at the effective temperature the model returns from their luminosity and radius (stated, in solar units of 3.828e26 W and 6.957e8 m), and the given star (3.828e+26 W, 695.7 Mm): Sun 5772 K, Sirius A 9906 K, Sirius B 3.064e+04 K, Vega 9455 K, Proxima 2918 K, Betelgeuse 3934 K, Rigel 1.209e+04 K, Aldebaran 3934 K, Arcturus 4135 K, given 5772 K. Your inputs: L = 3.828e+26 W, R = 6.957e+08 m.*

![Effective temperature as R runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/effective-temperature-live-1)

*Effective temperature as R runs from 4.174e+08 to 9.74e+08 m, every other input as you entered it (41 solutions). Marked: your R = 6.957e+08 m gives T = 5772. Your inputs: L = 3.828e+26 W, R = 6.957e+08 m.*

![Effective temperature as L runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/effective-temperature-live-2)

*Effective temperature as L runs from 2.297e+26 to 5.359e+26 W, every other input as you entered it (29 solutions). Marked: your L = 3.828e+26 W gives T = 5772. Your inputs: L = 3.828e+26 W, R = 6.957e+08 m.*

```python
astronomy_ai999x.solve('effective_temperature', L=71.25602647365321, R=1.0)
# -> T = 100 K   (SOLVED)
```

```python
astronomy_ai999x.solve('effective_temperature', L=4453501654603.325, R=4.0)
# -> T = 25000 K   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Effective temperature: L = 1 Lsun, R = 1 Rsun | SOLVED | T = 5772 K |
| Effective temperature: L = 1 Lsun, R = 0 m | INVALID_INPUT | Effective temperature of a star: the luminosity and the radius must be a positive number. |
| Effective temperature: L = 1 Lsun | NEEDS_INPUT | effective_temperature needs R (stellar radius (m, km, Rsun)). Example: solve('effective_temperature', L='71.256026473653 |

<a id="planet_equilibrium_temperature"></a>

### Equilibrium temperature of a planet: `planet_equilibrium_temperature`

The equilibrium temperature of a rapidly rotating planet of Bond albedo A at distance a from a star of temperature T and radius R, and the stellar flux it receives.

$$
T_\mathrm{eq} = T_\star\sqrt{\frac{R_\star}{2a}}\,(1 - A)^{1/4}
$$

| input | meaning | condition |
|---|---|---|
| `T_star` | effective temperature of the star |  |
| `R_star` | radius of the star (m, Rsun) |  |
| `a` | orbital distance (m, au) |  |
| `A` | Bond albedo (0 to 1, default 0) |  |

| output | meaning |
|---|---|
| `T_eq` | equilibrium temperature |
| `S` | stellar flux at the planet |

- **Method:** $(1 - A)$ S pi $Rp^{2} = 4$ pi $Rp^{2}$ sigma $T^{4}$ with $S = \sigma$ T*^4 (R*/a)^2: $T = T sqrt$(R*/(2a)) $(1 - A)^{1/4}$
- **Assumes:** $0  \leq  A < 1$; a > R_star; full heat redistribution, unit emissivity
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Equilibrium temperatures of the eight planets for the Sun (5772 K, radius 6](https://www.ai999x.com/?ax_fig=astronomy-ai999x/planet-equilibrium-temperature-bars)

*Equilibrium temperatures of the eight planets for the Sun (5772 K, radius 6.957e8 m) from their mean distances and Bond albedos (stated), each computed by the model (no-atmosphere values, without any greenhouse warming). Each bar is a solve of the model; the last is the given inputs. Marked: Merc. 437.2 K, Venus 229.1 K, Earth 254 K, Mars 209.8 K, Jup. 102.5 K, Sat. 81.17 K, Uran. 58.12 K, Nep. 46.59 K, given 318.3 K. Your inputs: T_star = 3300 K, R_star = 1.3914e+08 m, a = 7.47989e+09 m.*

![Equilibrium temperature as T_star runs from 1980 to 4620 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/planet-equilibrium-temperature-live-1)

*Equilibrium temperature as T_star runs from 1980 to 4620 K, every other input as you entered it (41 solutions). Marked: your T_star = 3300 K gives T_eq = 318.3. Your inputs: T_star = 3300 K, R_star = 1.3914e+08 m, a = 7.47989e+09 m.*

![Equilibrium temperature as a runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/planet-equilibrium-temperature-live-2)

*Equilibrium temperature as a runs from 4.488e+09 to 1.047e+10 m, every other input as you entered it (29 solutions). Marked: your a = 7.48e+09 m gives T_eq = 318.3. Your inputs: T_star = 3300 K, R_star = 1.3914e+08 m, a = 7.47989e+09 m.*

```python
astronomy_ai999x.solve('planet_equilibrium_temperature', T_star=1000.0, R_star=2.0, a=4.0)
# -> T_eq = 500 K, S = 14175.9 W/m^2   (SOLVED)
```

```python
astronomy_ai999x.solve('planet_equilibrium_temperature', T_star=1000.0, R_star=2.0, a=4.0, A=0.9375)
# -> T_eq = 250 K, S = 14175.9 W/m^2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Equilibrium temperature: T_star = 1000 K, R_star = 2 m, a = 4 m | SOLVED | T_eq = 500 K, S = 14175.9 W/m^2 |
| Equilibrium temperature: T_star = 5772 K, R_star = 1 Rsun, a = 1 au, A = 0.3 | SOLVED | T_eq = 254.586 K, S = 1361.16 W/m^2 |
| Equilibrium temperature: T_star = 5772 K, R_star = 1 Rsun, a = 1 au, A = 1.5 | INVALID_INPUT | Equilibrium temperature of a planet: the Bond albedo A must satisfy 0 <= A < 1. |
| Equilibrium temperature: T_star = 5772 K, a = 1 au | NEEDS_INPUT | planet_equilibrium_temperature needs R_star (radius of the star (m, Rsun)). Example: solve('planet_equilibrium_temperatu |

<a id="saha_hydrogen_ionization"></a>

### Ionization fraction of hydrogen (Saha equation), `saha_hydrogen_ionization`

The ionized fraction of pure hydrogen in thermal equilibrium at temperature T and total hydrogen density n (Saha equation, ground-state partition functions).

$$
\frac{x^2}{1-x} = \frac{1}{n}\left(\frac{2\pi m_e k T}{h^2}\right)^{3/2} e^{-\chi/kT}
$$

| input | meaning | condition |
|---|---|---|
| `T` | temperature |  |
| `n` | total density of hydrogen nuclei (m^-3 or cm^-3) |  |

| output | meaning |
|---|---|
| `x` | ionized fraction n_II / n |
| `neutral` | neutral fraction n_I / n |
| `n_e` | electron density |

- **Method:** $x^{2}/(1 - x) = (1/n)$ (2 pi m_e k T / h^2)^(3/2) exp(-chi / kT), solved in a cancellation-free form
- **Assumes:** 1000 $K  \leq  T  \leq  1e6$ K; pure hydrogen, $n_e = n_II$; $g_I = 2$, $g_II = 1$, $\chi = 13.598$ eV
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![x (ionized fraction n_II / n) against T (temperature) from 5000 K to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/saha-hydrogen-ionization-curve-2)

*x (ionized fraction n_II / n) against T (temperature) from 5000 K to 1.5e+04 K, one curve for each value of n (total density of hydrogen nuclei (m^-3 or cm^-3)): 1e+20 1/m³, 5e+19 1/m³, 2e+20 1/m³; 31 points per curve, each a solve of the model. Marked: the given point, T = 1e+04 K, x = 0.8074. Your inputs: T = 10000 K, n = 1e20 m^-3.*

![X as T runs from 6000 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/saha-hydrogen-ionization-live-1)

*X as T runs from 6000 to 1.4e+04 K, every other input as you entered it (41 solutions). Marked: your T = 1e+04 K gives x = 0.8074. Your inputs: T = 10000 K, n = 1e20 m^-3.*

![X as n runs from 6e+19 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/saha-hydrogen-ionization-live-2)

*X as n runs from 6e+19 to 1.4e+20 m^-3, every other input as you entered it (29 solutions). Marked: your n = 1e+20 m^-3 gives x = 0.8074. Your inputs: T = 10000 K, n = 1e20 m^-3.*

```python
astronomy_ai999x.solve('saha_hydrogen_ionization', T=10000.0, n=6.769876143152214e+20)
# -> x = 0.5, neutral = 0.5, n_e = 3.38494e+20 1/m^3   (SOLVED)
```

```python
astronomy_ai999x.solve('saha_hydrogen_ionization', T=8000.0, n=2.343302528357021e+18)
# -> x = 0.732051, neutral = 0.267949, n_e = 1.71542e+18 1/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Saha equation: T = 10000 K, n = 1e20 m^-3 | SOLVED | x = 0.807409, neutral = 0.192591, n_e = 8.07409e+19 1/m^3 |
| Saha equation: T = 300 K, n = 1e20 m^-3 | INVALID_INPUT | Ionization fraction of hydrogen (Saha equation): the temperature must lie between 1000 K and 1e6 K. |
| Saha equation: T = 10000 K | NEEDS_INPUT | saha_hydrogen_ionization needs n (total density of hydrogen nuclei (m^-3 or cm^-3)). Example: solve('saha_hydrogen_ioniz |

<a id="schwarzschild_radius"></a>

### Schwarzschild radius: `schwarzschild_radius`

The Schwarzschild radius (event horizon of a non-rotating black hole) of a mass M: $r_s = 2$ G $M / c^{2.}$

$$
r_s = \frac{2GM}{c^2}
$$

| input | meaning | condition |
|---|---|---|
| `M` | mass (kg, Msun, Mearth) |  |

| output | meaning |
|---|---|
| `rs` | Schwarzschild radius |
| `rs_km` | Schwarzschild radius in kilometres |
| `M_Msun` | mass in solar masses |

- **Method:** $r_s = 2$ G $M / c^{2}$
- **Assumes:** $M > 0$; non-rotating, uncharged mass
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Schwarzschild radius as M runs from 1](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/schwarzschild-radius-live-1.svg)

*Schwarzschild radius as M runs from 1.193e+30 to 2.784e+30 kg, every other input as you entered it (41 solutions). Marked: your M = 1.988e+30 kg gives rs = 2953. Your inputs: M = 1.98841e+30 kg.*

```python
astronomy_ai999x.solve('schwarzschild_radius', M=1.988409870698051e+30)
# -> rs = 2953.25 m, rs_km = 2.95325, M_Msun = 1   (SOLVED)
```

```python
astronomy_ai999x.solve('schwarzschild_radius', M=6.732954607500545e+26)
# -> rs = 1 m, rs_km = 0.001, M_Msun = 0.00033861   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Schwarzschild radius: M = 1 Msun | SOLVED | rs = 2953.25 m, rs_km = 2.95325, M_Msun = 1 |
| Schwarzschild radius: M = 5.972e24 kg | SOLVED | rs = 0.00886981 m, rs_km = 8.86981e-06, M_Msun = 3.0034e-06 |
| Schwarzschild radius: M = 0 kg | INVALID_INPUT | Schwarzschild radius: the mass must be a positive number. |
| Schwarzschild radius: | NEEDS_INPUT | schwarzschild_radius needs M (mass (kg, Msun, Mearth)). Example: solve('schwarzschild_radius', M='1.988409870698051e+30  |

<a id="atmospheric_scale_height"></a>

### Scale height of an isothermal atmosphere: `atmospheric_scale_height`

The pressure scale height $H = R$ T / (mu g) of an isothermal planetary atmosphere: pressure falls by a factor e every H.

$$
H = \frac{RT}{\mu g}
$$

| input | meaning | condition |
|---|---|---|
| `T` | temperature of the atmosphere |  |
| `mu` | mean molar mass (kg/mol or g/mol) |  |
| `g` | surface gravity |  |

| output | meaning |
|---|---|
| `H` | scale height |
| `H_km` | scale height in kilometres |

- **Method:** $H = R$ T / (mu g)
- **Assumes:** T, mu, $g > 0$; isothermal ideal gas, constant gravity
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Atmospheric scale heights, the height over which the pressure falls by a factor e, from temperature, mean molar mass and surface gravity (stated: Venus 737 K, 43](https://www.ai999x.com/?ax_fig=astronomy-ai999x/atmospheric-scale-height-bars)

*Atmospheric scale heights, the height over which the pressure falls by a factor e, from temperature, mean molar mass and surface gravity (stated: Venus 737 K, 43.45 g/mol, 8.87 m/s²; Earth 288 K, 28.97 g/mol, 9.81 m/s²; Mars 210 K, 43.34 g/mol, 3.71 m/s²; Jupiter 165 K, 2.22 g/mol, 24.79 m/s²; Titan 94 K, 28 g/mol, 1.352 m/s²). Each bar is a solve of the model; the last is the given inputs. Marked: Venus 15.9 km, Earth 8.426 km, Mars 10.86 km, Jupiter 24.93 km, Titan 20.65 km, given 8.426 km. In SI units: H = 8426 m. Your inputs: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2.*

![Scale height as T runs from 248 to 328 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/atmospheric-scale-height-live-1)

*Scale height as T runs from 248 to 328 K, every other input as you entered it (41 solutions). Marked: your T = 288 K gives H = 8426. Your inputs: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2.*

![Scale height as μ runs from 17](https://www.ai999x.com/?ax_fig=astronomy-ai999x/atmospheric-scale-height-live-2)

*Scale height as μ runs from 17.38 to 40.56 g/mol, every other input as you entered it (29 solutions). Marked: your μ = 28.97 g/mol gives H = 8426. Your inputs: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2.*

```python
astronomy_ai999x.solve('atmospheric_scale_height', T=300.0, mu=0.25452436586183386, g=9.8)
# -> H = 1000 m, H_km = 1   (SOLVED)
```

```python
astronomy_ai999x.solve('atmospheric_scale_height', T=288.0, mu=0.02897, g=9.81)
# -> H = 8425.76 m, H_km = 8.42576   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Scale height: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2 | SOLVED | H = 8425.76 m, H_km = 8.42576 |
| Scale height: T = 288 K, mu = 0 g/mol, g = 9.81 m/s^2 | INVALID_INPUT | Scale height of an isothermal atmosphere: the temperature, the molar mass and the gravity must be a positive number. |
| Scale height: T = 288 K, g = 9.81 m/s^2 | NEEDS_INPUT | atmospheric_scale_height needs mu (mean molar mass (kg/mol or g/mol)). Example: solve('atmospheric_scale_height', T='300 |

<a id="free_fall_time"></a>

### Free-fall time of a uniform cloud: `free_fall_time`

The time a uniform, pressureless sphere of density rho takes to collapse from rest under its own gravity.

$$
t_\mathrm{ff} = \sqrt{\frac{3\pi}{32\,G\rho}}
$$

| input | meaning | condition |
|---|---|---|
| `rho` | mean density (kg/m^3; g/cm^3) |  |

| output | meaning |
|---|---|
| `t_ff` | free-fall time |
| `t_ff_yr` | free-fall time in years |

- **Method:** $t_ff = sqrt$(3 pi / (32 G rho))
- **Assumes:** $\rho > 0$; uniform, pressureless, non-rotating sphere starting at rest
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Free-fall time as ρ runs from 6e-18 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/free-fall-time-live-1)

*Free-fall time as ρ runs from 6e-18 to 1.4e-17 kg/m^3, every other input as you entered it (41 solutions). Marked: your ρ = 1e-17 kg/m^3 gives t_ff = 6.657e+05 yr. Your inputs: rho = 1e-17 kg/m^3.*

```python
astronomy_ai999x.solve('free_fall_time', rho=4412811999.3713665)
# -> t_ff = 1 s, t_ff_yr = 3.16881e-08   (SOLVED)
```

```python
astronomy_ai999x.solve('free_fall_time', rho=1103202999.8428416)
# -> t_ff = 2 s, t_ff_yr = 6.33762e-08   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Free-fall time: rho = 1e-17 kg/m^3 | SOLVED | t_ff = 2.10067e+13 s, t_ff_yr = 665662 |
| Free-fall time: rho = -1 kg/m^3 | INVALID_INPUT | Free-fall time of a uniform cloud: the density must be a positive number. |
| Free-fall time: | NEEDS_INPUT | free_fall_time needs rho (mean density (kg/m^3; g/cm^3)). Example: solve('free_fall_time', rho='4412811999.3713665 kg/m^ |

<a id="binary_star_masses"></a>

### Masses of a visual binary star: `binary_star_masses`

The total and individual masses of a binary star from its orbital period and each star's semi-major axis about the centre of mass (Kepler's third law and the centre-of-mass condition).

$$
M_1 + M_2 = \frac{4\pi^2 (a_1 + a_2)^3}{G P^2},\quad M_1 a_1 = M_2 a_2
$$

| input | meaning | condition |
|---|---|---|
| `P` | orbital period (s, day, yr) |  |
| `a1` | semi-major axis of star 1 about the centre of mass (au, m) |  |
| `a2` | semi-major axis of star 2 about the centre of mass (au, m) |  |

| output | meaning |
|---|---|
| `M_total` | total mass |
| `M_total_Msun` | total mass in solar masses |
| `M1_Msun` | mass of star 1 in solar masses |
| `M2_Msun` | mass of star 2 in solar masses |
| `a_au` | relative semi-major axis $a_{1} + a_{2}$ in au |

- **Method:** $M_{1} + M_{2} = 4$ $pi^{2}$ (a1 + a2)^3 / (G P^2); M1 $a_{1} = M_{2}$ a2
- **Assumes:** P, a1, $a_{2} > 0$; true (de-projected) semi-major axes
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A binary star drawn to scale about its centre of mass (the cross), taking circular orbits: period 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/binary-star-masses-orbit)

*A binary star drawn to scale about its centre of mass (the cross), taking circular orbits: period 1.581 Gs, orbit radii 957.4 Gm (star 1) and 2.005 Tm (star 2). The model returns a total mass of 3.093 solar masses, split inversely to the orbit radii: M1 = 2.093, M2 = 0.9997 solar masses; the heavier star moves on the smaller circle. Your inputs: P = 50.1 yr, a1 = 9.57426e+11 m, a2 = 2.00461e+12 m.*

![Total mass in solar masses as a2 runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/binary-star-masses-live-1)

*Total mass in solar masses as a2 runs from 1.203e+12 to 2.806e+12 m, every other input as you entered it (41 solutions). Marked: your a2 = 2.005e+12 m gives M_total_Msun = 3.093. Your inputs: P = 50.1 yr, a1 = 9.57426e+11 m, a2 = 2.00461e+12 m.*

![Total mass in solar masses as P runs from 30](https://www.ai999x.com/?ax_fig=astronomy-ai999x/binary-star-masses-live-2)

*Total mass in solar masses as P runs from 30.06 to 70.14 yr, every other input as you entered it (29 solutions). Marked: your P = 50.1 yr gives M_total_Msun = 3.093. Your inputs: P = 50.1 yr, a1 = 9.57426e+11 m, a2 = 2.00461e+12 m.*

```python
astronomy_ai999x.solve('binary_star_masses', P=6.283185307179586, a1=0.25, a2=0.75)
# -> M_total = 1.49828e+10 kg, M_total_Msun = 7.53509e-21, M1_Msun = 5.65132e-21, M2_Msun = 1.88377e-21, a_au = 6.68459e-12   (SOLVED)
```

```python
astronomy_ai999x.solve('binary_star_masses', P=31557600.0, a1=74798935350.0, a2=74798935350.0)
# -> M_total = 1.98848e+30 kg, M_total_Msun = 1.00004, M1_Msun = 0.500019, M2_Msun = 0.500019, a_au = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Binary star masses: P = 1 yr, a1 = 0.25 au, a2 = 0.75 au | SOLVED | M_total = 1.98848e+30 kg, M_total_Msun = 1.00004, M1_Msun = 0.750028, M2_Msun = 0.250009, a_au = 1 |
| Binary star masses: P = 1 yr, a1 = 0 au, a2 = 0.75 au | INVALID_INPUT | Masses of a visual binary star: the period and the two semi-major axes must be a positive number. |
| Binary star masses: P = 1 yr, a1 = 0.25 au | NEEDS_INPUT | binary_star_masses needs a2 (semi-major axis of star 2 about the centre of mass (au, m)). Example: solve('binary_star_ma |

<a id="hohmann_transfer"></a>

### Hohmann transfer between circular orbits: `hohmann_transfer`

The two velocity changes, their sum and the transfer time of a Hohmann transfer between coplanar circular orbits of radii r1 and r2 around a body of mass M.

$$
\Delta v_1 = \sqrt{\frac{\mu}{r_1}}\left(\sqrt{\frac{2r_2}{r_1 + r_2}} - 1\right),\ \Delta v_2 = \sqrt{\frac{\mu}{r_2}}\left(1 - \sqrt{\frac{2r_1}{r_1 + r_2}}\right),\ t = \pi\sqrt{\frac{a^3}{\mu}}
$$

| input | meaning | condition |
|---|---|---|
| `r1` | radius of the initial circular orbit (km, au) |  |
| `r2` | radius of the final circular orbit |  |
| `M` | mass of the central body (kg, Mearth, Msun) |  |

| output | meaning |
|---|---|
| `dv1` | first burn (departure) |
| `dv2` | second burn (arrival) |
| `dv` | total delta-v |
| `t_transfer` | transfer time (half the transfer ellipse) |
| `t_days` | transfer time in days |
| `a_transfer` | semi-major axis of the transfer ellipse |

- **Method:** $vp = sqrt$(2 mu r2 / (r1 $(r_{1} + r_{2})$)), $va = sqrt$(2 mu r1 / (r2 $(r_{1} + r_{2})$)); $t = pi$ sqrt(a^3/mu)
- **Assumes:** r1, r2, $M > 0$; $r_{1}  \neq  r_{2}$; impulsive burns, coplanar circular orbits
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The Hohmann transfer from a circular orbit of r1 = 6](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/hohmann-transfer-orbit.svg)

*The Hohmann transfer from a circular orbit of r1 = 6.678 Mm to one of r2 = 42.16 Mm, computed by the model: transfer ellipse a = 24.42 Mm (returned), e = 0.7265 (derived from the two radii), drawn as its half orbit; burns Δv1 = 2426 m/s and Δv2 = 1467 m/s, total 3893 m/s, transfer time 18.99 ks. Your inputs: r1 = 6678 km, r2 = 42164 km, M = 5.97217e+24 kg.*

![Total delta-v, first burn and second burn as r2 runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/hohmann-transfer-live-1)

*Total delta-v, first burn and second burn as r2 runs from 2.53e+04 to 5.903e+04 km, every other input as you entered it (41 solutions). Marked: your r2 = 4.216e+04 km gives dv = 3893. Your inputs: r1 = 6678 km, r2 = 42164 km, M = 5.97217e+24 kg.*

![Total delta-v, first burn and second burn as r1 runs from 4007 to 9349 km, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/hohmann-transfer-live-2)

*Total delta-v, first burn and second burn as r1 runs from 4007 to 9349 km, every other input as you entered it (29 solutions). Marked: your r1 = 6678 km gives dv = 3893. Your inputs: r1 = 6678 km, r2 = 42164 km, M = 5.97217e+24 kg.*

```python
astronomy_ai999x.solve('hohmann_transfer', r1=1.0, r2=3.0, M=14982844642.8839)
# -> dv1 = 0.224745 m/s, dv2 = 0.169102 m/s, dv = 0.393847 m/s, t_transfer = 8.88577 s, t_days = 0.000102845, a_transfer = 2 m   (SOLVED)
```

```python
astronomy_ai999x.solve('hohmann_transfer', r1=3.0, r2=1.0, M=14982844642.8839)
# -> dv1 = 0.169102 m/s, dv2 = 0.224745 m/s, dv = 0.393847 m/s, t_transfer = 8.88577 s, t_days = 0.000102845, a_transfer = 2 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hohmann transfer: r1 = 6678 km, r2 = 42164 km, M = 5.972e24 kg | SOLVED | dv1 = 2425.73 m/s, dv2 = 1466.82 m/s, dv = 3892.55 m/s, t_transfer = 18990.3 s, t_days = 0.219795, a_transfer = 2.4421e+07 m |
| Hohmann transfer: r1 = 7000 km, r2 = 7000 km, M = 1 Mearth | INVALID_INPUT | Hohmann transfer between circular orbits: the two orbits must have different radii. |
| Hohmann transfer: r1 = 7000 km, r2 = 42164 km | NEEDS_INPUT | hohmann_transfer needs M (mass of the central body (kg, Mearth, Msun)). Example: solve('hohmann_transfer', r1='1 m', r2= |

<a id="apsis_speeds"></a>

### Speeds at periapsis and apoapsis: `apsis_speeds`

The orbital speeds at the closest and farthest points of an elliptical orbit, with its semi-major axis, eccentricity and period.

$$
v_p = \sqrt{\frac{2\mu\,r_a}{r_p (r_p + r_a)}},\quad v_a = \sqrt{\frac{2\mu\,r_p}{r_a (r_p + r_a)}}
$$

| input | meaning | condition |
|---|---|---|
| `rp` | periapsis distance from the centre of the central body |  |
| `ra` | apoapsis distance |  |
| `M` | mass of the central body |  |

| output | meaning |
|---|---|
| `vp` | speed at periapsis |
| `va` | speed at apoapsis |
| `a` | semi-major axis |
| `e` | eccentricity |
| `P` | orbital period |

- **Method:** energy and angular momentum: $vp = sqrt$(2 mu ra / (rp (rp + ra))), $va = vp$ rp / ra
- **Assumes:** $0 < rp < ra$; $M > 0$; two-body orbit, the orbiting mass negligible
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![An orbit around a mass M = 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/apsis-speeds-orbit)

*An orbit around a mass M = 1.988e+30 kg from periapsis 147.1 Gm to apoapsis 152.1 Gm, drawn to scale; velocity arrows to a common scale. The model returns vp = 3.03e+04 m/s, va = 2.928e+04 m/s (ratio 1.035, the inverse ratio of the distances), a = 149.6 Gm, e = 0.017, P = 31.56 Ms. Your inputs: rp = 1.47055e+11 m, ra = 1.52141e+11 m, M = 1.98841e+30 kg.*

![Speed at periapsis and speed at apoapsis as rp runs from 8](https://www.ai999x.com/?ax_fig=astronomy-ai999x/apsis-speeds-live-1)

*Speed at periapsis and speed at apoapsis as rp runs from 8.823e+10 to 1.5e+11 m, every other input as you entered it (22 solutions). Marked: your rp = 1.471e+11 m gives vp = 3.03e+04. Your inputs: rp = 1.47055e+11 m, ra = 1.52141e+11 m, M = 1.98841e+30 kg.*

![Speed at periapsis and speed at apoapsis as ra runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/apsis-speeds-live-2)

*Speed at periapsis and speed at apoapsis as ra runs from 1.478e+11 to 2.13e+11 m, every other input as you entered it (16 solutions). Marked: your ra = 1.521e+11 m gives vp = 3.03e+04. Your inputs: rp = 1.47055e+11 m, ra = 1.52141e+11 m, M = 1.98841e+30 kg.*

```python
astronomy_ai999x.solve('apsis_speeds', rp=1.0, ra=3.0, M=14982844642.8839)
# -> vp = 1.22474 m/s, va = 0.408248 m/s, a = 2 m, e = 0.5, P = 17.7715 s   (SOLVED)
```

```python
astronomy_ai999x.solve('apsis_speeds', rp=1.0, ra=7.0, M=14982844642.8839)
# -> vp = 1.32288 m/s, va = 0.188982 m/s, a = 4 m, e = 0.75, P = 50.2655 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Apsis speeds: rp = 1 m, ra = 3 m, M = 1.4982e10 kg | SOLVED | vp = 1.22471 m/s, va = 0.408237 m/s, a = 2 m, e = 0.5, P = 17.772 s |
| Apsis speeds: rp = 3 au, ra = 1 au, M = 1 Msun | INVALID_INPUT | Speeds at periapsis and apoapsis: the periapsis distance must not exceed the apoapsis distance. |
| Apsis speeds: rp = 1 au, ra = 3 au | NEEDS_INPUT | apsis_speeds needs M (mass of the central body). Example: solve('apsis_speeds', rp='1 m', ra='3 m', M='14982844642.8839  |

<a id="escape_delta_v"></a>

### Departure burn from a circular orbit: `escape_delta_v`

The velocity change that takes a spacecraft from a circular orbit of radius r onto an escape (or hyperbolic) trajectory with excess speed v_inf.

$$
\Delta v = \sqrt{v_\infty^2 + \frac{2\mu}{r}} - \sqrt{\frac{\mu}{r}}
$$

| input | meaning | condition |
|---|---|---|
| `M` | mass of the central body |  |
| `r` | radius of the circular orbit |  |
| `v_inf` | hyperbolic excess speed (default 0: just escape) |  |

| output | meaning |
|---|---|
| `dv` | departure delta-v |
| `v_circ` | circular orbital speed |
| `v_depart` | speed needed after the burn |

- **Method:** $dv = sqrt$(v_inf^2 + 2 mu / r) - sqrt(mu / r)
- **Assumes:** M, $r > 0$; v_inf >= 0; a single impulsive burn
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Departure delta-v as r runs from 4007 to 9349 km, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/escape-delta-v-live-1)

*Departure delta-v as r runs from 4007 to 9349 km, every other input as you entered it (41 solutions). Marked: your r = 6678 km gives dv = 3200. Your inputs: M = 5.97217e+24 kg, r = 6678 km.*

![Departure delta-v as M runs from 3](https://www.ai999x.com/?ax_fig=astronomy-ai999x/escape-delta-v-live-2)

*Departure delta-v as M runs from 3.583e+24 to 8.361e+24 kg, every other input as you entered it (29 solutions). Marked: your M = 5.972e+24 kg gives dv = 3200. Your inputs: M = 5.97217e+24 kg, r = 6678 km.*

```python
astronomy_ai999x.solve('escape_delta_v', M=14982844642.8839, r=1.0)
# -> dv = 0.414214 m/s, v_circ = 1 m/s, v_depart = 1.41421 m/s   (SOLVED)
```

```python
astronomy_ai999x.solve('escape_delta_v', M=14982844642.8839, r=1.0, v_inf=1.4142135623730951)
# -> dv = 1 m/s, v_circ = 1 m/s, v_depart = 2 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Escape delta-v: M = 5.972e24 kg, r = 6678 km | SOLVED | dv = 3200.1 m/s, v_circ = 7725.73 m/s, v_depart = 10925.8 m/s |
| Departure burn: M = 1 Mearth, r = 6678 km, v_inf = -1 km/s | INVALID_INPUT | Departure burn from a circular orbit: the hyperbolic excess speed must not be negative. |
| Departure burn: M = 1 Mearth | NEEDS_INPUT | escape_delta_v needs r (radius of the circular orbit). Example: solve('escape_delta_v', M='14982844642.8839 kg', r='1 m' |

<a id="plane_change_delta_v"></a>

### Delta-v of a combined plane change: `plane_change_delta_v`

The velocity change needed to turn a velocity v1 into a velocity v2 at an angle di to it (a plane change, possibly combined with a speed change).

$$
\Delta v = \sqrt{v_1^2 + v_2^2 - 2 v_1 v_2\cos\Delta i}
$$

| input | meaning | condition |
|---|---|---|
| `v1` | speed before the burn |  |
| `v2` | speed after the burn |  |
| `di` | angle between the two velocities (plane-change angle) |  |

| output | meaning |
|---|---|
| `dv` | delta-v of the burn |

- **Method:** law of cosines: $dv^{2} = v_{1}^{2} + v_{2}^{2} - 2$ v1 v2 cos(di), written as $(v_{1} - v_{2})^{2} + 4$ v1 v2 $sin^{2}$(di/2)
- **Assumes:** v1, $v_{2} > 0$; $0  \leq  di  \leq  180$ degrees; an impulsive burn
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Delta-v of the burn as di runs from 17](https://www.ai999x.com/?ax_fig=astronomy-ai999x/plane-change-delta-v-live-1)

*Delta-v of the burn as di runs from 17.1 to 39.9 deg, every other input as you entered it (41 solutions). Marked: your di = 28.5 deg gives dv = 3806. Your inputs: v1 = 7.73 km/s, v2 = 7.73 km/s, di = 28.5 deg.*

![Delta-v of the burn as v1 runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/plane-change-delta-v-live-2)

*Delta-v of the burn as v1 runs from 4.638 to 10.82 km/s, every other input as you entered it (29 solutions). Marked: your v1 = 7.73 km/s gives dv = 3806. Your inputs: v1 = 7.73 km/s, v2 = 7.73 km/s, di = 28.5 deg.*

```python
astronomy_ai999x.solve('plane_change_delta_v', v1=1.0, v2=1.0, di=1.0471975511965976)
# -> dv = 1 m/s   (SOLVED)
```

```python
astronomy_ai999x.solve('plane_change_delta_v', v1=3.0, v2=4.0, di=1.5707963267948966)
# -> dv = 5 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plane change: v1 = 3 km/s, v2 = 4 km/s, di = 90 deg | SOLVED | dv = 5000 m/s |
| Plane change: v1 = 7.7 km/s, v2 = 7.7 km/s, di = 60 deg | SOLVED | dv = 7700 m/s |
| Plane change: v1 = 3 km/s, v2 = 4 km/s, di = 200 deg | INVALID_INPUT | Delta-v of a combined plane change: the angle between the velocities must lie between 0 and 180 degrees. |
| Plane change: v1 = 3 km/s, di = 90 deg | NEEDS_INPUT | plane_change_delta_v needs v2 (speed after the burn). Example: solve('plane_change_delta_v', v1='1 m/s', v2='1 m/s', di= |

<a id="orbit_from_state"></a>

### Orbital elements from position and velocity, `orbit_from_state`

The semi-major axis, eccentricity, periapsis and apoapsis distances, period and specific energy of a bound orbit from the distance, speed and flight-path angle at one point.

$$
a = \frac{r}{2 - r v^2/\mu},\quad e = \sqrt{1 - \frac{(r v\cos\gamma)^2}{\mu a}}
$$

| input | meaning | condition |
|---|---|---|
| `M` | mass of the central body |  |
| `r` | distance from the centre of the central body |  |
| `v` | orbital speed at that point |  |
| `gamma` | flight-path angle above the local horizontal (default 0) |  |

| output | meaning |
|---|---|
| `a` | semi-major axis |
| `e` | eccentricity |
| `rp` | periapsis distance |
| `ra` | apoapsis distance |
| `P` | orbital period |
| `energy` | specific orbital energy (J/kg) |

- **Method:** vis-viva a = r / (2 - r v^2/mu); e^2 = (s $cos^{2}$ $g - 1)^{2} + (s$ cos g sin g)^2 with $s = r$ $v^{2}/mu$
- **Assumes:** $v^{2} < 2$ G M / r (bound orbit); |gamma| < 90 degrees; two-body motion
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Orbits launched from the same point r = 149](https://www.ai999x.com/?ax_fig=astronomy-ai999x/orbit-from-state-orbit)

*Orbits launched from the same point r = 149.6 Gm at 0.8, 1, 1.2 and 1.3 times the given speed 3.5e+04 m/s (the given flight-path angle), each ellipse drawn from the semi-major axis and eccentricity the model returns (unbound ones left out): 0.8 v: a = 134 Gm, e = 0.1162, periapsis 118.4 Gm, apoapsis 149.6 Gm; 1 v: a = 241.6 Gm, e = 0.3809, periapsis 149.6 Gm, apoapsis 333.6 Gm; 1.2 v: a = 12.94 Tm, e = 0.9884, periapsis 149.6 Gm, apoapsis 25.73 Tm. In SI units: a = 2.416e+11 m. Your inputs: M = 1.98841e+30 kg, r = 1.49598e+11 m, v = 35 km/s.*

![Semi-major axis, periapsis distance and apoapsis distance as r runs from 8](https://www.ai999x.com/?ax_fig=astronomy-ai999x/orbit-from-state-live-1)

*Semi-major axis, periapsis distance and apoapsis distance as r runs from 8.976e+10 to 2.094e+11 m, every other input as you entered it (41 solutions). Marked: your r = 1.496e+11 m gives a = 2.416e+11. Your inputs: M = 1.98841e+30 kg, r = 1.49598e+11 m, v = 35 km/s.*

![Semi-major axis, periapsis distance and apoapsis distance as M runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/orbit-from-state-live-2)

*Semi-major axis, periapsis distance and apoapsis distance as M runs from 1.42e+30 to 2.784e+30 kg, every other input as you entered it (25 solutions). Marked: your M = 1.988e+30 kg gives a = 2.416e+11. Your inputs: M = 1.98841e+30 kg, r = 1.49598e+11 m, v = 35 km/s.*

```python
astronomy_ai999x.solve('orbit_from_state', M=14982844642.8839, r=1.0, v=1.0)
# -> a = 1 m, e = 0, rp = 1 m, ra = 1 m, P = 6.28319 s, energy = -0.5 J/kg   (SOLVED)
```

```python
astronomy_ai999x.solve('orbit_from_state', M=14982844642.8839, r=1.0, v=1.224744871391589)
# -> a = 2 m, e = 0.5, rp = 1 m, ra = 3 m, P = 17.7715 s, energy = -0.25 J/kg   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Orbit from state: M = 1.4982e10 kg, r = 1 m, v = 1.2 m/s | SOLVED | a = 1.78597 m, e = 0.440081, rp = 1 m, ra = 2.57195 m, P = 14.997 s, energy = -0.279944 J/kg |
| Orbit from state: M = 1 Mearth, r = 7000 km, v = 20 km/s | INVALID_INPUT | Orbital elements from position and velocity: the orbit is not bound (the speed is at or above the escape speed). |
| Orbit from state: M = 1 Mearth, r = 7000 km | NEEDS_INPUT | orbit_from_state needs v (orbital speed at that point). Example: solve('orbit_from_state', M='14982844642.8839 kg', r='1 |

<a id="kepler_position_at_time"></a>

### Position on an elliptical orbit at a given time (Kepler's equation), `kepler_position_at_time`

The mean, eccentric and true anomalies, the distance and the speed of a body on an elliptical orbit a time t after periapsis passage.

$$
M = E - e\sin E,\quad \tan\frac{\nu}{2} = \sqrt{\frac{1+e}{1-e}}\tan\frac{E}{2},\quad r = a(1 - e\cos E)
$$

| input | meaning | condition |
|---|---|---|
| `M` | mass of the central body |  |
| `a` | semi-major axis |  |
| `e` | eccentricity $(0  \leq  e < 0.99)$ |  |
| `t` | time since periapsis passage |  |

| output | meaning |
|---|---|
| `mean_anomaly` | mean anomaly (degrees) |
| `E` | eccentric anomaly (degrees) |
| `nu` | true anomaly (degrees, 0 to 360) |
| `r` | distance from the central body's centre |
| `v` | orbital speed |
| `P` | orbital period |

- **Method:** Kepler's equation $M = E - e$ sin E solved by safeguarded Newton iteration; tan(nu/2) = sqrt($(1+e)/(1-e)$) tan(E/2); $r = a$($1 - e$ cos E)
- **Assumes:** $0  \leq  e < 0.99$; $t  \geq  0$ (reduced modulo the period); two-body motion
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![An orbit with a = 149](https://www.ai999x.com/?ax_fig=astronomy-ai999x/kepler-position-at-time-orbit)

*An orbit with a = 149.6 Gm and e = 0.5: 121 positions (true anomaly and distance) solved by the model from Kepler's equation over one period P = 31.56 Ms, and 16 positions at equal time steps (dots): equal areas swept, bunched near apoapsis. Marked: the given time 8.64 Ms, ν = 145°, r = 190 Gm, speed 2.258e+04 m/s. Your inputs: M = 1.98841e+30 kg, a = 1.49598e+11 m, e = 0.5, t = 100 day.*

![True anomaly as a runs from 8](https://www.ai999x.com/?ax_fig=astronomy-ai999x/kepler-position-at-time-live-1)

*True anomaly as a runs from 8.976e+10 to 2.094e+11 m, every other input as you entered it (41 solutions). Marked: your a = 1.496e+11 m gives ν = 145. Your inputs: M = 1.98841e+30 kg, a = 1.49598e+11 m, e = 0.5, t = 100 day.*

![True anomaly as t runs from 60 to 140 day, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/kepler-position-at-time-live-2)

*True anomaly as t runs from 60 to 140 day, every other input as you entered it (29 solutions). Marked: your t = 100 day gives ν = 145. Your inputs: M = 1.98841e+30 kg, a = 1.49598e+11 m, e = 0.5, t = 100 day.*

```python
astronomy_ai999x.solve('kepler_position_at_time', M=14982844642.8839, a=1.0, e=0.0, t=1.5707963267948966)
# -> mean_anomaly = 90, E = 90, nu = 90, r = 1 m, v = 1 m/s, P = 6.28319 s   (SOLVED)
```

```python
astronomy_ai999x.solve('kepler_position_at_time', M=14982844642.8839, a=1.0, e=0.5, t=3.141592653589793)
# -> mean_anomaly = 180, E = 180, nu = 180, r = 1.5 m, v = 0.57735 m/s, P = 6.28319 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Kepler's equation: M = 1.4982e10 kg, a = 1 m, e = 0.5, t = 3.14159265358979 s | SOLVED | mean_anomaly = 179.995, E = 179.997, nu = 179.998, r = 1.5 m, v = 0.577334 m/s, P = 6.28336 s |
| Kepler's equation: M = 1 Msun, a = 1 au, e = 1.2, t = 100 day | INVALID_INPUT | Position on an elliptical orbit at a given time (Kepler's equation): the eccentricity must satisfy 0 <= e < 0.99 (an ell |
| Kepler's equation: M = 1 Msun, a = 1 au, e = 0.5 | NEEDS_INPUT | kepler_position_at_time needs t (time since periapsis passage). Example: solve('kepler_position_at_time', M='14982844642 |

<a id="time_of_flight"></a>

### Time of flight between two points of an elliptical orbit, `time_of_flight`

The time a body takes to move forward along an elliptical orbit from true anomaly nu1 to true anomaly nu2.

$$
\Delta t = \frac{(E_2 - e\sin E_2) - (E_1 - e\sin E_1)}{n},\quad n = \sqrt{\mu/a^3}
$$

| input | meaning | condition |
|---|---|---|
| `M` | mass of the central body |  |
| `a` | semi-major axis |  |
| `e` | eccentricity $(0  \leq  e < 0.99)$ |  |
| `nu1` | starting true anomaly |  |
| `nu2` | final true anomaly |  |

| output | meaning |
|---|---|
| `dt` | time of flight |
| `dt_days` | time of flight in days |
| `P` | orbital period |

- **Method:** eccentric anomalies from the true anomalies, mean anomalies $M = E - e$ sin E, $dt = (M_{2} - M_{1})/n$
- **Assumes:** $0  \leq  e < 0.99$; $nu1  \neq  nu2$; motion in the forward (prograde) direction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Equal 90° sweeps of true anomaly on an orbit with a = 188](https://www.ai999x.com/?ax_fig=astronomy-ai999x/time-of-flight-orbit)

*Equal 90° sweeps of true anomaly on an orbit with a = 188.5 Gm and e = 0.2: from -45° to 45° about periapsis the model returns 7.542 Ms, from 135° to 225° about apoapsis 15.63 Ms, of a period of 44.63 Ms; the shaded areas take times in proportion to their areas (Kepler's second law; the shapes are drawn from the conic r = a(1 - e²)/(1 + e cos ν)). The given sweep takes 8.336 Ms. In SI units: dt = 8.336e+06 s. Your inputs: M = 1.98841e+30 kg, a = 1.88493e+11 m, e = 0.2, nu1 = 0 deg, nu2 = 90 deg.*

![Time of flight and orbital period as a runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/time-of-flight-live-1)

*Time of flight and orbital period as a runs from 1.131e+11 to 2.639e+11 m, every other input as you entered it (41 solutions). Marked: your a = 1.885e+11 m gives dt = 96.48 day. Your inputs: M = 1.98841e+30 kg, a = 1.88493e+11 m, e = 0.2, nu1 = 0 deg, nu2 = 90 deg.*

![Time of flight as ν2 runs from 54 to 126 deg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/time-of-flight-live-2)

*Time of flight as ν2 runs from 54 to 126 deg, every other input as you entered it (29 solutions). Marked: your ν2 = 90 deg gives dt = 0.2642 yr. Your inputs: M = 1.98841e+30 kg, a = 1.88493e+11 m, e = 0.2, nu1 = 0 deg, nu2 = 90 deg.*

```python
astronomy_ai999x.solve('time_of_flight', M=14982844642.8839, a=1.0, e=0.0, nu1=0.0, nu2=3.141592653589793)
# -> dt = 3.14159 s, dt_days = 3.6361e-05, P = 6.28319 s   (SOLVED)
```

```python
astronomy_ai999x.solve('time_of_flight', M=14982844642.8839, a=1.0, e=0.5, nu1=0.0, nu2=3.141592653589793)
# -> dt = 3.14159 s, dt_days = 3.6361e-05, P = 6.28319 s   (SOLVED)
```

```python
astronomy_ai999x.solve('time_of_flight', M=14982844642.8839, a=1.0, e=0.5, nu1=3.141592653589793, nu2=0.0)
# -> dt = 3.14159 s, dt_days = 3.6361e-05, P = 6.28319 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Time of flight: M = 1.4982e10 kg, a = 1 m, e = 0.5, nu1 = 0 deg, nu2 = 180 deg | SOLVED | dt = 3.14168 s, dt_days = 3.63621e-05, P = 6.28336 s |
| Time of flight: M = 1 Msun, a = 1 au, e = 0.5, nu1 = 30 deg, nu2 = 30 deg | INVALID_INPUT | Time of flight between two points of an elliptical orbit: the two true anomalies must differ. |
| Time of flight: M = 1 Msun, a = 1 au, e = 0.5, nu1 = 30 deg | NEEDS_INPUT | time_of_flight needs nu2 (final true anomaly). Example: solve('time_of_flight', M='14982844642.8839 kg', a='1 m', e=0, n |

<a id="radial_velocity_semi_amplitude"></a>

### Radial-velocity semi-amplitude of a star: `radial_velocity_semi_amplitude`

The semi-amplitude K of the star's line-of-sight velocity wobble caused by an orbiting planet (or companion) of mass m.

$$
K = \left(\frac{2\pi G}{P}\right)^{1/3}\frac{m\sin i}{(M_\star + m)^{2/3}\sqrt{1 - e^2}}
$$

| input | meaning | condition |
|---|---|---|
| `M_star` | mass of the star (Msun, kg) |  |
| `m_p` | mass of the planet (Mjup, Mearth, kg) |  |
| `P` | orbital period |  |
| `e` | eccentricity (default 0) |  |
| `i` | orbital inclination (default 90 degrees, edge-on) |  |

| output | meaning |
|---|---|
| `K` | radial-velocity semi-amplitude of the star |
| `a` | relative semi-major axis |
| `a_au` | relative semi-major axis in au |

- **Method:** K = (2 pi G / P)^(1/3) m sin i / ($(M + m)^{2/3}$ sqrt(1 - e^2))
- **Assumes:** $0  \leq  e < 0.99$; $0 < i  \leq  180$ degrees; two-body Keplerian orbit
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Radial-velocity semi-amplitude of the star as m_p runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/radial-velocity-semi-amplitude-live-1)

*Radial-velocity semi-amplitude of the star as m_p runs from 1.139e+27 to 2.657e+27 kg, every other input as you entered it (41 solutions). Marked: your m_p = 1.898e+27 kg gives K = 12.46. Your inputs: M_star = 1.98841e+30 kg, m_p = 1.89812e+27 kg, P = 11.86 yr.*

![Radial-velocity semi-amplitude of the star as P runs from 7](https://www.ai999x.com/?ax_fig=astronomy-ai999x/radial-velocity-semi-amplitude-live-2)

*Radial-velocity semi-amplitude of the star as P runs from 7.116 to 16.6 yr, every other input as you entered it (29 solutions). Marked: your P = 11.86 yr gives K = 12.46. Your inputs: M_star = 1.98841e+30 kg, m_p = 1.89812e+27 kg, P = 11.86 yr.*

```python
astronomy_ai999x.solve('radial_velocity_semi_amplitude', M_star=14967861798.241016, m_p=14982844.6428839, P=6.283185307179586)
# -> K = 0.001 m/s, a = 1 m, a_au = 6.68459e-12   (SOLVED)
```

```python
astronomy_ai999x.solve('radial_velocity_semi_amplitude', M_star=7491422321.44195, m_p=7491422321.44195, P=6.283185307179586, e=0.6, i=0.5235987755982988)
# -> K = 0.3125 m/s, a = 1 m, a_au = 6.68459e-12   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup, P = 11.86 yr | SOLVED | K = 12.4598 m/s, a = 7.78242e+11 m, a_au = 5.20222 |
| Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup, P = 11.86 yr, e = 1.5 | INVALID_INPUT | Radial-velocity semi-amplitude of a star: the eccentricity must satisfy 0 <= e < 0.99. |
| Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup | NEEDS_INPUT | radial_velocity_semi_amplitude needs P (orbital period). Example: solve('radial_velocity_semi_amplitude', M_star='149678 |

<a id="transit_depth"></a>

### Transit depth: `transit_depth`

The fractional dimming of a uniformly bright star when a planet of radius Rp crosses it at impact parameter b (b = 0 central; grazing transits included).

$$
\delta = \frac{A_\mathrm{overlap}}{\pi R_\star^2} = \left(\frac{R_p}{R_\star}\right)^2\ \text{(full transit)}
$$

| input | meaning | condition |
|---|---|---|
| `Rp` | planet radius (Rjup, Rearth, km) |  |
| `Rs` | stellar radius (Rsun, km) |  |
| `b` | impact parameter in stellar radii (default 0) |  |

| output | meaning |
|---|---|
| `depth` | fractional flux drop |
| `depth_ppm` | depth in parts per million |
| `k` | radius ratio Rp/Rs |
| `grazing` | whether the planet's disk crosses the stellar limb |

- **Method:** overlapping area of the two disks divided by the stellar disk area; $(Rp/Rs)^{2}$ for a full transit
- **Assumes:** $Rp < Rs$; $0  \leq  b < 1 + Rp/Rs$; uniform stellar disk (no limb darkening)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Planets crossing a Sun-sized star (radius 6](https://www.ai999x.com/?ax_fig=astronomy-ai999x/transit-depth-sketch)

*Planets crossing a Sun-sized star (radius 6.957e8 m), drawn to scale: Earth (6.371e6 m), Neptune (2.4622e7 m) and Jupiter (7.1492e7 m) through the centre, Jupiter on a chord with impact parameter 0.95 (stated; dashed: the path), and the given planet across its star, drawn with its star at the same size. Marked: the fractional dimming the model returns, Earth 83.86 ppm, Neptune 1253 ppm, Jupiter 1.056e+04 ppm, Jupiter, b = 0.95 8337 ppm, given 1.056e+04 ppm. In SI units: depth = 0.01056. Your inputs: Rp = 7.1492e+07 m, Rs = 6.957e+08 m.*

![Fractional flux drop as Rp runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/transit-depth-live-1)

*Fractional flux drop as Rp runs from 4.29e+07 to 1.001e+08 m, every other input as you entered it (41 solutions). Marked: your Rp = 7.149e+07 m gives depth = 0.01056. Your inputs: Rp = 7.1492e+07 m, Rs = 6.957e+08 m.*

![Fractional flux drop as Rs runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/transit-depth-live-2)

*Fractional flux drop as Rs runs from 4.174e+08 to 9.74e+08 m, every other input as you entered it (29 solutions). Marked: your Rs = 6.957e+08 m gives depth = 0.01056. Your inputs: Rp = 7.1492e+07 m, Rs = 6.957e+08 m.*

```python
astronomy_ai999x.solve('transit_depth', Rp=1.0, Rs=10.0)
# -> depth = 0.01, depth_ppm = 10000, k = 0.1, grazing = False   (SOLVED)
```

```python
astronomy_ai999x.solve('transit_depth', Rp=1.0, Rs=10.0, b=0.5)
# -> depth = 0.01, depth_ppm = 10000, k = 0.1, grazing = False   (SOLVED)
```

```python
astronomy_ai999x.solve('transit_depth', Rp=1.0, Rs=10.0, b=1.0)
# -> depth = 0.00489387, depth_ppm = 4893.87, k = 0.1, grazing = True   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transit depth: Rp = 1 Rjup, Rs = 1 Rsun | SOLVED | depth = 0.0105602, depth_ppm = 10560.2, k = 0.102763, grazing = False |
| Transit depth: Rp = 0.1 m, Rs = 1 m, b = 0.3 | SOLVED | depth = 0.01, depth_ppm = 10000, k = 0.1, grazing = False |
| Transit depth: Rp = 1 Rjup, Rs = 1 Rsun, b = 2 | INVALID_INPUT | Transit depth: no transit: the impact parameter must satisfy 0 <= b < 1 + Rp/Rs. |
| Transit depth: Rp = 1 Rjup | NEEDS_INPUT | transit_depth needs Rs (stellar radius (Rsun, km)). Example: solve('transit_depth', Rp='1 m', Rs='10 m'). |

<a id="transit_duration"></a>

### Transit duration: `transit_duration`

The total (first to fourth contact) and full (second to third contact) durations of a transit of a planet on a circular orbit, and its impact parameter.

$$
T_{14} = \frac{P}{\pi}\arcsin\left(\frac{\sqrt{(R_\star + R_p)^2 - (bR_\star)^2}}{a\sin i}\right),\quad b = \frac{a\cos i}{R_\star}
$$

| input | meaning | condition |
|---|---|---|
| `P` | orbital period |  |
| `a` | orbit radius (au, m) |  |
| `Rs` | stellar radius |  |
| `Rp` | planet radius |  |
| `i` | orbital inclination (90 degrees = edge-on) |  |

| output | meaning |
|---|---|
| `T14` | total transit duration |
| `T14_h` | total duration in hours |
| `T23` | full-transit duration (absent for a grazing transit) |
| `b` | impact parameter a cos i / Rs |

- **Method:** $T14 = (P/pi)$ asin( sqrt((Rs + Rp)^2 - (b Rs)^2) / (a sin i) ), T23 with Rs - Rp
- **Assumes:** circular orbit; $Rp < Rs$; $a > Rs + Rp$; $0 < i  \leq  90$ degrees; $b < 1 + Rp/Rs$ (the planet transits)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Total transit duration and full-transit duration as Rs runs from 5](https://www.ai999x.com/?ax_fig=astronomy-ai999x/transit-duration-live-1)

*Total transit duration and full-transit duration as Rs runs from 5.009e+08 to 1.169e+09 m, every other input as you entered it (41 solutions). Marked: your Rs = 8.348e+08 m gives T14 = 3.23 h. Your inputs: P = 304128 s, a = 7.0311e+09 m, Rs = 8.3484e+08 m, Rp = 9.8659e+07 m, i = 86.7 deg.*

![Total transit duration and full-transit duration as a runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/transit-duration-live-2)

*Total transit duration and full-transit duration as a runs from 4.219e+09 to 9.844e+09 m, every other input as you entered it (29 solutions). Marked: your a = 7.031e+09 m gives T14 = 3.23 h. Your inputs: P = 304128 s, a = 7.0311e+09 m, Rs = 8.3484e+08 m, Rp = 9.8659e+07 m, i = 86.7 deg.*

```python
astronomy_ai999x.solve('transit_duration', P=6.283185307179586, a=2.0, Rs=0.9, Rp=0.1, i=1.5707963267948966)
# -> T14 = 1.0472 s, T23 = 0.823034 s, T14_h = 0.000290888, b = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transit duration: P = 6.283185307179586 s, a = 2 m, Rs = 0.9 m, Rp = 0.1 m, i = 90 deg | SOLVED | T14 = 1.0472 s, T23 = 0.823034 s, T14_h = 0.000290888, b = 0 |
| Transit duration: P = 365.25 d, a = 1 au, Rs = 1 Rsun, Rp = 1 Rearth, i = 80 deg | INVALID_INPUT | Transit duration: the planet does not transit (impact parameter b >= 1 + Rp/Rs). |
| Transit duration: P = 365.25 d, a = 1 au, Rs = 1 Rsun, Rp = 1 Rearth | NEEDS_INPUT | transit_duration needs i (orbital inclination (90 degrees = edge-on)). Example: solve('transit_duration', P='6.283185307 |

<a id="cosmological_distances"></a>

### Cosmological distances at a redshift: `cosmological_distances`

The comoving, luminosity and angular-diameter distances and the distance modulus of an object at redshift z in a flat Lambda-CDM universe.

$$
D_C = \frac{c}{H_0}\int_0^z \frac{dz'}{\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}},\quad D_L = (1+z)D_C,\quad D_A = \frac{D_C}{1+z}
$$

| input | meaning | condition |
|---|---|---|
| `z` | redshift |  |
| `H0` | Hubble constant in km/s/Mpc |  |
| `Om` | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) |  |

| output | meaning |
|---|---|
| `D_C` | comoving distance |
| `D_C_Mpc` | comoving distance in Mpc |
| `D_L_Mpc` | luminosity distance in Mpc |
| `D_A_Mpc` | angular-diameter distance in Mpc |
| `mu` | distance modulus |
| `D_L` | luminosity distance |

- **Method:** $D_C = (c/H_{0})$ integral_0^z dz'/E(z'), $E = sqrt$(Om (1+z)^3 + 1 - Om) (mpmath quadrature); $D_L = (1+z)$ D_C; $D_A = D_C/(1+z)$
- **Assumes:** $0 < z  \leq  1100$; $1  \leq  H_{0}  \leq  1000$ km/s/Mpc; $0.01  \leq  Omega_m  \leq  1$; flat, no radiation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A Hubble diagram: distance modulus against redshift (41 redshifts from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/cosmological-distances-hubble)

*A Hubble diagram: distance modulus against redshift (41 redshifts from 0.01 to 1.2, H0 = 67.7 km/s/Mpc), each a solve of the model for a flat universe with Ωm = 0.3 and with the given Ωm = 0.31. Marked: the given z = 0.5, μ = 42.33. In SI units: D_L_Mpc = 2919. Your inputs: z = 0.5, H0 = 67.7, Om = 0.31.*

![Luminosity distance in Mpc as z runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/cosmological-distances-live-1)

*Luminosity distance in Mpc as z runs from 0.125 to 0.875, every other input as you entered it (41 solutions). Marked: your z = 0.5 gives D_L_Mpc = 2919. Your inputs: z = 0.5, H0 = 67.7, Om = 0.31.*

![Luminosity distance in Mpc as H0 runs from 16](https://www.ai999x.com/?ax_fig=astronomy-ai999x/cosmological-distances-live-2)

*Luminosity distance in Mpc as H0 runs from 16.93 to 118.5, every other input as you entered it (29 solutions). Marked: your H0 = 67.7 gives D_L_Mpc = 2919. Your inputs: z = 0.5, H0 = 67.7, Om = 0.31.*

```python
astronomy_ai999x.solve('cosmological_distances', z=3.0, H0=70.0, Om=1.0)
# -> D_C = 1.32152e+26 m, D_C_Mpc = 4282.75, D_L_Mpc = 17131, D_A_Mpc = 1070.69, mu = 46.1689, D_L = 5.28607e+26 m   (SOLVED)
```

```python
astronomy_ai999x.solve('cosmological_distances', z=8.0, H0=100.0, Om=1.0)
# -> D_C = 1.23342e+26 m, D_C_Mpc = 3997.23, D_L_Mpc = 35975.1, D_A_Mpc = 444.137, mu = 47.78, D_L = 1.11008e+27 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cosmological distances: z = 3, H0 = 70, Om = 1 | SOLVED | D_C = 1.32152e+26 m, D_C_Mpc = 4282.75, D_L_Mpc = 17131, D_A_Mpc = 1070.69, mu = 46.1689, D_L = 5.28607e+26 m |
| Cosmological distances: z = 1, H0 = 70 km/s/Mpc, Om = 1.5 | INVALID_INPUT | Cosmological distances at a redshift: Omega_m must lie between 0.01 and 1 (flat universe: Omega_Lambda = 1 - Omega_m). |
| Cosmological distances: z = 1, H0 = 70 | NEEDS_INPUT | cosmological_distances needs Om (matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m)). Example: solve('c |

<a id="angular_size_at_redshift"></a>

### Angular size of an object at a redshift: `angular_size_at_redshift`

The angle subtended by an object of proper size D at redshift z, and the proper length per arcsecond, in a flat Lambda-CDM universe.

$$
\theta = \frac{D}{D_A},\quad D_A = \frac{1}{1+z}\frac{c}{H_0}\int_0^z\frac{dz'}{E(z')}
$$

| input | meaning | condition |
|---|---|---|
| `z` | redshift |  |
| `H0` | Hubble constant in km/s/Mpc |  |
| `Om` | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) |  |
| `D` | proper size of the object (kpc, Mpc, m) |  |

| output | meaning |
|---|---|
| `theta_arcsec` | angular size in arcseconds |
| `scale_kpc_per_arcsec` | proper kiloparsecs per arcsecond |
| `D_A_Mpc` | angular-diameter distance in Mpc |

- **Method:** $\theta = D / D_A$ with $D_A = D_C / (1 + z)$
- **Assumes:** $0 < z  \leq  1100$; $D > 0$; small angles; flat Lambda-CDM, no radiation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Angular size in arcseconds as H0 runs from 17](https://www.ai999x.com/?ax_fig=astronomy-ai999x/angular-size-at-redshift-live-1)

*Angular size in arcseconds as H0 runs from 17.5 to 122.5, every other input as you entered it (41 solutions). Marked: your H0 = 70 gives θ_arcsec = 1. Your inputs: z = 3, H0 = 70, Om = 1, D = 1.6017254774542562e+20 m.*

![Angular size in arcseconds as D runs from 9](https://www.ai999x.com/?ax_fig=astronomy-ai999x/angular-size-at-redshift-live-2)

*Angular size in arcseconds as D runs from 9.61e+19 to 2.242e+20 m, every other input as you entered it (29 solutions). Marked: your D = 1.602e+20 m gives θ_arcsec = 1. Your inputs: z = 3, H0 = 70, Om = 1, D = 1.6017254774542562e+20 m.*

```python
astronomy_ai999x.solve('angular_size_at_redshift', z=3.0, H0=70.0, Om=1.0, D=1.6017254774542562e+20)
# -> theta_arcsec = 1, scale_kpc_per_arcsec = 5.19084, D_A_Mpc = 1070.69   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Angular size at redshift: z = 3, H0 = 70, Om = 1, D = 1 Mpc | SOLVED | theta_arcsec = 192.647, scale_kpc_per_arcsec = 5.19084, D_A_Mpc = 1070.69 |
| Angular size at redshift: z = 1, H0 = 70, Om = 0.3, D = -5 kpc | INVALID_INPUT | Angular size of an object at a redshift: the size must be a positive number. |
| Angular size at redshift: z = 1, H0 = 70, Om = 0.3 | NEEDS_INPUT | angular_size_at_redshift needs D (proper size of the object (kpc, Mpc, m)). Example: solve('angular_size_at_redshift', z |

<a id="lookback_time"></a>

### Lookback time to a redshift: `lookback_time`

How long ago the light received from redshift z was emitted, in a flat Lambda-CDM universe.

$$
t_L = \frac{1}{H_0}\int_0^z \frac{dz'}{(1+z')\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}}
$$

| input | meaning | condition |
|---|---|---|
| `z` | redshift |  |
| `H0` | Hubble constant in km/s/Mpc |  |
| `Om` | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) |  |

| output | meaning |
|---|---|
| `t_L` | lookback time |
| `t_L_Gyr` | lookback time in Gyr |

- **Method:** $t_L = (1/H_{0})$ integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature
- **Assumes:** $0 < z  \leq  1100$; flat Lambda-CDM, no radiation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Lookback time in Gyr as H0 runs from 17](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/lookback-time-live-1.svg)

*Lookback time in Gyr as H0 runs from 17.5 to 122.5, every other input as you entered it (41 solutions). Marked: your H0 = 70 gives t_L_Gyr = 8.148. Your inputs: z = 3, H0 = 70, Om = 1.*

![Lookback time in Gyr as Om runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/lookback-time-live-2)

*Lookback time in Gyr as Om runs from 0.25 to 1, every other input as you entered it (15 solutions). Marked: your Om = 1 gives t_L_Gyr = 8.148. Your inputs: z = 3, H0 = 70, Om = 1.*

```python
astronomy_ai999x.solve('lookback_time', z=3.0, H0=70.0, Om=1.0)
# -> t_L = 2.5714e+17 s, t_L_Gyr = 8.14827   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Lookback time: z = 3, H0 = 70, Om = 1 | SOLVED | t_L = 2.5714e+17 s, t_L_Gyr = 8.14827 |
| Lookback time: z = -1, H0 = 70, Om = 0.3 | INVALID_INPUT | Lookback time to a redshift: the redshift must lie between 0 and 1100. |
| Lookback time: z = 1, Om = 0.3 | NEEDS_INPUT | lookback_time needs H0 (Hubble constant in km/s/Mpc). Example: solve('lookback_time', z=3, H0=70, Om=1). |

<a id="age_at_redshift"></a>

### Age of the universe at a redshift: `age_at_redshift`

The age of a flat Lambda-CDM universe when the light from redshift z was emitted, and its present age.

$$
t(z) = \frac{1}{H_0}\int_z^\infty \frac{dz'}{(1+z')\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}}
$$

| input | meaning | condition |
|---|---|---|
| `z` | redshift |  |
| `H0` | Hubble constant in km/s/Mpc |  |
| `Om` | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) |  |

| output | meaning |
|---|---|
| `age` | age at redshift z |
| `age_Gyr` | age at redshift z in Gyr |
| `age_now_Gyr` | present age in Gyr |

- **Method:** t(z) = (1/H0) integral_z^inf dz' / ((1 + z') E(z')) by mpmath quadrature
- **Assumes:** $0  \leq  z  \leq  1100$; flat Lambda-CDM, no radiation (not accurate in the radiation era)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Age at redshift z in Gyr as H0 runs from 16](https://www.ai999x.com/?ax_fig=astronomy-ai999x/age-at-redshift-live-1)

*Age at redshift z in Gyr as H0 runs from 16.93 to 118.5, every other input as you entered it (41 solutions). Marked: your H0 = 67.7 gives age_Gyr = 13.8. Your inputs: z = 0, H0 = 67.7, Om = 0.31.*

![Age at redshift z in Gyr as z runs from 0 to 1, every other input as you entered it (15 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/age-at-redshift-live-2)

*Age at redshift z in Gyr as z runs from 0 to 1, every other input as you entered it (15 solutions). Marked: your z = 0 gives age_Gyr = 13.8. Your inputs: z = 0, H0 = 67.7, Om = 0.31.*

```python
astronomy_ai999x.solve('age_at_redshift', z=3.0, H0=70.0, Om=1.0)
# -> age = 3.67343e+16 s, age_Gyr = 1.16404, age_now_Gyr = 9.31231   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Age at redshift: z = 3, H0 = 70, Om = 1 | SOLVED | age = 3.67343e+16 s, age_Gyr = 1.16404, age_now_Gyr = 9.31231 |
| Age at redshift: z = 3, H0 = 70, Om = 0 | INVALID_INPUT | Age of the universe at a redshift: Omega_m must lie between 0.01 and 1 (flat universe: Omega_Lambda = 1 - Omega_m). |
| Age at redshift: z = 3 | NEEDS_INPUT | age_at_redshift needs H0, Om (Hubble constant in km/s/Mpc; matter density parameter Omega_m (flat: Omega_Lambda = 1 - Om |

<a id="redshift_at_lookback_time"></a>

### Redshift at a given lookback time: `redshift_at_lookback_time`

The redshift of light emitted a lookback time t ago in a flat Lambda-CDM universe (exact inverse of the age-redshift relation).

$$
1 + z = \left[\frac{\sqrt{\Omega_\Lambda/\Omega_m}}{\sinh\left(\tfrac{3}{2}H_0\sqrt{\Omega_\Lambda}\,(t_0 - t_L)\right)}\right]^{2/3}
$$

| input | meaning | condition |
|---|---|---|
| `t` | lookback time (Gyr, Myr, yr) |  |
| `H0` | Hubble constant in km/s/Mpc |  |
| `Om` | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) |  |

| output | meaning |
|---|---|
| `z` | redshift |
| `age_now_Gyr` | present age of the universe in Gyr |

- **Method:** t($z) = 2/(3$ H0 sqrt(OL)) asinh( sqrt(OL/Om) $(1+z)^{-3/2}$ ), inverted in closed form (Om = 1: t = 2/(3 H0) $(1+z)^{-3/2}$)
- **Assumes:** 0 < t < present age; flat Lambda-CDM, no radiation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Redshift as t runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/redshift-at-lookback-time-live-1)

*Redshift as t runs from 1.543e+17 to 2.931e+17 s, every other input as you entered it (28 solutions). Marked: your t = 2.571e+17 s gives z = 3. Your inputs: t = 2.571397984576139e+17 s, H0 = 70, Om = 1.*

![Redshift as H0 runs from 17](https://www.ai999x.com/?ax_fig=astronomy-ai999x/redshift-at-lookback-time-live-2)

*Redshift as H0 runs from 17.5 to 77.5, every other input as you entered it (17 solutions). Marked: your H0 = 70 gives z = 3. Your inputs: t = 2.571397984576139e+17 s, H0 = 70, Om = 1.*

```python
astronomy_ai999x.solve('redshift_at_lookback_time', t=2.571397984576139e+17, H0=70.0, Om=1.0)
# -> z = 3, age_now_Gyr = 9.31231   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Redshift at lookback time: t = 8 Gyr, H0 = 70, Om = 0.3 | SOLVED | z = 1.07467, age_now_Gyr = 13.467 |
| Redshift at lookback time: t = 30 Gyr, H0 = 70, Om = 0.3 | INVALID_INPUT | Redshift at a given lookback time: the lookback time must be shorter than the age of the universe. |
| Redshift at lookback time: t = 8 Gyr, H0 = 70 | NEEDS_INPUT | redshift_at_lookback_time needs Om (matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m)). Example: solve |

<a id="comoving_volume"></a>

### Comoving volume out to a redshift: `comoving_volume`

The all-sky comoving volume enclosed within redshift z in a flat Lambda-CDM universe.

$$
V_C = \frac{4\pi}{3}D_C^3
$$

| input | meaning | condition |
|---|---|---|
| `z` | redshift |  |
| `H0` | Hubble constant in km/s/Mpc |  |
| `Om` | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) |  |

| output | meaning |
|---|---|
| `V` | comoving volume |
| `V_Gpc3` | comoving volume in cubic gigaparsecs |

- **Method:** V = (4 pi / 3) $D_C^{3}$ (flat space)
- **Assumes:** $0 < z  \leq  1100$; flat Lambda-CDM, no radiation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Comoving volume in cubic gigaparsecs as H0 runs from 17](https://www.ai999x.com/?ax_fig=astronomy-ai999x/comoving-volume-live-1)

*Comoving volume in cubic gigaparsecs as H0 runs from 17.5 to 122.5, every other input as you entered it (41 solutions). Marked: your H0 = 70 gives V_Gpc3 = 329. Your inputs: z = 3, H0 = 70, Om = 1.*

![Comoving volume in cubic gigaparsecs as z runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/comoving-volume-live-2)

*Comoving volume in cubic gigaparsecs as z runs from 0.75 to 5.25, every other input as you entered it (29 solutions). Marked: your z = 3 gives V_Gpc3 = 329. Your inputs: z = 3, H0 = 70, Om = 1.*

```python
astronomy_ai999x.solve('comoving_volume', z=3.0, H0=70.0, Om=1.0)
# -> V = 9.66737e+78 m^3, V_Gpc3 = 329.046   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Comoving volume: z = 3, H0 = 70, Om = 1 | SOLVED | V = 9.66737e+78 m^3, V_Gpc3 = 329.046 |
| Comoving volume: z = 0, H0 = 70, Om = 0.3 | INVALID_INPUT | Comoving volume out to a redshift: the redshift must be positive. |
| Comoving volume: H0 = 70, Om = 0.3 | NEEDS_INPUT | comoving_volume needs z (redshift). Example: solve('comoving_volume', z=3, H0=70, Om=1). |

<a id="recession_velocity_from_redshift"></a>

### Radial velocity from a Doppler redshift: `recession_velocity_from_redshift`

The radial velocity of a source from its redshift interpreted as a special-relativistic Doppler shift (negative for a blueshift), with the low-velocity value cz for comparison.

$$
\frac{v}{c} = \frac{(1+z)^2 - 1}{(1+z)^2 + 1}
$$

| input | meaning | condition |
|---|---|---|
| `z` | redshift (negative for a blueshift) |  |

| output | meaning |
|---|---|
| `v` | radial velocity (positive receding) |
| `beta` | v/c |
| `v_cz` | non-relativistic estimate cz |

- **Method:** v/c = ((1+z)^2 - 1) / ((1+z)^2 + 1)
- **Assumes:** $-1 < z$; a purely kinematic (special-relativistic, radial) Doppler shift; not a cosmological recession
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Radial velocity and non-relativistic estimate cz as z runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/recession-velocity-from-redshift-live-1)

*Radial velocity and non-relativistic estimate cz as z runs from 0.125 to 0.875, every other input as you entered it (41 solutions). Marked: your z = 0.5 gives v = 1.153e+08. Your inputs: z = 0.5.*

![Every answer of one unit for your inputs: v (radial velocity (positive receding)) = 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/recession-velocity-from-redshift-live-2)

*Every answer of one unit for your inputs: v (radial velocity (positive receding)) = 1.153e+08; v_cz (non-relativistic estimate cz) = 1.499e+08. Your inputs: z = 0.5.*

```python
astronomy_ai999x.solve('recession_velocity_from_redshift', z=1.0)
# -> v = 1.79875e+08 m/s, beta = 0.6, v_cz = 2.99792e+08 m/s   (SOLVED)
```

```python
astronomy_ai999x.solve('recession_velocity_from_redshift', z=2.0)
# -> v = 2.39834e+08 m/s, beta = 0.8, v_cz = 5.99585e+08 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Radial velocity from redshift: z = 1 | SOLVED | v = 1.79875e+08 m/s, beta = 0.6, v_cz = 2.99792e+08 m/s |
| Radial velocity from redshift: z = -1.5 | INVALID_INPUT | Radial velocity from a Doppler redshift: the redshift must lie between -1 (excluded) and 10000. |
| Radial velocity from redshift: | NEEDS_INPUT | recession_velocity_from_redshift needs z (redshift (negative for a blueshift)). Example: solve('recession_velocity_from_ |

<a id="redshift_from_wavelengths"></a>

### Redshift and radial velocity from a spectral line, `redshift_from_wavelengths`

The redshift z = lambda_obs/lambda_rest - 1 of a spectral line and the corresponding relativistic Doppler radial velocity.

$$
z = \frac{\lambda_\mathrm{obs}}{\lambda_\mathrm{rest}} - 1,\quad \frac{v}{c} = \frac{(1+z)^2 - 1}{(1+z)^2 + 1}
$$

| input | meaning | condition |
|---|---|---|
| `lam_obs` | observed wavelength (nm, Angstrom, m) |  |
| `lam_rest` | rest (laboratory) wavelength |  |

| output | meaning |
|---|---|
| `z` | redshift |
| `v` | radial velocity (positive receding) |
| `beta` | v/c |
| `d_lambda` | wavelength shift |

- **Method:** z = lambda_obs/lambda_rest - 1; v/c = ((1+z)^2 - 1)/((1+z)^2 + 1)
- **Assumes:** both wavelengths > 0; a purely kinematic radial Doppler shift
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Four Balmer lines of hydrogen at their rest wavelengths (656](https://www.ai999x.com/?ax_fig=astronomy-ai999x/redshift-from-wavelengths-spectrum)

*Four Balmer lines of hydrogen at their rest wavelengths (656.28, 486.13, 434.05, 410.17 nm) and as observed, each observed wavelength 1.067 times the rest one (the given ratio); the model returns the same redshift for every pair, z = 0.06662, 0.06662, 0.06662, 0.06662, and a recession speed of 1.931e+04 km/s. Every line shifts by the same factor, so redder lines move farther. Your inputs: lam_obs = 700 nm, lam_rest = 656.28 nm.*

![Redshift as λ_obs runs from 420 to 980 nm, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/redshift-from-wavelengths-live-1)

*Redshift as λ_obs runs from 420 to 980 nm, every other input as you entered it (41 solutions). Marked: your λ_obs = 700 nm gives z = 0.06662. Your inputs: lam_obs = 700 nm, lam_rest = 656.28 nm.*

![Redshift as λ_rest runs from 393](https://www.ai999x.com/?ax_fig=astronomy-ai999x/redshift-from-wavelengths-live-2)

*Redshift as λ_rest runs from 393.8 to 918.8 nm, every other input as you entered it (29 solutions). Marked: your λ_rest = 656.3 nm gives z = 0.06662. Your inputs: lam_obs = 700 nm, lam_rest = 656.28 nm.*

```python
astronomy_ai999x.solve('redshift_from_wavelengths', lam_obs=2e-07, lam_rest=1e-07)
# -> z = 1, v = 1.79875e+08 m/s, beta = 0.6, d_lambda = 1e-07 m   (SOLVED)
```

```python
astronomy_ai999x.solve('redshift_from_wavelengths', lam_obs=3e-07, lam_rest=1e-07)
# -> z = 2, v = 2.39834e+08 m/s, beta = 0.8, d_lambda = 2e-07 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Redshift from spectral line: lam_obs = 1312.56 nm, lam_rest = 656.28 nm | SOLVED | z = 1, v = 1.79875e+08 m/s, beta = 0.6, d_lambda = 6.5628e-07 m |
| Redshift from spectral line: lam_obs = 0 nm, lam_rest = 656.28 nm | INVALID_INPUT | Redshift and radial velocity from a spectral line: the two wavelengths must be a positive number. |
| Redshift from spectral line: lam_obs = 700 nm | NEEDS_INPUT | redshift_from_wavelengths needs lam_rest (rest (laboratory) wavelength). Example: solve('redshift_from_wavelengths', lam |

<a id="rayleigh_resolution"></a>

### Diffraction limit of a telescope (Rayleigh criterion), `rayleigh_resolution`

The smallest angular separation a circular aperture of diameter D resolves at wavelength lambda: the angle of the first dark ring of the Airy pattern (1.21967 lambda/D for small angles; often rounded to 1.22).

$$
\sin\theta = \frac{j_{1,1}}{\pi}\frac{\lambda}{D} \approx 1.22\,\frac{\lambda}{D}
$$

| input | meaning | condition |
|---|---|---|
| `lam` | wavelength (nm, m) |  |
| `D` | aperture diameter |  |

| output | meaning |
|---|---|
| `theta` | angular resolution |
| `theta_arcsec` | angular resolution in arcseconds |

- **Method:** sin(theta) = j_{1,1} lambda / (pi D), j_{1,1} = 3.8317059702 (first zero of J1)
- **Assumes:** lambda << D; a circular unobstructed aperture, no atmosphere
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Angular resolution in arcseconds as λ runs from 330 to 770 nm, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/rayleigh-resolution-live-1)

*Angular resolution in arcseconds as λ runs from 330 to 770 nm, every other input as you entered it (41 solutions). Marked: your λ = 550 nm gives θ_arcsec = 0.05765. Your inputs: lam = 550 nm, D = 2.4 m.*

![Angular resolution in arcseconds as D runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/rayleigh-resolution-live-2)

*Angular resolution in arcseconds as D runs from 1.44 to 3.36 m, every other input as you entered it (29 solutions). Marked: your D = 2.4 m gives θ_arcsec = 0.05765. Your inputs: lam = 550 nm, D = 2.4 m.*

```python
astronomy_ai999x.solve('rayleigh_resolution', lam=0.20497349470552245, D=1.0)
# -> theta = 0.25268 rad, theta_arcsec = 52119   (SOLVED)
```

```python
astronomy_ai999x.solve('rayleigh_resolution', lam=5.5e-07, D=2.4)
# -> theta = 2.79508e-07 rad, theta_arcsec = 0.0576526   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rayleigh criterion: lam = 550 nm, D = 2.4 m | SOLVED | theta = 2.79508e-07 rad, theta_arcsec = 0.0576526 |
| Rayleigh criterion: lam = 2 m, D = 1 m | INVALID_INPUT | Diffraction limit of a telescope (Rayleigh criterion): the wavelength must be much smaller than the aperture (J11 lambda |
| Rayleigh criterion: lam = 550 nm | NEEDS_INPUT | rayleigh_resolution needs D (aperture diameter). Example: solve('rayleigh_resolution', lam='0.20497349470552245 m', D='1 |

<a id="telescope_magnification"></a>

### Telescope magnification, exit pupil and true field, `telescope_magnification`

The angular magnification of a telescope used with an eyepiece, the diameter of its exit pupil and (given the eyepiece's apparent field) its true field of view.

$$
M = \frac{f_\mathrm{obj}}{f_\mathrm{eye}},\quad d_\mathrm{exit} = \frac{D}{M}
$$

| input | meaning | condition |
|---|---|---|
| `f_obj` | focal length of the objective (mm, m) |  |
| `f_eye` | focal length of the eyepiece |  |
| `D` | aperture diameter |  |
| `afov` | apparent field of view of the eyepiece (optional) |  |

| output | meaning |
|---|---|
| `M` | angular magnification |
| `exit_pupil` | exit-pupil diameter |
| `exit_pupil_mm` | exit-pupil diameter in millimetres |
| `tfov_deg` | true field of view in degrees (when afov is given) |

- **Method:** M = f_obj / f_eye; exit pupil = D / M; true field = apparent field / M
- **Assumes:** focal lengths and aperture > 0; thin lenses, afocal (focused at infinity); true field = apparent field / M
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Angular magnification as f_obj runs from 720 to 1680 mm, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/telescope-magnification-live-1)

*Angular magnification as f_obj runs from 720 to 1680 mm, every other input as you entered it (41 solutions). Marked: your f_obj = 1200 mm gives M = 48. Your inputs: f_obj = 1200 mm, f_eye = 25 mm, D = 200 mm, afov = 52 deg.*

![Angular magnification as f_eye runs from 15 to 35 mm, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/telescope-magnification-live-2)

*Angular magnification as f_eye runs from 15 to 35 mm, every other input as you entered it (29 solutions). Marked: your f_eye = 25 mm gives M = 48. Your inputs: f_obj = 1200 mm, f_eye = 25 mm, D = 200 mm, afov = 52 deg.*

```python
astronomy_ai999x.solve('telescope_magnification', f_obj=1.2, f_eye=0.025, D=0.2, afov=0.9075712110370514)
# -> M = 48 1, exit_pupil = 0.00416667 m, exit_pupil_mm = 4.16667, tfov_deg = 1.08333   (SOLVED)
```

```python
astronomy_ai999x.solve('telescope_magnification', f_obj=2.0, f_eye=0.01, D=0.2)
# -> M = 200 1, exit_pupil = 0.001 m, exit_pupil_mm = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Telescope magnification: f_obj = 1200 mm, f_eye = 25 mm, D = 200 mm, afov = 52 deg | SOLVED | M = 48 1, exit_pupil = 0.00416667 m, exit_pupil_mm = 4.16667, tfov_deg = 1.08333 |
| Telescope magnification: f_obj = 1200 mm, f_eye = 0 mm, D = 200 mm | INVALID_INPUT | Telescope magnification, exit pupil and true field: the focal lengths and the aperture must be a positive number. |
| Telescope magnification: f_obj = 1200 mm, D = 200 mm | NEEDS_INPUT | telescope_magnification needs f_eye (focal length of the eyepiece). Example: solve('telescope_magnification', f_obj='1.2 |

<a id="light_gathering_power"></a>

### Light-gathering power of two telescopes: `light_gathering_power`

How many times more light one telescope collects than another (ratio of aperture areas) and the corresponding gain in limiting magnitude.

$$
\frac{P_1}{P_2} = \left(\frac{D_1}{D_2}\right)^2,\quad \Delta m = 5\log_{10}\frac{D_1}{D_2}
$$

| input | meaning | condition |
|---|---|---|
| `D1` | aperture of the first telescope |  |
| `D2` | aperture of the second telescope (or the eye's pupil) |  |

| output | meaning |
|---|---|
| `area_ratio` | light-gathering ratio $(D_{1}/D_{2})^{2}$ |
| `mag_gain` | gain in limiting magnitude 5 log10(D1/D2) |

- **Method:** ratio = (D1/D2)^2; gain = 2.5 log10(ratio)
- **Assumes:** D1, $D_{2} > 0$; unobstructed circular apertures
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![How much more light five apertures collect than a 7 mm dark-adapted eye (stated: binoculars 50 mm, a 200 mm telescope, Hubble 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/light-gathering-power-bars)

*How much more light five apertures collect than a 7 mm dark-adapted eye (stated: binoculars 50 mm, a 200 mm telescope, Hubble 2.4 m, the VLT 8.2 m, the ELT 39 m), and the given aperture against its own reference. Each bar is a solve of the model; the last is the given inputs. Marked: binoculars 51.02×, 200 mm 816.3×, Hubble 1.176e+05×, VLT 1.372e+06×, ELT 3.104e+07×, given 816.3×. Your inputs: D1 = 200 mm, D2 = 7 mm.*

![Light-gathering ratio (D1/D2)^2 as D2 runs from 4](https://www.ai999x.com/?ax_fig=astronomy-ai999x/light-gathering-power-live-1)

*Light-gathering ratio (D1/D2)^2 as D2 runs from 4.2 to 9.8 mm, every other input as you entered it (41 solutions). Marked: your D2 = 7 mm gives area_ratio = 816.3. Your inputs: D1 = 200 mm, D2 = 7 mm.*

![Light-gathering ratio (D1/D2)^2 as D1 runs from 120 to 280 mm, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/light-gathering-power-live-2)

*Light-gathering ratio (D1/D2)^2 as D1 runs from 120 to 280 mm, every other input as you entered it (29 solutions). Marked: your D1 = 200 mm gives area_ratio = 816.3. Your inputs: D1 = 200 mm, D2 = 7 mm.*

```python
astronomy_ai999x.solve('light_gathering_power', D1=0.2, D2=0.002)
# -> area_ratio = 10000, mag_gain = 10   (SOLVED)
```

```python
astronomy_ai999x.solve('light_gathering_power', D1=2.0, D2=1.0)
# -> area_ratio = 4, mag_gain = 1.50515   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Light-gathering power: D1 = 200 mm, D2 = 2 mm | SOLVED | area_ratio = 10000, mag_gain = 10 |
| Light-gathering power: D1 = 200 mm, D2 = -2 mm | INVALID_INPUT | Light-gathering power of two telescopes: the two apertures must be a positive number. |
| Light-gathering power: D1 = 200 mm | NEEDS_INPUT | light_gathering_power needs D2 (aperture of the second telescope (or the eye's pupil)). Example: solve('light_gathering_ |

<a id="camera_pixel_scale"></a>

### Pixel scale and field of view of a telescope camera, `camera_pixel_scale`

The angle on the sky covered by one detector pixel at the focal plane of a telescope and, for a row of N pixels, the field of view.

$$
\theta = 2\arctan\frac{w}{2f} \approx 206265''\,\frac{w}{f}
$$

| input | meaning | condition |
|---|---|---|
| `f` | focal length of the telescope |  |
| `p` | pixel size (um, mm) |  |
| `N` | number of pixels across the detector (optional) |  |

| output | meaning |
|---|---|
| `pixel_scale_arcsec` | pixel scale in arcseconds per pixel |
| `fov_deg` | field of view in degrees (with N) |
| `fov_arcmin` | field of view in arcminutes (with N) |

- **Method:** angle = 2 atan(w / (2 f)) for a width w centred on the optical axis (about 206265 w/f arcseconds)
- **Assumes:** f, $p > 0$; $N  \geq  1$; distortion-free optics; angles centred on the axis
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Fields of view of one camera (6248 pixels of 3](https://www.ai999x.com/?ax_fig=astronomy-ai999x/camera-pixel-scale-field-of-view)

*Fields of view of one camera (6248 pixels of 3.76 µm) behind telescopes of focal length 0.8, 1, 2 and 5.6 times the given 2 m, drawn to scale around the full Moon, whose angular diameter (31.08′) the model's angular-size calculation returns for 3.4748e6 m at 3.844e8 m. Marked: f = 1.6 m → 50.47′ field, 0.4847″ per pixel; f = 2 m → 40.38′ field, 0.3878″ per pixel; f = 4 m → 20.19′ field, 0.1939″ per pixel; f = 11.2 m → 7.211′ field, 0.06925″ per pixel. Your inputs: f = 2000 mm, p = 3.76 um, N = 6248.*

![Pixel scale in arcseconds per pixel as f runs from 1200 to 2800 mm, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/camera-pixel-scale-live-1)

*Pixel scale in arcseconds per pixel as f runs from 1200 to 2800 mm, every other input as you entered it (41 solutions). Marked: your f = 2000 mm gives pixel_scale_arcsec = 0.3878. Your inputs: f = 2000 mm, p = 3.76 um, N = 6248.*

![Pixel scale in arcseconds per pixel as p runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/camera-pixel-scale-live-2)

*Pixel scale in arcseconds per pixel as p runs from 2.256 to 5.264 um, every other input as you entered it (29 solutions). Marked: your p = 3.76 um gives pixel_scale_arcsec = 0.3878. Your inputs: f = 2000 mm, p = 3.76 um, N = 6248.*

```python
astronomy_ai999x.solve('camera_pixel_scale', f=0.5, p=1.0, N=1)
# -> pixel_scale_arcsec = 324000, fov_deg = 90, fov_arcmin = 5400   (SOLVED)
```

```python
astronomy_ai999x.solve('camera_pixel_scale', f=2.0, p=3.76e-06)
# -> pixel_scale_arcsec = 0.387778   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Pixel scale: f = 2000 mm, p = 3.76 um, N = 6248 | SOLVED | pixel_scale_arcsec = 0.387778, fov_deg = 0.673002, fov_arcmin = 40.3801 |
| Pixel scale: f = 2000 mm, p = 3.76 um, N = 0 | INVALID_INPUT | N: must be at least 1, got 0. |
| Pixel scale: f = 2000 mm | NEEDS_INPUT | camera_pixel_scale needs p (pixel size (um, mm)). Example: solve('camera_pixel_scale', f='0.5 m', p='1 m', N=1). |

<a id="airmass_extinction"></a>

### Airmass and atmospheric extinction: `airmass_extinction`

The airmass X = sec z of a star at zenith angle z (plane-parallel atmosphere) and, with an extinction coefficient k, the extinction k X and the magnitude corrected to outside the atmosphere.

$$
X = \sec z,\quad m_0 = m - kX
$$

| input | meaning | condition |
|---|---|---|
| `zen` | zenith angle (90 degrees minus the altitude) |  |
| `k` | extinction coefficient in magnitudes per airmass (optional) |  |
| `m` | observed magnitude (optional; needs k) |  |

| output | meaning |
|---|---|
| `X` | airmass |
| `alt_deg` | altitude in degrees |
| `extinction` | extinction k X in magnitudes |
| `m0` | magnitude above the atmosphere m - k X |

- **Method:** $X = 1/cos$ z; $m_{0} = m - k$ X
- **Assumes:** $0  \leq  z  \leq  85$ degrees; plane-parallel atmosphere; $k  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Airmass as zen runs from 36 to 84 deg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/airmass-extinction-live-1)

*Airmass as zen runs from 36 to 84 deg, every other input as you entered it (41 solutions). Marked: your zen = 60 deg gives X = 2. Your inputs: zen = 60 deg.*

```python
astronomy_ai999x.solve('airmass_extinction', zen=1.0471975511965976)
# -> X = 2, alt_deg = 30   (SOLVED)
```

```python
astronomy_ai999x.solve('airmass_extinction', zen=1.0471975511965976, k=0.2, m=10.4)
# -> X = 2, alt_deg = 30, extinction = 0.4, m0 = 10   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Airmass: zen = 60 deg | SOLVED | X = 2, alt_deg = 30 |
| Airmass: zen = 60 deg, k = 0.2, m = 10.4 | SOLVED | X = 2, alt_deg = 30, extinction = 0.4, m0 = 10 |
| Airmass: zen = 89 deg | INVALID_INPUT | Airmass and atmospheric extinction: the zenith angle must lie between 0 and 85 degrees (the plane-parallel model fails n |
| Airmass: | NEEDS_INPUT | airmass_extinction needs zen (zenith angle (90 degrees minus the altitude)). Example: solve('airmass_extinction', zen='1 |

<a id="altaz_from_hadec"></a>

### Altitude and azimuth from hour angle and declination, `altaz_from_hadec`

The altitude and azimuth of a star of declination dec at hour angle H seen from latitude lat (horizontal coordinates from equatorial ones).

$$
\sin a = \sin\phi\sin\delta + \cos\phi\cos\delta\cos H,\quad \tan A = \frac{-\cos\delta\sin H}{\sin\delta\cos\phi - \cos\delta\sin\phi\cos H}
$$

| input | meaning | condition |
|---|---|---|
| `H` | hour angle (hours or degrees; positive west of the meridian) |  |
| `dec` | declination |  |
| `lat` | observer's latitude |  |

| output | meaning |
|---|---|
| `alt_deg` | altitude in degrees (geometric, no refraction) |
| `az_deg` | azimuth in degrees from north through east |

- **Method:** sin $a = sin$(lat) sin(dec) + cos(lat) cos(dec) cos H; tan $A = -cos$(dec) sin H / (sin(dec) cos(lat) - cos(dec) sin(lat) cos H)
- **Assumes:** |dec| <= 90 degrees; |lat| <= 90 degrees; not at the zenith or nadir; no refraction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Daily paths across the sky of stars of four declinations seen from latitude 40° (azimuth around the rim, north up and east to the left, zenith distance outward; 61 hour angles each converted by the model to altitude and azimuth, only the part above the horizon drawn): δ = 60°: highest at altitude 70°, azimuth 3](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/altaz-from-hadec-polar.svg)

*Daily paths across the sky of stars of four declinations seen from latitude 40° (azimuth around the rim, north up and east to the left, zenith distance outward; 61 hour angles each converted by the model to altitude and azimuth, only the part above the horizon drawn): δ = 60°: highest at altitude 70°, azimuth 3.72e-14°; δ = 20°: highest at altitude 70°, azimuth 180°; δ = 0°: highest at altitude 50°, azimuth 180°; δ = -30°: highest at altitude 20°, azimuth 180°. Marked: the given position, altitude 19.52°, azimuth 134.1°. Your inputs: H = -45 deg, dec = -16.7 deg, lat = 40 deg.*

![Altitude in degrees as lat runs from 10 to 70 deg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/altaz-from-hadec-live-1)

*Altitude in degrees as lat runs from 10 to 70 deg, every other input as you entered it (41 solutions). Marked: your lat = 40 deg gives alt_deg = 19.52. Your inputs: H = -45 deg, dec = -16.7 deg, lat = 40 deg.*

![Altitude in degrees as H runs from -63 to -27 deg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/altaz-from-hadec-live-2)

*Altitude in degrees as H runs from -63 to -27 deg, every other input as you entered it (29 solutions). Marked: your H = -45 deg gives alt_deg = 19.52. Your inputs: H = -45 deg, dec = -16.7 deg, lat = 40 deg.*

```python
astronomy_ai999x.solve('altaz_from_hadec', H=1.5707963267948966, dec=0.5235987755982988, lat=0.7853981633974483)
# -> alt_deg = 20.7048, az_deg = 292.208   (SOLVED)
```

```python
astronomy_ai999x.solve('altaz_from_hadec', H=0.0, dec=0.0, lat=0.7853981633974483)
# -> alt_deg = 45, az_deg = 180   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Altitude-azimuth from hour angle: H = 0 h, dec = 0 deg, lat = 45 deg | SOLVED | alt_deg = 45, az_deg = 180 |
| Altitude-azimuth from hour angle: H = 6 h, dec = 0 deg, lat = 45 deg | SOLVED | alt_deg = 0, az_deg = 270 |
| Altitude-azimuth from hour angle: H = 0 h, dec = 100 deg, lat = 45 deg | INVALID_INPUT | Altitude and azimuth from hour angle and declination: the declination must lie between -90 and 90 degrees. |
| Altitude-azimuth from hour angle: H = 0 h, dec = 10 deg | NEEDS_INPUT | altaz_from_hadec needs lat (observer's latitude). Example: solve('altaz_from_hadec', H='1.5707963267948966 rad', dec='0. |

<a id="culmination_altitude"></a>

### Altitude of a star at culmination: `culmination_altitude`

The altitudes of a star when it crosses the meridian above the pole (upper culmination) and below it (lower culmination), and whether it is circumpolar or never rises.

$$
a_\mathrm{upper} = 90^\circ - |\phi - \delta|,\quad a_\mathrm{lower} = |\phi + \delta| - 90^\circ
$$

| input | meaning | condition |
|---|---|---|
| `dec` | declination |  |
| `lat` | observer's latitude |  |

| output | meaning |
|---|---|
| `alt_upper_deg` | altitude at upper culmination (degrees) |
| `alt_lower_deg` | altitude at lower culmination (degrees) |
| `circumpolar` | never sets |
| `never_rises` | never rises |

- **Method:** upper: $a = 90$ - |lat - dec|; lower: a = |lat + dec| - 90
- **Assumes:** |dec|, |lat| <= 90 degrees; geometric horizon, no refraction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The meridian seen edge-on from latitude 51](https://www.ai999x.com/?ax_fig=astronomy-ai999x/culmination-altitude-sky)

*The meridian seen edge-on from latitude 51.5°: horizon from north to south, the celestial pole at the latitude's altitude, the celestial equator (dashed) at right angles, and stars whose daily circles appear as chords from upper to lower culmination, the given declination among them (the culmination altitudes the model returns place the ends): δ = 38.78°: upper 77.28°, lower 0.28°; δ = 60°: upper 81.5°, lower 21.5°; δ = -10°: upper 28.5°, lower -48.5°. Your inputs: dec = 38.78 deg, lat = 51.5 deg.*

![Altitude at upper culmination as dec runs from 23](https://www.ai999x.com/?ax_fig=astronomy-ai999x/culmination-altitude-live-1)

*Altitude at upper culmination as dec runs from 23.27 to 54.29 deg, every other input as you entered it (41 solutions). Marked: your dec = 38.78 deg gives alt_upper_deg = 77.28. Your inputs: dec = 38.78 deg, lat = 51.5 deg.*

![Never sets for 20 5 x 5 combinations of dec and lat around your inputs (the other inputs as entered); the ring marks your inputs, which give yes](https://www.ai999x.com/?ax_fig=astronomy-ai999x/culmination-altitude-live-2)

*Never sets for 20 5 x 5 combinations of dec and lat around your inputs (the other inputs as entered); the ring marks your inputs, which give yes. Blank cells admit no answer. Your inputs: dec = 38.78 deg, lat = 51.5 deg.*

![Altitude at upper culmination as lat runs from 12](https://www.ai999x.com/?ax_fig=astronomy-ai999x/culmination-altitude-live-3)

*Altitude at upper culmination as lat runs from 12.88 to 87.26 deg, every other input as you entered it (28 solutions). Marked: your lat = 51.5 deg gives alt_upper_deg = 77.28. Your inputs: dec = 38.78 deg, lat = 51.5 deg.*

```python
astronomy_ai999x.solve('culmination_altitude', dec=0.3490658503988659, lat=0.8726646259971648)
# -> alt_upper_deg = 60, alt_lower_deg = -20, circumpolar = False, never_rises = False   (SOLVED)
```

```python
astronomy_ai999x.solve('culmination_altitude', dec=1.2217304763960306, lat=0.8726646259971648)
# -> alt_upper_deg = 70, alt_lower_deg = 30, circumpolar = True, never_rises = False   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Culmination altitude: dec = 20 deg, lat = 50 deg | SOLVED | alt_upper_deg = 60, alt_lower_deg = -20, circumpolar = False, never_rises = False |
| Culmination altitude: dec = -60 deg, lat = 50 deg | SOLVED | alt_upper_deg = -20, alt_lower_deg = -80, circumpolar = False, never_rises = True |
| Culmination altitude: dec = 20 deg, lat = 95 deg | INVALID_INPUT | Altitude of a star at culmination: the latitude must lie between -90 and 90 degrees. |
| Culmination altitude: dec = 20 deg | NEEDS_INPUT | culmination_altitude needs lat (observer's latitude). Example: solve('culmination_altitude', dec='0.3490658503988659 rad |

<a id="rise_set_hour_angle"></a>

### Rising and setting of a star: `rise_set_hour_angle`

The hour angle at which a star sets, how long it stays above the horizon (sidereal hours) and the azimuths at which it rises and sets (geometric horizon, no refraction).

$$
\cos H_0 = -\tan\phi\tan\delta,\quad \cos A_\mathrm{rise} = \frac{\sin\delta}{\cos\phi}
$$

| input | meaning | condition |
|---|---|---|
| `dec` | declination |  |
| `lat` | observer's latitude |  |

| output | meaning |
|---|---|
| `H_set_h` | hour angle at setting (hours) |
| `time_up_sidereal_h` | time above the horizon (sidereal hours) |
| `az_rise_deg` | azimuth of rising (degrees from north through east) |
| `az_set_deg` | azimuth of setting |

- **Method:** cos $H_{0} = -tan$(lat) tan(dec); cos $A_{rise} = \frac{\sin{\left(dec \right)}}{\cos{\left(lat \right)}}$
- **Assumes:** the star rises and sets: |tan(lat) tan(dec)| < 1; geometric horizon (no refraction, no star size)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Hour angle at setting as lat runs from 11](https://www.ai999x.com/?ax_fig=astronomy-ai999x/rise-set-hour-angle-live-1)

*Hour angle at setting as lat runs from 11.25 to 68.62 deg, every other input as you entered it (35 solutions). Marked: your lat = 45 deg gives H_set_h = 7.423. Your inputs: dec = 20 deg, lat = 45 deg.*

![Hour angle at setting as dec runs from 12 to 28 deg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/rise-set-hour-angle-live-2)

*Hour angle at setting as dec runs from 12 to 28 deg, every other input as you entered it (29 solutions). Marked: your dec = 20 deg gives H_set_h = 7.423. Your inputs: dec = 20 deg, lat = 45 deg.*

```python
astronomy_ai999x.solve('rise_set_hour_angle', dec=0.0, lat=0.7853981633974483)
# -> H_set_h = 6, time_up_sidereal_h = 12, az_rise_deg = 90, az_set_deg = 270   (SOLVED)
```

```python
astronomy_ai999x.solve('rise_set_hour_angle', dec=0.5235987755982988, lat=0.0)
# -> H_set_h = 6, time_up_sidereal_h = 12, az_rise_deg = 60, az_set_deg = 300   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rising and setting: dec = 0 deg, lat = 45 deg | SOLVED | H_set_h = 6, time_up_sidereal_h = 12, az_rise_deg = 90, az_set_deg = 270 |
| Rising and setting: dec = 80 deg, lat = 50 deg | INVALID_INPUT | Rising and setting of a star: the star does not rise and set at this latitude (it is circumpolar or never rises). |
| Rising and setting: dec = 20 deg | NEEDS_INPUT | rise_set_hour_angle needs lat (observer's latitude). Example: solve('rise_set_hour_angle', dec='0 rad', lat='0.785398163 |

<a id="angular_separation"></a>

### Angular separation of two sky positions: `angular_separation`

The great-circle angle between two objects given by their right ascensions and declinations.

$$
\cos\theta = \sin\delta_1\sin\delta_2 + \cos\delta_1\cos\delta_2\cos(\alpha_2 - \alpha_1)
$$

| input | meaning | condition |
|---|---|---|
| `ra1` | right ascension of object 1 (hours or degrees) |  |
| `dec1` | declination of object 1 |  |
| `ra2` | right ascension of object 2 |  |
| `dec2` | declination of object 2 |  |

| output | meaning |
|---|---|
| `sep_deg` | separation in degrees |
| `sep_arcsec` | separation in arcseconds |

- **Method:** Vincenty's formula (accurate at all separations)
- **Assumes:** |dec| <= 90 degrees
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Separation in degrees as ra2 runs from 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/angular-separation-live-1)

*Separation in degrees as ra2 runs from 0.9425 to 2.199 rad, every other input as you entered it (39 solutions). Marked: your ra2 = 1.571 rad gives sep_deg = 90. Your inputs: ra1 = 0 rad, dec1 = 0 rad, ra2 = 1.5707963267948966 rad, dec2 = 0 rad.*

![Separation in degrees as ra1 runs from -1 to 1 rad, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/angular-separation-live-2)

*Separation in degrees as ra1 runs from -1 to 1 rad, every other input as you entered it (29 solutions). Marked: your ra1 = 0 rad gives sep_deg = 90. Your inputs: ra1 = 0 rad, dec1 = 0 rad, ra2 = 1.5707963267948966 rad, dec2 = 0 rad.*

```python
astronomy_ai999x.solve('angular_separation', ra1=0.0, dec1=0.0, ra2=1.5707963267948966, dec2=0.0)
# -> sep_deg = 90, sep_arcsec = 324000   (SOLVED)
```

```python
astronomy_ai999x.solve('angular_separation', ra1=0.0, dec1=0.7853981633974483, ra2=3.141592653589793, dec2=0.7853981633974483)
# -> sep_deg = 90, sep_arcsec = 324000   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Angular separation: ra1 = 0 h, dec1 = 0 deg, ra2 = 6 h, dec2 = 0 deg | SOLVED | sep_deg = 90, sep_arcsec = 324000 |
| Angular separation: ra1 = 0 h, dec1 = 95 deg, ra2 = 6 h, dec2 = 0 deg | INVALID_INPUT | Angular separation of two sky positions: declinations must lie between -90 and 90 degrees. |
| Angular separation: ra1 = 0 h, dec1 = 0 deg | NEEDS_INPUT | angular_separation needs ra2, dec2 (right ascension of object 2; declination of object 2). Example: solve('angular_separ |

<a id="position_angle"></a>

### Position angle between two sky positions: `position_angle`

The position angle of object 2 as seen from object 1, measured from north through east.

$$
\tan\mathrm{PA} = \frac{\sin\Delta\alpha\cos\delta_2}{\cos\delta_1\sin\delta_2 - \sin\delta_1\cos\delta_2\cos\Delta\alpha}
$$

| input | meaning | condition |
|---|---|---|
| `ra1` | right ascension of object 1 (hours or degrees) |  |
| `dec1` | declination of object 1 |  |
| `ra2` | right ascension of object 2 |  |
| `dec2` | declination of object 2 |  |

| output | meaning |
|---|---|
| `pa_deg` | position angle in degrees (0 = north, 90 = east) |

- **Method:** tan $PA = sin$(dra) cos(dec2) / (cos(dec1) sin(dec2) - sin(dec1) cos(dec2) cos(dra))
- **Assumes:** |dec| < 90 degrees; distinct, non-antipodal positions
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Position angles of six companions around a star at the given first position, drawn as on the sky with north up and east to the left; each companion's offset in arcseconds east and north is stated (the right-ascension offset divided by cos δ)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/position-angle-sky)

*Position angles of six companions around a star at the given first position, drawn as on the sky with north up and east to the left; each companion's offset in arcseconds east and north is stated (the right-ascension offset divided by cos δ). The model measures each angle from north through east: (0″ E, 10″ N) → 0°; (8″ E, 6″ N) → 53.13°; (10″ E, 0″ N) → 90°; (6″ E, -8″ N) → 143.1°; (-6″ E, -8″ N) → 216.9°; (-10″ E, 2″ N) → 281.3°. The given pair: 0°. Your inputs: ra1 = 0 rad, dec1 = 0 rad, ra2 = 0 rad, dec2 = 0.1 rad.*

![Position angle in degrees as ra1 runs from -1 to 1 rad, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/position-angle-live-1)

*Position angle in degrees as ra1 runs from -1 to 1 rad, every other input as you entered it (41 solutions). Marked: your ra1 = 0 rad gives pa_deg = 0. Your inputs: ra1 = 0 rad, dec1 = 0 rad, ra2 = 0 rad, dec2 = 0.1 rad.*

![Position angle in degrees as dec1 runs from -1 to 1 rad, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/position-angle-live-2)

*Position angle in degrees as dec1 runs from -1 to 1 rad, every other input as you entered it (29 solutions). Marked: your dec1 = 0 rad gives pa_deg = 0. Your inputs: ra1 = 0 rad, dec1 = 0 rad, ra2 = 0 rad, dec2 = 0.1 rad.*

```python
astronomy_ai999x.solve('position_angle', ra1=0.0, dec1=0.0, ra2=0.0, dec2=0.1)
# -> pa_deg = 0   (SOLVED)
```

```python
astronomy_ai999x.solve('position_angle', ra1=0.0, dec1=0.0, ra2=0.1, dec2=0.0)
# -> pa_deg = 90   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Position angle: ra1 = 0 deg, dec1 = 0 deg, ra2 = 1 deg, dec2 = 0 deg | SOLVED | pa_deg = 90 |
| Position angle: ra1 = 0 deg, dec1 = 0 deg, ra2 = 0 deg, dec2 = 0 deg | INVALID_INPUT | Position angle between two sky positions: the two positions coincide (or are antipodal): no position angle. |
| Position angle: ra1 = 0 deg, dec1 = 0 deg | NEEDS_INPUT | position_angle needs ra2, dec2 (right ascension of object 2; declination of object 2). Example: solve('position_angle',  |

<a id="julian_date"></a>

### Julian date of a calendar date: `julian_date`

The Julian date and the modified Julian date of a Gregorian calendar date and time of day (in the same time scale as the date, e.g. TT or UT).

$$
\mathrm{JD} = \lfloor 365.25(y + 4716)\rfloor + \lfloor 30.6001(m + 1)\rfloor + D + B - 1524.5
$$

| input | meaning | condition |
|---|---|---|
| `year` | year (Gregorian, 1583 or later) |  |
| `month` | month $(1-12)$ |  |
| `day` | day of the month |  |
| `hour` | hour (0-23, default 0) |  |
| `minute` | minute (0-59, default 0) |  |
| `second` | second (0 <= s < 60, default 0) |  |

| output | meaning |
|---|---|
| `JD` | Julian date (days) |
| `MJD` | modified Julian date $JD - 2400000.5$ |

- **Method:** Meeus' algorithm in exact integer arithmetic: JD = floor(365.25(y+4716)) + floor(30.6001(m+1)) + $D + B - 1524.5$
- **Assumes:** Gregorian calendar (on or after 1582-10-15); a valid date and time of day (no leap second)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![6 dates on the Julian day count, the given one among them (dates as inputs, UT), each a solve of the model: Gregorian calendar 1583-01-01 0h → JD 2299238](https://www.ai999x.com/?ax_fig=astronomy-ai999x/julian-date-timeline)

*6 dates on the Julian day count, the given one among them (dates as inputs, UT), each a solve of the model: Gregorian calendar 1583-01-01 0h → JD 2299238.5 (MJD -1.008e+05); MJD day 0 1858-11-17 0h → JD 2400000.5 (MJD 0); 1900 January 0.5 1900-01-01 12h → JD 2415021 (MJD 1.502e+04); Unix epoch 1970-01-01 0h → JD 2440587.5 (MJD 4.059e+04); J2000.0 2000-01-01 12h → JD 2451545 (MJD 5.154e+04); given 2000-01-01 → JD 2451545 (MJD 5.154e+04). In SI units: JD = 2.452e+06. Your inputs: year = 2000, month = 1, day = 1, hour = 12.*

![Julian date as year runs from 1980 to 2020, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/julian-date-live-1)

*Julian date as year runs from 1980 to 2020, every other input as you entered it (41 solutions). Marked: your year = 2000 gives JD = 2.452e+06. Your inputs: year = 2000, month = 1, day = 1, hour = 12.*

![Julian date as month runs from 1 to 5, every other input as you entered it (5 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/julian-date-live-2)

*Julian date as month runs from 1 to 5, every other input as you entered it (5 solutions). Marked: your month = 1 gives JD = 2.452e+06. Your inputs: year = 2000, month = 1, day = 1, hour = 12.*

```python
astronomy_ai999x.solve('julian_date', year=2000, month=1, day=1, hour=12)
# -> JD = 2.45154e+06, MJD = 51544.5   (SOLVED)
```

```python
astronomy_ai999x.solve('julian_date', year=1957, month=10, day=4, hour=19, minute=26, second=24)
# -> JD = 2.43612e+06, MJD = 36115.8   (SOLVED)
```

```python
astronomy_ai999x.solve('julian_date', year=1858, month=11, day=18)
# -> JD = 2.4e+06, MJD = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Julian date: year = 2000, month = 1, day = 1, hour = 12 | SOLVED | JD = 2.45154e+06, MJD = 51544.5 |
| Julian date: year = 2023, month = 2, day = 29 | INVALID_INPUT | Julian date of a calendar date: day must be 1 to 28 in that month. |
| Julian date: year = 2000, month = 1 | NEEDS_INPUT | julian_date needs day (day of the month). Example: solve('julian_date', year=2000, month=1, day=1, hour=12). |

<a id="calendar_date_from_julian_date"></a>

### Calendar date from a Julian date: `calendar_date_from_julian_date`

The Gregorian calendar date and the time of day (decimal hours) of a Julian date.

$$
Z = \lfloor \mathrm{JD} + 0.5\rfloor,\ F = \mathrm{JD} + 0.5 - Z
$$

| input | meaning | condition |
|---|---|---|
| `jd` | Julian date (days) |  |

| output | meaning |
|---|---|
| `year` | year |
| `month` | month |
| `day` | day of the month |
| `hours` | time of day in decimal hours |

- **Method:** Meeus' inverse algorithm in exact integer and rational arithmetic
- **Assumes:** $2299160.5  \leq  JD  \leq  5373484.5$ (Gregorian era up to the year 9999)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Day of the month as jd runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/calendar-date-from-julian-date-live-1)

*Day of the month as jd runs from 2.452e+06 to 2.452e+06, every other input as you entered it (41 solutions). Marked: your jd = 2.452e+06 gives day = 1. Your inputs: jd = 2451545.0.*

```python
astronomy_ai999x.solve('calendar_date_from_julian_date', jd=2451545.0)
# -> year = 2000, month = 1, day = 1, hours = 12   (SOLVED)
```

```python
astronomy_ai999x.solve('calendar_date_from_julian_date', jd=2436116.31)
# -> year = 1957, month = 10, day = 4, hours = 19.44   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Calendar date from Julian date: jd = 2451545.0 | SOLVED | year = 2000, month = 1, day = 1, hours = 12 |
| Calendar date from Julian date: jd = 1000000 | INVALID_INPUT | Calendar date from a Julian date: the Julian date must lie in the Gregorian era (JD 2299160.5 to 5373484.5). |
| Calendar date from Julian date: | NEEDS_INPUT | calendar_date_from_julian_date needs jd (Julian date (days)). Example: solve('calendar_date_from_julian_date', jd=245154 |

<a id="sidereal_time"></a>

### Greenwich and local mean sidereal time: `sidereal_time`

The Greenwich mean sidereal time (IAU 1982 expression) at a Julian date in UT1, and the local mean sidereal time at an east longitude.

$$
\mathrm{GMST} = 24110.54841^\mathrm{s} + 8640184.812866^\mathrm{s}\,T + 0.093104^\mathrm{s}\,T^2 - 6.2\times10^{-6}\,{}^\mathrm{s}T^3 + \mathrm{UT1}
$$

| input | meaning | condition |
|---|---|---|
| `jd` | Julian date (UT1) |  |
| `lon` | east longitude (default 0) |  |

| output | meaning |
|---|---|
| `gmst_h` | Greenwich mean sidereal time (hours) |
| `lmst_h` | local mean sidereal time (hours) |

- **Method:** GMST = 24110.54841 $s + 8640184.812866$ $T + 0.093104$ $T^{2} - 6.2e-6$ $T^{3} + UT1$ seconds of the day (T from J2000, Julian centuries); LMST = GMST + longitude/15
- **Assumes:** $2299160.5  \leq  JD  \leq  5373484.5$; the date in UT1; east longitudes positive
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Greenwich mean sidereal time as jd runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/sidereal-time-live-1)

*Greenwich mean sidereal time as jd runs from 2.452e+06 to 2.452e+06, every other input as you entered it (31 solutions). Marked: your jd = 2.452e+06 gives gmst_h = 18.7. Your inputs: jd = 2451545.0.*

```python
astronomy_ai999x.solve('sidereal_time', jd=2451545.0)
# -> gmst_h = 18.6974, lmst_h = 18.6974   (SOLVED)
```

```python
astronomy_ai999x.solve('sidereal_time', jd=2451545.0, lon=0.2617993877991494)
# -> gmst_h = 18.6974, lmst_h = 19.6974   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Sidereal time: jd = 2451545.0 | SOLVED | gmst_h = 18.6974, lmst_h = 18.6974 |
| Sidereal time: jd = 100 | INVALID_INPUT | Greenwich and local mean sidereal time: the Julian date must lie between 2299160.5 and 5373484.5. |
| Sidereal time: | NEEDS_INPUT | sidereal_time needs jd (Julian date (UT1)). Example: solve('sidereal_time', jd=2451545.0). |

<a id="plasma_frequency"></a>

### Plasma frequency: `plasma_frequency`

The plasma (Langmuir) oscillation frequency of electrons (or protons) of number density n.

$$
\omega_p = \sqrt{\frac{n e^2}{\varepsilon_0 m}}
$$

| input | meaning | condition |
|---|---|---|
| `n` | number density (m^-3, cm^-3) |  |
| `particle` | particle species: electron or proton (default electron) |  |

| output | meaning |
|---|---|
| `omega_p` | plasma angular frequency |
| `f_p` | plasma frequency |

- **Method:** $omega_p = sqrt$(n e^2 / (eps0 m))
- **Assumes:** $n > 0$; cold, unmagnetized plasma; singly charged particles
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![plasma frequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/plasma-frequency-bars)

*plasma frequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind (electron) 2.008e+04, corona (electron) 2.839e+08, ionos. (electron) 8.979e+06, tail (electron) 4918, ISM (electron) 2839, tokamak (electron) 8.979e+10, given (electron) 8.979e+06, wind (proton) 468.5, corona (proton) 6.626e+06, ionos. (proton) 2.095e+05, tail (proton) 114.8, ISM (proton) 66.26, tokamak (proton) 2.095e+09. Your inputs: n = 1e12 m^-3.*

![Plasma frequency as n runs from 6e+11 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/plasma-frequency-live-1)

*Plasma frequency as n runs from 6e+11 to 1.4e+12 m^-3, every other input as you entered it (41 solutions). Marked: your n = 1e+12 m^-3 gives f_p = 8979 kHz. Your inputs: n = 1e12 m^-3.*

```python
astronomy_ai999x.solve('plasma_frequency', n=0.0003142077833705378)
# -> omega_p = 1 rad/s, f_p = 0.159155 Hz   (SOLVED)
```

```python
astronomy_ai999x.solve('plasma_frequency', n=2.3077338457826597, particle='proton')
# -> omega_p = 2 rad/s, f_p = 0.31831 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plasma frequency: n = 1e12 m^-3 | SOLVED | omega_p = 5.64146e+07 rad/s, f_p = 8.97866e+06 Hz |
| Plasma frequency: n = 1e12 m^-3, particle = neutron | INVALID_INPUT | particle: one of electron, proton; got 'neutron'. |
| Plasma frequency: | NEEDS_INPUT | plasma_frequency needs n (number density (m^-3, cm^-3)). Example: solve('plasma_frequency', n='0.0003142077833705378 1/m |

<a id="debye_length"></a>

### Debye length: `debye_length`

The electron Debye screening length of a plasma of electron temperature T and electron density n.

$$
\lambda_D = \sqrt{\frac{\varepsilon_0 k_B T_e}{n_e e^2}}
$$

| input | meaning | condition |
|---|---|---|
| `T` | electron temperature |  |
| `n` | electron density (m^-3, cm^-3) |  |

| output | meaning |
|---|---|
| `lambda_D` | Debye length |
| `lambda_D_nm` | Debye length in nanometres |

- **Method:** $lambda_D = sqrt$(eps0 k T / (n e^2))
- **Assumes:** T, $n > 0$; electron screening only
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Debye length for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/debye-length-bars)

*Debye length for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 9.759, corona 0.002182, ionos. 0.002182, tail 890.9, ISM 19.52, tokamak 6.901e-05, given 2.182e-05. Your inputs: T = 1e5 K, n = 1e18 m^-3.*

![Debye length as n runs from 6e+17 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/debye-length-live-1)

*Debye length as n runs from 6e+17 to 1.4e+18 m^-3, every other input as you entered it (41 solutions). Marked: your n = 1e+18 m^-3 gives λ_D = 21.82 um. Your inputs: T = 1e5 K, n = 1e18 m^-3.*

![Debye length as T runs from 6e+04 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/debye-length-live-2)

*Debye length as T runs from 6e+04 to 1.4e+05 K, every other input as you entered it (29 solutions). Marked: your T = 1e+05 K gives λ_D = 21.82 um. Your inputs: T = 1e5 K, n = 1e18 m^-3.*

```python
astronomy_ai999x.solve('debye_length', T=0.00020998524273112756, n=1.0)
# -> lambda_D = 1 m, lambda_D_nm = 1e+09   (SOLVED)
```

```python
astronomy_ai999x.solve('debye_length', T=0.0008399409709245102, n=1.0)
# -> lambda_D = 2 m, lambda_D_nm = 2e+09   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Debye length: T = 1e5 K, n = 1e18 m^-3 | SOLVED | lambda_D = 2.18226e-05 m, lambda_D_nm = 21822.6 |
| Debye length: T = 1e5 K, n = 0 m^-3 | INVALID_INPUT | Debye length: the temperature and the density must be a positive number. |
| Debye length: T = 1e5 K | NEEDS_INPUT | debye_length needs n (electron density (m^-3, cm^-3)). Example: solve('debye_length', T='0.00020998524273112756 K', n='1 |

<a id="debye_number"></a>

### Number of particles in a Debye sphere: `debye_number`

The number of electrons in a sphere of one Debye length radius (N_D >> 1 means the gas behaves as a plasma).

$$
N_D = \frac{4\pi}{3}n_e\lambda_D^3
$$

| input | meaning | condition |
|---|---|---|
| `T` | electron temperature |  |
| `n` | electron density |  |

| output | meaning |
|---|---|
| `N_D` | particles per Debye sphere |
| `lambda_D` | Debye length |

- **Method:** N_D = (4 pi / 3) n $lambda_D^{3}$
- **Assumes:** T, $n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![particles per Debye sphere for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/debye-number-bars)

*particles per Debye sphere for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 1.947e+10, corona 4.353e+07, ionos. 4.353e+04, tail 8.886e+14, ISM 3.115e+09, tokamak 1.377e+08, given 4.353e+07. Your inputs: T = 1e6 K, n = 1e15 m^-3.*

![Particles per Debye sphere as T runs from 6e+05 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/debye-number-live-1)

*Particles per Debye sphere as T runs from 6e+05 to 1.4e+06 K, every other input as you entered it (41 solutions). Marked: your T = 1e+06 K gives N_D = 4.353e+07. Your inputs: T = 1e6 K, n = 1e15 m^-3.*

![Particles per Debye sphere as n runs from 6e+14 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/debye-number-live-2)

*Particles per Debye sphere as n runs from 6e+14 to 1.4e+15 m^-3, every other input as you entered it (29 solutions). Marked: your n = 1e+15 m^-3 gives N_D = 4.353e+07. Your inputs: T = 1e6 K, n = 1e15 m^-3.*

```python
astronomy_ai999x.solve('debye_number', T=0.00020998524273112756, n=1.0)
# -> N_D = 4.18879, lambda_D = 1 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Debye number: T = 1e6 K, n = 1e15 m^-3 | SOLVED | N_D = 4.35317e+07, lambda_D = 0.00218226 m |
| Debye number: T = -1 K, n = 1e15 m^-3 | INVALID_INPUT | Number of particles in a Debye sphere: the temperature and the density must be a positive number. |
| Debye number: n = 1e15 m^-3 | NEEDS_INPUT | debye_number needs T (electron temperature). Example: solve('debye_number', T='0.00020998524273112756 K', n='1 1/m^3'). |

<a id="alfven_speed"></a>

### Alfven speed: `alfven_speed`

The Alfven speed of a fully ionized hydrogen plasma (n protons and n electrons per unit volume) in a magnetic field B.

$$
v_A = \frac{B}{\sqrt{\mu_0\,n\,(m_p + m_e)}}
$$

| input | meaning | condition |
|---|---|---|
| `B` | magnetic field strength (T, nT, gauss) |  |
| `n` | proton number density |  |

| output | meaning |
|---|---|
| `v_A` | Alfven speed |
| `v_A_km_s` | Alfven speed in km/s |

- **Method:** $v_A = B / sqrt$(mu0 n (m_p + m_e))
- **Assumes:** B, $n > 0$; mass density n (m_p + m_e); non-relativistic (v_A << c)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Alfvén speed for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/alfven-speed-bars)

*Alfvén speed for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 4.876e+04, corona 6.896e+05, ionos. 1.09e+06, tail 3.981e+05, ISM 3.448e+04, tokamak 1.09e+07, given 4.876e+04. Your inputs: B = 5 nT, n = 5 cm^-3.*

![Alfven speed as B runs from 3 to 7 nT, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/alfven-speed-live-1)

*Alfven speed as B runs from 3 to 7 nT, every other input as you entered it (41 solutions). Marked: your B = 5 nT gives v_A = 4.876e+04. Your inputs: B = 5 nT, n = 5 cm^-3.*

![Alfven speed as n runs from 3 to 7 cm^-3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/alfven-speed-live-2)

*Alfven speed as n runs from 3 to 7 cm^-3, every other input as you entered it (29 solutions). Marked: your n = 5 cm^-3 gives v_A = 4.876e+04. Your inputs: B = 5 nT, n = 5 cm^-3.*

```python
astronomy_ai999x.solve('alfven_speed', B=4.5858733307403914e-17, n=1.0)
# -> v_A = 1 m/s, v_A_km_s = 0.001   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Alfven speed: B = 5 nT, n = 5 cm^-3 | SOLVED | v_A = 48759.9 m/s, v_A_km_s = 48.7599 |
| Alfven speed: B = 0 nT, n = 5 cm^-3 | INVALID_INPUT | Alfven speed: the field and the density must be a positive number. |
| Alfven speed: B = 5 nT | NEEDS_INPUT | alfven_speed needs n (proton number density). Example: solve('alfven_speed', B='4.5858733307403914e-17 T', n='1 1/m^3'). |

<a id="gyrofrequency"></a>

### Gyrofrequency (cyclotron frequency): `gyrofrequency`

The angular and ordinary gyrofrequency and the gyration period of an electron or proton in a magnetic field B.

$$
\omega_c = \frac{|q|B}{m}
$$

| input | meaning | condition |
|---|---|---|
| `B` | magnetic field strength (T, nT, gauss) |  |
| `particle` | particle species: electron or proton (default electron) |  |

| output | meaning |
|---|---|
| `omega_c` | angular gyrofrequency |
| `f_c` | gyrofrequency |
| `T_c` | gyration period |

- **Method:** omega_c = |q| B / m
- **Assumes:** $B > 0$; non-relativistic particle
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![gyrofrequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/gyrofrequency-bars)

*gyrofrequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind (electron) 140, corona (electron) 2.799e+07, ionos. (electron) 1.4e+06, tail (electron) 279.9, ISM (electron) 14, tokamak (electron) 1.4e+11, given (electron) 0.1592, wind (proton) 0.07623, corona (proton) 1.525e+04, ionos. (proton) 762.3, tail (proton) 0.1525, ISM (proton) 0.007623, tokamak (proton) 7.623e+07. Your inputs: B = 5.685630111305193e-12 T.*

![Gyrofrequency as B runs from 3](https://www.ai999x.com/?ax_fig=astronomy-ai999x/gyrofrequency-live-1)

*Gyrofrequency as B runs from 3.411e-12 to 7.96e-12 T, every other input as you entered it (41 solutions). Marked: your B = 5.686e-12 T gives f_c = 0.1592. Your inputs: B = 5.685630111305193e-12 T.*

```python
astronomy_ai999x.solve('gyrofrequency', B=5.685630111305193e-12)
# -> omega_c = 1 rad/s, f_c = 0.159155 Hz, T_c = 6.28319 s   (SOLVED)
```

```python
astronomy_ai999x.solve('gyrofrequency', B=1.0439684928958963e-08, particle='proton')
# -> omega_c = 1 rad/s, f_c = 0.159155 Hz, T_c = 6.28319 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Gyrofrequency: B = 1 T, particle = proton | SOLVED | omega_c = 9.57883e+07 rad/s, f_c = 1.52452e+07 Hz, T_c = 6.55945e-08 s |
| Gyrofrequency: B = -1 T | INVALID_INPUT | Gyrofrequency (cyclotron frequency): the field must be a positive number. |
| Gyrofrequency: particle = proton | NEEDS_INPUT | gyrofrequency needs B (magnetic field strength (T, nT, gauss)). Example: solve('gyrofrequency', B='5.685630111305193e-12 |

<a id="gyroradius"></a>

### Gyroradius (Larmor radius): `gyroradius`

The radius of the circular motion of an electron or proton moving with perpendicular speed v_perp in a magnetic field B.

$$
r_L = \frac{m v_\perp}{|q|B}
$$

| input | meaning | condition |
|---|---|---|
| `B` | magnetic field strength |  |
| `v_perp` | speed perpendicular to the field |  |
| `particle` | particle species: electron or proton (default electron) |  |

| output | meaning |
|---|---|
| `r_L` | gyroradius |
| `r_L_km` | gyroradius in kilometres |
| `r_L_nm` | gyroradius in nanometres |

- **Method:** $r_L = m$ v_perp / (|q| B)
- **Assumes:** B, v_perp > 0; v_perp < 0.1 c (non-relativistic)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![gyroradius for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/gyroradius-bars)

*gyroradius for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 6.264, corona 3.132e-05, ionos. 0.0006264, tail 3.132, ISM 62.64, tokamak 6.264e-09, given 3. Your inputs: B = 1.0439684928958963e-08 T, v_perp = 3 m/s, particle = proton.*

![Gyroradius as B runs from 6](https://www.ai999x.com/?ax_fig=astronomy-ai999x/gyroradius-live-1)

*Gyroradius as B runs from 6.264e-09 to 1.462e-08 T, every other input as you entered it (41 solutions). Marked: your B = 1.044e-08 T gives r_L = 3. Your inputs: B = 1.0439684928958963e-08 T, v_perp = 3 m/s, particle = proton.*

![Gyroradius for every choice of particle, every other input as you entered it (highlighted: your choice)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/gyroradius-live-2)

*Gyroradius for every choice of particle, every other input as you entered it (highlighted: your choice). electron: 0.001634; proton: 3. Your inputs: B = 1.0439684928958963e-08 T, v_perp = 3 m/s, particle = proton.*

![Gyroradius as v_perp runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/gyroradius-live-3)

*Gyroradius as v_perp runs from 1.8 to 4.2 m/s, every other input as you entered it (29 solutions). Marked: your v_perp = 3 m/s gives r_L = 3. Your inputs: B = 1.0439684928958963e-08 T, v_perp = 3 m/s, particle = proton.*

```python
astronomy_ai999x.solve('gyroradius', B=1.0439684928958963e-08, v_perp=3.0, particle='proton')
# -> r_L = 3 m, r_L_km = 0.003, r_L_nm = 3e+09   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Gyroradius: B = 5 nT, v_perp = 400 km/s, particle = proton | SOLVED | r_L = 835175 m, r_L_km = 835.175, r_L_nm = 8.35175e+14 |
| Gyroradius: B = 5 nT, v_perp = 2e8 m/s | INVALID_INPUT | Gyroradius (Larmor radius): the speed must be well below c (v < 0.1 c) for the non-relativistic gyroradius. |
| Gyroradius: B = 5 nT | NEEDS_INPUT | gyroradius needs v_perp (speed perpendicular to the field). Example: solve('gyroradius', B='1.0439684928958963e-08 T', v |

<a id="thermal_speed"></a>

### Thermal speed of plasma particles: `thermal_speed`

The thermal speed of electrons or protons of temperature T under a stated convention: most probable sqrt(2kT/m), root-mean-square sqrt(3kT/m) or mean sqrt(8kT/(pi m)).

$$
v_\mathrm{mp} = \sqrt{\frac{2kT}{m}},\ v_\mathrm{rms} = \sqrt{\frac{3kT}{m}},\ \bar v = \sqrt{\frac{8kT}{\pi m}}
$$

| input | meaning | condition |
|---|---|---|
| `T` | temperature |  |
| `convention` | which thermal speed: most_probable, rms or mean |  |
| `particle` | particle species: electron or proton (default electron) |  |

| output | meaning |
|---|---|
| `v_th` | thermal speed |

- **Method:** $v = sqrt$(c k T / m) with $c = 2$ (most probable), 3 (rms), $8/pi$ (mean)
- **Assumes:** $T > 0$; Maxwellian distribution in three dimensions; the convention must be stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Thermal speed as T runs from 3](https://www.ai999x.com/?ax_fig=astronomy-ai999x/thermal-speed-live-1)

*Thermal speed as T runs from 3.299e-08 to 1.155e-07 K, every other input as you entered it (41 solutions). Marked: your T = 6.598e-08 K gives v_th = 1.732. Your inputs: T = 6.597899765907193e-08 K, convention = rms.*

![Thermal speed for every choice of convention, every other input as you entered it (highlighted: your choice)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/thermal-speed-live-2)

*Thermal speed for every choice of convention, every other input as you entered it (highlighted: your choice). most_probable: 1.414; rms: 1.732; mean: 1.596. Your inputs: T = 6.597899765907193e-08 K, convention = rms.*

```python
astronomy_ai999x.solve('thermal_speed', T=6.597899765907193e-08, convention='rms')
# -> v_th = 1.73205 m/s   (SOLVED)
```

```python
astronomy_ai999x.solve('thermal_speed', T=0.00012114751294137755, convention='most_probable', particle='proton')
# -> v_th = 1.41421 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Thermal speed: T = 1e6 K, convention = rms, particle = proton | SOLVED | v_th = 157363 m/s |
| Thermal speed: T = 1e6 K, convention = fastest | INVALID_INPUT | convention: one of most_probable, rms, mean; got 'fastest'. |
| Thermal speed: T = 1e6 K | NEEDS_INPUT | thermal_speed needs convention (which thermal speed: most_probable, rms or mean). Example: solve('thermal_speed', T='6.5 |

<a id="plasma_beta"></a>

### Plasma beta: `plasma_beta`

The ratio of the thermal pressure n k T of one particle species to the magnetic pressure B^2/(2 mu0).

$$
\beta = \frac{n k_B T}{B^2/(2\mu_0)}
$$

| input | meaning | condition |
|---|---|---|
| `T` | temperature of the species |  |
| `n` | number density of the species |  |
| `B` | magnetic field strength |  |

| output | meaning |
|---|---|
| `beta` | plasma beta |
| `p_thermal` | thermal pressure n k T |
| `p_magnetic` | magnetic pressure |

- **Method:** $\beta = n$ k T / (B^2 / (2 mu0))
- **Assumes:** T, n, $B > 0$; one species (add species' pressures for a total beta)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![plasma beta for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/plasma-beta-bars)

*plasma beta for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 0.694, corona 0.0347, ionos. 1.388e-05, tail 5.205, ISM 0.111, tokamak 0.01388, given 0.694. Your inputs: T = 1e5 K, n = 5 cm^-3, B = 5 nT.*

![Plasma beta as B runs from 3 to 7 nT, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/plasma-beta-live-1)

*Plasma beta as B runs from 3 to 7 nT, every other input as you entered it (41 solutions). Marked: your B = 5 nT gives β = 0.694. Your inputs: T = 1e5 K, n = 5 cm^-3, B = 5 nT.*

![Plasma beta as T runs from 6e+04 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/plasma-beta-live-2)

*Plasma beta as T runs from 6e+04 to 1.4e+05 K, every other input as you entered it (29 solutions). Marked: your T = 1e+05 K gives β = 0.694. Your inputs: T = 1e5 K, n = 5 cm^-3, B = 5 nT.*

```python
astronomy_ai999x.solve('plasma_beta', T=7.24297051603992e+22, n=1.0, B=0.0015853309189377467)
# -> beta = 1, p_thermal = 1 Pa, p_magnetic = 1 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plasma beta: T = 1e5 K, n = 5 cm^-3, B = 5 nT | SOLVED | beta = 0.69399, p_thermal = 6.90325e-12 Pa, p_magnetic = 9.94718e-12 Pa |
| Plasma beta: T = 1e5 K, n = 5 cm^-3, B = 0 nT | INVALID_INPUT | Plasma beta: the temperature, the density and the field must be a positive number. |
| Plasma beta: T = 1e5 K, n = 5 cm^-3 | NEEDS_INPUT | plasma_beta needs B (magnetic field strength). Example: solve('plasma_beta', T='7.24297051603992e+22 K', n='1 1/m^3', B= |

<a id="magnetic_pressure"></a>

### Magnetic pressure: `magnetic_pressure`

The magnetic pressure (equal to the magnetic energy density) of a field B in vacuum or a plasma.

$$
p_B = \frac{B^2}{2\mu_0}
$$

| input | meaning | condition |
|---|---|---|
| `B` | magnetic field strength (T, nT, gauss) |  |

| output | meaning |
|---|---|
| `p_B` | magnetic pressure |
| `energy_density` | magnetic energy density $(J/m^{3})$ |
| `p_B_nPa` | magnetic pressure in nanopascals |

- **Method:** p_B = B^2 / (2 mu0)
- **Assumes:** $B > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![magnetic pressure for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/magnetic-pressure-bars)

*magnetic pressure for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 9.947e-12, corona 0.3979, ionos. 0.0009947, tail 3.979e-11, ISM 9.947e-14, tokamak 9.947e+06, given 9.947e-12. Your inputs: B = 5 nT.*

![Magnetic pressure and magnetic energy density as B runs from 3 to 7 nT, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/magnetic-pressure-live-1)

*Magnetic pressure and magnetic energy density as B runs from 3 to 7 nT, every other input as you entered it (41 solutions). Marked: your B = 5 nT gives p_B = 9.947e-12. Your inputs: B = 5 nT.*

![Every answer of one unit for your inputs: p_B (magnetic pressure) = 9](https://www.ai999x.com/?ax_fig=astronomy-ai999x/magnetic-pressure-live-2)

*Every answer of one unit for your inputs: p_B (magnetic pressure) = 9.947e-12; energy_density (magnetic energy density (J/m^3)) = 9.947e-12. Your inputs: B = 5 nT.*

```python
astronomy_ai999x.solve('magnetic_pressure', B=0.0015853309189377467)
# -> p_B = 1 Pa, energy_density = 1 Pa, p_B_nPa = 1e+09   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Magnetic pressure: B = 1 T | SOLVED | p_B = 397887 Pa, energy_density = 397887 Pa, p_B_nPa = 3.97887e+14 |
| Magnetic pressure: B = 0 T | INVALID_INPUT | Magnetic pressure: the field must be a positive number. |
| Magnetic pressure: | NEEDS_INPUT | magnetic_pressure needs B (magnetic field strength (T, nT, gauss)). Example: solve('magnetic_pressure', B='0.00158533091 |

<a id="upper_hybrid_frequency"></a>

### Upper hybrid frequency: `upper_hybrid_frequency`

The upper hybrid frequency of a magnetized electron plasma: sqrt(omega_pe^2 + omega_ce^2).

$$
\omega_{uh} = \sqrt{\omega_{pe}^2 + \omega_{ce}^2}
$$

| input | meaning | condition |
|---|---|---|
| `B` | magnetic field strength |  |
| `n` | electron density |  |

| output | meaning |
|---|---|
| `omega_uh` | upper hybrid angular frequency |
| `f_uh` | upper hybrid frequency |

- **Method:** $\omega_{uh} = \sqrt{\omega_{pe}^{2} + \omega_{ce}^{2}}$
- **Assumes:** B, $n > 0$; cold plasma, waves perpendicular to B
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![frequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/upper-hybrid-frequency-bars)

*frequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 2.008e+04, corona 2.853e+08, ionos. 9.087e+06, tail 4926, ISM 2839, tokamak 1.663e+11, given 9.087e+06. Your inputs: B = 50000 nT, n = 1e12 m^-3.*

![Upper hybrid frequency as n runs from 6e+11 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/upper-hybrid-frequency-live-1)

*Upper hybrid frequency as n runs from 6e+11 to 1.4e+12 m^-3, every other input as you entered it (41 solutions). Marked: your n = 1e+12 m^-3 gives f_uh = 9087 kHz. Your inputs: B = 50000 nT, n = 1e12 m^-3.*

![Upper hybrid frequency as B runs from 3e+04 to 7e+04 nT, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/upper-hybrid-frequency-live-2)

*Upper hybrid frequency as B runs from 3e+04 to 7e+04 nT, every other input as you entered it (29 solutions). Marked: your B = 5e+04 nT gives f_uh = 9087 kHz. Your inputs: B = 50000 nT, n = 1e12 m^-3.*

```python
astronomy_ai999x.solve('upper_hybrid_frequency', B=1.7056890333915583e-11, n=0.005027324533928605)
# -> omega_uh = 5 rad/s, f_uh = 0.795775 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Upper hybrid frequency: B = 1 T, n = 1e18 m^-3 | SOLVED | omega_uh = 1.84708e+11 rad/s, f_uh = 2.93972e+10 Hz |
| Upper hybrid frequency: B = 1 T, n = -1e18 m^-3 | INVALID_INPUT | Upper hybrid frequency: the field and the density must be a positive number. |
| Upper hybrid frequency: B = 1 T | NEEDS_INPUT | upper_hybrid_frequency needs n (electron density). Example: solve('upper_hybrid_frequency', B='1.7056890333915583e-11 T' |

<a id="lower_hybrid_frequency"></a>

### Lower hybrid frequency: `lower_hybrid_frequency`

The lower hybrid frequency of a magnetized electron-proton plasma.

$$
\frac{1}{\omega_{lh}^2} = \frac{1}{\omega_{ci}\,\omega_{ce}} + \frac{1}{\omega_{pi}^2}
$$

| input | meaning | condition |
|---|---|---|
| `B` | magnetic field strength |  |
| `n` | density (electrons = protons) |  |

| output | meaning |
|---|---|
| `omega_lh` | lower hybrid angular frequency |
| `f_lh` | lower hybrid frequency |

- **Method:** 1/omega_lh^2 = 1/(omega_ci omega_ce) + 1/omega_pi^2
- **Assumes:** B, $n > 0$; cold quasi-neutral electron-proton plasma
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![frequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/lower-hybrid-frequency-bars)

*frequency for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind 3.266, corona 6.501e+05, ionos. 3.227e+04, tail 6.522, ISM 0.3266, tokamak 1.764e+09, given 3.227e+04. Your inputs: B = 50000 nT, n = 1e12 m^-3.*

![Lower hybrid frequency as B runs from 3e+04 to 7e+04 nT, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=astronomy-ai999x/lower-hybrid-frequency-live-1)

*Lower hybrid frequency as B runs from 3e+04 to 7e+04 nT, every other input as you entered it (41 solutions). Marked: your B = 5e+04 nT gives f_lh = 32.27 kHz. Your inputs: B = 50000 nT, n = 1e12 m^-3.*

![Lower hybrid frequency as n runs from 6e+11 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/lower-hybrid-frequency-live-2)

*Lower hybrid frequency as n runs from 6e+11 to 1.4e+12 m^-3, every other input as you entered it (29 solutions). Marked: your n = 1e+12 m^-3 gives f_lh = 32.27 kHz. Your inputs: B = 50000 nT, n = 1e12 m^-3.*

```python
astronomy_ai999x.solve('lower_hybrid_frequency', B=1.0, n=9.719853830824368e+18)
# -> omega_lh = 2.90236e+09 rad/s, f_lh = 4.61926e+08 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Lower hybrid frequency: B = 1 T, n = 1e18 m^-3 | SOLVED | omega_lh = 1.25364e+09 rad/s, f_lh = 1.99523e+08 Hz |
| Lower hybrid frequency: B = 1 T, n = 0 m^-3 | INVALID_INPUT | Lower hybrid frequency: the field and the density must be a positive number. |
| Lower hybrid frequency: B = 1 T | NEEDS_INPUT | lower_hybrid_frequency needs n (density (electrons = protons)). Example: solve('lower_hybrid_frequency', B='1 T', n='9.7 |

<a id="inertial_length"></a>

### Inertial length (skin depth): `inertial_length`

The electron (or proton) inertial length c / omega_p, the collisionless skin depth of a plasma.

$$
d = \frac{c}{\omega_p}
$$

| input | meaning | condition |
|---|---|---|
| `n` | number density |  |
| `particle` | particle species: electron or proton (default electron) |  |

| output | meaning |
|---|---|
| `d` | inertial length |
| `d_km` | inertial length in kilometres |

- **Method:** $d = c / omega_p = c$ sqrt(eps0 m / (n e^2))
- **Assumes:** $n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![inertial length for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0](https://www.ai999x.com/?ax_fig=astronomy-ai999x/inertial-length-bars)

*inertial length for six plasmas (stated density, temperature and field: solar wind at 1 au 5e6 m⁻³, 1e5 K, 5 nT; solar corona 1e15 m⁻³, 1e6 K, 1e-3 T; ionosphere F layer 1e12 m⁻³, 1000 K, 5e-5 T; Earth's magnetotail plasma sheet 3e5 m⁻³, 5e7 K, 10 nT; warm interstellar medium 1e5 m⁻³, 8000 K, 0.5 nT; a tokamak 1e20 m⁻³, 1e8 K, 5 T; only the quantities this calculation takes are used), each a solve of the model; the seventh bar is the given inputs. Marked: wind (electron) 2377, corona (electron) 0.168, ionos. (electron) 5.314, tail (electron) 9702, ISM (electron) 1.68e+04, tokamak (electron) 0.0005314, given (electron) 0.005314, wind (proton) 1.018e+05, corona (proton) 7.201, ionos. (proton) 227.7, tail (proton) 4.157e+05, ISM (proton) 7.201e+05, tokamak (proton) 0.02277. Your inputs: n = 1e18 m^-3.*

![Inertial length as n runs from 6e+17 to 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/inertial-length-live-1)

*Inertial length as n runs from 6e+17 to 1.4e+18 m^-3, every other input as you entered it (41 solutions). Marked: your n = 1e+18 m^-3 gives d = 5.314 mm. Your inputs: n = 1e18 m^-3.*

```python
astronomy_ai999x.solve('inertial_length', n=28239587250368.695)
# -> d = 1 m, d_km = 0.001   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Inertial length: n = 5 cm^-3, particle = proton | SOLVED | d = 101835 m, d_km = 101.835 |
| Inertial length: n = 0 cm^-3 | INVALID_INPUT | Inertial length (skin depth): the density must be a positive number. |
| Inertial length: | NEEDS_INPUT | inertial_length needs n (number density). Example: solve('inertial_length', n='28239587250368.695 1/m^3'). |

<a id="einstein_radius"></a>

### Einstein radius of a point-mass lens: `einstein_radius`

The angular Einstein radius of a point mass M at distance D_l lensing a source at distance D_s (static space: D_ls = D_s - D_l), and the corresponding radius in the lens plane.

$$
\theta_E = \sqrt{\frac{4GM}{c^2}\frac{D_{ls}}{D_l D_s}}
$$

| input | meaning | condition |
|---|---|---|
| `M` | lens mass (Msun, kg) |  |
| `D_l` | distance to the lens (pc, kpc) |  |
| `D_s` | distance to the source |  |

| output | meaning |
|---|---|
| `theta_E` | Einstein angle |
| `theta_E_arcsec` | Einstein angle in arcseconds |
| `R_E` | Einstein radius in the lens plane |
| `R_E_au` | Einstein radius in au |

- **Method:** $theta_E = sqrt$( (4 G M / c^2) (D_s - D_l) / (D_l D_s) )
- **Assumes:** $D_s > D_l > 0$; point mass, weak field; non-cosmological distances $(D_ls = D_s - D_l)$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Einstein angle in arcseconds as D_s runs from 1](https://www.ai999x.com/?ax_fig=astronomy-ai999x/einstein-radius-live-1)

*Einstein angle in arcseconds as D_s runs from 1.481e+20 to 3.456e+20 m, every other input as you entered it (41 solutions). Marked: your D_s = 2.469e+20 m gives θ_E_arcsec = 0.0007134. Your inputs: M = 9.94205e+29 kg, D_l = 1.23427e+20 m, D_s = 2.46854e+20 m.*

![Einstein angle in arcseconds as D_l runs from 7](https://www.ai999x.com/?ax_fig=astronomy-ai999x/einstein-radius-live-2)

*Einstein angle in arcseconds as D_l runs from 7.406e+19 to 1.728e+20 m, every other input as you entered it (29 solutions). Marked: your D_l = 1.234e+20 m gives θ_E_arcsec = 0.0007134. Your inputs: M = 9.94205e+29 kg, D_l = 1.23427e+20 m, D_s = 2.46854e+20 m.*

```python
astronomy_ai999x.solve('einstein_radius', M=3.3664773037502725e+26, D_l=1.0, D_s=2.0)
# -> theta_E = 0.707107 rad, theta_E_arcsec = 145851, R_E = 0.707107 m, R_E_au = 4.72672e-12   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Einstein radius: M = 0.5 Msun, D_l = 4 kpc, D_s = 8 kpc | SOLVED | theta_E = 3.45884e-09 rad, theta_E_arcsec = 0.000713436, R_E = 4.26914e+11 m, R_E_au = 2.85374 |
| Einstein radius: M = 0.5 Msun, D_l = 8 kpc, D_s = 4 kpc | INVALID_INPUT | Einstein radius of a point-mass lens: the source must lie behind the lens (D_s > D_l). |
| Einstein radius: M = 0.5 Msun, D_l = 4 kpc | NEEDS_INPUT | einstein_radius needs D_s (distance to the source). Example: solve('einstein_radius', M='3.3664773037502725e+26 kg', D_l |

<a id="distance_from_flux"></a>

### Distance from luminosity and flux (inverse-square law), `distance_from_flux`

The distance of a source of known luminosity L (a standard candle) from the bolometric flux F measured at the observer.

$$
d = \sqrt{\frac{L}{4\pi F}}
$$

| input | meaning | condition |
|---|---|---|
| `L` | luminosity (W or Lsun) |  |
| `F` | measured flux $(W/m^{2})$ |  |

| output | meaning |
|---|---|
| `d` | distance |
| `d_pc` | distance in parsecs |
| `d_ly` | distance in light-years |

- **Method:** F = L / (4 pi d^2): $d = sqrt$(L / (4 pi F))
- **Assumes:** L, $F > 0$; isotropic emission, no extinction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Distance in parsecs as F runs from 816](https://www.ai999x.com/?ax_fig=astronomy-ai999x/distance-from-flux-live-1)

*Distance in parsecs as F runs from 816.6 to 1905 W/m^2, every other input as you entered it (41 solutions). Marked: your F = 1361 W/m^2 gives d_pc = 4.848e-06. Your inputs: L = 3.828e+26 W, F = 1361 W/m^2.*

![Distance in parsecs as L runs from 2](https://www.ai999x.com/?ax_fig=astronomy-ai999x/distance-from-flux-live-2)

*Distance in parsecs as L runs from 2.297e+26 to 5.359e+26 W, every other input as you entered it (29 solutions). Marked: your L = 3.828e+26 W gives d_pc = 4.848e-06. Your inputs: L = 3.828e+26 W, F = 1361 W/m^2.*

```python
astronomy_ai999x.solve('distance_from_flux', L=12.566370614359172, F=1.0)
# -> d = 1 m, d_pc = 3.24078e-17, d_ly = 1.057e-16   (SOLVED)
```

```python
astronomy_ai999x.solve('distance_from_flux', L=3.0128e+28, F=2.518021002633401e-10)
# -> d = 3.08568e+18 m, d_pc = 100, d_ly = 326.156   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Distance from flux: L = 1 Lsun, F = 1361 W/m^2 | SOLVED | d = 1.49607e+11 m, d_pc = 4.84843e-06, d_ly = 1.58135e-05 |
| Distance from flux: L = 1 Lsun, F = 0 W/m^2 | INVALID_INPUT | Distance from luminosity and flux (inverse-square law): the luminosity and the flux must be a positive number. |
| Distance from flux: L = 1 Lsun | NEEDS_INPUT | distance_from_flux needs F (measured flux (W/m^2)). Example: solve('distance_from_flux', L='12.566370614359172 W', F='1  |

<a id="critical_density"></a>

### Hubble parameter and critical density: `critical_density`

The Hubble parameter H(z) and the critical density 3H^2/(8 pi G) of a flat Lambda-CDM universe at redshift z (z = 0 gives today's values), with the equivalent number of hydrogen atoms per cubic metre.

$$
H(z) = H_0\sqrt{\Omega_m(1+z)^3 + \Omega_\Lambda},\quad \rho_c = \frac{3H^2}{8\pi G}
$$

| input | meaning | condition |
|---|---|---|
| `H0` | Hubble constant in km/s/Mpc |  |
| `Om` | matter density parameter Omega_m |  |
| `z` | redshift (default 0) |  |

| output | meaning |
|---|---|
| `H_z` | Hubble parameter H(z) in km/s/Mpc |
| `rho_c` | critical density |
| `n_H` | critical density in hydrogen-atom (proton) masses per cubic metre |

- **Method:** H(z) = H0 sqrt(Om (1+z)^3 + 1 - Om); $rho_c = 3$ H^2 / (8 pi G)
- **Assumes:** $1  \leq  H_{0}  \leq  1000$; $0.01  \leq  Omega_m  \leq  1$; $0  \leq  z  \leq  1100$; flat, no radiation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Critical density as H0 runs from 17](https://www.ai999x.com/?ax_fig=astronomy-ai999x/critical-density-live-1)

*Critical density as H0 runs from 17.5 to 122.5, every other input as you entered it (41 solutions). Marked: your H0 = 70 gives ρ_c = 9.204e-27. Your inputs: H0 = 70, Om = 0.3.*

```python
astronomy_ai999x.solve('critical_density', H0=70.0, Om=0.3)
# -> H_z = 70, rho_c = 9.20387e-27 kg/m^3, n_H = 5.50266 1/m^3   (SOLVED)
```

```python
astronomy_ai999x.solve('critical_density', H0=70.0, Om=1.0, z=3.0)
# -> H_z = 560, rho_c = 5.89048e-25 kg/m^3, n_H = 352.17 1/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Critical density: H0 = 70 km/s/Mpc, Om = 0.3 | SOLVED | H_z = 70, rho_c = 9.20387e-27 kg/m^3, n_H = 5.50266 1/m^3 |
| Critical density: H0 = 70, Om = 0.3, z = -2 | INVALID_INPUT | Hubble parameter and critical density: the redshift must lie between 0 and 1100. |
| Critical density: H0 = 70 | NEEDS_INPUT | critical_density needs Om (matter density parameter Omega_m). Example: solve('critical_density', H0=70, Om=0.3). |
