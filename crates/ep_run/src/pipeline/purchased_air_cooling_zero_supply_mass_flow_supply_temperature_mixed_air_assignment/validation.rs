//! Fail-closed validation for CP427 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] = &[
    "read-retained-mixed-air-temperature-for-zero-supply-mass-flow-supply-temperature-assignment",
    "assign-purchased-air-supply-temperature-from-mixed-air-temperature-for-zero-supply-mass-flow",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp426: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP427 evidence is missing".to_string())?;
    let predecessor = predecessor_cp426
        .ok_or_else(|| "direct-zone IdealLoads CP427 CP426 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP427 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP427 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
            != ORDER
    {
        return Err("direct-zone IdealLoads CP427 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP427 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP427 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP427 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP427 CP426 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP427 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentRuntimeState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
    {
        return Err("direct-zone IdealLoads CP427 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        for values in [
            &state.predecessor_route_counts,
            &state.zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_counts,
        ] {
            if !PUBLIC.contains(&index) && values[index] != 0 {
                return Err(format!(
                    "direct-zone IdealLoads CP427 non-direct route {index} is active"
                ));
            }
        }
        let expected = if index == 2 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(
            state.zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_counts[index],
            expected,
            "assignment_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let assignments = checked_sum(
        &state.zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_counts,
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP427 inactive partition underflowed".to_string())?;
    let sites = assignments
        .checked_mul(2)
        .ok_or_else(|| "direct-zone IdealLoads CP427 site count overflowed".to_string())?;
    let predecessor_humidity_owners = predecessor
        .cp425_supply_humidity_ratio_state_owner_count
        .checked_add(predecessor.cp426_supply_humidity_ratio_state_owner_count)
        .ok_or_else(|| "direct-zone IdealLoads CP427 owner count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        (
            "assignment_count",
            assignments,
            state.zero_supply_mass_flow_supply_temperature_mixed_air_assignment_count,
        ),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor_humidity_owners, state.cp426_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp426_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp425_supply_enthalpy_state_owner_count, state.cp426_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp426_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp425_supply_temperature_state_owner_count, state.cp426_supply_temperature_state_owner_count),
        ("temperature_preservation_count", state.cp426_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
        ("cp427_temperature_owner_count", assignments, state.cp427_supply_temperature_state_owner_count),
        ("mixed_air_owner_read_count", assignments, state.cp329_retained_mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_owned_read_count),
        ("mixed_air_read_count", assignments, state.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_read_count),
        ("assignment_write_count", assignments, state.supply_temperature_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "direct-zone IdealLoads CP427 count overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP427 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::lineage::active_temperature_chain_is_exact;

    #[test]
    fn active_temperature_chain_rejects_each_none_forgery() {
        let value = f64::from_bits(0x4036_0000_0000_0000);
        assert!(active_temperature_chain_is_exact(
            Some(value),
            Some(value),
            Some(value)
        ));
        assert!(!active_temperature_chain_is_exact(
            None,
            Some(value),
            Some(value)
        ));
        assert!(!active_temperature_chain_is_exact(
            Some(value),
            None,
            Some(value)
        ));
        assert!(!active_temperature_chain_is_exact(
            Some(value),
            Some(value),
            None
        ));
    }
}
