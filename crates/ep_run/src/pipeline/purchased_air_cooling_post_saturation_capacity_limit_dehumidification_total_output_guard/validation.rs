//! Fail-closed validation for CP383 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary as CapacityLifecycle,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot as CapacitySnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary as CapacityCorroboratorLifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot as CapacityCorroboratorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Snapshot,
    PurchasedAirInitLifecycleSummary,
};

mod counts;
mod snapshot;

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp382: Option<&PredecessorLifecycle>,
    capacity_cp321: Option<&CapacityLifecycle>,
    capacity_corroborator_cp340: Option<&CapacityCorroboratorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP383 total-output guard evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp382
        .ok_or_else(|| "direct-zone IdealLoads CP383 has no CP382 evidence".to_string())?;
    let capacity = capacity_cp321
        .ok_or_else(|| "direct-zone IdealLoads CP383 has no CP321 capacity evidence".to_string())?;
    let corroborator = capacity_corroborator_cp340.ok_or_else(|| {
        "direct-zone IdealLoads CP383 has no CP340 capacity corroboration".to_string()
    })?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP383 has no initialization evidence".to_string())?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP383 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP383 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP383 has no controlled Zone".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || capacity.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || capacity.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || corroborator.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER.len()
            != 4
        || lifecycle.state.system != expected_system
        || predecessor.state.system != expected_system
        || capacity.state.system != expected_system
        || corroborator.state.system != expected_system
    {
        return Err("direct-zone IdealLoads CP383 provenance is invalid".into());
    }

    counts::validate(&lifecycle.state, &predecessor.state, calls)?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP383 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP383 CP382 latest evidence is missing".to_string()
    })?;
    let capacity_latest = capacity.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP383 CP321 latest evidence is missing".to_string()
    })?;
    let corroborator_latest = corroborator.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP383 CP340 latest evidence is missing".to_string()
    })?;
    if !snapshot::metadata_is_exact(
        latest,
        predecessor_latest,
        capacity_latest,
        corroborator_latest,
        expected_system,
        expected_zone,
        calls,
    ) || !snapshot::links_exactly(
        latest,
        predecessor_latest,
        capacity_latest,
        corroborator_latest,
    ) || !counts::latest_route_has_cumulative_evidence(
        &lifecycle.state,
        &predecessor.state,
        latest,
    ) {
        return Err("direct-zone IdealLoads CP383 latest lineage is invalid".into());
    }
    Ok(())
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
