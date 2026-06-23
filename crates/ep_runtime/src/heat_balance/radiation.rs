//! Heat-balance radiation source-order ownership notes.

use crate::time_axis::{DEFAULT_RUN_PERIOD_YEAR, day_of_year};
use crate::weather::EpwRecord;
use ep_model::{AutoOrNumber, SiteLocation, Surface};

/// Current longwave/solar source-order owner for outside-face inputs.
pub const EXTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceOutsideSurf";

/// Current longwave source-order owner for inside-face inputs.
pub const INTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceInsideSurf";

const ENERGYPLUS_SUN_IS_UP_COS_ZENITH: f64 = 0.00001;
const ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS: usize = 20;

pub(crate) fn weighted_solar_value(
    previous: f64,
    current: f64,
    next: f64,
    previous_weight: f64,
    current_weight: f64,
    next_weight: f64,
) -> f64 {
    previous.max(0.0) * previous_weight
        + current.max(0.0) * current_weight
        + next.max(0.0) * next_weight
}

pub(crate) fn solar_weather_interpolation_weights(
    zone_steps_per_hour: u32,
    timestep: u32,
) -> (f64, f64, f64) {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    let current_weight = solar_interpolation_weight(steps, timestep);
    if steps == 1 {
        return (0.0, current_weight, 0.0);
    }
    let timestep_fraction = 1.0 / f64::from(steps);
    if (current_weight - 1.0).abs() <= f64::EPSILON {
        (0.0, current_weight, 0.0)
    } else if f64::from(timestep) * timestep_fraction < 0.5 {
        (1.0 - current_weight, current_weight, 0.0)
    } else {
        (0.0, current_weight, 1.0 - current_weight)
    }
}

fn solar_interpolation_weight(zone_steps_per_hour: u32, timestep: u32) -> f64 {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    if steps.is_multiple_of(2) {
        let halfpoint = steps / 2;
        let distance = timestep.abs_diff(halfpoint);
        return 1.0 - f64::from(distance) / f64::from(steps);
    }

    if steps == 1 {
        0.5
    } else if steps == 3 {
        match timestep {
            1 | 2 => 5.0 / 6.0,
            _ => 0.5,
        }
    } else {
        let timestep_weight = 1.0 / f64::from(steps);
        let halfpoint = steps / 2;
        let peak_weight = 1.0 - timestep_weight / 2.0;
        if timestep == halfpoint || timestep == halfpoint + 1 {
            peak_weight
        } else if timestep > halfpoint + 1 {
            peak_weight - f64::from(timestep - (halfpoint + 1)) * timestep_weight
        } else {
            peak_weight - f64::from(halfpoint - timestep) * timestep_weight
        }
    }
}

pub(crate) fn surface_ground_view_factor(surface: &Surface, tilt_rad: f64) -> f64 {
    match surface.view_factor_to_ground {
        AutoOrNumber::Value(value) => value.clamp(0.0, 1.0),
        AutoOrNumber::AutoCalculate => ((1.0 - tilt_rad.cos()) * 0.5).clamp(0.0, 1.0),
    }
}

pub(crate) fn surface_sky_view_factor(surface: &Surface, tilt_rad: f64) -> f64 {
    match surface.view_factor_to_ground {
        AutoOrNumber::Value(value) => (1.0 - value).clamp(0.0, 1.0),
        AutoOrNumber::AutoCalculate => ((1.0 + tilt_rad.cos()) * 0.5).clamp(0.0, 1.0),
    }
}

pub(crate) fn surface_air_sky_radiation_split(tilt_rad: f64) -> f64 {
    ((1.0 + tilt_rad.cos()) * 0.5)
        .max(0.0)
        .sqrt()
        .clamp(0.0, 1.0)
}

pub(crate) fn solar_position_rad_at_local_hour(
    site: &SiteLocation,
    record: &EpwRecord,
    local_hour: f64,
) -> Option<(f64, f64)> {
    let day = energyplus_weather_record_day_of_year(record)?;
    let (sin_declination, cos_declination, equation_of_time_hours) =
        energyplus_daily_solar_coefficients(day);
    solar_position_rad_from_coefficients(
        site,
        local_hour,
        sin_declination,
        cos_declination,
        equation_of_time_hours,
    )
}

pub(crate) fn solar_position_rad_from_coefficients(
    site: &SiteLocation,
    local_hour: f64,
    sin_declination: f64,
    cos_declination: f64,
    equation_of_time_hours: f64,
) -> Option<(f64, f64)> {
    let latitude_rad = site.latitude_deg.to_radians();
    let sin_latitude = latitude_rad.sin();
    let cos_latitude = latitude_rad.cos();
    let time_zone_meridian_deg = 15.0 * site.time_zone_hours;
    let hour_angle_deg = 15.0 * (12.0 - (local_hour + equation_of_time_hours))
        + (time_zone_meridian_deg - site.longitude_deg);
    let hour_angle_rad = hour_angle_deg.to_radians();

    let cos_zenith =
        sin_declination * sin_latitude + cos_declination * cos_latitude * hour_angle_rad.cos();
    if cos_zenith < ENERGYPLUS_SUN_IS_UP_COS_ZENITH {
        return None;
    }

    let altitude_rad = cos_zenith.clamp(-1.0, 1.0).asin();
    let solar_zenith_rad = cos_zenith.clamp(-1.0, 1.0).acos();
    let azimuth_denominator = cos_latitude * solar_zenith_rad.sin();
    let mut azimuth_rad = if azimuth_denominator.abs() > 1.0e-12 {
        let cos_azimuth = -((sin_latitude * cos_zenith - sin_declination) / azimuth_denominator);
        cos_azimuth.clamp(-1.0, 1.0).acos()
    } else {
        0.0
    };
    if hour_angle_deg < 0.0 {
        azimuth_rad = 2.0 * std::f64::consts::PI - azimuth_rad;
    }

    Some((altitude_rad, azimuth_rad))
}

pub(crate) fn energyplus_shadowing_period_solar_coefficients(
    weather_records: &[EpwRecord],
    record_index: usize,
) -> Option<(f64, f64, f64)> {
    if weather_records.is_empty() {
        return None;
    }

    let total_days = weather_records.len().div_ceil(24);
    let day_of_sim_zero = record_index / 24;
    let period_start_day_zero = (day_of_sim_zero / ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS)
        * ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS;
    let period_length = ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS
        .min(total_days.saturating_sub(period_start_day_zero))
        .max(1);
    let period_start_record = weather_records.get(period_start_day_zero * 24)?;
    let period_start_day_of_year = energyplus_weather_record_day_of_year(period_start_record)?;

    Some(energyplus_average_solar_coefficients(
        period_start_day_of_year,
        period_length,
    ))
}

pub(crate) fn energyplus_average_solar_coefficients(
    start_day_of_year: u32,
    day_count: usize,
) -> (f64, f64, f64) {
    let day_count = day_count.max(1);
    let mut sin_declination_sum = 0.0;
    let mut equation_of_time_sum = 0.0;
    for offset in 0..day_count {
        let (sin_declination, _cos_declination, equation_of_time_hours) =
            energyplus_daily_solar_coefficients(start_day_of_year + offset as u32);
        sin_declination_sum += sin_declination;
        equation_of_time_sum += equation_of_time_hours;
    }

    let sin_declination = sin_declination_sum / day_count as f64;
    let cos_declination = (1.0 - sin_declination.powi(2)).sqrt();
    let equation_of_time_hours = equation_of_time_sum / day_count as f64;

    (sin_declination, cos_declination, equation_of_time_hours)
}

pub(crate) fn energyplus_weather_record_day_of_year(record: &EpwRecord) -> Option<u32> {
    day_of_year(DEFAULT_RUN_PERIOD_YEAR, record.month, record.day)
}

pub(crate) fn energyplus_daily_solar_coefficients(day_of_year: u32) -> (f64, f64, f64) {
    const SINE_SOLAR_DECLINATION_COEFFICIENTS: [f64; 9] = [
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
    const EQUATION_OF_TIME_COEFFICIENTS: [f64; 9] = [
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

    let angle = 2.0 * std::f64::consts::PI * f64::from(day_of_year) / 366.0;
    let sin_x = angle.sin();
    let cos_x = angle.cos();
    let sin_2x = sin_x * cos_x * 2.0;
    let cos_2x = cos_x.powi(2) - sin_x.powi(2);
    let sin_3x = sin_x * cos_2x + cos_x * sin_2x;
    let cos_3x = cos_x * cos_2x - sin_x * sin_2x;
    let sin_4x = 2.0 * sin_2x * cos_2x;
    let cos_4x = cos_2x.powi(2) - sin_2x.powi(2);
    let basis = [
        1.0, sin_x, cos_x, sin_2x, cos_2x, sin_3x, cos_3x, sin_4x, cos_4x,
    ];

    let sin_declination = SINE_SOLAR_DECLINATION_COEFFICIENTS
        .iter()
        .zip(basis)
        .map(|(coefficient, term)| coefficient * term)
        .sum::<f64>();
    let cos_declination = (1.0 - sin_declination.powi(2)).sqrt();
    let equation_of_time_hours = EQUATION_OF_TIME_COEFFICIENTS
        .iter()
        .zip(basis)
        .map(|(coefficient, term)| coefficient * term)
        .sum::<f64>();

    (sin_declination, cos_declination, equation_of_time_hours)
}

pub(crate) fn energyplus_anisotropic_sky_multiplier(
    surface: &Surface,
    site: &SiteLocation,
    tilt_rad: f64,
    solar_altitude_rad: f64,
    direct_normal_w_per_m2: f64,
    diffuse_horizontal_w_per_m2: f64,
    cos_incidence: f64,
    circumsolar_sunlit_fraction: f64,
) -> f64 {
    const EPSILON_LIMIT: [f64; 7] = [1.065, 1.23, 1.5, 1.95, 2.8, 4.5, 6.2];
    const F11R: [f64; 8] = [
        -0.0083117, 0.1299457, 0.3296958, 0.5682053, 0.8730280, 1.1326077, 1.0601591, 0.6777470,
    ];
    const F12R: [f64; 8] = [
        0.5877285, 0.6825954, 0.4868735, 0.1874525, -0.3920403, -1.2367284, -1.5999137, -0.3272588,
    ];
    const F13R: [f64; 8] = [
        -0.0620636, -0.1513752, -0.2210958, -0.2951290, -0.3616149, -0.4118494, -0.3589221,
        -0.2504286,
    ];
    const F21R: [f64; 8] = [
        -0.0596012, -0.0189325, 0.0554140, 0.1088631, 0.2255647, 0.2877813, 0.2642124, 0.1561313,
    ];
    const F22R: [f64; 8] = [
        0.0721249, 0.0659650, -0.0639588, -0.1519229, -0.4620442, -0.8230357, -1.1272340,
        -1.3765031,
    ];
    const F23R: [f64; 8] = [
        -0.0220216, -0.0288748, -0.0260542, -0.0139754, 0.0012448, 0.0558651, 0.1310694, 0.2506212,
    ];

    let diffuse_horizontal = diffuse_horizontal_w_per_m2.max(0.0);
    if diffuse_horizontal <= f64::EPSILON {
        return surface_sky_view_factor(surface, tilt_rad);
    }

    let direct_normal = direct_normal_w_per_m2.max(0.0);
    let cos_zenith = solar_altitude_rad.sin().clamp(0.0, 1.0);
    if cos_zenith < ENERGYPLUS_SUN_IS_UP_COS_ZENITH {
        return surface_sky_view_factor(surface, tilt_rad);
    }

    let zenith_rad = cos_zenith.acos();
    let zenith_deg = zenith_rad.to_degrees();
    let air_mass_height = 1.0 - 0.1 * site.elevation_m / 1000.0;
    let air_mass = if zenith_deg <= 75.0 {
        air_mass_height / cos_zenith
    } else {
        air_mass_height / (cos_zenith + 0.15 * (93.9 - zenith_deg).powf(-1.253))
    };
    let kappa_z3 = 1.041 * zenith_rad.powi(3);
    let epsilon =
        ((direct_normal + diffuse_horizontal) / diffuse_horizontal + kappa_z3) / (1.0 + kappa_z3);
    let delta = diffuse_horizontal * air_mass / 1353.0;
    let epsilon_bin = EPSILON_LIMIT
        .iter()
        .position(|limit| epsilon < *limit)
        .unwrap_or(F11R.len() - 1);
    let f1 =
        (F11R[epsilon_bin] + F12R[epsilon_bin] * delta + F13R[epsilon_bin] * zenith_rad).max(0.0);
    let f2 = F21R[epsilon_bin] + F22R[epsilon_bin] * delta + F23R[epsilon_bin] * zenith_rad;

    let mut circumsolar_factor = cos_incidence.max(0.0) / cos_zenith.max(0.0871557);
    if circumsolar_factor > 0.0 && cos_zenith < 0.0871557 && tilt_rad.to_degrees() < 2.0 {
        circumsolar_factor = 1.0;
    }

    let view_factor_sky = surface_sky_view_factor(surface, tilt_rad);
    let multiplier = view_factor_sky * (1.0 - f1)
        + f1 * circumsolar_factor * circumsolar_sunlit_fraction.clamp(0.0, 1.0)
        + f2 * tilt_rad.sin();
    multiplier.max(0.0)
}
