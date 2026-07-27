//! Direct-Zone predictor-to-PurchasedAir production coupling.

mod validation;

use crate::heat_balance::{
    state::ZoneHeatBalanceState,
    zone_predictor_corrector::predicted_system_load::{
        DirectZoneDualSetpointThirdOrderDemand, DirectZoneDualSetpointThirdOrderDemandError,
        DirectZoneDualSetpointThirdOrderDemandInput,
        predict_direct_zone_dual_setpoint_third_order_demand,
    },
};
use ep_model::{IdealLoadsAirSystem, NodeId};

use super::{
    IdealLoadsPurchasedAirBranch, IdealLoadsSensibleLimitContext, IdealLoadsZoneState,
    PurchasedAirInitSnapshot, SimPurchasedAirCompatError, SimPurchasedAirCompatInput,
    SimPurchasedAirCompatOutput, sim_purchased_air_compat_with_init_flags,
};
use validation::{initialized_limit_context, validate_coupling_inputs, validate_supported_branch};

/// Inputs for one bounded direct-Zone predictor-to-PurchasedAir coupling step.
///
/// The caller owns the prebound proof that `system`, `supply_node`, and
/// `zone_state` describe the same single, fully mixed controlled Zone. This
/// production boundary accepts the no-outdoor-air sensible PurchasedAir
/// branches with either no limit or resolved numeric flow/capacity limits.
pub struct DirectZonePurchasedAirCouplingInput<'a> {
    /// Live heat-balance state read by CP299 and updated with correction feedback.
    pub zone_state: &'a mut ZoneHeatBalanceState,
    /// Active low thermostat setpoint in degrees Celsius.
    pub heating_setpoint_c: f64,
    /// Active high thermostat setpoint in degrees Celsius.
    pub cooling_setpoint_c: f64,
    /// Current direct-Zone system-node temperature in degrees Celsius.
    pub zone_node_temperature_c: f64,
    /// Bound blank-exhaust return-node state projected for recirculation.
    pub recirculation_state: IdealLoadsZoneState,
    /// Fully mixed Zone load-correction factor, inclusive from -3 through 3.
    pub load_correction_factor: f64,
    /// Positive EnergyPlus Zone multiplier.
    pub zone_multiplier: u32,
    /// Positive EnergyPlus ZoneList multiplier.
    pub zone_list_multiplier: u32,
    /// Active system timestep in seconds.
    pub system_timestep_seconds: f64,
    /// Prebound IdealLoads system for the controlled Zone.
    pub system: &'a IdealLoadsAirSystem,
    /// Prebound supply node updated by PurchasedAir.
    pub supply_node: NodeId,
    /// Availability-schedule result for the current system timestep.
    pub unit_available: bool,
    /// Psychrometric and standard-density context for PurchasedAir.
    pub limit_context: IdealLoadsSensibleLimitContext,
    /// Persistent `InitPurchasedAir` snapshot for this exact timestep.
    pub initialization: PurchasedAirInitSnapshot,
}

/// Typed relation checked between initialization and direct coupling state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectZonePurchasedAirInitializationRelation {
    /// Initialized IdealLoads system identity.
    System,
    /// Initialized controlled Zone identity.
    ControlledZone,
    /// Initialized supply-node identity.
    SupplyNode,
}

/// Source-ordered system-air correction feedback derived from PurchasedAir.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZonePurchasedAirSystemFeedback {
    /// Zone multiplied PurchasedAir supply-node mass flow in kg/s.
    pub multiplied_supply_mass_flow_rate_kg_per_s: f64,
    /// Product of the Zone and ZoneList multipliers removed exactly once.
    pub multiplier_product: f64,
    /// Supply mass flow assigned to the base Zone heat balance in kg/s.
    pub zone_supply_mass_flow_rate_kg_per_s: f64,
    /// EnergyPlus `PsyCpAirFnW` value at the live Zone humidity ratio in J/kg-K.
    pub cp_air_j_per_kg_k: f64,
    /// Final PurchasedAir supply-node temperature in degrees Celsius.
    pub supply_temperature_c: f64,
    /// EnergyPlus correction-time `SumSysMCp` in W/K.
    pub sum_sys_mcp_w_per_k: f64,
    /// EnergyPlus correction-time `SumSysMCpT` in W.
    pub sum_sys_mcp_t_w: f64,
}

/// Result of one committed direct-Zone predictor-to-PurchasedAir coupling step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZonePurchasedAirCouplingOutput {
    /// CP299 predicted demand and its retained source-order snapshots.
    pub prediction: DirectZoneDualSetpointThirdOrderDemand,
    /// Generic PurchasedAir calculation, node update, and report snapshot.
    pub purchased_air: SimPurchasedAirCompatOutput,
    /// System-air feedback written to the live Zone heat-balance state.
    pub feedback: DirectZonePurchasedAirSystemFeedback,
}

/// Fail-closed error for direct-Zone predictor-to-PurchasedAir coupling.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectZonePurchasedAirCouplingError {
    /// The typed IdealLoads system selected a branch outside this coupling boundary.
    UnsupportedBranch {
        /// Branch selected from the typed IdealLoads inputs.
        branch: IdealLoadsPurchasedAirBranch,
    },
    /// The persistent initialization snapshot belongs to another typed object.
    InitializationIdentityMismatch {
        /// Relation whose typed IDs did not agree.
        relation: DirectZonePurchasedAirInitializationRelation,
    },
    /// The persistent lifecycle has not completed the bounded release gates.
    InitializationNotReady,
    /// A begin-environment cache value violated its physical invariant.
    InitializationCacheInvalid {
        /// Stable cache field name.
        field: &'static str,
        /// Rejected cache value, or NaN when the cache was absent.
        value: f64,
    },
    /// A live air-state or PurchasedAir context input was NaN or infinite.
    InputNotFinite {
        /// Stable input field name.
        field: &'static str,
    },
    /// A finite input violated a nonnegative physical-state invariant.
    InputNegative {
        /// Stable input field name.
        field: &'static str,
        /// Rejected finite value.
        value: f64,
    },
    /// CP299 rejected predictor, thermostat, scaling, or timestep state.
    Prediction(DirectZoneDualSetpointThirdOrderDemandError),
    /// The generic PurchasedAir compatibility boundary rejected the system.
    PurchasedAir(SimPurchasedAirCompatError),
    /// A PurchasedAir or feedback result was NaN or infinite.
    ResultNotFinite {
        /// Stable result field name.
        field: &'static str,
    },
    /// A finite result violated a nonnegative physical-state invariant.
    ResultNegative {
        /// Stable result field name.
        field: &'static str,
        /// Rejected finite value.
        value: f64,
    },
    /// A finite result violated a strictly-positive physical-state invariant.
    ResultNotPositive {
        /// Stable result field name.
        field: &'static str,
        /// Rejected finite value.
        value: f64,
    },
}

/// Predicts direct-Zone demand, simulates PurchasedAir, and commits system-air feedback.
///
/// The function is transactional with respect to `zone_state`: CP299 and
/// PurchasedAir run against immutable snapshots, every feedback scalar is
/// validated in a local buffer, and only then are `sum_sys_mcp_w_per_k` and
/// `sum_sys_mcp_t_w` overwritten. `system_dependent_zone_loads_lagged_w` and all
/// other heat-balance fields are deliberately left unchanged.
pub fn couple_direct_zone_predicted_demand_to_purchased_air(
    input: DirectZonePurchasedAirCouplingInput<'_>,
) -> Result<DirectZonePurchasedAirCouplingOutput, DirectZonePurchasedAirCouplingError> {
    validate_supported_branch(input.system)?;
    validate_coupling_inputs(&input)?;
    initialized_limit_context(&input)?;
    let prediction = predict_direct_zone_demand_for_purchased_air(
        DirectZoneDualSetpointThirdOrderDemandInput {
            zone_state: &*input.zone_state,
            heating_setpoint_c: input.heating_setpoint_c,
            cooling_setpoint_c: input.cooling_setpoint_c,
            zone_node_temperature_c: input.zone_node_temperature_c,
            load_correction_factor: input.load_correction_factor,
            zone_multiplier: input.zone_multiplier,
            zone_list_multiplier: input.zone_list_multiplier,
            system_timestep_seconds: input.system_timestep_seconds,
        },
    )?;
    complete_direct_zone_purchased_air_coupling(input, prediction)
}

/// Runs the direct-Zone demand producer before the PurchasedAir Init stage.
pub(super) fn predict_direct_zone_demand_for_purchased_air(
    input: DirectZoneDualSetpointThirdOrderDemandInput<'_>,
) -> Result<DirectZoneDualSetpointThirdOrderDemand, DirectZonePurchasedAirCouplingError> {
    predict_direct_zone_dual_setpoint_third_order_demand(input)
        .map_err(DirectZonePurchasedAirCouplingError::Prediction)
}

/// Completes Calc, Update, Report, and Zone feedback from an earlier prediction.
pub(super) fn complete_direct_zone_purchased_air_coupling(
    input: DirectZonePurchasedAirCouplingInput<'_>,
    prediction: DirectZoneDualSetpointThirdOrderDemand,
) -> Result<DirectZonePurchasedAirCouplingOutput, DirectZonePurchasedAirCouplingError> {
    validate_supported_branch(input.system)?;
    validate_coupling_inputs(&input)?;
    let initialized_limit_context = initialized_limit_context(&input)?;

    let purchased_air_zone_state = IdealLoadsZoneState {
        air_temperature_c: input.zone_node_temperature_c,
        air_humidity_ratio: input.zone_state.air_humidity_ratio,
    };
    let purchased_air = sim_purchased_air_compat_with_init_flags(
        SimPurchasedAirCompatInput {
            system: input.system,
            supply_node: input.supply_node,
            zone_state: purchased_air_zone_state,
            recirculation_state: input.recirculation_state,
            demand: prediction.zone_demand,
            unit_available: input.unit_available,
            limit_context: initialized_limit_context,
        },
        input.initialization.flags,
    )
    .map_err(DirectZonePurchasedAirCouplingError::PurchasedAir)?;

    let feedback = derive_system_air_feedback(
        purchased_air,
        input.zone_multiplier,
        input.zone_list_multiplier,
    )?;

    input.zone_state.sum_sys_mcp_w_per_k = feedback.sum_sys_mcp_w_per_k;
    input.zone_state.sum_sys_mcp_t_w = feedback.sum_sys_mcp_t_w;

    Ok(DirectZonePurchasedAirCouplingOutput {
        prediction,
        purchased_air,
        feedback,
    })
}

fn derive_system_air_feedback(
    purchased_air: SimPurchasedAirCompatOutput,
    zone_multiplier: u32,
    zone_list_multiplier: u32,
) -> Result<DirectZonePurchasedAirSystemFeedback, DirectZonePurchasedAirCouplingError> {
    let multiplied_supply_mass_flow_rate_kg_per_s = require_nonnegative_result(
        purchased_air.supply_node_update.mass_flow_rate_kg_per_s,
        "purchased_air.supply_node_update.mass_flow_rate_kg_per_s",
    )?;
    let supply_temperature_c = require_finite_result(
        purchased_air.supply_node_update.temperature_c,
        "purchased_air.supply_node_update.temperature_c",
    )?;
    let cp_air_j_per_kg_k = require_positive_result(
        purchased_air.calculation.cp_air_j_per_kg_k,
        "purchased_air.calculation.cp_air_j_per_kg_k",
    )?;
    require_nonnegative_result(
        purchased_air.supply_node_update.humidity_ratio,
        "purchased_air.supply_node_update.humidity_ratio",
    )?;
    require_finite_result(
        purchased_air.supply_node_update.enthalpy_j_per_kg,
        "purchased_air.supply_node_update.enthalpy_j_per_kg",
    )?;
    let multiplier_product = require_positive_result(
        f64::from(zone_multiplier) * f64::from(zone_list_multiplier),
        "multiplier_product",
    )?;
    let zone_supply_mass_flow_rate_kg_per_s = require_nonnegative_result(
        multiplied_supply_mass_flow_rate_kg_per_s / multiplier_product,
        "zone_supply_mass_flow_rate_kg_per_s",
    )?;
    let multiplied_sum_sys_mcp_w_per_k = require_nonnegative_result(
        multiplied_supply_mass_flow_rate_kg_per_s * cp_air_j_per_kg_k,
        "multiplied_sum_sys_mcp_w_per_k",
    )?;
    let sum_sys_mcp_w_per_k = require_nonnegative_result(
        multiplied_sum_sys_mcp_w_per_k / multiplier_product,
        "sum_sys_mcp_w_per_k",
    )?;
    let multiplied_sum_sys_mcp_t_w = require_finite_result(
        multiplied_sum_sys_mcp_w_per_k * supply_temperature_c,
        "multiplied_sum_sys_mcp_t_w",
    )?;
    let sum_sys_mcp_t_w = require_finite_result(
        multiplied_sum_sys_mcp_t_w / multiplier_product,
        "sum_sys_mcp_t_w",
    )?;

    Ok(DirectZonePurchasedAirSystemFeedback {
        multiplied_supply_mass_flow_rate_kg_per_s,
        multiplier_product,
        zone_supply_mass_flow_rate_kg_per_s,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        sum_sys_mcp_w_per_k,
        sum_sys_mcp_t_w,
    })
}

fn require_finite_result(
    value: f64,
    field: &'static str,
) -> Result<f64, DirectZonePurchasedAirCouplingError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(DirectZonePurchasedAirCouplingError::ResultNotFinite { field })
    }
}

fn require_nonnegative_result(
    value: f64,
    field: &'static str,
) -> Result<f64, DirectZonePurchasedAirCouplingError> {
    let value = require_finite_result(value, field)?;
    if value < 0.0 {
        Err(DirectZonePurchasedAirCouplingError::ResultNegative { field, value })
    } else {
        Ok(value)
    }
}

fn require_positive_result(
    value: f64,
    field: &'static str,
) -> Result<f64, DirectZonePurchasedAirCouplingError> {
    let value = require_finite_result(value, field)?;
    if value <= 0.0 {
        Err(DirectZonePurchasedAirCouplingError::ResultNotPositive { field, value })
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        energyplus_moist_air_specific_heat_j_per_kg_k,
        heat_balance::state::ZoneAirTemperatureCoefficients,
        ideal_loads::{
            IdealLoadsSensibleMode, IdealLoadsUnsupportedFeature, PurchasedAirInitBoundTopology,
            PurchasedAirInitCallContext, PurchasedAirInitManagerPlan,
            PurchasedAirInitManagerPlanRow, PurchasedAirRuntimeState, init_purchased_air_runtime,
            select_purchased_air_branch,
        },
    };
    use ep_model::{
        AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
        HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystemId, IdealLoadsFuelType,
        IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType, ZoneEquipmentListId, ZoneId,
    };

    const ABS_TOLERANCE: f64 = 1.0e-9;

    #[test]
    fn heating_prediction_feeds_purchased_air_and_commits_feedback() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        let original = state.clone();
        let system = test_system();

        let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect("bounded heating coupling");

        assert_eq!(
            output.purchased_air.calculation.mode,
            IdealLoadsSensibleMode::Heating
        );
        assert_close(
            output.prediction.predicted_loads.total_output_required_w,
            2_000.0,
        );
        assert!(output.feedback.multiplied_supply_mass_flow_rate_kg_per_s > 0.0);
        assert_close(
            output.feedback.cp_air_j_per_kg_k,
            energyplus_moist_air_specific_heat_j_per_kg_k(original.air_humidity_ratio),
        );
        assert_close(
            state.sum_sys_mcp_w_per_k,
            output.feedback.sum_sys_mcp_w_per_k,
        );
        assert_close(state.sum_sys_mcp_t_w, output.feedback.sum_sys_mcp_t_w);
        assert_only_system_air_sums_changed(&original, &state);
    }

    #[test]
    fn cooling_prediction_feeds_purchased_air_with_negative_threshold_intact() {
        let mut state = zone_state_for_temp_independent_load(3_000.0);
        let system = test_system();

        let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect("bounded cooling coupling");

        assert_eq!(
            output.purchased_air.calculation.mode,
            IdealLoadsSensibleMode::Cooling
        );
        assert_close(
            output
                .prediction
                .zone_demand
                .remaining_output_req_to_cool_sp_w,
            -600.0,
        );
        assert!(output.feedback.sum_sys_mcp_w_per_k > 0.0);
        assert_close(
            output.feedback.sum_sys_mcp_t_w,
            output.feedback.sum_sys_mcp_w_per_k * system.minimum_cooling_supply_air_temperature_c,
        );
    }

    #[test]
    fn purchased_air_reads_zone_node_temperature_instead_of_mean_air_temperature() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        assert_eq!(state.mean_air_temperature_c, 22.0);
        let system = test_system();
        let mut input = coupling_input(&mut state, &system, 1, 1);
        input.zone_node_temperature_c = 21.0;
        input.recirculation_state.air_temperature_c = 21.0;

        let output = couple_direct_zone_predicted_demand_to_purchased_air(input)
            .expect("bounded heating coupling with distinct zone-node temperature");

        assert_eq!(
            output.purchased_air.trace.zone_state.air_temperature_c,
            21.0
        );
        assert_eq!(
            output
                .purchased_air
                .trace
                .recirculation_state
                .air_temperature_c,
            21.0
        );
        assert_close(
            output.feedback.multiplied_supply_mass_flow_rate_kg_per_s,
            2_000.0
                / (output.feedback.cp_air_j_per_kg_k
                    * (system.maximum_heating_supply_air_temperature_c - 21.0)),
        );
    }

    #[test]
    fn deadband_prediction_overwrites_stale_system_air_sums_with_exact_zeros() {
        let mut state = zone_state_for_temp_independent_load(2_200.0);
        state.sum_sys_mcp_w_per_k = 12.0;
        state.sum_sys_mcp_t_w = 420.0;
        let lagged_load = state.system_dependent_zone_loads_lagged_w;
        let system = test_system();

        let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect("bounded deadband coupling");

        assert_eq!(
            output.purchased_air.calculation.mode,
            IdealLoadsSensibleMode::Deadband
        );
        assert_eq!(
            output.feedback.multiplied_supply_mass_flow_rate_kg_per_s,
            0.0
        );
        assert_eq!(output.feedback.sum_sys_mcp_w_per_k, 0.0);
        assert_eq!(output.feedback.sum_sys_mcp_t_w, 0.0);
        assert_eq!(state.sum_sys_mcp_w_per_k, 0.0);
        assert_eq!(state.sum_sys_mcp_t_w, 0.0);
        assert_eq!(
            state.system_dependent_zone_loads_lagged_w, lagged_load,
            "the coupling must never advance the lagged-load owner"
        );
    }

    #[test]
    fn multiplied_purchased_air_flow_is_divided_exactly_once_for_zone_feedback() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        let system = test_system();

        let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 2, 3,
        ))
        .expect("bounded multiplied heating coupling");

        assert_eq!(output.feedback.multiplier_product, 6.0);
        assert_close(
            output.prediction.predicted_loads.total_output_required_w,
            12_000.0,
        );
        assert_close(
            output.feedback.zone_supply_mass_flow_rate_kg_per_s,
            output.feedback.multiplied_supply_mass_flow_rate_kg_per_s / 6.0,
        );
        assert_close(
            output.feedback.sum_sys_mcp_w_per_k,
            output.feedback.multiplied_supply_mass_flow_rate_kg_per_s
                * output.feedback.cp_air_j_per_kg_k
                / 6.0,
        );
        assert_close(
            output.feedback.sum_sys_mcp_t_w,
            output.feedback.multiplied_supply_mass_flow_rate_kg_per_s
                * output.feedback.cp_air_j_per_kg_k
                * output.feedback.supply_temperature_c
                / 6.0,
        );
        assert_close(
            output.feedback.sum_sys_mcp_w_per_k,
            2_000.0 / (system.maximum_heating_supply_air_temperature_c - 22.0),
        );
        assert_close(
            output.feedback.sum_sys_mcp_t_w,
            output.feedback.sum_sys_mcp_w_per_k * system.maximum_heating_supply_air_temperature_c,
        );
    }

    #[test]
    fn predictor_error_leaves_entire_zone_state_unchanged() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        state.sum_sys_mcp_w_per_k = 17.0;
        state.sum_sys_mcp_t_w = 23.0;
        let original = state.clone();
        let system = test_system();
        let mut input = coupling_input(&mut state, &system, 1, 1);
        input.system_timestep_seconds = 0.0;

        let error = couple_direct_zone_predicted_demand_to_purchased_air(input)
            .expect_err("zero timestep must fail before commit");

        assert!(matches!(
            error,
            DirectZonePurchasedAirCouplingError::Prediction(
                DirectZoneDualSetpointThirdOrderDemandError::PredictorTerms(_)
            )
        ));
        assert_eq!(state, original);
    }

    #[test]
    fn nonfinite_zone_node_is_rejected_in_heating_before_any_state_commit() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        let original = state.clone();
        let system = test_system();
        let mut input = coupling_input(&mut state, &system, 1, 1);
        input.zone_node_temperature_c = f64::NAN;

        let error = couple_direct_zone_predicted_demand_to_purchased_air(input)
            .expect_err("live Zone node temperature must be finite in every mode");

        assert_eq!(
            error,
            DirectZonePurchasedAirCouplingError::InputNotFinite {
                field: "zone_node_temperature_c"
            }
        );
        assert_eq!(state, original);
    }

    #[test]
    fn feedback_validation_error_leaves_entire_zone_state_unchanged() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        state.air_humidity_ratio = f64::MAX;
        state.sum_sys_mcp_w_per_k = 31.0;
        state.sum_sys_mcp_t_w = 37.0;
        let original = state.clone();
        let system = test_system();

        let error = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect_err("overflowing moist-air specific heat must fail before commit");

        assert!(matches!(
            error,
            DirectZonePurchasedAirCouplingError::ResultNotFinite {
                field: "purchased_air.calculation.cp_air_j_per_kg_k"
            }
        ));
        assert_eq!(state, original);
    }

    #[test]
    fn feedback_rejects_invalid_supply_node_payload() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        let system = test_system();
        let purchased_air = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect("valid PurchasedAir output")
        .purchased_air;

        let mut negative_humidity = purchased_air;
        negative_humidity.supply_node_update.humidity_ratio = -0.001;
        assert_eq!(
            derive_system_air_feedback(negative_humidity, 1, 1),
            Err(DirectZonePurchasedAirCouplingError::ResultNegative {
                field: "purchased_air.supply_node_update.humidity_ratio",
                value: -0.001,
            })
        );

        let mut nonfinite_enthalpy = purchased_air;
        nonfinite_enthalpy.supply_node_update.enthalpy_j_per_kg = f64::NAN;
        assert_eq!(
            derive_system_air_feedback(nonfinite_enthalpy, 1, 1),
            Err(DirectZonePurchasedAirCouplingError::ResultNotFinite {
                field: "purchased_air.supply_node_update.enthalpy_j_per_kg",
            })
        );
    }

    #[test]
    fn purchased_air_error_leaves_entire_zone_state_unchanged() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        state.sum_sys_mcp_w_per_k = 47.0;
        state.sum_sys_mcp_t_w = 53.0;
        let original = state.clone();
        let mut system = test_system();
        system.heat_recovery_type = HeatRecoveryType::Sensible;
        assert_eq!(
            select_purchased_air_branch(&system),
            IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible
        );

        let error = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect_err("unsupported heat recovery must fail inside PurchasedAir");

        assert_eq!(
            error,
            DirectZonePurchasedAirCouplingError::PurchasedAir(SimPurchasedAirCompatError {
                system_id: system.id,
                unsupported_features: vec![IdealLoadsUnsupportedFeature::HeatRecovery],
            })
        );
        assert_eq!(state, original);
    }

    #[test]
    fn invalid_finite_hard_size_fails_at_the_generic_boundary_without_commit() {
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            let mut state = zone_state_for_temp_independent_load(0.0);
            state.sum_sys_mcp_w_per_k = 47.0;
            state.sum_sys_mcp_t_w = 53.0;
            let original = state.clone();
            let mut system = test_system();
            let initialization = initialized_snapshot(&system);
            system.heating_limit = IdealLoadsLimit::LimitCapacity;
            system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(value));

            let error = couple_direct_zone_predicted_demand_to_purchased_air(
                coupling_input_with_initialization(&mut state, &system, 1, 1, initialization),
            )
            .expect_err("invalid hard size must fail inside generic PurchasedAir");

            assert_eq!(
                error,
                DirectZonePurchasedAirCouplingError::PurchasedAir(SimPurchasedAirCompatError {
                    system_id: system.id,
                    unsupported_features: vec![
                        IdealLoadsUnsupportedFeature::UnresolvedHeatingLimit
                    ],
                })
            );
            assert_eq!(state, original);
        }
    }

    #[test]
    fn source_order_mcp_overflow_fails_before_multiplier_division_and_commit() {
        let mut state = zone_state_for_temp_independent_load(-2.5e298);
        state.sum_sys_mcp_w_per_k = 39.0;
        state.sum_sys_mcp_t_w = 41.0;
        let original = state.clone();
        let mut system = test_system();
        system.maximum_heating_supply_air_temperature_c = 22.002;

        let error = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 46_340, 46_340,
        ))
        .expect_err("source m-dot-times-cp arithmetic must overflow before scaling");

        assert_eq!(
            error,
            DirectZonePurchasedAirCouplingError::ResultNotFinite {
                field: "multiplied_sum_sys_mcp_w_per_k"
            }
        );
        assert_eq!(state, original);
    }

    #[test]
    fn finite_capacity_branch_uses_predicted_demand_and_commits_limited_feedback() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        let mut system = test_system();
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));

        let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect("hard-sized finite-capacity branch");

        assert_eq!(
            output.purchased_air.branch,
            IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity
        );
        assert_eq!(
            output.prediction.zone_demand.sensible_input_kind,
            crate::ZoneSensibleDemandInputKind::SourceSetpointThresholds
        );
        assert_close(
            output.prediction.predicted_loads.total_output_required_w,
            2_000.0,
        );
        assert_close(
            output
                .purchased_air
                .calculation
                .zone_sensible_heating_rate_w,
            1_000.0,
        );
        assert_close(
            state.sum_sys_mcp_w_per_k,
            output.feedback.sum_sys_mcp_w_per_k,
        );
        assert_close(state.sum_sys_mcp_t_w, output.feedback.sum_sys_mcp_t_w);
    }

    #[test]
    fn finite_capacity_uses_the_explicit_bound_return_projection() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        let mut system = test_system();
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));
        let mut input = coupling_input(&mut state, &system, 1, 1);
        input.zone_node_temperature_c = 21.0;
        input.recirculation_state = IdealLoadsZoneState {
            air_temperature_c: 18.0,
            air_humidity_ratio: 0.006,
        };

        let output = couple_direct_zone_predicted_demand_to_purchased_air(input)
            .expect("finite capacity with a distinct explicit return projection");

        assert_eq!(
            output.purchased_air.trace.zone_state.air_temperature_c,
            21.0
        );
        assert_eq!(
            output.purchased_air.trace.recirculation_state,
            IdealLoadsZoneState {
                air_temperature_c: 18.0,
                air_humidity_ratio: 0.006,
            }
        );
        let return_cp_air_j_per_kg_k = energyplus_moist_air_specific_heat_j_per_kg_k(0.006);
        assert_close(
            output.purchased_air.calculation.supply_temperature_c,
            18.0 + 1_000.0
                / (return_cp_air_j_per_kg_k
                    * output
                        .purchased_air
                        .calculation
                        .supply_mass_flow_rate_kg_per_s),
        );
        assert_close(
            output.feedback.supply_temperature_c,
            output.purchased_air.calculation.supply_temperature_c,
        );
    }

    #[test]
    fn finite_flow_and_combined_branches_commit_final_limited_supply_feedback() {
        for (limit, expected_branch) in [
            (
                IdealLoadsLimit::LimitFlowRate,
                IdealLoadsPurchasedAirBranch::NoOaFiniteFlow,
            ),
            (
                IdealLoadsLimit::LimitFlowRateAndCapacity,
                IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity,
            ),
        ] {
            let mut state = zone_state_for_temp_independent_load(0.0);
            let mut system = test_system();
            system.heating_limit = limit;
            system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.01));
            system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(100.0));

            let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
                &mut state, &system, 1, 1,
            ))
            .expect("hard-sized finite-flow branch");

            assert_eq!(output.purchased_air.branch, expected_branch);
            assert_close(
                output.feedback.multiplied_supply_mass_flow_rate_kg_per_s,
                output
                    .purchased_air
                    .calculation
                    .supply_mass_flow_rate_kg_per_s,
            );
            assert_close(
                output.feedback.sum_sys_mcp_t_w,
                output.feedback.sum_sys_mcp_w_per_k
                    * output.purchased_air.calculation.supply_temperature_c,
            );
            assert!(
                output
                    .purchased_air
                    .calculation
                    .zone_sensible_heating_rate_w
                    < output.prediction.predicted_loads.total_output_required_w
            );
        }
    }

    #[test]
    fn all_finite_branches_limit_state_backed_cooling_and_commit_feedback() {
        for (limit, expected_branch) in [
            (
                IdealLoadsLimit::LimitCapacity,
                IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity,
            ),
            (
                IdealLoadsLimit::LimitFlowRate,
                IdealLoadsPurchasedAirBranch::NoOaFiniteFlow,
            ),
            (
                IdealLoadsLimit::LimitFlowRateAndCapacity,
                IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity,
            ),
        ] {
            let mut state = zone_state_for_temp_independent_load(3_000.0);
            let mut system = test_system();
            system.cooling_limit = limit;
            system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.005));
            system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(200.0));

            let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
                &mut state, &system, 1, 1,
            ))
            .expect("hard-sized finite cooling branch");

            assert_eq!(output.purchased_air.branch, expected_branch);
            assert_eq!(
                output.purchased_air.calculation.mode,
                IdealLoadsSensibleMode::Cooling
            );
            assert_close(
                output
                    .prediction
                    .zone_demand
                    .remaining_output_req_to_cool_sp_w,
                -600.0,
            );
            assert!(
                output
                    .purchased_air
                    .calculation
                    .zone_sensible_cooling_rate_w
                    > 0.0
            );
            assert!(
                output
                    .purchased_air
                    .calculation
                    .zone_sensible_cooling_rate_w
                    < 600.0
            );
            assert_close(
                output.feedback.sum_sys_mcp_t_w,
                output.feedback.sum_sys_mcp_w_per_k
                    * output.purchased_air.calculation.supply_temperature_c,
            );
            assert_close(
                state.sum_sys_mcp_w_per_k,
                output.feedback.sum_sys_mcp_w_per_k,
            );
        }
    }

    #[test]
    fn zero_finite_capacity_clears_stale_system_air_feedback() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        state.sum_sys_mcp_w_per_k = 41.0;
        state.sum_sys_mcp_t_w = 43.0;
        let mut system = test_system();
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(0.0));

        let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect("zero hard-sized capacity remains a supported finite branch");

        assert_eq!(
            output.purchased_air.branch,
            IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity
        );
        assert_eq!(output.feedback.sum_sys_mcp_w_per_k, 0.0);
        assert_eq!(output.feedback.sum_sys_mcp_t_w, 0.0);
        assert_eq!(state.sum_sys_mcp_w_per_k, 0.0);
        assert_eq!(state.sum_sys_mcp_t_w, 0.0);
    }

    #[test]
    fn zero_flow_limit_preserves_the_source_positive_only_clamp() {
        for (limit, capacity, expected_heating_rate_w) in [
            (IdealLoadsLimit::LimitFlowRate, None, 2_000.0),
            (
                IdealLoadsLimit::LimitFlowRateAndCapacity,
                Some(AutosizeOrNumber::Value(1_000.0)),
                1_000.0,
            ),
        ] {
            let mut state = zone_state_for_temp_independent_load(0.0);
            let mut system = test_system();
            system.heating_limit = limit;
            system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.0));
            system.maximum_sensible_heating_capacity_w = capacity;

            let output = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
                &mut state, &system, 1, 1,
            ))
            .expect("zero hard-sized flow retains the source positive-only clamp behavior");

            assert!(
                output
                    .purchased_air
                    .calculation
                    .supply_mass_flow_rate_kg_per_s
                    > 0.0,
                "EnergyPlus applies the finite-flow clamp only when the maximum is positive"
            );
            assert_close(
                output
                    .purchased_air
                    .calculation
                    .zone_sensible_heating_rate_w,
                expected_heating_rate_w,
            );
            assert!(output.feedback.sum_sys_mcp_w_per_k > 0.0);
        }
    }

    #[test]
    fn unsupported_humidity_branch_leaves_entire_zone_state_unchanged() {
        let mut state = zone_state_for_temp_independent_load(0.0);
        state.sum_sys_mcp_w_per_k = 41.0;
        state.sum_sys_mcp_t_w = 43.0;
        let original = state.clone();
        let mut system = test_system();
        system.dehumidification_control_type =
            DehumidificationControlType::ConstantSensibleHeatRatio;

        let error = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut state, &system, 1, 1,
        ))
        .expect_err("humidity-selected branch remains outside direct coupling");

        assert_eq!(
            error,
            DirectZonePurchasedAirCouplingError::UnsupportedBranch {
                branch: IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling,
            }
        );
        assert_eq!(state, original);

        let mut finite_state = zone_state_for_temp_independent_load(0.0);
        let finite_original = finite_state.clone();
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));
        let error = couple_direct_zone_predicted_demand_to_purchased_air(coupling_input(
            &mut finite_state,
            &system,
            1,
            1,
        ))
        .expect_err("finite branch selection must not hide humidity controls");
        assert_eq!(
            error,
            DirectZonePurchasedAirCouplingError::UnsupportedBranch {
                branch: IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity,
            }
        );
        assert_eq!(finite_state, finite_original);
    }

    fn coupling_input<'a>(
        zone_state: &'a mut ZoneHeatBalanceState,
        system: &'a IdealLoadsAirSystem,
        zone_multiplier: u32,
        zone_list_multiplier: u32,
    ) -> DirectZonePurchasedAirCouplingInput<'a> {
        let initialization = initialized_snapshot(system);
        coupling_input_with_initialization(
            zone_state,
            system,
            zone_multiplier,
            zone_list_multiplier,
            initialization,
        )
    }

    fn coupling_input_with_initialization<'a>(
        zone_state: &'a mut ZoneHeatBalanceState,
        system: &'a IdealLoadsAirSystem,
        zone_multiplier: u32,
        zone_list_multiplier: u32,
        initialization: PurchasedAirInitSnapshot,
    ) -> DirectZonePurchasedAirCouplingInput<'a> {
        let air_humidity_ratio = zone_state.air_humidity_ratio;
        DirectZonePurchasedAirCouplingInput {
            zone_state,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 24.0,
            zone_node_temperature_c: 22.0,
            recirculation_state: IdealLoadsZoneState {
                air_temperature_c: 22.0,
                air_humidity_ratio,
            },
            load_correction_factor: 1.0,
            zone_multiplier,
            zone_list_multiplier,
            system_timestep_seconds: 600.0,
            system,
            supply_node: NodeId(3),
            unit_available: true,
            limit_context: IdealLoadsSensibleLimitContext::default(),
            initialization,
        }
    }

    fn initialized_snapshot(system: &IdealLoadsAirSystem) -> PurchasedAirInitSnapshot {
        let limit_context = IdealLoadsSensibleLimitContext::default();
        let mut state = PurchasedAirRuntimeState::default();
        let manager_plan =
            PurchasedAirInitManagerPlan::try_from_rows(vec![PurchasedAirInitManagerPlanRow {
                system: system.id,
                first_matching_equipment_list: Some(ZoneEquipmentListId(0)),
                return_plenum_active: false,
            }])
            .expect("test manager plan must be valid");
        init_purchased_air_runtime(
            &mut state,
            &manager_plan,
            PurchasedAirInitBoundTopology {
                system: system.id,
                controlled_zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                supply_node: NodeId(3),
                recirculation_node: NodeId(4),
            },
            system,
            PurchasedAirInitCallContext {
                zone_equipment_inputs_filled: true,
                system_sizing_calculation: false,
                begin_environment: true,
                standard_air_density_kg_per_m3: limit_context.standard_air_density_kg_per_m3,
                heating_setpoint_c: 20.0,
                cooling_setpoint_c: 24.0,
                overall_availability: 1.0,
                heating_availability: 1.0,
                cooling_availability: 1.0,
            },
        )
        .expect("test system must initialize")
    }

    fn zone_state_for_temp_independent_load(temp_independent_load_w: f64) -> ZoneHeatBalanceState {
        ZoneHeatBalanceState {
            zone_id: ZoneId(0),
            zone_name: "ZONE ONE".to_string(),
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
            opaque_surface_conductance_w_per_k: 100.0,
            opaque_surface_heat_gain_w: 0.0,
            opaque_surface_outside_conduction_w: 0.0,
            sum_ha_w_per_k: 100.0,
            sum_hat_surf_w: temp_independent_load_w,
            sum_hat_ref_w: 0.0,
            sum_mcp_w_per_k: 0.0,
            sum_mcp_t_w: 0.0,
            sum_sys_mcp_w_per_k: 7.0,
            sum_sys_mcp_t_w: 11.0,
            system_dependent_zone_loads_lagged_w: 0.0,
            zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
            system_timestep_average_surface_convection_report_w: None,
            system_timestep_average_air_storage_report_w: None,
        }
    }

    fn test_system() -> IdealLoadsAirSystem {
        IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("ZONE ONE IDEAL LOADS"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("ZONE ONE INLET"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::None,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        }
    }

    fn assert_only_system_air_sums_changed(
        original: &ZoneHeatBalanceState,
        actual: &ZoneHeatBalanceState,
    ) {
        let mut expected = original.clone();
        expected.sum_sys_mcp_w_per_k = actual.sum_sys_mcp_w_per_k;
        expected.sum_sys_mcp_t_w = actual.sum_sys_mcp_t_w;
        assert_eq!(*actual, expected);
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= ABS_TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }
}
