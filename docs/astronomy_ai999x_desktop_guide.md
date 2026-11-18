# Astronomy: the desktop app, user guide

**Astronomy**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Astronomy covers celestial mechanics and orbits, stellar properties, blackbody radiation, magnitudes and distances, cosmology basics, the Sun and space environment and plasma parameters. Its desktop app holds all 67 calculations in 13 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-astronomy-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-astronomy-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-astronomy-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-astronomy.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-astronomy-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-astronomy-1.6.5-linux-x86_64.tar.gz
./ai999x-astronomy-1.6.5-linux-x86_64/ai999x-astronomy
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Astronomy app, opened on Distance from parallax with its worked example.](https://www.ai999x.com/?ax_shot=astronomy-ai999x/hero.webp)

*The Astronomy app, opened on Distance from parallax with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Distances and magnitudes › Distance from parallax (or type part of its name in the search box). It opens with a worked example filled in: “Parallax distance: p = 0.1 arcsec” The answer: d = 3.085677581e+17 m, d_pc = 10, d_ly = 32.61563777, p_arcsec = 0.1.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Distance from parallax: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-1.webp)

*Distance from parallax: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Parallax distance: p = 0.768 arcsec”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Distance from parallax
d = 4.017809351e+16 m
d_pc = 1.302083333
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Hohmann transfer between circular orbits](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-2.webp)

*Hohmann transfer between circular orbits*

![Lookback time to a redshift](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-3.webp)

*Lookback time to a redshift*

![Luminosity of a star (Stefan-Boltzmann)](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-4.webp)

*Luminosity of a star (Stefan-Boltzmann)*

![Altitude and azimuth from hour angle and declination](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-5.webp)

*Altitude and azimuth from hour angle and declination*

![Schwarzschild radius](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-6.webp)

*Schwarzschild radius*

![Radial-velocity semi-amplitude of a star](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-7.webp)

*Radial-velocity semi-amplitude of a star*

![Equilibrium temperature of a planet](https://www.ai999x.com/?ax_shot=astronomy-ai999x/scene-8.webp)

*Equilibrium temperature of a planet*

![Distance from parallax: a chart, enlarged.](https://www.ai999x.com/?ax_shot=astronomy-ai999x/chart-1.webp)

*Distance from parallax: a chart, enlarged.*

![Hohmann transfer between circular orbits: a chart, enlarged.](https://www.ai999x.com/?ax_shot=astronomy-ai999x/chart-2.webp)

*Hohmann transfer between circular orbits: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Astronomy app in the dark theme.](https://www.ai999x.com/?ax_shot=astronomy-ai999x/hero-dark.webp)

*The Astronomy app in the dark theme.*

## 7. Every calculation

67 calculations in 13 topics; each has its own entry on the model's page (https://www.ai999x.com/models/astronomy-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Compact objects | Schwarzschild radius |
| Cosmology | Age of the universe at a redshift, Angular size of an object at a redshift, Comoving volume out to a redshift, Cosmological distances at a redshift, Hubble parameter and critical density, Lookback time to a redshift, Radial velocity from a Doppler redshift, Redshift and radial velocity from a spectral line, Redshift at a given lookback time |
| Distances and magnitudes | Absolute magnitude, Astronomical distance units, Combined magnitude of two stars, Distance from luminosity and flux (inverse-square law), Distance from parallax, Distance from the distance modulus, Flux ratio from a magnitude difference, Magnitude difference from a flux ratio |
| Exoplanets | Radial-velocity semi-amplitude of a star, Transit depth, Transit duration |
| Gravitational lensing | Einstein radius of a point-mass lens |
| Observing | Airmass and atmospheric extinction, Angular size, Linear size from angular size |
| Orbits | Delta-v of a combined plane change, Departure burn from a circular orbit, Hohmann transfer between circular orbits, Masses of a visual binary star, Orbital elements from position and velocity, Position on an elliptical orbit at a given time (Kepler's equation), Speeds at periapsis and apoapsis, Time of flight between two points of an elliptical orbit |
| Planets | Equilibrium temperature of a planet, Scale height of an isothermal atmosphere |
| Plasma physics | Alfven speed, Debye length, Gyrofrequency (cyclotron frequency), Gyroradius (Larmor radius), Inertial length (skin depth), Lower hybrid frequency, Magnetic pressure, Number of particles in a Debye sphere, Plasma beta, Plasma frequency, Thermal speed of plasma particles, Upper hybrid frequency |
| Positional astronomy | Altitude and azimuth from hour angle and declination, Altitude of a star at culmination, Angular separation of two sky positions, Position angle between two sky positions, Rising and setting of a star |
| Stars | Absolute bolometric magnitude from luminosity, Apparent bolometric magnitude and flux, Effective temperature of a star, Free-fall time of a uniform cloud, Ionization fraction of hydrogen (Saha equation), Luminosity from absolute bolometric magnitude, Luminosity of a star (Stefan-Boltzmann), Radius of a star from luminosity and temperature |
| Telescopes | Diffraction limit of a telescope (Rayleigh criterion), Light-gathering power of two telescopes, Pixel scale and field of view of a telescope camera, Telescope magnification, exit pupil and true field |
| Time | Calendar date from a Julian date, Greenwich and local mean sidereal time, Julian date of a calendar date |

## 8. Other ways to use it

The same engine runs on the command line (`astronomy`, its own guide), in Python (`pip install astronomy-ai999x`, `import astronomy_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/astronomy-ai999x/.

## Credits

- Star positions: Yale Bright Star Catalogue, 5th ed. (Hoffleit & Warren 1991).

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/astronomy-ai999x/
