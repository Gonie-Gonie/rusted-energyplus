//! ZoneTempPredictorCorrector source-order stage contract.

use super::state::ZoneAirTemperatureCoefficients;
use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};

/// EnergyPlus `ZoneTempPredictorCorrector::ManageZoneAirUpdates`.
#[must_use]
pub const fn manage_zone_air_updates_stage() -> EnergyPlusCompatibilityStage {
    EnergyPlusCompatibilityStage {
        kind: ExecutionStageKind::ManageZoneAirUpdates,
        stage_name: "manage-zone-air-updates",
        source_file: "src/EnergyPlus/ZoneTempPredictorCorrector.cc",
        source_routine: "ManageZoneAirUpdates",
    }
}

/// EnergyPlus `ZoneTempPredictorCorrector::ManageZoneAirUpdates` source-order wrapper.
pub(crate) fn manage_zone_air_updates_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Source-order ownership note for current zone-air history state.
pub const ZONE_AIR_HISTORY_OWNER: &str =
    "MAT history and zone-air output timing are owned by ManageZoneAirUpdates.";

/// Source-order note for EnergyPlus `PredictStep` ownership.
pub const ZONE_AIR_PREDICT_STEP_PATH: &str =
    "PredictStep is represented by zone-air coefficient assembly before correction.";

/// EnergyPlus `ZoneTempPredictorCorrector::PredictStep` source-order wrapper.
pub(crate) fn predict_step_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Source-order note for EnergyPlus `CorrectStep` ownership.
pub const ZONE_AIR_CORRECT_STEP_PATH: &str =
    "CorrectStep is represented by analytical or third-order zone-air correction.";

/// EnergyPlus `ZoneTempPredictorCorrector::CorrectStep` source-order wrapper.
pub(crate) fn correct_step_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Source-order note for zone-air history push and revert ownership.
pub const ZONE_AIR_HISTORY_PUSH_REVERT_PATH: &str =
    "History push/revert is owned by ManageZoneAirUpdates and timestep history synchronization.";

/// EnergyPlus `ZoneTempPredictorCorrector::RevertZoneTimestepHistories` source-order wrapper.
pub(crate) fn revert_zone_timestep_histories_source_order_path<T>(
    execute: impl FnOnce() -> T,
) -> T {
    execute()
}

/// EnergyPlus `ZoneTempPredictorCorrector::PushZoneTimestepHistories` source-order wrapper.
pub(crate) fn push_zone_timestep_histories_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// EnergyPlus `ZoneTempPredictorCorrector::PushSystemTimestepHistories` source-order wrapper.
pub(crate) fn push_system_timestep_histories_source_order_path<T>(
    execute: impl FnOnce() -> T,
) -> T {
    execute()
}

pub(crate) fn step_zone_air_temperature(
    current_temperature_c: f64,
    outdoor_temperature_c: f64,
    internal_gain_w: f64,
    conductance_w_per_k: f64,
    heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
) -> f64 {
    if heat_capacity_j_per_k <= 0.0 || timestep_seconds <= 0.0 {
        return current_temperature_c;
    }

    energyplus_analytical_zone_air_temperature_c(
        current_temperature_c,
        internal_gain_w + conductance_w_per_k * outdoor_temperature_c,
        conductance_w_per_k,
        heat_capacity_j_per_k,
        timestep_seconds,
    )
}

/// Builds EnergyPlus zone-air temperature coefficients for an uncontrolled zone.
///
/// This mirrors the coefficient assembly in `correctZoneAirTemps` for the
/// current diagnostic subset:
/// `TempDepCoef = SumHA + SumMCp` and
/// `TempIndCoef = SumIntGain + SumHATsurf - SumHATref + SumMCpT`.
#[must_use]
pub fn energyplus_zone_air_temperature_coefficients(
    sum_ha_w_per_k: f64,
    sum_hat_surf_w: f64,
    sum_hat_ref_w: f64,
    sum_internal_gain_w: f64,
    sum_mcp_w_per_k: f64,
    sum_mcp_t_w: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
) -> ZoneAirTemperatureCoefficients {
    let temp_dependent_coefficient_w_per_k = sum_ha_w_per_k + sum_mcp_w_per_k;
    let temp_independent_coefficient_w =
        sum_internal_gain_w + sum_hat_surf_w - sum_hat_ref_w + sum_mcp_t_w;
    energyplus_zone_air_temperature_coefficients_from_terms(
        temp_dependent_coefficient_w_per_k,
        temp_independent_coefficient_w,
        air_heat_capacity_j_per_k,
        timestep_seconds,
        previous_mean_air_temperatures_c,
    )
}

fn energyplus_zone_air_temperature_coefficients_from_terms(
    temp_dependent_coefficient_w_per_k: f64,
    temp_independent_coefficient_w: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
) -> ZoneAirTemperatureCoefficients {
    let air_power_cap_w_per_k = if air_heat_capacity_j_per_k > 0.0 && timestep_seconds > 0.0 {
        air_heat_capacity_j_per_k / timestep_seconds
    } else {
        0.0
    };
    let third_order_history_term_w = air_power_cap_w_per_k
        * (3.0 * previous_mean_air_temperatures_c[0]
            - (3.0 / 2.0) * previous_mean_air_temperatures_c[1]
            + (1.0 / 3.0) * previous_mean_air_temperatures_c[2]);

    ZoneAirTemperatureCoefficients {
        temp_dependent_coefficient_w_per_k,
        temp_independent_coefficient_w,
        air_power_cap_w_per_k,
        third_order_history_term_w,
        third_order_temp_dependent_load_w_per_k: (11.0 / 6.0) * air_power_cap_w_per_k
            + temp_dependent_coefficient_w_per_k,
        third_order_temp_independent_load_w: third_order_history_term_w
            + temp_independent_coefficient_w,
    }
}

/// EnergyPlus third-order zone-air temperature solution for one timestep.
///
/// This mirrors the `ThirdOrder` branch in `correctZoneAirTemps`:
/// `ZT = (TempIndCoef + TempHistoryTerm) /
///       ((11/6) * AirPowerCap + TempDepCoef)`.
#[must_use]
pub fn energyplus_third_order_zone_air_temperature_c(
    previous_temperature_c: f64,
    temp_independent_coefficient_w: f64,
    temp_dependent_coefficient_w_per_k: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
) -> f64 {
    let coefficients = energyplus_zone_air_temperature_coefficients_from_terms(
        temp_dependent_coefficient_w_per_k,
        temp_independent_coefficient_w,
        air_heat_capacity_j_per_k,
        timestep_seconds,
        previous_mean_air_temperatures_c,
    );
    energyplus_third_order_zone_air_temperature_from_coefficients(
        previous_temperature_c,
        coefficients,
    )
}

pub(crate) fn energyplus_third_order_zone_air_temperature_from_coefficients(
    previous_temperature_c: f64,
    coefficients: ZoneAirTemperatureCoefficients,
) -> f64 {
    let denominator = coefficients.third_order_temp_dependent_load_w_per_k;
    if denominator.abs() <= f64::EPSILON {
        previous_temperature_c
    } else {
        coefficients.third_order_temp_independent_load_w / denominator
    }
}

/// EnergyPlus analytical zone-air temperature solution for one timestep.
///
/// This mirrors the `AnalyticalSolution` branch in
/// `ZoneTempPredictorCorrector.cc`, using `TempIndCoef`, `TempDepCoef`, and
/// `AirPowerCap = C_air / dt`.
#[must_use]
pub fn energyplus_analytical_zone_air_temperature_c(
    previous_temperature_c: f64,
    temp_independent_coefficient_w: f64,
    temp_dependent_coefficient_w_per_k: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
) -> f64 {
    if air_heat_capacity_j_per_k <= 0.0 || timestep_seconds <= 0.0 {
        return previous_temperature_c;
    }

    let air_power_cap_w_per_k = air_heat_capacity_j_per_k / timestep_seconds;
    if temp_dependent_coefficient_w_per_k.abs() <= f64::EPSILON {
        return previous_temperature_c + temp_independent_coefficient_w / air_power_cap_w_per_k;
    }

    let equilibrium_temperature_c =
        temp_independent_coefficient_w / temp_dependent_coefficient_w_per_k;
    let exponent = (-temp_dependent_coefficient_w_per_k / air_power_cap_w_per_k).min(700.0);
    (previous_temperature_c - equilibrium_temperature_c) * exponent.exp()
        + equilibrium_temperature_c
}
