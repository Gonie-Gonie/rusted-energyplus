//! Fail-closed validation for CP379 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary as TemperatureLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as TemperatureState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as TemperatureSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as HumidityLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as HumidityState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as HumiditySnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod counts;
mod snapshot;

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    humidity_predecessor: Option<&HumidityLifecycle>,
    temperature_predecessor: Option<&TemperatureLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP379 post-saturation supply-enthalpy evidence"
            .to_string()
    })?;
    let humidity = humidity_predecessor
        .ok_or_else(|| "direct-zone IdealLoads CP379 has no CP378 humidity evidence".to_string())?;
    let temperature = temperature_predecessor.ok_or_else(|| {
        "direct-zone IdealLoads CP379 has no CP377 temperature evidence".to_string()
    })?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP379 has no initialization evidence".to_string())?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP379 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP379 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP379 has no controlled Zone".to_string())?;
    validate_release_state(
        lifecycle,
        humidity,
        temperature,
        expected_system,
        expected_zone,
        calls,
    )
}

fn validate_release_state(
    lifecycle: &Lifecycle,
    humidity: &HumidityLifecycle,
    temperature: &TemperatureLifecycle,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> Result<(), String> {
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || humidity.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        || humidity.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || temperature.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || temperature.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER.len()
            != 4
        || [
            lifecycle.state.system,
            humidity.state.system,
            temperature.state.system,
        ]
        .into_iter()
        .any(|system| system != expected_system)
    {
        return Err("direct-zone IdealLoads CP379 provenance is invalid".into());
    }
    counts::validate(&lifecycle.state, &humidity.state, &temperature.state, calls)?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP379 latest evidence is missing".to_string())?;
    let humidity_latest = humidity.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP379 CP378 latest evidence is missing".to_string()
    })?;
    let temperature_latest = temperature.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP379 CP377 latest evidence is missing".to_string()
    })?;
    if !snapshot::metadata_is_exact(
        latest,
        humidity_latest,
        temperature_latest,
        expected_system,
        expected_zone,
        calls,
    ) || !snapshot::links_exactly(latest, humidity_latest, temperature_latest)
        || !counts::latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &humidity.state,
            &temperature.state,
            latest,
        )
    {
        return Err("direct-zone IdealLoads CP379 latest lineage is invalid".into());
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
