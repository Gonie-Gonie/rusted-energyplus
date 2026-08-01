//! Fail-closed validation for CP381 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState as MixedAirState,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as SupplyCorroboratorLifecycle,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as SupplyCorroboratorState,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as SupplyCorroboratorSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as SupplyOwnerLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as SupplyOwnerState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as SupplyOwnerSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod counts;
mod snapshot;

#[allow(clippy::too_many_arguments)]
pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp380: Option<&PredecessorLifecycle>,
    supply_owner_cp378: Option<&SupplyOwnerLifecycle>,
    supply_corroborator_cp379: Option<&SupplyCorroboratorLifecycle>,
    mixed_air_owner_cp329: Option<&MixedAirLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP381 dehumidification-guard evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp380
        .ok_or_else(|| "direct-zone IdealLoads CP381 has no CP380 evidence".to_string())?;
    let supply_owner = supply_owner_cp378
        .ok_or_else(|| "direct-zone IdealLoads CP381 has no CP378 supply owner".to_string())?;
    let supply_corroborator = supply_corroborator_cp379.ok_or_else(|| {
        "direct-zone IdealLoads CP381 has no CP379 supply corroborator".to_string()
    })?;
    let mixed_air_owner = mixed_air_owner_cp329
        .ok_or_else(|| "direct-zone IdealLoads CP381 has no CP329 mixed-air owner".to_string())?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP381 has no initialization evidence".to_string())?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP381 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP381 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP381 has no controlled Zone".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || supply_owner.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        || supply_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || supply_corroborator.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || supply_corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || mixed_air_owner.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER.len()
            != 4
        || [
            lifecycle.state.system,
            predecessor.state.system,
            supply_owner.state.system,
            supply_corroborator.state.system,
            mixed_air_owner.state.system,
        ]
        .into_iter()
        .any(|system| system != expected_system)
    {
        return Err("direct-zone IdealLoads CP381 provenance is invalid".into());
    }

    counts::validate(
        &lifecycle.state,
        &predecessor.state,
        &supply_owner.state,
        &supply_corroborator.state,
        &mixed_air_owner.state,
        calls,
    )?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP381 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP381 CP380 latest evidence is missing".to_string()
    })?;
    let supply_owner_latest = supply_owner.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP381 CP378 latest evidence is missing".to_string()
    })?;
    let supply_corroborator_latest = supply_corroborator.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP381 CP379 latest evidence is missing".to_string()
    })?;
    let mixed_air_owner_latest = mixed_air_owner.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP381 CP329 latest evidence is missing".to_string()
    })?;

    if !snapshot::metadata_is_exact(
        latest,
        predecessor_latest,
        supply_owner_latest,
        supply_corroborator_latest,
        mixed_air_owner_latest,
        expected_system,
        expected_zone,
        calls,
    ) || !snapshot::links_exactly(
        latest,
        predecessor_latest,
        supply_owner_latest,
        supply_corroborator_latest,
        mixed_air_owner_latest,
    ) || !counts::latest_route_has_cumulative_evidence(
        &lifecycle.state,
        &predecessor.state,
        &supply_owner.state,
        &supply_corroborator.state,
        &mixed_air_owner.state,
        latest,
    ) {
        return Err("direct-zone IdealLoads CP381 latest lineage is invalid".into());
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
