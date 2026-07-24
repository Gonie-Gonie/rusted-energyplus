//! Bounded `calcPredictedSystemLoad` arithmetic for the source-order port.

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

#[cfg(test)]
mod tests {
    use super::*;

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
