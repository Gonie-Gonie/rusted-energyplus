//! Release validation for the bounded cooling-entry gate.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    IdealLoadsSensibleMode, PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER,
    PurchasedAirCalcCoolingEntryGateLifecycleSummary,
    PurchasedAirCalcMinimumOaPrefixLifecycleSummary, PurchasedAirTemperatureControlType,
};

use super::DirectZonePurchasedAirCoupledRuntimeError;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let entry = output.calculation_entry;
    let minimum_oa = output.calculation_minimum_outdoor_air;
    let gate = output.calculation_cooling_entry_gate;
    let common = gate.source == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        && gate.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        && gate.source_order == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER
        && gate.system == binding.ideal_loads_air_system
        && gate.system == minimum_oa.system
        && gate.parent_call_ordinal == call_ordinal
        && gate.parent_call_ordinal == minimum_oa.parent_call_ordinal
        && gate.parent_call_ordinal == entry.call_ordinal
        && gate.controlled_zone == binding.zone
        && gate.controlled_zone == minimum_oa.controlled_zone
        && gate.controlled_zone == entry.controlled_zone
        && gate.unit_body_entered == minimum_oa.unit_body_entered
        && gate.unit_body_entered == entry.unit_body_entered
        && release_wrapper_inputs_match(
            output.schedules.control_type,
            gate.unit_body_entered,
            entry.demand.remaining_output_req_to_cool_sp_w,
        )
        && !gate.single_heat_blocked;
    if !common {
        return false;
    }
    if gate.unit_body_entered {
        let expected_cooling = 0.0 >= entry.demand.remaining_output_req_to_cool_sp_w;
        gate.minimum_outdoor_air_sensible_output_w == Some(0.0)
            && gate.cooling_setpoint_demand_w
                == Some(entry.demand.remaining_output_req_to_cool_sp_w)
            && gate.sensible_comparison_evaluated
            && gate.sensible_comparison_satisfied == Some(expected_cooling)
            && gate.temperature_control_type_read == expected_cooling
            && gate.temperature_control_type
                == expected_cooling.then_some(PurchasedAirTemperatureControlType::DualHeatCool)
            && gate.temperature_control_type_permits_cooling == expected_cooling.then_some(true)
            && gate.cooling_body_entered == expected_cooling
            && gate.assigned_operating_mode
                == expected_cooling.then_some(IdealLoadsSensibleMode::Cooling)
            && numerical_mode_matches_release(
                true,
                expected_cooling,
                output.coupling.purchased_air.calculation.mode,
            )
    } else {
        gate.minimum_outdoor_air_sensible_output_w.is_none()
            && gate.cooling_setpoint_demand_w.is_none()
            && !gate.sensible_comparison_evaluated
            && gate.sensible_comparison_satisfied.is_none()
            && !gate.temperature_control_type_read
            && gate.temperature_control_type.is_none()
            && gate.temperature_control_type_permits_cooling.is_none()
            && !gate.cooling_body_entered
            && gate.assigned_operating_mode.is_none()
            && numerical_mode_matches_release(
                false,
                false,
                output.coupling.purchased_air.calculation.mode,
            )
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingEntryGateLifecycleSummary,
    minimum_oa_lifecycle: &PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), DirectZonePurchasedAirCoupledRuntimeError> {
    let state = &lifecycle.state;
    let minimum_oa_state = &minimum_oa_lifecycle.state;
    let source_skip_partition = checked_partition(
        state.source_execution_count,
        state.unit_off_skip_count,
        "source_skip_partition_overflow",
        timestep_count,
    )?;
    let cooling_fallthrough_partition = checked_partition(
        state.cooling_body_entry_count,
        state.active_fallthrough_count,
        "cooling_fallthrough_partition_overflow",
        state.source_execution_count,
    )?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "minimum_oa_transition_count",
            minimum_oa_state.transition_count,
            state.transition_count,
        ),
        (
            "source_execution_count",
            minimum_oa_state.source_execution_count,
            state.source_execution_count,
        ),
        (
            "unit_off_skip_count",
            minimum_oa_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "sensible_comparison_count",
            state.source_execution_count,
            state.sensible_comparison_count,
        ),
        (
            "satisfied_comparison_read_count",
            state.sensible_comparison_satisfied_count,
            state.temperature_control_type_read_count,
        ),
        (
            "thermostat_read_cooling_entry_count",
            state.temperature_control_type_read_count,
            state.cooling_body_entry_count,
        ),
        ("single_heat_block_count", 0, state.single_heat_block_count),
        (
            "operating_mode_assignment_count",
            state.cooling_body_entry_count,
            state.operating_mode_assignment_count,
        ),
        (
            "numerical_cooling_count",
            numerical_cooling_count,
            state.cooling_body_entry_count,
        ),
        (
            "source_skip_partition",
            timestep_count,
            source_skip_partition,
        ),
        (
            "cooling_fallthrough_partition",
            state.source_execution_count,
            cooling_fallthrough_partition,
        ),
    ] {
        if actual != expected {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEntryGateLifecycleInvariant {
                    field,
                    expected,
                    actual,
                },
            );
        }
    }
    let latest = state.latest.as_ref().ok_or(
        DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEntryGateLifecycleInvariant {
            field: "latest_snapshot_present",
            expected: 1,
            actual: 0,
        },
    )?;
    let ready = lifecycle.source == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        && lifecycle.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        && state.system == binding.ideal_loads_air_system
        && latest == &latest_output.calculation_cooling_entry_gate
        && snapshot_matches_release(latest_output, timestep_count, binding);
    if !ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEntryGateLifecycleInvariant {
                field: "latest_release_snapshot_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    Ok(())
}

pub(super) fn numerical_mode_matches_release(
    unit_body_entered: bool,
    expected_cooling: bool,
    actual: IdealLoadsSensibleMode,
) -> bool {
    if !unit_body_entered {
        actual == IdealLoadsSensibleMode::Off
    } else if expected_cooling {
        actual == IdealLoadsSensibleMode::Cooling
    } else {
        matches!(
            actual,
            IdealLoadsSensibleMode::Heating | IdealLoadsSensibleMode::Deadband
        )
    }
}

pub(super) fn release_wrapper_inputs_match(
    control_type: f64,
    unit_body_entered: bool,
    cooling_setpoint_demand_w: f64,
) -> bool {
    control_type == 4.0 && (!unit_body_entered || cooling_setpoint_demand_w.is_finite())
}

pub(super) fn checked_partition(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, DirectZonePurchasedAirCoupledRuntimeError> {
    left.checked_add(right).ok_or(
        DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEntryGateLifecycleInvariant {
            field,
            expected,
            actual: usize::MAX,
        },
    )
}
