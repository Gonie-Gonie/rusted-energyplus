//! Run-summary evidence for the bounded PurchasedAir cooling flow-limit guard.

use ep_model::IdealLoadsLimit;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_fixed_selector_route, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary>,
    predecessor_cp324: Option<
        &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    model_cooling_limit: Option<IdealLoadsLimit>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling flow-limit guard evidence"
            .to_string()
    })?;
    let predecessor_cp324 = predecessor_cp324.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard has no CP324 evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard has no initialization evidence".to_string()
    })?;
    let model_cooling_limit = model_cooling_limit.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard has no model cooling-limit selector"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard has no coupling call count".to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp324.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE
        || predecessor_cp324.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling flow-limit guard provenance is invalid".to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor = &predecessor_cp324.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let transition_partition = checked_add(
        skipped,
        state.cooling_body_entry_count,
        "transition partition",
    )?;
    let active_partition = checked_add(
        state.supply_mass_flow_limit_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "active partition",
    )?;
    for (field, expected, actual) in [
        (
            "transition_count",
            coupling_call_count,
            state.transition_count,
        ),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "cooling_body_entry_count",
            predecessor.cooling_body_entry_count,
            state.cooling_body_entry_count,
        ),
        (
            "active_partition",
            state.cooling_body_entry_count,
            active_partition,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    let maximum = init_lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s;
    if !maximum.is_finite() || maximum < 0.0 {
        return Err(
            "direct-zone IdealLoads cooling flow-limit guard initialization cache is invalid"
                .to_string(),
        );
    }
    validate_source_counters(state, maximum)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard has no latest CP324 snapshot".to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling flow-limit guard has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            expected_system,
            expected_zone,
            maximum,
            coupling_call_count,
        )
        || !snapshot_route_matches_history(state, latest, model_cooling_limit)
    {
        return Err(
            "direct-zone IdealLoads cooling flow-limit guard latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    guard: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    maximum: f64,
    call_count: usize,
) -> bool {
    guard.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE
        && guard.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && guard.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER
        && guard.system == expected_system
        && predecessor.system == expected_system
        && guard.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && guard.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_entered
            == predecessor.predecessor_ems_supply_mass_flow_override_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_skipped == predecessor.body_skipped
        && guard.predecessor_ems_disabled_fallthrough == predecessor.ems_disabled_fallthrough
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.cooling_body_entered == predecessor.cooling_body_entered
        && snapshot_shape(guard, maximum)
}

fn snapshot_route_matches_history(
    state: &ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    latest: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    model_cooling_limit: IdealLoadsLimit,
) -> bool {
    if validate_fixed_selector_route(state, model_cooling_limit).is_err() {
        return false;
    }
    if !latest.cooling_body_entered {
        return true;
    }
    let first = model_cooling_limit == IdealLoadsLimit::LimitFlowRate;
    let read_second = !first;
    let combined = model_cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    latest.first_cooling_limit == Some(model_cooling_limit)
        && latest.cooling_limit_flow_rate == Some(first)
        && latest.second_cooling_limit_read == read_second
        && latest.second_cooling_limit == read_second.then_some(model_cooling_limit)
        && latest.cooling_limit_flow_rate_and_capacity == read_second.then_some(combined)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling flow-limit guard {label} overflowed")
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling flow-limit guard invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    };

    use super::validation::validate_source_counters;
    use super::*;

    fn active_snapshot(
        maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
        cooling_limit: IdealLoadsLimit,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
        let first = cooling_limit == IdealLoadsLimit::LimitFlowRate;
        let read_second = !first;
        let combined = cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
        let selected = first || combined;
        let positive = selected && maximum_cooling_air_mass_flow_rate_kg_per_s > 0.0;
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_ems_supply_mass_flow_override_body_entered: false,
            predecessor_ems_supply_mass_flow_override_body_skipped: true,
            predecessor_ems_disabled_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            first_cooling_limit_read: true,
            first_cooling_limit: Some(cooling_limit),
            cooling_limit_flow_rate_comparison_evaluated: true,
            cooling_limit_flow_rate: Some(first),
            second_cooling_limit_read: read_second,
            second_cooling_limit: read_second.then_some(cooling_limit),
            cooling_limit_flow_rate_and_capacity_comparison_evaluated: read_second,
            cooling_limit_flow_rate_and_capacity: read_second.then_some(combined),
            cooling_limit_condition_satisfied: Some(selected),
            maximum_cooling_air_mass_flow_rate_read: selected,
            maximum_cooling_air_mass_flow_rate_kg_per_s: selected
                .then_some(maximum_cooling_air_mass_flow_rate_kg_per_s),
            maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: selected,
            maximum_cooling_air_mass_flow_rate_strictly_positive: selected.then_some(positive),
            supply_mass_flow_limit_body_entered: positive,
            active_guard_false_fallthrough: !positive,
        }
    }

    fn unit_off_snapshot() -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
            parent_call_ordinal: 2,
            unit_body_entered: false,
            predecessor_cooling_body_entered: false,
            predecessor_ems_supply_mass_flow_override_body_skipped: false,
            predecessor_ems_disabled_fallthrough: false,
            unit_off_skipped: true,
            cooling_body_entered: false,
            first_cooling_limit_read: false,
            first_cooling_limit: None,
            cooling_limit_flow_rate_comparison_evaluated: false,
            cooling_limit_flow_rate: None,
            second_cooling_limit_read: false,
            second_cooling_limit: None,
            cooling_limit_flow_rate_and_capacity_comparison_evaluated: false,
            cooling_limit_flow_rate_and_capacity: None,
            cooling_limit_condition_satisfied: None,
            maximum_cooling_air_mass_flow_rate_read: false,
            maximum_cooling_air_mass_flow_rate_kg_per_s: None,
            maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: false,
            maximum_cooling_air_mass_flow_rate_strictly_positive: None,
            supply_mass_flow_limit_body_entered: false,
            active_guard_false_fallthrough: false,
            ..active_snapshot(0.5, IdealLoadsLimit::LimitFlowRateAndCapacity)
        }
    }

    #[test]
    fn maximum_mass_flow_ieee_bits_are_fixed_width_hex_strings() {
        for (maximum, expected) in [(0.0, "0x0000000000000000"), (-0.0, "0x8000000000000000")] {
            let snapshot = active_snapshot(maximum, IdealLoadsLimit::LimitFlowRate);
            let mut state =
                PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(snapshot.system);
            state.latest = Some(snapshot);
            let lifecycle = PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
                state,
            };

            let value = lifecycle_json(&lifecycle);
            let bits = &value["latest"]["maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits"];
            assert_eq!(bits, expected);
            assert_eq!(bits.as_str().map(str::len), Some(18));
        }
    }

    #[test]
    fn mixed_active_then_skip_rejects_coordinated_selector_counter_corruption() {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 2;
        state.cooling_body_entry_count = 1;
        state.unit_off_skip_count = 1;
        state.first_cooling_limit_read_count = 1;
        state.cooling_limit_flow_rate_comparison_count = 1;
        state.second_cooling_limit_read_count = 1;
        state.cooling_limit_flow_rate_and_capacity_comparison_count = 1;
        state.cooling_limit_flow_rate_and_capacity_match_count = 1;
        state.maximum_cooling_air_mass_flow_rate_read_count = 1;
        state.maximum_cooling_air_mass_flow_rate_positive_comparison_count = 1;
        state.maximum_cooling_air_mass_flow_rate_strictly_positive_count = 1;
        state.supply_mass_flow_limit_body_entry_count = 1;
        let latest = unit_off_snapshot();
        state.latest = Some(latest);

        assert!(
            validate_source_counters(&state, 0.5).is_ok(),
            "coordinated aggregate corruption otherwise remains internally consistent"
        );
        assert!(snapshot_route_matches_history(
            &state,
            &latest,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
        ));
        assert!(!snapshot_route_matches_history(
            &state,
            &latest,
            IdealLoadsLimit::LimitFlowRate,
        ));
    }
}
