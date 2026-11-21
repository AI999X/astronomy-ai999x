# Astronomy (astronomy-ai999x)

[![tests](https://github.com/AI999X/astronomy-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/astronomy-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/astronomy-ai999x)](https://pypi.org/project/astronomy-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/astronomy-ai999x)

Astronomy covers celestial mechanics and orbits, stellar properties, blackbody radiation, magnitudes and distances, cosmology basics, the Sun and space environment and plasma parameters: 67 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Astronomy desktop app: Distance from parallax, with its worked example and live scene](https://www.ai999x.com/?ax_shot=astronomy-ai999x/hero.webp)](https://www.ai999x.com/models/astronomy-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-astronomy-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-astronomy-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `astronomy` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/astronomy-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install astronomy-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Astronomy desktop app: every scene and chart is drawn from the computed answer.

[![Astronomy desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=astronomy-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=astronomy-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=astronomy-ai999x/clip.mp4)*

| | |
|---|---|
| [![Distance from parallax: the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-1.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-parallax_distance)<br>Distance from parallax | [![Hohmann transfer between circular orbits: the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-2.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-hohmann_transfer)<br>Hohmann transfer between circular orbits |
| [![Lookback time to a redshift: the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-3.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-lookback_time)<br>Lookback time to a redshift | [![Luminosity of a star (Stefan-Boltzmann): the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-4.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-stellar_luminosity)<br>Luminosity of a star (Stefan-Boltzmann) |
| [![Altitude and azimuth from hour angle and declination: the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-5.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-altaz_from_hadec)<br>Altitude and azimuth from hour angle and declination | [![Schwarzschild radius: the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-6.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-schwarzschild_radius)<br>Schwarzschild radius |
| [![Radial-velocity semi-amplitude of a star: the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-7.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-radial_velocity_semi_amplitude)<br>Radial-velocity semi-amplitude of a star | [![Equilibrium temperature of a planet: the live scene in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-8.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-planet_equilibrium_temperature)<br>Equilibrium temperature of a planet |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Distance from parallax: a chart in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/chart-1.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-parallax_distance)<br>Distance from parallax | [![Hohmann transfer between circular orbits: a chart in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/chart-2.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-hohmann_transfer)<br>Hohmann transfer between circular orbits | [![Lookback time to a redshift: a chart in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/chart-3.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-lookback_time)<br>Lookback time to a redshift | [![Luminosity of a star (Stefan-Boltzmann): a chart in the Astronomy desktop app](https://www.ai999x.com/?ax_shot=astronomy-ai999x/chart-4.webp)](https://www.ai999x.com/models/astronomy-ai999x/#cap-stellar_luminosity)<br>Luminosity of a star (Stefan-Boltzmann) |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import astronomy_ai999x as astronomy

a = astronomy.ask("Schwarzschild radius: M = 1 Msun")
print(a.outcome, a.values)
# SOLVED {'rs': 2953.2500761, 'rs_km': 2.9532500761, 'M_Msun': 1.0}

a = astronomy.ask("Absolute magnitude: m = 8, d = 100 pc, A = 1")
print(a.outcome, a.values)
# SOLVED {'M': 2.0, 'mu': 5.0, 'd_pc': 100.0}

a = astronomy.ask(
    "Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup, P "
    "= 11.86 yr"
)
print(a.outcome, a.values)
# SOLVED {'K': 12.4597741631, 'a': 778241756142.0, 'a_au': 5.20222482112}

a = astronomy.ask("Airmass: zen = 60 deg")
print(a.outcome, a.values)
# SOLVED {'X': 2.0, 'alt_deg': 30.0}

a = astronomy.ask("Plane change: v1 = 3 km/s, v2 = 4 km/s, di = 90 deg")
print(a.outcome, a.values)
# SOLVED {'dv': 5000.0}

a = astronomy.ask(
    "Equilibrium temperature: T_star = 1000 K, R_star = 2 m, a = 4 m"
)
print(a.outcome, a.values)
# SOLVED {'T_eq': 500.0, 'S': 14175.936048}

a = astronomy.ask("Alfven speed: B = 5 nT, n = 5 cm^-3")
print(a.outcome, a.values)
# SOLVED {'v_A': 48759.9158597, 'v_A_km_s': 48.7599158597}

a = astronomy.ask(
    "Altitude-azimuth from hour angle: H = 0 h, dec = 0 deg, lat = 45 "
    "deg"
)
print(a.outcome, a.values)
# SOLVED {'alt_deg': 45.0, 'az_deg': 180.0}

a = astronomy.ask("Absolute bolometric magnitude: L = 1 Lsun")
print(a.outcome, a.values)
# SOLVED {'M_bol': 4.73999593392, 'L_Lsun': 1.0}

a = astronomy.ask("Rayleigh criterion: lam = 550 nm, D = 2.4 m")
print(a.outcome, a.values)
# SOLVED {'theta': 2.79507683415e-07, 'theta_arcsec': 0.0576525981642}

a = astronomy.ask("Calendar date from Julian date: jd = 2451545.0")
print(a.outcome, a.values)
# SOLVED {'year': 2000, 'month': 1, 'day': 1, 'hours': 12.0}

a = astronomy.ask(
    "Scale height: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2"
)
print(a.outcome, a.values)
# SOLVED {'H': 8425.76166363, 'H_km': 8.42576166363}

a = astronomy.ask("Debye length: T = 1e5 K, n = 1e18 m^-3")
print(a.outcome, a.values)
# SOLVED {'lambda_D': 2.18225558021e-05, 'lambda_D_nm': 21822.5558021}

a = astronomy.ask(
    "Apparent bolometric magnitude: L = 3.0128e28 W, d = 100 pc"
)
print(a.outcome, a.values)
# SOLVED {'m_bol': 5.0, 'F': 2.51802100263e-10, 'M_bol': 0.0}

a = astronomy.ask("Light-gathering power: D1 = 200 mm, D2 = 2 mm")
print(a.outcome, a.values)
# SOLVED {'area_ratio': 10000.0, 'mag_gain': 10.0}
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Astronomy on ai999x.com](https://www.ai999x.com/models/astronomy-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/astronomy-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/astronomy-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `astronomy` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/astronomy`) and its data (`rust/models/astronomy/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p astronomy-ai999x
./target/release/astronomy --help
cargo test -p astronomy-ai999x
```
