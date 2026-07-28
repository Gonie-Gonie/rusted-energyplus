//! Run-summary evidence for the bounded PurchasedAir EMS mass-flow override guard.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary>,
    predecessor_cp322: Option<&PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose EMS mass-flow override guard evidence"
            .to_string()
    })?;
    let predecessor_cp322 = predecessor_cp322.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override guard has no CP322 evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override guard has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override guard has no coupling call count".to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp322.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        || predecessor_cp322.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads EMS mass-flow override guard provenance is invalid".to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor_cp322.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let partition = checked_add(
        skipped,
        state.cooling_body_entry_count,
        "transition partition",
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
        ("transition_partition", state.transition_count, partition),
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
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override guard has no latest snapshot".to_string()
    })?;
    let predecessor = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override guard has no latest CP322 snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads EMS mass-flow override guard has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads EMS mass-flow override guard has no controlled Zone".to_string()
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
            "direct-zone IdealLoads EMS mass-flow override guard latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    guard: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    guard.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE
        && guard.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE
        && guard.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER
        && guard.system == expected_system
        && predecessor.system == expected_system
        && guard.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && guard.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.cooling_body_entered == predecessor.cooling_body_entered
        && snapshot_shape(guard)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads EMS mass-flow override guard {label} overflowed")
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads EMS mass-flow override guard invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    };

    use super::{lifecycle_json, snapshot_shape};

    fn active_snapshot() -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            ems_supply_mass_flow_override_flag_read: true,
            ems_supply_mass_flow_override_enabled: Some(false),
            ems_supply_mass_flow_override_guard_evaluated: true,
            ems_supply_mass_flow_override_body_entered: false,
            ems_supply_mass_flow_override_guard_false_fallthrough: true,
        }
    }

    fn unit_off_snapshot() -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
            unit_body_entered: false,
            predecessor_cooling_body_entered: false,
            unit_off_skipped: true,
            cooling_body_entered: false,
            ems_supply_mass_flow_override_flag_read: false,
            ems_supply_mass_flow_override_enabled: None,
            ems_supply_mass_flow_override_guard_evaluated: false,
            ems_supply_mass_flow_override_guard_false_fallthrough: false,
            ..active_snapshot()
        }
    }

    #[test]
    fn exact_false_guard_shape_and_json_are_stable() {
        let snapshot = active_snapshot();
        assert!(snapshot_shape(&snapshot));
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(snapshot.system);
        state.transition_count = 1;
        state.cooling_body_entry_count = 1;
        state.ems_supply_mass_flow_override_flag_read_count = 1;
        state.ems_supply_mass_flow_override_guard_evaluation_count = 1;
        state.ems_supply_mass_flow_override_guard_false_fallthrough_count = 1;
        state.latest = Some(snapshot);
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );
        assert_eq!(
            value["latest"]["source_order"],
            serde_json::json!([
                "read-ems-supply-mass-flow-override-flag",
                "evaluate-ems-supply-mass-flow-override-guard",
                "enter-ems-supply-mass-flow-override-body-if-enabled",
            ])
        );
        assert_eq!(
            value["latest"]["ems_supply_mass_flow_override_enabled"],
            false
        );
        assert_eq!(
            value["latest"]["ems_supply_mass_flow_override_guard_false_fallthrough"],
            true
        );
    }

    #[test]
    fn enabled_or_entered_ems_body_is_not_exact_direct_evidence() {
        let mut snapshot = active_snapshot();
        snapshot.ems_supply_mass_flow_override_enabled = Some(true);
        snapshot.ems_supply_mass_flow_override_body_entered = true;
        snapshot.ems_supply_mass_flow_override_guard_false_fallthrough = false;
        assert!(!snapshot_shape(&snapshot));
    }

    #[test]
    fn unit_off_skips_all_three_guard_sites_and_serializes_null_flag() {
        let snapshot = unit_off_snapshot();
        assert!(snapshot_shape(&snapshot));
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(snapshot.system);
        state.transition_count = 1;
        state.unit_off_skip_count = 1;
        state.latest = Some(snapshot);
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );
        assert_eq!(
            value["latest"]["ems_supply_mass_flow_override_flag_read"],
            false
        );
        assert!(value["latest"]["ems_supply_mass_flow_override_enabled"].is_null());
        assert_eq!(
            value["latest"]["ems_supply_mass_flow_override_guard_false_fallthrough"],
            false
        );
    }
}
