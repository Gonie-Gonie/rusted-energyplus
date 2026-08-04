//! EnergyPlus psychrometric helper functions used by runtime and IdealLoads.

#[path = "psychrometrics_spline_tables.rs"]
mod spline_tables;

use spline_tables::{
    ENERGYPLUS_TSAT_FN_PB_D2Y, ENERGYPLUS_TSAT_FN_PB_Y, ENERGYPLUS_TSAT_SPLINE_SAMPLE_COUNT,
};

const KELVIN_OFFSET: f64 = 273.15;
const ENERGYPLUS_MIN_HUMIDITY_RATIO: f64 = 1.0e-5;
const ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE: f64 = 0.0001;
const ENERGYPLUS_WET_BULB_MAX_ITERATIONS: u32 = 100;
const ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS: u32 = 50;
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

#[inline]
fn energyplus_psy_rhov_fn_tdb_w_pb_raw(
    dry_bulb_c: f64,
    humidity_ratio: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    humidity_ratio * atmospheric_pressure_pa
        / (461.52 * (dry_bulb_c + KELVIN_OFFSET) * (humidity_ratio + 0.621_98))
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

/// Canonical EnergyPlus 26.1 `PsyTdbFnHW` dry-bulb inversion in Celsius.
#[must_use]
#[inline]
pub fn energyplus_psy_tdb_fn_h_w(enthalpy_j_per_kg: f64, humidity_ratio: f64) -> f64 {
    let humidity_ratio = energyplus_humidity_ratio_floor(humidity_ratio);
    (enthalpy_j_per_kg - 2.500_94e6 * humidity_ratio) / (1.004_84e3 + 1.858_95e3 * humidity_ratio)
}

/// Canonical EnergyPlus 26.1 `PsyRhovFnTdbRhLBnd0C` vapor density in kg/m3.
///
/// Despite the historical source name, this routine does not clamp dry-bulb
/// temperature to 0 C; the source exponential is evaluated with the supplied
/// temperature and relative humidity without validation.
#[must_use]
#[inline]
pub fn energyplus_psy_rhov_fn_tdb_rh_lbnd0c(dry_bulb_c: f64, relative_humidity: f64) -> f64 {
    relative_humidity / (461.52 * (dry_bulb_c + KELVIN_OFFSET))
        * (23.709_3 - 4_111.0 / ((dry_bulb_c + KELVIN_OFFSET) - 35.45)).exp()
}

/// Canonical EnergyPlus 26.1 `PsyRhovFnTdbWPb` vapor density in kg/m3.
#[must_use]
#[inline]
pub fn energyplus_psy_rhov_fn_tdb_w_pb(
    dry_bulb_c: f64,
    humidity_ratio: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    energyplus_psy_rhov_fn_tdb_w_pb_raw(
        dry_bulb_c,
        energyplus_humidity_ratio_floor(humidity_ratio),
        atmospheric_pressure_pa,
    )
}

/// Canonical EnergyPlus 26.1 `PsyRhovFnTdbWPb_fast` numerical path.
///
/// The caller must provide `humidity_ratio >= 1.0e-5`. As in the C++ source,
/// the precondition is checked only when debug assertions are enabled.
#[must_use]
#[inline]
pub fn energyplus_psy_rhov_fn_tdb_w_pb_fast(
    dry_bulb_c: f64,
    humidity_ratio: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    debug_assert!(humidity_ratio >= ENERGYPLUS_MIN_HUMIDITY_RATIO);
    energyplus_psy_rhov_fn_tdb_w_pb_raw(dry_bulb_c, humidity_ratio, atmospheric_pressure_pa)
}

/// Canonical EnergyPlus 26.1 `PsyRhFnTdbRhovLBnd0C` numerical path.
///
/// This preserves the source's pre-formula positive-vapor test and its
/// out-of-range-only `0.01..=1.0` correction. The optional `EP_psych_stats`
/// counter and `EP_psych_errors` warmup/recurring-warning state are separate
/// stateful source contracts and are not represented by this pure helper.
#[must_use]
#[inline]
pub fn energyplus_psy_rh_fn_tdb_rhov_lbnd0c(dry_bulb_c: f64, vapor_density_kg_per_m3: f64) -> f64 {
    let relative_humidity = if vapor_density_kg_per_m3 > 0.0 {
        vapor_density_kg_per_m3
            * 461.52
            * (dry_bulb_c + KELVIN_OFFSET)
            * (-23.709_3 + 4_111.0 / ((dry_bulb_c + KELVIN_OFFSET) - 35.45)).exp()
    } else {
        0.0
    };

    if relative_humidity < 0.0 {
        0.01
    } else if relative_humidity > 1.0 {
        1.0
    } else {
        relative_humidity
    }
}

/// Canonical EnergyPlus 26.1 `PsyVFnTdbWPb` numerical path in m3/kg.
///
/// This preserves the source humidity-ratio floor, arithmetic grouping, and
/// unconditional `0.83` fallback for every negative calculated volume. The
/// optional `EP_psych_stats` counter and `EP_psych_errors` warmup/recurring
/// warning state are separate stateful contracts and are not represented by
/// this pure helper.
#[must_use]
#[inline]
pub fn energyplus_psy_v_fn_tdb_w_pb(
    dry_bulb_c: f64,
    humidity_ratio: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let humidity_ratio = energyplus_humidity_ratio_floor(humidity_ratio);
    let specific_volume = 1.594_73e2 * (1.0 + 1.6078 * humidity_ratio) * (1.8 * dry_bulb_c + 492.0)
        / atmospheric_pressure_pa;

    if specific_volume < 0.0 {
        0.83
    } else {
        specific_volume
    }
}

/// Canonical EnergyPlus 26.1 `PsyWFnTdbH` numerical path.
///
/// The raw inverse is returned unchanged unless it is strictly negative, in
/// which case EnergyPlus returns literal `1.0e-5`. This ordered comparison
/// preserves source NaN and negative-zero behavior. Optional statistics and
/// the `CalledFrom`/`SuppressWarnings` diagnostic state are not represented by
/// this pure helper.
#[must_use]
#[inline]
pub fn energyplus_psy_w_fn_tdb_h(dry_bulb_c: f64, enthalpy_j_per_kg: f64) -> f64 {
    let humidity_ratio =
        (enthalpy_j_per_kg - 1.004_84e3 * dry_bulb_c) / (2.500_94e6 + 1.858_95e3 * dry_bulb_c);

    if humidity_ratio < 0.0 {
        ENERGYPLUS_MIN_HUMIDITY_RATIO
    } else {
        humidity_ratio
    }
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

/// Canonical EnergyPlus 26.1 default non-IF97 `PsyPsatFnTemp_raw` numerical path.
///
/// The source range clamps and ice/liquid-water expressions are preserved.
/// Optional statistics, out-of-range diagnostics, and the alternate `EP_IF97`
/// compile branch are outside this pure numerical scaffold.
#[must_use]
#[inline]
pub fn energyplus_psy_psat_fn_temp_raw(temperature_c: f64) -> f64 {
    let temperature_k = temperature_c + KELVIN_OFFSET;
    if temperature_k < 173.15 {
        return 0.001405102123874164;
    }
    if temperature_k < 273.16 {
        return (-5674.5359 / temperature_k
            + 6.392_524_7
            + temperature_k
                * (-0.967_784_3e-2
                    + temperature_k
                        * (0.622_157_01e-6
                            + temperature_k
                                * (0.207_478_25e-8 - 0.948_402_4e-12 * temperature_k)))
            + 4.163_501_9 * temperature_k.ln())
        .exp();
    }
    if temperature_k <= 473.15 {
        return (-5800.2206 / temperature_k
            + 1.391_499_3
            + temperature_k
                * (-0.048_640_239
                    + temperature_k * (0.417_647_68e-4 - 0.144_520_93e-7 * temperature_k))
            + 6.545_967_3 * temperature_k.ln())
        .exp();
    }
    1_555_073.745_636_215
}

#[inline]
fn energyplus_psy_psat_fn_temp_default_numerical_projection(temperature_c: f64) -> f64 {
    energyplus_psy_psat_fn_temp_raw(energyplus_psychrometric_psat_cache_temperature_c(
        temperature_c,
    ))
}

/// Canonical EnergyPlus 26.1 `PsyRhovFnTdbRh` default-build numerical path.
///
/// The source calls the default `PsyPsatFnTemp` cache before applying the
/// ideal-gas expression. This pure scaffold preserves the cache's representative
/// temperature and source arithmetic order while deferring cache lifecycle,
/// statistics, diagnostics, and the history-dependent sentinel edge.
#[must_use]
#[inline]
pub fn energyplus_psy_rhov_fn_tdb_rh(dry_bulb_c: f64, relative_humidity: f64) -> f64 {
    (energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c) * relative_humidity)
        / (461.52 * (dry_bulb_c + KELVIN_OFFSET))
}

/// Canonical EnergyPlus 26.1 `PsyRhFnTdbRhov` default-build numerical path.
///
/// Nonpositive and NaN vapor density bypass saturation pressure and return
/// positive zero. Raw relative humidity is corrected only when it lies outside
/// `0.0..=1.0`: negative values become `0.01` and values above one become
/// `1.0`. Optional statistics, diagnostics, and saturation-pressure cache state
/// remain separate stateful source contracts.
#[must_use]
#[inline]
pub fn energyplus_psy_rh_fn_tdb_rhov(dry_bulb_c: f64, vapor_density_kg_per_m3: f64) -> f64 {
    let relative_humidity = if vapor_density_kg_per_m3 > 0.0 {
        vapor_density_kg_per_m3 * 461.52 * (dry_bulb_c + KELVIN_OFFSET)
            / energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c)
    } else {
        0.0
    };

    if relative_humidity < 0.0 {
        0.01
    } else if relative_humidity > 1.0 {
        1.0
    } else {
        relative_humidity
    }
}

/// Canonical EnergyPlus 26.1 `PsyRhFnTdbWPb` ordinary-finite default-build
/// numerical path.
///
/// Saturation pressure is evaluated before applying the humidity-ratio floor,
/// and the degree-of-saturation expression retains the source grouping. This
/// pure scaffold uses the default cache's representative temperature while
/// deferring cache lifecycle, statistics, diagnostics, compile variants, and
/// the history-dependent negative-NaN sentinel edge.
#[must_use]
#[inline]
pub fn energyplus_psy_rh_fn_tdb_w_pb(
    dry_bulb_c: f64,
    humidity_ratio: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let saturation_pressure_pa =
        energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c);
    let humidity_ratio = energyplus_humidity_ratio_floor(humidity_ratio);
    let degree_of_saturation = humidity_ratio
        / (0.621_98 * saturation_pressure_pa / (atmospheric_pressure_pa - saturation_pressure_pa));
    let relative_humidity = degree_of_saturation
        / (1.0 - (1.0 - degree_of_saturation) * (saturation_pressure_pa / atmospheric_pressure_pa));

    if relative_humidity < 0.0 {
        0.01
    } else if relative_humidity > 1.0 {
        1.0
    } else {
        relative_humidity
    }
}

/// Canonical EnergyPlus 26.1 `PsyWFnTdbRhPb` ordinary-finite default-build
/// numerical path.
///
/// The saturation-pressure call remains unconditional, and both the 1000 Pa
/// denominator floor and 1e-5 humidity-ratio floor use the source's ordered
/// comparisons. This preserves first-argument NaN propagation instead of
/// adopting Rust's `f64::max` NaN behavior. Cache lifecycle, statistics,
/// diagnostics, compile variants, and the history-dependent cache-sentinel
/// edge remain separate stateful source contracts.
#[must_use]
#[inline]
pub fn energyplus_psy_w_fn_tdb_rh_pb(
    dry_bulb_c: f64,
    relative_humidity: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let dew_pressure_pa =
        relative_humidity * energyplus_psy_psat_fn_temp_default_numerical_projection(dry_bulb_c);
    let pressure_difference_pa = atmospheric_pressure_pa - dew_pressure_pa;
    let denominator_pa = if pressure_difference_pa < 1000.0 {
        1000.0
    } else {
        pressure_difference_pa
    };
    let humidity_ratio = dew_pressure_pa * 0.621_98 / denominator_pa;

    if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO {
        ENERGYPLUS_MIN_HUMIDITY_RATIO
    } else {
        humidity_ratio
    }
}

/// Canonical EnergyPlus 26.1 `PsyWFnTdbTwbPb` ordinary-finite default-build
/// numerical path.
///
/// This preserves the source's ordered wet-bulb clamp, saturation-pressure
/// evaluation, coefficient grouping, and strictly-negative fallback through
/// `PsyWFnTdbRhPb` at 0.01% relative humidity. Cache lifecycle, statistics,
/// both diagnostic helpers, compile variants, and history-dependent cache
/// sentinel behavior remain separate stateful source contracts.
#[must_use]
#[inline]
pub fn energyplus_psy_w_fn_tdb_twb_pb(
    dry_bulb_c: f64,
    wet_bulb_c: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let wet_bulb_c = if wet_bulb_c > dry_bulb_c {
        dry_bulb_c
    } else {
        wet_bulb_c
    };
    let wet_saturation_pressure_pa =
        energyplus_psy_psat_fn_temp_default_numerical_projection(wet_bulb_c);
    let saturated_humidity_ratio = 0.621_98 * wet_saturation_pressure_pa
        / (atmospheric_pressure_pa - wet_saturation_pressure_pa);
    let humidity_ratio = ((2501.0 - 2.381 * wet_bulb_c) * saturated_humidity_ratio
        - (dry_bulb_c - wet_bulb_c))
        / (2501.0 + 1.805 * dry_bulb_c - 4.186 * wet_bulb_c);

    if humidity_ratio < 0.0 {
        energyplus_psy_w_fn_tdb_rh_pb(dry_bulb_c, 0.0001, atmospheric_pressure_pa)
    } else {
        humidity_ratio
    }
}

/// Canonical EnergyPlus 26.1 `PsyHFnTdbRhPb` ordinary-finite default-build
/// numerical path in J/kg.
///
/// The source first calls `PsyWFnTdbRhPb`, applies a second ordered 1e-5
/// humidity-ratio floor, and then calls `PsyHFnTdbW`, whose own floor remains
/// intact. This pure composition preserves that order while deferring nested
/// saturation-pressure cache, statistics, diagnostics, caller, and compile
/// variant state.
#[must_use]
#[inline]
pub fn energyplus_psy_h_fn_tdb_rh_pb(
    dry_bulb_c: f64,
    relative_humidity: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let humidity_ratio = energyplus_humidity_ratio_floor(energyplus_psy_w_fn_tdb_rh_pb(
        dry_bulb_c,
        relative_humidity,
        atmospheric_pressure_pa,
    ));
    energyplus_psy_h_fn_tdb_w(dry_bulb_c, humidity_ratio)
}

/// EnergyPlus 26.1 `PsyTsatFnHPb_raw` default-build numerical miss path.
///
/// This retains the source's signed enthalpy floor, binary-searched nine-piece
/// polynomial seed, pressure-correction predicate, and bounded secant update
/// order. The public two-input cache, statistics, diagnostics, caller context,
/// and history-dependent nested cache effects remain outside this pure core.
#[must_use]
pub fn energyplus_psy_tsat_fn_h_pb_raw(enthalpy_j_per_kg: f64, barometric_pressure_pa: f64) -> f64 {
    const CASE_RANGE: [f64; 10] = [
        -4.24e4, -2.2138e4, -6.7012e2, 2.7297e4, 7.5222e4, 1.8379e5, 4.7577e5, 1.5445e6, 3.8353e6,
        4.5866e7,
    ];

    let mut shifted_enthalpy = enthalpy_j_per_kg + 1.78637e4;
    let local_enthalpy = if enthalpy_j_per_kg >= 0.0 {
        if enthalpy_j_per_kg < 1.0e-5 {
            1.0e-5
        } else {
            enthalpy_j_per_kg
        }
    } else if -1.0e-5 < enthalpy_j_per_kg {
        -1.0e-5
    } else {
        enthalpy_j_per_kg
    };

    let mut begin = 0usize;
    let mut end = 9usize;
    while begin + 1 < end {
        let middle = (begin + end) >> 1;
        if shifted_enthalpy > CASE_RANGE[middle] {
            begin = middle;
        } else {
            end = middle;
        }
    }
    let case_index = begin + 1;
    let seed = match case_index {
        1 => {
            if shifted_enthalpy < -4.24e4 {
                shifted_enthalpy = -4.24e4;
            }
            energyplus_f6(
                shifted_enthalpy,
                -19.44,
                8.53675e-4,
                -5.12637e-9,
                -9.85546e-14,
                -1.00102e-18,
                -4.2705e-24,
            )
        }
        2 => energyplus_f6(
            shifted_enthalpy,
            -1.94224e1,
            8.5892e-4,
            -4.50709e-9,
            -6.19492e-14,
            8.71734e-20,
            8.73051e-24,
        ),
        3 => energyplus_f6(
            shifted_enthalpy,
            -1.94224e1,
            8.59061e-4,
            -4.4875e-9,
            -5.76696e-14,
            7.72217e-19,
            3.97894e-24,
        ),
        4 => energyplus_f6(
            shifted_enthalpy,
            -2.01147e1,
            9.04936e-4,
            -6.83305e-9,
            2.3261e-14,
            7.27237e-20,
            -6.31939e-25,
        ),
        5 => energyplus_f6(
            shifted_enthalpy,
            -1.82124e1,
            8.31683e-4,
            -6.16461e-9,
            3.06411e-14,
            -8.60964e-20,
            1.03003e-25,
        ),
        6 => energyplus_f6(
            shifted_enthalpy,
            -1.29419,
            3.88538e-4,
            -1.30237e-9,
            2.78254e-15,
            -3.27225e-21,
            1.60969e-27,
        ),
        7 => energyplus_f6(
            shifted_enthalpy,
            2.39214e1,
            1.27519e-4,
            -1.52089e-10,
            1.1043e-16,
            -4.33919e-23,
            7.05296e-30,
        ),
        8 => energyplus_f6(
            shifted_enthalpy,
            4.88446e1,
            3.85534e-5,
            -1.78805e-11,
            4.87224e-18,
            -7.15283e-25,
            4.36246e-32,
        ),
        _ => {
            if shifted_enthalpy > 4.5866e7 {
                shifted_enthalpy = 4.5866e7;
            }
            energyplus_f7(
                shifted_enthalpy,
                7.60565e11,
                5.80534e4,
                -7.36433e-3,
                5.11531e-10,
                -1.93619e-17,
                3.70511e-25,
                -2.77313e-33,
            )
        }
    };

    let pressure_correction_required = (barometric_pressure_pa - 1.0133e5).abs() / 1.0133e5 > 0.01;
    if !pressure_correction_required {
        return seed;
    }

    let mut first_temperature_c = seed;
    let first_enthalpy = energyplus_psy_h_fn_tdb_w(
        first_temperature_c,
        energyplus_psy_w_fn_tdb_twb_pb(
            first_temperature_c,
            first_temperature_c,
            barometric_pressure_pa,
        ),
    );
    let mut first_error = first_enthalpy - local_enthalpy;
    if (first_error / local_enthalpy).abs() <= 0.1e-4 {
        return first_temperature_c;
    }

    let mut second_temperature_c = first_temperature_c * 0.9;
    let mut result = seed;
    let mut iteration_count = 0;
    while iteration_count <= 30 {
        iteration_count += 1;
        let second_enthalpy = energyplus_psy_h_fn_tdb_w(
            second_temperature_c,
            energyplus_psy_w_fn_tdb_twb_pb(
                second_temperature_c,
                second_temperature_c,
                barometric_pressure_pa,
            ),
        );
        let second_error = second_enthalpy - local_enthalpy;
        if (second_error / local_enthalpy).abs() <= 0.1e-4 || second_error == first_error {
            result = second_temperature_c;
            break;
        }

        let next_temperature_c = second_temperature_c
            - second_error / (second_error - first_error)
                * (second_temperature_c - first_temperature_c);
        first_temperature_c = second_temperature_c;
        second_temperature_c = next_temperature_c;
        first_error = second_error;
    }
    result
}

/// Canonical EnergyPlus 26.1 default cached-build `PsyTsatFnPb_raw`
/// non-interpolation numerical path in Celsius.
///
/// This preserves the ordered pressure bounds, strict triple-point shortcut,
/// 100 C initial guess, nested default `PsyPsatFnTemp` representative, and
/// the source's 50-iteration `General::Iterate` sequence. The source routine's
/// saved-value sentinel and last-call shortcut, interpolation override,
/// statistics, diagnostics, cache lifecycle, and nested nonfinite sentinel
/// behavior remain separate state contracts.
#[must_use]
pub fn energyplus_psy_tsat_fn_pb_raw(pressure_pa: f64) -> f64 {
    if pressure_pa >= 1_555_000.0 {
        return 200.0;
    }
    if pressure_pa <= 0.0017 {
        return -100.0;
    }
    if pressure_pa > 611.0 && pressure_pa < 611.25 {
        return 0.0;
    }

    let mut saturation_temperature_c = 100.0;
    let mut previous_temperature_c = 0.0;
    let mut previous_error_pa = 0.0;
    for iteration in 1..=ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS {
        let saturation_pressure_pa =
            energyplus_psy_psat_fn_temp_default_numerical_projection(saturation_temperature_c);
        let error_pa = pressure_pa - saturation_pressure_pa;
        let (next_temperature_c, converged) = energyplus_general_iterate(
            saturation_temperature_c,
            error_pa,
            &mut previous_temperature_c,
            &mut previous_error_pa,
            iteration,
            ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE,
        );
        saturation_temperature_c = next_temperature_c;
        if converged {
            break;
        }
    }
    saturation_temperature_c
}

/// Canonical EnergyPlus 26.1 `PsyTdpFnWPb` default cached-build,
/// interpolation-disabled numerical miss projection in Celsius.
///
/// This preserves the ordered 1e-5 humidity-ratio floor and the source's
/// multiply/add/divide grouping before calling the isolated `PsyTsatFnPb_raw`
/// numerical core. It models a nonzero-tag outer-cache miss and a raw
/// saved-value miss. The public saturation-temperature cache's tag-zero false
/// hit, first-writer and collision history, raw saved pair, interpolation,
/// statistics, diagnostics, lifecycle, and compile variants remain separate
/// state contracts.
#[must_use]
#[inline]
pub fn energyplus_psy_tdp_fn_w_pb(humidity_ratio: f64, atmospheric_pressure_pa: f64) -> f64 {
    let humidity_ratio = energyplus_humidity_ratio_floor(humidity_ratio);
    let dew_pressure_pa = atmospheric_pressure_pa * humidity_ratio / (0.621_98 + humidity_ratio);
    energyplus_psy_tsat_fn_pb_raw(dew_pressure_pa)
}

/// Canonical EnergyPlus 26.1 `PsyTdpFnTdbTwbPb` ordinary-finite
/// default-build numerical path in Celsius.
///
/// This preserves the source composition through `PsyWFnTdbTwbPb`, its
/// second ordered 1e-5 humidity-ratio floor, `PsyTdpFnWPb`, and the final
/// ordered clamp to the original wet-bulb temperature. Statistics, warnings,
/// recurring diagnostics, nested cache history, interpolation, lifecycle,
/// and compile variants remain separate stateful source contracts.
#[must_use]
#[inline]
pub fn energyplus_psy_tdp_fn_tdb_twb_pb(
    dry_bulb_c: f64,
    wet_bulb_c: f64,
    atmospheric_pressure_pa: f64,
) -> f64 {
    let humidity_ratio = energyplus_humidity_ratio_floor(energyplus_psy_w_fn_tdb_twb_pb(
        dry_bulb_c,
        wet_bulb_c,
        atmospheric_pressure_pa,
    ));
    let dew_point_c = energyplus_psy_tdp_fn_w_pb(humidity_ratio, atmospheric_pressure_pa);

    if dew_point_c > wet_bulb_c {
        wet_bulb_c
    } else {
        dew_point_c
    }
}

/// Canonical EnergyPlus 26.1 `F6` fifth-degree Horner polynomial.
///
/// The nested multiply/add order is part of the source contract; this helper
/// intentionally does not expand, reassociate, or fuse the expression.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn energyplus_f6(x: f64, a0: f64, a1: f64, a2: f64, a3: f64, a4: f64, a5: f64) -> f64 {
    a0 + x * (a1 + x * (a2 + x * (a3 + x * (a4 + x * a5))))
}

/// Canonical EnergyPlus 26.1 `F7` sixth-degree scaled Horner polynomial.
///
/// EnergyPlus completes the nested polynomial before the final `1.0E10`
/// division. Scaling coefficients early would change rounding and overflow.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn energyplus_f7(x: f64, a0: f64, a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, a6: f64) -> f64 {
    (a0 + x * (a1 + x * (a2 + x * (a3 + x * (a4 + x * (a5 + x * a6)))))) / 1.0e10
}

/// Canonical EnergyPlus 26.1 `CPCW` chilled-water specific heat in J/(kg K).
///
/// The source intentionally ignores its temperature argument.
#[must_use]
#[inline]
pub const fn energyplus_cpcw(_temperature_c: f64) -> f64 {
    4_180.0
}

/// Canonical EnergyPlus 26.1 `CPHW` hot-water specific heat in J/(kg K).
///
/// The source intentionally ignores its temperature argument.
#[must_use]
#[inline]
pub const fn energyplus_cphw(_temperature_c: f64) -> f64 {
    4_180.0
}

/// Canonical EnergyPlus 26.1 `RhoH2O` liquid-water density in kg/m3.
///
/// The source evaluates separate square and cube powers before combining the
/// polynomial from left to right. It documents a 0 C through 150 C range but
/// applies no runtime clamp or validation.
#[must_use]
#[inline]
pub fn energyplus_rho_h2o(temperature_c: f64) -> f64 {
    let temperature_squared = temperature_c * temperature_c;
    let temperature_cubed = temperature_c * temperature_c * temperature_c;
    1_000.120_7 + 8.321_587_4e-4 * temperature_c - 4.929_976e-3 * temperature_squared
        + 8.479_186_3e-6 * temperature_cubed
}

/// Canonical EnergyPlus 26.1 `PsyDeltaHSenFnTdb2Tdb1W` sensible
/// moist-air enthalpy difference in J/kg.
///
/// The source passes literal `1.0e-5` as the first argument to its ordered
/// maximum. Consequently an unordered NaN humidity ratio selects the literal
/// floor instead of propagating NaN.
#[must_use]
#[inline]
pub fn energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(
    dry_bulb_2_c: f64,
    dry_bulb_1_c: f64,
    humidity_ratio: f64,
) -> f64 {
    let humidity_ratio = if ENERGYPLUS_MIN_HUMIDITY_RATIO < humidity_ratio {
        humidity_ratio
    } else {
        ENERGYPLUS_MIN_HUMIDITY_RATIO
    };
    (1.004_84e3 + humidity_ratio * 1.858_95e3) * (dry_bulb_2_c - dry_bulb_1_c)
}

/// Canonical EnergyPlus 26.1 `PsyDeltaHSenFnTdb2W2Tdb1W1` sensible
/// moist-air enthalpy difference in J/kg.
///
/// The ordered source minimum returns its second argument when the comparison
/// is unordered, then delegates unchanged to `PsyDeltaHSenFnTdb2Tdb1W`.
#[must_use]
#[inline]
pub fn energyplus_psy_delta_h_sen_fn_tdb2_w2_tdb1_w1(
    dry_bulb_2_c: f64,
    humidity_ratio_2: f64,
    dry_bulb_1_c: f64,
    humidity_ratio_1: f64,
) -> f64 {
    let minimum_humidity_ratio = if humidity_ratio_1 < humidity_ratio_2 {
        humidity_ratio_1
    } else {
        humidity_ratio_2
    };
    energyplus_psy_delta_h_sen_fn_tdb2_tdb1_w(dry_bulb_2_c, dry_bulb_1_c, minimum_humidity_ratio)
}

/// Canonical EnergyPlus 26.1 `CSplineint` fixed-table cubic spline.
///
/// This is not a general spline API: `sample_data_size` selects a prefix of
/// EnergyPlus's immutable 1,651-point pressure tables. The source's shifted
/// 64-Pa bin, truncation toward zero, endpoint extrapolation, and decimal
/// `0.1666666667` factor are preserved exactly.
///
/// EnergyPlus has undefined behavior for a sample count outside 2..=1651 or
/// for an `x` whose C++ `double`-to-`int` conversion is undefined. This safe
/// port asserts that supported domain.
#[must_use]
pub fn energyplus_c_splineint(sample_data_size: i32, x: f64) -> f64 {
    assert!(
        (2..=ENERGYPLUS_TSAT_SPLINE_SAMPLE_COUNT as i32).contains(&sample_data_size),
        "EnergyPlus CSplineint sample_data_size must be in 2..={ENERGYPLUS_TSAT_SPLINE_SAMPLE_COUNT}"
    );
    assert!(x.is_finite(), "EnergyPlus CSplineint x must be finite");
    let truncated_x = x.trunc();
    assert!(
        truncated_x >= f64::from(i32::MIN) && truncated_x <= f64::from(i32::MAX),
        "EnergyPlus CSplineint truncated x must fit in a C++ int"
    );

    let x_int = truncated_x as i32;
    let mut sample_index = (x_int >> 6) - 1;
    if sample_index < 0 {
        sample_index = 0;
    }
    if sample_index > sample_data_size - 2 {
        sample_index = sample_data_size - 2;
    }

    let sample_index = sample_index as usize;
    let h = 64.0;
    let upper_sample_x = 64 * (sample_index as i32 + 1);
    let a = (f64::from(upper_sample_x) - x) / h;
    let b = 1.0 - a;
    a * ENERGYPLUS_TSAT_FN_PB_Y[sample_index]
        + b * ENERGYPLUS_TSAT_FN_PB_Y[sample_index + 1]
        + ((a * a * a - a) * ENERGYPLUS_TSAT_FN_PB_D2Y[sample_index]
            + (b * b * b - b) * ENERGYPLUS_TSAT_FN_PB_D2Y[sample_index + 1])
            * (h * h)
            * 0.166_666_666_7
}

fn energyplus_psychrometric_saturation_pressure_pa(temperature_c: f64) -> Option<f64> {
    if !temperature_c.is_finite() {
        return None;
    }
    Some(energyplus_psy_psat_fn_temp_default_numerical_projection(
        temperature_c,
    ))
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

#[cfg(test)]
#[path = "psychrometrics_inverse_density_tests.rs"]
mod inverse_density_tests;

#[cfg(test)]
#[path = "psychrometrics_relative_humidity_tests.rs"]
mod relative_humidity_tests;

#[cfg(test)]
#[path = "psychrometrics_specific_volume_tests.rs"]
mod specific_volume_tests;

#[cfg(test)]
#[path = "psychrometrics_humidity_ratio_tests.rs"]
mod humidity_ratio_tests;

#[cfg(test)]
#[path = "psychrometrics_saturation_pressure_tests.rs"]
mod saturation_pressure_tests;

#[cfg(test)]
#[path = "psychrometrics_vapor_density_relative_humidity_tests.rs"]
mod vapor_density_relative_humidity_tests;

#[cfg(test)]
#[path = "psychrometrics_humidity_ratio_relative_humidity_tests.rs"]
mod humidity_ratio_relative_humidity_tests;

#[cfg(test)]
#[path = "psychrometrics_relative_humidity_humidity_ratio_tests.rs"]
mod relative_humidity_humidity_ratio_tests;

#[cfg(test)]
#[path = "psychrometrics_wet_bulb_humidity_ratio_tests.rs"]
mod wet_bulb_humidity_ratio_tests;

#[cfg(test)]
#[path = "psychrometrics_relative_humidity_enthalpy_tests.rs"]
mod relative_humidity_enthalpy_tests;

#[cfg(test)]
#[path = "psychrometrics_saturation_temperature_pressure_tests.rs"]
mod saturation_temperature_pressure_tests;

#[cfg(test)]
#[path = "psychrometrics_saturation_temperature_enthalpy_pressure_tests.rs"]
mod saturation_temperature_enthalpy_pressure_tests;

#[cfg(test)]
#[path = "psychrometrics_dew_point_humidity_ratio_tests.rs"]
mod dew_point_humidity_ratio_tests;

#[cfg(test)]
#[path = "psychrometrics_dew_point_dry_wet_bulb_tests.rs"]
mod dew_point_dry_wet_bulb_tests;

#[cfg(test)]
#[path = "psychrometrics_polynomial_water_tests.rs"]
mod polynomial_water_tests;

#[cfg(test)]
#[path = "psychrometrics_water_density_sensible_enthalpy_tests.rs"]
mod water_density_sensible_enthalpy_tests;

#[cfg(test)]
#[path = "psychrometrics_spline_tests.rs"]
mod spline_tests;
