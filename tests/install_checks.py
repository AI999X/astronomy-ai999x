"""astronomy checks run by tools/clean_install.py inside a fresh environment (written by tools/new_domain.py from the certified procedures' documented questions)."""

SAMPLE = ('Absolute magnitude: m = 8, d = 100 pc, A = 1', 2.0)
LATENCY = ["Absolute magnitude: m = 8, d = 100 pc, A = 1", "Age at redshift: z = 3, H0 = 70, Om = 1", "Airmass: zen = 60 deg"]
QUESTIONS = [['Absolute magnitude: m = 8, d = 100 pc, A = 1', {'M': 2.0}], ['Age at redshift: z = 3, H0 = 70, Om = 1', {'age': 3.673425692251627e+16}], ['Airmass: zen = 60 deg', {'X': 2.0}], ['Alfven speed: B = 5 nT, n = 5 cm^-3', {'v_A': 48759.915859664085}], ['Altitude-azimuth from hour angle: H = 0 h, dec = 0 deg, lat = 45 deg', {'alt_deg': 45.0, 'az_deg': 180.0}], ['Angular separation: ra1 = 0 h, dec1 = 0 deg, ra2 = 6 h, dec2 = 0 deg', {'sep_deg': 90.0}], ['Angular size: D = 2 m, d = 1 m', {'theta_deg': 90.0}], ['Angular size at redshift: z = 3, H0 = 70, Om = 1, D = 1 Mpc', {'theta_arcsec': 192.64709370769756}], ['Apparent bolometric magnitude: L = 3.0128e28 W, d = 100 pc', {'m_bol': 5.0}], ['Apsis speeds: rp = 1 m, ra = 3 m, M = 1.4982e10 kg', {'e': 0.5}], ['Scale height: T = 288 K, mu = 28.97 g/mol, g = 9.81 m/s^2', {'H': 8425.761663628737}], ['Binary star masses: P = 1 yr, a1 = 0.25 au, a2 = 0.75 au', {'M_total': 1.9884849805923904e+30, 'M1_Msun': 0.7500283303868003}], ['Absolute bolometric magnitude: L = 1 Lsun', {'M_bol': 4.7399959339194595}], ['Calendar date from Julian date: jd = 2451545.0', {'year': 2000, 'month': 1, 'day': 1}], ['Pixel scale: f = 2000 mm, p = 3.76 um, N = 6248', {'fov_deg': 0.6730022391427005}], ['Combined magnitude: m1 = 5, m2 = 5', {'m': 4.247425010840047}], ['Comoving volume: z = 3, H0 = 70, Om = 1', {'V_Gpc3': 329.04598659952455}], ['Cosmological distances: z = 3, H0 = 70, Om = 1', {'D_L_Mpc': 17130.9976}], ['Critical density: H0 = 70 km/s/Mpc, Om = 0.3', {'rho_c': 9.203873922972522e-27}], ['Culmination altitude: dec = 20 deg, lat = 50 deg', {'alt_upper_deg': 60.0}], ['Debye length: T = 1e5 K, n = 1e18 m^-3', {'lambda_D': 2.1822555802126222e-05}], ['Debye number: T = 1e6 K, n = 1e15 m^-3', {'N_D': 43531681.94033303}], ['Distance from distance modulus: m = 10, M = 0', {'d_pc': 1000.0}], ['Distance from flux: L = 1 Lsun, F = 1361 W/m^2', {'d': 149607019160.41156}], ['Distance conversion: d = 1 pc', {'d_pc': 1.0, 'd_au': 206264.806247096}], ['Effective temperature: L = 1 Lsun, R = 1 Rsun', {'T': 5772.003429098914}], ['Einstein radius: M = 0.5 Msun, D_l = 4 kpc, D_s = 8 kpc', {'theta_E_arcsec': 0.0007134359325094637}], ['Escape delta-v: M = 5.972e24 kg, r = 6678 km', {'dv': 3200.1023494325973}], ['Flux ratio: m1 = 1, m2 = 6', {'ratio': 100.0}], ['Free-fall time: rho = 1e-17 kg/m^3', {'t_ff': 21006694169648.316}], ['Gyrofrequency: B = 1 T, particle = proton', {'omega_c': 95788331.43000986}], ['Gyroradius: B = 5 nT, v_perp = 400 km/s, particle = proton', {'r_L': 835174.7943167171}], ['Hohmann transfer: r1 = 6678 km, r2 = 42164 km, M = 5.972e24 kg', {'t_transfer': 18990.319729358405}], ['Inertial length: n = 5 cm^-3, particle = proton', {'d': 101835.35105854443}], ['Julian date: year = 2000, month = 1, day = 1, hour = 12', {'JD': 2451545.0}], ["Kepler's equation: M = 1.4982e10 kg, a = 1 m, e = 0.5, t = 3.14159265358979 s", {'r': 1.5}], ['Light-gathering power: D1 = 200 mm, D2 = 2 mm', {'area_ratio': 10000.0, 'mag_gain': 10.0}], ['Linear size: theta = 90 deg, d = 1 m', {'D': 2.0}], ['Lookback time: z = 3, H0 = 70, Om = 1', {'t_L': 2.571397984576139e+17}], ['Lower hybrid frequency: B = 1 T, n = 1e18 m^-3', {'omega_lh': 1253639221.454949}]]


def _close(a, b):
    if isinstance(b, list):
        return isinstance(a, (list, tuple)) and len(a) == len(b) and all(_close(x, y) for x, y in zip(a, b))
    try:
        return abs(complex(a) - complex(b)) <= 1e-6 * max(1.0, abs(complex(b)))
    except (TypeError, ValueError):
        return a == b


def checks(P) -> dict:
    out = {"certified_procedures": len(P.pack().procedures) == 67,
           "refuses_nonsense": not P.ask("What is it?").solved}
    for i, (q, want) in enumerate(QUESTIONS):
        a = P.ask(q)
        out[f"question_{i}"] = a.solved and all(_close(a.values.get(k), v) for k, v in want.items())
    return out
