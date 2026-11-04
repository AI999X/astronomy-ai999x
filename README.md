# astronomy-ai999x — offline, verified astronomy solver with step-by-step answers

[![tests](https://github.com/AI999X/astronomy-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/astronomy-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/astronomy-ai999x)](https://pypi.org/project/astronomy-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/astronomy-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/astronomy-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/astronomy-ai999x/)** · **[Guides](https://huggingface.co/AI999X/astronomy-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/astronomy-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Physical Sciences** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

astronomy-ai999x is an offline, deterministic Python solver that answers astronomy questions with verified results from 67 certified capabilities, gives 350 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**astronomy-ai999x** is a compact, deterministic astronomy engine for Python. It reads a question in plain English (or a typed call), picks a certified astronomy capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** astronomy-ai999x comes as the AI999X desktop app and as the command-line program `astronomy` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Astronomy desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-astronomy-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-astronomy-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`astronomy`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/astronomy-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install astronomy-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Astronomy also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.3](https://github.com/AI999X/astronomy-ai999x/releases/tag/v1.0.3) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Astronomy) | Windows x86-64 | `ai999x-astronomy-1.2.0-windows-x86_64.zip` | 7.1 MB | [Hub](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/rust/ai999x-astronomy-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/astronomy-ai999x/releases/download/v1.0.3/ai999x-astronomy-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-astronomy-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-astronomy.exe` (no installer) |
| Desktop app (Astronomy) | Linux x86-64 | `ai999x-astronomy-1.2.0-linux-x86_64.tar.gz` | 8.0 MB | [Hub](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/rust/ai999x-astronomy-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/astronomy-ai999x/releases/download/v1.0.3/ai999x-astronomy-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-astronomy-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-astronomy` (X11 or Wayland desktop) |
| Command line (`astronomy`) | Windows x86-64 | `ai999x-astronomy-cli-1.2.0-windows-x86_64.zip` | 2.7 MB | [Hub](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/rust/ai999x-astronomy-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/astronomy-ai999x/releases/download/v1.0.3/ai999x-astronomy-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `astronomy.exe ask "<question>"` or `astronomy.exe solve <capability> name=value` |
| Command line (`astronomy`) | Linux x86-64 | `ai999x-astronomy-cli-1.2.0-linux-x86_64.tar.gz` | 2.7 MB | [Hub](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/rust/ai999x-astronomy-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/astronomy-ai999x/releases/download/v1.0.3/ai999x-astronomy-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./astronomy ask "<question>"` or `./astronomy solve <capability> name=value` |

The command-line archive holds `astronomy` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Astronomy** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Astronomy among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-astronomy-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-astronomy-1.2.0-linux-x86_64.tar.gz
./ai999x-astronomy-1.2.0-linux-x86_64/ai999x-astronomy
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
astronomy --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
astronomy --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ astronomy ask "Parallax distance: p = 0.768 arcsec"
Read as: Distance from parallax
d = 4.017809351e+16 m
d_pc = 1.302083333
d_ly = 4.246827835
p_arcsec = 0.768
  by: Distance from parallax  (d = 1/p with p in arcseconds gives parsecs)
  p = 3.723369071e-06 rad
  assumes: p > 0 (small-angle definition of the parsec)
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ astronomy info lookback_time
Lookback time to a redshift
  t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature
  in   z        redshift []
  in   H0       Hubble constant in km/s/Mpc []
  in   Om       matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) []
  out  t_L      lookback time [s]
  out  t_L_Gyr  lookback time in Gyr []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ astronomy solve lookback_time z=1.0 H0=70.0 Om=0.3
t_L = 2.43477519e+17 s
t_L_Gyr = 7.715337004
  by: Lookback time to a redshift  (t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature)
  z = 1
  H0 = 70
  Om = 0.3
  assumes: 0 < z <= 1100; flat Lambda-CDM, no radiation
```

**Draw the figures** — The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```console
$ astronomy figures parallax_distance "p=3.723369071e-06 rad" --out .
[
  {
    "slug": "parallax-distance-live-1",
    "title": "Distance in parsecs as annual parallax (arcsec, mas, rad, deg) varies",
    "caption": "Distance in parsecs as p runs from 9.308e-07 to 6.516e-06 rad, every other input as you entered it (41 solutions). Marked: your p = 3.723e-06 rad gives d_pc = 1.302. Your inputs: p = 3.723369071e-06 rad.",
    "file": "./parallax-distance-live-1.svg"
  },
  {
    "slug": "parallax-distance-live-2",
    "title": "The computed values side by side",
    "caption": "Every answer of one unit for your inputs: d_pc (distance in parsecs) = 1.302; d_ly (distance in light-years) = 4.247; p_arcsec (parallax in arcseconds) = 0.768. Your inputs: p = 3.723369071e-06 rad.",
    "file": "./parallax-distance-live-2.svg"
  }
]
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | astronomy batch

`cases.jsonl`:

```json
{"key": "lookback_time", "inputs": {"z": 1.0, "H0": 70.0, "Om": 0.3}}
{"key": "parallax_distance", "inputs": {"p": "3.723369071e-06 rad"}}
```

```console
$ astronomy batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":7.71533700361,"unit":"","solved_for":"t_L_Gyr","si":[["z",1.0],["H0",70.0],["Om",0.3]],"values":[["t_L",2.43477519025e+17,"s"],["t_L_Gyr",7.71533700361,""]],"target":"t_L_Gyr","phrase":"lookback time","assumptions":["0 < z <= 1100","flat Lambda-CDM, no radiation"],"name":"Lookback time to a redshift","equation":"t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature"}
{"outcome":"SOLVED","message":"","value":1.30208333331,"unit":"","solved_for":"d_pc","si":[["p",3.723369071e-6]],"values":[["d",4.01780935082e+16,"m"],["d_pc",1.30208333331,""],["d_ly",4.24682783476,""],["p_arcsec",0.768000000016,""]],"target":"d_pc","phrase":"distance","assumptions":["p > 0 (small-angle definition of the parsec)"],"name":"Distance from parallax","equation":"d = 1/p with p in arcseconds gives parsecs"}
```

### In your browser

Open [Astronomy on ai999x.com](https://www.ai999x.com/models/astronomy-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `astronomy_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install astronomy-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Schwarzschild radius

The Schwarzschild radius (event horizon of a non-rotating black hole) of a mass M: r_s = 2 G M / c^2.

Command line:

```console
$ astronomy solve schwarzschild_radius "M=1.988409871e+30 kg"
rs = 2953.250077 m
rs_km = 2.953250077
M_Msun = 1
  by: Schwarzschild radius  (r_s = 2 G M / c^2)
  M = 1.988409871e+30 kg
  assumes: M > 0; non-rotating, uncharged mass
```

Python:

```python
import astronomy_ai999x as m

a = m.solve("schwarzschild_radius", M="1.988409871e+30 kg")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/astronomy-ai999x/#cap-schwarzschild_radius) and press **Compute in your browser**.

_What this means:_ This calculation determines the event horizon radius for a non-rotating, uncharged black hole based on its total mass. Using the formula relating gravity and light speed, the resulting Schwarzschild radius is 2953.25 m. This distance represents the boundary from within which no information or light can escape the gravitational pull of the object with mass 1.98841e+30 kg.

### Age of the universe at a redshift

The age of a flat Lambda-CDM universe when the light from redshift z was emitted, and its present age.

Command line:

```console
$ astronomy solve age_at_redshift z=3 H0=70 Om=1
age = 3.673425692e+16 s
age_Gyr = 1.164038359
age_now_Gyr = 9.312306873
  by: Age of the universe at a redshift  (t(z) = (1/H0) integral_z^inf dz' / ((1 + z') E(z')) by mpmath quadrature)
  z = 3
  H0 = 70
  Om = 1
  assumes: 0 <= z <= 1100; flat Lambda-CDM, no radiation (not accurate in the radiation era)
```

Python:

```python
import astronomy_ai999x as m

a = m.solve("age_at_redshift", z="3", H0="70", Om="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/astronomy-ai999x/#cap-age_at_redshift) and press **Compute in your browser**.

_What this means:_ This calculation determines the age of a flat Lambda-CDM universe at the time light was emitted from an object at redshift 3. Using the Hubble constant 70 and matter density parameter 1, it computes that the universe was 1.16404 billion years old then. The present total age of this model universe is calculated to be 9.31231 billion years.

### Angular size of an object at a redshift

The angle subtended by an object of proper size D at redshift z, and the proper length per arcsecond, in a flat Lambda-CDM universe.

Command line:

```console
$ astronomy solve angular_size_at_redshift z=3 H0=70 Om=1 "D=3.085677581e+22 m"
theta_arcsec = 192.6470937
scale_kpc_per_arcsec = 5.190838755
D_A_Mpc = 1070.68735
  by: Angular size of an object at a redshift  (theta = D / D_A with D_A = D_C / (1 + z))
  z = 3
  H0 = 70
  Om = 1
  D = 3.085677581e+22 m
  assumes: 0 < z <= 1100; D > 0; small angles; flat Lambda-CDM, no radiation
```

Python:

```python
import astronomy_ai999x as m

a = m.solve("angular_size_at_redshift", z="3", H0="70", Om="1", D="3.085677581e+22 m")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/astronomy-ai999x/#cap-angular_size_at_redshift) and press **Compute in your browser**.

_What this means:_ This calculation determines the apparent angular size on the sky for a physical object of proper size 3.08568e+22 m located at redshift 3. It uses the angular-diameter distance 1070.69, derived from the cosmological parameters 70 and 1, to compute that the object subtends 192.647 arcseconds. Additionally, it provides the scale factor of 5.19084 kiloparsecs per arcsecond, which helps translate observed separations into physical distances in this flat Lambda-CDM universe.

### Comoving volume out to a redshift

The all-sky comoving volume enclosed within redshift z in a flat Lambda-CDM universe.

Command line:

```console
$ astronomy solve comoving_volume z=3 H0=70 Om=1
V = 9.667367618e+78 m^3
V_Gpc3 = 329.0459866
  by: Comoving volume out to a redshift  (V = (4 pi / 3) D_C^3 (flat space))
  z = 3
  H0 = 70
  Om = 1
  assumes: 0 < z <= 1100; flat Lambda-CDM, no radiation
```

Python:

```python
import astronomy_ai999x as m

a = m.solve("comoving_volume", z="3", H0="70", Om="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/astronomy-ai999x/#cap-comoving_volume) and press **Compute in your browser**.

_What this means:_ This calculation determines the total all-sky comoving volume enclosed within the specified redshift limit for a flat Lambda-CDM universe. Using the provided Hubble constant and matter density parameter, the result is 9.66737e+78 m^3 cubic meters, which corresponds to 329.046 cubic gigaparsecs. This metric represents the physical space available for counting galaxies and analyzing large-scale structure statistics at that cosmological distance.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/astronomy-ai999x/blob/main/docs/astronomy_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/astronomy-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified astronomy capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 350 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `astronomy` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** distances and magnitudes, stars and blackbody radiation, orbits and exoplanets, cosmology, telescopes and observing, positional astronomy and time, plasma and space physics.

## Model overview

| Property | Value |
|---|---|
| Package | `astronomy-ai999x` 1.0.3 (import `astronomy_ai999x`) |
| Type | Deterministic neuro-symbolic astronomy engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 67 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 350 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `astronomy-ai999x "<question>"`; for the native program use `astronomy` (above). The outputs above were produced by running this exact code on astronomy-ai999x 1.0.3.

## Find the right capability, and draw it

`astronomy.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "astronomy-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import astronomy_ai999x as astronomy

astronomy.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `astronomy-ai999x`; the answer is the one it returned, with the steps it took.

**Distance from parallax** · _Distances and magnitudes_

> Parallax distance: p = 0.768 arcsec

→ **d = 4.017809351e+16 m, d_pc = 1.302083333, d_ly = 4.246827835, p_arcsec = 0.768**

<details><summary>Steps</summary>

- Distance from parallax: p = 3.723369071e-06 rad → d = 4.017809351e+16 m, d_pc = 1.302083333, d_ly = 4.246827835, p_arcsec = 0.768

</details>

**Hohmann transfer between circular orbits** · _Orbits_

> Hohmann transfer: r1 = 6678 km, r2 = 42164 km, M = 1 Mearth

→ **dv1 = 2425.768901 m/s, dv2 = 1466.838638 m/s, dv = 3892.607539 m/s, t_transfer = 18990.05283 s, t_days = 0.2197922782, a_transfer = 24421000 m**

<details><summary>Steps</summary>

- Hohmann transfer between circular orbits: r1 = 6678 km, r2 = 42164 km, M = 5.972167868e+24 kg → dv1 = 2425.768901 m/s, dv2 = 1466.838638 m/s, dv = 3892.607539 m/s, t_transfer = 18990.05283 s, t_days = 0.2197922782 m/s, a_transfer = 24421000 m

</details>

**Lookback time to a redshift** · _Cosmology_

> Lookback time: z = 1, H0 = 70 km/s/Mpc, Om = 0.3

→ **t_L = 2.43477519e+17 s, t_L_Gyr = 7.715337004**

<details><summary>Steps</summary>

- Lookback time to a redshift: z = 1, H0 = 70, Om = 0.3 → t_L = 2.43477519e+17 s, t_L_Gyr = 7.715337004

</details>

**Luminosity of a star (Stefan-Boltzmann)** · _Stars_

> Stellar luminosity: R = 1 Rsun, T = 5772 K

→ **L = 3.827990903e+26 W, L_Lsun = 0.9999976236**

<details><summary>Steps</summary>

- Luminosity of a star (Stefan-Boltzmann): R = 695700000 m, T = 5772 K → L = 3.827990903e+26 W, L_Lsun = 0.9999976236 W

</details>

**Altitude and azimuth from hour angle and declination** · _Positional astronomy_

`m.solve("altaz_from_hadec", H="30 deg", dec="38.78 deg", lat="52 deg")`

→ **alt_deg = 65.39488814, az_deg = 249.4151744**

<details><summary>Steps</summary>

- Altitude and azimuth from hour angle and declination: H = 30 deg, dec = 38.78 deg, lat = 52 deg → alt_deg = 65.39488814, az_deg = 249.4151744

</details>

**Schwarzschild radius** · _Compact objects_

> Schwarzschild radius: M = 1 Msun

→ **rs = 2953.250076 m, rs_km = 2.953250076, M_Msun = 1**

<details><summary>Steps</summary>

- Schwarzschild radius: M = 1.988409871e+30 kg → rs = 2953.250076 m, rs_km = 2.953250076 m, M_Msun = 1 m

</details>

## Figures

Every point was computed by `astronomy-ai999x`.

**A Kepler orbit sampled at equal times**

![A Kepler orbit sampled at equal times — computed by astronomy-ai999x](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/kepler-position-at-time-orbit.svg)

_An orbit with a = 1.5 au and e = 0.6 around one solar mass: 181 positions (true anomaly and distance) solved by the package from Kepler's equation over one period P = 671.01 days, and 16 positions at equal time steps — equal areas swept, bunched near aphelion; marked: perihelion r = 0.6 au, speed 48.6389 km/s (aphelion 12.1597 km/s)._

**An Earth-to-Mars Hohmann transfer**

![An Earth-to-Mars Hohmann transfer — computed by astronomy-ai999x](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/hohmann-transfer-orbit.svg)

_The Hohmann transfer from a circular orbit of r₁ = 1 au to one of r₂ = 1.52368 au around the Sun, computed by the package: transfer ellipse a = 1.26184 au, e = 0.207506 (drawn from 91 positions it solved over the 258.862-day half orbit), burns Δv₁ = 2.94473 km/s and Δv₂ = 2.64894 km/s, total 5.59367 km/s._

**Daily paths of stars on a sky chart**

![Daily paths of stars on a sky chart — computed by astronomy-ai999x](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/altaz-from-hadec-polar.svg)

_Daily paths across the sky of stars of four declinations, seen from latitude 45° north (azimuth around the rim, zenith distance outward; 97 hour angles each converted by the package to altitude and azimuth, only the part above the horizon drawn): δ = 60°: culminates at altitude 75°, azimuth 0°; δ = 20°: culminates at altitude 65°, azimuth 180°; δ = 0°: culminates at altitude 45°, azimuth 180°; δ = -30°: culminates at altitude 15°, azimuth 180°. The δ = 60° star circles the pole without setting._

**Cosmological distances against redshift**

![Cosmological distances against redshift — computed by astronomy-ai999x](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/cosmological-distances-curve.svg)

_Distance measures in a flat ΛCDM universe with H₀ = 70 km/s/Mpc and Ωm = 0.3, 100 redshifts each computed by the package; at z = 3: comoving 6355.69 Mpc, luminosity 25422.7 Mpc, angular-diameter 1588.92 Mpc. The angular-diameter distance peaks — marked: z = 1.6, D_A = 1747.57 Mpc (largest of the computed points)._

**Equilibrium temperatures of the planets**

![Equilibrium temperatures of the planets — computed by astronomy-ai999x](https://huggingface.co/AI999X/astronomy-ai999x/resolve/main/figures/planet-equilibrium-temperature-bars.svg)

_Equilibrium temperatures of the eight planets for the Sun (5772 K, radius 6.957e8 m), from their mean distances and Bond albedos (inputs), each computed by the package: Mercury 437.223 K, Venus 229.11 K, Earth 254.038 K, Mars 209.813 K, Jupiter 102.452 K, Saturn 81.1727 K, Uranus 58.1161 K, Neptune 46.5916 K (Earth highlighted; these are no-atmosphere values, without any greenhouse warming)._

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
@software{astronomy_ai999x_1_0_3,
  title  = {astronomy-ai999x: an offline, verified astronomy solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
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
