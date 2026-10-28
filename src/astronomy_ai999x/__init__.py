"""astronomy-ai999x — offline astronomy answers with their formula, steps and verification.

    import astronomy_ai999x as m
    m.ask("Parallax distance: p = 0.768 arcsec").show()          # d = 1.30208 pc = 4.24683 ly
    m.ask("Hohmann transfer: r1 = 6678 km, r2 = 42164 km, M = 1 Mearth").show()   # dv = 3892.6 m/s
    m.solve("altaz_from_hadec", H="30 deg", dec="38.78 deg", lat="52 deg")       # alt 65.39 deg, az 249.42 deg
    m.capabilities()                   # every certified capability

67 certified capabilities: distances and magnitudes (parallax, distance modulus, flux ratios, bolometric scale),
stars and blackbody radiation (Stefan-Boltzmann luminosity, radius, effective temperature, Saha ionization,
Schwarzschild radius, free-fall time, planetary equilibrium temperature and scale height), orbits (binary masses,
Hohmann transfers, apsis speeds, orbital elements, Kepler's equation, time of flight, departure and plane-change burns),
exoplanets (radial-velocity semi-amplitude, transit depth and duration), flat Lambda-CDM cosmology (distances, angular
scale, lookback time, age, comoving volume, critical density), redshift, telescopes (Rayleigh limit, magnification,
light gathering, pixel scale, airmass), positional astronomy and time (alt-az, culmination, rising and setting,
separations, position angles, Julian dates, sidereal time), plasma and space physics (plasma and hybrid frequencies,
Debye length, Alfven speed, gyration, thermal speed, beta, magnetic pressure) and gravitational lensing.
Questions use the form "<capability phrase>: name = value, ..."; ask() also reads au, pc, kpc, Mpc, ly, Msun, Mearth,
Mjup, Rsun, Rearth, Rjup, Lsun, arcsec, arcmin, mas, Myr, Gyr and gauss, while solve() takes the shared SI-style units.
Built by the AI999X model machine from the same engine as every domain; a question that is not determined
is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("astronomy", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
