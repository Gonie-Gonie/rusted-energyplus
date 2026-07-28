//! Run-summary evidence for the bounded PurchasedAir EMS mass-flow override body.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary>,
    predecessor_cp323: Option<
        &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose EMS mass-flow override body evidence"
            .to_string()
    })?;
    let predecessor_cp323 = predecessor_cp323.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override body has no CP323 evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override body has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override body has no coupling call count".to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE
        || predecessor_cp323.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE
        || predecessor_cp323.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads EMS mass-flow override body provenance is invalid".to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor_cp323.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let route_partition = checked_add(skipped, state.cooling_body_entry_count, "route partition")?;
    let body_partition = checked_add(
        state.body_entry_count,
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
            predecessor_state.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, route_partition),
        ("body_partition", state.transition_count, body_partition),
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
            "cooling_body_entry_count",
            predecessor_state.cooling_body_entry_count,
            state.cooling_body_entry_count,
        ),
        (
            "predecessor_body_entry_count",
            predecessor_state.ems_supply_mass_flow_override_body_entry_count,
            state.body_entry_count,
        ),
        (
            "predecessor_false_fallthrough_count",
            predecessor_state.ems_supply_mass_flow_override_guard_false_fallthrough_count,
            state.ems_disabled_fallthrough_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override body has no latest snapshot".to_string()
    })?;
    let predecessor = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override body has no latest CP323 snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads EMS mass-flow override body has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override body has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor,
            expected_system,
            expected_zone,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads EMS mass-flow override body latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    body: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    body.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE
        && body.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE
        && body.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER
        && body.system == expected_system
        && predecessor.system == expected_system
        && body.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && body.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && body.unit_body_entered == predecessor.unit_body_entered
        && body.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_entered
            == predecessor.ems_supply_mass_flow_override_body_entered
        && body.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
            == predecessor.ems_supply_mass_flow_override_guard_false_fallthrough
        && body.unit_off_skipped == predecessor.unit_off_skipped
        && body.non_cooling_skipped == predecessor.non_cooling_skipped
        && body.cooling_body_entered == predecessor.cooling_body_entered
        && snapshot_shape(body)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads EMS mass-flow override body {label} overflowed")
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads EMS mass-flow override body invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    };

    use super::{lifecycle_json, snapshot_shape};

    fn active_snapshot() -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_ems_supply_mass_flow_override_body_entered: false,
            predecessor_ems_supply_mass_flow_override_guard_false_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            body_skipped: true,
            ems_disabled_fallthrough: true,
            ems_supply_mass_flow_override_value_read: false,
            ems_supply_mass_flow_override_value_kg_per_s: None,
            supply_mass_flow_rate_override_assignment_performed: false,
            assigned_supply_mass_flow_rate_kg_per_s: None,
            outdoor_air_mass_flow_rate_for_minimum_read: false,
            outdoor_air_mass_flow_rate_before_override_kg_per_s: None,
            supply_mass_flow_rate_for_minimum_read: false,
            supply_mass_flow_rate_for_minimum_kg_per_s: None,
            source_shaped_two_argument_minimum_evaluated: false,
            minimum_outdoor_air_mass_flow_rate_kg_per_s: None,
            outdoor_air_mass_flow_rate_assignment_performed: false,
            assigned_outdoor_air_mass_flow_rate_kg_per_s: None,
        }
    }

    fn unit_off_snapshot() -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
            unit_body_entered: false,
            predecessor_cooling_body_entered: false,
            predecessor_ems_supply_mass_flow_override_guard_false_fallthrough: false,
            unit_off_skipped: true,
            cooling_body_entered: false,
            ems_disabled_fallthrough: false,
            ..active_snapshot()
        }
    }

    #[test]
    fn exact_false_guard_skip_shape_and_json_are_stable() {
        let snapshot = active_snapshot();
        assert!(snapshot_shape(&snapshot));
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(snapshot.system);
        state.transition_count = 1;
        state.cooling_body_entry_count = 1;
        state.body_skip_count = 1;
        state.ems_disabled_fallthrough_count = 1;
        state.latest = Some(snapshot);
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );
        assert_eq!(
            value["latest"]["source_order"],
            serde_json::json!([
                "read-ems-supply-mass-flow-override-value",
                "assign-supply-mass-flow-rate-from-ems-override",
                "read-outdoor-air-mass-flow-rate-for-minimum",
                "read-supply-mass-flow-rate-for-minimum",
                "apply-source-shaped-two-argument-minimum",
                "assign-outdoor-air-mass-flow-rate",
            ])
        );
        assert_eq!(value["latest"]["body_skipped"], true);
        assert_eq!(value["latest"]["ems_disabled_fallthrough"], true);
        assert!(value["latest"]["ems_supply_mass_flow_override_value_kg_per_s"].is_null());
    }

    #[test]
    fn any_executed_source_site_is_not_exact_direct_evidence() {
        let mut snapshot = active_snapshot();
        snapshot.body_skipped = false;
        snapshot.ems_supply_mass_flow_override_value_read = true;
        snapshot.ems_supply_mass_flow_override_value_kg_per_s = Some(0.5);
        assert!(!snapshot_shape(&snapshot));
    }

    #[test]
    fn unit_off_skips_all_six_body_sites() {
        let snapshot = unit_off_snapshot();
        assert!(snapshot_shape(&snapshot));
        assert!(snapshot.unit_off_skipped);
        assert!(!snapshot.ems_disabled_fallthrough);
    }
}
