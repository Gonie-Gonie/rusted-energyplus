//! Bounded `calcPredictedSystemLoad` arithmetic for the source-order port.

use crate::heat_balance::state::ZoneHeatBalanceState;
use crate::zone_equipment::ZoneSysEnergyDemand;
use ep_model::ZoneId;

/// Predictor-time inputs for the bounded direct-Zone ThirdOrder load-term assembly.
///
/// EnergyPlus prediction deliberately excludes `SumSysMCp` and `SumSysMCpT`,
/// but includes `SysDepZoneLoadsLagged` in `TempIndCoef`. This DTO exposes no
/// dedicated system-air-sum fields, but its plain scalars cannot prove provenance.
/// Callers must separately prove predictor timing plus a fully mixed, non-AFN
/// direct Zone because the source AFN branch replaces these terms.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PredictorThirdOrderLoadTermsInput {
    /// EnergyPlus `SumHA` in W/K.
    pub sum_ha_w_per_k: f64,
    /// EnergyPlus non-system `SumMCp` in W/K.
    pub sum_mcp_w_per_k: f64,
    /// EnergyPlus `SumIntGain` in W.
    pub sum_internal_gain_w: f64,
    /// EnergyPlus `SumHATsurf` in W.
    pub sum_hat_surf_w: f64,
    /// EnergyPlus `SumHATref` in W.
    pub sum_hat_ref_w: f64,
    /// EnergyPlus non-system `SumMCpT` in W.
    pub sum_mcp_t_w: f64,
    /// EnergyPlus predictor-time `SysDepZoneLoadsLagged` in W.
    pub system_dependent_zone_loads_lagged_w: f64,
    /// Moist-zone-air heat capacity before division by the system timestep in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// Caller-supplied system timestep in seconds; the bounded contract requires positive.
    pub timestep_seconds: f64,
    /// Caller-supplied `ZTM[0..=2]` temperature history in degrees Celsius.
    pub previous_mean_air_temperatures_c: [f64; 3],
}

/// Immutable EnergyPlus ThirdOrder predictor coefficients and final load terms.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PredictorThirdOrderLoadTerms {
    /// EnergyPlus `TempDepCoef = SumHA + SumMCp` in W/K.
    pub temp_dependent_coefficient_w_per_k: f64,
    /// EnergyPlus `TempIndCoef`, including the lagged system-dependent load, in W.
    pub temp_independent_coefficient_w: f64,
    /// EnergyPlus `AirPowerCap = C_air / dt` in W/K.
    pub air_power_cap_w_per_k: f64,
    /// EnergyPlus ThirdOrder `TempHistoryTerm` in W.
    pub temperature_history_term_w: f64,
    /// Final EnergyPlus predictor `tempDepLoad` in W/K.
    pub temp_dependent_load_w_per_k: f64,
    /// Final EnergyPlus predictor `tempIndLoad` in W.
    pub temp_independent_load_w: f64,
}

/// Fail-closed rejection for bounded predictor-term inputs and arithmetic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PredictorThirdOrderLoadTermsError {
    /// An input read by the bounded source slice is NaN or infinite.
    InputNotFinite {
        /// Stable input field name.
        field: &'static str,
    },
    /// The upstream system-timestep invariant was not positive.
    TimestepSecondsNotPositive {
        /// Rejected finite timestep.
        value: f64,
    },
    /// Finite inputs overflowed while assembling one source-ordered term.
    ResultNotFinite {
        /// Stable computed field name.
        field: &'static str,
    },
}

fn require_predictor_term_finite(
    value: f64,
    field: &'static str,
) -> Result<(), PredictorThirdOrderLoadTermsError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(PredictorThirdOrderLoadTermsError::InputNotFinite { field })
    }
}

fn require_predictor_term_finite_result(
    value: f64,
    field: &'static str,
) -> Result<f64, PredictorThirdOrderLoadTermsError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(PredictorThirdOrderLoadTermsError::ResultNotFinite { field })
    }
}

/// Assembles the bounded direct-Zone ThirdOrder predictor load terms.
///
/// This mirrors the source assignment order around `calcZoneOrSpaceSums`:
/// `AirPowerCap` first, non-system dependent/independent coefficients next, the
/// three-slot history term, then `tempDepLoad` and `tempIndLoad`. The caller
/// must prove a fully mixed, non-AFN direct-Zone topology. This pure prerequisite
/// does not sample a thermostat, mutate histories, or call zone equipment.
pub fn assemble_predictor_third_order_load_terms(
    input: PredictorThirdOrderLoadTermsInput,
) -> Result<PredictorThirdOrderLoadTerms, PredictorThirdOrderLoadTermsError> {
    require_predictor_term_finite(input.air_heat_capacity_j_per_k, "air_heat_capacity_j_per_k")?;
    require_predictor_term_finite(input.timestep_seconds, "timestep_seconds")?;
    if input.timestep_seconds <= 0.0 {
        return Err(
            PredictorThirdOrderLoadTermsError::TimestepSecondsNotPositive {
                value: input.timestep_seconds,
            },
        );
    }
    let air_power_cap_w_per_k = require_predictor_term_finite_result(
        input.air_heat_capacity_j_per_k / input.timestep_seconds,
        "air_power_cap_w_per_k",
    )?;

    require_predictor_term_finite(input.sum_ha_w_per_k, "sum_ha_w_per_k")?;
    require_predictor_term_finite(input.sum_mcp_w_per_k, "sum_mcp_w_per_k")?;
    require_predictor_term_finite(input.sum_internal_gain_w, "sum_internal_gain_w")?;
    require_predictor_term_finite(input.sum_hat_surf_w, "sum_hat_surf_w")?;
    require_predictor_term_finite(input.sum_hat_ref_w, "sum_hat_ref_w")?;
    require_predictor_term_finite(input.sum_mcp_t_w, "sum_mcp_t_w")?;
    require_predictor_term_finite(
        input.system_dependent_zone_loads_lagged_w,
        "system_dependent_zone_loads_lagged_w",
    )?;
    let temp_dependent_coefficient_w_per_k = require_predictor_term_finite_result(
        input.sum_ha_w_per_k + input.sum_mcp_w_per_k,
        "temp_dependent_coefficient_w_per_k",
    )?;
    let temp_independent_coefficient_w = require_predictor_term_finite_result(
        input.sum_internal_gain_w + input.sum_hat_surf_w - input.sum_hat_ref_w
            + input.sum_mcp_t_w
            + input.system_dependent_zone_loads_lagged_w,
        "temp_independent_coefficient_w",
    )?;

    for (index, temperature_c) in input
        .previous_mean_air_temperatures_c
        .iter()
        .copied()
        .enumerate()
    {
        let field = match index {
            0 => "previous_mean_air_temperatures_c[0]",
            1 => "previous_mean_air_temperatures_c[1]",
            _ => "previous_mean_air_temperatures_c[2]",
        };
        require_predictor_term_finite(temperature_c, field)?;
    }
    let temperature_history_term_w = require_predictor_term_finite_result(
        air_power_cap_w_per_k
            * (3.0 * input.previous_mean_air_temperatures_c[0]
                - (3.0 / 2.0) * input.previous_mean_air_temperatures_c[1]
                + (1.0 / 3.0) * input.previous_mean_air_temperatures_c[2]),
        "temperature_history_term_w",
    )?;
    let temp_dependent_load_w_per_k = require_predictor_term_finite_result(
        (11.0 / 6.0) * air_power_cap_w_per_k + temp_dependent_coefficient_w_per_k,
        "temp_dependent_load_w_per_k",
    )?;
    let temp_independent_load_w = require_predictor_term_finite_result(
        temperature_history_term_w + temp_independent_coefficient_w,
        "temp_independent_load_w",
    )?;

    Ok(PredictorThirdOrderLoadTerms {
        temp_dependent_coefficient_w_per_k,
        temp_independent_coefficient_w,
        air_power_cap_w_per_k,
        temperature_history_term_w,
        temp_dependent_load_w_per_k,
        temp_independent_load_w,
    })
}
/// Supported operating mode selected from the dual-setpoint loads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredictedZoneSensibleLoadMode {
    /// Both setpoint loads are positive, so the heating-setpoint load is active.
    Heating,
    /// Both setpoint loads are negative, so the cooling-setpoint load is active.
    Cooling,
    /// Heating is nonpositive and cooling is nonnegative, including zero boundaries.
    Deadband,
}

/// Predictor-timing inputs for the bounded direct-Zone, DualSetpoint, ThirdOrder slice.
///
/// `temp_dependent_load_w_per_k` and `temp_independent_load_w` must be the
/// predictor-side `tempDepLoad` and `tempIndLoad` values. They are deliberately
/// not accepted through the current Rust correction coefficient snapshot,
/// whose system-sum and lagged-load timing is not yet source-equivalent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DualSetpointThirdOrderSystemLoadInput {
    /// Typed identity retained across the pure calculation.
    pub zone: ZoneId,
    /// Predictor-side EnergyPlus `tempDepLoad` in W/K.
    pub temp_dependent_load_w_per_k: f64,
    /// Predictor-side EnergyPlus `tempIndLoad` in W.
    pub temp_independent_load_w: f64,
    /// Active low thermostat setpoint in degrees Celsius.
    pub heating_setpoint_c: f64,
    /// Active high thermostat setpoint in degrees Celsius.
    pub cooling_setpoint_c: f64,
    /// Current direct-Zone system-node temperature, read only in deadband.
    pub zone_air_temperature_c: f64,
    /// EnergyPlus Zone load-correction factor, inclusive from -3 through 3.
    pub load_correction_factor: f64,
    /// Positive integer EnergyPlus Zone multiplier within the source signed-int range.
    pub zone_multiplier: u32,
    /// Positive integer EnergyPlus ZoneList multiplier within the source signed-int range.
    pub zone_list_multiplier: u32,
}

/// Immutable output of the bounded predicted sensible-load calculation.
///
/// The raw loads preserve the three values owned by `calcPredictedSystemLoad`.
/// The predicted and output-required values preserve the subsequent correction
/// and Zone/List multiplier order. This snapshot does not mutate a node,
/// thermostat, history, or `ZoneSysEnergyDemand` record.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PredictedZoneSensibleLoads {
    /// Typed identity retained from the input.
    pub zone: ZoneId,
    /// Sign-branch outcome selected before scaling.
    pub mode: PredictedZoneSensibleLoadMode,
    /// Unscaled active total sensible load in W.
    pub raw_total_load_w: f64,
    /// Unscaled load to the heating setpoint in W.
    pub raw_heating_setpoint_load_w: f64,
    /// Unscaled load to the cooling setpoint in W.
    pub raw_cooling_setpoint_load_w: f64,
    /// Active total load after the load-correction factor in W.
    pub predicted_rate_w: f64,
    /// Heating-setpoint load after the load-correction factor in W.
    pub predicted_heating_setpoint_rate_w: f64,
    /// Cooling-setpoint load after the load-correction factor in W.
    pub predicted_cooling_setpoint_rate_w: f64,
    /// Active total load after Zone and ZoneList multipliers in W.
    pub total_output_required_w: f64,
    /// Heating-setpoint load after Zone and ZoneList multipliers in W.
    pub output_required_to_heating_setpoint_w: f64,
    /// Cooling-setpoint load after Zone and ZoneList multipliers in W.
    pub output_required_to_cooling_setpoint_w: f64,
    /// Source-selected node thermostat setpoint in degrees Celsius.
    pub selected_zone_setpoint_c: f64,
    /// Whether the source sign branch selected deadband/setback operation.
    pub deadband_or_setback: bool,
}

/// Fail-closed rejection for inputs or arithmetic outside the bounded slice.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PredictedSystemLoadError {
    /// A conditionally used bounded input is NaN or infinite.
    NonFiniteInput {
        /// Stable input field name.
        field: &'static str,
    },
    /// A typed Zone or ZoneList multiplier is zero.
    ZeroMultiplier {
        /// Stable multiplier field name.
        field: &'static str,
    },
    /// A typed multiplier lies outside EnergyPlus's signed-integer range.
    MultiplierOutsideSourceIntegerRange {
        /// Stable multiplier field name.
        field: &'static str,
        /// Rejected unsigned typed value.
        value: u32,
    },
    /// The signed-integer Zone and ZoneList multiplier product overflowed.
    MultiplierProductOverflow {
        /// Zone multiplier supplied by the typed model.
        zone_multiplier: u32,
        /// ZoneList multiplier supplied by the typed model.
        zone_list_multiplier: u32,
    },
    /// A finite bounded input lies outside its inclusive source range.
    InputOutsideInclusiveRange {
        /// Stable input field name.
        field: &'static str,
        /// Rejected finite value.
        value: f64,
        /// Inclusive lower bound.
        minimum: f64,
        /// Inclusive upper bound.
        maximum: f64,
    },
    /// The effective heating load exceeds the effective cooling load.
    HeatingLoadExceedsCoolingLoad {
        /// Computed heating-setpoint load in W.
        heating_load_w: f64,
        /// Computed cooling-setpoint load in W.
        cooling_load_w: f64,
    },

    /// Finite inputs overflowed while producing a load or scale.
    NonFiniteResult {
        /// Stable computed field name.
        field: &'static str,
    },
}

fn require_finite(value: f64, field: &'static str) -> Result<(), PredictedSystemLoadError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(PredictedSystemLoadError::NonFiniteInput { field })
    }
}

fn require_inclusive_range(
    value: f64,
    field: &'static str,
    minimum: f64,
    maximum: f64,
) -> Result<(), PredictedSystemLoadError> {
    require_finite(value, field)?;
    if value < minimum || value > maximum {
        Err(PredictedSystemLoadError::InputOutsideInclusiveRange {
            field,
            value,
            minimum,
            maximum,
        })
    } else {
        Ok(())
    }
}

fn require_finite_result(value: f64, field: &'static str) -> Result<f64, PredictedSystemLoadError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(PredictedSystemLoadError::NonFiniteResult { field })
    }
}

/// Calculates the bounded direct-Zone, DualSetpoint, ThirdOrder predicted load.
///
/// This mirrors the EnergyPlus 26.1 branch equations and strict sign tests:
/// positive/positive selects heating, negative/negative selects cooling, and
/// nonpositive/nonnegative selects deadband including both zero boundaries.
/// The scope fixes RAFN fraction to one and excludes Space, ITE-adjusted return
/// temperature, staged control, non-ThirdOrder algorithms, and state mutation.
pub fn calc_predicted_system_load_dual_setpoint_third_order(
    input: DualSetpointThirdOrderSystemLoadInput,
) -> Result<PredictedZoneSensibleLoads, PredictedSystemLoadError> {
    require_finite(
        input.temp_dependent_load_w_per_k,
        "temp_dependent_load_w_per_k",
    )?;
    require_finite(input.temp_independent_load_w, "temp_independent_load_w")?;
    require_finite(input.heating_setpoint_c, "heating_setpoint_c")?;
    require_finite(input.cooling_setpoint_c, "cooling_setpoint_c")?;

    let heating_load_w = require_finite_result(
        input.temp_dependent_load_w_per_k * input.heating_setpoint_c
            - input.temp_independent_load_w,
        "raw_heating_setpoint_load_w",
    )?;
    let cooling_load_w = require_finite_result(
        input.temp_dependent_load_w_per_k * input.cooling_setpoint_c
            - input.temp_independent_load_w,
        "raw_cooling_setpoint_load_w",
    )?;

    if heating_load_w > cooling_load_w {
        return Err(PredictedSystemLoadError::HeatingLoadExceedsCoolingLoad {
            heating_load_w,
            cooling_load_w,
        });
    }

    let (total_load_w, selected_zone_setpoint_c, deadband_or_setback, mode) =
        if heating_load_w > 0.0 && cooling_load_w > 0.0 {
            (
                heating_load_w,
                input.heating_setpoint_c,
                false,
                PredictedZoneSensibleLoadMode::Heating,
            )
        } else if heating_load_w < 0.0 && cooling_load_w < 0.0 {
            (
                cooling_load_w,
                input.cooling_setpoint_c,
                false,
                PredictedZoneSensibleLoadMode::Cooling,
            )
        } else {
            require_finite(input.zone_air_temperature_c, "zone_air_temperature_c")?;
            (
                0.0,
                input
                    .zone_air_temperature_c
                    .max(input.heating_setpoint_c)
                    .min(input.cooling_setpoint_c),
                true,
                PredictedZoneSensibleLoadMode::Deadband,
            )
        };

    require_inclusive_range(
        input.load_correction_factor,
        "load_correction_factor",
        -3.0,
        3.0,
    )?;

    let predicted_rate_w = require_finite_result(
        total_load_w * input.load_correction_factor,
        "predicted_rate_w",
    )?;
    let predicted_heating_setpoint_rate_w = require_finite_result(
        heating_load_w * input.load_correction_factor,
        "predicted_heating_setpoint_rate_w",
    )?;
    let predicted_cooling_setpoint_rate_w = require_finite_result(
        cooling_load_w * input.load_correction_factor,
        "predicted_cooling_setpoint_rate_w",
    )?;
    if input.zone_multiplier == 0 {
        return Err(PredictedSystemLoadError::ZeroMultiplier {
            field: "zone_multiplier",
        });
    }
    if input.zone_list_multiplier == 0 {
        return Err(PredictedSystemLoadError::ZeroMultiplier {
            field: "zone_list_multiplier",
        });
    }
    let zone_multiplier = i32::try_from(input.zone_multiplier).map_err(|_| {
        PredictedSystemLoadError::MultiplierOutsideSourceIntegerRange {
            field: "zone_multiplier",
            value: input.zone_multiplier,
        }
    })?;
    let zone_list_multiplier = i32::try_from(input.zone_list_multiplier).map_err(|_| {
        PredictedSystemLoadError::MultiplierOutsideSourceIntegerRange {
            field: "zone_list_multiplier",
            value: input.zone_list_multiplier,
        }
    })?;
    let zone_multiplier_product = zone_multiplier.checked_mul(zone_list_multiplier).ok_or(
        PredictedSystemLoadError::MultiplierProductOverflow {
            zone_multiplier: input.zone_multiplier,
            zone_list_multiplier: input.zone_list_multiplier,
        },
    )? as f64;
    let total_output_required_w = require_finite_result(
        predicted_rate_w * zone_multiplier_product,
        "total_output_required_w",
    )?;
    let output_required_to_heating_setpoint_w = require_finite_result(
        predicted_heating_setpoint_rate_w * zone_multiplier_product,
        "output_required_to_heating_setpoint_w",
    )?;
    let output_required_to_cooling_setpoint_w = require_finite_result(
        predicted_cooling_setpoint_rate_w * zone_multiplier_product,
        "output_required_to_cooling_setpoint_w",
    )?;

    Ok(PredictedZoneSensibleLoads {
        zone: input.zone,
        mode,
        raw_total_load_w: total_load_w,
        raw_heating_setpoint_load_w: heating_load_w,
        raw_cooling_setpoint_load_w: cooling_load_w,
        predicted_rate_w,
        predicted_heating_setpoint_rate_w,
        predicted_cooling_setpoint_rate_w,
        total_output_required_w,
        output_required_to_heating_setpoint_w,
        output_required_to_cooling_setpoint_w,
        selected_zone_setpoint_c,
        deadband_or_setback,
    })
}

/// State-backed inputs for the bounded direct-Zone, DualSetpoint, ThirdOrder demand producer.
///
/// The zone state owns the predictor heat-balance sums, active temperature
/// history, air heat capacity, and lagged system-dependent load. The caller
/// supplies thermostat, node, scaling, and system-timestep values that are not
/// yet owned by the current heat-balance state shell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZoneDualSetpointThirdOrderDemandInput<'a> {
    /// Predictor-time state for one fully mixed direct Zone.
    pub zone_state: &'a ZoneHeatBalanceState,
    /// Active low thermostat setpoint in degrees Celsius.
    pub heating_setpoint_c: f64,
    /// Active high thermostat setpoint in degrees Celsius.
    pub cooling_setpoint_c: f64,
    /// Current direct-Zone system-node temperature, read only in deadband.
    pub zone_node_temperature_c: f64,
    /// EnergyPlus Zone load-correction factor, inclusive from -3 through 3.
    pub load_correction_factor: f64,
    /// Positive integer EnergyPlus Zone multiplier within the source signed-int range.
    pub zone_multiplier: u32,
    /// Positive integer EnergyPlus ZoneList multiplier within the source signed-int range.
    pub zone_list_multiplier: u32,
    /// Active system timestep in seconds.
    pub system_timestep_seconds: f64,
}

/// Immutable result of the state-backed sensible-demand producer.
///
/// The snapshots retain each composition boundary: predictor terms, predicted
/// and scaled loads, then the `ZoneSysEnergyDemand` thresholds consumed by zone
/// equipment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZoneDualSetpointThirdOrderDemand {
    /// CP298 source-ordered predictor coefficients and load terms.
    pub predictor_terms: PredictorThirdOrderLoadTerms,
    /// CP296 dual-setpoint predicted and output-required sensible loads.
    pub predicted_loads: PredictedZoneSensibleLoads,
    /// CP297 zone-equipment demand projection.
    pub zone_demand: ZoneSysEnergyDemand,
}

/// Fail-closed error from one of the bounded producer stages.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirectZoneDualSetpointThirdOrderDemandError {
    /// CP298 predictor-term assembly rejected its state or timestep input.
    PredictorTerms(PredictorThirdOrderLoadTermsError),
    /// CP296 predicted-load calculation rejected thermostat, node, or scaling input.
    PredictedLoad(PredictedSystemLoadError),
}

/// Produces direct-Zone ThirdOrder sensible demand from owned heat-balance state.
///
/// The composition preserves the bounded source order: CP298 assembles
/// predictor terms from non-system heat-balance sums and
/// `SysDepZoneLoadsLagged`; CP296 calculates and scales the DualSetpoint loads;
/// CP297 projects the finalized heating/cooling thresholds into
/// `ZoneSysEnergyDemand`. `SumSysMCp` and `SumSysMCpT` are deliberately not read
/// during prediction. This function does not mutate histories or advance the
/// lagged-load owner.
pub fn predict_direct_zone_dual_setpoint_third_order_demand(
    input: DirectZoneDualSetpointThirdOrderDemandInput<'_>,
) -> Result<DirectZoneDualSetpointThirdOrderDemand, DirectZoneDualSetpointThirdOrderDemandError> {
    let zone_state = input.zone_state;
    let previous_mean_air_temperatures_c = if zone_state.use_zone_timestep_history {
        zone_state.previous_mean_air_temperatures_c
    } else {
        zone_state.previous_system_mean_air_temperatures_c
    };
    let predictor_terms =
        assemble_predictor_third_order_load_terms(PredictorThirdOrderLoadTermsInput {
            sum_ha_w_per_k: zone_state.sum_ha_w_per_k,
            sum_mcp_w_per_k: zone_state.sum_mcp_w_per_k,
            sum_internal_gain_w: zone_state.convective_internal_gain_w,
            sum_hat_surf_w: zone_state.sum_hat_surf_w,
            sum_hat_ref_w: zone_state.sum_hat_ref_w,
            sum_mcp_t_w: zone_state.sum_mcp_t_w,
            system_dependent_zone_loads_lagged_w: zone_state.system_dependent_zone_loads_lagged_w,
            air_heat_capacity_j_per_k: zone_state.air_heat_capacity_j_per_k,
            timestep_seconds: input.system_timestep_seconds,
            previous_mean_air_temperatures_c,
        })
        .map_err(DirectZoneDualSetpointThirdOrderDemandError::PredictorTerms)?;
    let predicted_loads = calc_predicted_system_load_dual_setpoint_third_order(
        DualSetpointThirdOrderSystemLoadInput {
            zone: zone_state.zone_id,
            temp_dependent_load_w_per_k: predictor_terms.temp_dependent_load_w_per_k,
            temp_independent_load_w: predictor_terms.temp_independent_load_w,
            heating_setpoint_c: input.heating_setpoint_c,
            cooling_setpoint_c: input.cooling_setpoint_c,
            zone_air_temperature_c: input.zone_node_temperature_c,
            load_correction_factor: input.load_correction_factor,
            zone_multiplier: input.zone_multiplier,
            zone_list_multiplier: input.zone_list_multiplier,
        },
    )
    .map_err(DirectZoneDualSetpointThirdOrderDemandError::PredictedLoad)?;
    let zone_demand = ZoneSysEnergyDemand::from_output_required_setpoint_loads(
        predicted_loads.zone,
        predicted_loads.output_required_to_heating_setpoint_w,
        predicted_loads.output_required_to_cooling_setpoint_w,
    );

    Ok(DirectZoneDualSetpointThirdOrderDemand {
        predictor_terms,
        predicted_loads,
        zone_demand,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        heat_balance::state::ZoneAirTemperatureCoefficients,
        zone_equipment::ZoneSensibleDemandInputKind,
    };

    fn predictor_terms_input() -> PredictorThirdOrderLoadTermsInput {
        PredictorThirdOrderLoadTermsInput {
            sum_ha_w_per_k: 18.456,
            sum_mcp_w_per_k: 3.0,
            sum_internal_gain_w: 12.0,
            sum_hat_surf_w: 369.12,
            sum_hat_ref_w: 2.0,
            sum_mcp_t_w: 45.0,
            system_dependent_zone_loads_lagged_w: 7.0,
            air_heat_capacity_j_per_k: 1207.2,
            timestep_seconds: 600.0,
            previous_mean_air_temperatures_c: [20.0, 19.0, 18.0],
        }
    }

    fn zone_state() -> ZoneHeatBalanceState {
        ZoneHeatBalanceState {
            zone_id: ZoneId(7),
            zone_name: "ZONE SEVEN".to_string(),
            mean_air_temperature_c: 22.0,
            zone_timestep_average_air_temperature_c: 22.0,
            previous_mean_air_temperatures_c: [0.0; 3],
            previous_system_mean_air_temperatures_c: [0.0; 3],
            previous_system_timestep_count: 1,
            air_humidity_ratio: 0.008,
            zone_timestep_average_air_humidity_ratio: 0.008,
            previous_air_humidity_ratios: [0.008; 3],
            previous_system_air_humidity_ratios: [0.008; 3],
            use_zone_timestep_history: true,
            shorten_timestep_sys: false,
            prior_timestep_seconds: 600.0,
            volume_m3: 100.0,
            air_heat_capacity_j_per_k: 0.0,
            convective_internal_gain_w: 0.0,
            opaque_surface_conductance_w_per_k: 0.0,
            opaque_surface_heat_gain_w: 0.0,
            opaque_surface_outside_conduction_w: 0.0,
            sum_ha_w_per_k: 120.0,
            sum_hat_surf_w: 0.0,
            sum_hat_ref_w: 0.0,
            sum_mcp_w_per_k: 5.0,
            sum_mcp_t_w: 0.0,
            sum_sys_mcp_w_per_k: f64::NAN,
            sum_sys_mcp_t_w: f64::INFINITY,
            system_dependent_zone_loads_lagged_w: 250.0,
            zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
            system_timestep_average_surface_convection_report_w: None,
            system_timestep_average_air_storage_report_w: None,
        }
    }

    fn state_backed_input(
        zone_state: &ZoneHeatBalanceState,
    ) -> DirectZoneDualSetpointThirdOrderDemandInput<'_> {
        DirectZoneDualSetpointThirdOrderDemandInput {
            zone_state,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 24.0,
            zone_node_temperature_c: f64::NAN,
            load_correction_factor: 0.8,
            zone_multiplier: 2,
            zone_list_multiplier: 3,
            system_timestep_seconds: 600.0,
        }
    }

    #[test]
    fn state_backed_producer_composes_predictor_load_and_demand_once() {
        let state = zone_state();
        let output =
            predict_direct_zone_dual_setpoint_third_order_demand(state_backed_input(&state))
                .expect("bounded state-backed heating demand");

        assert_eq!(output.predictor_terms.temp_dependent_load_w_per_k, 125.0);
        assert_eq!(output.predictor_terms.temp_independent_load_w, 250.0);
        assert_eq!(
            output.predicted_loads.mode,
            PredictedZoneSensibleLoadMode::Heating
        );
        assert_eq!(output.predicted_loads.raw_heating_setpoint_load_w, 2_250.0);
        assert_eq!(output.predicted_loads.raw_cooling_setpoint_load_w, 2_750.0);
        assert_eq!(output.predicted_loads.predicted_rate_w, 1_800.0);
        assert_eq!(output.predicted_loads.total_output_required_w, 10_800.0);
        assert_eq!(output.predicted_loads.zone, state.zone_id);
        assert_eq!(output.zone_demand.zone, state.zone_id);
        assert_eq!(
            output.zone_demand.sensible_input_kind,
            ZoneSensibleDemandInputKind::SourceSetpointThresholds
        );
        assert_eq!(
            output.zone_demand.remaining_output_req_to_heat_sp_w,
            10_800.0
        );
        assert_eq!(
            output.zone_demand.remaining_output_req_to_cool_sp_w,
            13_200.0
        );
        assert!(output.zone_demand.has_inactive_moisture_demand());

        let mut without_lagged = state.clone();
        without_lagged.system_dependent_zone_loads_lagged_w = 0.0;
        let without_lagged_output = predict_direct_zone_dual_setpoint_third_order_demand(
            state_backed_input(&without_lagged),
        )
        .expect("same state without lagged system-dependent load");
        assert_eq!(
            output.predictor_terms.temp_independent_load_w
                - without_lagged_output
                    .predictor_terms
                    .temp_independent_load_w,
            250.0
        );
        assert_eq!(
            without_lagged_output
                .predicted_loads
                .raw_heating_setpoint_load_w
                - output.predicted_loads.raw_heating_setpoint_load_w,
            250.0
        );
    }

    #[test]
    fn state_backed_producer_reads_only_the_active_temperature_history() {
        let mut zone_history_state = zone_state();
        zone_history_state.air_heat_capacity_j_per_k = 600.0;
        zone_history_state.previous_mean_air_temperatures_c = [10.0, 9.0, 8.0];
        zone_history_state.previous_system_mean_air_temperatures_c = [f64::NAN; 3];
        let zone_history_output = predict_direct_zone_dual_setpoint_third_order_demand(
            state_backed_input(&zone_history_state),
        )
        .expect("zone-timestep history is active");
        assert!(
            (zone_history_output
                .predictor_terms
                .temperature_history_term_w
                - (3.0 * 10.0 - (3.0 / 2.0) * 9.0 + (1.0 / 3.0) * 8.0))
                .abs()
                < 1.0e-12
        );

        let mut system_history_state = zone_state();
        system_history_state.use_zone_timestep_history = false;
        system_history_state.air_heat_capacity_j_per_k = 600.0;
        system_history_state.previous_mean_air_temperatures_c = [f64::NAN; 3];
        system_history_state.previous_system_mean_air_temperatures_c = [20.0, 19.0, 18.0];
        let system_history_output = predict_direct_zone_dual_setpoint_third_order_demand(
            state_backed_input(&system_history_state),
        )
        .expect("system-timestep history is active");
        assert!(
            (system_history_output
                .predictor_terms
                .temperature_history_term_w
                - (3.0 * 20.0 - (3.0 / 2.0) * 19.0 + (1.0 / 3.0) * 18.0))
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn state_backed_producer_preserves_cooling_deadband_and_zero_thresholds() {
        let cases = [
            (
                3_500.0,
                PredictedZoneSensibleLoadMode::Cooling,
                -1_500.0,
                -1_100.0,
            ),
            (
                2_200.0,
                PredictedZoneSensibleLoadMode::Deadband,
                -200.0,
                200.0,
            ),
            (2_000.0, PredictedZoneSensibleLoadMode::Deadband, 0.0, 400.0),
            (
                2_400.0,
                PredictedZoneSensibleLoadMode::Deadband,
                -400.0,
                0.0,
            ),
        ];

        for (independent_load_w, expected_mode, expected_heating_w, expected_cooling_w) in cases {
            let mut state = zone_state();
            state.sum_ha_w_per_k = 100.0;
            state.sum_mcp_w_per_k = 0.0;
            state.convective_internal_gain_w = independent_load_w;
            state.system_dependent_zone_loads_lagged_w = 0.0;
            let mut input = state_backed_input(&state);
            input.zone_node_temperature_c = 22.0;
            input.load_correction_factor = 1.0;
            input.zone_multiplier = 1;
            input.zone_list_multiplier = 1;

            let output = predict_direct_zone_dual_setpoint_third_order_demand(input)
                .expect("bounded sign-branch fixture");
            assert_eq!(output.predicted_loads.mode, expected_mode);
            assert_eq!(
                output.zone_demand.remaining_output_req_to_heat_sp_w,
                expected_heating_w
            );
            assert_eq!(
                output.zone_demand.remaining_output_req_to_cool_sp_w,
                expected_cooling_w
            );
            assert_eq!(
                output.zone_demand.sensible_input_kind,
                ZoneSensibleDemandInputKind::SourceSetpointThresholds
            );
        }
    }

    #[test]
    fn state_backed_producer_wraps_stage_errors_without_mutating_state() {
        let mut state = zone_state();
        state.sum_sys_mcp_w_per_k = 0.0;
        state.sum_sys_mcp_t_w = 0.0;
        let original = state.clone();

        let mut invalid_timestep = state_backed_input(&state);
        invalid_timestep.system_timestep_seconds = 0.0;
        assert_eq!(
            predict_direct_zone_dual_setpoint_third_order_demand(invalid_timestep),
            Err(DirectZoneDualSetpointThirdOrderDemandError::PredictorTerms(
                PredictorThirdOrderLoadTermsError::TimestepSecondsNotPositive { value: 0.0 }
            ))
        );
        assert_eq!(state, original);

        let mut invalid_correction = state_backed_input(&state);
        invalid_correction.load_correction_factor = 4.0;
        assert_eq!(
            predict_direct_zone_dual_setpoint_third_order_demand(invalid_correction),
            Err(DirectZoneDualSetpointThirdOrderDemandError::PredictedLoad(
                PredictedSystemLoadError::InputOutsideInclusiveRange {
                    field: "load_correction_factor",
                    value: 4.0,
                    minimum: -3.0,
                    maximum: 3.0,
                }
            ))
        );
        assert_eq!(state, original);
    }

    #[test]
    fn assembles_source_order_predictor_terms_with_lagged_load() {
        let terms = assemble_predictor_third_order_load_terms(predictor_terms_input())
            .expect("finite predictor terms must be supported");

        let air_power_cap_w_per_k = 1207.2 / 600.0;
        let expected_history_w =
            air_power_cap_w_per_k * (3.0 * 20.0 - (3.0 / 2.0) * 19.0 + (1.0 / 3.0) * 18.0);
        assert!((terms.temp_dependent_coefficient_w_per_k - 21.456).abs() < 1.0e-12);
        assert!((terms.temp_independent_coefficient_w - 431.12).abs() < 1.0e-12);
        assert!((terms.air_power_cap_w_per_k - air_power_cap_w_per_k).abs() < 1.0e-12);
        assert!((terms.temperature_history_term_w - expected_history_w).abs() < 1.0e-12);
        assert!(
            (terms.temp_dependent_load_w_per_k - ((11.0 / 6.0) * air_power_cap_w_per_k + 21.456))
                .abs()
                < 1.0e-12
        );
        assert!((terms.temp_independent_load_w - (expected_history_w + 431.12)).abs() < 1.0e-12);

        let mut negative_capacity = predictor_terms_input();
        negative_capacity.air_heat_capacity_j_per_k = -1207.2;
        let negative_capacity = assemble_predictor_third_order_load_terms(negative_capacity)
            .expect("finite negative air heat capacity must not be silently rewritten");
        assert!((negative_capacity.air_power_cap_w_per_k + 2.012).abs() < 1.0e-12);
    }

    #[test]
    fn lagged_system_dependent_load_changes_only_independent_predictor_terms() {
        let mut without_lagged = predictor_terms_input();
        without_lagged.system_dependent_zone_loads_lagged_w = 0.0;
        let without_lagged = assemble_predictor_third_order_load_terms(without_lagged)
            .expect("zero lagged load must be supported");

        let mut with_lagged = predictor_terms_input();
        with_lagged.system_dependent_zone_loads_lagged_w = 55.0;
        let with_lagged = assemble_predictor_third_order_load_terms(with_lagged)
            .expect("finite lagged load must be supported");

        assert_eq!(
            with_lagged.temp_dependent_coefficient_w_per_k,
            without_lagged.temp_dependent_coefficient_w_per_k
        );
        assert_eq!(
            with_lagged.air_power_cap_w_per_k,
            without_lagged.air_power_cap_w_per_k
        );
        assert_eq!(
            with_lagged.temperature_history_term_w,
            without_lagged.temperature_history_term_w
        );
        assert_eq!(
            with_lagged.temp_dependent_load_w_per_k,
            without_lagged.temp_dependent_load_w_per_k
        );
        assert!(
            (with_lagged.temp_independent_coefficient_w
                - without_lagged.temp_independent_coefficient_w
                - 55.0)
                .abs()
                < 1.0e-12
        );
        assert!(
            (with_lagged.temp_independent_load_w - without_lagged.temp_independent_load_w - 55.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn assembled_predictor_terms_feed_the_bounded_dual_setpoint_kernel() {
        let terms = assemble_predictor_third_order_load_terms(PredictorThirdOrderLoadTermsInput {
            sum_ha_w_per_k: 120.0,
            sum_mcp_w_per_k: 5.0,
            sum_internal_gain_w: 0.0,
            sum_hat_surf_w: 0.0,
            sum_hat_ref_w: 0.0,
            sum_mcp_t_w: 0.0,
            system_dependent_zone_loads_lagged_w: 0.0,
            air_heat_capacity_j_per_k: 0.0,
            timestep_seconds: 600.0,
            previous_mean_air_temperatures_c: [0.0; 3],
        })
        .expect("finite steady predictor terms must be supported");
        let output = calc_predicted_system_load_dual_setpoint_third_order(
            DualSetpointThirdOrderSystemLoadInput {
                zone: ZoneId(7),
                temp_dependent_load_w_per_k: terms.temp_dependent_load_w_per_k,
                temp_independent_load_w: terms.temp_independent_load_w,
                heating_setpoint_c: 20.0,
                cooling_setpoint_c: 24.0,
                zone_air_temperature_c: f64::NAN,
                load_correction_factor: 1.0,
                zone_multiplier: 1,
                zone_list_multiplier: 1,
            },
        )
        .expect("assembled predictor terms must reach the bounded load kernel");

        assert_eq!(terms.temp_dependent_load_w_per_k, 125.0);
        assert_eq!(terms.temp_independent_load_w, 0.0);
        assert_eq!(output.mode, PredictedZoneSensibleLoadMode::Heating);
        assert_eq!(output.raw_heating_setpoint_load_w, 2500.0);
        assert_eq!(output.raw_cooling_setpoint_load_w, 3000.0);
    }

    #[test]
    fn predictor_term_assembly_rejects_invalid_inputs_and_overflow() {
        let mut nonfinite = predictor_terms_input();
        nonfinite.system_dependent_zone_loads_lagged_w = f64::NAN;
        assert_eq!(
            assemble_predictor_third_order_load_terms(nonfinite),
            Err(PredictorThirdOrderLoadTermsError::InputNotFinite {
                field: "system_dependent_zone_loads_lagged_w"
            })
        );

        for timestep_seconds in [0.0, -0.0, -1.0] {
            let mut invalid_timestep = predictor_terms_input();
            invalid_timestep.timestep_seconds = timestep_seconds;
            assert_eq!(
                assemble_predictor_third_order_load_terms(invalid_timestep),
                Err(
                    PredictorThirdOrderLoadTermsError::TimestepSecondsNotPositive {
                        value: timestep_seconds
                    }
                )
            );
        }

        let mut overflowing = predictor_terms_input();
        overflowing.air_heat_capacity_j_per_k = f64::MAX;
        overflowing.timestep_seconds = 0.5;
        overflowing.sum_ha_w_per_k = f64::MAX;
        overflowing.sum_mcp_w_per_k = f64::MAX;
        assert_eq!(
            assemble_predictor_third_order_load_terms(overflowing),
            Err(PredictorThirdOrderLoadTermsError::ResultNotFinite {
                field: "air_power_cap_w_per_k"
            })
        );

        let mut coefficient_overflow = predictor_terms_input();
        coefficient_overflow.sum_ha_w_per_k = f64::MAX;
        coefficient_overflow.sum_mcp_w_per_k = f64::MAX;
        assert_eq!(
            assemble_predictor_third_order_load_terms(coefficient_overflow),
            Err(PredictorThirdOrderLoadTermsError::ResultNotFinite {
                field: "temp_dependent_coefficient_w_per_k"
            })
        );
    }

    fn input(
        temp_dependent_load_w_per_k: f64,
        temp_independent_load_w: f64,
        heating_setpoint_c: f64,
        cooling_setpoint_c: f64,
        zone_air_temperature_c: f64,
    ) -> DualSetpointThirdOrderSystemLoadInput {
        DualSetpointThirdOrderSystemLoadInput {
            zone: ZoneId(7),
            temp_dependent_load_w_per_k,
            temp_independent_load_w,
            heating_setpoint_c,
            cooling_setpoint_c,
            zone_air_temperature_c,
            load_correction_factor: 1.0,
            zone_multiplier: 1,
            zone_list_multiplier: 1,
        }
    }

    #[test]
    fn matches_energyplus_dual_setpoint_heating_fixture() {
        let output = calc_predicted_system_load_dual_setpoint_third_order(input(
            125.0,
            0.0,
            20.0,
            24.0,
            f64::NAN,
        ))
        .expect("heating fixture must be supported");

        assert_eq!(output.zone, ZoneId(7));
        assert_eq!(output.mode, PredictedZoneSensibleLoadMode::Heating);
        assert_eq!(output.raw_heating_setpoint_load_w, 2500.0);
        assert_eq!(output.raw_cooling_setpoint_load_w, 3000.0);
        assert_eq!(output.raw_total_load_w, 2500.0);
        assert_eq!(output.selected_zone_setpoint_c, 20.0);
        assert!(!output.deadband_or_setback);
        assert_eq!(output.total_output_required_w, 2500.0);
    }

    #[test]
    fn matches_energyplus_dual_setpoint_cooling_fixture() {
        let output = calc_predicted_system_load_dual_setpoint_third_order(input(
            40.0,
            3500.0,
            20.0,
            25.0,
            f64::INFINITY,
        ))
        .expect("cooling fixture must be supported");

        assert_eq!(output.mode, PredictedZoneSensibleLoadMode::Cooling);
        assert_eq!(output.raw_heating_setpoint_load_w, -2700.0);
        assert_eq!(output.raw_cooling_setpoint_load_w, -2500.0);
        assert_eq!(output.raw_total_load_w, -2500.0);
        assert_eq!(output.selected_zone_setpoint_c, 25.0);
        assert!(!output.deadband_or_setback);
        assert_eq!(output.total_output_required_w, -2500.0);
    }

    #[test]
    fn clamps_deadband_setpoint_to_both_bounds() {
        for (node_temperature_c, expected_setpoint_c) in [(18.0, 20.0), (22.0, 22.0), (26.0, 24.0)]
        {
            let output = calc_predicted_system_load_dual_setpoint_third_order(input(
                100.0,
                2200.0,
                20.0,
                24.0,
                node_temperature_c,
            ))
            .expect("deadband fixture must be supported");
            assert_eq!(output.mode, PredictedZoneSensibleLoadMode::Deadband);
            assert_eq!(output.raw_heating_setpoint_load_w, -200.0);
            assert_eq!(output.raw_cooling_setpoint_load_w, 200.0);
            assert_eq!(output.raw_total_load_w, 0.0);
            assert_eq!(output.selected_zone_setpoint_c, expected_setpoint_c);
            assert!(output.deadband_or_setback);
        }
    }

    #[test]
    fn treats_both_zero_boundaries_as_deadband() {
        for (temp_independent_load_w, expected_heating_w, expected_cooling_w) in
            [(2000.0, 0.0, 400.0), (2400.0, -400.0, 0.0)]
        {
            let output = calc_predicted_system_load_dual_setpoint_third_order(input(
                100.0,
                temp_independent_load_w,
                20.0,
                24.0,
                22.0,
            ))
            .expect("zero boundary must be deadband");
            assert_eq!(output.mode, PredictedZoneSensibleLoadMode::Deadband);
            assert_eq!(output.raw_heating_setpoint_load_w, expected_heating_w);
            assert_eq!(output.raw_cooling_setpoint_load_w, expected_cooling_w);
            assert_eq!(output.raw_total_load_w, 0.0);
        }
    }

    #[test]
    fn applies_correction_then_zone_multiplier_product() {
        let mut scaled = input(100.0, 1500.0, 20.0, 24.0, f64::NAN);
        scaled.load_correction_factor = 0.8;
        scaled.zone_multiplier = 2;
        scaled.zone_list_multiplier = 3;

        let output = calc_predicted_system_load_dual_setpoint_third_order(scaled)
            .expect("scaled heating fixture must be supported");
        assert_eq!(output.raw_total_load_w, 500.0);
        assert_eq!(output.raw_heating_setpoint_load_w, 500.0);
        assert_eq!(output.raw_cooling_setpoint_load_w, 900.0);
        assert_eq!(output.predicted_rate_w, 400.0);
        assert_eq!(output.predicted_heating_setpoint_rate_w, 400.0);
        assert_eq!(output.predicted_cooling_setpoint_rate_w, 720.0);
        assert_eq!(output.total_output_required_w, 2400.0);
        assert_eq!(output.output_required_to_heating_setpoint_w, 2400.0);
        assert_eq!(output.output_required_to_cooling_setpoint_w, 4320.0);

        let mut grouping = input(1.0, 17.0, 20.0, 24.0, f64::NAN);
        grouping.load_correction_factor = 0.1;
        grouping.zone_multiplier = 1;
        grouping.zone_list_multiplier = 5;
        let grouped_output = calc_predicted_system_load_dual_setpoint_third_order(grouping)
            .expect("non-associative grouping fixture must be supported");
        let source_grouped = (3.0_f64 * 0.1) * (1.0 * 5.0);
        let regrouped = 3.0_f64 * (0.1 * (1.0 * 5.0));
        assert_ne!(source_grouped.to_bits(), regrouped.to_bits());
        assert_eq!(
            grouped_output.total_output_required_w.to_bits(),
            source_grouped.to_bits()
        );
    }

    #[test]
    fn accepts_signed_source_load_correction_range_and_rejects_outside_values() {
        for (load_correction_factor, expected_total_w, expected_cooling_w) in [
            (-3.0, -1500.0, -2700.0),
            (0.0, 0.0, 0.0),
            (3.0, 1500.0, 2700.0),
        ] {
            let mut corrected = input(100.0, 1500.0, 20.0, 24.0, f64::NAN);
            corrected.load_correction_factor = load_correction_factor;
            let output = calc_predicted_system_load_dual_setpoint_third_order(corrected)
                .expect("inclusive EnergyPlus correction range must be supported");
            assert_eq!(output.predicted_rate_w, expected_total_w);
            assert_eq!(output.predicted_heating_setpoint_rate_w, expected_total_w);
            assert_eq!(output.predicted_cooling_setpoint_rate_w, expected_cooling_w);
            assert_eq!(output.total_output_required_w, expected_total_w);
        }

        for load_correction_factor in [-3.000_000_1, 3.000_000_1] {
            let mut invalid = input(100.0, 1500.0, 20.0, 24.0, 22.0);
            invalid.load_correction_factor = load_correction_factor;
            assert_eq!(
                calc_predicted_system_load_dual_setpoint_third_order(invalid),
                Err(PredictedSystemLoadError::InputOutsideInclusiveRange {
                    field: "load_correction_factor",
                    value: load_correction_factor,
                    minimum: -3.0,
                    maximum: 3.0,
                })
            );
        }
    }
    #[test]
    fn rejects_inverted_setpoints_before_reading_scaling_inputs() {
        let mut inverted = input(100.0, 0.0, 24.0, 20.0, 22.0);
        inverted.load_correction_factor = f64::NAN;
        inverted.zone_multiplier = 0;
        let error = calc_predicted_system_load_dual_setpoint_third_order(inverted)
            .expect_err("inverted effective setpoints must fail before scaling inputs");
        assert_eq!(
            error,
            PredictedSystemLoadError::HeatingLoadExceedsCoolingLoad {
                heating_load_w: 2400.0,
                cooling_load_w: 2000.0,
            }
        );

        let mut negative_dependent = input(-100.0, 0.0, 20.0, 24.0, 22.0);
        negative_dependent.load_correction_factor = f64::NAN;
        assert_eq!(
            calc_predicted_system_load_dual_setpoint_third_order(negative_dependent),
            Err(PredictedSystemLoadError::HeatingLoadExceedsCoolingLoad {
                heating_load_w: -2000.0,
                cooling_load_w: -2400.0,
            })
        );
    }

    #[test]
    fn rejects_nonfinite_used_inputs_but_reads_node_temperature_only_in_deadband() {
        let mut invalid = input(f64::NAN, 0.0, 20.0, 24.0, 22.0);
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::NonFiniteInput {
                field: "temp_dependent_load_w_per_k"
            })
        ));

        invalid = input(100.0, f64::INFINITY, 20.0, 24.0, 22.0);
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::NonFiniteInput {
                field: "temp_independent_load_w"
            })
        ));

        invalid = input(100.0, 0.0, f64::NEG_INFINITY, 24.0, 22.0);
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::NonFiniteInput {
                field: "heating_setpoint_c"
            })
        ));

        invalid = input(100.0, 0.0, 20.0, f64::NAN, 22.0);
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::NonFiniteInput {
                field: "cooling_setpoint_c"
            })
        ));

        invalid = input(100.0, 2200.0, 20.0, 24.0, f64::NAN);
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::NonFiniteInput {
                field: "zone_air_temperature_c"
            })
        ));

        invalid = input(100.0, 0.0, 20.0, 24.0, 22.0);
        invalid.load_correction_factor = f64::INFINITY;
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::NonFiniteInput {
                field: "load_correction_factor"
            })
        ));
    }

    #[test]
    fn accepts_zero_dependent_load_and_rejects_invalid_multiplier_state() {
        for (temp_independent_load_w, expected_mode, expected_total_w) in [
            (-500.0, PredictedZoneSensibleLoadMode::Heating, 500.0),
            (500.0, PredictedZoneSensibleLoadMode::Cooling, -500.0),
            (0.0, PredictedZoneSensibleLoadMode::Deadband, 0.0),
        ] {
            let output = calc_predicted_system_load_dual_setpoint_third_order(input(
                0.0,
                temp_independent_load_w,
                20.0,
                24.0,
                22.0,
            ))
            .expect("zero dependent load follows the source sign branches");
            assert_eq!(output.mode, expected_mode);
            assert_eq!(output.raw_total_load_w, expected_total_w);
            assert_eq!(output.raw_heating_setpoint_load_w, -temp_independent_load_w);
            assert_eq!(output.raw_cooling_setpoint_load_w, -temp_independent_load_w);
        }

        let mut invalid = input(100.0, 0.0, 20.0, 24.0, 22.0);
        invalid.zone_multiplier = 0;
        assert_eq!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::ZeroMultiplier {
                field: "zone_multiplier"
            })
        );

        invalid = input(100.0, 0.0, 20.0, 24.0, 22.0);
        invalid.zone_list_multiplier = 0;
        assert_eq!(
            calc_predicted_system_load_dual_setpoint_third_order(invalid),
            Err(PredictedSystemLoadError::ZeroMultiplier {
                field: "zone_list_multiplier"
            })
        );
    }

    #[test]
    fn rejects_nonfinite_intermediate_or_scaled_results() {
        let raw_overflow = input(f64::MAX, 0.0, 2.0, 3.0, 22.0);
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(raw_overflow),
            Err(PredictedSystemLoadError::NonFiniteResult {
                field: "raw_heating_setpoint_load_w"
            })
        ));

        let mut correction_overflow = input(f64::MAX / 2.0, 0.0, 1.0, 1.0, 22.0);
        correction_overflow.load_correction_factor = 3.0;
        assert!(matches!(
            calc_predicted_system_load_dual_setpoint_third_order(correction_overflow),
            Err(PredictedSystemLoadError::NonFiniteResult {
                field: "predicted_rate_w"
            })
        ));

        let mut multiplier_out_of_range = input(100.0, 1500.0, 20.0, 24.0, 22.0);
        multiplier_out_of_range.zone_multiplier = (i32::MAX as u32) + 1;
        assert_eq!(
            calc_predicted_system_load_dual_setpoint_third_order(multiplier_out_of_range),
            Err(
                PredictedSystemLoadError::MultiplierOutsideSourceIntegerRange {
                    field: "zone_multiplier",
                    value: (i32::MAX as u32) + 1,
                }
            )
        );

        let mut multiplier_overflow = input(100.0, 1500.0, 20.0, 24.0, 22.0);
        multiplier_overflow.zone_multiplier = 46_341;
        multiplier_overflow.zone_list_multiplier = 46_341;
        assert_eq!(
            calc_predicted_system_load_dual_setpoint_third_order(multiplier_overflow),
            Err(PredictedSystemLoadError::MultiplierProductOverflow {
                zone_multiplier: 46_341,
                zone_list_multiplier: 46_341,
            })
        );
    }
}
