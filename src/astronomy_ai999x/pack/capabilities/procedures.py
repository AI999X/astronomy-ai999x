"""Astronomy procedures: deterministic capabilities with typed inputs and several outputs (domain code, loaded by the
pack; enabled only once tools/certify_procedures.py has certified them).

Topics: distances and magnitudes, stars and blackbody radiation, orbits and celestial mechanics, exoplanets,
cosmology (flat Lambda-CDM), telescopes and observing, positional astronomy and time, plasma and space physics.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question template it parses,
worked examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are
imported inside the reference functions) and a deliberately wrong variant that the checks must reject. The shipped
code (run, verify, parse) is plain Python (math, re, fractions) plus mpmath where stated.

Conventions (fixed, so run and reference agree exactly):
- Constants: CODATA 2022 (as in astropy 8 and scipy.constants 1.18): G = 6.67430e-11, c, h, k, e exact,
  m_e = 9.1093837139e-31 kg, m_p = 1.67262192595e-27 kg, eps0 = 8.8541878188e-12 F/m, mu0 = 1.25663706127e-6 N/A^2.
  IAU 2015 nominal values: au = 149597870700 m, pc = au * 648000/pi, light-year = c * 365.25 d,
  M_sun = 1.3271244e20/G, R_sun = 6.957e8 m, L_sun = 3.828e26 W, M_earth = 3.986004e14/G, R_earth = 6.3781e6 m,
  M_jup = 1.2668653e17/G, R_jup = 7.1492e7 m, bolometric zero points L0 = 3.0128e28 W (absolute) and
  F0 = L0 / (4 pi (10 pc)^2) (apparent). A year is the Julian year (365.25 d); Gyr = 1e9 Julian years.
- Magnitudes follow Pogson's scale: m1 - m2 = -2.5 log10(F1/F2). Distances in parsecs follow d = 10^(mu/5 + 1).
- Angles of sky positions: right ascension may be written in hours (h) or degrees; azimuth is measured from north
  through east in [0, 360) degrees; position angle from north through east in [0, 360) degrees; hour angle in hours,
  positive west of the meridian. Altitudes are geometric (no refraction).
- Orbits: two-body Keplerian motion; anomalies in degrees in [0, 360); time measured from periapsis.
- Cosmology: flat Lambda-CDM with matter and a cosmological constant only (no radiation, Omega_Lambda = 1 - Omega_m);
  H0 in km/s/Mpc.
- Blackbody: a star radiates as a blackbody of its effective temperature; a planet's equilibrium temperature assumes
  rapid rotation (full redistribution) and unit emissivity.
- Plasma: non-relativistic formulas, SI units; particles are electrons ("electron") or protons ("proton"); the Alfven
  speed uses the mass density n (m_p + m_e) of a fully ionized hydrogen plasma; thermal speeds need a stated
  convention (most probable, rms or mean).
- Spectral redshifts are read as special-relativistic radial Doppler shifts (not cosmological recession).
- Results built from cos, sin, exp or log are rounded to 12 significant digits (identical on every platform);
  values that are zero by geometry are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Parallax distance: p = 0.768 arcsec`); physical values carry a unit (astronomical units such as au, pc, kpc, Mpc,
ly, Msun, Mearth, Mjup, Rsun, Rearth, Rjup, Lsun, arcsec, arcmin, mas, Myr, Gyr and gauss are converted); right
ascension and hour angle may be written in hours. Any other number or an unknown name is refused.
"""
from __future__ import annotations

import math
import re
from fractions import Fraction

# ================================================================================================ constants
G = 6.6743e-11
C = 299792458.0
H = 6.62607015e-34
KB = 1.380649e-23
QE = 1.602176634e-19
ME = 9.1093837139e-31
MP = 1.67262192595e-27
EPS0 = 8.8541878188e-12
MU0 = 1.25663706127e-6
RGAS = 8.31446261815324
SIGMA = 2 * math.pi ** 5 * KB ** 4 / (15 * H ** 3 * C ** 2)
AU = 149597870700.0
PC = AU * 648000 / math.pi
LY = C * 365.25 * 86400.0
YR = 365.25 * 86400.0
GYR = 1e9 * YR
MPC = 1e6 * PC
MSUN = 1.3271244e20 / G
RSUN = 6.957e8
LSUN = 3.828e26
MEARTH = 3.986004e14 / G
REARTH = 6.3781e6
MJUP = 1.2668653e17 / G
RJUP = 7.1492e7
L0_BOL = 3.0128e28
F0_BOL = L0_BOL / (4 * math.pi * (10 * PC) ** 2)
ARCSEC = math.pi / 648000
DEG = math.pi / 180
CHI_H = 13.598434599702 * QE          # ionization energy of hydrogen (NIST)
J11 = 3.8317059702075125              # first zero of the Bessel function J1 (checked against scipy in the reference)


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str):
        return x
    if isinstance(x, int):
        return x
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    s = max(abs(a), abs(b))
    return d / s if s > 0 else math.inf


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _pos(name, *vals):
    for v in vals:
        if not (math.isfinite(v) and v > 0):
            raise ValueError(f"{name} must be a positive number")


# astronomical units the shared unit reader does not know: converted here to SI (factor, SI unit)
_ASTRO = {
    "au": (AU, "m"), "AU": (AU, "m"), "pc": (PC, "m"), "kpc": (1e3 * PC, "m"), "Mpc": (MPC, "m"), "Gpc": (1e9 * PC, "m"),
    "ly": (LY, "m"), "lyr": (LY, "m"), "Rsun": (RSUN, "m"), "R_sun": (RSUN, "m"), "Rearth": (REARTH, "m"),
    "R_earth": (REARTH, "m"), "Rjup": (RJUP, "m"), "R_jup": (RJUP, "m"), "Angstrom": (1e-10, "m"), "Å": (1e-10, "m"),
    "Msun": (MSUN, "kg"), "M_sun": (MSUN, "kg"), "Mearth": (MEARTH, "kg"), "M_earth": (MEARTH, "kg"),
    "Mjup": (MJUP, "kg"), "M_jup": (MJUP, "kg"), "Lsun": (LSUN, "W"), "L_sun": (LSUN, "W"),
    "arcsec": (ARCSEC, "rad"), "arcmin": (60 * ARCSEC, "rad"), "mas": (1e-3 * ARCSEC, "rad"),
    "Myr": (1e6 * YR, "s"), "Gyr": (GYR, "s"), "d": (86400.0, "s"), "G": (1e-4, "T"), "gauss": (1e-4, "T"),
}
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_ \-]*?)\s*$")


def _name(names, written):
    """The symbol a written name stands for: the exact spelling first (m and M differ), then lower case."""
    return names.get(written, names.get(written.lower()))


def _pairs(question, phrase, names, phys=(), words=(), num_units=None, hours=()):
    """{symbol: value} from '<phrase>: name = value, ...'. None unless the phrase opens the question and every part is
    a 'name = value' pair of a known name. `phys`: symbols that need a unit (returned as 'value unit' strings in SI
    when the unit is astronomical, read by the runtime otherwise); `words`: symbols whose value is a word (a choice);
    `num_units`: {symbol: [allowed unit spellings]} for plain numbers that may carry a unit word; `hours`: angle
    symbols that may be written in hours (1 h = 15 degrees)."""
    if not isinstance(question, str):
        return None
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", question, re.I)
    if not m:
        return None
    rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
    if not rest:
        return {}
    out = {}
    num_units = num_units or {}
    for seg in re.split(r"[,;]", rest):
        if not seg.strip():
            continue
        mw = _WORD.match(seg)
        if mw and _name(names, mw.group(1)) in set(words):
            key = _name(names, mw.group(1))
            if key in out:
                return None
            out[key] = mw.group(2).strip().lower()
            continue
        ms = _SEG.match(seg)
        if not ms:
            return None
        key = _name(names, ms.group(1))
        if key is None or key in out or key in words:
            return None
        num, unit = ms.group(2), ms.group(3).strip()
        if key in phys:
            if not unit:
                return None
            if key in hours and unit in ("h", "hr", "hours", "hour"):
                out[key] = f"{float(num) * 15!r} deg"
            elif unit in _ASTRO:
                f, si = _ASTRO[unit]
                out[key] = f"{float(num) * f!r} {si}"
            else:
                out[key] = f"{num} {unit}"
        else:
            if unit and unit not in num_units.get(key, ()):
                return None
            out[key] = float(num)
    return out


def _phys(sym, kind, meaning, required=True):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}


def _numin(sym, meaning, required=True, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    spec.setdefault("verify", _recheck(spec["run"]))
    PROCEDURES[key] = spec


# ---------------------------------------------------------------------------------------------- reference helpers
def _ref_planck_integral():
    """The integral of x^3/(e^x - 1) over (0, inf) by scipy quadrature (pi^4/15; never written in closed form here)."""
    from scipy.integrate import quad
    f = lambda x: x ** 3 * math.exp(-x) / -math.expm1(-x) if x > 0 else 0.0     # noqa: E731  x^3/(e^x - 1)
    a, _ = quad(f, 0, 50, epsabs=0, epsrel=2e-14, limit=200)
    b, _ = quad(f, 50, math.inf, epsabs=0, epsrel=2e-14, limit=200)
    return a + b


def _ref_bb_flux(T):
    """Blackbody exitance pi * integral of B_nu over all frequencies, from Planck's law integrated numerically."""
    return 2 * math.pi * KB ** 4 * T ** 4 / (H ** 3 * C ** 2) * _ref_planck_integral()


def _ref_kepler_period(a, mu):
    """Kepler's second law: dt = r^2/h dtheta, integrated around an ellipse (e = 0.3) of semi-major axis a."""
    from scipy.integrate import quad
    e = 0.3
    p = a * (1 - e * e)
    h = math.sqrt(mu * p)
    T, _ = quad(lambda th: (p / (1 + e * math.cos(th))) ** 2 / h, 0, 2 * math.pi, epsabs=0, epsrel=2e-14, limit=200)
    return T


def _ref_flight_time(mu, p, e, th1, th2):
    """Time from true anomaly th1 to th2 (radians, th2 > th1) by quadrature of dt = r^2/h dtheta."""
    from scipy.integrate import quad
    h = math.sqrt(mu * p)
    T, _ = quad(lambda th: (p / (1 + e * math.cos(th))) ** 2 / h, th1, th2, epsabs=0, epsrel=2e-14, limit=400)
    return T


def _ref_work_to_infinity(GM, r):
    """Work per kilogram to move from r to infinity against gravity (scipy quadrature)."""
    from scipy.integrate import quad
    # x = r e^s maps (r, inf) onto (0, inf): dW = GM/x^2 dx = (GM/r) e^(-s) ds
    w, _ = quad(lambda s_: GM / r * math.exp(-s_), 0, math.inf, epsabs=0, epsrel=2e-14)
    return w


def _ref_unit_vec(ra, dec):
    import numpy as np
    return np.array([math.cos(dec) * math.cos(ra), math.cos(dec) * math.sin(ra), math.sin(dec)])


def _ref_angle_between(a, b):
    import numpy as np
    return float(np.arctan2(np.linalg.norm(np.cross(a, b)), np.dot(a, b)))


# ================================================================================================ distances and magnitudes
def distance_conversion(d):
    _pos("the distance", d)
    return _out(d_m=d, d_km=d / 1e3, d_au=d / AU, d_ly=d / LY, d_pc=d / PC)


def ref_distance_conversion(d):
    import astropy.units as u
    q = d * u.m
    return {"d_km": q.to(u.km).value, "d_au": q.to(u.au).value, "d_ly": q.to(u.lyr).value, "d_pc": q.to(u.pc).value}


_add("distance_unit_conversion",
     name="Astronomical distance units", topic="distances and magnitudes / distance units",
     purpose="A distance expressed in metres, kilometres, astronomical units, light-years and parsecs.",
     inputs=[_phys("d", "length", "distance (m, km, au, ly, pc, kpc, Mpc ...)")],
     outputs=[_o("d_m", "length", "distance in metres"), _o("d_km", "number", "distance in kilometres"),
              _o("d_au", "number", "distance in astronomical units"), _o("d_ly", "number", "distance in light-years"),
              _o("d_pc", "number", "distance in parsecs")],
     primary="d_pc", conditions=["d > 0"],
     method="1 au = 149597870700 m; 1 pc = au * 648000/pi; 1 ly = c * 365.25 days",
     latex=r"1\,\mathrm{pc} = \frac{648000}{\pi}\,\mathrm{au},\quad 1\,\mathrm{ly} = c \times 365.25\,\mathrm{d}",
     run=distance_conversion,
     parse=lambda q: _pairs(q, r"distance (?:unit )?conversion", {"d": "d"}, phys={"d"}),
     reference=ref_distance_conversion,
     wrong=_safe(lambda d: _out(d_m=d, d_km=d / 1e3, d_au=d / AU, d_ly=d / (C * 365 * 86400.0), d_pc=d / (3.26 * C * 365 * 86400.0))),
     random_inputs=lambda rng: {"d": _lu(rng, 1e9, 1e26)},
     templates=["Distance conversion: d = 4.24 ly", "Distance conversion: d = 1 pc", "Distance conversion: d = 2.5 Mpc"],
     examples=[{"inputs": {"d": PC}, "outputs": {"d_pc": 1.0, "d_au": 206264.806247096}},
               {"inputs": {"d": AU}, "outputs": {"d_au": 1.0, "d_km": 149597870.7}},
               {"inputs": {"d": LY}, "outputs": {"d_ly": 1.0}}],
     questions=[{"q": "Distance conversion: d = 1 pc", "outputs": {"d_pc": 1.0, "d_au": 206264.806247096}},
                {"q": "Distance conversion: d = 1 ly", "outputs": {"d_ly": 1.0, "d_m": 9460730472580800.0}},
                {"q": "Distance conversion: d = -3 ly", "outcome": "INVALID_INPUT"},
                {"q": "Distance conversion: d = 5", "outcome": "UNRECOGNIZED_FORMAT"}])


def parallax_distance(p):
    _pos("the parallax", p)
    d_pc = ARCSEC / p
    return _out(d=d_pc * PC, d_pc=d_pc, d_ly=d_pc * PC / LY, p_arcsec=p / ARCSEC)


def ref_parallax_distance(p):
    import astropy.units as u
    q = p * u.rad
    return {"d_pc": q.to(u.pc, equivalencies=u.parallax()).value, "d_ly": q.to(u.lyr, equivalencies=u.parallax()).value}


_add("parallax_distance",
     name="Distance from parallax", topic="distances and magnitudes / parallax",
     purpose="The distance of a star from its annual trigonometric parallax: d [pc] = 1 / p [arcsec].",
     inputs=[_phys("p", "angle", "annual parallax (arcsec, mas, rad, deg)")],
     outputs=[_o("d", "length", "distance"), _o("d_pc", "number", "distance in parsecs"),
              _o("d_ly", "number", "distance in light-years"), _o("p_arcsec", "number", "parallax in arcseconds")],
     primary="d_pc", conditions=["p > 0 (small-angle definition of the parsec)"],
     method="d = 1/p with p in arcseconds gives parsecs", latex=r"d\,[\mathrm{pc}] = \frac{1}{p\,[\mathrm{arcsec}]}",
     run=parallax_distance,
     parse=lambda q: _pairs(q, r"parallax distance|distance from parallax", {"p": "p", "parallax": "p"}, phys={"p"}),
     reference=ref_parallax_distance,
     wrong=_safe(lambda p: (lambda d_pc: _out(d=d_pc * PC, d_pc=d_pc, d_ly=d_pc * PC / LY, p_arcsec=p / ARCSEC))(ARCSEC / p / 1000)),
     random_inputs=lambda rng: {"p": _lu(rng, 1e-10, 1e-5)},
     templates=["Parallax distance: p = 0.768 arcsec", "Parallax distance: p = 2.5 mas"],
     examples=[{"inputs": {"p": ARCSEC}, "outputs": {"d_pc": 1.0}},
               {"inputs": {"p": 0.1 * ARCSEC}, "outputs": {"d_pc": 10.0}},
               {"inputs": {"p": 0.768 * ARCSEC}, "outputs": {"d_pc": 1 / 0.768}}],
     questions=[{"q": "Parallax distance: p = 0.1 arcsec", "outputs": {"d_pc": 10.0}},
                {"q": "Parallax distance: p = 2 mas", "outputs": {"d_pc": 500.0}},
                {"q": "Parallax distance: p = 0 arcsec", "outcome": "INVALID_INPUT"},
                {"q": "Parallax distance:", "outcome": "NEEDS_INPUT"}])

_MAG = ["mag", "mags", "magnitude", "magnitudes"]


def distance_from_modulus(m, M, A=0.0):
    if A < 0:
        raise ValueError("the extinction A must not be negative")
    mu = m - M - A
    if not -10 < mu < 60:
        raise ValueError("the distance modulus must lie between -10 and 60 magnitudes")
    d_pc = 10 ** (mu / 5 + 1)
    return _out(d=d_pc * PC, d_pc=d_pc, d_ly=d_pc * PC / LY, mu=mu)


def ref_distance_from_modulus(m, M, A=0.0):
    import astropy.units as u
    from astropy.coordinates import Distance
    d = Distance(distmod=(m - M - A) * u.mag)
    return {"d_pc": d.to(u.pc).value, "d_ly": d.to(u.lyr).value}


_add("distance_from_distance_modulus",
     name="Distance from the distance modulus", topic="distances and magnitudes / distance modulus",
     purpose="The distance of a star from its apparent and absolute magnitudes (and an optional extinction A): m - M - A = 5 log10(d / 10 pc).",
     inputs=[_numin("m", "apparent magnitude"), _numin("M", "absolute magnitude"),
             _numin("A", "interstellar extinction in magnitudes (default 0)", required=False)],
     outputs=[_o("d", "length", "distance"), _o("d_pc", "number", "distance in parsecs"),
              _o("d_ly", "number", "distance in light-years"), _o("mu", "number", "distance modulus m - M - A")],
     primary="d_pc", conditions=["A >= 0", "-10 < m - M - A < 60"],
     method="d = 10^((m - M - A)/5 + 1) pc", latex=r"d = 10^{(m - M - A)/5 + 1}\ \mathrm{pc}",
     run=distance_from_modulus,
     parse=lambda q: _pairs(q, r"distance from (?:the )?distance modulus", {"m": "m", "M": "M", "A": "A", "a": "A"},
                            num_units={"m": _MAG, "M": _MAG, "A": _MAG}),
     reference=ref_distance_from_modulus,
     wrong=_safe(lambda m, M, A=0.0: (lambda d_pc: _out(d=d_pc * PC, d_pc=d_pc, d_ly=d_pc * PC / LY, mu=m - M - A))(10 ** ((m - M - A) / 5))),
     random_inputs=lambda rng: {"m": rng.uniform(-1, 22), "M": rng.uniform(-8, 12),
                                **({"A": rng.uniform(0, 3)} if rng.random() < 0.5 else {})},
     templates=["Distance from distance modulus: m = 10.5, M = 0.5", "Distance from distance modulus: m = 15, M = 4.8, A = 1.2"],
     examples=[{"inputs": {"m": 10.0, "M": 0.0}, "outputs": {"d_pc": 1000.0, "mu": 10.0}},
               {"inputs": {"m": 15.0, "M": 5.0, "A": 0.0}, "outputs": {"d_pc": 1000.0}},
               {"inputs": {"m": 7.0, "M": 2.0, "A": 1.0}, "outputs": {"d_pc": 10 ** 1.8}}],
     questions=[{"q": "Distance from distance modulus: m = 10, M = 0", "outputs": {"d_pc": 1000.0}},
                {"q": "Distance from distance modulus: m = 7, M = 2, A = 1", "outputs": {"d_pc": 10 ** 1.8}},
                {"q": "Distance from distance modulus: m = 7, M = 2, A = -1", "outcome": "INVALID_INPUT"},
                {"q": "Distance from distance modulus: m = 10", "outcome": "NEEDS_INPUT"}])


def absolute_magnitude(m, d, A=0.0):
    _pos("the distance", d)
    if A < 0:
        raise ValueError("the extinction A must not be negative")
    mu = 5 * math.log10(d / PC) - 5
    return _out(M=m - mu - A, mu=mu, d_pc=d / PC)


def ref_absolute_magnitude(m, d, A=0.0):
    import astropy.units as u
    from astropy.coordinates import Distance
    mu = Distance(d * u.m).distmod.value
    return {"M": m - mu - A, "mu": mu}


_add("absolute_magnitude",
     name="Absolute magnitude", topic="distances and magnitudes / absolute magnitude",
     purpose="The absolute magnitude of a star from its apparent magnitude and distance (and an optional extinction A).",
     inputs=[_numin("m", "apparent magnitude"), _phys("d", "length", "distance (pc, ly, m ...)"),
             _numin("A", "interstellar extinction in magnitudes (default 0)", required=False)],
     outputs=[_o("M", "number", "absolute magnitude"), _o("mu", "number", "distance modulus 5 log10(d/10 pc)"),
              _o("d_pc", "number", "distance in parsecs")],
     primary="M", conditions=["d > 0", "A >= 0"],
     method="M = m - 5 log10(d / 10 pc) - A", latex=r"M = m - 5\log_{10}\frac{d}{10\,\mathrm{pc}} - A",
     run=absolute_magnitude,
     parse=lambda q: _pairs(q, r"absolute magnitude", {"m": "m", "d": "d", "A": "A", "a": "A"}, phys={"d"},
                            num_units={"m": _MAG, "A": _MAG}),
     reference=ref_absolute_magnitude,
     wrong=_safe(lambda m, d, A=0.0: _out(M=m - 5 * math.log10(d / PC) - A, mu=5 * math.log10(d / PC), d_pc=d / PC)),
     random_inputs=lambda rng: {"m": rng.uniform(-1, 22), "d": _lu(rng, 1.3 * PC, 1e9 * PC),
                                **({"A": rng.uniform(0, 3)} if rng.random() < 0.5 else {})},
     templates=["Absolute magnitude: m = 0.03, d = 7.68 pc", "Absolute magnitude: m = 12.5, d = 2.4 kpc, A = 0.8"],
     examples=[{"inputs": {"m": 12.0, "d": 1000 * PC}, "outputs": {"M": 2.0}},
               {"inputs": {"m": 7.0, "d": 100 * PC}, "outputs": {"M": 2.0}},
               {"inputs": {"m": 8.0, "d": 100 * PC, "A": 1.0}, "outputs": {"M": 2.0}}],
     questions=[{"q": "Absolute magnitude: m = 8, d = 100 pc, A = 1", "outputs": {"M": 2.0}},
                {"q": "Absolute magnitude: m = 6.5, d = 10 pc", "outputs": {"M": 6.5}},
                {"q": "Absolute magnitude: m = 6.5, d = 0 pc", "outcome": "INVALID_INPUT"},
                {"q": "Absolute magnitude: m = 6.5", "outcome": "NEEDS_INPUT"}])


def flux_ratio(m1, m2):
    dm = m1 - m2
    if abs(dm) > 100:
        raise ValueError("the magnitude difference must be at most 100")
    return _out(ratio=10 ** (-0.4 * dm), dm=dm)


_add("flux_ratio_from_magnitudes",
     name="Flux ratio from a magnitude difference", topic="distances and magnitudes / magnitude scale",
     purpose="How many times brighter star 1 is than star 2: F1/F2 = 10^(-0.4 (m1 - m2)).",
     inputs=[_numin("m1", "magnitude of the first object"), _numin("m2", "magnitude of the second object")],
     outputs=[_o("ratio", "number", "flux ratio F1/F2"), _o("dm", "number", "magnitude difference m1 - m2")],
     primary="ratio", conditions=["|m1 - m2| <= 100"],
     method="Pogson's scale: F1/F2 = 10^(-0.4 (m1 - m2))", latex=r"\frac{F_1}{F_2} = 10^{-0.4\,(m_1 - m_2)}",
     run=flux_ratio,
     parse=lambda q: _pairs(q, r"flux ratio(?: from (?:a )?magnitude difference)?", {"m1": "m1", "m2": "m2"},
                            num_units={"m1": _MAG, "m2": _MAG}),
     reference=lambda m1, m2: (lambda u: {"ratio": float(u.Magnitude(m1 - m2).physical)})(__import__("astropy.units", fromlist=["units"])),
     wrong=_safe(lambda m1, m2: _out(ratio=2.5 ** (-(m1 - m2)), dm=m1 - m2)),
     random_inputs=lambda rng: {"m1": rng.uniform(-2, 20), "m2": rng.uniform(-2, 20)},
     templates=["Flux ratio: m1 = 1, m2 = 6", "Flux ratio from magnitude difference: m1 = -1.46, m2 = 0.03"],
     examples=[{"inputs": {"m1": 1.0, "m2": 6.0}, "outputs": {"ratio": 100.0}},
               {"inputs": {"m1": 0.0, "m2": 2.5}, "outputs": {"ratio": 10.0}},
               {"inputs": {"m1": 4.0, "m2": 4.0}, "outputs": {"ratio": 1.0}}],
     questions=[{"q": "Flux ratio: m1 = 1, m2 = 6", "outputs": {"ratio": 100.0}},
                {"q": "Flux ratio: m1 = 0 mag, m2 = 5 mag", "outputs": {"ratio": 100.0}},
                {"q": "Flux ratio: m1 = 0, m2 = 500", "outcome": "INVALID_INPUT"},
                {"q": "Flux ratio: m1 = 1", "outcome": "NEEDS_INPUT"}])


def magnitude_difference(ratio):
    _pos("the flux ratio", ratio)
    return _out(dm=-2.5 * math.log10(ratio))


_add("magnitude_difference_from_flux_ratio",
     name="Magnitude difference from a flux ratio", topic="distances and magnitudes / magnitude scale",
     purpose="The magnitude difference m1 - m2 of two objects whose fluxes are in the ratio F1/F2.",
     inputs=[_numin("ratio", "flux ratio F1/F2")],
     outputs=[_o("dm", "number", "magnitude difference m1 - m2")],
     primary="dm", conditions=["F1/F2 > 0"],
     method="m1 - m2 = -2.5 log10(F1/F2)", latex=r"m_1 - m_2 = -2.5\log_{10}\frac{F_1}{F_2}",
     run=magnitude_difference,
     parse=lambda q: _pairs(q, r"magnitude difference(?: from (?:a )?flux ratio)?", {"ratio": "ratio", "r": "ratio"}),
     reference=lambda ratio: (lambda u: {"dm": float(u.Magnitude(ratio * u.one).value)})(__import__("astropy.units", fromlist=["units"])),
     wrong=_safe(lambda ratio: _out(dm=-2.5 * math.log(ratio))),
     random_inputs=lambda rng: {"ratio": rng.choice([_lu(rng, 1e-6, 0.95), _lu(rng, 1.05, 1e6)])},
     templates=["Magnitude difference from flux ratio: ratio = 100", "Magnitude difference: ratio = 0.5"],
     examples=[{"inputs": {"ratio": 100.0}, "outputs": {"dm": -5.0}},
               {"inputs": {"ratio": 0.01}, "outputs": {"dm": 5.0}},
               {"inputs": {"ratio": 10.0}, "outputs": {"dm": -2.5}}],
     questions=[{"q": "Magnitude difference: ratio = 100", "outputs": {"dm": -5.0}},
                {"q": "Magnitude difference from flux ratio: ratio = 0.01", "outputs": {"dm": 5.0}},
                {"q": "Magnitude difference: ratio = -2", "outcome": "INVALID_INPUT"},
                {"q": "Magnitude difference:", "outcome": "NEEDS_INPUT"}])


def combined_magnitude(m1, m2):
    if abs(m1 - m2) > 100:
        raise ValueError("the two magnitudes must differ by at most 100")
    lo = min(m1, m2)
    total = lo - 2.5 * math.log10(1 + 10 ** (-0.4 * abs(m1 - m2)))
    return _out(m=total, brightening=lo - total)


def ref_combined_magnitude(m1, m2):
    import astropy.units as u
    f = u.Magnitude(m1).physical + u.Magnitude(m2).physical
    return {"m": float(u.Magnitude(f * u.one).value)}


_add("combined_magnitude",
     name="Combined magnitude of two stars", topic="distances and magnitudes / magnitude scale",
     purpose="The magnitude of two unresolved stars seen together (their fluxes add).",
     inputs=[_numin("m1", "magnitude of the first star"), _numin("m2", "magnitude of the second star")],
     outputs=[_o("m", "number", "combined magnitude"), _o("brightening", "number", "how much brighter than the brighter star alone (mag)")],
     primary="m", conditions=["|m1 - m2| <= 100"],
     method="fluxes add: m = -2.5 log10(10^(-0.4 m1) + 10^(-0.4 m2))",
     latex=r"m = -2.5\log_{10}\left(10^{-0.4 m_1} + 10^{-0.4 m_2}\right)",
     run=combined_magnitude,
     parse=lambda q: _pairs(q, r"combined magnitude", {"m1": "m1", "m2": "m2"}, num_units={"m1": _MAG, "m2": _MAG}),
     reference=ref_combined_magnitude,
     wrong=_safe(lambda m1, m2: _out(m=(m1 + m2) / 2, brightening=0.0)),
     random_inputs=lambda rng: (lambda a, b: {"m1": a, "m2": b})(rng.uniform(0.5, 20), rng.uniform(0.5, 20)),
     templates=["Combined magnitude: m1 = 2.0, m2 = 3.0", "Combined magnitude: m1 = 5, m2 = 5"],
     examples=[{"inputs": {"m1": 5.0, "m2": 5.0}, "outputs": {"m": 5 - 2.5 * math.log10(2)}},
               {"inputs": {"m1": 1.0, "m2": 6.0}, "outputs": {"m": 1 - 2.5 * math.log10(1.01)}}],
     questions=[{"q": "Combined magnitude: m1 = 5, m2 = 5", "outputs": {"m": 5 - 2.5 * math.log10(2)}},
                {"q": "Combined magnitude: m1 = 1, m2 = 6", "outputs": {"m": 1 - 2.5 * math.log10(1.01)}},
                {"q": "Combined magnitude: m1 = 1, m2 = 600", "outcome": "INVALID_INPUT"},
                {"q": "Combined magnitude: m1 = 1", "outcome": "NEEDS_INPUT"}])


def bolometric_magnitude(L):
    _pos("the luminosity", L)
    return _out(M_bol=-2.5 * math.log10(L / L0_BOL), L_Lsun=L / LSUN)


_add("bolometric_magnitude_from_luminosity",
     name="Absolute bolometric magnitude from luminosity", topic="stars / bolometric magnitude",
     purpose="The absolute bolometric magnitude of a star of luminosity L on the IAU 2015 scale (zero point L0 = 3.0128e28 W).",
     inputs=[_phys("L", "power", "luminosity (W or Lsun)")],
     outputs=[_o("M_bol", "number", "absolute bolometric magnitude"), _o("L_Lsun", "number", "luminosity in solar luminosities")],
     primary="M_bol", conditions=["L > 0"],
     method="M_bol = -2.5 log10(L / L0), L0 = 3.0128e28 W (IAU 2015 B2)", latex=r"M_\mathrm{bol} = -2.5\log_{10}\frac{L}{L_0}",
     run=bolometric_magnitude,
     parse=lambda q: _pairs(q, r"bolometric magnitude from luminosity|absolute bolometric magnitude", {"l": "L"}, phys={"L"}),
     reference=lambda L: (lambda u: {"M_bol": float(u.Magnitude(L * u.W, unit=u.mag(u.Bol)).value)})(__import__("astropy.units", fromlist=["units"])),
     wrong=_safe(lambda L: _out(M_bol=-2.5 * math.log(L / L0_BOL), L_Lsun=L / LSUN)),
     random_inputs=lambda rng: {"L": rng.choice([_lu(rng, 1e22, 2.4e28), _lu(rng, 3.8e28, 1e33)])},
     templates=["Absolute bolometric magnitude: L = 1 Lsun", "Bolometric magnitude from luminosity: L = 3.828e26 W"],
     examples=[{"inputs": {"L": L0_BOL}, "outputs": {"M_bol": 0.0}},
               {"inputs": {"L": 100 * L0_BOL}, "outputs": {"M_bol": -5.0}},
               {"inputs": {"L": LSUN}, "outputs": {"M_bol": -2.5 * math.log10(LSUN / L0_BOL)}}],
     questions=[{"q": "Absolute bolometric magnitude: L = 1 Lsun", "outputs": {"M_bol": -2.5 * math.log10(LSUN / L0_BOL)}},
                {"q": "Absolute bolometric magnitude: L = 3.0128e30 W", "outputs": {"M_bol": -5.0}},
                {"q": "Absolute bolometric magnitude: L = 0 W", "outcome": "INVALID_INPUT"},
                {"q": "Absolute bolometric magnitude:", "outcome": "NEEDS_INPUT"}])


def luminosity_from_magnitude(M_bol):
    if not -30 < M_bol < 40:
        raise ValueError("the absolute bolometric magnitude must lie between -30 and 40")
    L = L0_BOL * 10 ** (-0.4 * M_bol)
    return _out(L=L, L_Lsun=L / LSUN)


_add("luminosity_from_bolometric_magnitude",
     name="Luminosity from absolute bolometric magnitude", topic="stars / bolometric magnitude",
     purpose="The luminosity of a star from its absolute bolometric magnitude on the IAU 2015 scale.",
     inputs=[_numin("M_bol", "absolute bolometric magnitude")],
     outputs=[_o("L", "power", "luminosity"), _o("L_Lsun", "number", "luminosity in solar luminosities")],
     primary="L_Lsun", conditions=["-30 < M_bol < 40"],
     method="L = L0 * 10^(-0.4 M_bol), L0 = 3.0128e28 W", latex=r"L = L_0\,10^{-0.4 M_\mathrm{bol}}",
     run=luminosity_from_magnitude,
     parse=lambda q: _pairs(q, r"luminosity from (?:absolute )?bolometric magnitude", {"m_bol": "M_bol", "mbol": "M_bol", "m": "M_bol"},
                            num_units={"M_bol": _MAG}),
     reference=lambda M_bol: (lambda u: (lambda L: {"L": L, "L_Lsun": L / LSUN})(float((M_bol * u.mag(u.Bol)).physical.to(u.W).value)))(__import__("astropy.units", fromlist=["units"])),
     wrong=_safe(lambda M_bol: _out(L=L0_BOL * 10 ** (-0.5 * M_bol), L_Lsun=L0_BOL * 10 ** (-0.5 * M_bol) / LSUN)),
     random_inputs=lambda rng: {"M_bol": rng.choice([rng.uniform(-10, -0.5), rng.uniform(0.5, 17)])},
     templates=["Luminosity from bolometric magnitude: M_bol = 4.74", "Luminosity from absolute bolometric magnitude: M_bol = -5"],
     examples=[{"inputs": {"M_bol": 0.0}, "outputs": {"L": L0_BOL}},
               {"inputs": {"M_bol": -5.0}, "outputs": {"L": 100 * L0_BOL}}],
     questions=[{"q": "Luminosity from bolometric magnitude: M_bol = -5", "outputs": {"L": 100 * L0_BOL}},
                {"q": "Luminosity from bolometric magnitude: M_bol = 0 mag", "outputs": {"L": L0_BOL}},
                {"q": "Luminosity from bolometric magnitude: M_bol = 99", "outcome": "INVALID_INPUT"},
                {"q": "Luminosity from bolometric magnitude:", "outcome": "NEEDS_INPUT"}])


def apparent_bolometric(L, d):
    _pos("the luminosity and the distance", L, d)
    F = L / (4 * math.pi * d * d)
    return _out(m_bol=-2.5 * math.log10(F / F0_BOL), F=F, M_bol=-2.5 * math.log10(L / L0_BOL))


def ref_apparent_bolometric(L, d):
    import astropy.units as u
    from astropy.coordinates import Distance
    M = float(u.Magnitude(L * u.W, unit=u.mag(u.Bol)).value)          # absolute scale (L0) ...
    return {"M_bol": M, "m_bol": M + float(Distance(d * u.m).distmod.value)}   # ... plus the distance modulus


_add("apparent_bolometric_magnitude",
     name="Apparent bolometric magnitude and flux", topic="stars / bolometric magnitude",
     purpose="The bolometric flux at the observer and the apparent bolometric magnitude of a star of luminosity L at distance d.",
     inputs=[_phys("L", "power", "luminosity (W or Lsun)"), _phys("d", "length", "distance (pc, ly, m ...)")],
     outputs=[_o("m_bol", "number", "apparent bolometric magnitude"), _o("F", "intensity", "bolometric flux at the observer"),
              _o("M_bol", "number", "absolute bolometric magnitude")],
     primary="m_bol", conditions=["L > 0", "d > 0", "no extinction"],
     method="F = L/(4 pi d^2), m_bol = -2.5 log10(F/F0), F0 = L0/(4 pi (10 pc)^2)",
     latex=r"F = \frac{L}{4\pi d^2},\quad m_\mathrm{bol} = -2.5\log_{10}\frac{F}{F_0}",
     run=apparent_bolometric,
     parse=lambda q: _pairs(q, r"apparent bolometric magnitude", {"l": "L", "d": "d"}, phys={"L", "d"}),
     reference=ref_apparent_bolometric,
     wrong=_safe(lambda L, d: (lambda F: _out(m_bol=-2.5 * math.log10(F / F0_BOL), F=F, M_bol=-2.5 * math.log10(L / L0_BOL)))(L / (math.pi * d * d))),
     random_inputs=lambda rng: {"L": _lu(rng, 1e24, 1e32), "d": _lu(rng, 1.3 * PC, 1e6 * PC)},
     templates=["Apparent bolometric magnitude: L = 1 Lsun, d = 1 au", "Apparent bolometric magnitude: L = 25 Lsun, d = 7.68 pc"],
     examples=[{"inputs": {"L": L0_BOL, "d": 100 * PC}, "outputs": {"m_bol": 5.0, "F": F0_BOL / 100}},
               {"inputs": {"L": 100 * L0_BOL, "d": 1000 * PC}, "outputs": {"m_bol": 5.0}}],
     questions=[{"q": "Apparent bolometric magnitude: L = 3.0128e28 W, d = 100 pc", "outputs": {"m_bol": 5.0}},
                {"q": "Apparent bolometric magnitude: L = 3.0128e28 W, d = 1 kpc", "outputs": {"m_bol": 10.0}},
                {"q": "Apparent bolometric magnitude: L = 1 Lsun, d = 0 pc", "outcome": "INVALID_INPUT"},
                {"q": "Apparent bolometric magnitude: L = 1 Lsun", "outcome": "NEEDS_INPUT"}])


def angular_size(D, d):
    _pos("the size and the distance", D, d)
    th = 2 * math.atan(D / (2 * d))
    return _out(theta=th, theta_arcsec=th / ARCSEC, theta_deg=th / DEG)


def ref_angular_size(D, d):
    import numpy as np
    a, b = np.array([-D / 2, d, 0.0]), np.array([D / 2, d, 0.0])     # the object's two edges seen from the origin
    th = _ref_angle_between(a, b)
    return {"theta": th, "theta_arcsec": th / ARCSEC}


_add("angular_size",
     name="Angular size", topic="observing / angular size",
     purpose="The angle subtended by an object of diameter D at distance d (exact: 2 atan(D / 2d)).",
     inputs=[_phys("D", "length", "diameter of the object"), _phys("d", "length", "distance to the object")],
     outputs=[_o("theta", "angle", "angular diameter"), _o("theta_arcsec", "number", "angular diameter in arcseconds"),
              _o("theta_deg", "number", "angular diameter in degrees")],
     primary="theta_arcsec", conditions=["D > 0", "d > 0 (distance to the object's centre plane)"],
     method="theta = 2 atan(D / (2 d)); for D << d this is D/d radians", latex=r"\theta = 2\arctan\frac{D}{2d} \approx \frac{D}{d}",
     run=angular_size,
     parse=lambda q: _pairs(q, r"angular size", {"D": "D", "d": "d"}, phys={"D", "d"}),
     reference=ref_angular_size,
     wrong=_safe(lambda D, d: (lambda th: _out(theta=th, theta_arcsec=th / ARCSEC, theta_deg=th / DEG))(2 * math.atan(D / d))),
     random_inputs=lambda rng: (lambda d: {"d": d, "D": d * _lu(rng, 1e-9, 2.0)})(_lu(rng, 1e3, 1e22)),
     templates=["Angular size: D = 3474.8 km, d = 384400 km", "Angular size: D = 1.392e9 m, d = 1 au"],
     examples=[{"inputs": {"D": 2.0, "d": 1.0}, "outputs": {"theta_deg": 90.0}},
               {"inputs": {"D": 2.0 * math.sqrt(3), "d": 1.0}, "outputs": {"theta_deg": 120.0}}],
     questions=[{"q": "Angular size: D = 2 m, d = 1 m", "outputs": {"theta_deg": 90.0}},
                {"q": "Angular size: D = 3474.8 km, d = 384400 km", "outputs": {"theta": 2 * math.atan(3474.8 / 768800)}},
                {"q": "Angular size: D = 2 m, d = -1 m", "outcome": "INVALID_INPUT"},
                {"q": "Angular size: D = 2 m", "outcome": "NEEDS_INPUT"}])


def linear_size(theta, d):
    _pos("the angle and the distance", theta, d)
    if theta >= math.pi:
        raise ValueError("the angular size must be smaller than 180 degrees")
    D = 2 * d * math.tan(theta / 2)
    return _out(D=D, D_km=D / 1e3, D_au=D / AU)


def ref_linear_size(theta, d):
    import numpy as np
    # each edge ray from the origin, direction at +-theta/2 from the line of sight (y axis), meets the plane y = d
    xs = []
    for s in (-1, 1):
        u_ = np.array([math.sin(s * theta / 2), math.cos(s * theta / 2)])
        t, x = np.linalg.solve(np.array([[u_[1], 0.0], [u_[0], -1.0]]), np.array([d, 0.0]))
        xs.append(x)
    return {"D": float(xs[1] - xs[0])}


_add("linear_size_from_angle",
     name="Linear size from angular size", topic="observing / angular size",
     purpose="The true diameter of an object of angular size theta at distance d (exact: 2 d tan(theta/2)).",
     inputs=[_phys("theta", "angle", "angular diameter (arcsec, arcmin, deg, rad)"), _phys("d", "length", "distance to the object")],
     outputs=[_o("D", "length", "diameter"), _o("D_km", "number", "diameter in kilometres"), _o("D_au", "number", "diameter in astronomical units")],
     primary="D", conditions=["0 < theta < 180 degrees", "d > 0"],
     method="D = 2 d tan(theta/2); for small angles D = theta d", latex=r"D = 2d\tan\frac{\theta}{2} \approx \theta d",
     run=linear_size,
     parse=lambda q: _pairs(q, r"linear size(?: from angular size)?", {"theta": "theta", "d": "d"}, phys={"theta", "d"}),
     reference=ref_linear_size,
     wrong=_safe(lambda theta, d: _out(D=d * theta / 2, D_km=d * theta / 2e3, D_au=d * theta / 2 / AU)),
     random_inputs=lambda rng: {"theta": _lu(rng, 1e-10, 2.5), "d": _lu(rng, 1e3, 1e24)},
     templates=["Linear size: theta = 31.6 arcmin, d = 384400 km", "Linear size from angular size: theta = 2 arcsec, d = 10 Mpc"],
     examples=[{"inputs": {"theta": math.pi / 2, "d": 1.0}, "outputs": {"D": 2.0}},
               {"inputs": {"theta": 2 * math.pi / 3, "d": 1.0}, "outputs": {"D": 2 * math.sqrt(3)}}],
     questions=[{"q": "Linear size: theta = 90 deg, d = 1 m", "outputs": {"D": 2.0}},
                {"q": "Linear size: theta = 1 arcsec, d = 1 pc", "outputs": {"D_au": 2 * PC * math.tan(ARCSEC / 2) / AU}},
                {"q": "Linear size: theta = 200 deg, d = 1 m", "outcome": "INVALID_INPUT"},
                {"q": "Linear size: theta = 1 arcsec", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ stars and blackbody radiation
def stellar_luminosity(R, T):
    _pos("the radius and the temperature", R, T)
    L = 4 * math.pi * R * R * SIGMA * T ** 4
    return _out(L=L, L_Lsun=L / LSUN)


_add("stellar_luminosity",
     name="Luminosity of a star (Stefan-Boltzmann)", topic="stars / luminosity",
     purpose="The luminosity of a star of radius R radiating as a blackbody at effective temperature T: L = 4 pi R^2 sigma T^4.",
     inputs=[_phys("R", "length", "stellar radius (m, km, Rsun)"), _phys("T", "temperature", "effective temperature")],
     outputs=[_o("L", "power", "luminosity"), _o("L_Lsun", "number", "luminosity in solar luminosities")],
     primary="L", conditions=["R > 0", "T > 0"],
     method="L = 4 pi R^2 sigma T^4 with sigma = 2 pi^5 k^4 / (15 h^3 c^2)", latex=r"L = 4\pi R^2 \sigma T_\mathrm{eff}^4",
     run=stellar_luminosity,
     parse=lambda q: _pairs(q, r"stellar luminosity|luminosity of a star", {"r": "R", "t": "T"}, phys={"R", "T"}),
     reference=lambda R, T: {"L": 4 * math.pi * R * R * _ref_bb_flux(T)},
     wrong=_safe(lambda R, T: _out(L=SIGMA * R * R * T ** 4, L_Lsun=SIGMA * R * R * T ** 4 / LSUN)),
     random_inputs=lambda rng: {"R": _lu(rng, 1e6, 1e12), "T": rng.uniform(2000, 50000)},
     templates=["Stellar luminosity: R = 1 Rsun, T = 5772 K", "Luminosity of a star: R = 7.0e8 m, T = 9600 K"],
     examples=[{"inputs": {"R": 1.0, "T": 100.0}, "outputs": {"L": 4 * math.pi * SIGMA * 1e8}},
               {"inputs": {"R": RSUN, "T": 5772.0}, "outputs": {"L_Lsun": 4 * math.pi * RSUN ** 2 * SIGMA * 5772.0 ** 4 / LSUN}}],
     questions=[{"q": "Stellar luminosity: R = 1 Rsun, T = 5772 K", "outputs": {"L_Lsun": 4 * math.pi * RSUN ** 2 * SIGMA * 5772.0 ** 4 / LSUN}},
                {"q": "Stellar luminosity: R = 1 m, T = 100 K", "outputs": {"L": 4 * math.pi * SIGMA * 1e8}},
                {"q": "Stellar luminosity: R = 1 Rsun, T = -5 K", "outcome": "INVALID_INPUT"},
                {"q": "Stellar luminosity: R = 1 Rsun", "outcome": "NEEDS_INPUT"}])


def stellar_radius(L, T):
    _pos("the luminosity and the temperature", L, T)
    R = math.sqrt(L / (4 * math.pi * SIGMA)) / (T * T)
    return _out(R=R, R_Rsun=R / RSUN)


_add("stellar_radius",
     name="Radius of a star from luminosity and temperature", topic="stars / radius",
     purpose="The radius of a star from its luminosity and effective temperature: R = sqrt(L / (4 pi sigma T^4)).",
     inputs=[_phys("L", "power", "luminosity (W or Lsun)"), _phys("T", "temperature", "effective temperature")],
     outputs=[_o("R", "length", "stellar radius"), _o("R_Rsun", "number", "radius in solar radii")],
     primary="R", conditions=["L > 0", "T > 0", "blackbody emission"],
     method="R = sqrt(L / (4 pi sigma)) / T^2", latex=r"R = \sqrt{\frac{L}{4\pi\sigma T_\mathrm{eff}^4}}",
     run=stellar_radius,
     parse=lambda q: _pairs(q, r"stellar radius|radius of a star", {"l": "L", "t": "T"}, phys={"L", "T"}),
     reference=lambda L, T: (lambda R: {"R": R, "R_Rsun": R / RSUN})(math.sqrt(L / (4 * math.pi * _ref_bb_flux(T)))),
     wrong=_safe(lambda L, T: (lambda R: _out(R=R, R_Rsun=R / RSUN))(math.sqrt(L / (SIGMA * T ** 4)))),
     random_inputs=lambda rng: {"L": _lu(rng, 1e22, 1e33), "T": rng.uniform(2000, 50000)},
     templates=["Stellar radius: L = 1 Lsun, T = 5772 K", "Radius of a star: L = 40 Lsun, T = 9940 K"],
     examples=[{"inputs": {"L": 4 * math.pi * SIGMA * 1e8, "T": 100.0}, "outputs": {"R": 1.0}},
               {"inputs": {"L": 4 * math.pi * SIGMA * 4e8 * 1e8, "T": 100.0}, "outputs": {"R": 2e4}}],
     questions=[{"q": "Stellar radius: L = 1 Lsun, T = 5772 K", "outputs": {"R": math.sqrt(LSUN / (4 * math.pi * SIGMA)) / 5772.0 ** 2}},
                {"q": "Stellar radius: L = 1 Lsun, T = 0 K", "outcome": "INVALID_INPUT"},
                {"q": "Stellar radius: T = 5772 K", "outcome": "NEEDS_INPUT"}])


def effective_temperature(L, R):
    _pos("the luminosity and the radius", L, R)
    T = (L / (4 * math.pi * SIGMA * R * R)) ** 0.25
    return _out(T=T)


def ref_effective_temperature(L, R):
    from scipy.optimize import brentq
    I0 = _ref_planck_integral()
    flux = lambda T: 2 * math.pi * KB ** 4 * T ** 4 / (H ** 3 * C ** 2) * I0        # noqa: E731  Planck, integrated
    return {"T": brentq(lambda T: 4 * math.pi * R * R * flux(T) / L - 1, 1e-3, 1e8, xtol=1e-14, rtol=8.9e-16, maxiter=500)}


_add("effective_temperature",
     name="Effective temperature of a star", topic="stars / effective temperature",
     purpose="The effective temperature of a star from its luminosity and radius: T = (L / (4 pi R^2 sigma))^(1/4).",
     inputs=[_phys("L", "power", "luminosity (W or Lsun)"), _phys("R", "length", "stellar radius (m, km, Rsun)")],
     outputs=[_o("T", "temperature", "effective temperature")],
     primary="T", conditions=["L > 0", "R > 0", "blackbody emission"],
     method="T = (L / (4 pi R^2 sigma))^(1/4)", latex=r"T_\mathrm{eff} = \left(\frac{L}{4\pi R^2\sigma}\right)^{1/4}",
     run=effective_temperature,
     parse=lambda q: _pairs(q, r"effective temperature", {"l": "L", "r": "R"}, phys={"L", "R"}),
     reference=ref_effective_temperature,
     wrong=_safe(lambda L, R: _out(T=(L / (4 * math.pi * SIGMA * R * R)) ** 0.5)),
     random_inputs=lambda rng: (lambda R, T: {"R": R, "L": 4 * math.pi * R * R * SIGMA * T ** 4})(_lu(rng, 1e6, 1e12), rng.uniform(2000, 50000)),
     templates=["Effective temperature: L = 1 Lsun, R = 1 Rsun", "Effective temperature: L = 3.828e26 W, R = 6.957e8 m"],
     examples=[{"inputs": {"L": 4 * math.pi * SIGMA * 1e8, "R": 1.0}, "outputs": {"T": 100.0}},
               {"inputs": {"L": 4 * math.pi * SIGMA * 6.25e18, "R": 4.0}, "outputs": {"T": 25000.0}}],
     questions=[{"q": "Effective temperature: L = 1 Lsun, R = 1 Rsun", "outputs": {"T": (LSUN / (4 * math.pi * SIGMA * RSUN ** 2)) ** 0.25}},
                {"q": "Effective temperature: L = 1 Lsun, R = 0 m", "outcome": "INVALID_INPUT"},
                {"q": "Effective temperature: L = 1 Lsun", "outcome": "NEEDS_INPUT"}])


def planet_equilibrium(T_star, R_star, a, A=0.0):
    _pos("the stellar temperature, the stellar radius and the orbital distance", T_star, R_star, a)
    if not 0 <= A < 1:
        raise ValueError("the Bond albedo A must satisfy 0 <= A < 1")
    if a <= R_star:
        raise ValueError("the orbital distance must exceed the stellar radius")
    T = T_star * math.sqrt(R_star / (2 * a)) * (1 - A) ** 0.25
    S = SIGMA * T_star ** 4 * (R_star / a) ** 2
    return _out(T_eq=T, S=S)


def ref_planet_equilibrium(T_star, R_star, a, A=0.0):
    from scipy.optimize import brentq
    I0 = _ref_planck_integral()
    flux = lambda T: 2 * math.pi * KB ** 4 * T ** 4 / (H ** 3 * C ** 2) * I0        # noqa: E731  Planck, integrated
    L = 4 * math.pi * R_star ** 2 * flux(T_star)
    S = L / (4 * math.pi * a * a)
    Rp = 1.0                                     # absorbed (1 - A) S pi Rp^2 = emitted 4 pi Rp^2 F(T): Rp cancels
    T = brentq(lambda T: 4 * math.pi * Rp ** 2 * flux(T) / ((1 - A) * S * math.pi * Rp ** 2) - 1, 1e-6, 1e7,
               xtol=1e-14, rtol=8.9e-16, maxiter=500)
    return {"T_eq": T, "S": S}


_add("planet_equilibrium_temperature",
     name="Equilibrium temperature of a planet", topic="planets / equilibrium temperature",
     purpose="The equilibrium temperature of a rapidly rotating planet of Bond albedo A at distance a from a star of temperature T and radius R, and the stellar flux it receives.",
     inputs=[_phys("T_star", "temperature", "effective temperature of the star"), _phys("R_star", "length", "radius of the star (m, Rsun)"),
             _phys("a", "length", "orbital distance (m, au)"), _numin("A", "Bond albedo (0 to 1, default 0)", required=False)],
     outputs=[_o("T_eq", "temperature", "equilibrium temperature"), _o("S", "intensity", "stellar flux at the planet")],
     primary="T_eq", conditions=["0 <= A < 1", "a > R_star", "full heat redistribution, unit emissivity"],
     method="(1 - A) S pi Rp^2 = 4 pi Rp^2 sigma T^4 with S = sigma T*^4 (R*/a)^2: T = T* sqrt(R*/(2a)) (1 - A)^(1/4)",
     latex=r"T_\mathrm{eq} = T_\star\sqrt{\frac{R_\star}{2a}}\,(1 - A)^{1/4}",
     run=planet_equilibrium,
     parse=lambda q: _pairs(q, r"(?:planet )?equilibrium temperature", {"t_star": "T_star", "tstar": "T_star", "r_star": "R_star",
                                                                          "rstar": "R_star", "a": "a", "A": "A", "albedo": "A"},
                            phys={"T_star", "R_star", "a"}),
     reference=ref_planet_equilibrium,
     wrong=_safe(lambda T_star, R_star, a, A=0.0: _out(T_eq=T_star * math.sqrt(R_star / a) * (1 - A) ** 0.25,
                                                      S=SIGMA * T_star ** 4 * (R_star / a) ** 2)),
     random_inputs=lambda rng: (lambda R: {"T_star": rng.uniform(2500, 40000), "R_star": R, "a": R * _lu(rng, 3, 1e5),
                                           **({"A": rng.uniform(0, 0.9)} if rng.random() < 0.7 else {})})(_lu(rng, 1e7, 1e11)),
     templates=["Equilibrium temperature: T_star = 5772 K, R_star = 1 Rsun, a = 1 au, A = 0.3",
                "Planet equilibrium temperature: T_star = 3300 K, R_star = 0.2 Rsun, a = 0.05 au"],
     examples=[{"inputs": {"T_star": 1000.0, "R_star": 2.0, "a": 4.0}, "outputs": {"T_eq": 500.0}},
               {"inputs": {"T_star": 1000.0, "R_star": 2.0, "a": 4.0, "A": 0.9375}, "outputs": {"T_eq": 250.0}}],
     questions=[{"q": "Equilibrium temperature: T_star = 1000 K, R_star = 2 m, a = 4 m", "outputs": {"T_eq": 500.0}},
                {"q": "Equilibrium temperature: T_star = 5772 K, R_star = 1 Rsun, a = 1 au, A = 0.3",
                 "outputs": {"T_eq": 5772 * math.sqrt(RSUN / (2 * AU)) * 0.7 ** 0.25}},
                {"q": "Equilibrium temperature: T_star = 5772 K, R_star = 1 Rsun, a = 1 au, A = 1.5", "outcome": "INVALID_INPUT"},
                {"q": "Equilibrium temperature: T_star = 5772 K, a = 1 au", "outcome": "NEEDS_INPUT"}])


def _saha_s(T, n):
    return (2 * math.pi * ME * KB * T / H ** 2) ** 1.5 * math.exp(-CHI_H / (KB * T)) / n


def saha_hydrogen(T, n):
    _pos("the temperature and the density", T, n)
    if not 1000 <= T <= 1e6:
        raise ValueError("the temperature must lie between 1000 K and 1e6 K")
    s = _saha_s(T, n)
    if not 1e-200 < s < 1e14:
        raise ValueError("the hydrogen is completely neutral or completely ionized at these conditions (fraction not resolvable)")
    x = 2 * s / (s + math.sqrt(s * s + 4 * s))
    return _out(x=x, neutral=x * x / s, n_e=x * n)


def ref_saha_hydrogen(T, n):
    import astropy.units as u
    import plasmapy.formulary as pf
    from scipy.optimize import brentq

    def f(z):                                   # z = ln(x/(1-x)); plasmapy's Saha ratio n_II/n_I at n_e = x n
        x = 1 / (1 + math.exp(-z))
        ratio = pf.Saha(1, 2, x * n * u.m ** -3, CHI_H * u.J, T * u.K)
        return z - math.log(float(ratio))
    z = brentq(f, -700, 700, xtol=1e-13, rtol=8.9e-16, maxiter=500)
    return {"x": 1 / (1 + math.exp(-z))}


def _saha_random(rng):
    T = rng.uniform(4000, 40000)
    s = _lu(rng, 1e-6, 1e4)
    return {"T": T, "n": (2 * math.pi * ME * KB * T / H ** 2) ** 1.5 * math.exp(-CHI_H / (KB * T)) / s}


_add("saha_hydrogen_ionization",
     name="Ionization fraction of hydrogen (Saha equation)", topic="stars / stellar atmospheres",
     purpose="The ionized fraction of pure hydrogen in thermal equilibrium at temperature T and total hydrogen density n (Saha equation, ground-state partition functions).",
     inputs=[_phys("T", "temperature", "temperature"), _phys("n", "number_density", "total density of hydrogen nuclei (m^-3 or cm^-3)")],
     outputs=[_o("x", "number", "ionized fraction n_II / n"), _o("neutral", "number", "neutral fraction n_I / n"),
              _o("n_e", "number_density", "electron density")],
     primary="x", conditions=["1000 K <= T <= 1e6 K", "pure hydrogen, n_e = n_II", "g_I = 2, g_II = 1, chi = 13.598 eV"],
     method="x^2/(1 - x) = (1/n) (2 pi m_e k T / h^2)^(3/2) exp(-chi / kT), solved in a cancellation-free form",
     latex=r"\frac{x^2}{1-x} = \frac{1}{n}\left(\frac{2\pi m_e k T}{h^2}\right)^{3/2} e^{-\chi/kT}",
     run=saha_hydrogen,
     parse=lambda q: _pairs(q, r"saha (?:equation|ionization)(?: for hydrogen)?|hydrogen ionization fraction", {"t": "T", "n": "n"}, phys={"T", "n"}),
     reference=ref_saha_hydrogen,
     wrong=_safe(lambda T, n: (lambda s: _out(x=s / (1 + s), neutral=1 / (1 + s), n_e=n * s / (1 + s)))(_saha_s(T, n))),
     random_inputs=_saha_random,
     templates=["Saha equation: T = 10000 K, n = 1e20 m^-3", "Hydrogen ionization fraction: T = 8000 K, n = 1e15 cm^-3"],
     examples=[{"inputs": {"T": 10000.0, "n": (2 * math.pi * ME * KB * 1e4 / H ** 2) ** 1.5 * math.exp(-CHI_H / (KB * 1e4)) * 2}, "outputs": {"x": 0.5}},
               {"inputs": {"T": 8000.0, "n": (2 * math.pi * ME * KB * 8e3 / H ** 2) ** 1.5 * math.exp(-CHI_H / (KB * 8e3)) / 2}, "outputs": {"x": math.sqrt(3) - 1}}],
     questions=[{"q": "Saha equation: T = 10000 K, n = 1e20 m^-3", "outputs": {"x": (lambda s: 2 * s / (s + math.sqrt(s * s + 4 * s)))(_saha_s(1e4, 1e20))}},
                {"q": "Saha equation: T = 300 K, n = 1e20 m^-3", "outcome": "INVALID_INPUT"},
                {"q": "Saha equation: T = 10000 K", "outcome": "NEEDS_INPUT"}])


def schwarzschild_radius(M):
    _pos("the mass", M)
    rs = 2 * G * M / C ** 2
    return _out(rs=rs, rs_km=rs / 1e3, M_Msun=M / MSUN)


def ref_schwarzschild_radius(M):
    from scipy.optimize import brentq
    # the radius at which the escape speed (from the work integral to infinity) equals the speed of light
    lr = brentq(lambda lr: math.log(2 * _ref_work_to_infinity(G * M, math.exp(lr))) - 2 * math.log(C), -100, 200,
                xtol=1e-14, rtol=8.9e-16, maxiter=500)
    return {"rs": math.exp(lr)}


_add("schwarzschild_radius",
     name="Schwarzschild radius", topic="compact objects / black holes",
     purpose="The Schwarzschild radius (event horizon of a non-rotating black hole) of a mass M: r_s = 2 G M / c^2.",
     inputs=[_phys("M", "mass", "mass (kg, Msun, Mearth)")],
     outputs=[_o("rs", "length", "Schwarzschild radius"), _o("rs_km", "number", "Schwarzschild radius in kilometres"),
              _o("M_Msun", "number", "mass in solar masses")],
     primary="rs", conditions=["M > 0", "non-rotating, uncharged mass"],
     method="r_s = 2 G M / c^2", latex=r"r_s = \frac{2GM}{c^2}",
     run=schwarzschild_radius,
     parse=lambda q: _pairs(q, r"schwarzschild radius", {"m": "M", "mass": "M"}, phys={"M"}),
     reference=ref_schwarzschild_radius,
     wrong=_safe(lambda M: _out(rs=G * M / C ** 2, rs_km=G * M / C ** 2 / 1e3, M_Msun=M / MSUN)),
     random_inputs=lambda rng: {"M": _lu(rng, 1e20, 1e42)},
     templates=["Schwarzschild radius: M = 1 Msun", "Schwarzschild radius: M = 4.3e6 Msun"],
     examples=[{"inputs": {"M": MSUN}, "outputs": {"rs": 2 * 1.3271244e20 / C ** 2}},
               {"inputs": {"M": C ** 2 / (2 * G)}, "outputs": {"rs": 1.0}}],
     questions=[{"q": "Schwarzschild radius: M = 1 Msun", "outputs": {"rs_km": 2 * 1.3271244e20 / C ** 2 / 1e3}},
                {"q": "Schwarzschild radius: M = 5.972e24 kg", "outputs": {"rs": 2 * G * 5.972e24 / C ** 2}},
                {"q": "Schwarzschild radius: M = 0 kg", "outcome": "INVALID_INPUT"},
                {"q": "Schwarzschild radius:", "outcome": "NEEDS_INPUT"}])


def scale_height(T, mu, g):
    _pos("the temperature, the molar mass and the gravity", T, mu, g)
    Hs = RGAS * T / (mu * g)
    return _out(H=Hs, H_km=Hs / 1e3)


def ref_scale_height(T, mu, g):
    from scipy.integrate import solve_ivp
    # hydrostatic equilibrium dP/dz = -rho g with the ideal gas rho = P mu / (R T), integrated over Z = 1000 m
    Z = 1000.0
    sol = solve_ivp(lambda z, P: [-P[0] * mu * g / (RGAS * T)], (0, Z), [1.0], method="DOP853", rtol=1e-13, atol=1e-300)
    return {"H": -Z / math.log(sol.y[0, -1])}


_add("atmospheric_scale_height",
     name="Scale height of an isothermal atmosphere", topic="planets / atmospheres",
     purpose="The pressure scale height H = R T / (mu g) of an isothermal planetary atmosphere: pressure falls by a factor e every H.",
     inputs=[_phys("T", "temperature", "temperature of the atmosphere"), _phys("mu", "molar_mass", "mean molar mass (kg/mol or g/mol)"),
             _phys("g", "acceleration", "surface gravity")],
     outputs=[_o("H", "length", "scale height"), _o("H_km", "number", "scale height in kilometres")],
     primary="H", conditions=["T, mu, g > 0", "isothermal ideal gas, constant gravity"],
     method="H = R T / (mu g)", latex=r"H = \frac{RT}{\mu g}",
     run=scale_height,
     parse=lambda q: _pairs(q, r"(?:atmospheric )?scale height", {"t": "T", "mu": "mu", "m": "mu", "g": "g"}, phys={"T", "mu", "g"}),
     reference=ref_scale_height,
     wrong=_safe(lambda T, mu, g: _out(H=RGAS * T / g, H_km=RGAS * T / g / 1e3)),
     random_inputs=lambda rng: {"T": rng.uniform(50, 3000), "mu": rng.uniform(0.002, 0.05), "g": rng.uniform(1, 30)},
     templates=["Scale height: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2", "Atmospheric scale height: T = 210 K, mu = 43.3 g/mol, g = 3.71 m/s^2"],
     examples=[{"inputs": {"T": 300.0, "mu": RGAS * 300 / 9.8 / 1000, "g": 9.8}, "outputs": {"H": 1000.0}},
               {"inputs": {"T": 288.0, "mu": 0.02897, "g": 9.81}, "outputs": {"H": RGAS * 288 / (0.02897 * 9.81)}}],
     questions=[{"q": "Scale height: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2", "outputs": {"H": RGAS * 288 / (0.02897 * 9.81)}},
                {"q": "Scale height: T = 288 K, mu = 0 g/mol, g = 9.81 m/s^2", "outcome": "INVALID_INPUT"},
                {"q": "Scale height: T = 288 K, g = 9.81 m/s^2", "outcome": "NEEDS_INPUT"}])


def free_fall_time(rho):
    _pos("the density", rho)
    t = math.sqrt(3 * math.pi / (32 * G * rho))
    return _out(t_ff=t, t_ff_yr=t / YR)


def ref_free_fall_time(rho):
    from scipy.integrate import quad
    R = 1.0                                     # the collapse time does not depend on the cloud's radius
    GM = G * 4 / 3 * math.pi * R ** 3 * rho
    # energy conservation for the edge: dt = dr / sqrt(2 GM (1/r - 1/R)); the (R - r)^(-1/2) singularity as a weight
    t, _ = quad(lambda r: math.sqrt(r * R / (2 * GM)), 0, R, weight="alg", wvar=(0, -0.5), epsabs=0, epsrel=2e-14)
    return {"t_ff": t}


_add("free_fall_time",
     name="Free-fall time of a uniform cloud", topic="stars / star formation",
     purpose="The time a uniform, pressureless sphere of density rho takes to collapse from rest under its own gravity.",
     inputs=[_phys("rho", "density", "mean density (kg/m^3; g/cm^3)")],
     outputs=[_o("t_ff", "time", "free-fall time"), _o("t_ff_yr", "number", "free-fall time in years")],
     primary="t_ff", conditions=["rho > 0", "uniform, pressureless, non-rotating sphere starting at rest"],
     method="t_ff = sqrt(3 pi / (32 G rho))", latex=r"t_\mathrm{ff} = \sqrt{\frac{3\pi}{32\,G\rho}}",
     run=free_fall_time,
     parse=lambda q: _pairs(q, r"free[- ]fall time", {"rho": "rho", "density": "rho"}, phys={"rho"}),
     reference=ref_free_fall_time,
     wrong=_safe(lambda rho: _out(t_ff=math.sqrt(1 / (G * rho)), t_ff_yr=math.sqrt(1 / (G * rho)) / YR)),
     random_inputs=lambda rng: {"rho": _lu(rng, 1e-22, 1e5)},
     templates=["Free-fall time: rho = 1e-17 kg/m^3", "Free-fall time: rho = 1.41 g/cm^3"],
     examples=[{"inputs": {"rho": 3 * math.pi / (32 * G)}, "outputs": {"t_ff": 1.0}},
               {"inputs": {"rho": 3 * math.pi / (32 * G) / 4}, "outputs": {"t_ff": 2.0}}],
     questions=[{"q": "Free-fall time: rho = 1e-17 kg/m^3", "outputs": {"t_ff": math.sqrt(3 * math.pi / (32 * G * 1e-17))}},
                {"q": "Free-fall time: rho = -1 kg/m^3", "outcome": "INVALID_INPUT"},
                {"q": "Free-fall time:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ orbits and celestial mechanics
def binary_masses(P, a1, a2):
    _pos("the period and the two semi-major axes", P, a1, a2)
    a = a1 + a2
    M = 4 * math.pi ** 2 * a ** 3 / (G * P * P)
    return _out(M_total=M, M_total_Msun=M / MSUN, M1_Msun=M * a2 / a / MSUN, M2_Msun=M * a1 / a / MSUN, a_au=a / AU)


def ref_binary_masses(P, a1, a2):
    a = a1 + a2
    T_test = _ref_kepler_period(a, G * MSUN)          # the period this orbit would have around one solar mass
    M = MSUN * (T_test / P) ** 2                      # dt = r^2/h dtheta with h ~ sqrt(M): T scales as M^(-1/2)
    return {"M_total_Msun": M / MSUN, "M1_Msun": M * a2 / a / MSUN, "M2_Msun": M * a1 / a / MSUN}


def _binary_random(rng):
    M, q, P = _lu(rng, 0.1, 100) * MSUN, rng.uniform(0.05, 1.0), _lu(rng, 86400.0, 1000 * YR)
    a = (G * M * P * P / (4 * math.pi ** 2)) ** (1 / 3)
    return {"P": P, "a1": a * q / (1 + q), "a2": a / (1 + q)}


_add("binary_star_masses",
     name="Masses of a visual binary star", topic="orbits / binary stars",
     purpose="The total and individual masses of a binary star from its orbital period and each star's semi-major axis about the centre of mass (Kepler's third law and the centre-of-mass condition).",
     inputs=[_phys("P", "time", "orbital period (s, day, yr)"), _phys("a1", "length", "semi-major axis of star 1 about the centre of mass (au, m)"),
             _phys("a2", "length", "semi-major axis of star 2 about the centre of mass (au, m)")],
     outputs=[_o("M_total", "mass", "total mass"), _o("M_total_Msun", "number", "total mass in solar masses"),
              _o("M1_Msun", "number", "mass of star 1 in solar masses"), _o("M2_Msun", "number", "mass of star 2 in solar masses"),
              _o("a_au", "number", "relative semi-major axis a1 + a2 in au")],
     primary="M_total_Msun", conditions=["P, a1, a2 > 0", "true (de-projected) semi-major axes"],
     method="M1 + M2 = 4 pi^2 (a1 + a2)^3 / (G P^2); M1 a1 = M2 a2",
     latex=r"M_1 + M_2 = \frac{4\pi^2 (a_1 + a_2)^3}{G P^2},\quad M_1 a_1 = M_2 a_2",
     run=binary_masses,
     parse=lambda q: _pairs(q, r"binary star masses|masses of a (?:visual )?binary(?: star)?", {"p": "P", "a1": "a1", "a2": "a2"},
                            phys={"P", "a1", "a2"}),
     reference=ref_binary_masses,
     wrong=_safe(lambda P, a1, a2: (lambda M: _out(M_total=M, M_total_Msun=M / MSUN, M1_Msun=M / 2 / MSUN, M2_Msun=M / 2 / MSUN,
                                                   a_au=a1 / AU))(4 * math.pi ** 2 * a1 ** 3 / (G * P * P))),
     random_inputs=_binary_random,
     templates=["Binary star masses: P = 50.1 yr, a1 = 6.4 au, a2 = 13.4 au", "Masses of a binary star: P = 10 yr, a1 = 2 au, a2 = 3 au"],
     examples=[{"inputs": {"P": 2 * math.pi, "a1": 0.25, "a2": 0.75}, "outputs": {"M_total": 1.0 / G, "M1_Msun": 0.75 / G / MSUN}},
               {"inputs": {"P": YR, "a1": 0.5 * AU, "a2": 0.5 * AU}, "outputs": {"M_total": 4 * math.pi ** 2 * AU ** 3 / (G * YR * YR)}}],
     questions=[{"q": "Binary star masses: P = 1 yr, a1 = 0.25 au, a2 = 0.75 au",
                 "outputs": {"M_total": 4 * math.pi ** 2 * AU ** 3 / (G * YR * YR), "M1_Msun": 0.75 * 4 * math.pi ** 2 * AU ** 3 / (G * YR * YR) / MSUN}},
                {"q": "Binary star masses: P = 1 yr, a1 = 0 au, a2 = 0.75 au", "outcome": "INVALID_INPUT"},
                {"q": "Binary star masses: P = 1 yr, a1 = 0.25 au", "outcome": "NEEDS_INPUT"}])


def hohmann(r1, r2, M):
    _pos("the two orbit radii and the central mass", r1, r2, M)
    if abs(r1 - r2) <= 1e-9 * max(r1, r2):
        raise ValueError("the two orbits must have different radii")
    mu = G * M
    v1, v2 = math.sqrt(mu / r1), math.sqrt(mu / r2)
    vp = math.sqrt(2 * mu * r2 / (r1 * (r1 + r2)))
    va = math.sqrt(2 * mu * r1 / (r2 * (r1 + r2)))
    at = (r1 + r2) / 2
    t = math.pi * math.sqrt(at ** 3 / mu)
    dv1, dv2 = abs(vp - v1), abs(v2 - va)
    return _out(dv1=dv1, dv2=dv2, dv=dv1 + dv2, t_transfer=t, t_days=t / 86400, a_transfer=at)


def ref_hohmann(r1, r2, M):
    import numpy as np
    mu = G * M
    # speeds at the two apsides of the transfer ellipse from conservation of energy and angular momentum
    vp2, va2 = np.linalg.solve(np.array([[1.0, -1.0], [r1 * r1, -r2 * r2]]), np.array([2 * mu * (1 / r1 - 1 / r2), 0.0]))
    vp, va = math.sqrt(vp2), math.sqrt(va2)
    v1, v2 = math.sqrt(mu / r1), math.sqrt(mu / r2)                  # circular speeds (gravity = centripetal force)
    e, p = abs(r2 - r1) / (r1 + r2), 2 * r1 * r2 / (r1 + r2)
    t = _ref_flight_time(mu, p, e, 0.0, math.pi)                     # half a revolution by Kepler's second law
    return {"dv1": abs(vp - v1), "dv2": abs(v2 - va), "t_days": t / 86400}


def _hohmann_random(rng):
    M = _lu(rng, 1e22, 1e31)
    r1 = _lu(rng, 1e6, 1e13)
    return {"M": M, "r1": r1, "r2": r1 * rng.choice([_lu(rng, 1.1, 50), 1 / _lu(rng, 1.1, 50)])}


_add("hohmann_transfer",
     name="Hohmann transfer between circular orbits", topic="orbits / orbital manoeuvres",
     purpose="The two velocity changes, their sum and the transfer time of a Hohmann transfer between coplanar circular orbits of radii r1 and r2 around a body of mass M.",
     inputs=[_phys("r1", "length", "radius of the initial circular orbit (km, au)"), _phys("r2", "length", "radius of the final circular orbit"),
             _phys("M", "mass", "mass of the central body (kg, Mearth, Msun)")],
     outputs=[_o("dv1", "speed", "first burn (departure)"), _o("dv2", "speed", "second burn (arrival)"), _o("dv", "speed", "total delta-v"),
              _o("t_transfer", "time", "transfer time (half the transfer ellipse)"), _o("t_days", "number", "transfer time in days"),
              _o("a_transfer", "length", "semi-major axis of the transfer ellipse")],
     primary="dv", conditions=["r1, r2, M > 0", "r1 != r2", "impulsive burns, coplanar circular orbits"],
     method="vp = sqrt(2 mu r2 / (r1 (r1 + r2))), va = sqrt(2 mu r1 / (r2 (r1 + r2))); t = pi sqrt(a^3/mu)",
     latex=r"\Delta v_1 = \sqrt{\frac{\mu}{r_1}}\left(\sqrt{\frac{2r_2}{r_1 + r_2}} - 1\right),\ \Delta v_2 = \sqrt{\frac{\mu}{r_2}}\left(1 - \sqrt{\frac{2r_1}{r_1 + r_2}}\right),\ t = \pi\sqrt{\frac{a^3}{\mu}}",
     run=hohmann,
     parse=lambda q: _pairs(q, r"hohmann transfer", {"r1": "r1", "r2": "r2", "m": "M"}, phys={"r1", "r2", "M"}),
     reference=ref_hohmann,
     wrong=_safe(lambda r1, r2, M: (lambda o: {**o, "t_transfer": _r(2 * o["t_transfer"]), "t_days": _r(2 * o["t_days"])})(hohmann(r1, r2, M))),
     random_inputs=_hohmann_random,
     templates=["Hohmann transfer: r1 = 6678 km, r2 = 42164 km, M = 1 Mearth", "Hohmann transfer: r1 = 1 au, r2 = 1.524 au, M = 1 Msun"],
     examples=[{"inputs": {"r1": 1.0, "r2": 3.0, "M": 1 / G}, "outputs": {"dv1": math.sqrt(1.5) - 1, "dv2": math.sqrt(1 / 3) - math.sqrt(1 / 6),
                                                                       "t_transfer": math.pi * 8 ** 0.5}},
               {"inputs": {"r1": 3.0, "r2": 1.0, "M": 1 / G}, "outputs": {"dv": math.sqrt(1.5) - 1 + math.sqrt(1 / 3) - math.sqrt(1 / 6)}}],
     questions=[{"q": "Hohmann transfer: r1 = 6678 km, r2 = 42164 km, M = 5.972e24 kg",
                 "outputs": {"t_transfer": math.pi * math.sqrt(((6678e3 + 42164e3) / 2) ** 3 / (G * 5.972e24))}},
                {"q": "Hohmann transfer: r1 = 7000 km, r2 = 7000 km, M = 1 Mearth", "outcome": "INVALID_INPUT"},
                {"q": "Hohmann transfer: r1 = 7000 km, r2 = 42164 km", "outcome": "NEEDS_INPUT"}])


def apsis_speeds(rp, ra, M):
    _pos("the periapsis and apoapsis distances and the central mass", rp, ra, M)
    if rp > ra:
        raise ValueError("the periapsis distance must not exceed the apoapsis distance")
    mu = G * M
    a = (rp + ra) / 2
    e = (ra - rp) / (ra + rp)
    return _out(vp=math.sqrt(2 * mu * ra / (rp * (rp + ra))), va=math.sqrt(2 * mu * rp / (ra * (rp + ra))), a=a, e=e,
                P=2 * math.pi * math.sqrt(a ** 3 / mu))


def ref_apsis_speeds(rp, ra, M):
    import numpy as np
    mu = G * M
    vp2, va2 = np.linalg.solve(np.array([[1.0, -1.0], [rp * rp, -ra * ra]]), np.array([2 * mu * (1 / rp - 1 / ra), 0.0]))
    e, p = (ra - rp) / (ra + rp), 2 * rp * ra / (rp + ra)
    return {"vp": math.sqrt(vp2), "va": math.sqrt(va2), "P": _ref_flight_time(mu, p, e, 0.0, 2 * math.pi)}


_add("apsis_speeds",
     name="Speeds at periapsis and apoapsis", topic="orbits / elliptical orbits",
     purpose="The orbital speeds at the closest and farthest points of an elliptical orbit, with its semi-major axis, eccentricity and period.",
     inputs=[_phys("rp", "length", "periapsis distance from the centre of the central body"), _phys("ra", "length", "apoapsis distance"),
             _phys("M", "mass", "mass of the central body")],
     outputs=[_o("vp", "speed", "speed at periapsis"), _o("va", "speed", "speed at apoapsis"), _o("a", "length", "semi-major axis"),
              _o("e", "number", "eccentricity"), _o("P", "time", "orbital period")],
     primary="vp", conditions=["0 < rp < ra", "M > 0", "two-body orbit, the orbiting mass negligible"],
     method="energy and angular momentum: vp = sqrt(2 mu ra / (rp (rp + ra))), va = vp rp / ra",
     latex=r"v_p = \sqrt{\frac{2\mu\,r_a}{r_p (r_p + r_a)}},\quad v_a = \sqrt{\frac{2\mu\,r_p}{r_a (r_p + r_a)}}",
     run=apsis_speeds,
     parse=lambda q: _pairs(q, r"apsis speeds|speeds at periapsis and apoapsis", {"rp": "rp", "ra": "ra", "m": "M"}, phys={"rp", "ra", "M"}),
     reference=ref_apsis_speeds,
     wrong=_safe(lambda rp, ra, M: (lambda o: {**o, "vp": _r(math.sqrt(G * M / rp)), "va": _r(math.sqrt(G * M / ra))})(apsis_speeds(rp, ra, M))),
     random_inputs=lambda rng: (lambda rp: {"rp": rp, "ra": rp * _lu(rng, 1.05, 100), "M": _lu(rng, 1e20, 1e31)})(_lu(rng, 1e6, 1e13)),
     templates=["Apsis speeds: rp = 0.983 au, ra = 1.017 au, M = 1 Msun", "Speeds at periapsis and apoapsis: rp = 6700 km, ra = 46000 km, M = 1 Mearth"],
     examples=[{"inputs": {"rp": 1.0, "ra": 3.0, "M": 1 / G}, "outputs": {"vp": math.sqrt(1.5), "va": math.sqrt(1.5) / 3, "e": 0.5}},
               {"inputs": {"rp": 1.0, "ra": 7.0, "M": 1 / G}, "outputs": {"va": math.sqrt(1 / 28), "a": 4.0, "P": 16 * math.pi}}],
     questions=[{"q": "Apsis speeds: rp = 1 m, ra = 3 m, M = 1.4982e10 kg", "outputs": {"e": 0.5}},
                {"q": "Apsis speeds: rp = 3 au, ra = 1 au, M = 1 Msun", "outcome": "INVALID_INPUT"},
                {"q": "Apsis speeds: rp = 1 au, ra = 3 au", "outcome": "NEEDS_INPUT"}])


def escape_dv(M, r, v_inf=0.0):
    _pos("the central mass and the orbit radius", M, r)
    if v_inf < 0:
        raise ValueError("the hyperbolic excess speed must not be negative")
    mu = G * M
    vc = math.sqrt(mu / r)
    vd = math.sqrt(v_inf * v_inf + 2 * mu / r)
    return _out(dv=vd - vc, v_circ=vc, v_depart=vd)


def ref_escape_dv(M, r, v_inf=0.0):
    W = _ref_work_to_infinity(G * M, r)              # kinetic energy per kg needed to reach infinity at rest
    vd = math.sqrt(v_inf * v_inf + 2 * W)
    return {"dv": vd - math.sqrt(G * M / r), "v_depart": vd}


_add("escape_delta_v",
     name="Departure burn from a circular orbit", topic="orbits / orbital manoeuvres",
     purpose="The velocity change that takes a spacecraft from a circular orbit of radius r onto an escape (or hyperbolic) trajectory with excess speed v_inf.",
     inputs=[_phys("M", "mass", "mass of the central body"), _phys("r", "length", "radius of the circular orbit"),
             _phys("v_inf", "speed", "hyperbolic excess speed (default 0: just escape)", required=False)],
     outputs=[_o("dv", "speed", "departure delta-v"), _o("v_circ", "speed", "circular orbital speed"),
              _o("v_depart", "speed", "speed needed after the burn")],
     primary="dv", conditions=["M, r > 0", "v_inf >= 0", "a single impulsive burn"],
     method="dv = sqrt(v_inf^2 + 2 mu / r) - sqrt(mu / r)", latex=r"\Delta v = \sqrt{v_\infty^2 + \frac{2\mu}{r}} - \sqrt{\frac{\mu}{r}}",
     run=escape_dv,
     parse=lambda q: _pairs(q, r"departure burn|escape delta-v", {"m": "M", "r": "r", "v_inf": "v_inf", "vinf": "v_inf"}, phys={"M", "r", "v_inf"}),
     reference=ref_escape_dv,
     wrong=_safe(lambda M, r, v_inf=0.0: (lambda vd: _out(dv=vd, v_circ=math.sqrt(G * M / r), v_depart=vd))(math.sqrt(v_inf * v_inf + 2 * G * M / r))),
     random_inputs=lambda rng: {"M": _lu(rng, 1e20, 1e31), "r": _lu(rng, 1e6, 1e12),
                                **({"v_inf": _lu(rng, 10, 3e4)} if rng.random() < 0.6 else {})},
     templates=["Departure burn: M = 1 Mearth, r = 6678 km, v_inf = 2.94 km/s", "Escape delta-v: M = 1 Mearth, r = 6678 km"],
     examples=[{"inputs": {"M": 1 / G, "r": 1.0}, "outputs": {"dv": math.sqrt(2) - 1}},
               {"inputs": {"M": 1 / G, "r": 1.0, "v_inf": math.sqrt(2)}, "outputs": {"dv": 1.0}}],
     questions=[{"q": "Escape delta-v: M = 5.972e24 kg, r = 6678 km", "outputs": {"dv": (math.sqrt(2) - 1) * math.sqrt(G * 5.972e24 / 6678e3)}},
                {"q": "Departure burn: M = 1 Mearth, r = 6678 km, v_inf = -1 km/s", "outcome": "INVALID_INPUT"},
                {"q": "Departure burn: M = 1 Mearth", "outcome": "NEEDS_INPUT"}])


def plane_change(v1, v2, di):
    _pos("the two speeds", v1, v2)
    if not 0 <= di <= math.pi:
        raise ValueError("the angle between the velocities must lie between 0 and 180 degrees")
    dv = math.sqrt((v1 - v2) ** 2 + 4 * v1 * v2 * math.sin(di / 2) ** 2)
    return _out(dv=dv)


def ref_plane_change(v1, v2, di):
    import numpy as np
    a = np.array([v1, 0.0, 0.0])
    b = v2 * np.array([math.cos(di), 0.0, math.sin(di)])            # the new velocity, turned by di out of the plane
    return {"dv": float(np.linalg.norm(b - a))}


_add("plane_change_delta_v",
     name="Delta-v of a combined plane change", topic="orbits / orbital manoeuvres",
     purpose="The velocity change needed to turn a velocity v1 into a velocity v2 at an angle di to it (a plane change, possibly combined with a speed change).",
     inputs=[_phys("v1", "speed", "speed before the burn"), _phys("v2", "speed", "speed after the burn"),
             _phys("di", "angle", "angle between the two velocities (plane-change angle)")],
     outputs=[_o("dv", "speed", "delta-v of the burn")],
     primary="dv", conditions=["v1, v2 > 0", "0 <= di <= 180 degrees", "an impulsive burn"],
     method="law of cosines: dv^2 = v1^2 + v2^2 - 2 v1 v2 cos(di), written as (v1 - v2)^2 + 4 v1 v2 sin^2(di/2)",
     latex=r"\Delta v = \sqrt{v_1^2 + v_2^2 - 2 v_1 v_2\cos\Delta i}",
     run=plane_change,
     parse=lambda q: _pairs(q, r"plane change(?: delta-v)?", {"v1": "v1", "v2": "v2", "di": "di", "angle": "di"}, phys={"v1", "v2", "di"}),
     reference=ref_plane_change,
     wrong=_safe(lambda v1, v2, di: _out(dv=math.sqrt(abs(v1 * v1 + v2 * v2 - 2 * v1 * v2 * math.sin(di))))),
     random_inputs=lambda rng: {"v1": _lu(rng, 100, 5e4), "v2": _lu(rng, 100, 5e4), "di": rng.uniform(1, 179) * DEG},
     templates=["Plane change: v1 = 7.73 km/s, v2 = 7.73 km/s, di = 28.5 deg", "Plane change delta-v: v1 = 1.6 km/s, v2 = 3.07 km/s, di = 28.5 deg"],
     examples=[{"inputs": {"v1": 1.0, "v2": 1.0, "di": math.pi / 3}, "outputs": {"dv": 1.0}},
               {"inputs": {"v1": 3.0, "v2": 4.0, "di": math.pi / 2}, "outputs": {"dv": 5.0}}],
     questions=[{"q": "Plane change: v1 = 3 km/s, v2 = 4 km/s, di = 90 deg", "outputs": {"dv": 5000.0}},
                {"q": "Plane change: v1 = 7.7 km/s, v2 = 7.7 km/s, di = 60 deg", "outputs": {"dv": 7700.0}},
                {"q": "Plane change: v1 = 3 km/s, v2 = 4 km/s, di = 200 deg", "outcome": "INVALID_INPUT"},
                {"q": "Plane change: v1 = 3 km/s, di = 90 deg", "outcome": "NEEDS_INPUT"}])


def orbit_from_state(M, r, v, gamma=0.0):
    _pos("the central mass, the distance and the speed", M, r, v)
    if not abs(gamma) < math.pi / 2:
        raise ValueError("the flight-path angle must lie strictly between -90 and 90 degrees")
    mu = G * M
    s = r * v * v / mu
    if s >= 2:
        raise ValueError("the orbit is not bound (the speed is at or above the escape speed)")
    c, sn = math.cos(gamma), math.sin(gamma)
    a = r / (2 - s)
    e = math.hypot(s * c * c - 1, s * c * sn)
    e = 0.0 if e < 1e-12 else e                 # circular to rounding: exactly 0
    return _out(a=a, e=e, rp=a * (1 - e), ra=a * (1 + e), P=2 * math.pi * math.sqrt(a ** 3 / mu),
                energy=v * v / 2 - mu / r)


def ref_orbit_from_state(M, r, v, gamma=0.0):
    import numpy as np
    mu = G * M
    rv = np.array([r, 0.0, 0.0])
    vv = np.array([v * math.sin(gamma), v * math.cos(gamma), 0.0])
    hv = np.cross(rv, vv)
    ev = np.cross(vv, hv) / mu - rv / r                              # the Laplace-Runge-Lenz (eccentricity) vector
    e = float(np.linalg.norm(ev))
    p = float(np.dot(hv, hv)) / mu
    rp, ra = p / (1 + e), p / (1 - e)
    return {"e": e, "rp": rp, "ra": ra, "a": (rp + ra) / 2, "P": _ref_flight_time(mu, p, e, 0.0, 2 * math.pi)}


def _state_random(rng):
    while True:
        M, r = _lu(rng, 1e20, 1e31), _lu(rng, 1e6, 1e12)
        v = math.sqrt(G * M / r) * rng.uniform(0.6, 1.35)
        g = rng.uniform(-60, 60) * DEG
        o = orbit_from_state(M, r, v, g) if r * v * v / (G * M) < 1.9 else None
        if o and 0.05 < o["e"] < 0.95:
            return {"M": M, "r": r, "v": v, "gamma": g}


_add("orbit_from_state",
     name="Orbital elements from position and velocity", topic="orbits / orbit determination",
     purpose="The semi-major axis, eccentricity, periapsis and apoapsis distances, period and specific energy of a bound orbit from the distance, speed and flight-path angle at one point.",
     inputs=[_phys("M", "mass", "mass of the central body"), _phys("r", "length", "distance from the centre of the central body"),
             _phys("v", "speed", "orbital speed at that point"),
             _phys("gamma", "angle", "flight-path angle above the local horizontal (default 0)", required=False)],
     outputs=[_o("a", "length", "semi-major axis"), _o("e", "number", "eccentricity"), _o("rp", "length", "periapsis distance"),
              _o("ra", "length", "apoapsis distance"), _o("P", "time", "orbital period"), _o("energy", "latent_heat", "specific orbital energy (J/kg)")],
     primary="a", conditions=["v^2 < 2 G M / r (bound orbit)", "|gamma| < 90 degrees", "two-body motion"],
     method="vis-viva a = r / (2 - r v^2/mu); e^2 = (s cos^2 g - 1)^2 + (s cos g sin g)^2 with s = r v^2/mu",
     latex=r"a = \frac{r}{2 - r v^2/\mu},\quad e = \sqrt{1 - \frac{(r v\cos\gamma)^2}{\mu a}}",
     run=orbit_from_state,
     parse=lambda q: _pairs(q, r"orbit(?:al elements)? from (?:the )?state", {"m": "M", "r": "r", "v": "v", "gamma": "gamma"},
                            phys={"M", "r", "v", "gamma"}),
     reference=ref_orbit_from_state,
     wrong=_safe(lambda M, r, v, gamma=0.0: (lambda o: {**o, "a": _r(r / (1 - r * v * v / (G * M)) if r * v * v < G * M else -1.0)})(orbit_from_state(M, r, v, gamma))),
     random_inputs=_state_random,
     templates=["Orbit from state: M = 1 Mearth, r = 7000 km, v = 8.5 km/s, gamma = 10 deg", "Orbital elements from state: M = 1 Msun, r = 1 au, v = 35 km/s"],
     examples=[{"inputs": {"M": 1 / G, "r": 1.0, "v": 1.0}, "outputs": {"a": 1.0, "e": 0.0, "P": 2 * math.pi}},
               {"inputs": {"M": 1 / G, "r": 1.0, "v": math.sqrt(1.5)}, "outputs": {"a": 2.0, "e": 0.5, "ra": 3.0}}],
     questions=[{"q": "Orbit from state: M = 1.4982e10 kg, r = 1 m, v = 1.2 m/s",
                 "outputs": {"a": 1 / (2 - 1.44 / (G * 1.4982e10))}},
                {"q": "Orbit from state: M = 1 Mearth, r = 7000 km, v = 20 km/s", "outcome": "INVALID_INPUT"},
                {"q": "Orbit from state: M = 1 Mearth, r = 7000 km", "outcome": "NEEDS_INPUT"}])


def _kepler_E(Mn, e):
    """Eccentric anomaly in [0, 2 pi) from the mean anomaly Mn in [0, 2 pi): safeguarded Newton (always converges)."""
    lo, hi = 0.0, 2 * math.pi
    E = Mn + e * math.sin(Mn) if e < 0.8 else math.pi
    for _ in range(200):
        f = E - e * math.sin(E) - Mn
        if f > 0:
            hi = E
        else:
            lo = E
        step = f / (1 - e * math.cos(E))
        En = E - step
        if not lo < En < hi:
            En = (lo + hi) / 2
        if abs(En - E) <= 4e-16 * max(1.0, abs(E)):
            return En
        E = En
    return E


def kepler_position(M, a, e, t):
    _pos("the central mass and the semi-major axis", M, a)
    if not 0 <= e < 0.99:
        raise ValueError("the eccentricity must satisfy 0 <= e < 0.99 (an elliptical orbit)")
    if t < 0:
        raise ValueError("the time since periapsis must not be negative")
    mu = G * M
    n = math.sqrt(mu / a ** 3)
    P = 2 * math.pi / n
    Mn = math.fmod(n * t, 2 * math.pi)
    E = _kepler_E(Mn, e)
    nu = math.atan2(math.sqrt(1 - e * e) * math.sin(E), math.cos(E) - e) % (2 * math.pi)
    r = a * (1 - e * math.cos(E))
    return _out(mean_anomaly=Mn / DEG, E=E / DEG, nu=nu / DEG, r=r, v=math.sqrt(mu * (2 / r - 1 / a)), P=P)


def ref_kepler_position(M, a, e, t):
    from scipy.integrate import solve_ivp
    mu = G * M
    tau = math.fmod(t * math.sqrt(mu / a ** 3), 2 * math.pi)      # time in units where a = 1 and mu = 1
    y0 = [1 - e, 0.0, 0.0, math.sqrt((1 + e) / (1 - e))]          # at periapsis

    def rhs(_t, y):
        rr = math.hypot(y[0], y[1]) ** 3
        return [y[2], y[3], -y[0] / rr, -y[1] / rr]
    if tau == 0:
        y = y0
    else:
        y = solve_ivp(rhs, (0, tau), y0, method="DOP853", rtol=1e-13, atol=1e-15).y[:, -1]
    nu = math.atan2(y[1], y[0]) % (2 * math.pi)
    return {"nu": nu / DEG, "r": a * math.hypot(y[0], y[1]), "v": math.sqrt(mu / a) * math.hypot(y[2], y[3])}


def _kepler_random(rng):
    M, a, e = _lu(rng, 1e20, 1e31), _lu(rng, 1e6, 1e13), rng.uniform(0.05, 0.9)
    P = 2 * math.pi * math.sqrt(a ** 3 / (G * M))
    return {"M": M, "a": a, "e": e, "t": P * rng.uniform(0.02, 0.98)}


_add("kepler_position_at_time",
     name="Position on an elliptical orbit at a given time (Kepler's equation)", topic="orbits / Kepler's equation",
     purpose="The mean, eccentric and true anomalies, the distance and the speed of a body on an elliptical orbit a time t after periapsis passage.",
     inputs=[_phys("M", "mass", "mass of the central body"), _phys("a", "length", "semi-major axis"), _numin("e", "eccentricity (0 <= e < 0.99)"),
             _phys("t", "time", "time since periapsis passage")],
     outputs=[_o("mean_anomaly", "number", "mean anomaly (degrees)"), _o("E", "number", "eccentric anomaly (degrees)"),
              _o("nu", "number", "true anomaly (degrees, 0 to 360)"), _o("r", "length", "distance from the central body's centre"),
              _o("v", "speed", "orbital speed"), _o("P", "time", "orbital period")],
     primary="nu", conditions=["0 <= e < 0.99", "t >= 0 (reduced modulo the period)", "two-body motion"],
     method="Kepler's equation M = E - e sin E solved by safeguarded Newton iteration; tan(nu/2) = sqrt((1+e)/(1-e)) tan(E/2); r = a(1 - e cos E)",
     latex=r"M = E - e\sin E,\quad \tan\frac{\nu}{2} = \sqrt{\frac{1+e}{1-e}}\tan\frac{E}{2},\quad r = a(1 - e\cos E)",
     run=kepler_position,
     parse=lambda q: _pairs(q, r"kepler'?s equation|position on (?:an )?(?:elliptical )?orbit", {"m": "M", "a": "a", "e": "e", "t": "t"},
                            phys={"M", "a", "t"}),
     reference=ref_kepler_position,
     wrong=_safe(lambda M, a, e, t: (lambda o: {**o, "nu": o["E"]})(kepler_position(M, a, e, t))),
     random_inputs=_kepler_random,
     templates=["Kepler's equation: M = 1 Msun, a = 1 au, e = 0.5, t = 100 day", "Position on orbit: M = 1 Mearth, a = 26600 km, e = 0.74, t = 3 h"],
     examples=[{"inputs": {"M": 1 / G, "a": 1.0, "e": 0.0, "t": math.pi / 2}, "outputs": {"nu": 90.0, "r": 1.0, "v": 1.0}},
               {"inputs": {"M": 1 / G, "a": 1.0, "e": 0.5, "t": math.pi}, "outputs": {"nu": 180.0, "E": 180.0, "r": 1.5}}],
     questions=[{"q": "Kepler's equation: M = 1.4982e10 kg, a = 1 m, e = 0.5, t = 3.14159265358979 s", "outputs": {"r": 1.5}},
                {"q": "Kepler's equation: M = 1 Msun, a = 1 au, e = 1.2, t = 100 day", "outcome": "INVALID_INPUT"},
                {"q": "Kepler's equation: M = 1 Msun, a = 1 au, e = 0.5", "outcome": "NEEDS_INPUT"}])


def time_of_flight(M, a, e, nu1, nu2):
    _pos("the central mass and the semi-major axis", M, a)
    if not 0 <= e < 0.99:
        raise ValueError("the eccentricity must satisfy 0 <= e < 0.99 (an elliptical orbit)")
    n1, n2 = nu1 % (2 * math.pi), nu2 % (2 * math.pi)
    if abs(n1 - n2) < 1e-12:
        raise ValueError("the two true anomalies must differ")
    mu = G * M
    n = math.sqrt(mu / a ** 3)

    def mean(nu):
        E = math.atan2(math.sqrt(1 - e * e) * math.sin(nu), e + math.cos(nu)) % (2 * math.pi)
        return E - e * math.sin(E)
    dM = mean(n2) - mean(n1)
    if n2 < n1:
        dM += 2 * math.pi
    elif dM < 0:
        dM += 2 * math.pi
    return _out(dt=dM / n, dt_days=dM / n / 86400, P=2 * math.pi / n)


def ref_time_of_flight(M, a, e, nu1, nu2):
    n1, n2 = nu1 % (2 * math.pi), nu2 % (2 * math.pi)
    if n2 < n1:
        n2 += 2 * math.pi
    t = _ref_flight_time(G * M, a * (1 - e * e), e, n1, n2)
    return {"dt": t, "dt_days": t / 86400}


def _tof_random(rng):
    while True:
        x = {"M": _lu(rng, 1e20, 1e31), "a": _lu(rng, 1e6, 1e13), "e": rng.uniform(0.05, 0.9),
             "nu1": rng.uniform(0, 359) * DEG, "nu2": rng.uniform(0, 359) * DEG}
        if abs(x["nu1"] - x["nu2"]) > 2 * DEG:
            return x


_add("time_of_flight",
     name="Time of flight between two points of an elliptical orbit", topic="orbits / Kepler's equation",
     purpose="The time a body takes to move forward along an elliptical orbit from true anomaly nu1 to true anomaly nu2.",
     inputs=[_phys("M", "mass", "mass of the central body"), _phys("a", "length", "semi-major axis"), _numin("e", "eccentricity (0 <= e < 0.99)"),
             _phys("nu1", "angle", "starting true anomaly"), _phys("nu2", "angle", "final true anomaly")],
     outputs=[_o("dt", "time", "time of flight"), _o("dt_days", "number", "time of flight in days"), _o("P", "time", "orbital period")],
     primary="dt", conditions=["0 <= e < 0.99", "nu1 != nu2; motion in the forward (prograde) direction"],
     method="eccentric anomalies from the true anomalies, mean anomalies M = E - e sin E, dt = (M2 - M1)/n",
     latex=r"\Delta t = \frac{(E_2 - e\sin E_2) - (E_1 - e\sin E_1)}{n},\quad n = \sqrt{\mu/a^3}",
     run=time_of_flight,
     parse=lambda q: _pairs(q, r"time of flight", {"m": "M", "a": "a", "e": "e", "nu1": "nu1", "nu2": "nu2"}, phys={"M", "a", "nu1", "nu2"}),
     reference=ref_time_of_flight,
     wrong=_safe(lambda M, a, e, nu1, nu2: (lambda dM, n: _out(dt=dM / n, dt_days=dM / n / 86400, P=2 * math.pi / n))(
         (nu2 - nu1) % (2 * math.pi), math.sqrt(G * M / a ** 3))),
     random_inputs=_tof_random,
     templates=["Time of flight: M = 1 Msun, a = 1.26 au, e = 0.2, nu1 = 0 deg, nu2 = 90 deg"],
     examples=[{"inputs": {"M": 1 / G, "a": 1.0, "e": 0.0, "nu1": 0.0, "nu2": math.pi}, "outputs": {"dt": math.pi}},
               {"inputs": {"M": 1 / G, "a": 1.0, "e": 0.5, "nu1": 0.0, "nu2": math.pi}, "outputs": {"dt": math.pi}},
               {"inputs": {"M": 1 / G, "a": 1.0, "e": 0.5, "nu1": math.pi, "nu2": 0.0}, "outputs": {"dt": math.pi}}],
     questions=[{"q": "Time of flight: M = 1.4982e10 kg, a = 1 m, e = 0.5, nu1 = 0 deg, nu2 = 180 deg",
                 "outputs": {"dt": math.pi * math.sqrt(1 / (G * 1.4982e10))}},
                {"q": "Time of flight: M = 1 Msun, a = 1 au, e = 0.5, nu1 = 30 deg, nu2 = 30 deg", "outcome": "INVALID_INPUT"},
                {"q": "Time of flight: M = 1 Msun, a = 1 au, e = 0.5, nu1 = 30 deg", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ exoplanets
def rv_semi_amplitude(M_star, m_p, P, e=0.0, i=math.pi / 2):
    _pos("the masses and the period", M_star, m_p, P)
    if not 0 <= e < 0.99:
        raise ValueError("the eccentricity must satisfy 0 <= e < 0.99")
    if not 0 < i <= math.pi:
        raise ValueError("the inclination must lie between 0 (excluded) and 180 degrees")
    Mt = M_star + m_p
    K = (2 * math.pi * G / P) ** (1 / 3) * m_p * math.sin(i) / (Mt ** (2 / 3) * math.sqrt(1 - e * e))
    a = (G * Mt * P * P / (4 * math.pi ** 2)) ** (1 / 3)
    return _out(K=K, a=a, a_au=a / AU)


def ref_rv_semi_amplitude(M_star, m_p, P, e=0.0, i=math.pi / 2):
    from scipy.integrate import solve_ivp
    mu = G * (M_star + m_p)
    a = (P / _ref_kepler_period(1.0, mu)) ** (2 / 3)                # the period integral scales as a^(3/2)
    if e < 1e-3:
        k_rel = 1.0                                                  # circular: constant speed sqrt(mu/a)
    else:
        # relative orbit in units a = 1, mu = 1 from periapsis to apoapsis; line of sight along y, tilted by i
        def rhs(_t, y):
            rr = math.hypot(y[0], y[1]) ** 3
            return [y[2], y[3], -y[0] / rr, -y[1] / rr]
        apo = lambda _t, y: y[0] * y[2] + y[1] * y[3]                # noqa: E731  radial velocity
        apo.terminal, apo.direction = True, -1
        y0 = [1 - e, 0.0, 0.0, math.sqrt((1 + e) / (1 - e))]
        sol = solve_ivp(rhs, (0, 20.0), y0, method="DOP853", rtol=1e-13, atol=1e-15, events=apo)
        k_rel = (y0[3] - sol.y_events[0][0][3]) / 2                  # half the swing of the line-of-sight velocity
    return {"K": m_p / (M_star + m_p) * k_rel * math.sqrt(mu / a) * math.sin(i), "a": a}


_add("radial_velocity_semi_amplitude",
     name="Radial-velocity semi-amplitude of a star", topic="exoplanets / radial velocity",
     purpose="The semi-amplitude K of the star's line-of-sight velocity wobble caused by an orbiting planet (or companion) of mass m.",
     inputs=[_phys("M_star", "mass", "mass of the star (Msun, kg)"), _phys("m_p", "mass", "mass of the planet (Mjup, Mearth, kg)"),
             _phys("P", "time", "orbital period"), _numin("e", "eccentricity (default 0)", required=False),
             _phys("i", "angle", "orbital inclination (default 90 degrees, edge-on)", required=False)],
     outputs=[_o("K", "speed", "radial-velocity semi-amplitude of the star"), _o("a", "length", "relative semi-major axis"),
              _o("a_au", "number", "relative semi-major axis in au")],
     primary="K", conditions=["0 <= e < 0.99", "0 < i <= 180 degrees", "two-body Keplerian orbit"],
     method="K = (2 pi G / P)^(1/3) m sin i / ((M + m)^(2/3) sqrt(1 - e^2))",
     latex=r"K = \left(\frac{2\pi G}{P}\right)^{1/3}\frac{m\sin i}{(M_\star + m)^{2/3}\sqrt{1 - e^2}}",
     run=rv_semi_amplitude,
     parse=lambda q: _pairs(q, r"radial[- ]velocity semi[- ]amplitude", {"m_star": "M_star", "mstar": "M_star", "m_p": "m_p", "mp": "m_p",
                                                                        "p": "P", "e": "e", "i": "i"}, phys={"M_star", "m_p", "P", "i"}),
     reference=ref_rv_semi_amplitude,
     wrong=_safe(lambda M_star, m_p, P, e=0.0, i=math.pi / 2: (lambda o: {**o, "K": _r(o["K"] / (M_star + m_p) ** (1 / 3))})(rv_semi_amplitude(M_star, m_p, P, e, i))),
     random_inputs=lambda rng: (lambda Ms: {"M_star": Ms, "m_p": Ms * _lu(rng, 1e-7, 1e-1), "P": _lu(rng, 0.5 * 86400, 30 * YR),
                                            "e": rng.choice([0.0, rng.uniform(0.01, 0.9)]), "i": rng.uniform(10, 170) * DEG})(_lu(rng, 0.1, 10) * MSUN),
     templates=["Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup, P = 11.86 yr",
                "Radial velocity semi-amplitude: M_star = 1.1 Msun, m_p = 0.47 Mjup, P = 4.23 d, e = 0.01, i = 86 deg"],
     examples=[{"inputs": {"M_star": 1 / G - 1e-3 / G, "m_p": 1e-3 / G, "P": 2 * math.pi}, "outputs": {"K": 1e-3, "a": 1.0}},
               {"inputs": {"M_star": 0.5 / G, "m_p": 0.5 / G, "P": 2 * math.pi, "e": 0.6, "i": math.pi / 6}, "outputs": {"K": 0.5 * 0.5 / 0.8}}],
     questions=[{"q": "Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup, P = 11.86 yr",
                 "outputs": {"K": (2 * math.pi * G / (11.86 * YR)) ** (1 / 3) * MJUP / (MSUN + MJUP) ** (2 / 3)}},
                {"q": "Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup, P = 11.86 yr, e = 1.5", "outcome": "INVALID_INPUT"},
                {"q": "Radial velocity semi-amplitude: M_star = 1 Msun, m_p = 1 Mjup", "outcome": "NEEDS_INPUT"}])


def _overlap(k, d):
    """Area of overlap of a unit circle and a circle of radius k (< 1) whose centres are d apart."""
    if d >= 1 + k:
        return 0.0
    if d <= 1 - k:
        return math.pi * k * k
    a1 = math.acos(max(-1.0, min(1.0, (d * d + k * k - 1) / (2 * d * k))))
    a2 = math.acos(max(-1.0, min(1.0, (d * d + 1 - k * k) / (2 * d))))
    return k * k * a1 + a2 - 0.5 * math.sqrt(max(0.0, (-d + k + 1) * (d + k - 1) * (d - k + 1) * (d + k + 1)))


def transit_depth(Rp, Rs, b=0.0):
    _pos("the two radii", Rp, Rs)
    k = Rp / Rs
    if k >= 1:
        raise ValueError("the planet must be smaller than the star")
    if not 0 <= b < 1 + k:
        raise ValueError("no transit: the impact parameter must satisfy 0 <= b < 1 + Rp/Rs")
    depth = _overlap(k, b) / math.pi
    return _out(depth=depth, depth_ppm=depth * 1e6, k=k, grazing=b > 1 - k)


def ref_transit_depth(Rp, Rs, b=0.0):
    from scipy.integrate import quad
    k = Rp / Rs

    def chord(x):                              # length of the planet's chord at x that lies on the stellar disk
        hp = math.sqrt(max(0.0, k * k - (x - b) ** 2))
        hs = math.sqrt(max(0.0, 1 - x * x))
        return 2 * min(hp, hs)
    lo, hi = max(b - k, -1.0), min(b + k, 1.0)
    pts = [p for p in ([(1 + b * b - k * k) / (2 * b)] if b > 0 else []) if lo < p < hi]
    A, _ = quad(chord, lo, hi, points=pts or None, epsabs=0, epsrel=1e-13, limit=400)
    return {"depth": A / math.pi}


_add("transit_depth",
     name="Transit depth", topic="exoplanets / transits",
     purpose="The fractional dimming of a uniformly bright star when a planet of radius Rp crosses it at impact parameter b (b = 0 central; grazing transits included).",
     inputs=[_phys("Rp", "length", "planet radius (Rjup, Rearth, km)"), _phys("Rs", "length", "stellar radius (Rsun, km)"),
             _numin("b", "impact parameter in stellar radii (default 0)", required=False)],
     outputs=[_o("depth", "number", "fractional flux drop"), _o("depth_ppm", "number", "depth in parts per million"),
              _o("k", "number", "radius ratio Rp/Rs"), _o("grazing", "boolean", "whether the planet's disk crosses the stellar limb")],
     primary="depth", conditions=["Rp < Rs", "0 <= b < 1 + Rp/Rs", "uniform stellar disk (no limb darkening)"],
     method="overlapping area of the two disks divided by the stellar disk area; (Rp/Rs)^2 for a full transit",
     latex=r"\delta = \frac{A_\mathrm{overlap}}{\pi R_\star^2} = \left(\frac{R_p}{R_\star}\right)^2\ \text{(full transit)}",
     run=transit_depth,
     parse=lambda q: _pairs(q, r"transit depth", {"rp": "Rp", "rs": "Rs", "r_star": "Rs", "b": "b"}, phys={"Rp", "Rs"}),
     reference=ref_transit_depth,
     wrong=_safe(lambda Rp, Rs, b=0.0: _out(depth=Rp / Rs, depth_ppm=Rp / Rs * 1e6, k=Rp / Rs, grazing=b > 1 - Rp / Rs)),
     random_inputs=lambda rng: (lambda k: {"Rp": k * 7e8, "Rs": 7e8, "b": rng.choice([0.0, rng.uniform(0, 1 + k - 1e-6), rng.uniform(1 - k, 1 + k - 1e-6)])})(rng.uniform(0.005, 0.5)),
     templates=["Transit depth: Rp = 1 Rjup, Rs = 1 Rsun", "Transit depth: Rp = 1 Rearth, Rs = 1 Rsun, b = 0.5"],
     examples=[{"inputs": {"Rp": 1.0, "Rs": 10.0}, "outputs": {"depth": 0.01}},
               {"inputs": {"Rp": 1.0, "Rs": 10.0, "b": 0.5}, "outputs": {"depth": 0.01, "grazing": False}},
               {"inputs": {"Rp": 1.0, "Rs": 10.0, "b": 1.0}, "outputs": {"grazing": True}}],
     questions=[{"q": "Transit depth: Rp = 1 Rjup, Rs = 1 Rsun", "outputs": {"depth": (RJUP / RSUN) ** 2}},
                {"q": "Transit depth: Rp = 0.1 m, Rs = 1 m, b = 0.3", "outputs": {"depth_ppm": 10000.0}},
                {"q": "Transit depth: Rp = 1 Rjup, Rs = 1 Rsun, b = 2", "outcome": "INVALID_INPUT"},
                {"q": "Transit depth: Rp = 1 Rjup", "outcome": "NEEDS_INPUT"}])


def transit_duration(P, a, Rs, Rp, i):
    _pos("the period, the orbit radius and the two radii", P, a, Rs, Rp)
    if Rp >= Rs:
        raise ValueError("the planet must be smaller than the star")
    if a <= Rs + Rp:
        raise ValueError("the orbit radius must exceed the sum of the two radii")
    if not 0 < i <= math.pi / 2:
        raise ValueError("the inclination must lie between 0 (excluded) and 90 degrees")
    b = a * math.cos(i) / Rs
    b = 0.0 if b < 1e-12 else b                 # edge-on: exactly central
    k = Rp / Rs
    if b >= 1 + k:
        raise ValueError("the planet does not transit (impact parameter b >= 1 + Rp/Rs)")
    out = {"T14": P / math.pi * math.asin(math.sqrt((Rs + Rp) ** 2 - (b * Rs) ** 2) / (a * math.sin(i)))}
    if b < 1 - k:
        out["T23"] = P / math.pi * math.asin(math.sqrt((Rs - Rp) ** 2 - (b * Rs) ** 2) / (a * math.sin(i)))
    return _out(**out, T14_h=out["T14"] / 3600, b=b)


def ref_transit_duration(P, a, Rs, Rp, i):
    from scipy.optimize import brentq
    # the planet's projected distance from the star's centre, phase phi from mid-transit, on a circular orbit
    sep = lambda phi: a * math.hypot(math.sin(phi), math.cos(i) * math.cos(phi))     # noqa: E731
    out = {}
    for name, R in (("T14", Rs + Rp), ("T23", Rs - Rp)):
        if sep(0.0) < R:
            phi = brentq(lambda f: sep(f) - R, 0.0, math.pi / 2, xtol=1e-15, rtol=8.9e-16, maxiter=500)
            out[name] = 2 * phi / (2 * math.pi) * P
    return out


def _duration_random(rng):
    Rs = _lu(rng, 1e8, 1e10)
    k = rng.uniform(0.01, 0.2)
    a = Rs * _lu(rng, 3, 200)
    b = rng.uniform(0, 0.95) * (1 - k) if rng.random() < 0.8 else rng.uniform(1 - k + 1e-3, 1 + k - 1e-3)
    return {"P": _lu(rng, 0.5 * 86400, 1000 * 86400), "a": a, "Rs": Rs, "Rp": k * Rs, "i": math.acos(b * Rs / a)}


_add("transit_duration",
     name="Transit duration", topic="exoplanets / transits",
     purpose="The total (first to fourth contact) and full (second to third contact) durations of a transit of a planet on a circular orbit, and its impact parameter.",
     inputs=[_phys("P", "time", "orbital period"), _phys("a", "length", "orbit radius (au, m)"), _phys("Rs", "length", "stellar radius"),
             _phys("Rp", "length", "planet radius"), _phys("i", "angle", "orbital inclination (90 degrees = edge-on)")],
     outputs=[_o("T14", "time", "total transit duration"), _o("T14_h", "number", "total duration in hours"),
              _o("T23", "time", "full-transit duration (absent for a grazing transit)"), _o("b", "number", "impact parameter a cos i / Rs")],
     primary="T14", conditions=["circular orbit", "Rp < Rs", "a > Rs + Rp", "0 < i <= 90 degrees", "b < 1 + Rp/Rs (the planet transits)"],
     method="T14 = (P/pi) asin( sqrt((Rs + Rp)^2 - (b Rs)^2) / (a sin i) ), T23 with Rs - Rp",
     latex=r"T_{14} = \frac{P}{\pi}\arcsin\left(\frac{\sqrt{(R_\star + R_p)^2 - (bR_\star)^2}}{a\sin i}\right),\quad b = \frac{a\cos i}{R_\star}",
     run=transit_duration,
     parse=lambda q: _pairs(q, r"transit duration", {"p": "P", "a": "a", "rs": "Rs", "r_star": "Rs", "rp": "Rp", "i": "i"},
                            phys={"P", "a", "Rs", "Rp", "i"}),
     reference=ref_transit_duration,
     wrong=_safe(lambda P, a, Rs, Rp, i: (lambda o: {**o, "T14": _r(2 * o["T14"])})(transit_duration(P, a, Rs, Rp, i))),
     random_inputs=_duration_random,
     templates=["Transit duration: P = 3.52 d, a = 0.047 au, Rs = 1.2 Rsun, Rp = 1.38 Rjup, i = 86.7 deg",
                "Transit duration: P = 365.25 d, a = 1 au, Rs = 1 Rsun, Rp = 1 Rearth, i = 90 deg"],
     examples=[{"inputs": {"P": 2 * math.pi, "a": 2.0, "Rs": 0.9, "Rp": 0.1, "i": math.pi / 2},
                "outputs": {"T14": 2 * math.asin(0.5), "T23": 2 * math.asin(0.4), "b": 0.0}}],
     questions=[{"q": "Transit duration: P = 6.283185307179586 s, a = 2 m, Rs = 0.9 m, Rp = 0.1 m, i = 90 deg", "outputs": {"T14": math.pi / 3}},
                {"q": "Transit duration: P = 365.25 d, a = 1 au, Rs = 1 Rsun, Rp = 1 Rearth, i = 80 deg", "outcome": "INVALID_INPUT"},
                {"q": "Transit duration: P = 365.25 d, a = 1 au, Rs = 1 Rsun, Rp = 1 Rearth", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ cosmology (flat Lambda-CDM)
_H0U = ["km/s/Mpc", "km s^-1 Mpc^-1"]
_COSMO_NAMES = {"z": "z", "h0": "H0", "om": "Om", "om0": "Om", "omega_m": "Om", "omegam": "Om"}


def _cosmo_check(H0, Om, z=None):
    if not 1 <= H0 <= 1000:
        raise ValueError("H0 must lie between 1 and 1000 km/s/Mpc")
    if not 0.01 <= Om <= 1:
        raise ValueError("Omega_m must lie between 0.01 and 1 (flat universe: Omega_Lambda = 1 - Omega_m)")
    if z is not None and not 0 <= z <= 1100:
        raise ValueError("the redshift must lie between 0 and 1100")
    return H0 * 1000.0 / MPC                       # H0 in 1/s


def _inv_E(Om):
    import mpmath
    return lambda x: 1 / mpmath.sqrt(Om * (1 + x) ** 3 + (1 - Om))


def _comoving(z, Om):
    """Dimensionless comoving distance integral from 0 to z by mpmath tanh-sinh quadrature."""
    import mpmath
    return float(mpmath.quad(_inv_E(Om), [0, z])) if z > 0 else 0.0


def cosmo_distances(z, H0, Om):
    h = _cosmo_check(H0, Om, z)
    if z <= 0:
        raise ValueError("the redshift must be positive")
    DC = C / h * _comoving(z, Om)
    DL = (1 + z) * DC
    return _out(D_C=DC, D_C_Mpc=DC / MPC, D_L_Mpc=DL / MPC, D_A_Mpc=DC / (1 + z) / MPC, mu=5 * math.log10(DL / (10 * PC)), D_L=DL)


def _astropy_cosmo(H0, Om):
    from astropy.cosmology import FlatLambdaCDM
    return FlatLambdaCDM(H0=H0, Om0=Om, Tcmb0=0)


def ref_cosmo_distances(z, H0, Om):
    import astropy.units as u
    c = _astropy_cosmo(H0, Om)
    return {"D_C_Mpc": c.comoving_distance(z).to(u.Mpc).value, "D_L_Mpc": c.luminosity_distance(z).to(u.Mpc).value,
            "D_A_Mpc": c.angular_diameter_distance(z).to(u.Mpc).value, "mu": c.distmod(z).value}


def _cosmo_random(rng):
    return {"z": rng.choice([_lu(rng, 0.01, 1.0), _lu(rng, 1.0, 20.0)]), "H0": rng.uniform(50, 100),
            "Om": rng.choice([rng.uniform(0.05, 0.95), 1.0]) if rng.random() < 0.1 else rng.uniform(0.05, 0.95)}


_COSMO_IN = [_numin("z", "redshift"), _numin("H0", "Hubble constant in km/s/Mpc"), _numin("Om", "matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m)")]
_add("cosmological_distances",
     name="Cosmological distances at a redshift", topic="cosmology / distances",
     purpose="The comoving, luminosity and angular-diameter distances and the distance modulus of an object at redshift z in a flat Lambda-CDM universe.",
     inputs=_COSMO_IN,
     outputs=[_o("D_C", "length", "comoving distance"), _o("D_C_Mpc", "number", "comoving distance in Mpc"),
              _o("D_L_Mpc", "number", "luminosity distance in Mpc"), _o("D_A_Mpc", "number", "angular-diameter distance in Mpc"),
              _o("mu", "number", "distance modulus"), _o("D_L", "length", "luminosity distance")],
     primary="D_L_Mpc", conditions=["0 < z <= 1100", "1 <= H0 <= 1000 km/s/Mpc", "0.01 <= Omega_m <= 1", "flat, no radiation"],
     method="D_C = (c/H0) integral_0^z dz'/E(z'), E = sqrt(Om (1+z)^3 + 1 - Om) (mpmath quadrature); D_L = (1+z) D_C; D_A = D_C/(1+z)",
     latex=r"D_C = \frac{c}{H_0}\int_0^z \frac{dz'}{\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}},\quad D_L = (1+z)D_C,\quad D_A = \frac{D_C}{1+z}",
     run=cosmo_distances,
     parse=lambda q: _pairs(q, r"cosmological distances?", _COSMO_NAMES, num_units={"H0": _H0U}),
     reference=ref_cosmo_distances,
     wrong=_safe(lambda z, H0, Om: (lambda o: {**o, "D_L_Mpc": _r(o["D_C_Mpc"]), "D_A_Mpc": _r(o["D_C_Mpc"])})(cosmo_distances(z, H0, Om))),
     random_inputs=_cosmo_random,
     templates=["Cosmological distances: z = 1, H0 = 70 km/s/Mpc, Om = 0.3", "Cosmological distance: z = 0.5, H0 = 67.7, Om = 0.31"],
     examples=[{"inputs": {"z": 3.0, "H0": 70.0, "Om": 1.0}, "outputs": {"D_C_Mpc": C / 1000 / 70 * 2 * (1 - 0.5)}},
               {"inputs": {"z": 8.0, "H0": 100.0, "Om": 1.0}, "outputs": {"D_C_Mpc": C / 1000 / 100 * 2 * (1 - 1 / 3), "D_A_Mpc": C / 1000 / 100 * 4 / 3 / 9}}],
     questions=[{"q": "Cosmological distances: z = 3, H0 = 70, Om = 1", "outputs": {"D_L_Mpc": C / 1000 / 70 * 4}},
                {"q": "Cosmological distances: z = 1, H0 = 70 km/s/Mpc, Om = 1.5", "outcome": "INVALID_INPUT"},
                {"q": "Cosmological distances: z = 1, H0 = 70", "outcome": "NEEDS_INPUT"}])


def angular_size_at_z(z, H0, Om, D):
    h = _cosmo_check(H0, Om, z)
    if z <= 0:
        raise ValueError("the redshift must be positive")
    _pos("the size", D)
    DA = C / h * _comoving(z, Om) / (1 + z)
    return _out(theta_arcsec=D / DA / ARCSEC, scale_kpc_per_arcsec=DA * ARCSEC / (1e3 * PC), D_A_Mpc=DA / MPC)


def ref_angular_size_at_z(z, H0, Om, D):
    import astropy.units as u
    scale = _astropy_cosmo(H0, Om).kpc_proper_per_arcmin(z).to(u.kpc / u.arcsec).value
    return {"scale_kpc_per_arcsec": scale, "theta_arcsec": D / (scale * 1e3 * PC)}


_add("angular_size_at_redshift",
     name="Angular size of an object at a redshift", topic="cosmology / angular scale",
     purpose="The angle subtended by an object of proper size D at redshift z, and the proper length per arcsecond, in a flat Lambda-CDM universe.",
     inputs=_COSMO_IN + [_phys("D", "length", "proper size of the object (kpc, Mpc, m)")],
     outputs=[_o("theta_arcsec", "number", "angular size in arcseconds"), _o("scale_kpc_per_arcsec", "number", "proper kiloparsecs per arcsecond"),
              _o("D_A_Mpc", "number", "angular-diameter distance in Mpc")],
     primary="theta_arcsec", conditions=["0 < z <= 1100", "D > 0", "small angles", "flat Lambda-CDM, no radiation"],
     method="theta = D / D_A with D_A = D_C / (1 + z)", latex=r"\theta = \frac{D}{D_A},\quad D_A = \frac{1}{1+z}\frac{c}{H_0}\int_0^z\frac{dz'}{E(z')}",
     run=angular_size_at_z,
     parse=lambda q: _pairs(q, r"angular size at (?:a )?redshift", {**_COSMO_NAMES, "d": "D", "size": "D"}, phys={"D"}, num_units={"H0": _H0U}),
     reference=ref_angular_size_at_z,
     wrong=_safe(lambda z, H0, Om, D: (lambda DC: _out(theta_arcsec=D / DC / ARCSEC, scale_kpc_per_arcsec=DC * ARCSEC / (1e3 * PC), D_A_Mpc=DC / MPC))(
         C / _cosmo_check(H0, Om, z) * _comoving(z, Om))),
     random_inputs=lambda rng: {**_cosmo_random(rng), "D": _lu(rng, 0.1, 1e4) * 1e3 * PC},
     templates=["Angular size at redshift: z = 1, H0 = 70 km/s/Mpc, Om = 0.3, D = 10 kpc"],
     examples=[{"inputs": {"z": 3.0, "H0": 70.0, "Om": 1.0, "D": C / 1000 / 70 * 0.25 * MPC * ARCSEC}, "outputs": {"theta_arcsec": 1.0}}],
     questions=[{"q": "Angular size at redshift: z = 3, H0 = 70, Om = 1, D = 1 Mpc", "outputs": {"theta_arcsec": 1 / (C / 1000 / 70 * 0.25) / ARCSEC}},
                {"q": "Angular size at redshift: z = 1, H0 = 70, Om = 0.3, D = -5 kpc", "outcome": "INVALID_INPUT"},
                {"q": "Angular size at redshift: z = 1, H0 = 70, Om = 0.3", "outcome": "NEEDS_INPUT"}])


def lookback_time(z, H0, Om):
    import mpmath
    h = _cosmo_check(H0, Om, z)
    if z <= 0:
        raise ValueError("the redshift must be positive")
    f = _inv_E(Om)
    t = float(mpmath.quad(lambda x: f(x) / (1 + x), [0, z])) / h
    return _out(t_L=t, t_L_Gyr=t / GYR)


_add("lookback_time",
     name="Lookback time to a redshift", topic="cosmology / time",
     purpose="How long ago the light we receive from redshift z was emitted, in a flat Lambda-CDM universe.",
     inputs=_COSMO_IN,
     outputs=[_o("t_L", "time", "lookback time"), _o("t_L_Gyr", "number", "lookback time in Gyr")],
     primary="t_L_Gyr", conditions=["0 < z <= 1100", "flat Lambda-CDM, no radiation"],
     method="t_L = (1/H0) integral_0^z dz' / ((1 + z') E(z')) by mpmath quadrature",
     latex=r"t_L = \frac{1}{H_0}\int_0^z \frac{dz'}{(1+z')\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}}",
     run=lookback_time,
     parse=lambda q: _pairs(q, r"lookback time", _COSMO_NAMES, num_units={"H0": _H0U}),
     reference=lambda z, H0, Om: {"t_L_Gyr": _astropy_cosmo(H0, Om).lookback_time(z).to("Gyr").value},
     wrong=_safe(lambda z, H0, Om: (lambda t: _out(t_L=t, t_L_Gyr=t / GYR))(_comoving(z, Om) / _cosmo_check(H0, Om, z))),
     random_inputs=_cosmo_random,
     templates=["Lookback time: z = 1, H0 = 70 km/s/Mpc, Om = 0.3"],
     examples=[{"inputs": {"z": 3.0, "H0": 70.0, "Om": 1.0}, "outputs": {"t_L": 2 / 3 / (70e3 / MPC) * (1 - 1 / 8)}}],
     questions=[{"q": "Lookback time: z = 3, H0 = 70, Om = 1", "outputs": {"t_L": 2 / 3 / (70e3 / MPC) * (1 - 1 / 8)}},
                {"q": "Lookback time: z = -1, H0 = 70, Om = 0.3", "outcome": "INVALID_INPUT"},
                {"q": "Lookback time: z = 1, Om = 0.3", "outcome": "NEEDS_INPUT"}])


def age_at_z(z, H0, Om):
    import mpmath
    h = _cosmo_check(H0, Om, z)
    f = _inv_E(Om)
    g = lambda x: f(x) / (1 + x)                     # noqa: E731
    age = float(mpmath.quad(g, [z, mpmath.inf])) / h
    now = float(mpmath.quad(g, [0, mpmath.inf])) / h
    return _out(age=age, age_Gyr=age / GYR, age_now_Gyr=now / GYR)


_add("age_at_redshift",
     name="Age of the universe at a redshift", topic="cosmology / time",
     purpose="The age of a flat Lambda-CDM universe when the light from redshift z was emitted, and its present age.",
     inputs=_COSMO_IN,
     outputs=[_o("age", "time", "age at redshift z"), _o("age_Gyr", "number", "age at redshift z in Gyr"),
              _o("age_now_Gyr", "number", "present age in Gyr")],
     primary="age_Gyr", conditions=["0 <= z <= 1100", "flat Lambda-CDM, no radiation (not accurate in the radiation era)"],
     method="t(z) = (1/H0) integral_z^inf dz' / ((1 + z') E(z')) by mpmath quadrature",
     latex=r"t(z) = \frac{1}{H_0}\int_z^\infty \frac{dz'}{(1+z')\sqrt{\Omega_m(1+z')^3 + \Omega_\Lambda}}",
     run=age_at_z,
     parse=lambda q: _pairs(q, r"age (?:of the universe )?at (?:a )?redshift", _COSMO_NAMES, num_units={"H0": _H0U}),
     reference=lambda z, H0, Om: (lambda c: {"age_Gyr": c.age(z).to("Gyr").value, "age_now_Gyr": c.age(0).to("Gyr").value})(_astropy_cosmo(H0, Om)),
     wrong=_safe(lambda z, H0, Om: (lambda o: {**o, "age_Gyr": _r(o["age_now_Gyr"] - o["age_Gyr"])})(age_at_z(z, H0, Om))),
     random_inputs=_cosmo_random,
     templates=["Age at redshift: z = 2, H0 = 70 km/s/Mpc, Om = 0.3", "Age of the universe at redshift: z = 0, H0 = 67.7, Om = 0.31"],
     examples=[{"inputs": {"z": 3.0, "H0": 70.0, "Om": 1.0}, "outputs": {"age": 2 / 3 / (70e3 / MPC) / 8, "age_now_Gyr": 2 / 3 / (70e3 / MPC) / GYR}}],
     questions=[{"q": "Age at redshift: z = 3, H0 = 70, Om = 1", "outputs": {"age": 2 / 3 / (70e3 / MPC) / 8}},
                {"q": "Age at redshift: z = 3, H0 = 70, Om = 0", "outcome": "INVALID_INPUT"},
                {"q": "Age at redshift: z = 3", "outcome": "NEEDS_INPUT"}])


def redshift_at_lookback(t, H0, Om):
    h = _cosmo_check(H0, Om)
    _pos("the lookback time", t)
    if Om == 1:
        t0 = 2 / (3 * h)
        if t >= t0:
            raise ValueError("the lookback time must be shorter than the age of the universe")
        z = ((t0 - t) / t0) ** (-2 / 3) - 1
    else:
        OL = 1 - Om
        q = math.sqrt(OL / Om)
        k = 1.5 * h * math.sqrt(OL)
        t0 = math.asinh(q) / k
        if t >= t0:
            raise ValueError("the lookback time must be shorter than the age of the universe")
        z = (q / math.sinh(k * (t0 - t))) ** (2 / 3) - 1
    return _out(z=z, age_now_Gyr=t0 / GYR)


def ref_redshift_at_lookback(t, H0, Om):
    from scipy.integrate import quad
    from scipy.optimize import brentq
    h = H0 * 1000.0 / MPC
    f = lambda x: 1 / ((1 + x) * math.sqrt(Om * (1 + x) ** 3 + 1 - Om))        # noqa: E731
    tl = lambda z: quad(f, 0, z, epsabs=0, epsrel=2e-14, limit=200)[0] / h      # noqa: E731
    return {"z": brentq(lambda z: tl(z) - t, 0.0, 1e5, xtol=1e-14, rtol=8.9e-16, maxiter=500)}


def _tl_random(rng):
    x = _cosmo_random(rng)
    return {"t": lookback_time(x["z"], x["H0"], x["Om"])["t_L"], "H0": x["H0"], "Om": x["Om"]}


_add("redshift_at_lookback_time",
     name="Redshift at a given lookback time", topic="cosmology / time",
     purpose="The redshift of light emitted a lookback time t ago in a flat Lambda-CDM universe (exact inverse of the age-redshift relation).",
     inputs=[_phys("t", "time", "lookback time (Gyr, Myr, yr)"), _numin("H0", "Hubble constant in km/s/Mpc"),
             _numin("Om", "matter density parameter Omega_m (flat: Omega_Lambda = 1 - Omega_m)")],
     outputs=[_o("z", "number", "redshift"), _o("age_now_Gyr", "number", "present age of the universe in Gyr")],
     primary="z", conditions=["0 < t < present age", "flat Lambda-CDM, no radiation"],
     method="t(z) = 2/(3 H0 sqrt(OL)) asinh( sqrt(OL/Om) (1+z)^(-3/2) ), inverted in closed form (Om = 1: t = 2/(3 H0) (1+z)^(-3/2))",
     latex=r"1 + z = \left[\frac{\sqrt{\Omega_\Lambda/\Omega_m}}{\sinh\left(\tfrac{3}{2}H_0\sqrt{\Omega_\Lambda}\,(t_0 - t_L)\right)}\right]^{2/3}",
     run=redshift_at_lookback,
     parse=lambda q: _pairs(q, r"redshift at (?:a )?(?:given )?lookback time", {"t": "t", "t_l": "t", "h0": "H0", "om": "Om", "om0": "Om",
                                                                               "omega_m": "Om"}, phys={"t"}, num_units={"H0": _H0U}),
     reference=ref_redshift_at_lookback,
     wrong=_safe(lambda t, H0, Om: _out(z=t * H0 * 1000.0 / MPC, age_now_Gyr=0.0)),
     random_inputs=_tl_random,
     templates=["Redshift at lookback time: t = 8 Gyr, H0 = 70 km/s/Mpc, Om = 0.3"],
     examples=[{"inputs": {"t": 2 / 3 / (70e3 / MPC) * (1 - 1 / 8), "H0": 70.0, "Om": 1.0}, "outputs": {"z": 3.0}}],
     questions=[{"q": "Redshift at lookback time: t = 8 Gyr, H0 = 70, Om = 0.3",
                 "outputs": {"z": (math.sqrt(0.7 / 0.3) / math.sinh(math.asinh(math.sqrt(0.7 / 0.3)) - 1.5 * 70e3 / MPC * math.sqrt(0.7) * 8 * GYR)) ** (2 / 3) - 1}},
                {"q": "Redshift at lookback time: t = 30 Gyr, H0 = 70, Om = 0.3", "outcome": "INVALID_INPUT"},
                {"q": "Redshift at lookback time: t = 8 Gyr, H0 = 70", "outcome": "NEEDS_INPUT"}])


def comoving_volume(z, H0, Om):
    h = _cosmo_check(H0, Om, z)
    if z <= 0:
        raise ValueError("the redshift must be positive")
    DC = C / h * _comoving(z, Om)
    V = 4 / 3 * math.pi * DC ** 3
    return _out(V=V, V_Gpc3=V / (1e9 * PC) ** 3)


_add("comoving_volume",
     name="Comoving volume out to a redshift", topic="cosmology / volumes",
     purpose="The all-sky comoving volume enclosed within redshift z in a flat Lambda-CDM universe.",
     inputs=_COSMO_IN,
     outputs=[_o("V", "volume", "comoving volume"), _o("V_Gpc3", "number", "comoving volume in cubic gigaparsecs")],
     primary="V_Gpc3", conditions=["0 < z <= 1100", "flat Lambda-CDM, no radiation"],
     method="V = (4 pi / 3) D_C^3 (flat space)", latex=r"V_C = \frac{4\pi}{3}D_C^3",
     run=comoving_volume,
     parse=lambda q: _pairs(q, r"comoving volume", _COSMO_NAMES, num_units={"H0": _H0U}),
     reference=lambda z, H0, Om: {"V_Gpc3": _astropy_cosmo(H0, Om).comoving_volume(z).to("Gpc3").value},
     wrong=_safe(lambda z, H0, Om: (lambda o: {**o, "V_Gpc3": _r(o["V_Gpc3"] * 3)})(comoving_volume(z, H0, Om))),
     random_inputs=_cosmo_random,
     templates=["Comoving volume: z = 1, H0 = 70 km/s/Mpc, Om = 0.3"],
     examples=[{"inputs": {"z": 3.0, "H0": 70.0, "Om": 1.0}, "outputs": {"V_Gpc3": 4 / 3 * math.pi * (C / 1000 / 70 / 1000) ** 3}}],
     questions=[{"q": "Comoving volume: z = 3, H0 = 70, Om = 1", "outputs": {"V_Gpc3": 4 / 3 * math.pi * (C / 1000 / 70 / 1000) ** 3}},
                {"q": "Comoving volume: z = 0, H0 = 70, Om = 0.3", "outcome": "INVALID_INPUT"},
                {"q": "Comoving volume: H0 = 70, Om = 0.3", "outcome": "NEEDS_INPUT"}])


# ------------------------------------------------------------------------------------------------ redshift and velocity
def _ref_boost_beta(ratio):
    """Recession speed (v/c) at which a Lorentz boost (matrix exponential of the generator, scipy.linalg.expm) turns a
    photon emitted towards the observer into one of `ratio` times its energy."""
    import numpy as np
    from scipy.linalg import expm
    from scipy.optimize import brentq
    K = np.array([[0.0, 1.0], [1.0, 0.0]])

    def energy_ratio(phi):                       # source frame -> observer frame; the source recedes along +x
        L = expm(-phi * K)
        return float((L @ np.array([1.0, -1.0]))[0])
    phi = brentq(lambda f: math.log(energy_ratio(f)) - math.log(ratio), -15, 15, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    L = expm(phi * K)
    return float(-L[0, 1] / L[0, 0])            # the source's velocity is opposite to the boost of rapidity phi


def _doppler_beta(z):
    s = (1 + z) ** 2
    return (s - 1) / (s + 1)


def recession_velocity(z):
    if not -1 < z <= 1e4:
        raise ValueError("the redshift must lie between -1 (excluded) and 10000")
    b = _doppler_beta(z)
    return _out(v=b * C, beta=b, v_cz=C * z)


_add("recession_velocity_from_redshift",
     name="Radial velocity from a Doppler redshift", topic="cosmology / redshift",
     purpose="The radial velocity of a source from its redshift interpreted as a special-relativistic Doppler shift (negative for a blueshift), with the low-velocity value cz for comparison.",
     inputs=[_numin("z", "redshift (negative for a blueshift)")],
     outputs=[_o("v", "speed", "radial velocity (positive receding)"), _o("beta", "number", "v/c"), _o("v_cz", "speed", "non-relativistic estimate cz")],
     primary="v", conditions=["-1 < z", "a purely kinematic (special-relativistic, radial) Doppler shift; not a cosmological recession"],
     method="v/c = ((1+z)^2 - 1) / ((1+z)^2 + 1)", latex=r"\frac{v}{c} = \frac{(1+z)^2 - 1}{(1+z)^2 + 1}",
     run=recession_velocity,
     parse=lambda q: _pairs(q, r"(?:radial|recession) velocity from (?:a )?(?:doppler )?redshift", {"z": "z"}),
     reference=lambda z: {"beta": _ref_boost_beta(1 / (1 + z))},
     wrong=_safe(lambda z: _out(v=C * z, beta=z, v_cz=C * z)),
     random_inputs=lambda rng: {"z": rng.choice([_lu(rng, 0.01, 10), -_lu(rng, 0.01, 0.9)])},
     templates=["Radial velocity from redshift: z = 0.5", "Recession velocity from Doppler redshift: z = 2"],
     examples=[{"inputs": {"z": 1.0}, "outputs": {"beta": 0.6}},
               {"inputs": {"z": 2.0}, "outputs": {"beta": 0.8}}],
     questions=[{"q": "Radial velocity from redshift: z = 1", "outputs": {"v": 0.6 * C}},
                {"q": "Radial velocity from redshift: z = -1.5", "outcome": "INVALID_INPUT"},
                {"q": "Radial velocity from redshift:", "outcome": "NEEDS_INPUT"}])


def redshift_from_lines(lam_obs, lam_rest):
    _pos("the two wavelengths", lam_obs, lam_rest)
    z = lam_obs / lam_rest - 1
    b = _doppler_beta(z)
    return _out(z=z, v=b * C, beta=b, d_lambda=lam_obs - lam_rest)


_add("redshift_from_wavelengths",
     name="Redshift and radial velocity from a spectral line", topic="cosmology / redshift",
     purpose="The redshift z = lambda_obs/lambda_rest - 1 of a spectral line and the corresponding relativistic Doppler radial velocity.",
     inputs=[_phys("lam_obs", "length", "observed wavelength (nm, Angstrom, m)"), _phys("lam_rest", "length", "rest (laboratory) wavelength")],
     outputs=[_o("z", "number", "redshift"), _o("v", "speed", "radial velocity (positive receding)"), _o("beta", "number", "v/c"),
              _o("d_lambda", "length", "wavelength shift")],
     primary="z", conditions=["both wavelengths > 0", "a purely kinematic radial Doppler shift"],
     method="z = lambda_obs/lambda_rest - 1; v/c = ((1+z)^2 - 1)/((1+z)^2 + 1)",
     latex=r"z = \frac{\lambda_\mathrm{obs}}{\lambda_\mathrm{rest}} - 1,\quad \frac{v}{c} = \frac{(1+z)^2 - 1}{(1+z)^2 + 1}",
     run=redshift_from_lines,
     parse=lambda q: _pairs(q, r"redshift from (?:a )?(?:spectral line|wavelengths?)", {"lam_obs": "lam_obs", "lambda_obs": "lam_obs",
                                                                                     "lam_rest": "lam_rest", "lambda_rest": "lam_rest"},
                            phys={"lam_obs", "lam_rest"}),
     reference=lambda lam_obs, lam_rest: {"beta": _ref_boost_beta(lam_rest / lam_obs)},
     wrong=_safe(lambda lam_obs, lam_rest: (lambda z: _out(z=z, v=C * z, beta=z, d_lambda=lam_obs - lam_rest))(lam_obs / lam_rest - 1)),
     random_inputs=lambda rng: (lambda lr: {"lam_rest": lr, "lam_obs": lr * rng.choice([_lu(rng, 1.01, 11), 1 / _lu(rng, 1.01, 10)])})(_lu(rng, 1e-8, 1e-1)),
     templates=["Redshift from spectral line: lam_obs = 700 nm, lam_rest = 656.28 nm", "Redshift from wavelengths: lambda_obs = 4861 Angstrom, lambda_rest = 4340 Angstrom"],
     examples=[{"inputs": {"lam_obs": 2e-7, "lam_rest": 1e-7}, "outputs": {"z": 1.0, "beta": 0.6}},
               {"inputs": {"lam_obs": 3e-7, "lam_rest": 1e-7}, "outputs": {"z": 2.0, "beta": 0.8}}],
     questions=[{"q": "Redshift from spectral line: lam_obs = 1312.56 nm, lam_rest = 656.28 nm", "outputs": {"z": 1.0, "v": 0.6 * C}},
                {"q": "Redshift from spectral line: lam_obs = 0 nm, lam_rest = 656.28 nm", "outcome": "INVALID_INPUT"},
                {"q": "Redshift from spectral line: lam_obs = 700 nm", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ telescopes and observing
def rayleigh_resolution(lam, D):
    _pos("the wavelength and the aperture", lam, D)
    x = J11 * lam / (math.pi * D)
    if x >= 0.5:
        raise ValueError("the wavelength must be much smaller than the aperture (J11 lambda / (pi D) < 0.5)")
    th = math.asin(x)
    return _out(theta=th, theta_arcsec=th / ARCSEC)


def ref_rayleigh_resolution(lam, D):
    from scipy.optimize import brentq
    from scipy.special import j1
    x0 = brentq(j1, 3.0, 4.5, xtol=1e-15, rtol=8.9e-16, maxiter=500)     # first dark ring of the Airy pattern
    th = math.asin(x0 * lam / (math.pi * D))
    return {"theta": th, "theta_arcsec": th / ARCSEC}


_add("rayleigh_resolution",
     name="Diffraction limit of a telescope (Rayleigh criterion)", topic="telescopes / resolution",
     purpose="The smallest angular separation a circular aperture of diameter D resolves at wavelength lambda: the angle of the first dark ring of the Airy pattern (1.21967 lambda/D for small angles; often rounded to 1.22).",
     inputs=[_phys("lam", "length", "wavelength (nm, m)"), _phys("D", "length", "aperture diameter")],
     outputs=[_o("theta", "angle", "angular resolution"), _o("theta_arcsec", "number", "angular resolution in arcseconds")],
     primary="theta_arcsec", conditions=["lambda << D", "a circular unobstructed aperture, no atmosphere"],
     method="sin(theta) = j_{1,1} lambda / (pi D), j_{1,1} = 3.8317059702 (first zero of J1)",
     latex=r"\sin\theta = \frac{j_{1,1}}{\pi}\frac{\lambda}{D} \approx 1.22\,\frac{\lambda}{D}",
     run=rayleigh_resolution,
     parse=lambda q: _pairs(q, r"rayleigh (?:criterion|resolution)|diffraction limit", {"lam": "lam", "lambda": "lam", "d": "D"}, phys={"lam", "D"}),
     reference=ref_rayleigh_resolution,
     wrong=_safe(lambda lam, D: _out(theta=lam / D, theta_arcsec=lam / D / ARCSEC)),
     random_inputs=lambda rng: (lambda lam: {"lam": lam, "D": lam / _lu(rng, 1e-9, 0.3)})(_lu(rng, 1e-8, 1.0)),
     templates=["Rayleigh criterion: lam = 550 nm, D = 2.4 m", "Diffraction limit: lambda = 21 cm, D = 100 m"],
     examples=[{"inputs": {"lam": math.pi / J11 * 0.25, "D": 1.0}, "outputs": {"theta": math.asin(0.25)}},
               {"inputs": {"lam": 550e-9, "D": 2.4}, "outputs": {"theta": math.asin(J11 * 550e-9 / (math.pi * 2.4))}}],
     questions=[{"q": "Rayleigh criterion: lam = 550 nm, D = 2.4 m", "outputs": {"theta_arcsec": math.asin(J11 * 550e-9 / (math.pi * 2.4)) / ARCSEC}},
                {"q": "Rayleigh criterion: lam = 2 m, D = 1 m", "outcome": "INVALID_INPUT"},
                {"q": "Rayleigh criterion: lam = 550 nm", "outcome": "NEEDS_INPUT"}])


def telescope_magnification(f_obj, f_eye, D, afov=None):
    _pos("the focal lengths and the aperture", f_obj, f_eye, D)
    M = f_obj / f_eye
    out = {"M": M, "exit_pupil": D / M, "exit_pupil_mm": D / M * 1e3}
    if afov is not None:
        if not 0 < afov < math.pi:
            raise ValueError("the apparent field of view must lie between 0 and 180 degrees")
        out["tfov_deg"] = afov / M / DEG
    return _out(**out)


def ref_telescope_magnification(f_obj, f_eye, D, afov=None):
    import numpy as np
    lens = lambda f: np.array([[1.0, 0.0], [-1.0 / f, 1.0]])       # noqa: E731  ray-transfer (ABCD) matrices
    gap = lambda d: np.array([[1.0, d], [0.0, 1.0]])               # noqa: E731
    S = lens(f_eye) @ gap(f_obj + f_eye) @ lens(f_obj)             # an afocal (Keplerian) telescope
    M = abs(S[1, 1])                                               # output angle per input angle
    s = -S[0, 1] / S[1, 1]                                         # where the eyepiece images the objective (exit pupil)
    T = gap(s) @ S
    out = {"M": M, "exit_pupil": D * abs(T[0, 0])}
    if afov is not None:
        out["tfov_deg"] = afov / M / DEG
    return out


_add("telescope_magnification",
     name="Telescope magnification, exit pupil and true field", topic="telescopes / magnification",
     purpose="The angular magnification of a telescope used with an eyepiece, the diameter of its exit pupil and (given the eyepiece's apparent field) its true field of view.",
     inputs=[_phys("f_obj", "length", "focal length of the objective (mm, m)"), _phys("f_eye", "length", "focal length of the eyepiece"),
             _phys("D", "length", "aperture diameter"), _phys("afov", "angle", "apparent field of view of the eyepiece (optional)", required=False)],
     outputs=[_o("M", "magnification", "angular magnification"), _o("exit_pupil", "length", "exit-pupil diameter"),
              _o("exit_pupil_mm", "number", "exit-pupil diameter in millimetres"), _o("tfov_deg", "number", "true field of view in degrees (when afov is given)")],
     primary="M", conditions=["focal lengths and aperture > 0", "thin lenses, afocal (focused at infinity)", "true field = apparent field / M"],
     method="M = f_obj / f_eye; exit pupil = D / M; true field = apparent field / M",
     latex=r"M = \frac{f_\mathrm{obj}}{f_\mathrm{eye}},\quad d_\mathrm{exit} = \frac{D}{M}",
     run=telescope_magnification,
     parse=lambda q: _pairs(q, r"telescope magnification", {"f_obj": "f_obj", "fobj": "f_obj", "f_eye": "f_eye", "feye": "f_eye", "d": "D",
                                                             "afov": "afov"}, phys={"f_obj", "f_eye", "D", "afov"}),
     reference=ref_telescope_magnification,
     wrong=_safe(lambda f_obj, f_eye, D, afov=None: (lambda o: {**o, "M": _r(f_eye / f_obj)})(telescope_magnification(f_obj, f_eye, D, afov))),
     random_inputs=lambda rng: (lambda fe: {"f_obj": fe * _lu(rng, 5, 500), "f_eye": fe, "D": _lu(rng, 0.03, 10),
                                            **({"afov": rng.uniform(30, 110) * DEG} if rng.random() < 0.6 else {})})(_lu(rng, 0.003, 0.05)),
     templates=["Telescope magnification: f_obj = 1200 mm, f_eye = 25 mm, D = 200 mm, afov = 52 deg",
                "Telescope magnification: f_obj = 2 m, f_eye = 10 mm, D = 0.2 m"],
     examples=[{"inputs": {"f_obj": 1.2, "f_eye": 0.025, "D": 0.2, "afov": 52 * DEG}, "outputs": {"M": 48.0, "exit_pupil_mm": 200 / 48, "tfov_deg": 52 / 48}},
               {"inputs": {"f_obj": 2.0, "f_eye": 0.01, "D": 0.2}, "outputs": {"M": 200.0, "exit_pupil": 0.001}}],
     questions=[{"q": "Telescope magnification: f_obj = 1200 mm, f_eye = 25 mm, D = 200 mm, afov = 52 deg", "outputs": {"M": 48.0, "tfov_deg": 52 / 48}},
                {"q": "Telescope magnification: f_obj = 1200 mm, f_eye = 0 mm, D = 200 mm", "outcome": "INVALID_INPUT"},
                {"q": "Telescope magnification: f_obj = 1200 mm, D = 200 mm", "outcome": "NEEDS_INPUT"}])


def light_gathering(D1, D2):
    _pos("the two apertures", D1, D2)
    r = (D1 / D2) ** 2
    return _out(area_ratio=r, mag_gain=2.5 * math.log10(r))


def ref_light_gathering(D1, D2):
    import astropy.units as u
    from scipy.integrate import quad
    area = lambda D: quad(lambda r: 2 * math.pi * r, 0, D / 2, epsabs=0, epsrel=1e-13)[0]   # noqa: E731
    ratio = area(D1) / area(D2)
    return {"area_ratio": ratio, "mag_gain": -float(u.Magnitude(ratio * u.one).value)}


_add("light_gathering_power",
     name="Light-gathering power of two telescopes", topic="telescopes / light gathering",
     purpose="How many times more light one telescope collects than another (ratio of aperture areas) and the corresponding gain in limiting magnitude.",
     inputs=[_phys("D1", "length", "aperture of the first telescope"), _phys("D2", "length", "aperture of the second telescope (or the eye's pupil)")],
     outputs=[_o("area_ratio", "number", "light-gathering ratio (D1/D2)^2"), _o("mag_gain", "number", "gain in limiting magnitude 5 log10(D1/D2)")],
     primary="area_ratio", conditions=["D1, D2 > 0", "unobstructed circular apertures"],
     method="ratio = (D1/D2)^2; gain = 2.5 log10(ratio)", latex=r"\frac{P_1}{P_2} = \left(\frac{D_1}{D_2}\right)^2,\quad \Delta m = 5\log_{10}\frac{D_1}{D_2}",
     run=light_gathering,
     parse=lambda q: _pairs(q, r"light[- ]gathering(?: power)?", {"d1": "D1", "d2": "D2"}, phys={"D1", "D2"}),
     reference=ref_light_gathering,
     wrong=_safe(lambda D1, D2: _out(area_ratio=D1 / D2, mag_gain=2.5 * math.log10(D1 / D2))),
     random_inputs=lambda rng: (lambda D2: {"D2": D2, "D1": D2 * rng.choice([_lu(rng, 1.05, 2000), 1 / _lu(rng, 1.05, 100)])})(_lu(rng, 0.005, 10)),
     templates=["Light-gathering power: D1 = 200 mm, D2 = 7 mm", "Light gathering: D1 = 10 m, D2 = 2.4 m"],
     examples=[{"inputs": {"D1": 0.2, "D2": 0.002}, "outputs": {"area_ratio": 10000.0, "mag_gain": 10.0}},
               {"inputs": {"D1": 2.0, "D2": 1.0}, "outputs": {"area_ratio": 4.0}}],
     questions=[{"q": "Light-gathering power: D1 = 200 mm, D2 = 2 mm", "outputs": {"area_ratio": 10000.0, "mag_gain": 10.0}},
                {"q": "Light-gathering power: D1 = 200 mm, D2 = -2 mm", "outcome": "INVALID_INPUT"},
                {"q": "Light-gathering power: D1 = 200 mm", "outcome": "NEEDS_INPUT"}])


def camera_pixel_scale(f, p, N=None):
    _pos("the focal length and the pixel size", f, p)
    out = {"pixel_scale_arcsec": 2 * math.atan(p / (2 * f)) / ARCSEC}
    if N is not None:
        if N < 1:
            raise ValueError("the number of pixels must be at least 1")
        out["fov_deg"] = 2 * math.atan(N * p / (2 * f)) / DEG
        out["fov_arcmin"] = out["fov_deg"] * 60
    return _out(**out)


def ref_camera_pixel_scale(f, p, N=None):
    import numpy as np
    ang = lambda w: _ref_angle_between(np.array([-w / 2, f, 0.0]), np.array([w / 2, f, 0.0]))   # noqa: E731
    out = {"pixel_scale_arcsec": ang(p) / ARCSEC}
    if N is not None:
        out["fov_deg"] = ang(N * p) / DEG
    return out


_add("camera_pixel_scale",
     name="Pixel scale and field of view of a telescope camera", topic="telescopes / imaging",
     purpose="The angle on the sky covered by one detector pixel at the focal plane of a telescope and, for a row of N pixels, the field of view.",
     inputs=[_phys("f", "length", "focal length of the telescope"), _phys("p", "length", "pixel size (um, mm)"),
             _intin("N", "number of pixels across the detector (optional)", lo=1, required=False)],
     outputs=[_o("pixel_scale_arcsec", "number", "pixel scale in arcseconds per pixel"), _o("fov_deg", "number", "field of view in degrees (with N)"),
              _o("fov_arcmin", "number", "field of view in arcminutes (with N)")],
     primary="pixel_scale_arcsec", conditions=["f, p > 0", "N >= 1", "distortion-free optics; angles centred on the axis"],
     method="angle = 2 atan(w / (2 f)) for a width w centred on the optical axis (about 206265 w/f arcseconds)",
     latex=r"\theta = 2\arctan\frac{w}{2f} \approx 206265''\,\frac{w}{f}",
     run=camera_pixel_scale,
     parse=lambda q: _pairs(q, r"(?:camera )?pixel scale", {"f": "f", "p": "p", "n": "N"}, phys={"f", "p"}),
     reference=ref_camera_pixel_scale,
     wrong=_safe(lambda f, p, N=None: (lambda o: {**o, "pixel_scale_arcsec": _r(2 * math.atan(p / f) / ARCSEC)})(camera_pixel_scale(f, p, N))),
     random_inputs=lambda rng: {"f": _lu(rng, 0.05, 60), "p": _lu(rng, 1e-6, 3e-5), **({"N": rng.randint(100, 10000)} if rng.random() < 0.6 else {})},
     templates=["Pixel scale: f = 2000 mm, p = 3.76 um, N = 6248", "Camera pixel scale: f = 57.6 m, p = 15 um"],
     examples=[{"inputs": {"f": 0.5, "p": 1.0, "N": 1}, "outputs": {"pixel_scale_arcsec": 90 * 3600.0, "fov_deg": 90.0}},
               {"inputs": {"f": 2.0, "p": 3.76e-6}, "outputs": {"pixel_scale_arcsec": 2 * math.atan(3.76e-6 / 4) / ARCSEC}}],
     questions=[{"q": "Pixel scale: f = 2000 mm, p = 3.76 um, N = 6248", "outputs": {"fov_deg": 2 * math.atan(6248 * 3.76e-6 / 4) / DEG}},
                {"q": "Pixel scale: f = 2000 mm, p = 3.76 um, N = 0", "outcome": "INVALID_INPUT"},
                {"q": "Pixel scale: f = 2000 mm", "outcome": "NEEDS_INPUT"}])


def airmass(zen, k=None, m=None):
    if not 0 <= zen <= 85 * DEG:
        raise ValueError("the zenith angle must lie between 0 and 85 degrees (the plane-parallel model fails near the horizon)")
    X = 1 / math.cos(zen)
    out = {"X": X, "alt_deg": 90 - zen / DEG}
    if k is not None:
        if k < 0:
            raise ValueError("the extinction coefficient must not be negative")
        out["extinction"] = k * X
        if m is not None:
            out["m0"] = m - k * X
    elif m is not None:
        raise ValueError("an extinction-corrected magnitude needs the extinction coefficient k")
    return _out(**out)


def ref_airmass(zen, k=None, m=None):
    import numpy as np
    # the line of sight leaves the ground along (sin z, cos z) and crosses the top of a unit-thickness slab y = 1
    t = float(np.linalg.solve(np.array([[math.cos(zen)]]), np.array([1.0]))[0])
    out = {"X": t}
    if k is not None:
        out["extinction"] = k * t
        if m is not None:
            out["m0"] = m - k * t
    return out


_add("airmass_extinction",
     name="Airmass and atmospheric extinction", topic="observing / atmosphere",
     purpose="The airmass X = sec z of a star at zenith angle z (plane-parallel atmosphere) and, with an extinction coefficient k, the extinction k X and the magnitude corrected to outside the atmosphere.",
     inputs=[_phys("zen", "angle", "zenith angle (90 degrees minus the altitude)"),
             _numin("k", "extinction coefficient in magnitudes per airmass (optional)", required=False),
             _numin("m", "observed magnitude (optional; needs k)", required=False)],
     outputs=[_o("X", "number", "airmass"), _o("alt_deg", "number", "altitude in degrees"), _o("extinction", "number", "extinction k X in magnitudes"),
              _o("m0", "number", "magnitude above the atmosphere m - k X")],
     primary="X", conditions=["0 <= z <= 85 degrees", "plane-parallel atmosphere", "k >= 0"],
     method="X = 1/cos z; m0 = m - k X", latex=r"X = \sec z,\quad m_0 = m - kX",
     run=airmass,
     parse=lambda q: _pairs(q, r"airmass(?: and (?:atmospheric )?extinction)?", {"zen": "zen", "z": "zen", "zenith": "zen", "k": "k", "m": "m"},
                            phys={"zen"}, num_units={"m": _MAG, "k": ["mag/airmass"]}),
     reference=ref_airmass,
     wrong=_safe(lambda zen, k=None, m=None: (lambda o: {**o, "X": _r(math.cos(zen))})(airmass(zen, k, m))),
     random_inputs=lambda rng: {"zen": rng.uniform(5, 85) * DEG, **({"k": rng.uniform(0.05, 0.5), "m": rng.uniform(0, 20)} if rng.random() < 0.6 else {})},
     templates=["Airmass: zen = 60 deg", "Airmass and extinction: zen = 45 deg, k = 0.2, m = 10.3"],
     examples=[{"inputs": {"zen": math.pi / 3}, "outputs": {"X": 2.0}},
               {"inputs": {"zen": math.pi / 3, "k": 0.2, "m": 10.4}, "outputs": {"X": 2.0, "m0": 10.0}}],
     questions=[{"q": "Airmass: zen = 60 deg", "outputs": {"X": 2.0}},
                {"q": "Airmass: zen = 60 deg, k = 0.2, m = 10.4", "outputs": {"m0": 10.0}},
                {"q": "Airmass: zen = 89 deg", "outcome": "INVALID_INPUT"},
                {"q": "Airmass:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ positional astronomy and time
def _altaz(H, dec, lat):
    sa = math.sin(lat) * math.sin(dec) + math.cos(lat) * math.cos(dec) * math.cos(H)
    alt = math.asin(max(-1.0, min(1.0, sa)))
    az = math.atan2(-math.cos(dec) * math.sin(H), math.sin(dec) * math.cos(lat) - math.cos(dec) * math.sin(lat) * math.cos(H))
    return alt, az % (2 * math.pi)


def _check_sky(dec, lat):
    if not -math.pi / 2 <= dec <= math.pi / 2:
        raise ValueError("the declination must lie between -90 and 90 degrees")
    if not -math.pi / 2 <= lat <= math.pi / 2:
        raise ValueError("the latitude must lie between -90 and 90 degrees")


def altaz_from_hadec(H, dec, lat):
    _check_sky(dec, lat)
    alt, az = _altaz(H, dec, lat)
    if abs(abs(alt) - math.pi / 2) < 1e-9:
        raise ValueError("at the zenith or nadir the azimuth is undefined")
    az_deg, alt_deg = az / DEG, alt / DEG
    return _out(alt_deg=0.0 if abs(alt_deg) < 1e-10 else alt_deg, az_deg=0.0 if az_deg >= 360 - 1e-10 else az_deg)


def _astropy_altaz(H, dec, lat):
    import astropy.units as u
    from astropy.coordinates import AltAz, EarthLocation, HADec
    from astropy.time import Time
    loc = EarthLocation.from_geodetic(0 * u.deg, lat * u.rad, 0 * u.m)
    t = Time(2451545.0, format="jd", scale="tt")
    c = HADec(ha=H * u.rad, dec=dec * u.rad, location=loc, obstime=t, pressure=0 * u.hPa)
    aa = c.transform_to(AltAz(location=loc, obstime=t, pressure=0 * u.hPa))
    return aa.alt.to(u.deg).value, aa.az.to(u.deg).value


def _altaz_random(rng):
    while True:
        x = {"H": rng.uniform(-180, 180) * DEG, "dec": rng.uniform(-89, 89) * DEG, "lat": rng.uniform(-89, 89) * DEG}
        alt, az = _altaz(x["H"], x["dec"], x["lat"])
        if abs(alt) < 89 * DEG and 0.05 < az / DEG < 360 - 0.05 and abs(alt) > 0.05 * DEG:
            return x


_add("altaz_from_hadec",
     name="Altitude and azimuth from hour angle and declination", topic="positional astronomy / coordinates",
     purpose="The altitude and azimuth of a star of declination dec at hour angle H seen from latitude lat (horizontal coordinates from equatorial ones).",
     inputs=[_phys("H", "angle", "hour angle (hours or degrees; positive west of the meridian)"), _phys("dec", "angle", "declination"),
             _phys("lat", "angle", "observer's latitude")],
     outputs=[_o("alt_deg", "number", "altitude in degrees (geometric, no refraction)"), _o("az_deg", "number", "azimuth in degrees from north through east")],
     primary="alt_deg", conditions=["|dec| <= 90 degrees", "|lat| <= 90 degrees", "not at the zenith or nadir", "no refraction"],
     method="sin a = sin(lat) sin(dec) + cos(lat) cos(dec) cos H; tan A = -cos(dec) sin H / (sin(dec) cos(lat) - cos(dec) sin(lat) cos H)",
     latex=r"\sin a = \sin\phi\sin\delta + \cos\phi\cos\delta\cos H,\quad \tan A = \frac{-\cos\delta\sin H}{\sin\delta\cos\phi - \cos\delta\sin\phi\cos H}",
     run=altaz_from_hadec,
     parse=lambda q: _pairs(q, r"alt(?:itude)?[- ]?az(?:imuth)? from hour angle(?: and declination)?", {"h": "H", "ha": "H", "dec": "dec", "lat": "lat"},
                            phys={"H", "dec", "lat"}, hours={"H"}),
     reference=lambda H, dec, lat: dict(zip(("alt_deg", "az_deg"), _astropy_altaz(H, dec, lat))),
     wrong=_safe(lambda H, dec, lat: _out(alt_deg=math.asin(math.sin(lat) * math.sin(dec) + math.cos(lat) * math.cos(dec) * math.sin(H)) / DEG,
                                          az_deg=_altaz(H, dec, lat)[1] / DEG)),
     random_inputs=_altaz_random,
     templates=["Altitude-azimuth from hour angle: H = 2 h, dec = 38.78 deg, lat = 52 deg", "Alt-az from hour angle and declination: H = -45 deg, dec = -16.7 deg, lat = 40 deg"],
     examples=[{"inputs": {"H": math.pi / 2, "dec": math.pi / 6, "lat": math.pi / 4}, "outputs": {"alt_deg": math.asin(math.sin(math.pi / 4) * 0.5) / DEG}},
               {"inputs": {"H": 0.0, "dec": 0.0, "lat": math.pi / 4}, "outputs": {"alt_deg": 45.0, "az_deg": 180.0}}],
     questions=[{"q": "Altitude-azimuth from hour angle: H = 0 h, dec = 0 deg, lat = 45 deg", "outputs": {"alt_deg": 45.0, "az_deg": 180.0}},
                {"q": "Altitude-azimuth from hour angle: H = 6 h, dec = 0 deg, lat = 45 deg", "outputs": {"az_deg": 270.0}},
                {"q": "Altitude-azimuth from hour angle: H = 0 h, dec = 100 deg, lat = 45 deg", "outcome": "INVALID_INPUT"},
                {"q": "Altitude-azimuth from hour angle: H = 0 h, dec = 10 deg", "outcome": "NEEDS_INPUT"}])


def culmination(dec, lat):
    _check_sky(dec, lat)
    d, f = dec / DEG, lat / DEG
    up = 90 - abs(f - d)
    low = abs(f + d) - 90
    return _out(alt_upper_deg=up, alt_lower_deg=low, circumpolar=low > 0, never_rises=up < 0)


def ref_culmination(dec, lat):
    up = _astropy_altaz(0.0, dec, lat)[0]
    low = _astropy_altaz(math.pi, dec, lat)[0]
    return {"alt_upper_deg": up, "alt_lower_deg": low, "circumpolar": low > 0, "never_rises": up < 0}


def _culm_random(rng):
    while True:
        dec, lat = rng.uniform(-89, 89), rng.uniform(-89, 89)
        up, low = 90 - abs(lat - dec), abs(lat + dec) - 90
        if abs(up) > 0.5 and abs(low) > 0.5:
            return {"dec": dec * DEG, "lat": lat * DEG}


_add("culmination_altitude",
     name="Altitude of a star at culmination", topic="positional astronomy / meridian transit",
     purpose="The altitudes of a star when it crosses the meridian above the pole (upper culmination) and below it (lower culmination), and whether it is circumpolar or never rises.",
     inputs=[_phys("dec", "angle", "declination"), _phys("lat", "angle", "observer's latitude")],
     outputs=[_o("alt_upper_deg", "number", "altitude at upper culmination (degrees)"), _o("alt_lower_deg", "number", "altitude at lower culmination (degrees)"),
              _o("circumpolar", "boolean", "never sets"), _o("never_rises", "boolean", "never rises")],
     primary="alt_upper_deg", conditions=["|dec|, |lat| <= 90 degrees", "geometric horizon, no refraction"],
     method="upper: a = 90 - |lat - dec|; lower: a = |lat + dec| - 90", latex=r"a_\mathrm{upper} = 90^\circ - |\phi - \delta|,\quad a_\mathrm{lower} = |\phi + \delta| - 90^\circ",
     run=culmination,
     parse=lambda q: _pairs(q, r"culmination(?: altitude)?|altitude at culmination", {"dec": "dec", "lat": "lat"}, phys={"dec", "lat"}),
     reference=ref_culmination,
     wrong=_safe(lambda dec, lat: (lambda o: {**o, "alt_upper_deg": _r(90 - abs(lat + dec) / DEG)})(culmination(dec, lat))),
     random_inputs=_culm_random,
     templates=["Culmination altitude: dec = 38.78 deg, lat = 51.5 deg", "Altitude at culmination: dec = -60.8 deg, lat = -33.9 deg"],
     examples=[{"inputs": {"dec": 20 * DEG, "lat": 50 * DEG}, "outputs": {"alt_upper_deg": 60.0, "alt_lower_deg": -20.0, "circumpolar": False}},
               {"inputs": {"dec": 70 * DEG, "lat": 50 * DEG}, "outputs": {"alt_upper_deg": 70.0, "alt_lower_deg": 30.0, "circumpolar": True}}],
     questions=[{"q": "Culmination altitude: dec = 20 deg, lat = 50 deg", "outputs": {"alt_upper_deg": 60.0}},
                {"q": "Culmination altitude: dec = -60 deg, lat = 50 deg", "outputs": {"never_rises": True}},
                {"q": "Culmination altitude: dec = 20 deg, lat = 95 deg", "outcome": "INVALID_INPUT"},
                {"q": "Culmination altitude: dec = 20 deg", "outcome": "NEEDS_INPUT"}])


def rise_set(dec, lat):
    _check_sky(dec, lat)
    x = -math.tan(lat) * math.tan(dec)
    if abs(lat) >= math.pi / 2 - 1e-12 or not -1 < x < 1:
        raise ValueError("the star does not rise and set at this latitude (it is circumpolar or never rises)")
    H0 = math.acos(x)
    az = math.acos(max(-1.0, min(1.0, math.sin(dec) / math.cos(lat))))
    return _out(H_set_h=H0 / math.pi * 12, time_up_sidereal_h=2 * H0 / math.pi * 12, az_rise_deg=az / DEG, az_set_deg=360 - az / DEG)


def ref_rise_set(dec, lat):
    import numpy as np
    from scipy.optimize import brentq

    def horizon(H):                    # equatorial unit vector rotated into the horizon frame (rotation matrices)
        v = np.array([math.cos(dec) * math.cos(H), -math.cos(dec) * math.sin(H), math.sin(dec)])
        c = math.pi / 2 - lat
        Ry = np.array([[math.cos(c), 0.0, -math.sin(c)], [0.0, 1.0, 0.0], [math.sin(c), 0.0, math.cos(c)]])
        return Ry @ v                  # x: towards the south point, y: east, z: zenith
    H0 = brentq(lambda H: horizon(H)[2], 0.0, math.pi, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    w = horizon(-H0)
    az = math.atan2(w[1], -w[0]) % (2 * math.pi)
    return {"H_set_h": H0 / math.pi * 12, "az_rise_deg": az / DEG}


def _rise_random(rng):
    lat = rng.uniform(-70, 70) * DEG
    lim = math.atan(1 / math.tan(abs(lat))) if lat != 0 else math.pi / 2
    dec = rng.uniform(0.02, 0.98) * lim * rng.choice([-1, 1])
    return {"dec": dec, "lat": lat}


_add("rise_set_hour_angle",
     name="Rising and setting of a star", topic="positional astronomy / rising and setting",
     purpose="The hour angle at which a star sets, how long it stays above the horizon (sidereal hours) and the azimuths at which it rises and sets (geometric horizon, no refraction).",
     inputs=[_phys("dec", "angle", "declination"), _phys("lat", "angle", "observer's latitude")],
     outputs=[_o("H_set_h", "number", "hour angle at setting (hours)"), _o("time_up_sidereal_h", "number", "time above the horizon (sidereal hours)"),
              _o("az_rise_deg", "number", "azimuth of rising (degrees from north through east)"), _o("az_set_deg", "number", "azimuth of setting")],
     primary="H_set_h", conditions=["the star rises and sets: |tan(lat) tan(dec)| < 1", "geometric horizon (no refraction, no star size)"],
     method="cos H0 = -tan(lat) tan(dec); cos A_rise = sin(dec) / cos(lat)", latex=r"\cos H_0 = -\tan\phi\tan\delta,\quad \cos A_\mathrm{rise} = \frac{\sin\delta}{\cos\phi}",
     run=rise_set,
     parse=lambda q: _pairs(q, r"rising and setting|rise[- ]set(?: hour angle)?", {"dec": "dec", "lat": "lat"}, phys={"dec", "lat"}),
     reference=ref_rise_set,
     wrong=_safe(lambda dec, lat: (lambda o: {**o, "H_set_h": _r(math.acos(math.tan(lat) * math.tan(dec)) / math.pi * 12)})(rise_set(dec, lat))),
     random_inputs=_rise_random,
     templates=["Rising and setting: dec = 20 deg, lat = 45 deg", "Rise-set hour angle: dec = -16.7 deg, lat = 40.7 deg"],
     examples=[{"inputs": {"dec": 0.0, "lat": 45 * DEG}, "outputs": {"H_set_h": 6.0, "az_rise_deg": 90.0, "az_set_deg": 270.0}},
               {"inputs": {"dec": 30 * DEG, "lat": 0.0}, "outputs": {"H_set_h": 6.0, "az_rise_deg": 60.0}}],
     questions=[{"q": "Rising and setting: dec = 0 deg, lat = 45 deg", "outputs": {"H_set_h": 6.0, "time_up_sidereal_h": 12.0}},
                {"q": "Rising and setting: dec = 80 deg, lat = 50 deg", "outcome": "INVALID_INPUT"},
                {"q": "Rising and setting: dec = 20 deg", "outcome": "NEEDS_INPUT"}])


def angular_separation(ra1, dec1, ra2, dec2):
    for d in (dec1, dec2):
        if not -math.pi / 2 <= d <= math.pi / 2:
            raise ValueError("declinations must lie between -90 and 90 degrees")
    dra = ra2 - ra1
    num = math.hypot(math.cos(dec2) * math.sin(dra), math.cos(dec1) * math.sin(dec2) - math.sin(dec1) * math.cos(dec2) * math.cos(dra))
    den = math.sin(dec1) * math.sin(dec2) + math.cos(dec1) * math.cos(dec2) * math.cos(dra)
    s = math.atan2(num, den)
    return _out(sep_deg=s / DEG, sep_arcsec=s / ARCSEC)


_SKY2 = {"ra1": "ra1", "dec1": "dec1", "ra2": "ra2", "dec2": "dec2"}


def _sky2_random(rng):
    return {"ra1": rng.uniform(0, 360) * DEG, "dec1": rng.uniform(-89, 89) * DEG, "ra2": rng.uniform(0, 360) * DEG, "dec2": rng.uniform(-89, 89) * DEG}


_add("angular_separation",
     name="Angular separation of two sky positions", topic="positional astronomy / coordinates",
     purpose="The great-circle angle between two objects given by their right ascensions and declinations.",
     inputs=[_phys("ra1", "angle", "right ascension of object 1 (hours or degrees)"), _phys("dec1", "angle", "declination of object 1"),
             _phys("ra2", "angle", "right ascension of object 2"), _phys("dec2", "angle", "declination of object 2")],
     outputs=[_o("sep_deg", "number", "separation in degrees"), _o("sep_arcsec", "number", "separation in arcseconds")],
     primary="sep_deg", conditions=["|dec| <= 90 degrees"],
     method="Vincenty's formula (accurate at all separations)", latex=r"\cos\theta = \sin\delta_1\sin\delta_2 + \cos\delta_1\cos\delta_2\cos(\alpha_2 - \alpha_1)",
     run=angular_separation,
     parse=lambda q: _pairs(q, r"angular separation", _SKY2, phys={"ra1", "dec1", "ra2", "dec2"}, hours={"ra1", "ra2"}),
     reference=lambda ra1, dec1, ra2, dec2: (lambda s: {"sep_deg": s / DEG, "sep_arcsec": s / ARCSEC})(
         _ref_angle_between(_ref_unit_vec(ra1, dec1), _ref_unit_vec(ra2, dec2))),
     wrong=_safe(lambda ra1, dec1, ra2, dec2: (lambda s: _out(sep_deg=s / DEG, sep_arcsec=s / ARCSEC))(math.hypot(ra2 - ra1, dec2 - dec1))),
     random_inputs=_sky2_random,
     templates=["Angular separation: ra1 = 6.752 h, dec1 = -16.716 deg, ra2 = 7.655 h, dec2 = 5.225 deg"],
     examples=[{"inputs": {"ra1": 0.0, "dec1": 0.0, "ra2": math.pi / 2, "dec2": 0.0}, "outputs": {"sep_deg": 90.0}},
               {"inputs": {"ra1": 0.0, "dec1": math.pi / 4, "ra2": math.pi, "dec2": math.pi / 4}, "outputs": {"sep_deg": 90.0}}],
     questions=[{"q": "Angular separation: ra1 = 0 h, dec1 = 0 deg, ra2 = 6 h, dec2 = 0 deg", "outputs": {"sep_deg": 90.0}},
                {"q": "Angular separation: ra1 = 0 h, dec1 = 95 deg, ra2 = 6 h, dec2 = 0 deg", "outcome": "INVALID_INPUT"},
                {"q": "Angular separation: ra1 = 0 h, dec1 = 0 deg", "outcome": "NEEDS_INPUT"}])


def position_angle(ra1, dec1, ra2, dec2):
    for d in (dec1, dec2):
        if not -math.pi / 2 < d < math.pi / 2:
            raise ValueError("declinations must lie strictly between -90 and 90 degrees (no position angle at a pole)")
    dra = ra2 - ra1
    y = math.sin(dra) * math.cos(dec2)
    x = math.cos(dec1) * math.sin(dec2) - math.sin(dec1) * math.cos(dec2) * math.cos(dra)
    if math.hypot(x, y) < 1e-12:
        raise ValueError("the two positions coincide (or are antipodal): no position angle")
    pa = math.atan2(y, x) % (2 * math.pi) / DEG
    return _out(pa_deg=0.0 if pa >= 360 - 1e-10 else pa)


def ref_position_angle(ra1, dec1, ra2, dec2):
    import numpy as np
    b = _ref_unit_vec(ra2, dec2)
    north = np.array([-math.sin(dec1) * math.cos(ra1), -math.sin(dec1) * math.sin(ra1), math.cos(dec1)])
    east = np.array([-math.sin(ra1), math.cos(ra1), 0.0])
    return {"pa_deg": math.atan2(float(b @ east), float(b @ north)) % (2 * math.pi) / DEG}


def _pa_random(rng):
    while True:
        x = _sky2_random(rng)
        try:
            pa = position_angle(**x)["pa_deg"]
        except ValueError:
            continue
        if 1e-3 < pa < 360 - 1e-3:
            return x


_add("position_angle",
     name="Position angle between two sky positions", topic="positional astronomy / coordinates",
     purpose="The position angle of object 2 as seen from object 1, measured from north through east.",
     inputs=[_phys("ra1", "angle", "right ascension of object 1 (hours or degrees)"), _phys("dec1", "angle", "declination of object 1"),
             _phys("ra2", "angle", "right ascension of object 2"), _phys("dec2", "angle", "declination of object 2")],
     outputs=[_o("pa_deg", "number", "position angle in degrees (0 = north, 90 = east)")],
     primary="pa_deg", conditions=["|dec| < 90 degrees", "distinct, non-antipodal positions"],
     method="tan PA = sin(dra) cos(dec2) / (cos(dec1) sin(dec2) - sin(dec1) cos(dec2) cos(dra))",
     latex=r"\tan\mathrm{PA} = \frac{\sin\Delta\alpha\cos\delta_2}{\cos\delta_1\sin\delta_2 - \sin\delta_1\cos\delta_2\cos\Delta\alpha}",
     run=position_angle,
     parse=lambda q: _pairs(q, r"position angle", _SKY2, phys={"ra1", "dec1", "ra2", "dec2"}, hours={"ra1", "ra2"}),
     reference=ref_position_angle,
     wrong=_safe(lambda ra1, dec1, ra2, dec2: _out(pa_deg=(position_angle(ra1, dec1, ra2, dec2)["pa_deg"] + 180) % 360)),
     random_inputs=_pa_random,
     templates=["Position angle: ra1 = 10 h, dec1 = 20 deg, ra2 = 10.1 h, dec2 = 21 deg"],
     examples=[{"inputs": {"ra1": 0.0, "dec1": 0.0, "ra2": 0.0, "dec2": 0.1}, "outputs": {"pa_deg": 0.0}},
               {"inputs": {"ra1": 0.0, "dec1": 0.0, "ra2": 0.1, "dec2": 0.0}, "outputs": {"pa_deg": 90.0}}],
     questions=[{"q": "Position angle: ra1 = 0 deg, dec1 = 0 deg, ra2 = 1 deg, dec2 = 0 deg", "outputs": {"pa_deg": 90.0}},
                {"q": "Position angle: ra1 = 0 deg, dec1 = 0 deg, ra2 = 0 deg, dec2 = 0 deg", "outcome": "INVALID_INPUT"},
                {"q": "Position angle: ra1 = 0 deg, dec1 = 0 deg", "outcome": "NEEDS_INPUT"}])


_DAYS = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def _leap(y):
    return y % 4 == 0 and (y % 100 != 0 or y % 400 == 0)


def _jd_exact(year, month, day, hour, minute, second):
    if not (year > 1582 or (year == 1582 and (month > 10 or (month == 10 and day >= 15)))):
        raise ValueError("the date must be in the Gregorian calendar (on or after 1582-10-15)")
    if year > 9999:
        raise ValueError("the year must be at most 9999")
    if not 1 <= month <= 12:
        raise ValueError("the month must be 1 to 12")
    dim = 29 if (month == 2 and _leap(year)) else _DAYS[month - 1]
    if not 1 <= day <= dim:
        raise ValueError(f"day must be 1 to {dim} in that month")
    if not (0 <= hour <= 23 and 0 <= minute <= 59 and 0 <= second < 60):
        raise ValueError("the time must be 0-23 h, 0-59 min, 0 <= s < 60")
    y, m = (year - 1, month + 12) if month <= 2 else (year, month)
    a = y // 100
    b = 2 - a + a // 4
    whole = (1461 * (y + 4716)) // 4 + (306001 * (m + 1)) // 10000 + day + b
    return Fraction(whole) - Fraction(3049, 2) + (Fraction(hour * 3600 + minute * 60) + Fraction(second)) / 86400


def julian_date(year, month, day, hour=0, minute=0, second=0.0):
    jd = _jd_exact(int(year), int(month), int(day), int(hour), int(minute), second)
    return _out(JD=float(jd), MJD=float(jd - Fraction(4800001, 2)))


def ref_julian_date(year, month, day, hour=0, minute=0, second=0.0):
    from astropy.time import Time
    t = Time({"year": int(year), "month": int(month), "day": int(day), "hour": int(hour), "minute": int(minute), "second": float(second)},
             format="ymdhms", scale="tt")
    return {"JD": float(t.jd1 + t.jd2), "MJD": float(t.mjd)}


def _date_random(rng):
    while True:
        y = rng.randint(1583, 2500)
        if 1855 <= y <= 1862:
            continue
        mo = rng.randint(1, 12)
        d = rng.randint(1, 29 if (mo == 2 and _leap(y)) else _DAYS[mo - 1])
        return {"year": y, "month": mo, "day": d, "hour": rng.randint(0, 23), "minute": rng.randint(0, 59), "second": round(rng.uniform(0, 59.99), 3)}


_add("julian_date",
     name="Julian date of a calendar date", topic="time / calendars",
     purpose="The Julian date and the modified Julian date of a Gregorian calendar date and time of day (in the same time scale as the date, e.g. TT or UT).",
     inputs=[_intin("year", "year (Gregorian, 1583 or later)"), _intin("month", "month (1-12)", 1, 12), _intin("day", "day of the month"),
             _intin("hour", "hour (0-23, default 0)", 0, 23, required=False), _intin("minute", "minute (0-59, default 0)", 0, 59, required=False),
             _numin("second", "second (0 <= s < 60, default 0)", required=False)],
     outputs=[_o("JD", "number", "Julian date (days)"), _o("MJD", "number", "modified Julian date JD - 2400000.5")],
     primary="JD", conditions=["Gregorian calendar (on or after 1582-10-15)", "a valid date and time of day (no leap second)"],
     method="Meeus' algorithm in exact integer arithmetic: JD = floor(365.25(y+4716)) + floor(30.6001(m+1)) + D + B - 1524.5",
     latex=r"\mathrm{JD} = \lfloor 365.25(y + 4716)\rfloor + \lfloor 30.6001(m + 1)\rfloor + D + B - 1524.5",
     run=julian_date,
     parse=lambda q: _pairs(q, r"julian date", {"year": "year", "month": "month", "day": "day", "hour": "hour", "minute": "minute", "second": "second"}),
     reference=ref_julian_date,
     wrong=_safe(lambda year, month, day, hour=0, minute=0, second=0.0: (lambda jd: _out(JD=jd, MJD=jd - 2400000.5))(
         float(_jd_exact(int(year), int(month), int(day), int(hour), int(minute), second)) - 2 + int(year) // 100 - int(year) // 400)),
     random_inputs=_date_random,
     templates=["Julian date: year = 2000, month = 1, day = 1, hour = 12", "Julian date: year = 1969, month = 7, day = 20, hour = 20, minute = 17, second = 40"],
     examples=[{"inputs": {"year": 2000, "month": 1, "day": 1, "hour": 12}, "outputs": {"JD": 2451545.0, "MJD": 51544.5}},
               {"inputs": {"year": 1957, "month": 10, "day": 4, "hour": 19, "minute": 26, "second": 24}, "outputs": {"JD": 2436116.31}},
               {"inputs": {"year": 1858, "month": 11, "day": 18}, "outputs": {"MJD": 1.0}}],
     questions=[{"q": "Julian date: year = 2000, month = 1, day = 1, hour = 12", "outputs": {"JD": 2451545.0}},
                {"q": "Julian date: year = 2023, month = 2, day = 29", "outcome": "INVALID_INPUT"},
                {"q": "Julian date: year = 2000, month = 1", "outcome": "NEEDS_INPUT"}])


def calendar_from_jd(jd):
    if not 2299160.5 <= jd <= 5373484.5:
        raise ValueError("the Julian date must lie in the Gregorian era (JD 2299160.5 to 5373484.5)")
    J = Fraction(jd) + Fraction(1, 2)
    Z = J.numerator // J.denominator
    F = J - Z
    alpha = (Z * 100 - 186721625) // 3652425            # floor((Z - 1867216.25) / 36524.25)
    A = Z + 1 + alpha - alpha // 4
    B = A + 1524
    Cc = (B * 100 - 12210) // 36525                      # floor((B - 122.1) / 365.25)
    D = (1461 * Cc) // 4
    E = ((B - D) * 10000) // 306001
    day = B - D - (306001 * E) // 10000
    month = E - 1 if E < 14 else E - 13
    year = Cc - 4716 if month > 2 else Cc - 4715
    return _out(year=year, month=month, day=day, hours=float(24 * F))


def ref_calendar_from_jd(jd):
    from astropy.time import Time
    t = Time(jd, format="jd", scale="tt").ymdhms
    return {"year": int(t.year), "month": int(t.month), "day": int(t.day), "hours": float(t.hour + t.minute / 60 + t.second / 3600)}


_add("calendar_date_from_julian_date",
     name="Calendar date from a Julian date", topic="time / calendars",
     purpose="The Gregorian calendar date and the time of day (decimal hours) of a Julian date.",
     inputs=[_numin("jd", "Julian date (days)")],
     outputs=[_o("year", "number", "year"), _o("month", "number", "month"), _o("day", "number", "day of the month"),
              _o("hours", "number", "time of day in decimal hours")],
     primary="day", conditions=["2299160.5 <= JD <= 5373484.5 (Gregorian era up to the year 9999)"],
     method="Meeus' inverse algorithm in exact integer and rational arithmetic", latex=r"Z = \lfloor \mathrm{JD} + 0.5\rfloor,\ F = \mathrm{JD} + 0.5 - Z",
     run=calendar_from_jd,
     parse=lambda q: _pairs(q, r"calendar date(?: from (?:a )?julian date)?", {"jd": "jd"}),
     reference=ref_calendar_from_jd,
     wrong=_safe(lambda jd: (lambda o: {**o, "day": o["day"] % 28 + 1})(calendar_from_jd(jd - 13.0))),
     random_inputs=lambda rng: {"jd": float(rng.randint(2299200, 2900000)) - 0.5 + rng.randint(10000, 990000) / 1e6},
     templates=["Calendar date from Julian date: jd = 2451545.0", "Calendar date: jd = 2460000.25"],
     examples=[{"inputs": {"jd": 2451545.0}, "outputs": {"year": 2000, "month": 1, "day": 1, "hours": 12.0}},
               {"inputs": {"jd": 2436116.31}, "outputs": {"year": 1957, "month": 10, "day": 4, "hours": 0.81 * 24}}],
     questions=[{"q": "Calendar date from Julian date: jd = 2451545.0", "outputs": {"year": 2000, "month": 1, "day": 1}},
                {"q": "Calendar date from Julian date: jd = 1000000", "outcome": "INVALID_INPUT"},
                {"q": "Calendar date from Julian date:", "outcome": "NEEDS_INPUT"}])


def sidereal_time(jd, lon=0.0):
    if not 2299160.5 <= jd <= 5373484.5:
        raise ValueError("the Julian date must lie between 2299160.5 and 5373484.5")
    if not -math.pi <= lon <= 2 * math.pi:
        raise ValueError("the longitude must lie between -180 and 360 degrees (east positive)")
    d = Fraction(jd)
    t = (d - 2451545) / 36525
    f = 86400 * (d - (d.numerator // d.denominator))
    sec = (Fraction(24110.54841) - 43200) + (Fraction(8640184.812866) + (Fraction(0.093104) + Fraction(-6.2e-6) * t) * t) * t + f
    gmst = float(sec % 86400) / 3600
    lmst = (gmst + lon / DEG / 15) % 24
    return _out(gmst_h=gmst, lmst_h=lmst)


def ref_sidereal_time(jd, lon=0.0):
    import astropy.units as u
    from astropy.time import Time
    t = Time(jd, format="jd", scale="ut1")
    return {"gmst_h": float(t.sidereal_time("mean", "greenwich", model="IAU1982").hour),
            "lmst_h": float(t.sidereal_time("mean", longitude=lon * u.rad, model="IAU1982").hour)}


def _st_random(rng):
    while True:
        x = {"jd": float(rng.randint(2400000, 2500000)) + rng.randint(0, 999999) / 1e6, "lon": rng.uniform(-180, 180) * DEG}
        o = sidereal_time(**x)
        if 0.01 < o["gmst_h"] < 23.99 and 0.01 < o["lmst_h"] < 23.99:
            return x


_add("sidereal_time",
     name="Greenwich and local mean sidereal time", topic="time / sidereal time",
     purpose="The Greenwich mean sidereal time (IAU 1982 expression) at a Julian date in UT1, and the local mean sidereal time at an east longitude.",
     inputs=[_numin("jd", "Julian date (UT1)"), _phys("lon", "angle", "east longitude (default 0)", required=False)],
     outputs=[_o("gmst_h", "number", "Greenwich mean sidereal time (hours)"), _o("lmst_h", "number", "local mean sidereal time (hours)")],
     primary="gmst_h", conditions=["2299160.5 <= JD <= 5373484.5", "the date in UT1", "east longitudes positive"],
     method="GMST = 24110.54841 s + 8640184.812866 T + 0.093104 T^2 - 6.2e-6 T^3 + UT1 seconds of the day (T from J2000, Julian centuries); LMST = GMST + longitude/15",
     latex=r"\mathrm{GMST} = 24110.54841^\mathrm{s} + 8640184.812866^\mathrm{s}\,T + 0.093104^\mathrm{s}\,T^2 - 6.2\times10^{-6}\,{}^\mathrm{s}T^3 + \mathrm{UT1}",
     run=sidereal_time,
     parse=lambda q: _pairs(q, r"(?:mean )?sidereal time", {"jd": "jd", "lon": "lon", "longitude": "lon"}, phys={"lon"}),
     reference=ref_sidereal_time,
     wrong=_safe(lambda jd, lon=0.0: (lambda g: _out(gmst_h=g, lmst_h=(g + lon / DEG / 15) % 24))(
         float(((Fraction(24110.54841) - 43200) + Fraction(8640184.812866) * (Fraction(jd) - 2451545) / 36525) % 86400) / 3600)),
     random_inputs=_st_random,
     templates=["Sidereal time: jd = 2451545.0", "Mean sidereal time: jd = 2460000.5, lon = -71.06 deg"],
     examples=[{"inputs": {"jd": 2451545.0}, "outputs": {"gmst_h": float((Fraction(24110.54841) + 43200) % 86400) / 3600}},
               {"inputs": {"jd": 2451545.0, "lon": 15 * DEG}, "outputs": {"lmst_h": float((Fraction(24110.54841) + 43200) % 86400) / 3600 + 1}}],
     questions=[{"q": "Sidereal time: jd = 2451545.0", "outputs": {"gmst_h": float((Fraction(24110.54841) + 43200) % 86400) / 3600}},
                {"q": "Sidereal time: jd = 100", "outcome": "INVALID_INPUT"},
                {"q": "Sidereal time:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ plasma and space physics
_PARTICLES = {"electron": (ME, "e-"), "proton": (MP, "p+")}
_PART_IN = _choice("particle", ["electron", "proton"], "particle species: electron or proton (default electron)", required=False)


def _mass(particle):
    if particle not in _PARTICLES:
        raise ValueError("the particle must be 'electron' or 'proton'")
    return _PARTICLES[particle][0]


def _pf():
    import astropy.units as u
    import plasmapy.formulary as pf
    return u, pf


def plasma_frequency(n, particle="electron"):
    _pos("the density", n)
    w = math.sqrt(n * QE * QE / (EPS0 * _mass(particle)))
    return _out(omega_p=w, f_p=w / (2 * math.pi))


_add("plasma_frequency",
     name="Plasma frequency", topic="plasma physics / frequencies",
     purpose="The plasma (Langmuir) oscillation frequency of electrons (or protons) of number density n.",
     inputs=[_phys("n", "number_density", "number density (m^-3, cm^-3)"), _PART_IN],
     outputs=[_o("omega_p", "angular_frequency", "plasma angular frequency"), _o("f_p", "frequency", "plasma frequency")],
     primary="f_p", conditions=["n > 0", "cold, unmagnetized plasma; singly charged particles"],
     method="omega_p = sqrt(n e^2 / (eps0 m))", latex=r"\omega_p = \sqrt{\frac{n e^2}{\varepsilon_0 m}}",
     run=plasma_frequency,
     parse=lambda q: _pairs(q, r"(?:electron |proton )?plasma frequency", {"n": "n", "particle": "particle"}, phys={"n"}, words={"particle"}),
     reference=lambda n, particle="electron": (lambda u, pf: {"omega_p": float(pf.plasma_frequency(n * u.m ** -3, _PARTICLES[particle][1]).to(u.rad / u.s).value)})(*_pf()),
     wrong=_safe(lambda n, particle="electron": (lambda w: _out(omega_p=w, f_p=w))(math.sqrt(n * QE * QE / (4 * math.pi * EPS0 * _mass(particle))))),
     random_inputs=lambda rng: {"n": _lu(rng, 1e4, 1e30), **({"particle": rng.choice(["electron", "proton"])} if rng.random() < 0.6 else {})},
     templates=["Plasma frequency: n = 1e12 m^-3", "Plasma frequency: n = 5 cm^-3, particle = electron"],
     examples=[{"inputs": {"n": EPS0 * ME / QE ** 2}, "outputs": {"omega_p": 1.0}},
               {"inputs": {"n": 4 * EPS0 * MP / QE ** 2, "particle": "proton"}, "outputs": {"omega_p": 2.0}}],
     questions=[{"q": "Plasma frequency: n = 1e12 m^-3", "outputs": {"omega_p": math.sqrt(1e12 * QE ** 2 / (EPS0 * ME))}},
                {"q": "Plasma frequency: n = 1e12 m^-3, particle = neutron", "outcome": "INVALID_INPUT"},
                {"q": "Plasma frequency:", "outcome": "NEEDS_INPUT"}])


def debye_length(T, n):
    _pos("the temperature and the density", T, n)
    lam = math.sqrt(EPS0 * KB * T / (n * QE * QE))
    return _out(lambda_D=lam, lambda_D_nm=lam * 1e9)


_add("debye_length",
     name="Debye length", topic="plasma physics / lengths",
     purpose="The electron Debye screening length of a plasma of electron temperature T and electron density n.",
     inputs=[_phys("T", "temperature", "electron temperature"), _phys("n", "number_density", "electron density (m^-3, cm^-3)")],
     outputs=[_o("lambda_D", "length", "Debye length"), _o("lambda_D_nm", "number", "Debye length in nanometres")],
     primary="lambda_D", conditions=["T, n > 0", "electron screening only"],
     method="lambda_D = sqrt(eps0 k T / (n e^2))", latex=r"\lambda_D = \sqrt{\frac{\varepsilon_0 k_B T_e}{n_e e^2}}",
     run=debye_length,
     parse=lambda q: _pairs(q, r"debye length", {"t": "T", "n": "n"}, phys={"T", "n"}),
     reference=lambda T, n: (lambda u, pf: (lambda lam: {"lambda_D": lam, "lambda_D_nm": lam * 1e9})(float(pf.Debye_length(T * u.K, n * u.m ** -3).to(u.m).value)))(*_pf()),
     wrong=_safe(lambda T, n: (lambda lam: _out(lambda_D=lam, lambda_D_nm=lam * 1e9))(math.sqrt(EPS0 * KB * T / (n * QE)))),
     random_inputs=lambda rng: {"T": _lu(rng, 1e3, 1e9), "n": _lu(rng, 1e4, 1e26)},
     templates=["Debye length: T = 1e5 K, n = 1e18 m^-3", "Debye length: T = 1.5e6 K, n = 1e8 cm^-3"],
     examples=[{"inputs": {"T": QE * QE / (EPS0 * KB), "n": 1.0}, "outputs": {"lambda_D": 1.0}},
               {"inputs": {"T": 4 * QE * QE / (EPS0 * KB), "n": 1.0}, "outputs": {"lambda_D": 2.0}}],
     questions=[{"q": "Debye length: T = 1e5 K, n = 1e18 m^-3", "outputs": {"lambda_D": math.sqrt(EPS0 * KB * 1e5 / (1e18 * QE * QE))}},
                {"q": "Debye length: T = 1e5 K, n = 0 m^-3", "outcome": "INVALID_INPUT"},
                {"q": "Debye length: T = 1e5 K", "outcome": "NEEDS_INPUT"}])


def debye_number(T, n):
    _pos("the temperature and the density", T, n)
    lam = math.sqrt(EPS0 * KB * T / (n * QE * QE))
    return _out(N_D=4 / 3 * math.pi * n * lam ** 3, lambda_D=lam)


_add("debye_number",
     name="Number of particles in a Debye sphere", topic="plasma physics / plasma parameter",
     purpose="The number of electrons in a sphere of one Debye length radius (N_D >> 1 means the gas behaves as a plasma).",
     inputs=[_phys("T", "temperature", "electron temperature"), _phys("n", "number_density", "electron density")],
     outputs=[_o("N_D", "number", "particles per Debye sphere"), _o("lambda_D", "length", "Debye length")],
     primary="N_D", conditions=["T, n > 0"],
     method="N_D = (4 pi / 3) n lambda_D^3", latex=r"N_D = \frac{4\pi}{3}n_e\lambda_D^3",
     run=debye_number,
     parse=lambda q: _pairs(q, r"debye number|number of particles in a debye sphere", {"t": "T", "n": "n"}, phys={"T", "n"}),
     reference=lambda T, n: (lambda u, pf: {"N_D": float(pf.Debye_number(T * u.K, n * u.m ** -3).to(u.one).value)})(*_pf()),
     wrong=_safe(lambda T, n: (lambda o: {**o, "N_D": _r(n * o["lambda_D"] ** 3)})(debye_number(T, n))),
     random_inputs=lambda rng: {"T": _lu(rng, 1e3, 1e9), "n": _lu(rng, 1e4, 1e26)},
     templates=["Debye number: T = 1e6 K, n = 1e15 m^-3"],
     examples=[{"inputs": {"T": QE * QE / (EPS0 * KB), "n": 1.0}, "outputs": {"N_D": 4 / 3 * math.pi}}],
     questions=[{"q": "Debye number: T = 1e6 K, n = 1e15 m^-3", "outputs": {"N_D": 4 / 3 * math.pi * 1e15 * (EPS0 * KB * 1e6 / (1e15 * QE * QE)) ** 1.5}},
                {"q": "Debye number: T = -1 K, n = 1e15 m^-3", "outcome": "INVALID_INPUT"},
                {"q": "Debye number: n = 1e15 m^-3", "outcome": "NEEDS_INPUT"}])


def alfven_speed(B, n):
    _pos("the field and the density", B, n)
    v = B / math.sqrt(MU0 * n * (MP + ME))
    return _out(v_A=v, v_A_km_s=v / 1e3)


_add("alfven_speed",
     name="Alfven speed", topic="plasma physics / waves",
     purpose="The Alfven speed of a fully ionized hydrogen plasma (n protons and n electrons per unit volume) in a magnetic field B.",
     inputs=[_phys("B", "magnetic_field", "magnetic field strength (T, nT, gauss)"), _phys("n", "number_density", "proton number density")],
     outputs=[_o("v_A", "speed", "Alfven speed"), _o("v_A_km_s", "number", "Alfven speed in km/s")],
     primary="v_A", conditions=["B, n > 0", "mass density n (m_p + m_e)", "non-relativistic (v_A << c)"],
     method="v_A = B / sqrt(mu0 n (m_p + m_e))", latex=r"v_A = \frac{B}{\sqrt{\mu_0\,n\,(m_p + m_e)}}",
     run=alfven_speed,
     parse=lambda q: _pairs(q, r"alfv[eé]n speed", {"b": "B", "n": "n"}, phys={"B", "n"}),
     reference=lambda B, n: (lambda u, pf: {"v_A": float(pf.Alfven_speed(B * u.T, n * u.m ** -3, ion="p+").to(u.m / u.s).value)})(*_pf()),
     wrong=_safe(lambda B, n: (lambda v: _out(v_A=v, v_A_km_s=v / 1e3))(B / math.sqrt(MU0 * n * ME))),
     random_inputs=lambda rng: (lambda n: {"n": n, "B": math.sqrt(MU0 * n * MP) * _lu(rng, 1, 1e7)})(_lu(rng, 1e4, 1e24)),
     templates=["Alfven speed: B = 5 nT, n = 5 cm^-3", "Alfven speed: B = 0.01 T, n = 1e18 m^-3"],
     examples=[{"inputs": {"B": math.sqrt(MU0 * (MP + ME)), "n": 1.0}, "outputs": {"v_A": 1.0}}],
     questions=[{"q": "Alfven speed: B = 5 nT, n = 5 cm^-3", "outputs": {"v_A": 5e-9 / math.sqrt(MU0 * 5e6 * (MP + ME))}},
                {"q": "Alfven speed: B = 0 nT, n = 5 cm^-3", "outcome": "INVALID_INPUT"},
                {"q": "Alfven speed: B = 5 nT", "outcome": "NEEDS_INPUT"}])


def gyrofrequency(B, particle="electron"):
    _pos("the field", B)
    w = QE * B / _mass(particle)
    return _out(omega_c=w, f_c=w / (2 * math.pi), T_c=2 * math.pi / w)


_add("gyrofrequency",
     name="Gyrofrequency (cyclotron frequency)", topic="plasma physics / particle motion",
     purpose="The angular and ordinary gyrofrequency and the gyration period of an electron or proton in a magnetic field B.",
     inputs=[_phys("B", "magnetic_field", "magnetic field strength (T, nT, gauss)"), _PART_IN],
     outputs=[_o("omega_c", "angular_frequency", "angular gyrofrequency"), _o("f_c", "frequency", "gyrofrequency"), _o("T_c", "time", "gyration period")],
     primary="f_c", conditions=["B > 0", "non-relativistic particle"],
     method="omega_c = |q| B / m", latex=r"\omega_c = \frac{|q|B}{m}",
     run=gyrofrequency,
     parse=lambda q: _pairs(q, r"gyrofrequency|cyclotron frequency", {"b": "B", "particle": "particle"}, phys={"B"}, words={"particle"}),
     reference=lambda B, particle="electron": (lambda u, pf: {"omega_c": float(pf.gyrofrequency(B * u.T, _PARTICLES[particle][1]).to(u.rad / u.s).value)})(*_pf()),
     wrong=_safe(lambda B, particle="electron": (lambda w: _out(omega_c=w, f_c=w, T_c=1 / w))(QE * B / (2 * math.pi * _mass(particle)))),
     random_inputs=lambda rng: {"B": _lu(rng, 1e-12, 10), **({"particle": rng.choice(["electron", "proton"])} if rng.random() < 0.6 else {})},
     templates=["Gyrofrequency: B = 50000 nT, particle = electron", "Cyclotron frequency: B = 1 T, particle = proton"],
     examples=[{"inputs": {"B": ME / QE}, "outputs": {"omega_c": 1.0}},
               {"inputs": {"B": MP / QE, "particle": "proton"}, "outputs": {"omega_c": 1.0, "T_c": 2 * math.pi}}],
     questions=[{"q": "Gyrofrequency: B = 1 T, particle = proton", "outputs": {"omega_c": QE / MP}},
                {"q": "Gyrofrequency: B = -1 T", "outcome": "INVALID_INPUT"},
                {"q": "Gyrofrequency: particle = proton", "outcome": "NEEDS_INPUT"}])


def gyroradius(B, v_perp, particle="electron"):
    _pos("the field and the perpendicular speed", B, v_perp)
    if v_perp >= 0.1 * C:
        raise ValueError("the speed must be well below c (v < 0.1 c) for the non-relativistic gyroradius")
    r = _mass(particle) * v_perp / (QE * B)
    return _out(r_L=r, r_L_km=r / 1e3, r_L_nm=r * 1e9)


_add("gyroradius",
     name="Gyroradius (Larmor radius)", topic="plasma physics / particle motion",
     purpose="The radius of the circular motion of an electron or proton moving with perpendicular speed v_perp in a magnetic field B.",
     inputs=[_phys("B", "magnetic_field", "magnetic field strength"), _phys("v_perp", "speed", "speed perpendicular to the field"), _PART_IN],
     outputs=[_o("r_L", "length", "gyroradius"), _o("r_L_km", "number", "gyroradius in kilometres"), _o("r_L_nm", "number", "gyroradius in nanometres")],
     primary="r_L", conditions=["B, v_perp > 0", "v_perp < 0.1 c (non-relativistic)"],
     method="r_L = m v_perp / (|q| B)", latex=r"r_L = \frac{m v_\perp}{|q|B}",
     run=gyroradius,
     parse=lambda q: _pairs(q, r"gyroradius|larmor radius", {"b": "B", "v_perp": "v_perp", "vperp": "v_perp", "v": "v_perp", "particle": "particle"},
                            phys={"B", "v_perp"}, words={"particle"}),
     reference=lambda B, v_perp, particle="electron": (lambda u, pf: {"r_L": float(pf.gyroradius(B * u.T, _PARTICLES[particle][1], Vperp=v_perp * u.m / u.s,
                                                                                                    relativistic=False).to(u.m).value), "r_L_nm": 1e9 * float(pf.gyroradius(B * u.T, _PARTICLES[particle][1], Vperp=v_perp * u.m / u.s,
                                                                                                    relativistic=False).to(u.m).value)})(*_pf()),
     wrong=_safe(lambda B, v_perp, particle="electron": (lambda r: _out(r_L=r, r_L_km=r / 1e3, r_L_nm=r * 1e9))(QE * B / (_mass(particle) * v_perp))),
     random_inputs=lambda rng: {"B": _lu(rng, 1e-12, 1), "v_perp": _lu(rng, 1e3, 2.9e7), **({"particle": rng.choice(["electron", "proton"])} if rng.random() < 0.6 else {})},
     templates=["Gyroradius: B = 5 nT, v_perp = 400 km/s, particle = proton", "Larmor radius: B = 0.1 T, v_perp = 1e6 m/s"],
     examples=[{"inputs": {"B": MP / QE, "v_perp": 3.0, "particle": "proton"}, "outputs": {"r_L": 3.0}}],
     questions=[{"q": "Gyroradius: B = 5 nT, v_perp = 400 km/s, particle = proton", "outputs": {"r_L": MP * 4e5 / (QE * 5e-9)}},
                {"q": "Gyroradius: B = 5 nT, v_perp = 2e8 m/s", "outcome": "INVALID_INPUT"},
                {"q": "Gyroradius: B = 5 nT", "outcome": "NEEDS_INPUT"}])


_CONV = {"most_probable": (2.0, "most_probable"), "rms": (3.0, "rms"), "mean": (8 / math.pi, "mean_magnitude")}


def thermal_speed(T, convention, particle="electron"):
    _pos("the temperature", T)
    if convention not in _CONV:
        raise ValueError("the convention must be most_probable, rms or mean")
    return _out(v_th=math.sqrt(_CONV[convention][0] * KB * T / _mass(particle)))


_add("thermal_speed",
     name="Thermal speed of plasma particles", topic="plasma physics / velocities",
     purpose="The thermal speed of electrons or protons of temperature T under a stated convention: most probable sqrt(2kT/m), root-mean-square sqrt(3kT/m) or mean sqrt(8kT/(pi m)).",
     inputs=[_phys("T", "temperature", "temperature"), _choice("convention", ["most_probable", "rms", "mean"], "which thermal speed: most_probable, rms or mean"),
             _PART_IN],
     outputs=[_o("v_th", "speed", "thermal speed")],
     primary="v_th", conditions=["T > 0", "Maxwellian distribution in three dimensions", "the convention must be stated"],
     method="v = sqrt(c k T / m) with c = 2 (most probable), 3 (rms), 8/pi (mean)", latex=r"v_\mathrm{mp} = \sqrt{\frac{2kT}{m}},\ v_\mathrm{rms} = \sqrt{\frac{3kT}{m}},\ \bar v = \sqrt{\frac{8kT}{\pi m}}",
     run=thermal_speed,
     parse=lambda q: _pairs(q, r"thermal speed", {"t": "T", "convention": "convention", "particle": "particle"}, phys={"T"}, words={"convention", "particle"}),
     reference=lambda T, convention, particle="electron": (lambda u, pf: {"v_th": float(pf.thermal_speed(T * u.K, _PARTICLES[particle][1], method=_CONV[convention][1]).to(u.m / u.s).value)})(*_pf()),
     wrong=_safe(lambda T, convention, particle="electron": _out(v_th=math.sqrt(KB * T / _mass(particle)))),
     random_inputs=lambda rng: {"T": _lu(rng, 10, 1e8), "convention": rng.choice(["most_probable", "rms", "mean"]),
                                **({"particle": rng.choice(["electron", "proton"])} if rng.random() < 0.6 else {})},
     templates=["Thermal speed: T = 1e6 K, convention = rms, particle = proton", "Thermal speed: T = 1e4 K, convention = most_probable"],
     examples=[{"inputs": {"T": ME / KB, "convention": "rms"}, "outputs": {"v_th": math.sqrt(3)}},
               {"inputs": {"T": MP / KB, "convention": "most_probable", "particle": "proton"}, "outputs": {"v_th": math.sqrt(2)}}],
     questions=[{"q": "Thermal speed: T = 1e6 K, convention = rms, particle = proton", "outputs": {"v_th": math.sqrt(3 * KB * 1e6 / MP)}},
                {"q": "Thermal speed: T = 1e6 K, convention = fastest", "outcome": "INVALID_INPUT"},
                {"q": "Thermal speed: T = 1e6 K", "outcome": "NEEDS_INPUT"}])


def plasma_beta(T, n, B):
    _pos("the temperature, the density and the field", T, n, B)
    pt, pb = n * KB * T, B * B / (2 * MU0)
    return _out(beta=pt / pb, p_thermal=pt, p_magnetic=pb)


_add("plasma_beta",
     name="Plasma beta", topic="plasma physics / pressures",
     purpose="The ratio of the thermal pressure n k T of one particle species to the magnetic pressure B^2/(2 mu0).",
     inputs=[_phys("T", "temperature", "temperature of the species"), _phys("n", "number_density", "number density of the species"),
             _phys("B", "magnetic_field", "magnetic field strength")],
     outputs=[_o("beta", "number", "plasma beta"), _o("p_thermal", "pressure", "thermal pressure n k T"), _o("p_magnetic", "pressure", "magnetic pressure")],
     primary="beta", conditions=["T, n, B > 0", "one species (add species' pressures for a total beta)"],
     method="beta = n k T / (B^2 / (2 mu0))", latex=r"\beta = \frac{n k_B T}{B^2/(2\mu_0)}",
     run=plasma_beta,
     parse=lambda q: _pairs(q, r"plasma beta", {"t": "T", "n": "n", "b": "B"}, phys={"T", "n", "B"}),
     reference=lambda T, n, B: (lambda u, pf: {"beta": float(pf.beta(T * u.K, n * u.m ** -3, B * u.T).to(u.one).value)})(*_pf()),
     wrong=_safe(lambda T, n, B: (lambda o: {**o, "beta": _r(n * KB * T / (B * B / MU0))})(plasma_beta(T, n, B))),
     random_inputs=lambda rng: (lambda T, n: {"T": T, "n": n, "B": math.sqrt(2 * MU0 * n * KB * T / _lu(rng, 1e-4, 1e4))})(_lu(rng, 100, 1e9), _lu(rng, 1e4, 1e28)),
     templates=["Plasma beta: T = 1e5 K, n = 5 cm^-3, B = 5 nT"],
     examples=[{"inputs": {"T": 1 / KB, "n": 1.0, "B": math.sqrt(2 * MU0)}, "outputs": {"beta": 1.0}}],
     questions=[{"q": "Plasma beta: T = 1e5 K, n = 5 cm^-3, B = 5 nT", "outputs": {"beta": 5e6 * KB * 1e5 / (25e-18 / (2 * MU0))}},
                {"q": "Plasma beta: T = 1e5 K, n = 5 cm^-3, B = 0 nT", "outcome": "INVALID_INPUT"},
                {"q": "Plasma beta: T = 1e5 K, n = 5 cm^-3", "outcome": "NEEDS_INPUT"}])


def magnetic_pressure(B):
    _pos("the field", B)
    p = B * B / (2 * MU0)
    return _out(p_B=p, energy_density=p, p_B_nPa=p * 1e9)


_add("magnetic_pressure",
     name="Magnetic pressure", topic="plasma physics / pressures",
     purpose="The magnetic pressure (equal to the magnetic energy density) of a field B in vacuum or a plasma.",
     inputs=[_phys("B", "magnetic_field", "magnetic field strength (T, nT, gauss)")],
     outputs=[_o("p_B", "pressure", "magnetic pressure"), _o("energy_density", "pressure", "magnetic energy density (J/m^3)"),
              _o("p_B_nPa", "number", "magnetic pressure in nanopascals")],
     primary="p_B", conditions=["B > 0"],
     method="p_B = B^2 / (2 mu0)", latex=r"p_B = \frac{B^2}{2\mu_0}",
     run=magnetic_pressure,
     parse=lambda q: _pairs(q, r"magnetic pressure", {"b": "B"}, phys={"B"}),
     reference=lambda B: (lambda u, pf: {"p_B_nPa": float(pf.magnetic_pressure(B * u.T).to(u.nPa).value)})(*_pf()),
     wrong=_safe(lambda B: _out(p_B=B * B / MU0, energy_density=B * B / MU0, p_B_nPa=B * B / MU0 * 1e9)),
     random_inputs=lambda rng: {"B": _lu(rng, 1e-9, 100)},
     templates=["Magnetic pressure: B = 5 nT", "Magnetic pressure: B = 1 T"],
     examples=[{"inputs": {"B": math.sqrt(2 * MU0)}, "outputs": {"p_B": 1.0}}],
     questions=[{"q": "Magnetic pressure: B = 1 T", "outputs": {"p_B": 1 / (2 * MU0)}},
                {"q": "Magnetic pressure: B = 0 T", "outcome": "INVALID_INPUT"},
                {"q": "Magnetic pressure:", "outcome": "NEEDS_INPUT"}])


def upper_hybrid(B, n):
    _pos("the field and the density", B, n)
    wp2 = n * QE * QE / (EPS0 * ME)
    wc = QE * B / ME
    w = math.sqrt(wp2 + wc * wc)
    return _out(omega_uh=w, f_uh=w / (2 * math.pi))


_add("upper_hybrid_frequency",
     name="Upper hybrid frequency", topic="plasma physics / frequencies",
     purpose="The upper hybrid frequency of a magnetized electron plasma: sqrt(omega_pe^2 + omega_ce^2).",
     inputs=[_phys("B", "magnetic_field", "magnetic field strength"), _phys("n", "number_density", "electron density")],
     outputs=[_o("omega_uh", "angular_frequency", "upper hybrid angular frequency"), _o("f_uh", "frequency", "upper hybrid frequency")],
     primary="f_uh", conditions=["B, n > 0", "cold plasma, waves perpendicular to B"],
     method="omega_uh = sqrt(omega_pe^2 + omega_ce^2)", latex=r"\omega_{uh} = \sqrt{\omega_{pe}^2 + \omega_{ce}^2}",
     run=upper_hybrid,
     parse=lambda q: _pairs(q, r"upper hybrid frequency", {"b": "B", "n": "n"}, phys={"B", "n"}),
     reference=lambda B, n: (lambda u, pf: {"omega_uh": float(pf.upper_hybrid_frequency(B * u.T, n * u.m ** -3).to(u.rad / u.s).value)})(*_pf()),
     wrong=_safe(lambda B, n: (lambda w: _out(omega_uh=w, f_uh=w / (2 * math.pi)))(math.sqrt(n * QE * QE / (EPS0 * ME)) + QE * B / ME)),
     random_inputs=lambda rng: (lambda n: {"n": n, "B": ME / QE * math.sqrt(n * QE * QE / (EPS0 * ME)) * _lu(rng, 0.05, 20)})(_lu(rng, 1e4, 1e28)),
     templates=["Upper hybrid frequency: B = 50000 nT, n = 1e12 m^-3"],
     examples=[{"inputs": {"B": 3 * ME / QE, "n": 16 * EPS0 * ME / QE ** 2}, "outputs": {"omega_uh": 5.0}}],
     questions=[{"q": "Upper hybrid frequency: B = 1 T, n = 1e18 m^-3", "outputs": {"omega_uh": math.sqrt(1e18 * QE ** 2 / (EPS0 * ME) + (QE / ME) ** 2)}},
                {"q": "Upper hybrid frequency: B = 1 T, n = -1e18 m^-3", "outcome": "INVALID_INPUT"},
                {"q": "Upper hybrid frequency: B = 1 T", "outcome": "NEEDS_INPUT"}])


def lower_hybrid(B, n):
    _pos("the field and the density", B, n)
    wci, wce = QE * B / MP, QE * B / ME
    wpi2 = n * QE * QE / (EPS0 * MP)
    w = 1 / math.sqrt(1 / (wci * wce) + 1 / wpi2)
    return _out(omega_lh=w, f_lh=w / (2 * math.pi))


_add("lower_hybrid_frequency",
     name="Lower hybrid frequency", topic="plasma physics / frequencies",
     purpose="The lower hybrid frequency of a magnetized electron-proton plasma.",
     inputs=[_phys("B", "magnetic_field", "magnetic field strength"), _phys("n", "number_density", "density (electrons = protons)")],
     outputs=[_o("omega_lh", "angular_frequency", "lower hybrid angular frequency"), _o("f_lh", "frequency", "lower hybrid frequency")],
     primary="f_lh", conditions=["B, n > 0", "cold quasi-neutral electron-proton plasma"],
     method="1/omega_lh^2 = 1/(omega_ci omega_ce) + 1/omega_pi^2", latex=r"\frac{1}{\omega_{lh}^2} = \frac{1}{\omega_{ci}\,\omega_{ce}} + \frac{1}{\omega_{pi}^2}",
     run=lower_hybrid,
     parse=lambda q: _pairs(q, r"lower hybrid frequency", {"b": "B", "n": "n"}, phys={"B", "n"}),
     reference=lambda B, n: (lambda u, pf: {"omega_lh": float(pf.lower_hybrid_frequency(B * u.T, n * u.m ** -3, ion="p+").to(u.rad / u.s).value)})(*_pf()),
     wrong=_safe(lambda B, n: (lambda w: _out(omega_lh=w, f_lh=w / (2 * math.pi)))(math.sqrt(QE * B / MP * QE * B / ME))),
     random_inputs=lambda rng: (lambda B: {"B": B, "n": (QE * B / MP) * (QE * B / ME) * EPS0 * MP / QE ** 2 * _lu(rng, 0.05, 20)})(_lu(rng, 1e-10, 10)),
     templates=["Lower hybrid frequency: B = 50000 nT, n = 1e12 m^-3"],
     examples=[{"inputs": {"B": 1.0, "n": (QE / MP) * (QE / ME) * EPS0 * MP / QE ** 2}, "outputs": {"omega_lh": math.sqrt(QE * QE / (MP * ME) / 2)}}],
     questions=[{"q": "Lower hybrid frequency: B = 1 T, n = 1e18 m^-3",
                 "outputs": {"omega_lh": 1 / math.sqrt(1 / (QE * QE / (MP * ME)) + 1 / (1e18 * QE * QE / (EPS0 * MP)))}},
                {"q": "Lower hybrid frequency: B = 1 T, n = 0 m^-3", "outcome": "INVALID_INPUT"},
                {"q": "Lower hybrid frequency: B = 1 T", "outcome": "NEEDS_INPUT"}])


def inertial_length(n, particle="electron"):
    _pos("the density", n)
    w = math.sqrt(n * QE * QE / (EPS0 * _mass(particle)))
    return _out(d=C / w, d_km=C / w / 1e3)


_add("inertial_length",
     name="Inertial length (skin depth)", topic="plasma physics / lengths",
     purpose="The electron (or proton) inertial length c / omega_p, the collisionless skin depth of a plasma.",
     inputs=[_phys("n", "number_density", "number density"), _PART_IN],
     outputs=[_o("d", "length", "inertial length"), _o("d_km", "number", "inertial length in kilometres")],
     primary="d", conditions=["n > 0"],
     method="d = c / omega_p = c sqrt(eps0 m / (n e^2))", latex=r"d = \frac{c}{\omega_p}",
     run=inertial_length,
     parse=lambda q: _pairs(q, r"inertial length|skin depth", {"n": "n", "particle": "particle"}, phys={"n"}, words={"particle"}),
     reference=lambda n, particle="electron": (lambda u, pf: {"d": float(pf.inertial_length(n * u.m ** -3, _PARTICLES[particle][1]).to(u.m).value)})(*_pf()),
     wrong=_safe(lambda n, particle="electron": (lambda w: _out(d=C / (w / (2 * math.pi)), d_km=C / (w / (2 * math.pi)) / 1e3))(math.sqrt(n * QE * QE / (EPS0 * _mass(particle))))),
     random_inputs=lambda rng: {"n": _lu(rng, 1e4, 1e20), **({"particle": rng.choice(["electron", "proton"])} if rng.random() < 0.6 else {})},
     templates=["Inertial length: n = 5 cm^-3, particle = proton", "Skin depth: n = 1e18 m^-3"],
     examples=[{"inputs": {"n": EPS0 * ME * C * C / QE ** 2}, "outputs": {"d": 1.0}}],
     questions=[{"q": "Inertial length: n = 5 cm^-3, particle = proton", "outputs": {"d": C / math.sqrt(5e6 * QE * QE / (EPS0 * MP))}},
                {"q": "Inertial length: n = 0 cm^-3", "outcome": "INVALID_INPUT"},
                {"q": "Inertial length:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ gravitational lensing
def einstein_radius(M, D_l, D_s):
    _pos("the mass and the two distances", M, D_l, D_s)
    if D_s <= D_l:
        raise ValueError("the source must lie behind the lens (D_s > D_l)")
    th = math.sqrt(4 * G * M / C ** 2 * (D_s - D_l) / (D_l * D_s))
    return _out(theta_E=th, theta_E_arcsec=th / ARCSEC, R_E=th * D_l, R_E_au=th * D_l / AU)


def ref_einstein_radius(M, D_l, D_s):
    from scipy.integrate import quad
    from scipy.optimize import brentq
    # Newtonian transverse kick on light passing at impact parameter b, integrated along the path (b = 1 m),
    # doubled by general relativity; it scales as 1/b. Then the lens equation 0 = theta - (D_ls/D_s) alpha(theta D_l).
    kick, _ = quad(lambda z: G * M * 1.0 / (1.0 + z * z) ** 1.5, -math.inf, math.inf, epsabs=0, epsrel=1e-13)
    alpha1 = 2 * kick / C ** 2                                  # deflection at b = 1 m
    Dls = D_s - D_l
    x = brentq(lambda lt: lt - math.log(Dls / D_s * alpha1 / (math.exp(lt) * D_l)), -100, 10, xtol=1e-14, rtol=8.9e-16, maxiter=500)
    th = math.exp(x)
    return {"theta_E_arcsec": th / ARCSEC, "R_E": th * D_l}


_add("einstein_radius",
     name="Einstein radius of a point-mass lens", topic="gravitational lensing / point lens",
     purpose="The angular Einstein radius of a point mass M at distance D_l lensing a source at distance D_s (static space: D_ls = D_s - D_l), and the corresponding radius in the lens plane.",
     inputs=[_phys("M", "mass", "lens mass (Msun, kg)"), _phys("D_l", "length", "distance to the lens (pc, kpc)"), _phys("D_s", "length", "distance to the source")],
     outputs=[_o("theta_E", "angle", "Einstein angle"), _o("theta_E_arcsec", "number", "Einstein angle in arcseconds"),
              _o("R_E", "length", "Einstein radius in the lens plane"), _o("R_E_au", "number", "Einstein radius in au")],
     primary="theta_E_arcsec", conditions=["D_s > D_l > 0", "point mass, weak field", "non-cosmological distances (D_ls = D_s - D_l)"],
     method="theta_E = sqrt( (4 G M / c^2) (D_s - D_l) / (D_l D_s) )", latex=r"\theta_E = \sqrt{\frac{4GM}{c^2}\frac{D_{ls}}{D_l D_s}}",
     run=einstein_radius,
     parse=lambda q: _pairs(q, r"einstein radius", {"m": "M", "d_l": "D_l", "dl": "D_l", "d_s": "D_s", "ds": "D_s"}, phys={"M", "D_l", "D_s"}),
     reference=ref_einstein_radius,
     wrong=_safe(lambda M, D_l, D_s: (lambda th: _out(theta_E=th, theta_E_arcsec=th / ARCSEC, R_E=th * D_l, R_E_au=th * D_l / AU))(
         math.sqrt(2 * G * M / C ** 2 * (D_s - D_l) / (D_l * D_s)))),
     random_inputs=lambda rng: (lambda Dl: {"M": _lu(rng, 1e28, 1e43), "D_l": Dl, "D_s": Dl * _lu(rng, 1.05, 100)})(_lu(rng, 1e17, 1e25)),
     templates=["Einstein radius: M = 0.5 Msun, D_l = 4 kpc, D_s = 8 kpc", "Einstein radius: M = 1e12 Msun, D_l = 1 Gpc, D_s = 2 Gpc"],
     examples=[{"inputs": {"M": C ** 2 / (4 * G), "D_l": 1.0, "D_s": 2.0}, "outputs": {"theta_E": math.sqrt(0.5)}}],
     questions=[{"q": "Einstein radius: M = 0.5 Msun, D_l = 4 kpc, D_s = 8 kpc",
                 "outputs": {"theta_E_arcsec": math.sqrt(4 * G * 0.5 * MSUN / C ** 2 * 4e3 * PC / (4e3 * PC * 8e3 * PC)) / ARCSEC}},
                {"q": "Einstein radius: M = 0.5 Msun, D_l = 8 kpc, D_s = 4 kpc", "outcome": "INVALID_INPUT"},
                {"q": "Einstein radius: M = 0.5 Msun, D_l = 4 kpc", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ additions: flux distance, critical density
def distance_from_flux(L, F):
    _pos("the luminosity and the flux", L, F)
    d = math.sqrt(L / (4 * math.pi * F))
    return _out(d=d, d_pc=d / PC, d_ly=d / LY)


def ref_distance_from_flux(L, F):
    import astropy.units as u
    from astropy.coordinates import Distance
    m = float(u.Magnitude(F * u.W / u.m ** 2, unit=u.mag(u.bol)).value)     # apparent bolometric magnitude (zero point f0)
    M = float(u.Magnitude(L * u.W, unit=u.mag(u.Bol)).value)                # absolute bolometric magnitude (zero point L0)
    return {"d_pc": Distance(distmod=(m - M) * u.mag).to(u.pc).value}


_add("distance_from_flux",
     name="Distance from luminosity and flux (inverse-square law)", topic="distances and magnitudes / standard candles",
     purpose="The distance of a source of known luminosity L (a standard candle) from the bolometric flux F measured at the observer.",
     inputs=[_phys("L", "power", "luminosity (W or Lsun)"), _phys("F", "intensity", "measured flux (W/m^2)")],
     outputs=[_o("d", "length", "distance"), _o("d_pc", "number", "distance in parsecs"), _o("d_ly", "number", "distance in light-years")],
     primary="d_pc", conditions=["L, F > 0", "isotropic emission, no extinction"],
     method="F = L / (4 pi d^2): d = sqrt(L / (4 pi F))", latex=r"d = \sqrt{\frac{L}{4\pi F}}",
     run=distance_from_flux,
     parse=lambda q: _pairs(q, r"distance from (?:luminosity and )?flux", {"l": "L", "f": "F"}, phys={"L", "F"}),
     reference=ref_distance_from_flux,
     wrong=_safe(lambda L, F: (lambda d: _out(d=d, d_pc=d / PC, d_ly=d / LY))(L / (4 * math.pi * F))),
     random_inputs=lambda rng: (lambda L: {"L": L, "F": L / (4 * math.pi * (_lu(rng, 1e9, 1e25)) ** 2)})(_lu(rng, 1e20, 1e38)),
     templates=["Distance from flux: L = 1 Lsun, F = 1361 W/m^2", "Distance from luminosity and flux: L = 2.5e30 W, F = 1e-12 W/m^2"],
     examples=[{"inputs": {"L": 4 * math.pi, "F": 1.0}, "outputs": {"d": 1.0}},
               {"inputs": {"L": L0_BOL, "F": F0_BOL / 100}, "outputs": {"d_pc": 100.0}}],
     questions=[{"q": "Distance from flux: L = 1 Lsun, F = 1361 W/m^2", "outputs": {"d": math.sqrt(LSUN / (4 * math.pi * 1361))}},
                {"q": "Distance from flux: L = 1 Lsun, F = 0 W/m^2", "outcome": "INVALID_INPUT"},
                {"q": "Distance from flux: L = 1 Lsun", "outcome": "NEEDS_INPUT"}])


def critical_density(H0, Om, z=0.0):
    h = _cosmo_check(H0, Om, z)
    Hz = h * math.sqrt(Om * (1 + z) ** 3 + 1 - Om)
    rho = 3 * Hz * Hz / (8 * math.pi * G)
    return _out(H_z=Hz * MPC / 1000.0, rho_c=rho, n_H=rho / MP)


_add("critical_density",
     name="Hubble parameter and critical density", topic="cosmology / Friedmann equation",
     purpose="The Hubble parameter H(z) and the critical density 3H^2/(8 pi G) of a flat Lambda-CDM universe at redshift z (z = 0 gives today's values), with the equivalent number of hydrogen atoms per cubic metre.",
     inputs=[_numin("H0", "Hubble constant in km/s/Mpc"), _numin("Om", "matter density parameter Omega_m"),
             _numin("z", "redshift (default 0)", required=False)],
     outputs=[_o("H_z", "number", "Hubble parameter H(z) in km/s/Mpc"), _o("rho_c", "density", "critical density"),
              _o("n_H", "number_density", "critical density in hydrogen-atom (proton) masses per cubic metre")],
     primary="rho_c", conditions=["1 <= H0 <= 1000", "0.01 <= Omega_m <= 1", "0 <= z <= 1100", "flat, no radiation"],
     method="H(z) = H0 sqrt(Om (1+z)^3 + 1 - Om); rho_c = 3 H^2 / (8 pi G)",
     latex=r"H(z) = H_0\sqrt{\Omega_m(1+z)^3 + \Omega_\Lambda},\quad \rho_c = \frac{3H^2}{8\pi G}",
     run=critical_density,
     parse=lambda q: _pairs(q, r"critical density|hubble parameter(?: and critical density)?", _COSMO_NAMES, num_units={"H0": _H0U}),
     reference=lambda H0, Om, z=0.0: (lambda c: {"H_z": float(c.H(z).value), "n_H": float(c.critical_density(z).to("kg/m3").value) / MP})(_astropy_cosmo(H0, Om)),
     wrong=_safe(lambda H0, Om, z=0.0: (lambda o: {**o, "rho_c": _r(o["rho_c"] * 2), "n_H": _r(o["n_H"] * 2)})(critical_density(H0, Om, z))),
     random_inputs=lambda rng: {"H0": rng.uniform(50, 100), "Om": rng.uniform(0.05, 1.0), **({"z": _lu(rng, 0.01, 20)} if rng.random() < 0.6 else {})},
     templates=["Critical density: H0 = 70 km/s/Mpc, Om = 0.3", "Hubble parameter and critical density: H0 = 67.7, Om = 0.31, z = 2"],
     examples=[{"inputs": {"H0": 70.0, "Om": 0.3}, "outputs": {"H_z": 70.0, "rho_c": 3 * (70e3 / MPC) ** 2 / (8 * math.pi * G)}},
               {"inputs": {"H0": 70.0, "Om": 1.0, "z": 3.0}, "outputs": {"H_z": 560.0}}],
     questions=[{"q": "Critical density: H0 = 70 km/s/Mpc, Om = 0.3", "outputs": {"rho_c": 3 * (70e3 / MPC) ** 2 / (8 * math.pi * G)}},
                {"q": "Critical density: H0 = 70, Om = 0.3, z = -2", "outcome": "INVALID_INPUT"},
                {"q": "Critical density: H0 = 70", "outcome": "NEEDS_INPUT"}])


# === END ===
