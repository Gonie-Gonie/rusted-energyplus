//! Run-summary evidence for the bounded PurchasedAir cooling flow-limit body.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary>,
    predecessor_cp325: Option<&PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary>,
    supply_source_cp322: Option<&PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling flow-limit body evidence".to_string()
    })?;
    let predecessor_cp325 = predecessor_cp325.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no CP325 evidence".to_string()
    })?;
    let supply_source_cp322 = supply_source_cp322.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no CP322 supply-flow evidence"
            .to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no initialization evidence".to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no coupling call count".to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE
        || predecessor_cp325.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE
        || predecessor_cp325.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || supply_source_cp322.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        || supply_source_cp322.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling flow-limit body provenance is invalid".to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor = &predecessor_cp325.state;
    let source = &supply_source_cp322.state;
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
    let body_partition = checked_add(
        state.supply_mass_flow_limit_body_entry_count,
        state.body_skip_count,
        "body partition",
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
            "source_transition_count",
            source.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        ("body_partition", state.transition_count, body_partition),
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
            "supply_mass_flow_limit_body_entry_count",
            predecessor.supply_mass_flow_limit_body_entry_count,
            state.supply_mass_flow_limit_body_entry_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            predecessor.active_guard_false_fallthrough_count,
            state.active_guard_false_fallthrough_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no latest CP325 snapshot".to_string()
    })?;
    let source_latest = source.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no latest CP322 snapshot".to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling flow-limit body has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit body has no controlled Zone".to_string()
    })?;
    let maximum = init_lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s;
    if !maximum.is_finite() || maximum < 0.0 {
        return Err(
            "direct-zone IdealLoads cooling flow-limit body initialization cache is invalid"
                .to_string(),
        );
    }
    if state.system != expected_system
        || predecessor.system != expected_system
        || source.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            source_latest,
            expected_system,
            expected_zone,
            maximum,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling flow-limit body latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    body: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    source: &PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    maximum: f64,
    call_count: usize,
) -> bool {
    body.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE
        && body.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE
        && body.source_order == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER
        && source.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        && source.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
        && source.source_order == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER
        && body.system == expected_system
        && predecessor.system == expected_system
        && source.system == expected_system
        && body.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && source.parent_call_ordinal == call_count
        && body.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && source.controlled_zone == expected_zone
        && body.unit_body_entered == predecessor.unit_body_entered
        && body.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_entered
            == predecessor.predecessor_ems_supply_mass_flow_override_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_skipped
            == predecessor.predecessor_ems_supply_mass_flow_override_body_skipped
        && body.predecessor_ems_disabled_fallthrough
            == predecessor.predecessor_ems_disabled_fallthrough
        && body.unit_off_skipped == predecessor.unit_off_skipped
        && body.non_cooling_skipped == predecessor.non_cooling_skipped
        && body.cooling_body_entered == predecessor.cooling_body_entered
        && body.supply_mass_flow_limit_body_entered
            == predecessor.supply_mass_flow_limit_body_entered
        && body.active_guard_false_fallthrough == predecessor.active_guard_false_fallthrough
        && source.cooling_body_entered == predecessor.cooling_body_entered
        && snapshot_shape(
            body,
            source.resulting_supply_mass_flow_rate_kg_per_s,
            maximum,
        )
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads cooling flow-limit body {label} overflowed"))
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling flow-limit body invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};

    use super::*;

    fn active_fallthrough_snapshot(
        result: f64,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
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
            supply_mass_flow_limit_body_entered: false,
            body_skipped: true,
            active_guard_false_fallthrough: true,
            supply_mass_flow_rate_for_minimum_read: false,
            supply_mass_flow_rate_before_limit_kg_per_s: None,
            maximum_cooling_air_mass_flow_rate_for_minimum_read: false,
            maximum_cooling_air_mass_flow_rate_kg_per_s: None,
            source_shaped_two_argument_minimum_evaluated: false,
            minimum_supply_mass_flow_rate_kg_per_s: None,
            supply_mass_flow_rate_assignment_performed: false,
            assigned_supply_mass_flow_rate_kg_per_s: None,
            resulting_supply_mass_flow_rate_kg_per_s: Some(result),
        }
    }

    #[test]
    fn resulting_mass_flow_ieee_bits_are_fixed_width_hex_strings() {
        for (result, expected) in [(0.0, "0x0000000000000000"), (-0.0, "0x8000000000000000")] {
            let snapshot = active_fallthrough_snapshot(result);
            let mut state =
                ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState::new(
                    snapshot.system,
                );
            state.latest = Some(snapshot);
            let lifecycle = PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
                state,
            };

            let value = lifecycle_json(&lifecycle);
            let bits = &value["latest"]["resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"];
            assert_eq!(bits, expected);
            assert_eq!(bits.as_str().map(str::len), Some(18));
        }
    }
}
