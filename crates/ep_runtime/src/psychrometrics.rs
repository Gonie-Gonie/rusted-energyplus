//! EnergyPlus psychrometric helper functions used by runtime and IdealLoads.

const KELVIN_OFFSET: f64 = 273.15;
const ENERGYPLUS_MIN_HUMIDITY_RATIO: f64 = 1.0e-5;
const ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE: f64 = 0.0001;
const ENERGYPLUS_WET_BULB_MAX_ITERATIONS: u32 = 100;
const ENERGYPLUS_PSAT_CACHE_PRECISION_BITS: u32 = 24;
const ENERGYPLUS_PSAT_CACHE_GRID_SHIFT: u32 = 64 - 12 - ENERGYPLUS_PSAT_CACHE_PRECISION_BITS;
/// Standard atmospheric pressure used by EnergyPlus psychrometric defaults.
pub const ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA: f64 = 101_325.0;

#[inline]
fn energyplus_humidity_ratio_floor(humidity_ratio: f64) -> f64 {
    // EnergyPlus uses `std::max` in the stateless density overload and its
    // equivalent `ObjexxFCL::max` helper in the stateful/Cp paths. Unlike
    // `f64::max`, both return their first argument when it is NaN.
    if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO {
        ENERGYPLUS_MIN_HUMIDITY_RATIO
    } else {
        humidity_ratio
    }
}

#[inline]
fn energyplus_psy_rho_air_fn_pb_tdb_w_raw(
    atmospheric_pressure_pa: f64,
    dry_bulb_c: f64,
    humidity_ratio: f64,
) -> f64 {
    atmospheric_pressure_pa
        / (287.0 * (dry_bulb_c + KELVIN_OFFSET) * (1.0 + 1.607_768_7 * humidity_ratio))
}

#[inline]
fn energyplus_psy_h_fn_tdb_w_raw(dry_bulb_c: f64, humidity_ratio: f64) -> f64 {
    1.004_84e3 * dry_bulb_c + humidity_ratio * (2.500_94e6 + 1.858_95e3 * dry_bulb_c)
}

#[inline]
fn energyplus_psy_cp_air_fn_w_raw(humidity_ratio: f64) -> f64 {
    1.004_84e3 + humidity_ratio * 1.858_95e3
}

pub(crate) fn energyplus_outdoor_wet_bulb_c(
    dry_bulb_c: f64,
    relative_humidity_percent: f64,
    atmospheric_pressure_pa: f64,
) -> Option<f64> {
    if !dry_bulb_c.is_finite()
        || !relative_humidity_percent.is_finite()
        || !atmospheric_pressure_pa.is_finite()
        || atmospheric_pressure_pa <= 1000.0
    {
        return None;
    }

    let humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        dry_bulb_c,
        (relative_humidity_percent * 0.01).clamp(0.0, 1.0),
        atmospheric_pressure_pa,
    )?;
    let mut wet_bulb_c = dry_bulb_c;
    let mut previous_wet_bulb_c = 0.0;
    let mut previous_error = 0.0;
    for iteration in 1..=ENERGYPLUS_WET_BULB_MAX_ITERATIONS {
        let new_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_wet_bulb_guess(
            dry_bulb_c,
            wet_bulb_c,
            atmospheric_pressure_pa,
        )?;
        let error = humidity_ratio - new_humidity_ratio;
        let (next_wet_bulb_c, converged) = energyplus_general_iterate(
            wet_bulb_c,
            error,
            &mut previous_wet_bulb_c,
            &mut previous_error,
            iteration,
            ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE,
        );
        wet_bulb_c = next_wet_bulb_c;
        if converged {
            break;
        }
    }

    if !wet_bulb_c.is_finite() {
        return None;
    }

    Some(wet_bulb_c.min(dry_bulb_c))
}

fn energyplus_general_iterate(
    current_x: f64,
    current_y: f64,
    previous_x: &mut f64,
    previous_y: &mut f64,
    iteration: u32,
    tolerance: f64,
) -> (f64, bool) {
    const SMALL: f64 = 1.0e-9;
    const PERTURB: f64 = 0.1;

    if iteration != 1 && ((current_x - *previous_x).abs() < tolerance || current_y == 0.0) {
        return (current_x, true);
    }

    let result_x = if iteration == 1 {
        if current_x.abs() > SMALL {
            current_x * (1.0 + PERTURB)
        } else {
            PERTURB
        }
    } else {
        let mut delta_y = current_y - *previous_y;
        if delta_y.abs() < SMALL {
            delta_y = SMALL;
        }
        (current_y * *previous_x - *previous_y * current_x) / delta_y
    };

    *previous_x = current_x;
    *previous_y = current_y;

    (result_x, false)
}

/// Returns EnergyPlus-style zone air heat capacity in J/K.
///
/// This mirrors the moist-air density and specific-heat terms EnergyPlus uses
/// when building zone-air `AirPowerCap`; callers must provide the owning zone
/// humidity ratio.
pub fn energyplus_zone_air_heat_capacity_j_per_k(
    volume_m3: f64,
    atmospheric_pressure_pa: f64,
    dry_bulb_c: f64,
    humidity_ratio: f64,
) -> Option<f64> {
    if !volume_m3.is_finite() || volume_m3 <= 0.0 {
        return None;
    }
    let density_kg_per_m3 = energyplus_moist_air_density_kg_per_m3(
        atmospheric_pressure_pa,
        dry_bulb_c,
        humidity_ratio,
    )?;
    let specific_heat_j_per_kg_k = energyplus_moist_air_specific_heat_j_per_kg_k(humidity_ratio);

    Some(volume_m3 * density_kg_per_m3 * specific_heat_j_per_kg_k)
}

/// Returns EnergyPlus-style zone air heat capacity at standard pressure.
pub fn energyplus_standard_zone_air_heat_capacity_j_per_k(
    volume_m3: f64,
    dry_bulb_c: f64,
    humidity_ratio: f64,
) -> Option<f64> {
    energyplus_zone_air_heat_capacity_j_per_k(
        volume_m3,
        ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA,
        dry_bulb_c,
        humidity_ratio,
    )
}

/// Canonical, stateless EnergyPlus 26.1 `PsyRhoAirFnPbTdbW` calculation.
///
/// This follows the unguarded C++ overload exactly, including its humidity-ratio
/// floor and IEEE-754 propagation for NaN, infinity, signed zero, and invalid
/// physical inputs. The `EP_psych_errors` overload's stateful negative-density
/// fatal branch is a separate, deferred error-reporting boundary.
#[must_use]
#[inline]
pub fn energyplus_psy_rho_air_fn_pb_tdb_w(
    atmospheric_pressure_pa: f64,
    dry_bulb_c: f64,
    humidity_ratio: f64,
) -> f64 {
    energyplus_psy_rho_air_fn_pb_tdb_w_raw(
        atmospheric_pressure_pa,
        dry_bulb_c,
        energyplus_humidity_ratio_floor(humidity_ratio),
    )
}

/// Canonical EnergyPlus 26.1 `PsyRhoAirFnPbTdbW_fast` numerical path.
///
/// The caller must provide `humidity_ratio >= 1.0e-5`. EnergyPlus uses a
/// debug-only assertion for that precondition; its optional negative-density
/// diagnostic and fatal-error path remains a separate, deferred state boundary.
#[must_use]
#[inline]
pub fn energyplus_psy_rho_air_fn_pb_tdb_w_fast(
    atmospheric_pressure_pa: f64,
    dry_bulb_c: f64,
    humidity_ratio: f64,
) -> f64 {
    debug_assert!(humidity_ratio >= ENERGYPLUS_MIN_HUMIDITY_RATIO);
    energyplus_psy_rho_air_fn_pb_tdb_w_raw(atmospheric_pressure_pa, dry_bulb_c, humidity_ratio)
}

/// Canonical EnergyPlus 26.1 `PsyHfgAirFnWTdb` heat of vaporization in J/kg.
///
/// The humidity-ratio argument is intentionally unused by the source routine.
/// The two enthalpy terms remain separate to preserve the source evaluation
/// order and its IEEE-754 behavior at extreme temperatures.
#[must_use]
#[inline]
pub fn energyplus_psy_hfg_air_fn_w_tdb(_humidity_ratio: f64, dry_bulb_c: f64) -> f64 {
    let temperature_c = if dry_bulb_c < 0.0 { 0.0 } else { dry_bulb_c };
    (2_500_940.0 + 1_858.95 * temperature_c) - (4_180.0 * temperature_c)
}

/// Canonical EnergyPlus 26.1 `PsyHgAirFnWTdb` water-vapor gas enthalpy in J/kg.
///
/// The humidity-ratio argument is intentionally unused by the source routine.
#[must_use]
#[inline]
pub fn energyplus_psy_hg_air_fn_w_tdb(_humidity_ratio: f64, dry_bulb_c: f64) -> f64 {
    2_500_940.0 + 1_858.95 * dry_bulb_c
}

/// Canonical EnergyPlus 26.1 `PsyHFnTdbW` moist-air enthalpy in J/kg.
#[must_use]
#[inline]
pub fn energyplus_psy_h_fn_tdb_w(dry_bulb_c: f64, humidity_ratio: f64) -> f64 {
    energyplus_psy_h_fn_tdb_w_raw(dry_bulb_c, energyplus_humidity_ratio_floor(humidity_ratio))
}

/// Canonical EnergyPlus 26.1 `PsyHFnTdbW_fast` numerical path.
///
/// The caller must provide `humidity_ratio >= 1.0e-5`. As in the C++ source,
/// the precondition is checked only when debug assertions are enabled.
#[must_use]
#[inline]
pub fn energyplus_psy_h_fn_tdb_w_fast(dry_bulb_c: f64, humidity_ratio: f64) -> f64 {
    debug_assert!(humidity_ratio >= ENERGYPLUS_MIN_HUMIDITY_RATIO);
    energyplus_psy_h_fn_tdb_w_raw(dry_bulb_c, humidity_ratio)
}

/// Canonical, stateless EnergyPlus 26.1 `PsyCpAirFnW` calculation.
///
/// EnergyPlus wraps this expression in a last-call cache whose physical-domain
/// behavior is output-neutral. This pure function intentionally ports the
/// numerical result without mutable cache or sentinel state; cache accounting
/// and performance parity are separate, deferred work.
#[must_use]
#[inline]
pub fn energyplus_psy_cp_air_fn_w(humidity_ratio: f64) -> f64 {
    energyplus_psy_cp_air_fn_w_raw(energyplus_humidity_ratio_floor(humidity_ratio))
}

/// Canonical EnergyPlus 26.1 `PsyCpAirFnW_fast` numerical path.
///
/// The caller must provide `humidity_ratio >= 1.0e-5`; debug builds assert the
/// precondition before evaluating the pure numerical path.
///
/// EnergyPlus wraps this expression in a function-local last-call cache. This
/// pure function preserves the output-neutral valid-domain calculation while
/// deferring cache identity, hit/miss history, sentinel, and concurrency policy.
#[must_use]
#[inline]
pub fn energyplus_psy_cp_air_fn_w_fast(humidity_ratio: f64) -> f64 {
    debug_assert!(humidity_ratio >= ENERGYPLUS_MIN_HUMIDITY_RATIO);
    energyplus_psy_cp_air_fn_w_raw(humidity_ratio)
}

/// Returns guarded EnergyPlus-style moist-air density in kg/m3.
///
/// This compatibility wrapper retains its pre-existing validation contract and
/// NaN-humidity normalization; use [`energyplus_psy_rho_air_fn_pb_tdb_w`] for
/// the canonical unguarded EnergyPlus numerical semantics.
pub fn energyplus_moist_air_density_kg_per_m3(
    atmospheric_pressure_pa: f64,
    dry_bulb_c: f64,
    humidity_ratio: f64,
) -> Option<f64> {
    if !atmospheric_pressure_pa.is_finite()
        || atmospheric_pressure_pa <= 1000.0
        || !dry_bulb_c.is_finite()
    {
        return None;
    }
    let dry_bulb_k = dry_bulb_c + KELVIN_OFFSET;
    if dry_bulb_k <= 0.0 {
        return None;
    }

    // Preserve this wrapper's historical `f64::max` behavior, which maps NaN
    // humidity to the floor, before delegating to the canonical calculation.
    Some(energyplus_psy_rho_air_fn_pb_tdb_w(
        atmospheric_pressure_pa,
        dry_bulb_c,
        humidity_ratio.max(ENERGYPLUS_MIN_HUMIDITY_RATIO),
    ))
}

/// Returns guarded EnergyPlus-style moist-air specific heat in J/kg-K.
///
/// This compatibility wrapper retains its pre-existing NaN-humidity
/// normalization; use [`energyplus_psy_cp_air_fn_w`] for the canonical
/// EnergyPlus numerical semantics.
pub fn energyplus_moist_air_specific_heat_j_per_kg_k(humidity_ratio: f64) -> f64 {
    energyplus_psy_cp_air_fn_w(humidity_ratio.max(ENERGYPLUS_MIN_HUMIDITY_RATIO))
}

/// Returns EnergyPlus `PsyHgAirFnWTdb` water-vapor gas enthalpy in J/kg.
#[must_use]
pub fn energyplus_water_vapor_gas_enthalpy_j_per_kg(dry_bulb_c: f64) -> f64 {
    energyplus_psy_hg_air_fn_w_tdb(0.0, dry_bulb_c)
}

/// Returns EnergyPlus `PsyWFnTdbRhPb`-style humidity ratio from dry-bulb,
/// relative humidity, and barometric pressure.
pub fn energyplus_psychrometric_humidity_ratio_from_rh(
    dry_bulb_c: f64,
    relative_humidity: f64,
    atmospheric_pressure_pa: f64,
) -> Option<f64> {
    let saturation_pressure_pa = energyplus_psychrometric_saturation_pressure_pa(dry_bulb_c)?;
    let dew_pressure_pa = relative_humidity * saturation_pressure_pa;
    Some(
        (dew_pressure_pa * 0.62198 / (atmospheric_pressure_pa - dew_pressure_pa).max(1000.0))
            .max(ENERGYPLUS_MIN_HUMIDITY_RATIO),
    )
}

fn energyplus_psychrometric_humidity_ratio_from_wet_bulb_guess(
    dry_bulb_c: f64,
    wet_bulb_c: f64,
    atmospheric_pressure_pa: f64,
) -> Option<f64> {
    let saturation_pressure_pa = energyplus_psychrometric_saturation_pressure_pa(wet_bulb_c)?;
    let denominator = atmospheric_pressure_pa - saturation_pressure_pa;
    if denominator <= 0.0 {
        return None;
    }
    let saturated_humidity_ratio = 0.62198 * saturation_pressure_pa / denominator;
    if wet_bulb_c >= 0.0 {
        Some(
            ((2501.0 - 2.326 * wet_bulb_c) * saturated_humidity_ratio
                - 1.006 * (dry_bulb_c - wet_bulb_c))
                / (2501.0 + 1.86 * dry_bulb_c - 4.186 * wet_bulb_c),
        )
    } else {
        Some(
            ((2830.0 - 0.24 * wet_bulb_c) * saturated_humidity_ratio
                - 1.006 * (dry_bulb_c - wet_bulb_c))
                / (2830.0 + 1.86 * dry_bulb_c - 2.1 * wet_bulb_c),
        )
    }
}

fn energyplus_psychrometric_saturation_pressure_pa(temperature_c: f64) -> Option<f64> {
    if !temperature_c.is_finite() {
        return None;
    }
    // EnergyPlus' default PsyPsatFnTemp path keys a cache by truncating the dry-bulb
    // temperature bits before evaluating the raw saturation-pressure polynomial.
    let temperature_c = energyplus_psychrometric_psat_cache_temperature_c(temperature_c);
    let temperature_k = temperature_c + KELVIN_OFFSET;
    if temperature_k < 173.15 {
        return Some(0.001405102123874164);
    }
    if temperature_k < 273.16 {
        return Some(
            (-5674.5359 / temperature_k
                + 6.392_524_7
                + temperature_k
                    * (-0.967_784_3e-2
                        + temperature_k
                            * (0.622_157_01e-6
                                + temperature_k
                                    * (0.207_478_25e-8 - 0.948_402_4e-12 * temperature_k)))
                + 4.163_501_9 * temperature_k.ln())
            .exp(),
        );
    }
    if temperature_k <= 473.15 {
        return Some(
            (-5800.2206 / temperature_k
                + 1.391_499_3
                + temperature_k
                    * (-0.048_640_239
                        + temperature_k * (0.417_647_68e-4 - 0.144_520_93e-7 * temperature_k))
                + 6.545_967_3 * temperature_k.ln())
            .exp(),
        );
    }
    Some(1_555_073.745_636_215)
}

fn energyplus_psychrometric_psat_cache_temperature_c(temperature_c: f64) -> f64 {
    if !temperature_c.is_finite() {
        return temperature_c;
    }
    let mut tag = (temperature_c.to_bits() as i64) >> ENERGYPLUS_PSAT_CACHE_GRID_SHIFT;
    tag <<= ENERGYPLUS_PSAT_CACHE_GRID_SHIFT;
    f64::from_bits(tag as u64)
}

#[cfg(test)]
#[path = "psychrometrics_tests.rs"]
mod tests;
