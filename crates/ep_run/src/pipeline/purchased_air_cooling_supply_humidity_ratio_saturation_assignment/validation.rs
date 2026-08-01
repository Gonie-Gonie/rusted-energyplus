//! Fail-closed validation for CP377 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary as Cp344Lifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState as Cp344State,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot as Cp344Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary as Cp334Lifecycle,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot as Cp334Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    PurchasedAirInitLifecycleSummary,
};

mod counts;
mod snapshot;

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) pre_saturation_original_assignment_cp376:
        Option<&'a PredecessorLifecycle>,
    pub(in crate::pipeline) temperature_mixed_air_limit_cp334: Option<&'a Cp334Lifecycle>,
    pub(in crate::pipeline) capacity_temperature_mixed_air_limit_cp344: Option<&'a Cp344Lifecycle>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP377 saturation-assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessors
        .pre_saturation_original_assignment_cp376
        .ok_or_else(|| "direct-zone IdealLoads CP377 has no CP376 evidence".to_string())?;
    let cp334 = predecessors
        .temperature_mixed_air_limit_cp334
        .ok_or_else(|| "direct-zone IdealLoads CP377 has no CP334 owner evidence".to_string())?;
    let cp344 = predecessors
        .capacity_temperature_mixed_air_limit_cp344
        .ok_or_else(|| "direct-zone IdealLoads CP377 has no CP344 owner evidence".to_string())?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP377 has no initialization evidence".to_string())?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP377 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP377 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP377 has no controlled Zone".to_string())?;
    validate_release_state(
        lifecycle,
        predecessor,
        cp334,
        cp344,
        expected_system,
        expected_zone,
        calls,
    )
}

fn validate_release_state(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    cp334: &Cp334Lifecycle,
    cp344: &Cp344Lifecycle,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> Result<(), String> {
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || cp334.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || cp334.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || cp344.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || cp344.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER.len()
            != 4
        || [
            lifecycle.state.system,
            predecessor.state.system,
            cp334.state.system,
            cp344.state.system,
        ]
        .into_iter()
        .any(|system| system != expected_system)
    {
        return Err("direct-zone IdealLoads CP377 provenance is invalid".into());
    }
    counts::validate(&lifecycle.state, &predecessor.state, &cp344.state, calls)?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP377 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP377 CP376 latest evidence is missing".to_string()
    })?;
    let cp334_latest = cp334.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP377 CP334 latest evidence is missing".to_string()
    })?;
    let cp344_latest = cp344.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP377 CP344 latest evidence is missing".to_string()
    })?;
    if !snapshot::metadata_is_exact(
        latest,
        predecessor_latest,
        cp334_latest,
        cp344_latest,
        expected_system,
        expected_zone,
        calls,
    ) || !snapshot::links_exactly(latest, predecessor_latest, cp334_latest, cp344_latest)
        || !counts::latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            predecessor_latest,
        )
    {
        return Err("direct-zone IdealLoads CP377 latest lineage is invalid".into());
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

#[cfg(test)]
mod tests;
