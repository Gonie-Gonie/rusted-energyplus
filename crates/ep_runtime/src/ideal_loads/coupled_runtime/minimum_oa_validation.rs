//! Release validation for the bounded minimum-outdoor-air prefix.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
    PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER, PurchasedAirCalcEntryLifecycleSummary,
    PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
};

use super::DirectZonePurchasedAirCoupledRuntimeError;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let entry = output.calculation_entry;
    let prefix = output.calculation_minimum_outdoor_air;
    let common = prefix.source == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        && prefix.minimum_oa_child_source == PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        && prefix.source_order == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER
        && prefix.system == binding.ideal_loads_air_system
        && prefix.parent_call_ordinal == call_ordinal
        && prefix.parent_call_ordinal == entry.call_ordinal
        && prefix.controlled_zone == binding.zone
        && prefix.controlled_zone == entry.controlled_zone
        && prefix.unit_body_entered == entry.unit_body_entered
        && !prefix.ems_override_applied
        && prefix.psychrometric_call_count == 0;
    if !common {
        return false;
    }
    if prefix.unit_body_entered {
        prefix.zone_heat_balance_reference_bound
            && prefix.minimum_oa_child_called
            && prefix.minimum_oa_child_no_outdoor_air_route
            && prefix.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s == Some(0.0)
            && prefix.retained_minimum_outdoor_air_write_performed
            && prefix.ems_override_flag_read
            && prefix.ems_override_enabled == Some(false)
            && prefix.working_outdoor_air_mass_flow_rate_kg_per_s == Some(0.0)
            && prefix.outdoor_air_flag_read
            && prefix.outdoor_air_enabled == Some(false)
            && prefix.no_outdoor_air_zero_branch_entered
            && prefix.minimum_outdoor_air_sensible_output_w == Some(0.0)
            && prefix.minimum_outdoor_air_moisture_output_kg_per_s == Some(0.0)
    } else {
        !prefix.zone_heat_balance_reference_bound
            && !prefix.minimum_oa_child_called
            && !prefix.minimum_oa_child_no_outdoor_air_route
            && prefix
                .retained_minimum_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !prefix.retained_minimum_outdoor_air_write_performed
            && !prefix.ems_override_flag_read
            && prefix.ems_override_enabled.is_none()
            && prefix.working_outdoor_air_mass_flow_rate_kg_per_s.is_none()
            && !prefix.outdoor_air_flag_read
            && prefix.outdoor_air_enabled.is_none()
            && !prefix.no_outdoor_air_zero_branch_entered
            && prefix.minimum_outdoor_air_sensible_output_w.is_none()
            && prefix
                .minimum_outdoor_air_moisture_output_kg_per_s
                .is_none()
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
    calculation_entry_lifecycle: &PurchasedAirCalcEntryLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), DirectZonePurchasedAirCoupledRuntimeError> {
    let state = &lifecycle.state;
    let entry_state = &calculation_entry_lifecycle.state;
    let source_skip_partition = state
        .source_execution_count
        .checked_add(state.unit_off_skip_count)
        .ok_or(
            DirectZonePurchasedAirCoupledRuntimeError::CalcMinimumOaPrefixLifecycleInvariant {
                field: "source_skip_partition_overflow",
                expected: timestep_count,
                actual: usize::MAX,
            },
        )?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "source_execution_count",
            entry_state.unit_body_entry_count,
            state.source_execution_count,
        ),
        (
            "unit_off_skip_count",
            entry_state.unit_off_count,
            state.unit_off_skip_count,
        ),
        (
            "zone_heat_balance_reference_count",
            state.source_execution_count,
            state.zone_heat_balance_reference_count,
        ),
        (
            "minimum_oa_child_call_count",
            state.source_execution_count,
            state.minimum_oa_child_call_count,
        ),
        (
            "minimum_oa_child_no_outdoor_air_count",
            state.source_execution_count,
            state.minimum_oa_child_no_outdoor_air_count,
        ),
        (
            "retained_minimum_outdoor_air_write_count",
            state.source_execution_count,
            state.retained_minimum_outdoor_air_write_count,
        ),
        (
            "ems_override_flag_read_count",
            state.source_execution_count,
            state.ems_override_flag_read_count,
        ),
        (
            "ems_override_apply_count",
            0,
            state.ems_override_apply_count,
        ),
        (
            "outdoor_air_flag_read_count",
            state.source_execution_count,
            state.outdoor_air_flag_read_count,
        ),
        (
            "outdoor_air_effect_count",
            0,
            state.outdoor_air_effect_count,
        ),
        (
            "no_outdoor_air_zero_branch_count",
            state.source_execution_count,
            state.no_outdoor_air_zero_branch_count,
        ),
        (
            "psychrometric_call_count",
            0,
            state.psychrometric_call_count,
        ),
        (
            "source_skip_partition",
            timestep_count,
            source_skip_partition,
        ),
    ] {
        if actual != expected {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::CalcMinimumOaPrefixLifecycleInvariant {
                    field,
                    expected,
                    actual,
                },
            );
        }
    }
    let latest = state.latest.as_ref().ok_or(
        DirectZonePurchasedAirCoupledRuntimeError::CalcMinimumOaPrefixLifecycleInvariant {
            field: "latest_snapshot_present",
            expected: 1,
            actual: 0,
        },
    )?;
    let ready = lifecycle.source == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        && lifecycle.minimum_oa_child_source == PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        && state.system == binding.ideal_loads_air_system
        && latest == &latest_output.calculation_minimum_outdoor_air
        && snapshot_matches_release(latest_output, timestep_count, binding);
    if !ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcMinimumOaPrefixLifecycleInvariant {
                field: "latest_release_snapshot_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    Ok(())
}
