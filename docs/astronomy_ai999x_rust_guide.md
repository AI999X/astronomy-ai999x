# astronomy: the Astronomy program, user guide

**astronomy-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Astronomy covers celestial mechanics and orbits, stellar properties, blackbody radiation, magnitudes and distances, cosmology basics, the Sun and space environment and plasma parameters. This guide is about `astronomy`, the model as a self-contained program for Windows and Linux: 67 capabilities in 13 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/astronomy-ai999x/); the Python package (`pip install astronomy-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Astronomy | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-astronomy-1.5.0-windows-x86_64.zip |
| Desktop app: Astronomy | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-astronomy-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Astronomy in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-astronomy-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-astronomy.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-astronomy-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-astronomy-1.5.0-linux-x86_64.tar.gz
./ai999x-astronomy-1.5.0-linux-x86_64/ai999x-astronomy
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `astronomy` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
astronomy --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
astronomy --version
```

### In your browser

Open https://www.ai999x.com/models/astronomy-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install astronomy-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ astronomy --version
astronomy-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ astronomy search "lookback time to a redshift"
lookback_time                        Lookback time to a redshift  (cosmology / time)
redshift_at_lookback_time            Redshift at a given lookback time  (cosmology / time)
age_at_redshift                      Age of the universe at a redshift  (cosmology / time)
cosmological_distances               Cosmological distances at a redshift  (cosmology / distances)
comoving_volume                      Comoving volume out to a redshift  (cosmology / volumes)
angular_size_at_redshift             Angular size of an object at a redshift  (cosmology / angular scale)
recession_velocity_from_redshift     Radial velocity from a Doppler redshift  (cosmology / redshift)
redshift_from_wavelengths            Redshift and radial velocity from a spectral line  (cosmology / redshift)
… 9 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ astronomy list
absolute_magnitude                       Absolute magnitude
age_at_redshift                          Age of the universe at a redshift
airmass_extinction                       Airmass and atmospheric extinction
alfven_speed                             Alfven speed
altaz_from_hadec                         Altitude and azimuth from hour angle and declination
angular_separation                       Angular separation of two sky positions
angular_size                             Angular size
angular_size_at_redshift                 Angular size of an object at a redshift
… 59 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ astronomy info lookback_time
Lookback time to a redshift
  t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature
  in   z        redshift []
  in   H0       Hubble constant in km/s/Mpc []
  in   Om       matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) []
  out  t_L      lookback time [s]
  out  t_L_Gyr  lookback time in Gyr []
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ astronomy solve lookback_time z=1.0 H0=70.0 Om=0.3
t_L = 2.43477519e+17 s
t_L_Gyr = 7.715337004
  by: Lookback time to a redshift  (t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature)
  z = 1
  H0 = 70
  Om = 0.3
  assumes: 0 < z <= 1100; flat Lambda-CDM, no radiation
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ astronomy solve lookback_time z=1.0 H0=70.0 Om=0.3 --json
{"outcome":"SOLVED","message":"","value":7.71533700361,"unit":"","solved_for":"t_L_Gyr","si":[["z",1.0],["H0",70.0],["Om",0.3]],"values":[["t_L",2.43477519025e+17,"s"],["t_L_Gyr",7.71533700361,""]],"target":"t_L_Gyr","phrase":"lookback time","assumptions":["0 < z <= 1100","flat Lambda-CDM, no radiation"],"name":"Lookback time to a redshift","equation":"t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ astronomy solve lookback_time z=1.0 H0=70.0
NEEDS_INPUT: lookback_time needs Om (matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m)). Example: solve('lookback_time', z=3, H0=70, Om=1).
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | astronomy batch

`cases.jsonl`:

```json
{"key": "lookback_time", "inputs": {"z": 1.0, "H0": 70.0, "Om": 0.3}}
{"key": "parallax_distance", "inputs": {"p": "3.723369071e-06 rad"}}
```

```shell
$ astronomy batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":7.71533700361,"unit":"","solved_for":"t_L_Gyr","si":[["z",1.0],["H0",70.0],["Om",0.3]],"values":[["t_L",2.43477519025e+17,"s"],["t_L_Gyr",7.71533700361,""]],"target":"t_L_Gyr","phrase":"lookback time","assumptions":["0 < z <= 1100","flat Lambda-CDM, no radiation"],"name":"Lookback time to a redshift","equation":"t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature"}
{"outcome":"SOLVED","message":"","value":1.30208333331,"unit":"","solved_for":"d_pc","si":[["p",3.723369071e-6]],"values":[["d",4.01780935082e+16,"m"],["d_pc",1.30208333331,""],["d_ly",4.24682783476,""],["p_arcsec",0.768000000016,""]],"target":"d_pc","phrase":"distance","assumptions":["p > 0 (small-angle definition of the parsec)"],"name":"Distance from parallax","equation":"d = 1/p with p in arcseconds gives parsecs"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `astronomy info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`astronomy solve ... --json` prints one answer as one line of JSON. `astronomy batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | astronomy batch`. `astronomy catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["astronomy", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`astronomy figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `astronomy_ai999x.wasm` (https://www.ai999x.com/compute/rust/astronomy_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("astronomy_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `astronomy info <id>` shows the same inputs and outputs in the terminal.

### Compact objects (1)

<a id="schwarzschild_radius"></a>

#### Schwarzschild radius: `schwarzschild_radius`

The Schwarzschild radius (event horizon of a non-rotating black hole) of a mass M: r_s = 2 G M / c^2.

$$
r_s = \frac{2GM}{c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass (kg, Msun, Mearth) | kg |
| `rs` | out | Schwarzschild radius | m |
| `rs_km` | out | Schwarzschild radius in kilometres | - |
| `M_Msun` | out | mass in solar masses | - |

**Applies when:** M > 0; non-rotating, uncharged mass.

```shell
$ astronomy solve schwarzschild_radius "M=1.988409871e+30 kg"
rs = 2953.250077 m
rs_km = 2.953250077
M_Msun = 1
  by: Schwarzschild radius  (r_s = 2 G M / c^2)
  M = 1.988409871e+30 kg
  assumes: M > 0; non-rotating, uncharged mass
```

**What the worked example means:** This calculation determines the event horizon radius for a non-rotating, uncharged black hole based on its total mass. Using the formula relating gravity and light speed, the resulting Schwarzschild radius is 2953.25 m. This distance represents the boundary from within which no information or light can escape the gravitational pull of the object with mass 1.98841e+30 kg.

### Cosmology (9)

<a id="age_at_redshift"></a>

#### Age of the universe at a redshift: `age_at_redshift`

The age of a flat Lambda-CDM universe when the light from redshift z was emitted, and its present age.

$$
t(z) = \frac{1}{H_0}\int_z^\infty \frac{dz'}{(1+z')\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | redshift | - |
| `H0` | in | Hubble constant in km/s/Mpc | - |
| `Om` | in | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) | - |
| `age` | out | age at redshift z | s |
| `age_Gyr` | out | age at redshift z in Gyr | - |
| `age_now_Gyr` | out | present age in Gyr | - |

**Applies when:** 0 <= z <= 1100; flat Lambda-CDM, no radiation (not accurate in the radiation era).

```shell
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

**What the worked example means:** This calculation determines the age of a flat Lambda-CDM universe at the time light was emitted from an object at redshift 3. Using the Hubble constant 70 and matter density parameter 1, it computes that the universe was 1.16404 billion years old then. The present total age of this model universe is calculated to be 9.31231 billion years.

<a id="angular_size_at_redshift"></a>

#### Angular size of an object at a redshift: `angular_size_at_redshift`

The angle subtended by an object of proper size D at redshift z, and the proper length per arcsecond, in a flat Lambda-CDM universe.

$$
\theta = \frac{D}{D_A},\quad D_A = \frac{1}{1+z}\frac{c}{H_0}\int_0^z\frac{dz'}{E(z')}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | redshift | - |
| `H0` | in | Hubble constant in km/s/Mpc | - |
| `Om` | in | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) | - |
| `D` | in | proper size of the object (kpc, Mpc, m) | m |
| `theta_arcsec` | out | angular size in arcseconds | - |
| `scale_kpc_per_arcsec` | out | proper kiloparsecs per arcsecond | - |
| `D_A_Mpc` | out | angular-diameter distance in Mpc | - |

**Applies when:** 0 < z <= 1100; D > 0; small angles; flat Lambda-CDM, no radiation.

```shell
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

**What the worked example means:** This calculation determines the apparent angular size on the sky for a physical object of proper size 3.08568e+22 m located at redshift 3. It uses the angular-diameter distance 1070.69, derived from the cosmological parameters 70 and 1, to compute that the object subtends 192.647 arcseconds. Additionally, it provides the scale factor of 5.19084 kiloparsecs per arcsecond, which helps translate observed separations into physical distances in this flat Lambda-CDM universe.

<a id="comoving_volume"></a>

#### Comoving volume out to a redshift: `comoving_volume`

The all-sky comoving volume enclosed within redshift z in a flat Lambda-CDM universe.

$$
V_C = \frac{4\pi}{3}D_C^3
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | redshift | - |
| `H0` | in | Hubble constant in km/s/Mpc | - |
| `Om` | in | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) | - |
| `V` | out | comoving volume | m^3 |
| `V_Gpc3` | out | comoving volume in cubic gigaparsecs | - |

**Applies when:** 0 < z <= 1100; flat Lambda-CDM, no radiation.

```shell
$ astronomy solve comoving_volume z=3 H0=70 Om=1
V = 9.667367618e+78 m^3
V_Gpc3 = 329.0459866
  by: Comoving volume out to a redshift  (V = (4 pi / 3) D_C^3 (flat space))
  z = 3
  H0 = 70
  Om = 1
  assumes: 0 < z <= 1100; flat Lambda-CDM, no radiation
```

**What the worked example means:** This calculation determines the total all-sky comoving volume enclosed within the specified redshift limit for a flat Lambda-CDM universe. Using the provided Hubble constant and matter density parameter, the result is 9.66737e+78 m^3 cubic meters, which corresponds to 329.046 cubic gigaparsecs. This metric represents the physical space available for counting galaxies and analyzing large-scale structure statistics at that cosmological distance.

<a id="cosmological_distances"></a>

#### Cosmological distances at a redshift: `cosmological_distances`

The comoving, luminosity and angular-diameter distances and the distance modulus of an object at redshift z in a flat Lambda-CDM universe.

$$
D_C = \frac{c}{H_0}\int_0^z \frac{dz'}{\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}},\quad D_L = (1+z)D_C,\quad D_A = \frac{D_C}{1+z}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | redshift | - |
| `H0` | in | Hubble constant in km/s/Mpc | - |
| `Om` | in | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) | - |
| `D_C` | out | comoving distance | m |
| `D_C_Mpc` | out | comoving distance in Mpc | - |
| `D_L_Mpc` | out | luminosity distance in Mpc | - |
| `D_A_Mpc` | out | angular-diameter distance in Mpc | - |
| `mu` | out | distance modulus | - |
| `D_L` | out | luminosity distance | m |

**Applies when:** 0 < z <= 1100; 1 <= H0 <= 1000 km/s/Mpc; 0.01 <= Omega_m <= 1; flat, no radiation.

```shell
$ astronomy solve cosmological_distances z=3 H0=70 Om=1
D_C = 1.321518381e+26 m
D_C_Mpc = 4282.7494
D_L_Mpc = 17130.9976
D_A_Mpc = 1070.68735
mu = 46.16891327
D_L = 5.286073524e+26 m
  by: Cosmological distances at a redshift  (D_C = (c/H0) integral_0^z dz'/E(z'), E = sqrt(Om (1+z)^3 + 1 - Om) (mpmath quadrature); D_L = (1+z) D_C; D_A = D_C/(1+z))
  z = 3
  H0 = 70
  Om = 1
  assumes: 0 < z <= 1100; 1 <= H0 <= 1000 km/s/Mpc; 0.01 <= Omega_m <= 1; flat, no radiation
```

**What the worked example means:** This calculation determines three key distance measures for an object at redshift 3 in a flat universe with Hubble constant 70 km/s per megaparsec and matter density parameter 1. The comoving distance of 4282.75 megaparsecs represents the current proper separation between the observer and the object, accounting for the expansion of space. The luminosity distance of 17131 megaparsecs is used to relate observed brightness to intrinsic luminosity, while the angular-diameter distance of 1070.69 megaparsecs relates physical size to apparent angular size on the sky.

<a id="critical_density"></a>

#### Hubble parameter and critical density: `critical_density`

The Hubble parameter H(z) and the critical density 3H^2/(8 pi G) of a flat Lambda-CDM universe at redshift z (z = 0 gives today's values), with the equivalent number of hydrogen atoms per cubic metre.

$$
H(z) = H_0\sqrt{\Omega_m(1+z)^3 + \Omega_\Lambda},\quad \rho_c = \frac{3H^2}{8\pi G}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H0` | in | Hubble constant in km/s/Mpc | - |
| `Om` | in | matter density parameter Omega_m | - |
| `z` | in | redshift (default 0) (optional) | - |
| `H_z` | out | Hubble parameter H(z) in km/s/Mpc | - |
| `rho_c` | out | critical density | kg/m^3 |
| `n_H` | out | critical density in hydrogen-atom (proton) masses per cubic metre | 1/m^3 |

**Applies when:** 1 <= H0 <= 1000; 0.01 <= Omega_m <= 1; 0 <= z <= 1100; flat, no radiation.

```shell
$ astronomy solve critical_density H0=70 Om=0.3
H_z = 70
rho_c = 9.203873923e-27 kg/m^3
n_H = 5.502662485 1/m^3
  by: Hubble parameter and critical density  (H(z) = H0 sqrt(Om (1+z)^3 + 1 - Om); rho_c = 3 H^2 / (8 pi G))
  H0 = 70
  Om = 0.3
  assumes: 1 <= H0 <= 1000; 0.01 <= Omega_m <= 1; 0 <= z <= 1100; flat, no radiation
```

**What the worked example means:** This calculation determines the expansion rate and the density required for a flat universe at a specific cosmic epoch. Using the provided Hubble constant 70 and matter density parameter 0.3, it computes the Hubble parameter 70 and the corresponding critical density 9.20387e-27 kg/m^3. The result also expresses this density as 5.50266 1/m^3 hydrogen atoms per cubic metre, offering a tangible measure of the universe's mass content at that time.

<a id="lookback_time"></a>

#### Lookback time to a redshift: `lookback_time`

How long ago the light received from redshift z was emitted, in a flat Lambda-CDM universe.

$$
t_L = \frac{1}{H_0}\int_0^z \frac{dz'}{(1+z')\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | redshift | - |
| `H0` | in | Hubble constant in km/s/Mpc | - |
| `Om` | in | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) | - |
| `t_L` | out | lookback time | s |
| `t_L_Gyr` | out | lookback time in Gyr | - |

**Applies when:** 0 < z <= 1100; flat Lambda-CDM, no radiation.

```shell
$ astronomy solve lookback_time z=3 H0=70 Om=1
t_L = 2.571397985e+17 s
t_L_Gyr = 8.148268514
  by: Lookback time to a redshift  (t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature)
  z = 3
  H0 = 70
  Om = 1
  assumes: 0 < z <= 1100; flat Lambda-CDM, no radiation
```

**What the worked example means:** This calculation determines the lookback time, which is how long the light observed now has been traveling through space. By integrating the expansion history of a flat Lambda-CDM universe from the present day back to redshift 3, using a Hubble constant of 70 and matter density of 1, the model computes that this journey took 2.5714e+17 s seconds, or roughly 8.14827 billion years.

<a id="recession_velocity_from_redshift"></a>

#### Radial velocity from a Doppler redshift: `recession_velocity_from_redshift`

The radial velocity of a source from its redshift interpreted as a special-relativistic Doppler shift (negative for a blueshift), with the low-velocity value cz for comparison.

$$
\frac{v}{c} = \frac{(1+z)^2 - 1}{(1+z)^2 + 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | redshift (negative for a blueshift) | - |
| `v` | out | radial velocity (positive receding) | m/s |
| `beta` | out | v/c | - |
| `v_cz` | out | non-relativistic estimate cz | m/s |

**Applies when:** -1 < z; a purely kinematic (special-relativistic, radial) Doppler shift; not a cosmological recession.

```shell
$ astronomy solve recession_velocity_from_redshift z=1
v = 179875474.8 m/s
beta = 0.6
v_cz = 299792458 m/s
  by: Radial velocity from a Doppler redshift  (v/c = ((1+z)^2 - 1) / ((1+z)^2 + 1))
  z = 1
  assumes: -1 < z; a purely kinematic (special-relativistic, radial) Doppler shift; not a cosmological recession
```

**What the worked example means:** This calculation determines the radial velocity from a given redshift using the special-relativistic Doppler formula. By substituting the input redshift 1 into the equation, the resulting ratio of velocity to light speed is found to be 0.6. Multiplying this ratio by the speed of light yields the actual radial velocity of 1.79875e+08 m/s. For comparison, the non-relativistic estimate based on the simple product of the speed of light and the redshift is 2.99792e+08 m/s. A positive value indicates the source is moving away, while a negative value signifies it is approaching.

<a id="redshift_from_wavelengths"></a>

#### Redshift and radial velocity from a spectral line, `redshift_from_wavelengths`

The redshift z = lambda_obs/lambda_rest - 1 of a spectral line and the corresponding relativistic Doppler radial velocity.

$$
z = \frac{\lambda_\mathrm{obs}}{\lambda_\mathrm{rest}} - 1,\quad \frac{v}{c} = \frac{(1+z)^2 - 1}{(1+z)^2 + 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam_obs` | in | observed wavelength (nm, Angstrom, m) | m |
| `lam_rest` | in | rest (laboratory) wavelength | m |
| `z` | out | redshift | - |
| `v` | out | radial velocity (positive receding) | m/s |
| `beta` | out | v/c | - |
| `d_lambda` | out | wavelength shift | m |

**Applies when:** both wavelengths > 0; a purely kinematic radial Doppler shift.

```shell
$ astronomy solve redshift_from_wavelengths "lam_obs=1312.56 nm" "lam_rest=656.28 nm"
z = 1
v = 179875474.8 m/s
beta = 0.6
d_lambda = 6.5628e-07 m
  by: Redshift and radial velocity from a spectral line  (z = lambda_obs/lambda_rest - 1; v/c = ((1+z)^2 - 1)/((1+z)^2 + 1))
  lam_obs = 1.31256e-06 m
  lam_rest = 6.5628e-07 m
  assumes: both wavelengths > 0; a purely kinematic radial Doppler shift
```

**What the worked example means:** This calculation determines the redshift and relativistic Doppler radial velocity by comparing the observed wavelength 1312.56 nm with the known rest wavelength 656.28 nm. The resulting redshift value is 1, which corresponds to a dimensionless velocity beta of 0.6. Multiplying this ratio by the speed of light gives the physical radial velocity of 1.79875e+08 m/s. A positive velocity indicates that the source is moving away from the observer, while a negative value would indicate motion toward the observer.

<a id="redshift_at_lookback_time"></a>

#### Redshift at a given lookback time: `redshift_at_lookback_time`

The redshift of light emitted a lookback time t ago in a flat Lambda-CDM universe (exact inverse of the age-redshift relation).

$$
1 + z = \left[\frac{\sqrt{\Omega_\Lambda/\Omega_m}}{\sinh\left(\tfrac{3}{2}H_0\sqrt{\Omega_\Lambda}\,(t_0 - t_L)\right)}\right]^{2/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | lookback time (Gyr, Myr, yr) | s |
| `H0` | in | Hubble constant in km/s/Mpc | - |
| `Om` | in | matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m) | - |
| `z` | out | redshift | - |
| `age_now_Gyr` | out | present age of the universe in Gyr | - |

**Applies when:** 0 < t < present age; flat Lambda-CDM, no radiation.

```shell
$ astronomy solve redshift_at_lookback_time "t=2.524608e+17 s" H0=70 Om=0.3
z = 1.074672875
age_now_Gyr = 13.46698395
  by: Redshift at a given lookback time  (t(z) = 2/(3 H0 sqrt(OL)) asinh( sqrt(OL/Om) (1+z)^(-3/2) ), inverted in closed form (Om = 1: t = 2/(3 H0) (1+z)^(-3/2)))
  t = 2.524608e+17 s
  H0 = 70
  Om = 0.3
  assumes: 0 < t < present age; flat Lambda-CDM, no radiation
```

**What the worked example means:** This calculation determines the cosmological redshift corresponding to a specific lookback time in a flat Lambda-CDM universe. Given an input lookback time of 2.52461e+17 s and Hubble constant of 70, the method analytically inverts the age-redshift relation to yield a redshift of 1.07467. The result also provides the current age of the universe, calculated as 13.467, which serves as the reference point for measuring how far back in cosmic history the light was emitted.

### Distances and magnitudes (8)

<a id="absolute_magnitude"></a>

#### Absolute magnitude: `absolute_magnitude`

The absolute magnitude of a star from its apparent magnitude and distance (and an optional extinction A).

$$
M = m - 5\log_{10}\frac{d}{10\,\mathrm{pc}} - A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | apparent magnitude | - |
| `d` | in | distance (pc, ly, m ...) | m |
| `A` | in | interstellar extinction in magnitudes (default 0) (optional) | - |
| `M` | out | absolute magnitude | - |
| `mu` | out | distance modulus 5 log10(d/10 pc) | - |
| `d_pc` | out | distance in parsecs | - |

**Applies when:** d > 0; A >= 0.

```shell
$ astronomy solve absolute_magnitude m=8 "d=3.085677581e+18 m" A=1
M = 2
mu = 5
d_pc = 99.99999998
  by: Absolute magnitude  (M = m - 5 log10(d / 10 pc) - A)
  m = 8
  d = 3.085677581e+18 m
  A = 1
  assumes: d > 0; A >= 0
```

**What the worked example means:** This calculation determines the intrinsic brightness of a star, known as its absolute magnitude. It uses the observed apparent magnitude 8 and the distance 3.08568e+18 m to compute the value at a standard reference distance of ten parsecs. The resulting absolute magnitude 2 allows for direct comparison with other stars regardless of their actual location in space. If interstellar extinction is present, it is accounted for by the term A, ensuring the brightness reflects the star's true output rather than just what reaches the observer.

<a id="distance_unit_conversion"></a>

#### Astronomical distance units: `distance_unit_conversion`

A distance expressed in metres, kilometres, astronomical units, light-years and parsecs.

$$
1\,\mathrm{pc} = \frac{648000}{\pi}\,\mathrm{au},\quad 1\,\mathrm{ly} = c \times 365.25\,\mathrm{d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | distance (m, km, au, ly, pc, kpc, Mpc ...) | m |
| `d_m` | out | distance in metres | m |
| `d_km` | out | distance in kilometres | - |
| `d_au` | out | distance in astronomical units | - |
| `d_ly` | out | distance in light-years | - |
| `d_pc` | out | distance in parsecs | - |

**Applies when:** d > 0.

```shell
$ astronomy solve distance_unit_conversion "d=3.085677581e+16 m"
d_m = 3.085677581e+16 m
d_km = 3.085677581e+13
d_au = 206264.8062
d_ly = 3.261563777
d_pc = 0.9999999998
  by: Astronomical distance units  (1 au = 149597870700 m; 1 pc = au * 648000/pi; 1 ly = c * 365.25 days)
  d = 3.085677581e+16 m
  assumes: d > 0
```

**What the worked example means:** This calculation takes the input distance 3.08568e+16 m and expresses it in five standard astronomical units. The result shows the same physical length as 3.08568e+16 m in metres, 3.08568e+13 in kilometres, 206265 in astronomical units, 3.26156 in light-years, and 1 in parsecs. These conversions allow astronomers to compare distances across vastly different scales using consistent definitions based on the speed of light and the Earth-Sun distance.

<a id="combined_magnitude"></a>

#### Combined magnitude of two stars: `combined_magnitude`

The magnitude of two unresolved stars seen together (their fluxes add).

$$
m = -2.5\log_{10}\left(10^{-0.4 m_1} + 10^{-0.4 m_2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | magnitude of the first star | - |
| `m2` | in | magnitude of the second star | - |
| `m` | out | combined magnitude | - |
| `brightening` | out | how much brighter than the brighter star alone (mag) | - |

**Applies when:** |m1 - m2| <= 100.

```shell
$ astronomy solve combined_magnitude m1=5 m2=5
m = 4.247425011
brightening = 0.7525749892
  by: Combined magnitude of two stars  (fluxes add: m = -2.5 log10(10^(-0.4 m1) + 10^(-0.4 m2)))
  m1 = 5
  m2 = 5
  assumes: |m1 - m2| <= 100
```

**What the worked example means:** This calculation determines the total brightness of two unresolved stars by adding their light fluxes together. Given the individual magnitudes 5 and 5, the resulting combined magnitude is 4.24743. This value represents how bright the system appears as a single point source, while the metric 0.752575 quantifies the increase in brightness compared to the brighter component alone.

<a id="distance_from_flux"></a>

#### Distance from luminosity and flux (inverse-square law), `distance_from_flux`

The distance of a source of known luminosity L (a standard candle) from the bolometric flux F measured at the observer.

$$
d = \sqrt{\frac{L}{4\pi F}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | luminosity (W or Lsun) | W |
| `F` | in | measured flux (W/m^2) | W/m^2 |
| `d` | out | distance | m |
| `d_pc` | out | distance in parsecs | - |
| `d_ly` | out | distance in light-years | - |

**Applies when:** L, F > 0; isotropic emission, no extinction.

```shell
$ astronomy solve distance_from_flux "L=3.828e+26 W" "F=1361 W/m^2"
d = 1.496070192e+11 m
d_pc = 4.848433293e-06
d_ly = 1.58134744e-05
  by: Distance from luminosity and flux (inverse-square law)  (F = L / (4 pi d^2): d = sqrt(L / (4 pi F)))
  L = 3.828e+26 W
  F = 1361 W/m^2
  assumes: L, F > 0; isotropic emission, no extinction
```

**What the worked example means:** This calculation determines the distance of a celestial object using its known luminosity and measured flux. Applying the inverse-square law shows that for a source with luminosity 3.828e+26 W and observed flux 1361 W/m^2, the resulting distance is 1.49607e+11 m. This value can also be expressed as 4.84843e-06 in parsecs or 1.58135e-05 in light-years, providing a standard candle measurement for astronomical mapping.

<a id="parallax_distance"></a>

#### Distance from parallax: `parallax_distance`

The distance of a star from its annual trigonometric parallax: d [pc] = 1 / p [arcsec].

$$
d\,[\mathrm{pc}] = \frac{1}{p\,[\mathrm{arcsec}]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | annual parallax (arcsec, mas, rad, deg) | rad |
| `d` | out | distance | m |
| `d_pc` | out | distance in parsecs | - |
| `d_ly` | out | distance in light-years | - |
| `p_arcsec` | out | parallax in arcseconds | - |

**Applies when:** p > 0 (small-angle definition of the parsec).

```shell
$ astronomy solve parallax_distance "p=4.848136811e-07 rad"
d = 3.085677582e+17 m
d_pc = 10
d_ly = 32.61563777
p_arcsec = 0.1
  by: Distance from parallax  (d = 1/p with p in arcseconds gives parsecs)
  p = 4.848136811e-07 rad
  assumes: p > 0 (small-angle definition of the parsec)
```

**What the worked example means:** This calculation determines the stellar distance using the inverse relationship between parallax and distance. Given an annual parallax angle of 4.84814e-07 rad, the resulting distance is computed as 10 parsecs, which corresponds to 32.6156 light-years or 3.08568e+17 m meters. The method relies on the definition that one parsec is the distance at which one astronomical subtends an angle of one arcsecond.

<a id="distance_from_distance_modulus"></a>

#### Distance from the distance modulus: `distance_from_distance_modulus`

The distance of a star from its apparent and absolute magnitudes (and an optional extinction A): m - M - A = 5 log10(d / 10 pc).

$$
d = 10^{(m - M - A)/5 + 1}\ \mathrm{pc}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | apparent magnitude | - |
| `M` | in | absolute magnitude | - |
| `A` | in | interstellar extinction in magnitudes (default 0) (optional) | - |
| `d` | out | distance | m |
| `d_pc` | out | distance in parsecs | - |
| `d_ly` | out | distance in light-years | - |
| `mu` | out | distance modulus m - M - A | - |

**Applies when:** A >= 0; -10 < m - M - A < 60.

```shell
$ astronomy solve distance_from_distance_modulus m=10 M=0
d = 3.085677581e+19 m
d_pc = 1000
d_ly = 3261.563777
mu = 10
  by: Distance from the distance modulus  (d = 10^((m - M - A)/5 + 1) pc)
  m = 10
  M = 0
  assumes: A >= 0; -10 < m - M - A < 60
```

**What the worked example means:** This calculation determines the distance to a celestial object by comparing its apparent brightness with its intrinsic luminosity. Using the input values for apparent magnitude, absolute magnitude, and interstellar extinction, the model computes the distance modulus. The resulting distance is 1000 parsecs, which corresponds to 3261.56 light-years or 3.08568e+19 m meters. This method relies on the logarithmic relationship between magnitude differences and physical distance, providing an estimate of how far away the object is based on how much fainter it appears than it would at a standard reference distance.

<a id="flux_ratio_from_magnitudes"></a>

#### Flux ratio from a magnitude difference: `flux_ratio_from_magnitudes`

How many times brighter star 1 is than star 2: F1/F2 = 10^(-0.4 (m1 - m2)).

$$
\frac{F_1}{F_2} = 10^{-0.4\,(m_1 - m_2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | magnitude of the first object | - |
| `m2` | in | magnitude of the second object | - |
| `ratio` | out | flux ratio F1/F2 | - |
| `dm` | out | magnitude difference m1 - m2 | - |

**Applies when:** |m1 - m2| <= 100.

```shell
$ astronomy solve flux_ratio_from_magnitudes m1=1 m2=6
ratio = 100
dm = -5
  by: Flux ratio from a magnitude difference  (Pogson's scale: F1/F2 = 10^(-0.4 (m1 - m2)))
  m1 = 1
  m2 = 6
  assumes: |m1 - m2| <= 100
```

**What the worked example means:** This calculation determines the linear flux ratio between two astronomical objects using their magnitudes. By applying Pogson's scale, the system converts the logarithmic magnitude difference into a direct comparison of brightness. The result shows that for a magnitude difference of -5, the first object emits 100 times more flux than the second object.

<a id="magnitude_difference_from_flux_ratio"></a>

#### Magnitude difference from a flux ratio: `magnitude_difference_from_flux_ratio`

The magnitude difference m1 - m2 of two objects whose fluxes are in the ratio F1/F2.

$$
m_1 - m_2 = -2.5\log_{10}\frac{F_1}{F_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ratio` | in | flux ratio F1/F2 | - |
| `dm` | out | magnitude difference m1 - m2 | - |

**Applies when:** F1/F2 > 0.

```shell
$ astronomy solve magnitude_difference_from_flux_ratio ratio=100
dm = -5
  by: Magnitude difference from a flux ratio  (m1 - m2 = -2.5 log10(F1/F2))
  ratio = 100
  assumes: F1/F2 > 0
```

**What the worked example means:** This calculation determines the difference in magnitudes between two astronomical objects based on the ratio of their observed fluxes. By applying the logarithmic relationship defined by the magnitude system, an input flux ratio of 100 yields a magnitude difference of -5. This result quantifies how much fainter or brighter one object appears compared to the other, translating linear brightness measurements into the standard astronomical scale.

### Exoplanets (3)

<a id="radial_velocity_semi_amplitude"></a>

#### Radial-velocity semi-amplitude of a star: `radial_velocity_semi_amplitude`

The semi-amplitude K of the star's line-of-sight velocity wobble caused by an orbiting planet (or companion) of mass m.

$$
K = \left(\frac{2\pi G}{P}\right)^{1/3}\frac{m\sin i}{(M_\star + m)^{2/3}\sqrt{1 - e^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M_star` | in | mass of the star (Msun, kg) | kg |
| `m_p` | in | mass of the planet (Mjup, Mearth, kg) | kg |
| `P` | in | orbital period | s |
| `e` | in | eccentricity (default 0) (optional) | - |
| `i` | in | orbital inclination (default 90 degrees, edge-on) (optional) | rad |
| `K` | out | radial-velocity semi-amplitude of the star | m/s |
| `a` | out | relative semi-major axis | m |
| `a_au` | out | relative semi-major axis in au | - |

**Applies when:** 0 <= e < 0.99; 0 < i <= 180 degrees; two-body Keplerian orbit.

```shell
$ astronomy solve radial_velocity_semi_amplitude "M_star=1.988409871e+30 kg" "m_p=1.898124597e+27 kg" "P=11.86 yr"
K = 12.45977416 m/s
a = 7.782417562e+11 m
a_au = 5.202224821
  by: Radial-velocity semi-amplitude of a star  (K = (2 pi G / P)^(1/3) m sin i / ((M + m)^(2/3) sqrt(1 - e^2)))
  M_star = 1.988409871e+30 kg
  m_p = 1.898124597e+27 kg
  P = 374273136 s
  assumes: 0 <= e < 0.99; 0 < i <= 180 degrees; two-body Keplerian orbit
```

**What the worked example means:** The radial-velocity semi-amplitude is 12.4598 m/s, derived from a star of mass 1.98841e+30 kg and planet of mass 1.89812e+27 kg.

<a id="transit_depth"></a>

#### Transit depth: `transit_depth`

The fractional dimming of a uniformly bright star when a planet of radius Rp crosses it at impact parameter b (b = 0 central; grazing transits included).

$$
\delta = \frac{A_\mathrm{overlap}}{\pi R_\star^2} = \left(\frac{R_p}{R_\star}\right)^2\ \text{(full transit)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rp` | in | planet radius (Rjup, Rearth, km) | m |
| `Rs` | in | stellar radius (Rsun, km) | m |
| `b` | in | impact parameter in stellar radii (default 0) (optional) | - |
| `depth` | out | fractional flux drop | - |
| `depth_ppm` | out | depth in parts per million | - |
| `k` | out | radius ratio Rp/Rs | - |
| `grazing` | out | whether the planet's disk crosses the stellar limb | - |

**Applies when:** Rp < Rs; 0 <= b < 1 + Rp/Rs; uniform stellar disk (no limb darkening).

```shell
$ astronomy solve transit_depth "Rp=71492000 m" "Rs=695700000 m"
depth = 0.01056016944
depth_ppm = 10560.16944
k = 0.1027626851
grazing = false
  by: Transit depth  (overlapping area of the two disks divided by the stellar disk area; (Rp/Rs)^2 for a full transit)
  Rp = 71492000 m
  Rs = 695700000 m
  assumes: Rp < Rs; 0 <= b < 1 + Rp/Rs; uniform stellar disk (no limb darkening)
```

**What the worked example means:** The transit depth is 0.0105602, corresponding to a radius ratio of 0.102763.

<a id="transit_duration"></a>

#### Transit duration: `transit_duration`

The total (first to fourth contact) and full (second to third contact) durations of a transit of a planet on a circular orbit, and its impact parameter.

$$
T_{14} = \frac{P}{\pi}\arcsin\left(\frac{\sqrt{(R_\star + R_p)^2 - (bR_\star)^2}}{a\sin i}\right),\quad b = \frac{a\cos i}{R_\star}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | orbital period | s |
| `a` | in | orbit radius (au, m) | m |
| `Rs` | in | stellar radius | m |
| `Rp` | in | planet radius | m |
| `i` | in | orbital inclination (90 degrees = edge-on) | rad |
| `T14` | out | total transit duration | s |
| `T14_h` | out | total duration in hours | - |
| `T23` | out | full-transit duration (absent for a grazing transit) | s |
| `b` | out | impact parameter a cos i / Rs | - |

**Applies when:** circular orbit; Rp < Rs; a > Rs + Rp; 0 < i <= 90 degrees; b < 1 + Rp/Rs (the planet transits).

```shell
$ astronomy solve transit_duration "P=6.283185307 s" "a=2 m" "Rs=0.9 m" "Rp=0.1 m" "i=90 deg"
T14 = 1.047197551 s
T23 = 0.8230336921 s
T14_h = 0.0002908882087
b = 0
  by: Transit duration  (T14 = (P/pi) asin( sqrt((Rs + Rp)^2 - (b Rs)^2) / (a sin i) ), T23 with Rs - Rp)
  P = 6.283185307 s
  a = 2 m
  Rs = 0.9 m
  Rp = 0.1 m
  i = 1.570796327 rad
  assumes: circular orbit; Rp < Rs; a > Rs + Rp; 0 < i <= 90 degrees; b < 1 + Rp/Rs (the planet transits)
```

**What the worked example means:** This calculation determines how long a planet takes to cross the face of its star. Using an orbital period of 6.28319 s, a semi-major axis of 2 m, a stellar radius of 0.9 m, and a planet radius of 0.1 m, it computes the total time from first to fourth contact as 1.0472 s seconds, or 0.000290888 hours. It also finds the duration of the full transit phase between second and third contacts as 0.823034 s seconds. The impact parameter, which measures how centrally the planet crosses the star relative to the stellar radius, is calculated as 0. These values help constrain the geometry of the planetary system.

### Gravitational lensing (1)

<a id="einstein_radius"></a>

#### Einstein radius of a point-mass lens: `einstein_radius`

The angular Einstein radius of a point mass M at distance D_l lensing a source at distance D_s (static space: D_ls = D_s - D_l), and the corresponding radius in the lens plane.

$$
\theta_E = \sqrt{\frac{4GM}{c^2}\frac{D_{ls}}{D_l D_s}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | lens mass (Msun, kg) | kg |
| `D_l` | in | distance to the lens (pc, kpc) | m |
| `D_s` | in | distance to the source | m |
| `theta_E` | out | Einstein angle | rad |
| `theta_E_arcsec` | out | Einstein angle in arcseconds | - |
| `R_E` | out | Einstein radius in the lens plane | m |
| `R_E_au` | out | Einstein radius in au | - |

**Applies when:** D_s > D_l > 0; point mass, weak field; non-cosmological distances (D_ls = D_s - D_l).

```shell
$ astronomy solve einstein_radius "M=9.942049353e+29 kg" "D_l=1.234271033e+20 m" "D_s=2.468542065e+20 m"
theta_E = 3.458835005e-09 rad
theta_E_arcsec = 0.0007134359322
R_E = 4.269139855e+11 m
R_E_au = 2.85374373
  by: Einstein radius of a point-mass lens  (theta_E = sqrt( (4 G M / c^2) (D_s - D_l) / (D_l D_s) ))
  M = 9.942049353e+29 kg
  D_l = 1.234271033e+20 m
  D_s = 2.468542065e+20 m
  assumes: D_s > D_l > 0; point mass, weak field; non-cosmological distances (D_ls = D_s - D_l)
```

**What the worked example means:** This calculation determines the angular Einstein radius for a point-mass lens. Given a mass of 9.94205e+29 kg, a lens distance of 1.23427e+20 m, and a source distance of 2.46854e+20 m, the resulting Einstein angle is 3.45884e-09 rad. This value represents the characteristic scale at which gravitational lensing effects become significant, producing multiple images or an Einstein ring if the alignment is perfect. The physical size of this region in the lens plane is calculated as 4.26914e+11 m.

### Observing (3)

<a id="airmass_extinction"></a>

#### Airmass and atmospheric extinction: `airmass_extinction`

The airmass X = sec z of a star at zenith angle z (plane-parallel atmosphere) and, with an extinction coefficient k, the extinction k X and the magnitude corrected to outside the atmosphere.

$$
X = \sec z,\quad m_0 = m - kX
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `zen` | in | zenith angle (90 degrees minus the altitude) | rad |
| `k` | in | extinction coefficient in magnitudes per airmass (optional) (optional) | - |
| `m` | in | observed magnitude (optional; needs k) (optional) | - |
| `X` | out | airmass | - |
| `alt_deg` | out | altitude in degrees | - |
| `extinction` | out | extinction k X in magnitudes | - |
| `m0` | out | magnitude above the atmosphere m - k X | - |

**Applies when:** 0 <= z <= 85 degrees; plane-parallel atmosphere; k >= 0.

```shell
$ astronomy solve airmass_extinction "zen=60 deg"
X = 2
alt_deg = 30
  by: Airmass and atmospheric extinction  (X = 1/cos z; m0 = m - k X)
  zen = 1.047197551 rad
  assumes: 0 <= z <= 85 degrees; plane-parallel atmosphere; k >= 0
```

In the worked example, zen = 60 deg go through the equation above and give X = 2, alt_deg = 30. It applies only when 0 <= z <= 85 degrees; plane-parallel atmosphere; k >= 0: outside that it says what is wrong instead of returning a number.

<a id="angular_size"></a>

#### Angular size: `angular_size`

The angle subtended by an object of diameter D at distance d (exact: 2 atan(D / 2d)).

$$
\theta = 2\arctan\frac{D}{2d} \approx \frac{D}{d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diameter of the object | m |
| `d` | in | distance to the object | m |
| `theta` | out | angular diameter | rad |
| `theta_arcsec` | out | angular diameter in arcseconds | - |
| `theta_deg` | out | angular diameter in degrees | - |

**Applies when:** D > 0; d > 0 (distance to the object's centre plane).

```shell
$ astronomy solve angular_size "D=2 m" "d=1 m"
theta = 1.570796327 rad
theta_arcsec = 324000
theta_deg = 90
  by: Angular size  (theta = 2 atan(D / (2 d)); for D << d this is D/d radians)
  D = 2 m
  d = 1 m
  assumes: D > 0; d > 0 (distance to the object's centre plane)
```

**What the worked example means:** This calculation determines the angle an object occupies in the sky based on its physical size and distance from the observer. Using a diameter of 2 m at a distance of 1 m, the exact geometric formula yields an angular diameter of 1.5708 rad. This result is also presented as 324000 arcseconds and 90 degrees to facilitate comparison with telescope resolution limits. The small-angle approximation, where the angle is roughly the ratio of diameter to distance, holds well when the distance is significantly larger than the object's size.

<a id="linear_size_from_angle"></a>

#### Linear size from angular size: `linear_size_from_angle`

The true diameter of an object of angular size theta at distance d (exact: 2 d tan(theta/2)).

$$
D = 2d\tan\frac{\theta}{2} \approx \theta d
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta` | in | angular diameter (arcsec, arcmin, deg, rad) | rad |
| `d` | in | distance to the object | m |
| `D` | out | diameter | m |
| `D_km` | out | diameter in kilometres | - |
| `D_au` | out | diameter in astronomical units | - |

**Applies when:** 0 < theta < 180 degrees; d > 0.

```shell
$ astronomy solve linear_size_from_angle "theta=90 deg" "d=1 m"
D = 2 m
D_km = 0.002
D_au = 1.336917424e-11
  by: Linear size from angular size  (D = 2 d tan(theta/2); for small angles D = theta d)
  theta = 1.570796327 rad
  d = 1 m
  assumes: 0 < theta < 180 degrees; d > 0
```

**What the worked example means:** This calculation determines the true linear size of an astronomical object using its angular diameter and distance. By applying the small angle approximation, the result shows that an object with an angular size of 90 deg located at a distance of 1 m has a physical diameter of 2 m. This value corresponds to 0.002 in kilometres and 1.33692e-11 in astronomical units, providing a direct link between observational geometry and physical scale.

### Orbits (8)

<a id="plane_change_delta_v"></a>

#### Delta-v of a combined plane change: `plane_change_delta_v`

The velocity change needed to turn a velocity v1 into a velocity v2 at an angle di to it (a plane change, possibly combined with a speed change).

$$
\Delta v = \sqrt{v_1^2 + v_2^2 - 2 v_1 v_2\cos\Delta i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v1` | in | speed before the burn | m/s |
| `v2` | in | speed after the burn | m/s |
| `di` | in | angle between the two velocities (plane-change angle) | rad |
| `dv` | out | delta-v of the burn | m/s |

**Applies when:** v1, v2 > 0; 0 <= di <= 180 degrees; an impulsive burn.

```shell
$ astronomy solve plane_change_delta_v "v1=3 km/s" "v2=4 km/s" "di=90 deg"
dv = 5000 m/s
  by: Delta-v of a combined plane change  (law of cosines: dv^2 = v1^2 + v2^2 - 2 v1 v2 cos(di), written as (v1 - v2)^2 + 4 v1 v2 sin^2(di/2))
  v1 = 3000 m/s
  v2 = 4000 m/s
  di = 1.570796327 rad
  assumes: v1, v2 > 0; 0 <= di <= 180 degrees; an impulsive burn
```

**What the worked example means:** This calculation determines the delta-v needed to transform a spacecraft's initial speed of 3 km/s into a final speed of 4 km/s while changing its flight path by an angle of 90 deg. The result, 5000 m/s, represents the total magnitude of the impulsive burn required to achieve this combined plane and speed change.

<a id="escape_delta_v"></a>

#### Departure burn from a circular orbit: `escape_delta_v`

The velocity change that takes a spacecraft from a circular orbit of radius r onto an escape (or hyperbolic) trajectory with excess speed v_inf.

$$
\Delta v = \sqrt{v_\infty^2 + \frac{2\mu}{r}} - \sqrt{\frac{\mu}{r}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of the central body | kg |
| `r` | in | radius of the circular orbit | m |
| `v_inf` | in | hyperbolic excess speed (default 0: just escape) (optional) | m/s |
| `dv` | out | departure delta-v | m/s |
| `v_circ` | out | circular orbital speed | m/s |
| `v_depart` | out | speed needed after the burn | m/s |

**Applies when:** M, r > 0; v_inf >= 0; a single impulsive burn.

```shell
$ astronomy solve escape_delta_v "M=5.972e+24 kg" "r=6678 km"
dv = 3200.102349 m/s
v_circ = 7725.730493 m/s
v_depart = 10925.83284 m/s
  by: Departure burn from a circular orbit  (dv = sqrt(v_inf^2 + 2 mu / r) - sqrt(mu / r))
  M = 5.972e+24 kg
  r = 6678000 m
  assumes: M, r > 0; v_inf >= 0; a single impulsive burn
```

**What the worked example means:** The departure burn requires 3200.1 m/s to achieve the target excess speed from an orbit of radius 6678 km.

<a id="hohmann_transfer"></a>

#### Hohmann transfer between circular orbits: `hohmann_transfer`

The two velocity changes, their sum and the transfer time of a Hohmann transfer between coplanar circular orbits of radii r1 and r2 around a body of mass M.

$$
\Delta v_1 = \sqrt{\frac{\mu}{r_1}}\left(\sqrt{\frac{2r_2}{r_1 + r_2}} - 1\right),\ \Delta v_2 = \sqrt{\frac{\mu}{r_2}}\left(1 - \sqrt{\frac{2r_1}{r_1 + r_2}}\right),\ t = \pi\sqrt{\frac{a^3}{\mu}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r1` | in | radius of the initial circular orbit (km, au) | m |
| `r2` | in | radius of the final circular orbit | m |
| `M` | in | mass of the central body (kg, Mearth, Msun) | kg |
| `dv1` | out | first burn (departure) | m/s |
| `dv2` | out | second burn (arrival) | m/s |
| `dv` | out | total delta-v | m/s |
| `t_transfer` | out | transfer time (half the transfer ellipse) | s |
| `t_days` | out | transfer time in days | - |
| `a_transfer` | out | semi-major axis of the transfer ellipse | m |

**Applies when:** r1, r2, M > 0; r1 != r2; impulsive burns, coplanar circular orbits.

```shell
$ astronomy solve hohmann_transfer "r1=6678 km" "r2=42164 km" "M=5.972e+24 kg"
dv1 = 2425.734809 m/s
dv2 = 1466.818023 m/s
dv = 3892.552832 m/s
t_transfer = 18990.31973 s
t_days = 0.2197953672
a_transfer = 24421000 m
  by: Hohmann transfer between circular orbits  (vp = sqrt(2 mu r2 / (r1 (r1 + r2))), va = sqrt(2 mu r1 / (r2 (r1 + r2))); t = pi sqrt(a^3/mu))
  r1 = 6678000 m
  r2 = 42164000 m
  M = 5.972e+24 kg
  assumes: r1, r2, M > 0; r1 != r2; impulsive burns, coplanar circular orbits
```

**What the worked example means:** This calculation determines the fuel cost and duration for moving a spacecraft between two circular orbits. Using an initial radius of 6678 km and a final radius of 42164 km around a central mass of 5.972e+24 kg, the method computes the departure burn at 2425.73 m/s, the arrival burn at 1466.82 m/s, and their sum as the total requirement of 3892.55 m/s. The semi-major axis of the elliptical path is found to be 2.4421e+07 m, leading to a transfer time of 18990.3 s, which equals 0.219795. These values represent the minimum energy trajectory under idealized conditions.

<a id="binary_star_masses"></a>

#### Masses of a visual binary star: `binary_star_masses`

The total and individual masses of a binary star from its orbital period and each star's semi-major axis about the centre of mass (Kepler's third law and the centre-of-mass condition).

$$
M_1 + M_2 = \frac{4\pi^2 (a_1 + a_2)^3}{G P^2},\quad M_1 a_1 = M_2 a_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | orbital period (s, day, yr) | s |
| `a1` | in | semi-major axis of star 1 about the centre of mass (au, m) | m |
| `a2` | in | semi-major axis of star 2 about the centre of mass (au, m) | m |
| `M_total` | out | total mass | kg |
| `M_total_Msun` | out | total mass in solar masses | - |
| `M1_Msun` | out | mass of star 1 in solar masses | - |
| `M2_Msun` | out | mass of star 2 in solar masses | - |
| `a_au` | out | relative semi-major axis a1 + a2 in au | - |

**Applies when:** P, a1, a2 > 0; true (de-projected) semi-major axes.

```shell
$ astronomy solve binary_star_masses "P=1 yr" "a1=3.739946768e+10 m" "a2=1.12198403e+11 m"
M_total = 1.98848498e+30 kg
M_total_Msun = 1.000037773
M1_Msun = 0.75002833
M2_Msun = 0.2500094434
a_au = 0.9999999999
  by: Masses of a visual binary star  (M1 + M2 = 4 pi^2 (a1 + a2)^3 / (G P^2); M1 a1 = M2 a2)
  P = 31557600 s
  a1 = 3.739946768e+10 m
  a2 = 1.12198403e+11 m
  assumes: P, a1, a2 > 0; true (de-projected) semi-major axes
```

**What the worked example means:** This calculation determines the total and individual masses of a visual binary star using its orbital period and the semi-major axes of each component about the center of mass. By applying Kepler's third law, the sum of the masses is found to be 1.98848e+30 kg. The relative semi-major axis is 1. Using the center-of-mass condition, the individual masses are derived as 0.750028 for the first star and 0.250009 for the second star in solar units.

<a id="orbit_from_state"></a>

#### Orbital elements from position and velocity, `orbit_from_state`

The semi-major axis, eccentricity, periapsis and apoapsis distances, period and specific energy of a bound orbit from the distance, speed and flight-path angle at one point.

$$
a = \frac{r}{2 - r v^2/\mu},\quad e = \sqrt{1 - \frac{(r v\cos\gamma)^2}{\mu a}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of the central body | kg |
| `r` | in | distance from the centre of the central body | m |
| `v` | in | orbital speed at that point | m/s |
| `gamma` | in | flight-path angle above the local horizontal (default 0) (optional) | rad |
| `a` | out | semi-major axis | m |
| `e` | out | eccentricity | - |
| `rp` | out | periapsis distance | m |
| `ra` | out | apoapsis distance | m |
| `P` | out | orbital period | s |
| `energy` | out | specific orbital energy (J/kg) | J/kg |

**Applies when:** v^2 < 2 G M / r (bound orbit); |gamma| < 90 degrees; two-body motion.

```shell
$ astronomy solve orbit_from_state "M=1.4982e+10 kg" "r=1 m" "v=1.2 m/s"
a = 1.785973198 m
e = 0.4400811831
rp = 1 m
ra = 2.571946396 m
P = 14.9970043 s
energy = -0.279943626 J/kg
  by: Orbital elements from position and velocity  (vis-viva a = r / (2 - r v^2/mu); e^2 = (s cos^2 g - 1)^2 + (s cos g sin g)^2 with s = r v^2/mu)
  M = 1.4982e+10 kg
  r = 1 m
  v = 1.2 m/s
  assumes: v^2 < 2 G M / r (bound orbit); |gamma| < 90 degrees; two-body motion
```

**What the worked example means:** This calculation determines the shape and size of a bound orbit using the central body's mass, current distance, speed, and flight-path angle. The resulting semi-major axis is 1.78597 m, which defines the orbit's scale, while the eccentricity of 0.440081 describes how elongated the path is. These values allow practitioners to understand the satellite's trajectory without needing continuous tracking data.

<a id="kepler_position_at_time"></a>

#### Position on an elliptical orbit at a given time (Kepler's equation), `kepler_position_at_time`

The mean, eccentric and true anomalies, the distance and the speed of a body on an elliptical orbit a time t after periapsis passage.

$$
M = E - e\sin E,\quad \tan\frac{\nu}{2} = \sqrt{\frac{1+e}{1-e}}\tan\frac{E}{2},\quad r = a(1 - e\cos E)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of the central body | kg |
| `a` | in | semi-major axis | m |
| `e` | in | eccentricity (0 <= e < 0.99) | - |
| `t` | in | time since periapsis passage | s |
| `mean_anomaly` | out | mean anomaly (degrees) | - |
| `E` | out | eccentric anomaly (degrees) | - |
| `nu` | out | true anomaly (degrees, 0 to 360) | - |
| `r` | out | distance from the central body's centre | m |
| `v` | out | orbital speed | m/s |
| `P` | out | orbital period | s |

**Applies when:** 0 <= e < 0.99; t >= 0 (reduced modulo the period); two-body motion.

```shell
$ astronomy solve kepler_position_at_time "M=1.4982e+10 kg" "a=1 m" e=0.5 "t=3.141592654 s"
mean_anomaly = 179.9949263
E = 179.9966175
nu = 179.9980471
r = 1.499999999 m
v = 0.5773339959 m/s
P = 6.283362419 s
  by: Position on an elliptical orbit at a given time (Kepler's equation)  (Kepler's equation M = E - e sin E solved by safeguarded Newton iteration; tan(nu/2) = sqrt((1+e)/(1-e)) tan(E/2); r = a(1 - e cos E))
  M = 1.4982e+10 kg
  a = 1 m
  e = 0.5
  t = 3.141592654 s
  assumes: 0 <= e < 0.99; t >= 0 (reduced modulo the period); two-body motion
```

**What the worked example means:** This calculation determines the position and velocity of an object on an elliptical orbit at a specific time. Using the semi-major axis, eccentricity, central mass, and elapsed time since closest approach, it solves Kepler's equation to find the mean anomaly, eccentric anomaly, and true anomaly. The resulting distance from the center is 1.5 m meters, the orbital speed is 0.577334 m/s meters per second, and the period is 6.28336 s seconds.

<a id="apsis_speeds"></a>

#### Speeds at periapsis and apoapsis: `apsis_speeds`

The orbital speeds at the closest and farthest points of an elliptical orbit, with its semi-major axis, eccentricity and period.

$$
v_p = \sqrt{\frac{2\mu\,r_a}{r_p (r_p + r_a)}},\quad v_a = \sqrt{\frac{2\mu\,r_p}{r_a (r_p + r_a)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rp` | in | periapsis distance from the centre of the central body | m |
| `ra` | in | apoapsis distance | m |
| `M` | in | mass of the central body | kg |
| `vp` | out | speed at periapsis | m/s |
| `va` | out | speed at apoapsis | m/s |
| `a` | out | semi-major axis | m |
| `e` | out | eccentricity | - |
| `P` | out | orbital period | s |

**Applies when:** 0 < rp < ra; M > 0; two-body orbit, the orbiting mass negligible.

```shell
$ astronomy solve apsis_speeds "rp=1 m" "ra=3 m" "M=1.4982e+10 kg"
vp = 1.224710349 m/s
va = 0.408236783 m/s
a = 2 m
e = 0.5
P = 17.7720327 s
  by: Speeds at periapsis and apoapsis  (energy and angular momentum: vp = sqrt(2 mu ra / (rp (rp + ra))), va = vp rp / ra)
  rp = 1 m
  ra = 3 m
  M = 1.4982e+10 kg
  assumes: 0 < rp < ra; M > 0; two-body orbit, the orbiting mass negligible
```

**What the worked example means:** This calculation determines the instantaneous velocities of an object in an elliptical orbit at its closest approach, known as periapsis, and its farthest point, called apoapsis. Using the given distances from the central body's center, specifically the periapsis distance of 1 m and the apoapsis distance of 3 m, along with the central mass of 1.4982e+10 kg, gives the speeds. The resulting velocity at periapsis is 1.22471 m/s, while the slower velocity at apoapsis is 0.408237 m/s. These values also define the orbit's geometry, yielding a semi-major axis of 2 m and an eccentricity of 0.5.

<a id="time_of_flight"></a>

#### Time of flight between two points of an elliptical orbit, `time_of_flight`

The time a body takes to move forward along an elliptical orbit from true anomaly nu1 to true anomaly nu2.

$$
\Delta t = \frac{(E_2 - e\sin E_2) - (E_1 - e\sin E_1)}{n},\quad n = \sqrt{\mu/a^3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of the central body | kg |
| `a` | in | semi-major axis | m |
| `e` | in | eccentricity (0 <= e < 0.99) | - |
| `nu1` | in | starting true anomaly | rad |
| `nu2` | in | final true anomaly | rad |
| `dt` | out | time of flight | s |
| `dt_days` | out | time of flight in days | - |
| `P` | out | orbital period | s |

**Applies when:** 0 <= e < 0.99; nu1 != nu2; motion in the forward (prograde) direction.

```shell
$ astronomy solve time_of_flight "M=1.4982e+10 kg" "a=1 m" e=0.5 "nu1=0 deg" "nu2=180 deg"
dt = 3.141681209 s
dt_days = 3.636205103e-05
P = 6.283362419 s
  by: Time of flight between two points of an elliptical orbit  (eccentric anomalies from the true anomalies, mean anomalies M = E - e sin E, dt = (M2 - M1)/n)
  M = 1.4982e+10 kg
  a = 1 m
  e = 0.5
  nu1 = 0 rad
  nu2 = 3.141592654 rad
  assumes: 0 <= e < 0.99; nu1 != nu2; motion in the forward (prograde) direction
```

**What the worked example means:** This calculation determines the duration required for a body to travel along its elliptical orbit from the starting true anomaly of 0 deg to the final true anomaly of 180 deg. By converting these positions into mean anomalies and dividing by the mean motion derived from the central mass 1.4982e+10 kg and semi-major axis 1 m, the model computes a time of flight of 3.14168 s. This result indicates how long the journey takes in seconds, or equivalently 3.63621e-05 days, relative to the full orbital cycle of 6.28336 s.

### Planets (2)

<a id="planet_equilibrium_temperature"></a>

#### Equilibrium temperature of a planet: `planet_equilibrium_temperature`

The equilibrium temperature of a rapidly rotating planet of Bond albedo A at distance a from a star of temperature T and radius R, and the stellar flux it receives.

$$
T_\mathrm{eq} = T_\star\sqrt{\frac{R_\star}{2a}}\,(1 - A)^{1/4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T_star` | in | effective temperature of the star | K |
| `R_star` | in | radius of the star (m, Rsun) | m |
| `a` | in | orbital distance (m, au) | m |
| `A` | in | Bond albedo (0 to 1, default 0) (optional) | - |
| `T_eq` | out | equilibrium temperature | K |
| `S` | out | stellar flux at the planet | W/m^2 |

**Applies when:** 0 <= A < 1; a > R_star; full heat redistribution, unit emissivity.

```shell
$ astronomy solve planet_equilibrium_temperature "T_star=1000 K" "R_star=2 m" "a=4 m"
T_eq = 500 K
S = 14175.93605 W/m^2
  by: Equilibrium temperature of a planet  ((1 - A) S pi Rp^2 = 4 pi Rp^2 sigma T^4 with S = sigma T*^4 (R*/a)^2: T = T* sqrt(R*/(2a)) (1 - A)^(1/4))
  T_star = 1000 K
  R_star = 2 m
  a = 4 m
  assumes: 0 <= A < 1; a > R_star; full heat redistribution, unit emissivity
```

**What the worked example means:** This calculation determines the equilibrium temperature of a planet, which represents its average surface temperature assuming it absorbs and re-radiates energy efficiently. The result depends on the star's effective temperature, radius, the planet's orbital distance, and its Bond albedo. A higher stellar output or closer orbit increases the received flux, while a higher albedo reflects more light, lowering the temperature. For this specific case, the computed equilibrium temperature is 500 K, corresponding to an incident stellar flux of 14175.9 W/m^2.

<a id="atmospheric_scale_height"></a>

#### Scale height of an isothermal atmosphere: `atmospheric_scale_height`

The pressure scale height H = R T / (mu g) of an isothermal planetary atmosphere: pressure falls by a factor e every H.

$$
H = \frac{RT}{\mu g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature of the atmosphere | K |
| `mu` | in | mean molar mass (kg/mol or g/mol) | kg/mol |
| `g` | in | surface gravity | m/s^2 |
| `H` | out | scale height | m |
| `H_km` | out | scale height in kilometres | - |

**Applies when:** T, mu, g > 0; isothermal ideal gas, constant gravity.

```shell
$ astronomy solve atmospheric_scale_height "T=288 K" "mu=28.97 g/mol" "g=9.81 m/s^2"
H = 8425.761664 m
H_km = 8.425761664
  by: Scale height of an isothermal atmosphere  (H = R T / (mu g))
  T = 288 K
  mu = 0.02897 kg/mol
  g = 9.81 m/s^2
  assumes: T, mu, g > 0; isothermal ideal gas, constant gravity
```

**What the worked example means:** This calculation determines the vertical distance over which atmospheric pressure decreases by a factor of e, known as the scale height. Using the temperature 288 K, mean molar mass 28.97 g/mol, and surface gravity 9.81 m/s^2, the result is 8425.76 m. This value indicates how thick the planetary atmosphere is and how rapidly density drops with altitude under the assumption of an isothermal ideal gas.

### Plasma physics (12)

<a id="alfven_speed"></a>

#### Alfven speed: `alfven_speed`

The Alfven speed of a fully ionized hydrogen plasma (n protons and n electrons per unit volume) in a magnetic field B.

$$
v_A = \frac{B}{\sqrt{\mu_0\,n\,(m_p + m_e)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic field strength (T, nT, gauss) | T |
| `n` | in | proton number density | 1/m^3 |
| `v_A` | out | Alfven speed | m/s |
| `v_A_km_s` | out | Alfven speed in km/s | - |

**Applies when:** B, n > 0; mass density n (m_p + m_e); non-relativistic (v_A << c).

```shell
$ astronomy solve alfven_speed "B=5 nT" "n=5 cm^-3"
v_A = 48759.91586 m/s
v_A_km_s = 48.75991586
  by: Alfven speed  (v_A = B / sqrt(mu0 n (m_p + m_e)))
  B = 5e-09 T
  n = 5000000 1/m^3
  assumes: B, n > 0; mass density n (m_p + m_e); non-relativistic (v_A << c)
```

**What the worked example means:** This calculation determines the characteristic propagation speed of magnetic disturbances through a fully ionized hydrogen plasma. By using the provided magnetic field strength 5 nT and proton number density 5 cm^-3, the model computes the Alfven speed as 48759.9 m/s. This value, also expressed as 48.7599, represents how quickly information travels along magnetic field lines in this specific environment.

<a id="debye_length"></a>

#### Debye length: `debye_length`

The electron Debye screening length of a plasma of electron temperature T and electron density n.

$$
\lambda_D = \sqrt{\frac{\varepsilon_0 k_B T_e}{n_e e^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | electron temperature | K |
| `n` | in | electron density (m^-3, cm^-3) | 1/m^3 |
| `lambda_D` | out | Debye length | m |
| `lambda_D_nm` | out | Debye length in nanometres | - |

**Applies when:** T, n > 0; electron screening only.

```shell
$ astronomy solve debye_length "T=100000 K" "n=1e+18 m^-3"
lambda_D = 2.18225558e-05 m
lambda_D_nm = 21822.5558
  by: Debye length  (lambda_D = sqrt(eps0 k T / (n e^2)))
  T = 100000 K
  n = 1e+18 1/m^3
  assumes: T, n > 0; electron screening only
```

**What the worked example means:** This calculation determines the electron Debye screening length, which represents the distance over which electric fields are neutralized in a plasma. Using an electron temperature of 100000 K and an electron density of 1e+18 m^-3, the resulting screening scale is 2.18226e-05 m. This value defines the threshold for collective behavior, distinguishing a true plasma from a simple gas of charged particles where individual interactions dominate.

<a id="gyrofrequency"></a>

#### Gyrofrequency (cyclotron frequency): `gyrofrequency`

The angular and ordinary gyrofrequency and the gyration period of an electron or proton in a magnetic field B.

$$
\omega_c = \frac{|q|B}{m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic field strength (T, nT, gauss) | T |
| `particle` | in | particle species: electron or proton (default electron) (optional) | - |
| `omega_c` | out | angular gyrofrequency | rad/s |
| `f_c` | out | gyrofrequency | Hz |
| `T_c` | out | gyration period | s |

**Applies when:** B > 0; non-relativistic particle.

```shell
$ astronomy solve gyrofrequency "B=1 T" particle=proton
omega_c = 95788331.43 rad/s
f_c = 15245186.44 Hz
T_c = 6.559447496e-08 s
  by: Gyrofrequency (cyclotron frequency)  (omega_c = |q| B / m)
  B = 1 T
  particle = proton
  assumes: B > 0; non-relativistic particle
```

**What the worked example means:** This calculation determines the rate at which a charged particle orbits magnetic field lines. Using the magnetic field strength 1 T and the properties of the selected particle gives the angular gyrofrequency 9.57883e+07 rad/s in radians per second, the standard frequency 1.52452e+07 Hz in hertz, and the gyration period 6.55945e-08 s in seconds. These values describe how tightly and quickly the particle spirals around the field.

<a id="gyroradius"></a>

#### Gyroradius (Larmor radius): `gyroradius`

The radius of the circular motion of an electron or proton moving with perpendicular speed v_perp in a magnetic field B.

$$
r_L = \frac{m v_\perp}{|q|B}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic field strength | T |
| `v_perp` | in | speed perpendicular to the field | m/s |
| `particle` | in | particle species: electron or proton (default electron) (optional) | - |
| `r_L` | out | gyroradius | m |
| `r_L_km` | out | gyroradius in kilometres | - |
| `r_L_nm` | out | gyroradius in nanometres | - |

**Applies when:** B, v_perp > 0; v_perp < 0.1 c (non-relativistic).

```shell
$ astronomy solve gyroradius "B=5 nT" "v_perp=400 km/s" particle=proton
r_L = 835174.7943 m
r_L_km = 835.1747943
r_L_nm = 8.351747943e+14
  by: Gyroradius (Larmor radius)  (r_L = m v_perp / (|q| B))
  B = 5e-09 T
  v_perp = 400000 m/s
  particle = proton
  assumes: B, v_perp > 0; v_perp < 0.1 c (non-relativistic)
```

**What the worked example means:** This calculation determines the gyroradius, representing the radius of the circular path a charged particle follows when moving perpendicular to a magnetic field. Using the input values for magnetic field strength and speed perpendicular to the field, along with the specified particle species, the result shows how tightly the particle orbits. The computed gyroradius is 835175 m. This value indicates the spatial scale of the particle's motion, which is essential for understanding plasma confinement and energy transfer mechanisms in astronomical environments.

<a id="inertial_length"></a>

#### Inertial length (skin depth): `inertial_length`

The electron (or proton) inertial length c / omega_p, the collisionless skin depth of a plasma.

$$
d = \frac{c}{\omega_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number density | 1/m^3 |
| `particle` | in | particle species: electron or proton (default electron) (optional) | - |
| `d` | out | inertial length | m |
| `d_km` | out | inertial length in kilometres | - |

**Applies when:** n > 0.

```shell
$ astronomy solve inertial_length "n=5 cm^-3" particle=proton
d = 101835.3511 m
d_km = 101.8353511
  by: Inertial length (skin depth)  (d = c / omega_p = c sqrt(eps0 m / (n e^2)))
  n = 5000000 1/m^3
  particle = proton
  assumes: n > 0
```

**What the worked example means:** This calculation determines the collisionless skin depth, or inertial length, of the specified particle species based on its number density. The result, 101835 m, represents the characteristic scale over which magnetic fields penetrate the plasma due to particle inertia rather than resistance. For the given density of 5 cm^-3, the calculated length is 101.835 when expressed in kilometres, providing a fundamental spatial scale for analyzing plasma dynamics and wave propagation.

<a id="lower_hybrid_frequency"></a>

#### Lower hybrid frequency: `lower_hybrid_frequency`

The lower hybrid frequency of a magnetized electron-proton plasma.

$$
\frac{1}{\omega_{lh}^2} = \frac{1}{\omega_{ci}\,\omega_{ce}} + \frac{1}{\omega_{pi}^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic field strength | T |
| `n` | in | density (electrons = protons) | 1/m^3 |
| `omega_lh` | out | lower hybrid angular frequency | rad/s |
| `f_lh` | out | lower hybrid frequency | Hz |

**Applies when:** B, n > 0; cold quasi-neutral electron-proton plasma.

```shell
$ astronomy solve lower_hybrid_frequency "B=1 T" "n=1e+18 m^-3"
omega_lh = 1253639221 rad/s
f_lh = 199522878.9 Hz
  by: Lower hybrid frequency  (1/omega_lh^2 = 1/(omega_ci omega_ce) + 1/omega_pi^2)
  B = 1 T
  n = 1e+18 1/m^3
  assumes: B, n > 0; cold quasi-neutral electron-proton plasma
```

**What the worked example means:** This calculation determines the lower hybrid frequency for a cold, quasi-neutral electron-proton plasma using the magnetic field strength 1 T and particle density 1e+18 m^-3. The result shows that the characteristic oscillation occurs at 1.99523e+08 Hz, corresponding to an angular frequency of 1.25364e+09 rad/s. This value represents a key resonance where both electrons and ions participate in wave motion, which is critical for understanding energy transfer in magnetized plasmas.

<a id="magnetic_pressure"></a>

#### Magnetic pressure: `magnetic_pressure`

The magnetic pressure (equal to the magnetic energy density) of a field B in vacuum or a plasma.

$$
p_B = \frac{B^2}{2\mu_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic field strength (T, nT, gauss) | T |
| `p_B` | out | magnetic pressure | Pa |
| `energy_density` | out | magnetic energy density (J/m^3) | Pa |
| `p_B_nPa` | out | magnetic pressure in nanopascals | - |

**Applies when:** B > 0.

```shell
$ astronomy solve magnetic_pressure "B=1 T"
p_B = 397887.3578 Pa
energy_density = 397887.3578 Pa
p_B_nPa = 3.978873578e+14
  by: Magnetic pressure  (p_B = B^2 / (2 mu0))
  B = 1 T
  assumes: B > 0
```

**What the worked example means:** This calculation determines the pressure exerted by a magnetic field, which is numerically equal to its energy density. Using a field strength of 1 T, the resulting magnetic pressure is 397887 Pa. This value represents how strongly the field pushes against surrounding plasma, expressed also as 397887 Pa in energy units and 3.97887e+14 in nanopascals for finer scale comparisons.

<a id="debye_number"></a>

#### Number of particles in a Debye sphere: `debye_number`

The number of electrons in a sphere of one Debye length radius (N_D >> 1 means the gas behaves as a plasma).

$$
N_D = \frac{4\pi}{3}n_e\lambda_D^3
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | electron temperature | K |
| `n` | in | electron density | 1/m^3 |
| `N_D` | out | particles per Debye sphere | - |
| `lambda_D` | out | Debye length | m |

**Applies when:** T, n > 0.

```shell
$ astronomy solve debye_number "T=1000000 K" "n=1e+15 m^-3"
N_D = 43531681.94
lambda_D = 0.00218225558 m
  by: Number of particles in a Debye sphere  (N_D = (4 pi / 3) n lambda_D^3)
  T = 1000000 K
  n = 1e+15 1/m^3
  assumes: T, n > 0
```

**What the worked example means:** This calculation determines the number of electrons within a sphere whose radius equals one Debye length. Using an electron temperature of 1e+06 K and an electron density of 1e+15 m^-3, the model computes a Debye length of 0.00218226 m meters and a resulting particle count of 4.35317e+07. A value significantly greater than one confirms that collective plasma effects dominate over binary collisions.

<a id="plasma_beta"></a>

#### Plasma beta: `plasma_beta`

The ratio of the thermal pressure n k T of one particle species to the magnetic pressure B^2/(2 mu0).

$$
\beta = \frac{n k_B T}{B^2/(2\mu_0)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature of the species | K |
| `n` | in | number density of the species | 1/m^3 |
| `B` | in | magnetic field strength | T |
| `beta` | out | plasma beta | - |
| `p_thermal` | out | thermal pressure n k T | Pa |
| `p_magnetic` | out | magnetic pressure | Pa |

**Applies when:** T, n, B > 0; one species (add species' pressures for a total beta).

```shell
$ astronomy solve plasma_beta "T=100000 K" "n=5 cm^-3" "B=5 nT"
beta = 0.6939898808
p_thermal = 6.903245e-12 Pa
p_magnetic = 9.947183945e-12 Pa
  by: Plasma beta  (beta = n k T / (B^2 / (2 mu0)))
  T = 100000 K
  n = 5000000 1/m^3
  B = 5e-09 T
  assumes: T, n, B > 0; one species (add species' pressures for a total beta)
```

**What the worked example means:** This calculation determines the plasma beta, which represents the ratio of the thermal pressure exerted by particles to the magnetic pressure exerted by the field. Using a temperature of 100000 K, a number density of 5 cm^-3, and a magnetic field strength of 5 nT, the resulting thermal pressure is 6.90325e-12 Pa and the magnetic pressure is 9.94718e-12 Pa. The final dimensionless value of 0.69399 indicates whether the plasma dynamics are dominated by particle motion or magnetic confinement.

<a id="plasma_frequency"></a>

#### Plasma frequency: `plasma_frequency`

The plasma (Langmuir) oscillation frequency of electrons (or protons) of number density n.

$$
\omega_p = \sqrt{\frac{n e^2}{\varepsilon_0 m}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number density (m^-3, cm^-3) | 1/m^3 |
| `particle` | in | particle species: electron or proton (default electron) (optional) | - |
| `omega_p` | out | plasma angular frequency | rad/s |
| `f_p` | out | plasma frequency | Hz |

**Applies when:** n > 0; cold, unmagnetized plasma; singly charged particles.

```shell
$ astronomy solve plasma_frequency "n=1e+12 m^-3"
omega_p = 56414602.25 rad/s
f_p = 8978662.811 Hz
  by: Plasma frequency  (omega_p = sqrt(n e^2 / (eps0 m)))
  n = 1e+12 1/m^3
  assumes: n > 0; cold, unmagnetized plasma; singly charged particles
```

**What the worked example means:** This calculation determines the natural oscillation rate of charged particles in a plasma based on their number density. Given an input density of 1e+12 m^-3, the resulting plasma angular frequency is 5.64146e+07 rad/s and the standard frequency is 8.97866e+06 Hz. This value represents how quickly electrons or protons can collectively respond to electric disturbances, which is crucial for understanding wave propagation limits in space environments.

<a id="thermal_speed"></a>

#### Thermal speed of plasma particles: `thermal_speed`

The thermal speed of electrons or protons of temperature T under a stated convention: most probable sqrt(2kT/m), root-mean-square sqrt(3kT/m) or mean sqrt(8kT/(pi m)).

$$
v_\mathrm{mp} = \sqrt{\frac{2kT}{m}},\ v_\mathrm{rms} = \sqrt{\frac{3kT}{m}},\ \bar v = \sqrt{\frac{8kT}{\pi m}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `convention` | in | which thermal speed: most_probable, rms or mean | - |
| `particle` | in | particle species: electron or proton (default electron) (optional) | - |
| `v_th` | out | thermal speed | m/s |

**Applies when:** T > 0; Maxwellian distribution in three dimensions; the convention must be stated.

```shell
$ astronomy solve thermal_speed "T=1000000 K" convention=rms particle=proton
v_th = 157363.2716 m/s
  by: Thermal speed of plasma particles  (v = sqrt(c k T / m) with c = 2 (most probable), 3 (rms), 8/pi (mean))
  T = 1000000 K
  convention = rms
  particle = proton
  assumes: T > 0; Maxwellian distribution in three dimensions; the convention must be stated
```

**What the worked example means:** This calculation determines the characteristic velocity of plasma particles, specifically the proton, based on their temperature. By applying the rms definition to the Maxwellian distribution formula, the model computes a thermal speed of 157363 m/s. This value represents the typical kinetic energy scale of the particles in the gas, which is essential for understanding collision frequencies and pressure dynamics in astrophysical plasmas.

<a id="upper_hybrid_frequency"></a>

#### Upper hybrid frequency: `upper_hybrid_frequency`

The upper hybrid frequency of a magnetized electron plasma: sqrt(omega_pe^2 + omega_ce^2).

$$
\omega_{uh} = \sqrt{\omega_{pe}^2 + \omega_{ce}^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic field strength | T |
| `n` | in | electron density | 1/m^3 |
| `omega_uh` | out | upper hybrid angular frequency | rad/s |
| `f_uh` | out | upper hybrid frequency | Hz |

**Applies when:** B, n > 0; cold plasma, waves perpendicular to B.

```shell
$ astronomy solve upper_hybrid_frequency "B=1 T" "n=1e+18 m^-3"
omega_uh = 1.847081091e+11 rad/s
f_uh = 2.939720859e+10 Hz
  by: Upper hybrid frequency  (omega_uh = sqrt(omega_pe^2 + omega_ce^2))
  B = 1 T
  n = 1e+18 1/m^3
  assumes: B, n > 0; cold plasma, waves perpendicular to B
```

**What the worked example means:** This calculation determines the upper hybrid resonance frequency for a magnetized electron plasma using the magnetic field strength 1 T and electron density 1e+18 m^-3. The resulting upper hybrid angular frequency is 1.84708e+11 rad/s, which translates to a standard frequency of 2.93972e+10 Hz. This value represents the characteristic resonance where electromagnetic waves propagating perpendicular to the magnetic field are reflected or absorbed in a cold plasma environment.

### Positional astronomy (5)

<a id="altaz_from_hadec"></a>

#### Altitude and azimuth from hour angle and declination, `altaz_from_hadec`

The altitude and azimuth of a star of declination dec at hour angle H seen from latitude lat (horizontal coordinates from equatorial ones).

$$
\sin a = \sin\phi\sin\delta + \cos\phi\cos\delta\cos H,\quad \tan A = \frac{-\cos\delta\sin H}{\sin\delta\cos\phi - \cos\delta\sin\phi\cos H}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H` | in | hour angle (hours or degrees; positive west of the meridian) | rad |
| `dec` | in | declination | rad |
| `lat` | in | observer's latitude | rad |
| `alt_deg` | out | altitude in degrees (geometric, no refraction) | - |
| `az_deg` | out | azimuth in degrees from north through east | - |

**Applies when:** |dec| <= 90 degrees; |lat| <= 90 degrees; not at the zenith or nadir; no refraction.

```shell
$ astronomy solve altaz_from_hadec "H=0 deg" "dec=0 deg" "lat=45 deg"
alt_deg = 45
az_deg = 180
  by: Altitude and azimuth from hour angle and declination  (sin a = sin(lat) sin(dec) + cos(lat) cos(dec) cos H; tan A = -cos(dec) sin H / (sin(dec) cos(lat) - cos(dec) sin(lat) cos H))
  H = 0 rad
  dec = 0 rad
  lat = 0.7853981634 rad
  assumes: |dec| <= 90 degrees; |lat| <= 90 degrees; not at the zenith or nadir; no refraction
```

**What the worked example means:** This calculation converts equatorial coordinates into horizontal coordinates for a specific observer location. Using the input hour angle 0 deg, declination 0 deg, and latitude 45 deg, the method determines the geometric position of the celestial object in the local sky. The resulting altitude of 45 indicates how high the object appears above the horizon, while the azimuth of 180 specifies its direction measured clockwise from north through east. These values allow practitioners to determine where to point instruments or predict rise and set times.

<a id="culmination_altitude"></a>

#### Altitude of a star at culmination: `culmination_altitude`

The altitudes of a star when it crosses the meridian above the pole (upper culmination) and below it (lower culmination), and whether it is circumpolar or never rises.

$$
a_\mathrm{upper} = 90^\circ - |\phi - \delta|,\quad a_\mathrm{lower} = |\phi + \delta| - 90^\circ
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dec` | in | declination | rad |
| `lat` | in | observer's latitude | rad |
| `alt_upper_deg` | out | altitude at upper culmination (degrees) | - |
| `alt_lower_deg` | out | altitude at lower culmination (degrees) | - |
| `circumpolar` | out | never sets | - |
| `never_rises` | out | never rises | - |

**Applies when:** |dec|, |lat| <= 90 degrees; geometric horizon, no refraction.

```shell
$ astronomy solve culmination_altitude "dec=20 deg" "lat=50 deg"
alt_upper_deg = 60
alt_lower_deg = -20
circumpolar = false
never_rises = false
  by: Altitude of a star at culmination  (upper: a = 90 - |lat - dec|; lower: a = |lat + dec| - 90)
  dec = 0.3490658504 rad
  lat = 0.872664626 rad
  assumes: |dec|, |lat| <= 90 degrees; geometric horizon, no refraction
```

**What the worked example means:** This calculation determines the highest and lowest points a star reaches in the sky for an observer at latitude 50 deg viewing a star with declination 20 deg. The upper culmination altitude, which is the maximum height above the horizon when the star crosses the meridian, is computed as 60. The lower culmination altitude, representing the minimum height if the star remains visible below the pole, is -20. These values help identify whether the star is circumpolar, indicated by no, or if it never rises, indicated by no.

<a id="angular_separation"></a>

#### Angular separation of two sky positions: `angular_separation`

The great-circle angle between two objects given by their right ascensions and declinations.

$$
\cos\theta = \sin\delta_1\sin\delta_2 + \cos\delta_1\cos\delta_2\cos(\alpha_2 - \alpha_1)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ra1` | in | right ascension of object 1 (hours or degrees) | rad |
| `dec1` | in | declination of object 1 | rad |
| `ra2` | in | right ascension of object 2 | rad |
| `dec2` | in | declination of object 2 | rad |
| `sep_deg` | out | separation in degrees | - |
| `sep_arcsec` | out | separation in arcseconds | - |

**Applies when:** |dec| <= 90 degrees.

```shell
$ astronomy solve angular_separation "ra1=0 deg" "dec1=0 deg" "ra2=90 deg" "dec2=0 deg"
sep_deg = 90
sep_arcsec = 324000
  by: Angular separation of two sky positions  (Vincenty's formula (accurate at all separations))
  ra1 = 0 rad
  dec1 = 0 rad
  ra2 = 1.570796327 rad
  dec2 = 0 rad
  assumes: |dec| <= 90 degrees
```

**What the worked example means:** This calculation determines the great-circle distance on the celestial sphere between two objects defined by their right ascensions and declinations. Using the provided coordinates for object one (0 deg, 0 deg) and object two (90 deg, 0 deg), the algorithm computes the angle subtended at the observer's position. The resulting separation of 90 degrees indicates how far apart these objects appear in the sky, which is essential for identifying binary systems or measuring distances between stars and galaxies without accounting for physical depth.

<a id="position_angle"></a>

#### Position angle between two sky positions: `position_angle`

The position angle of object 2 as seen from object 1, measured from north through east.

$$
\tan\mathrm{PA} = \frac{\sin\Delta\alpha\cos\delta_2}{\cos\delta_1\sin\delta_2 - \sin\delta_1\cos\delta_2\cos\Delta\alpha}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ra1` | in | right ascension of object 1 (hours or degrees) | rad |
| `dec1` | in | declination of object 1 | rad |
| `ra2` | in | right ascension of object 2 | rad |
| `dec2` | in | declination of object 2 | rad |
| `pa_deg` | out | position angle in degrees (0 = north, 90 = east) | - |

**Applies when:** |dec| < 90 degrees; distinct, non-antipodal positions.

```shell
$ astronomy solve position_angle "ra1=0 deg" "dec1=0 deg" "ra2=1 deg" "dec2=0 deg"
pa_deg = 90
  by: Position angle between two sky positions  (tan PA = sin(dra) cos(dec2) / (cos(dec1) sin(dec2) - sin(dec1) cos(dec2) cos(dra)))
  ra1 = 0 rad
  dec1 = 0 rad
  ra2 = 0.01745329252 rad
  dec2 = 0 rad
  assumes: |dec| < 90 degrees; distinct, non-antipodal positions
```

**What the worked example means:** This calculation determines the orientation of one celestial object relative to another on the sky. It measures the angle starting from north and moving eastward, which is essential for describing binary star orbits or planetary positions. Practitioners use this to align instruments and interpret observational data accurately. The result is 90.

<a id="rise_set_hour_angle"></a>

#### Rising and setting of a star: `rise_set_hour_angle`

The hour angle at which a star sets, how long it stays above the horizon (sidereal hours) and the azimuths at which it rises and sets (geometric horizon, no refraction).

$$
\cos H_0 = -\tan\phi\tan\delta,\quad \cos A_\mathrm{rise} = \frac{\sin\delta}{\cos\phi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dec` | in | declination | rad |
| `lat` | in | observer's latitude | rad |
| `H_set_h` | out | hour angle at setting (hours) | - |
| `time_up_sidereal_h` | out | time above the horizon (sidereal hours) | - |
| `az_rise_deg` | out | azimuth of rising (degrees from north through east) | - |
| `az_set_deg` | out | azimuth of setting | - |

**Applies when:** the star rises and sets: |tan(lat) tan(dec)| < 1; geometric horizon (no refraction, no star size).

```shell
$ astronomy solve rise_set_hour_angle "dec=0 deg" "lat=45 deg"
H_set_h = 6
time_up_sidereal_h = 12
az_rise_deg = 90
az_set_deg = 270
  by: Rising and setting of a star  (cos H0 = -tan(lat) tan(dec); cos A_rise = sin(dec) / cos(lat))
  dec = 0 rad
  lat = 0.7853981634 rad
  assumes: the star rises and sets: |tan(lat) tan(dec)| < 1; geometric horizon (no refraction, no star size)
```

**What the worked example means:** This calculation determines the geometric rising and setting points of a celestial object based on its declination 0 deg and the observer's latitude 45 deg. The resulting hour angle at setting is 6, which corresponds to a total duration above the horizon of 12. These values indicate where the object appears on the horizon and how long it is visible, assuming no atmospheric refraction or terrain obstruction.

### Stars (8)

<a id="bolometric_magnitude_from_luminosity"></a>

#### Absolute bolometric magnitude from luminosity, `bolometric_magnitude_from_luminosity`

The absolute bolometric magnitude of a star of luminosity L on the IAU 2015 scale (zero point L0 = 3.0128e28 W).

$$
M_\mathrm{bol} = -2.5\log_{10}\frac{L}{L_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | luminosity (W or Lsun) | W |
| `M_bol` | out | absolute bolometric magnitude | - |
| `L_Lsun` | out | luminosity in solar luminosities | - |

**Applies when:** L > 0.

```shell
$ astronomy solve bolometric_magnitude_from_luminosity "L=3.828e+26 W"
M_bol = 4.739995934
L_Lsun = 1
  by: Absolute bolometric magnitude from luminosity  (M_bol = -2.5 log10(L / L0), L0 = 3.0128e28 W (IAU 2015 B2))
  L = 3.828e+26 W
  assumes: L > 0
```

**What the worked example means:** This calculation determines the absolute bolometric magnitude, which represents the intrinsic brightness of a star across all wavelengths. It uses the provided luminosity value of 3.828e+26 W and converts it to solar units as 1. The resulting magnitude is 4.74, indicating how bright the star would appear at a standard distance of ten parsecs. A lower numerical magnitude signifies a more luminous object.

<a id="apparent_bolometric_magnitude"></a>

#### Apparent bolometric magnitude and flux: `apparent_bolometric_magnitude`

The bolometric flux at the observer and the apparent bolometric magnitude of a star of luminosity L at distance d.

$$
F = \frac{L}{4\pi d^2},\quad m_\mathrm{bol} = -2.5\log_{10}\frac{F}{F_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | luminosity (W or Lsun) | W |
| `d` | in | distance (pc, ly, m ...) | m |
| `m_bol` | out | apparent bolometric magnitude | - |
| `F` | out | bolometric flux at the observer | W/m^2 |
| `M_bol` | out | absolute bolometric magnitude | - |

**Applies when:** L > 0; d > 0; no extinction.

```shell
$ astronomy solve apparent_bolometric_magnitude "L=3.0128e+28 W" "d=3.085677581e+18 m"
m_bol = 5
F = 2.518021003e-10 W/m^2
M_bol = 0
  by: Apparent bolometric magnitude and flux  (F = L/(4 pi d^2), m_bol = -2.5 log10(F/F0), F0 = L0/(4 pi (10 pc)^2))
  L = 3.0128e+28 W
  d = 3.085677581e+18 m
  assumes: L > 0; d > 0; no extinction
```

In the worked example, L = 3.0128e+28 W, d = 3.085677581e+18 m go through the equation above and give m_bol = 5, F = 2.518021003e-10 W/m^2, M_bol = 0. It applies only when L > 0; d > 0; no extinction: outside that it says what is wrong instead of returning a number.

<a id="effective_temperature"></a>

#### Effective temperature of a star: `effective_temperature`

The effective temperature of a star from its luminosity and radius: T = (L / (4 pi R^2 sigma))^(1/4).

$$
T_\mathrm{eff} = \left(\frac{L}{4\pi R^2\sigma}\right)^{1/4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | luminosity (W or Lsun) | W |
| `R` | in | stellar radius (m, km, Rsun) | m |
| `T` | out | effective temperature | K |

**Applies when:** L > 0; R > 0; blackbody emission.

```shell
$ astronomy solve effective_temperature "L=3.828e+26 W" "R=695700000 m"
T = 5772.003429 K
  by: Effective temperature of a star  (T = (L / (4 pi R^2 sigma))^(1/4))
  L = 3.828e+26 W
  R = 695700000 m
  assumes: L > 0; R > 0; blackbody emission
```

**What the worked example means:** This calculation determines the effective temperature of a star using its total energy output and physical size. By applying the Stefan-Boltzmann law, the model computes that for a star with a luminosity of 3.828e+26 W and a radius of 6.957e+08 m, the resulting surface temperature is 5772 K. This value represents the temperature of an ideal blackbody that would emit the same amount of radiation per unit area as the observed star.

<a id="free_fall_time"></a>

#### Free-fall time of a uniform cloud: `free_fall_time`

The time a uniform, pressureless sphere of density rho takes to collapse from rest under its own gravity.

$$
t_\mathrm{ff} = \sqrt{\frac{3\pi}{32\,G\rho}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | mean density (kg/m^3; g/cm^3) | kg/m^3 |
| `t_ff` | out | free-fall time | s |
| `t_ff_yr` | out | free-fall time in years | - |

**Applies when:** rho > 0; uniform, pressureless, non-rotating sphere starting at rest.

```shell
$ astronomy solve free_fall_time "rho=1e-17 kg/m^3"
t_ff = 2.100669417e+13 s
t_ff_yr = 665661.9695
  by: Free-fall time of a uniform cloud  (t_ff = sqrt(3 pi / (32 G rho)))
  rho = 1e-17 kg/m^3
  assumes: rho > 0; uniform, pressureless, non-rotating sphere starting at rest
```

**What the worked example means:** This calculation determines the time it takes for a uniform, pressureless sphere to collapse under its own gravity starting from rest. Given a mean density of 1e-17 kg/m^3, the resulting free-fall time is 2.10067e+13 s. This duration converts to approximately 665662, providing a fundamental timescale often used to estimate how quickly interstellar clouds might form stars if unsupported by other forces.

<a id="saha_hydrogen_ionization"></a>

#### Ionization fraction of hydrogen (Saha equation), `saha_hydrogen_ionization`

The ionized fraction of pure hydrogen in thermal equilibrium at temperature T and total hydrogen density n (Saha equation, ground-state partition functions).

$$
\frac{x^2}{1-x} = \frac{1}{n}\left(\frac{2\pi m_e k T}{h^2}\right)^{3/2} e^{-\chi/kT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `n` | in | total density of hydrogen nuclei (m^-3 or cm^-3) | 1/m^3 |
| `x` | out | ionized fraction n_II / n | - |
| `neutral` | out | neutral fraction n_I / n | - |
| `n_e` | out | electron density | 1/m^3 |

**Applies when:** 1000 K <= T <= 1e6 K; pure hydrogen, n_e = n_II; g_I = 2, g_II = 1, chi = 13.598 eV.

```shell
$ astronomy solve saha_hydrogen_ionization "T=10000 K" "n=1e+20 m^-3"
x = 0.807408863
neutral = 0.192591137
n_e = 8.07408863e+19 1/m^3
  by: Ionization fraction of hydrogen (Saha equation)  (x^2/(1 - x) = (1/n) (2 pi m_e k T / h^2)^(3/2) exp(-chi / kT), solved in a cancellation-free form)
  T = 10000 K
  n = 1e+20 1/m^3
  assumes: 1000 K <= T <= 1e6 K; pure hydrogen, n_e = n_II; g_I = 2, g_II = 1, chi = 13.598 eV
```

**What the worked example means:** This calculation determines the proportion of hydrogen atoms that have lost their electrons under thermal equilibrium conditions. Using the Saha equation, it balances temperature and density to find the ionization state. The result shows that at a temperature of 10000 K and a total hydrogen density of 1e+20 m^-3, the ionized fraction reaches 0.807409, which corresponds to a neutral fraction of 0.192591 and an electron density of 8.07409e+19 1/m^3. This helps characterize the physical state of gas in stellar atmospheres or nebulae.

<a id="luminosity_from_bolometric_magnitude"></a>

#### Luminosity from absolute bolometric magnitude, `luminosity_from_bolometric_magnitude`

The luminosity of a star from its absolute bolometric magnitude on the IAU 2015 scale.

$$
L = L_0\,10^{-0.4 M_\mathrm{bol}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M_bol` | in | absolute bolometric magnitude | - |
| `L` | out | luminosity | W |
| `L_Lsun` | out | luminosity in solar luminosities | - |

**Applies when:** -30 < M_bol < 40.

```shell
$ astronomy solve luminosity_from_bolometric_magnitude M_bol=-5
L = 3.0128e+30 W
L_Lsun = 7870.428422
  by: Luminosity from absolute bolometric magnitude  (L = L0 * 10^(-0.4 M_bol), L0 = 3.0128e28 W)
  M_bol = -5
  assumes: -30 < M_bol < 40
```

**What the worked example means:** This calculation determines the total energy output of a star based on its absolute bolometric magnitude. Applying the IAU 2015 scale definition converts the intrinsic brightness value of -5 into a physical power measurement. The resulting luminosity of 3.0128e+30 W W indicates how much energy the star radiates per second across all wavelengths. This value can also be expressed as 7870.43 times the Sun's luminosity, providing a direct comparison to the Sun's output without needing to account for distance from Earth.

<a id="stellar_luminosity"></a>

#### Luminosity of a star (Stefan-Boltzmann): `stellar_luminosity`

The luminosity of a star of radius R radiating as a blackbody at effective temperature T: L = 4 pi R^2 sigma T^4.

$$
L = 4\pi R^2 \sigma T_\mathrm{eff}^4
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | stellar radius (m, km, Rsun) | m |
| `T` | in | effective temperature | K |
| `L` | out | luminosity | W |
| `L_Lsun` | out | luminosity in solar luminosities | - |

**Applies when:** R > 0; T > 0.

```shell
$ astronomy solve stellar_luminosity "R=695700000 m" "T=5772 K"
L = 3.827990903e+26 W
L_Lsun = 0.9999976236
  by: Luminosity of a star (Stefan-Boltzmann)  (L = 4 pi R^2 sigma T^4 with sigma = 2 pi^5 k^4 / (15 h^3 c^2))
  R = 695700000 m
  T = 5772 K
  assumes: R > 0; T > 0
```

**What the worked example means:** This calculation determines the total energy output per unit time for a star with radius 6.957e+08 m and effective temperature 5772 K. Applying the Stefan-Boltzmann law shows that the luminosity is 3.82799e+26 W. This value indicates how bright the star is compared to the Sun, expressed here as 0.999998 times the solar luminosity. Understanding this brightness helps place the star on the Hertzsprung-Russell diagram and informs its evolutionary stage.

<a id="stellar_radius"></a>

#### Radius of a star from luminosity and temperature, `stellar_radius`

The radius of a star from its luminosity and effective temperature: R = sqrt(L / (4 pi sigma T^4)).

$$
R = \sqrt{\frac{L}{4\pi\sigma T_\mathrm{eff}^4}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | luminosity (W or Lsun) | W |
| `T` | in | effective temperature | K |
| `R` | out | stellar radius | m |
| `R_Rsun` | out | radius in solar radii | - |

**Applies when:** L > 0; T > 0; blackbody emission.

```shell
$ astronomy solve stellar_radius "L=3.828e+26 W" "T=5772 K"
R = 695700826.6 m
R_Rsun = 1.000001188
  by: Radius of a star from luminosity and temperature  (R = sqrt(L / (4 pi sigma)) / T^2)
  L = 3.828e+26 W
  T = 5772 K
  assumes: L > 0; T > 0; blackbody emission
```

**What the worked example means:** This calculation determines the physical size of a star by using its total energy output and surface temperature. Applying the Stefan-Boltzmann law, which relates luminosity to radius and temperature for a blackbody emitter, shows that the radius is 6.95701e+08 m. This value corresponds to 1 when expressed in units of the Sun's radius, providing a direct measure of the star's dimensions based on observable properties.

### Telescopes (4)

<a id="rayleigh_resolution"></a>

#### Diffraction limit of a telescope (Rayleigh criterion), `rayleigh_resolution`

The smallest angular separation a circular aperture of diameter D resolves at wavelength lambda: the angle of the first dark ring of the Airy pattern (1.21967 lambda/D for small angles; often rounded to 1.22).

$$
\sin\theta = \frac{j_{1,1}}{\pi}\frac{\lambda}{D} \approx 1.22\,\frac{\lambda}{D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | wavelength (nm, m) | m |
| `D` | in | aperture diameter | m |
| `theta` | out | angular resolution | rad |
| `theta_arcsec` | out | angular resolution in arcseconds | - |

**Applies when:** lambda << D; a circular unobstructed aperture, no atmosphere.

```shell
$ astronomy solve rayleigh_resolution "lam=550 nm" "D=2.4 m"
theta = 2.795076834e-07 rad
theta_arcsec = 0.05765259816
  by: Diffraction limit of a telescope (Rayleigh criterion)  (sin(theta) = j_{1,1} lambda / (pi D), j_{1,1} = 3.8317059702 (first zero of J1))
  lam = 5.5e-07 m
  D = 2.4 m
  assumes: lambda << D; a circular unobstructed aperture, no atmosphere
```

**What the worked example means:** This calculation determines the smallest angular separation that a telescope with an aperture diameter of 2.4 m can resolve at a wavelength of 550 nm. The result, 2.79508e-07 rad, represents the angle to the first dark ring of the Airy pattern, indicating the fundamental resolution limit imposed by diffraction. In practical terms, this means two point sources separated by less than 0.0576526 will appear as a single blurred image rather than distinct objects.

<a id="light_gathering_power"></a>

#### Light-gathering power of two telescopes: `light_gathering_power`

How many times more light one telescope collects than another (ratio of aperture areas) and the corresponding gain in limiting magnitude.

$$
\frac{P_1}{P_2} = \left(\frac{D_1}{D_2}\right)^2,\quad \Delta m = 5\log_{10}\frac{D_1}{D_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D1` | in | aperture of the first telescope | m |
| `D2` | in | aperture of the second telescope (or the eye's pupil) | m |
| `area_ratio` | out | light-gathering ratio (D1/D2)^2 | - |
| `mag_gain` | out | gain in limiting magnitude 5 log10(D1/D2) | - |

**Applies when:** D1, D2 > 0; unobstructed circular apertures.

```shell
$ astronomy solve light_gathering_power "D1=200 mm" "D2=2 mm"
area_ratio = 10000
mag_gain = 10
  by: Light-gathering power of two telescopes  (ratio = (D1/D2)^2; gain = 2.5 log10(ratio))
  D1 = 0.2 m
  D2 = 0.002 m
  assumes: D1, D2 > 0; unobstructed circular apertures
```

**What the worked example means:** This calculation determines how much more light one telescope can gather compared to another by comparing their aperture areas. Using the diameter of the first telescope (200 mm) and the diameter of the second (2 mm) gives the ratio of their collecting power as 10000. This ratio translates into a gain in limiting magnitude of 10, indicating how much fainter stars become visible with the larger instrument.

<a id="camera_pixel_scale"></a>

#### Pixel scale and field of view of a telescope camera, `camera_pixel_scale`

The angle on the sky covered by one detector pixel at the focal plane of a telescope and, for a row of N pixels, the field of view.

$$
\theta = 2\arctan\frac{w}{2f} \approx 206265''\,\frac{w}{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | focal length of the telescope | m |
| `p` | in | pixel size (um, mm) | m |
| `N` | in | number of pixels across the detector (optional) (optional) | - |
| `pixel_scale_arcsec` | out | pixel scale in arcseconds per pixel | - |
| `fov_deg` | out | field of view in degrees (with N) | - |
| `fov_arcmin` | out | field of view in arcminutes (with N) | - |

**Applies when:** f, p > 0; N >= 1; distortion-free optics; angles centred on the axis.

```shell
$ astronomy solve camera_pixel_scale "f=2000 mm" "p=3.76 um" N=6248
pixel_scale_arcsec = 0.3877778357
fov_deg = 0.6730022391
fov_arcmin = 40.38013435
  by: Pixel scale and field of view of a telescope camera  (angle = 2 atan(w / (2 f)) for a width w centred on the optical axis (about 206265 w/f arcseconds))
  f = 2 m
  p = 3.76e-06 m
  assumes: f, p > 0; N >= 1; distortion-free optics; angles centred on the axis
```

**What the worked example means:** This calculation determines the angular size on the sky covered by a single detector pixel, known as the pixel scale, and the total width of the sky visible to the camera, called the field of view. Using the telescope focal length 2000 mm and pixel size 3.76 um, the resulting pixel scale is 0.387778. If the number of pixels across the detector 6248 is provided, the total field of view is computed as 0.673002 or 40.3801. These values indicate how much detail can be resolved and how large an area of the sky is captured in one exposure.

<a id="telescope_magnification"></a>

#### Telescope magnification, exit pupil and true field, `telescope_magnification`

The angular magnification of a telescope used with an eyepiece, the diameter of its exit pupil and (given the eyepiece's apparent field) its true field of view.

$$
M = \frac{f_\mathrm{obj}}{f_\mathrm{eye}},\quad d_\mathrm{exit} = \frac{D}{M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f_obj` | in | focal length of the objective (mm, m) | m |
| `f_eye` | in | focal length of the eyepiece | m |
| `D` | in | aperture diameter | m |
| `afov` | in | apparent field of view of the eyepiece (optional) (optional) | rad |
| `M` | out | angular magnification | - |
| `exit_pupil` | out | exit-pupil diameter | m |
| `exit_pupil_mm` | out | exit-pupil diameter in millimetres | - |
| `tfov_deg` | out | true field of view in degrees (when afov is given) | - |

**Applies when:** focal lengths and aperture > 0; thin lenses, afocal (focused at infinity); true field = apparent field / M.

```shell
$ astronomy solve telescope_magnification "f_obj=1200 mm" "f_eye=25 mm" "D=200 mm" "afov=52 deg"
M = 48
exit_pupil = 0.004166666667 m
exit_pupil_mm = 4.166666667
tfov_deg = 1.083333333
  by: Telescope magnification, exit pupil and true field  (M = f_obj / f_eye; exit pupil = D / M; true field = apparent field / M)
  f_obj = 1.2 m
  f_eye = 0.025 m
  D = 0.2 m
  afov = 0.907571211 rad
  assumes: focal lengths and aperture > 0; thin lenses, afocal (focused at infinity); true field = apparent field / M
```

**What the worked example means:** This calculation determines the angular magnification by dividing the objective focal length 1200 mm by the eyepiece focal length 25 mm, resulting in 48. The exit pupil diameter is found by dividing the aperture 200 mm by this magnification, yielding 4.16667 millimetres. If the apparent field of view 52 deg is provided, the true field of view is calculated as 1.08333 degrees, indicating the actual sky area visible through the instrument.

### Time (3)

<a id="calendar_date_from_julian_date"></a>

#### Calendar date from a Julian date: `calendar_date_from_julian_date`

The Gregorian calendar date and the time of day (decimal hours) of a Julian date.

$$
Z = \lfloor \mathrm{JD} + 0.5\rfloor,\ F = \mathrm{JD} + 0.5 - Z
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `jd` | in | Julian date (days) | - |
| `year` | out | year | - |
| `month` | out | month | - |
| `day` | out | day of the month | - |
| `hours` | out | time of day in decimal hours | - |

**Applies when:** 2299160.5 <= JD <= 5373484.5 (Gregorian era up to the year 9999).

```shell
$ astronomy solve calendar_date_from_julian_date jd=2451545
year = 2000
month = 1
day = 1
hours = 12
  by: Calendar date from a Julian date  (Meeus' inverse algorithm in exact integer and rational arithmetic)
  jd = 2451545
  assumes: 2299160.5 <= JD <= 5373484.5 (Gregorian era up to the year 9999)
```

**What the worked example means:** This calculation determines the civil calendar date and time from a continuous astronomical time scale. By applying Meeus' inverse algorithm, the input Julian date 2.45154e+06 is converted into its constituent parts. The result provides the year 2000, month 1, and day 1, along with the fractional day expressed as decimal hours 12. This translation allows observers to map precise scientific timestamps back to standard dates for logging and reporting purposes.

<a id="sidereal_time"></a>

#### Greenwich and local mean sidereal time: `sidereal_time`

The Greenwich mean sidereal time (IAU 1982 expression) at a Julian date in UT1, and the local mean sidereal time at an east longitude.

$$
\mathrm{GMST} = 24110.54841^\mathrm{s} + 8640184.812866^\mathrm{s}\,T + 0.093104^\mathrm{s}\,T^2 - 6.2\times10^{-6}\,{}^\mathrm{s}T^3 + \mathrm{UT1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `jd` | in | Julian date (UT1) | - |
| `lon` | in | east longitude (default 0) (optional) | rad |
| `gmst_h` | out | Greenwich mean sidereal time (hours) | - |
| `lmst_h` | out | local mean sidereal time (hours) | - |

**Applies when:** 2299160.5 <= JD <= 5373484.5; the date in UT1; east longitudes positive.

```shell
$ astronomy solve sidereal_time jd=2451545
gmst_h = 18.69737456
lmst_h = 18.69737456
  by: Greenwich and local mean sidereal time  (GMST = 24110.54841 s + 8640184.812866 T + 0.093104 T^2 - 6.2e-6 T^3 + UT1 seconds of the day (T from J2000, Julian centuries); LMST = GMST + longitude/15)
  jd = 2451545
  assumes: 2299160.5 <= JD <= 5373484.5; the date in UT1; east longitudes positive
```

**What the worked example means:** This calculation determines the Earth's rotation angle relative to distant stars, known as sidereal time, for the given Julian date in Universal Time One. The result provides the Greenwich mean sidereal time as 18.6974 and adjusts it by the observer's east longitude to yield the local mean sidereal time of 18.6974, which indicates when celestial objects cross the observer's meridian.

<a id="julian_date"></a>

#### Julian date of a calendar date: `julian_date`

The Julian date and the modified Julian date of a Gregorian calendar date and time of day (in the same time scale as the date, e.g. TT or UT).

$$
\mathrm{JD} = \lfloor 365.25(y + 4716)\rfloor + \lfloor 30.6001(m + 1)\rfloor + D + B - 1524.5
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `year` | in | year (Gregorian, 1583 or later) | - |
| `month` | in | month (1-12) | - |
| `day` | in | day of the month | - |
| `hour` | in | hour (0-23, default 0) (optional) | - |
| `minute` | in | minute (0-59, default 0) (optional) | - |
| `second` | in | second (0 <= s < 60, default 0) (optional) | - |
| `JD` | out | Julian date (days) | - |
| `MJD` | out | modified Julian date JD - 2400000.5 | - |

**Applies when:** Gregorian calendar (on or after 1582-10-15); a valid date and time of day (no leap second).

```shell
$ astronomy solve julian_date year=2000 month=1 day=1 hour=12
JD = 2451545
MJD = 51544.5
  by: Julian date of a calendar date  (Meeus' algorithm in exact integer arithmetic: JD = floor(365.25(y+4716)) + floor(30.6001(m+1)) + D + B - 1524.5)
  assumes: Gregorian calendar (on or after 1582-10-15); a valid date and time of day (no leap second)
```

**What the worked example means:** This calculation determines the continuous day count known as the Julian date from a specific Gregorian calendar date and time. The result, 2.45154e+06, represents the total number of days since the epoch, while 51544.5 provides the same value adjusted by subtracting 2400000.5 for modern astronomical use. These values allow for precise scheduling and positional calculations in astronomy.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
