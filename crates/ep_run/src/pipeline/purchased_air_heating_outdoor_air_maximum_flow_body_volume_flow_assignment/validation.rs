//! Fail-closed validation for CP436 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as PredecessorState,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] = &[
    "read-cp435-retained-outdoor-air-mass-flow-for-outdoor-air-volume-flow-division",
    "read-environment-standard-air-density-for-outdoor-air-volume-flow-division",
    "calculate-outdoor-air-mass-flow-divided-by-standard-air-density",
    "assign-local-outdoor-air-volume-flow-rate",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp435: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP436 evidence is missing".to_string())?;
    let predecessor = predecessor_cp435
        .ok_or_else(|| "direct-zone IdealLoads CP436 CP435 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP436 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP436 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER
            != ORDER
    {
        return Err("direct-zone IdealLoads CP436 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP436 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP436 controlled Zone is missing".to_string())?;
    let density = init
        .standard_air_density_kg_per_m3
        .filter(|density| density.is_finite() && *density > 0.0)
        .ok_or_else(|| "direct-zone IdealLoads CP436 standard density is invalid".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP436 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP436 CP435 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest, density)
    {
        return Err("direct-zone IdealLoads CP436 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.maximum_heating_flow_body_entry_route_counts
        || state.heating_outdoor_air_volume_flow_assignment_route_counts
            != predecessor.maximum_heating_flow_body_entry_route_counts
    {
        return Err("direct-zone IdealLoads CP436 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        let predecessor_count = state.predecessor_route_counts[index];
        let false_count = state.predecessor_guard_false_fallthrough_route_counts[index];
        let body_count = state.predecessor_guard_body_entry_route_counts[index];
        let assignment_count = state.heating_outdoor_air_volume_flow_assignment_route_counts[index];
        if (!PUBLIC.contains(&index)
            && (predecessor_count != 0
                || false_count != 0
                || body_count != 0
                || assignment_count != 0))
            || body_count != 0
            || assignment_count != body_count
        {
            return Err(format!(
                "direct-zone IdealLoads CP436 route {index} is not public-release-ready"
            ));
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let assignments = checked_sum(&state.heating_outdoor_air_volume_flow_assignment_route_counts)?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP436 inactive partition underflowed".to_string())?;
    let source_sites = assignments
        .checked_mul(4)
        .ok_or_else(|| "direct-zone IdealLoads CP436 source sites overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "assignment_count",
            assignments,
            state.outdoor_air_volume_flow_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "outdoor_air_owner_read_count",
            assignments,
            state.cp435_outdoor_air_mass_flow_rate_owned_read_count,
        ),
        (
            "outdoor_air_read_count",
            assignments,
            state.outdoor_air_mass_flow_rate_for_volume_flow_division_read_count,
        ),
        (
            "density_owner_count",
            assignments,
            state.begin_environment_standard_air_density_owner_count,
        ),
        (
            "density_read_count",
            assignments,
            state.standard_air_density_for_volume_flow_division_read_count,
        ),
        (
            "division_count",
            assignments,
            state.outdoor_air_mass_flow_rate_standard_air_density_division_count,
        ),
        (
            "assignment_write_count",
            assignments,
            state.local_outdoor_air_volume_flow_rate_assignment_write_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp434_supply_humidity_ratio_state_owner_count,
            state.cp435_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp435_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp434_supply_enthalpy_state_owner_count,
            state.cp435_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp435_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp434_supply_temperature_state_owner_count,
            state.cp435_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp435_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "direct-zone IdealLoads CP436 route count overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP436 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_is_structural_public_only_and_has_no_numerical_or_dto_feed() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        for required in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_body_entry_route_counts",
            "heating_outdoor_air_volume_flow_assignment_route_counts",
            "body_count != 0",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "calculation.mode",
            "DirectZonePurchasedAirCouplingInput",
            "private_characterization",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
