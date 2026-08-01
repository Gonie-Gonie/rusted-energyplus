//! Fail-closed validation for CP382 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Snapshot,
    PurchasedAirInitLifecycleSummary,
};

mod counts;
mod snapshot;

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp381: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP382 total-output assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp381
        .ok_or_else(|| "direct-zone IdealLoads CP382 has no CP381 evidence".to_string())?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP382 has no initialization evidence".to_string())?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP382 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP382 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP382 has no controlled Zone".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len()
            != 6
        || lifecycle.state.system != expected_system
        || predecessor.state.system != expected_system
    {
        return Err("direct-zone IdealLoads CP382 provenance is invalid".into());
    }

    counts::validate(&lifecycle.state, &predecessor.state, calls)?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP382 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP382 CP381 latest evidence is missing".to_string()
    })?;
    if !snapshot::metadata_is_exact(
        latest,
        predecessor_latest,
        expected_system,
        expected_zone,
        calls,
    ) || !snapshot::links_exactly(latest, predecessor_latest)
        || !counts::latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            latest,
        )
    {
        return Err("direct-zone IdealLoads CP382 latest lineage is invalid".into());
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
