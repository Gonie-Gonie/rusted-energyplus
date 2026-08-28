//! Cheap coupled validation for CP431 heating-mode-guard evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE,
    PurchasedAirCalcHeatingModeGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingModeGuardRuntimeState as State,
    PurchasedAirCalcHeatingModeGuardSnapshot as Snapshot,
    PurchasedAirCalcHeatingOrNoLoadCaseEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirTemperatureControlType, heating_mode_guard_predecessor_cp430_snapshot,
    heating_mode_guard_snapshots_match_bit_exact,
    heating_or_no_load_case_entry_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_heating_or_no_load_case_entry;
    let snapshot = output.calculation_heating_mode_guard;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_or_no_load_case_entry_snapshots_match_bit_exact(
            heating_mode_guard_predecessor_cp430_snapshot(snapshot),
            predecessor,
        )
        && local_shape_is_exact(snapshot, predecessor, output)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp430: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp430.state;
    if lifecycle.source != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp430.source != PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE
        || predecessor_cp430.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER.len() != 6
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.heating_mode_guard_evaluation_route_counts
            != predecessor.heating_or_no_load_case_entry_route_counts
    {
        return Err(violation(
            "source_predecessor_route_and_system_identity",
            1,
            0,
        ));
    }
    validate_counts(state, predecessor, timestep_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !heating_mode_guard_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_mode_guard,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.heating_mode_guard_evaluation_route_counts,
        &state.heating_operating_mode_body_entry_route_counts,
        &state.heating_mode_guard_false_fallthrough_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let evaluations = state.heating_mode_guard_evaluation_route_counts[index];
        let body = state.heating_operating_mode_body_entry_route_counts[index];
        let fallthrough = state.heating_mode_guard_false_fallthrough_route_counts[index];
        let terminal = body
            .checked_add(fallthrough)
            .ok_or_else(|| violation("active_route_partition_overflow", 0, usize::MAX))?;
        ensure_count(
            terminal,
            if index == 1 { evaluations } else { 0 },
            "active_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let evaluations = checked_sum(&state.heating_mode_guard_evaluation_route_counts)?;
    let bodies = checked_sum(&state.heating_operating_mode_body_entry_route_counts)?;
    let fallthroughs = checked_sum(&state.heating_mode_guard_false_fallthrough_route_counts)?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| violation("inactive_partition_underflow", evaluations, transitions))?;
    let source_sites = evaluations
        .checked_add(bodies)
        .and_then(|count| count.checked_mul(3))
        .ok_or_else(|| violation("source_site_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_evaluation_count", evaluations, state.heating_mode_guard_evaluation_count),
        ("body_entry_count", bodies, state.heating_operating_mode_body_entry_count),
        ("false_fallthrough_count", fallthroughs, state.heating_mode_guard_false_fallthrough_count),
        ("source_site_execution_count", source_sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp429_supply_humidity_ratio_state_owner_count, state.cp430_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp430_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp429_supply_enthalpy_state_owner_count, state.cp430_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp430_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp429_supply_temperature_state_owner_count, state.cp430_supply_temperature_state_owner_count),
        ("temperature_preservation_count", state.cp430_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
        ("cp311_owner_read_count", evaluations, state.cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count),
        ("cp312_corroboration_count", evaluations, state.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count),
        ("minimum_oa_read_count", evaluations, state.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count),
        ("cp310_owner_read_count", evaluations, state.cp310_retained_heating_setpoint_demand_owner_read_count),
        ("heating_demand_read_count", evaluations, state.heating_setpoint_demand_for_heating_mode_guard_read_count),
        ("sensible_comparison_count", evaluations, state.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count),
        ("sensible_true_count", bodies, state.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count),
        ("temperature_type_owner_read_count", bodies, state.prevalidated_temperature_control_type_owner_read_count),
        ("temperature_type_read_count", bodies, state.temperature_control_type_read_after_sensible_comparison_short_circuit_count),
        ("single_cool_comparison_count", bodies, state.temperature_control_type_single_cool_comparison_count),
        ("permits_heating_count", bodies, state.temperature_control_type_permits_heating_count),
        ("single_cool_block_count", 0, state.single_cool_block_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> bool {
    let common = provenance_is_exact(
        snapshot.source,
        snapshot.first_excluded_source,
        snapshot.source_order,
    ) && snapshot.heating_or_no_load_case_entered
        == predecessor.heating_or_no_load_case_entered
        && snapshot.cp430_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp430_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp430_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && same(
            snapshot.predecessor_cp430_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp430_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp430_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && same(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
    common
        && if predecessor.heating_or_no_load_case_entered {
            active_shape_is_exact(snapshot, output)
        } else {
            inactive_shape_is_exact(snapshot)
        }
}

fn active_shape_is_exact(
    snapshot: Snapshot,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> bool {
    snapshot.heating_mode_guard_evaluated
        && snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read
        && snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated
        && snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read
        && same(
            snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w,
            output
                .calculation_minimum_outdoor_air
                .minimum_outdoor_air_sensible_output_w,
        )
        && same(
            snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w,
            output
                .calculation_cooling_entry_gate
                .minimum_outdoor_air_sensible_output_w,
        )
        && snapshot.cp310_retained_heating_setpoint_demand_owned_read
        && snapshot.heating_setpoint_demand_for_heating_mode_guard_read
        && same(
            snapshot.heating_setpoint_demand_for_heating_mode_guard_w,
            Some(
                output
                    .calculation_entry
                    .demand
                    .remaining_output_req_to_heat_sp_w,
            ),
        )
        && snapshot.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated
        && direct_guard_result_is_exact(snapshot, output)
}

fn inactive_shape_is_exact(snapshot: Snapshot) -> bool {
    !snapshot.heating_mode_guard_evaluated
        && !snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read
        && !snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated
        && !snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read
        && snapshot
            .minimum_outdoor_air_sensible_output_for_heating_mode_guard_w
            .is_none()
        && !snapshot.cp310_retained_heating_setpoint_demand_owned_read
        && !snapshot.heating_setpoint_demand_for_heating_mode_guard_read
        && snapshot
            .heating_setpoint_demand_for_heating_mode_guard_w
            .is_none()
        && !snapshot
            .minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated
        && snapshot
            .minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand
            .is_none()
        && !snapshot.prevalidated_temperature_control_type_owned_read
        && !snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit
        && snapshot.temperature_control_type.is_none()
        && !snapshot.temperature_control_type_single_cool_comparison_evaluated
        && snapshot.temperature_control_type_permits_heating.is_none()
        && !snapshot.single_cool_blocked
        && !snapshot.heating_operating_mode_body_entered
        && !snapshot.heating_mode_guard_false_fallthrough
}

fn direct_guard_result_is_exact(
    snapshot: Snapshot,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> bool {
    let (Some(minimum), Some(demand)) = (
        snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w,
        snapshot.heating_setpoint_demand_for_heating_mode_guard_w,
    ) else {
        return false;
    };
    let sensible = minimum < demand;
    snapshot.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand
        == Some(sensible)
        && output.schedules.control_type.to_bits() == 4.0f64.to_bits()
        && if sensible {
            snapshot.prevalidated_temperature_control_type_owned_read
                && snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit
                && snapshot.temperature_control_type
                    == Some(PurchasedAirTemperatureControlType::DualHeatCool)
                && snapshot.temperature_control_type_single_cool_comparison_evaluated
                && snapshot.temperature_control_type_permits_heating == Some(true)
                && !snapshot.single_cool_blocked
                && snapshot.heating_operating_mode_body_entered
                && !snapshot.heating_mode_guard_false_fallthrough
        } else {
            !snapshot.prevalidated_temperature_control_type_owned_read
                && !snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit
                && snapshot.temperature_control_type.is_none()
                && !snapshot.temperature_control_type_single_cool_comparison_evaluated
                && snapshot.temperature_control_type_permits_heating.is_none()
                && !snapshot.single_cool_blocked
                && !snapshot.heating_operating_mode_body_entered
                && snapshot.heating_mode_guard_false_fallthrough
        }
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE
        && first_excluded_source == PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE
        && source_order == PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("count_overflow", 0, usize::MAX))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcHeatingModeGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use crate::ideal_loads::coupled_output::tests::{scaled_output, test_system};

    use super::{
        PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
        PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE as SOURCE,
        PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER as ORDER, direct_guard_result_is_exact,
        inactive_shape_is_exact, local_shape_is_exact, provenance_is_exact,
    };

    #[test]
    fn snapshot_provenance_rejects_each_coordinated_field_forgery() {
        assert!(provenance_is_exact(SOURCE, EXCLUDED, ORDER));
        assert!(!provenance_is_exact("forged source", EXCLUDED, ORDER));
        assert!(!provenance_is_exact(SOURCE, "forged exclusion", ORDER));
        assert!(!provenance_is_exact(SOURCE, EXCLUDED, &["forged order"]));
    }

    #[test]
    fn direct_guard_accepts_both_deadband_short_circuit_and_dual_heating_body() {
        let mut output = scaled_output(&test_system(), 0, 1.0);
        let predecessor = output.calculation_heating_or_no_load_case_entry;
        assert!(predecessor.heating_or_no_load_case_entered);

        let minimum = output
            .calculation_minimum_outdoor_air
            .minimum_outdoor_air_sensible_output_w
            .expect("direct minimum OA");
        output
            .calculation_entry
            .demand
            .remaining_output_req_to_heat_sp_w = minimum - 1.0;
        let mut deadband = output.calculation_heating_mode_guard;
        set_active_numeric_prefix(
            &mut deadband,
            minimum,
            output
                .calculation_entry
                .demand
                .remaining_output_req_to_heat_sp_w,
        );
        deadband.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand =
            Some(false);
        deadband.prevalidated_temperature_control_type_owned_read = false;
        deadband.temperature_control_type_read_after_sensible_comparison_short_circuit = false;
        deadband.temperature_control_type = None;
        deadband.temperature_control_type_single_cool_comparison_evaluated = false;
        deadband.temperature_control_type_permits_heating = None;
        deadband.single_cool_blocked = false;
        deadband.heating_operating_mode_body_entered = false;
        deadband.heating_mode_guard_false_fallthrough = true;
        assert!(direct_guard_result_is_exact(deadband, &output));
        assert!(local_shape_is_exact(deadband, predecessor, &output));

        output
            .calculation_entry
            .demand
            .remaining_output_req_to_heat_sp_w = minimum + 1.0;
        let mut heating = deadband;
        set_active_numeric_prefix(
            &mut heating,
            minimum,
            output
                .calculation_entry
                .demand
                .remaining_output_req_to_heat_sp_w,
        );
        heating.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand =
            Some(true);
        heating.prevalidated_temperature_control_type_owned_read = true;
        heating.temperature_control_type_read_after_sensible_comparison_short_circuit = true;
        heating.temperature_control_type =
            Some(crate::ideal_loads::PurchasedAirTemperatureControlType::DualHeatCool);
        heating.temperature_control_type_single_cool_comparison_evaluated = true;
        heating.temperature_control_type_permits_heating = Some(true);
        heating.single_cool_blocked = false;
        heating.heating_operating_mode_body_entered = true;
        heating.heating_mode_guard_false_fallthrough = false;
        assert!(direct_guard_result_is_exact(heating, &output));
        assert!(local_shape_is_exact(heating, predecessor, &output));
    }

    #[test]
    #[rustfmt::skip]
    fn inactive_shape_rejects_each_injected_local_carrier_or_flag() {
        let output = scaled_output(&test_system(), 0, 1.0); let mut predecessor = output.calculation_heating_or_no_load_case_entry; predecessor.heating_or_no_load_case_entered = false; let mut exact = output.calculation_heating_mode_guard; exact.heating_or_no_load_case_entered = false; set_inactive_local(&mut exact); assert!(inactive_shape_is_exact(exact)); assert!(local_shape_is_exact(exact, predecessor, &output));
        macro_rules! reject { ($field:ident, $value:expr) => {{ let mut forged = exact; forged.$field = $value; assert!(!inactive_shape_is_exact(forged), stringify!($field)); assert!(!local_shape_is_exact(forged, predecessor, &output), stringify!($field)); }}; }
        reject!(heating_mode_guard_evaluated, true); reject!(cp311_retained_minimum_outdoor_air_sensible_output_owned_read, true); reject!(cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated, true); reject!(minimum_outdoor_air_sensible_output_for_heating_mode_guard_read, true); reject!(minimum_outdoor_air_sensible_output_for_heating_mode_guard_w, Some(0.0)); reject!(cp310_retained_heating_setpoint_demand_owned_read, true); reject!(heating_setpoint_demand_for_heating_mode_guard_read, true); reject!(heating_setpoint_demand_for_heating_mode_guard_w, Some(0.0)); reject!(minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated, true); reject!(minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand, Some(false)); reject!(prevalidated_temperature_control_type_owned_read, true); reject!(temperature_control_type_read_after_sensible_comparison_short_circuit, true); reject!(temperature_control_type, Some(crate::ideal_loads::PurchasedAirTemperatureControlType::DualHeatCool)); reject!(temperature_control_type_single_cool_comparison_evaluated, true); reject!(temperature_control_type_permits_heating, Some(true)); reject!(single_cool_blocked, true); reject!(heating_operating_mode_body_entered, true); reject!(heating_mode_guard_false_fallthrough, true);
    }

    fn set_active_numeric_prefix(
        snapshot: &mut crate::ideal_loads::PurchasedAirCalcHeatingModeGuardSnapshot,
        minimum: f64,
        demand: f64,
    ) {
        snapshot.heating_mode_guard_evaluated = true;
        snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read = true;
        snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated = true;
        snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read = true;
        snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w = Some(minimum);
        snapshot.cp310_retained_heating_setpoint_demand_owned_read = true;
        snapshot.heating_setpoint_demand_for_heating_mode_guard_read = true;
        snapshot.heating_setpoint_demand_for_heating_mode_guard_w = Some(demand);
        snapshot.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated =
            true;
    }

    fn set_inactive_local(
        snapshot: &mut crate::ideal_loads::PurchasedAirCalcHeatingModeGuardSnapshot,
    ) {
        snapshot.heating_mode_guard_evaluated = false;
        snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read = false;
        snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated = false;
        snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read = false;
        snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w = None;
        snapshot.cp310_retained_heating_setpoint_demand_owned_read = false;
        snapshot.heating_setpoint_demand_for_heating_mode_guard_read = false;
        snapshot.heating_setpoint_demand_for_heating_mode_guard_w = None;
        snapshot.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated =
            false;
        snapshot.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand =
            None;
        snapshot.prevalidated_temperature_control_type_owned_read = false;
        snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit = false;
        snapshot.temperature_control_type = None;
        snapshot.temperature_control_type_single_cool_comparison_evaluated = false;
        snapshot.temperature_control_type_permits_heating = None;
        snapshot.single_cool_blocked = false;
        snapshot.heating_operating_mode_body_entered = false;
        snapshot.heating_mode_guard_false_fallthrough = false;
    }

    #[test]
    fn hot_validator_is_bounded_and_keeps_the_numerical_dto_out() {
        let source = include_str!("heating_mode_guard_validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(
                include_str!("heating_mode_guard_validation.rs"),
                |(production, _)| production,
            );
        for required in [
            "predecessor_route_counts",
            "heating_mode_guard_evaluation_route_counts",
            "heating_operating_mode_body_entry_route_counts",
            "heating_mode_guard_false_fallthrough_route_counts",
            "predecessor_cp430_snapshot",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "snapshot_is_exact",
            "private_characterization",
            "DirectZonePurchasedAirCouplingInput",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
