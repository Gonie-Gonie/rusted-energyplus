//! Fail-closed validation for CP380 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary as SelectorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod counts;
mod snapshot;

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp379: Option<&PredecessorLifecycle>,
    selector_cp337: Option<&SelectorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    model_cooling_limit: Option<IdealLoadsLimit>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP380 post-saturation capacity-limit guard evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp379
        .ok_or_else(|| "direct-zone IdealLoads CP380 has no CP379 evidence".to_string())?;
    let selector = selector_cp337
        .ok_or_else(|| "direct-zone IdealLoads CP380 has no CP337 selector evidence".to_string())?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP380 has no initialization evidence".to_string())?;
    let cooling_limit = model_cooling_limit.ok_or_else(|| {
        "direct-zone IdealLoads CP380 has no typed cooling-limit selector".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP380 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP380 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP380 has no controlled Zone".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER.len()
            != 5
        || lifecycle.state.system != expected_system
        || predecessor.state.system != expected_system
    {
        return Err("direct-zone IdealLoads CP380 provenance is invalid".into());
    }

    counts::validate(&lifecycle.state, &predecessor.state, cooling_limit, calls)?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP380 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP380 CP379 latest evidence is missing".to_string()
    })?;
    let selector_latest = selector.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP380 CP337 latest evidence is missing".to_string()
    })?;
    if !snapshot::metadata_is_exact(
        latest,
        predecessor_latest,
        expected_system,
        expected_zone,
        calls,
    ) || !snapshot::links_exactly(latest, predecessor_latest, selector_latest, cooling_limit)
        || !counts::latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            latest,
        )
    {
        return Err("direct-zone IdealLoads CP380 latest lineage is invalid".into());
    }
    Ok(())
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP380 post-saturation capacity-limit guard {field} expected {expected}, got {actual}"
        ))
    }
}
