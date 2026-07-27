//! Pure retained-snapshot and counter invariants for the CP313 release.

use ep_model::IdealLoadsLimit;

use super::super::PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState;
use crate::ideal_loads::IdealLoadsSensibleMode;
use crate::ideal_loads::calc::cooling_entry_gate::{
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER, PurchasedAirCalcCoolingEntryGateSnapshot,
    PurchasedAirTemperatureControlType,
};
use crate::ideal_loads::calc::minimum_oa_prefix::{
    PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
    PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER, PurchasedAirCalcMinimumOaPrefixSnapshot,
};

pub(super) fn minimum_oa_snapshot_is_direct_release(
    snapshot: PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        && snapshot.minimum_oa_child_source == PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER;
    let common =
        provenance && !snapshot.ems_override_applied && snapshot.psychrometric_call_count == 0;
    if !common {
        return false;
    }
    if snapshot.unit_body_entered {
        snapshot.zone_heat_balance_reference_bound
            && snapshot.minimum_oa_child_called
            && snapshot.minimum_oa_child_no_outdoor_air_route
            && option_f64_has_bits(
                snapshot.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
                0.0,
            )
            && snapshot.retained_minimum_outdoor_air_write_performed
            && snapshot.ems_override_flag_read
            && snapshot.ems_override_enabled == Some(false)
            && option_f64_has_bits(snapshot.working_outdoor_air_mass_flow_rate_kg_per_s, 0.0)
            && snapshot.outdoor_air_flag_read
            && snapshot.outdoor_air_enabled == Some(false)
            && snapshot.no_outdoor_air_zero_branch_entered
            && option_f64_has_bits(snapshot.minimum_outdoor_air_sensible_output_w, 0.0)
            && option_f64_has_bits(snapshot.minimum_outdoor_air_moisture_output_kg_per_s, 0.0)
    } else {
        !snapshot.zone_heat_balance_reference_bound
            && !snapshot.minimum_oa_child_called
            && !snapshot.minimum_oa_child_no_outdoor_air_route
            && snapshot
                .retained_minimum_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.retained_minimum_outdoor_air_write_performed
            && !snapshot.ems_override_flag_read
            && snapshot.ems_override_enabled.is_none()
            && snapshot
                .working_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.outdoor_air_flag_read
            && snapshot.outdoor_air_enabled.is_none()
            && !snapshot.no_outdoor_air_zero_branch_entered
            && snapshot.minimum_outdoor_air_sensible_output_w.is_none()
            && snapshot
                .minimum_outdoor_air_moisture_output_kg_per_s
                .is_none()
    }
}

pub(super) fn cooling_entry_snapshot_is_direct_release(
    snapshot: PurchasedAirCalcCoolingEntryGateSnapshot,
    cooling_setpoint_demand_w: f64,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER;
    if !provenance {
        return false;
    }
    if !snapshot.unit_body_entered {
        return snapshot.minimum_outdoor_air_sensible_output_w.is_none()
            && snapshot.cooling_setpoint_demand_w.is_none()
            && !snapshot.sensible_comparison_evaluated
            && snapshot.sensible_comparison_satisfied.is_none()
            && !snapshot.temperature_control_type_read
            && snapshot.temperature_control_type.is_none()
            && snapshot.temperature_control_type_permits_cooling.is_none()
            && !snapshot.single_heat_blocked
            && !snapshot.cooling_body_entered
            && snapshot.assigned_operating_mode.is_none();
    }
    if !cooling_setpoint_demand_w.is_finite()
        || !option_f64_has_bits(snapshot.minimum_outdoor_air_sensible_output_w, 0.0)
        || !option_f64_has_bits(
            snapshot.cooling_setpoint_demand_w,
            cooling_setpoint_demand_w,
        )
        || !snapshot.sensible_comparison_evaluated
    {
        return false;
    }
    let admitted = 0.0 >= cooling_setpoint_demand_w;
    snapshot.sensible_comparison_satisfied == Some(admitted)
        && snapshot.temperature_control_type_read == admitted
        && snapshot.temperature_control_type
            == admitted.then_some(PurchasedAirTemperatureControlType::DualHeatCool)
        && snapshot.temperature_control_type_permits_cooling == admitted.then_some(true)
        && !snapshot.single_heat_blocked
        && snapshot.cooling_body_entered == admitted
        && snapshot.assigned_operating_mode == admitted.then_some(IdealLoadsSensibleMode::Cooling)
}

pub(super) fn cooling_oa_max_flow_runtime_state_is_consistent(
    state: &PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let selector_history_matches = match cooling_limit {
        IdealLoadsLimit::LimitFlowRate => {
            state.cooling_limit_flow_rate_match_count == state.source_execution_count
                && state.cooling_limit_flow_rate_and_capacity_comparison_count == 0
                && state.cooling_limit_flow_rate_and_capacity_match_count == 0
        }
        IdealLoadsLimit::LimitFlowRateAndCapacity => {
            state.cooling_limit_flow_rate_match_count == 0
                && state.cooling_limit_flow_rate_and_capacity_comparison_count
                    == state.source_execution_count
                && state.cooling_limit_flow_rate_and_capacity_match_count
                    == state.source_execution_count
        }
        IdealLoadsLimit::NoLimit | IdealLoadsLimit::LimitCapacity => {
            state.cooling_limit_flow_rate_match_count == 0
                && state.cooling_limit_flow_rate_and_capacity_comparison_count
                    == state.source_execution_count
                && state.cooling_limit_flow_rate_and_capacity_match_count == 0
        }
    };
    selector_history_matches
        && state
            .source_execution_count
            .checked_add(state.unit_off_skip_count)
            .and_then(|count| count.checked_add(state.non_cooling_skip_count))
            == Some(state.transition_count)
        && state.cooling_limit_flow_rate_comparison_count == state.source_execution_count
        && state
            .source_execution_count
            .checked_sub(state.cooling_limit_flow_rate_match_count)
            == Some(state.cooling_limit_flow_rate_and_capacity_comparison_count)
        && state.cooling_limit_flow_rate_and_capacity_match_count
            <= state.cooling_limit_flow_rate_and_capacity_comparison_count
        && state
            .cooling_limit_flow_rate_match_count
            .checked_add(state.cooling_limit_flow_rate_and_capacity_match_count)
            == Some(state.outdoor_air_mass_flow_rate_read_count)
        && state.outdoor_air_mass_flow_rate_read_count
            == state.maximum_cooling_air_mass_flow_rate_read_count
        && state.maximum_cooling_air_mass_flow_rate_read_count
            == state.strict_mass_flow_comparison_count
        && state.strict_mass_flow_comparison_satisfied_count
            == state.maximum_cooling_flow_body_entry_count
        && state
            .maximum_cooling_flow_body_entry_count
            .checked_add(state.active_fallthrough_count)
            == Some(state.source_execution_count)
}

pub(super) fn minimum_oa_snapshots_bitwise_equal(
    retained: PurchasedAirCalcMinimumOaPrefixSnapshot,
    supplied: PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    let floats_match = [
        (
            retained.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
            supplied.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        ),
        (
            retained.working_outdoor_air_mass_flow_rate_kg_per_s,
            supplied.working_outdoor_air_mass_flow_rate_kg_per_s,
        ),
        (
            retained.minimum_outdoor_air_sensible_output_w,
            supplied.minimum_outdoor_air_sensible_output_w,
        ),
        (
            retained.minimum_outdoor_air_moisture_output_kg_per_s,
            supplied.minimum_outdoor_air_moisture_output_kg_per_s,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_f64_bits_equal(left, right));
    if !floats_match {
        return false;
    }
    let mut retained_without_floats = retained;
    let mut supplied_without_floats = supplied;
    retained_without_floats.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s = None;
    retained_without_floats.working_outdoor_air_mass_flow_rate_kg_per_s = None;
    retained_without_floats.minimum_outdoor_air_sensible_output_w = None;
    retained_without_floats.minimum_outdoor_air_moisture_output_kg_per_s = None;
    supplied_without_floats.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s = None;
    supplied_without_floats.working_outdoor_air_mass_flow_rate_kg_per_s = None;
    supplied_without_floats.minimum_outdoor_air_sensible_output_w = None;
    supplied_without_floats.minimum_outdoor_air_moisture_output_kg_per_s = None;
    retained_without_floats == supplied_without_floats
}

pub(super) fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

pub(super) fn option_f64_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
