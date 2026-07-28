//! Run-summary evidence for the bounded cooling positive-supply temperature assignment.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcEntryLifecycleSummary, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary>,
    predecessor_cp331: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    >,
    entry_cp310: Option<&PurchasedAirCalcEntryLifecycleSummary>,
    sensible_flow_cp318: Option<&PurchasedAirCalcCoolingSensibleFlowLifecycleSummary>,
    mixed_air_cp329: Option<&PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    positive_guard_cp330: Option<
        &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling positive-supply temperature assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp331.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no CP331 evidence"
            .to_string()
    })?;
    let entry = entry_cp310.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no CP310 evidence"
            .to_string()
    })?;
    let sensible_flow = sensible_flow_cp318.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no CP318 evidence"
            .to_string()
    })?;
    let mixed_air = mixed_air_cp329.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no CP329 evidence"
            .to_string()
    })?;
    let positive_guard = positive_guard_cp330.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no CP330 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply temperature assignment provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let skipped = checked_add(
        skipped,
        state.positive_guard_false_fallthrough_skip_count,
        "guard-false partition",
    )?;
    let transition_partition = checked_add(
        skipped,
        state.supply_temperature_assignment_count,
        "transition partition",
    )?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "unit_off_skip_count",
            predecessor_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor_state.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor_state.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "supply_temperature_assignment_count",
            predecessor_state.cp_air_assignment_count,
            state.supply_temperature_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no latest CP331 snapshot"
            .to_string()
    })?;
    let entry_latest = entry.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no latest CP310 snapshot"
            .to_string()
    })?;
    let sensible_flow_latest = sensible_flow.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no latest CP318 snapshot"
            .to_string()
    })?;
    let mixed_air_latest = mixed_air.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no latest CP329 snapshot"
            .to_string()
    })?;
    let positive_guard_latest = positive_guard.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no latest CP330 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply temperature assignment has no controlled Zone"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || entry.state.system != expected_system
        || sensible_flow.state.system != expected_system
        || mixed_air.state.system != expected_system
        || positive_guard.state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            entry_latest,
            sensible_flow_latest,
            mixed_air_latest,
            positive_guard_latest,
            expected_system,
            expected_zone,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply temperature assignment latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn latest_matches_release(
    assignment: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    entry: &ep_runtime::PurchasedAirCalcEntrySnapshot,
    sensible_flow: &ep_runtime::PurchasedAirCalcCoolingSensibleFlowSnapshot,
    mixed_air: &ep_runtime::PurchasedAirCalcCoolingMixedAirCallSnapshot,
    positive_guard: &ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    assignment.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        && assignment.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && assignment.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER
        && assignment.system == expected_system
        && predecessor.system == expected_system
        && entry.system == expected_system
        && sensible_flow.system == expected_system
        && mixed_air.system == expected_system
        && positive_guard.system == expected_system
        && assignment.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && entry.call_ordinal == call_count
        && sensible_flow.parent_call_ordinal == call_count
        && mixed_air.parent_call_ordinal == call_count
        && positive_guard.parent_call_ordinal == call_count
        && assignment.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && entry.controlled_zone == expected_zone
        && sensible_flow.controlled_zone == expected_zone
        && mixed_air.controlled_zone == expected_zone
        && positive_guard.controlled_zone == expected_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot_shape(
            assignment,
            predecessor,
            entry,
            sensible_flow,
            mixed_air,
            positive_guard,
        )
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply temperature assignment {label} overflowed"
        )
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-supply temperature assignment invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState;

    use super::*;

    fn lifecycle_with_values(
        values: [f64; 8],
    ) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary {
        let snapshot = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            predecessor_active_guard_false_fallthrough: false,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            supply_temperature_assignment_executed: true,
            zone_cooling_setpoint_load_read: true,
            zone_cooling_setpoint_load_w: Some(values[0]),
            cp_air_read: true,
            cp_air_j_per_kg_k: Some(values[1]),
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(values[2]),
            cp_air_times_supply_mass_flow_rate_calculated: true,
            cp_air_times_supply_mass_flow_rate_w_per_k: Some(values[3]),
            zone_cooling_setpoint_load_over_denominator_calculated: true,
            zone_cooling_setpoint_load_over_denominator_c: Some(values[4]),
            zone_node_temperature_read: true,
            zone_node_temperature_c: Some(values[5]),
            supply_temperature_calculated: true,
            calculated_supply_temperature_c: Some(values[6]),
            supply_temperature_assigned: true,
            supply_temperature_c: Some(values[7]),
        };
        let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
            snapshot.system,
        );
        state.latest = Some(snapshot);
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    #[test]
    fn json_preserves_every_cp332_ieee_value_bit_pattern() {
        let values = [-0.0, 1005.0, 0.25, 251.25, -4.0, 22.0, 18.0, 18.0];
        let value = lifecycle_json(&lifecycle_with_values(values));
        let latest = &value["latest"];
        for (field, expected) in [
            ("zone_cooling_setpoint_load_w_ieee_bits", values[0]),
            ("cp_air_j_per_kg_k_ieee_bits", values[1]),
            ("supply_mass_flow_rate_kg_per_s_ieee_bits", values[2]),
            (
                "cp_air_times_supply_mass_flow_rate_w_per_k_ieee_bits",
                values[3],
            ),
            (
                "zone_cooling_setpoint_load_over_denominator_c_ieee_bits",
                values[4],
            ),
            ("zone_node_temperature_c_ieee_bits", values[5]),
            ("calculated_supply_temperature_c_ieee_bits", values[6]),
            ("supply_temperature_c_ieee_bits", values[7]),
        ] {
            assert_eq!(latest[field], format!("0x{:016x}", expected.to_bits()));
        }
    }

    #[test]
    fn json_keeps_non_finite_bits_when_raw_value_is_null() {
        let value = lifecycle_json(&lifecycle_with_values([f64::NAN; 8]));
        let latest = &value["latest"];
        assert!(latest["supply_temperature_c"].is_null());
        assert_eq!(
            latest["supply_temperature_c_ieee_bits"],
            format!("0x{:016x}", f64::NAN.to_bits())
        );
    }
}
