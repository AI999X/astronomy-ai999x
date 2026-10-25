# astronomy-ai999x — offline, verified astronomy solver with step-by-step answers

[![tests](https://github.com/AI999X/astronomy-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/astronomy-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/astronomy-ai999x)](https://pypi.org/project/astronomy-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/astronomy-ai999x)

**[Install](https://pypi.org/project/astronomy-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/astronomy-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/astronomy-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

astronomy-ai999x is an offline, deterministic Python solver that answers astronomy questions with verified results from 67 certified capabilities, gives 350 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**astronomy-ai999x** is a compact, deterministic astronomy engine for Python. It reads a question in plain English (or a typed call), picks a certified astronomy capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified astronomy capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 350 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** distances and magnitudes, stars and blackbody radiation, orbits and exoplanets, cosmology, telescopes and observing, positional astronomy and time, plasma and space physics.

## Model overview

| Property | Value |
|---|---|
| Package | `astronomy-ai999x` 1.0.0 (import `astronomy_ai999x`) |
| Type | Deterministic neuro-symbolic astronomy engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 67 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 350 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install astronomy-ai999x
```

```python
import astronomy_ai999x as astronomy

astronomy.ask("Parallax distance: p = 0.768 arcsec").show()
# -> SOLVED — d = 4.01781e+16, d_pc = 1.30208, d_ly = 4.24683, p_arcsec = 0.768  (parallax_distance)
astronomy.ask("Hohmann transfer: r1 = 6678 km, r2 = 42164 km, M = 1 Mearth").show()
# -> SOLVED — dv1 = 2425.77, dv2 = 1466.84, dv = 3892.61, t_transfer = 18990.1  (hohmann_transfer)
astronomy.ask("Lookback time: z = 1, H0 = 70 km/s/Mpc, Om = 0.3").show()
# -> SOLVED — t_L = 2.43478e+17, t_L_Gyr = 7.71534  (lookback_time)
astronomy.ask("Stellar luminosity: R = 1 Rsun, T = 5772 K").show()
# -> SOLVED — L = 3.82799e+26, L_Lsun = 0.999998  (stellar_luminosity)
astronomy.solve("altaz_from_hadec", H="30 deg", dec="38.78 deg", lat="52 deg").show()
# -> SOLVED — alt_deg = 65.3949, az_deg = 249.415  (altaz_from_hadec)
astronomy.ask("Who created you?").message
# -> astronomy-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `astronomy-ai999x "<question>"`, `astronomy-ai999x --capabilities`. The outputs above were produced by running this exact code on astronomy-ai999x 1.0.0.

## Find the right capability, and draw it

`astronomy.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "astronomy-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import astronomy_ai999x as astronomy

astronomy.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

67 certified procedures. A few of them:

- **Procedures:** Absolute bolometric magnitude from luminosity; Absolute magnitude; Age of the universe at a redshift; Airmass and atmospheric extinction; Alfven speed; Altitude and azimuth from hour angle and declination; Altitude of a star at culmination; Angular separation of two sky positions; Angular size; Angular size of an object at a redshift; Apparent bolometric magnitude and flux; Astronomical distance units; Calendar date from a Julian date; Combined magnitude of two stars; Comoving volume out to a redshift; Cosmological distances at a redshift; Debye length; Delta-v of a combined plane change; Departure burn from a circular orbit; Diffraction limit of a telescope (Rayleigh criterion); Distance from luminosity and flux (inverse-square law); Distance from parallax; Distance from the distance modulus; Effective temperature of a star …

`astronomy_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified astronomy capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (distances and magnitudes, stars and blackbody radiation, orbits and exoplanets, cosmology, telescopes and observing, positional astronomy and time, plasma and space physics); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{astronomy_ai999x_1_0_0,
  title  = {astronomy-ai999x: an offline, verified astronomy solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/astronomy-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/astronomy-ai999x/discussions or visit https://ai999x.com.

## Support the project

astronomy-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/astronomy-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/astronomy-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/astronomy-ai999x/tree/main/onnx).
