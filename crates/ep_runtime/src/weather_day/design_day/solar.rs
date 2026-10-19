//! Literal CalculateDailySolarCoeffs4128-4234 and selected sun-z calculation.

#[derive(Clone, Copy)]
pub(in super::super) struct Coefficients {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub annual_variation: f64,
    pub equation_of_time: f64,
    pub sin_declination: f64,
    pub cos_declination: f64,
}

fn series(coefficients: [f64; 9], sine: f64, cosine: f64) -> f64 {
    let sin2 = sine * cosine * 2.0;
    let cos2 = cosine * cosine - sine * sine;
    let terms = [
        sine,
        cosine,
        sin2,
        cos2,
        sine * cos2 + cosine * sin2,
        cosine * cos2 - sine * sin2,
        2.0 * sin2 * cos2,
        cos2 * cos2 - sin2 * sin2,
    ];
    let mut result = coefficients[0];
    for (coefficient, term) in coefficients[1..].iter().zip(terms) {
        result += coefficient * term;
    }
    result
}

pub(super) fn daily_coefficients(day: i32, latitude: f64) -> Coefficients {
    calculate_daily_coefficients(day, latitude, |_, _| {})
}
#[derive(Clone, Copy)]
enum DailySolarField {
    SineDeclination,
    CosineDeclination,
    EquationOfTime,
}
pub(in super::super) fn source_daily_coefficients(
    day: &mut super::super::DailyWeatherVariables,
    latitude: f64,
) -> Coefficients {
    calculate_daily_coefficients(day.day_of_year, latitude, |field, value| match field {
        DailySolarField::SineDeclination => day.sin_solar_declin_angle = value,
        DailySolarField::CosineDeclination => day.cos_solar_declin_angle = value,
        DailySolarField::EquationOfTime => day.equation_of_time = value,
    })
}
fn calculate_daily_coefficients(
    day: i32,
    latitude: f64,
    mut write_field: impl FnMut(DailySolarField, f64),
) -> Coefficients {
    const DECLINATION: [f64; 9] = [
        0.00561800,
        0.0657911,
        -0.392779,
        0.00064440,
        -0.00618495,
        -0.00010101,
        -0.00007951,
        -0.00011691,
        0.00002096,
    ];
    const EQUATION: [f64; 9] = [
        0.00021971,
        -0.122649,
        0.00762856,
        -0.156308,
        -0.0530028,
        -0.00388702,
        -0.00123978,
        -0.00270502,
        -0.00167992,
    ];
    const A: [f64; 9] = [
        1161.6685, 1.1554, 77.3575, -0.5359, -3.7622, 0.9875, -3.3924, -1.7445, 1.1198,
    ];
    const B: [f64; 9] = [
        0.171631,
        -0.00400448,
        -0.0344923,
        0.00000209,
        0.00325428,
        -0.00085429,
        0.00229562,
        0.0009034,
        -0.0011867,
    ];
    const C: [f64; 9] = [
        0.0905151,
        -0.00322522,
        -0.0407966,
        0.000104164,
        0.00745899,
        -0.00086461,
        0.0013111,
        0.000808275,
        -0.00170515,
    ];
    let correction = std::f64::consts::PI * 2.0 / 366.0;
    let mut angle = correction * f64::from(day);
    let mut sine = angle.sin();
    let mut cosine = angle.cos();
    let sin_declination = series(DECLINATION, sine, cosine);
    write_field(DailySolarField::SineDeclination, sin_declination);
    let cos_declination = (1.0 - sin_declination * sin_declination).sqrt();
    write_field(DailySolarField::CosineDeclination, cos_declination);
    let equation_of_time = series(EQUATION, sine, cosine);
    write_field(DailySolarField::EquationOfTime, equation_of_time);
    let annual_variation = 1.000047 + 0.000352615 * sine + 0.0334454 * cosine;
    let a = series(A, sine, cosine);
    if latitude < 0.0 {
        angle -= std::f64::consts::PI;
        sine = angle.sin();
        cosine = angle.cos();
    }
    Coefficients {
        a,
        b: series(B, sine, cosine),
        c: series(C, sine, cosine),
        annual_variation,
        equation_of_time,
        sin_declination,
        cos_declination,
    }
}

pub(super) fn cosine_zenith(
    time: f64,
    coefficients: Coefficients,
    latitude: f64,
    longitude: f64,
    time_zone: f64,
) -> f64 {
    let meridian = if time_zone <= 12.0 {
        time_zone * 15.0
    } else {
        time_zone * 15.0 - 360.0
    };
    let degrees = std::f64::consts::PI / 180.0;
    let hour_angle =
        (15.0 * (12.0 - (time + coefficients.equation_of_time)) + (meridian - longitude)) * degrees;
    let latitude = degrees * latitude;
    coefficients.sin_declination * latitude.sin()
        + coefficients.cos_declination * latitude.cos() * hour_angle.cos()
}

pub(in super::super) fn clear_sky_radiation(
    cosine_zenith: f64,
    sky_clear: f64,
    coefficients: Coefficients,
) -> (f64, f64) {
    // DataEnvironment.hh SunIsUpValue; no substituted horizon threshold.
    if cosine_zenith < 0.00001 {
        return (0.0, 0.0);
    }
    let exponent = coefficients.b / cosine_zenith;
    let total = if exponent > 700.0 {
        0.0
    } else {
        sky_clear
            * coefficients.a
            * (coefficients.c + cosine_zenith)
            * (-coefficients.b / cosine_zenith).exp()
    };
    let extraterrestrial = 1367.0 * coefficients.annual_variation * cosine_zenith;
    let mut kt = total / extraterrestrial;
    // ObjexxFCL::min(a,b) is a<b?a:b, including NaN and equal zeros.
    kt = if kt < 0.75 { kt } else { 0.75 };
    let mut diffuse = total * (1.0045 + kt * (0.04349 + kt * (-3.5227 + 2.6313 * kt)));
    if sky_clear > 0.70 {
        diffuse = total * coefficients.c / (coefficients.c + cosine_zenith);
    }
    let mut beam = (total - diffuse) / cosine_zenith;
    // Literal max(0.0,value), whose first operand wins an unordered/tied pair.
    diffuse = if 0.0 < diffuse { diffuse } else { 0.0 };
    beam = if 0.0 < beam { beam } else { 0.0 };
    (beam, diffuse)
}

// Narrow cfg-only transport to the actual full Source daily function.
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
impl crate::weather::day::WeatherDayState {
    pub(crate) fn native_gap_daily_solar_coefficients(&mut self, day: i32) -> [f64; 7] {
        self.today_variables.day_of_year = day;
        let v = source_daily_coefficients(&mut self.today_variables, self.environment.latitude);
        [v.a, v.b, v.c, v.annual_variation, v.equation_of_time, v.sin_declination, v.cos_declination]
    }
}
