//! Bounded committed CP330 supply-mass-flow owner capability.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute as Route,
};
use super::snapshots_match_bit_exact;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingMixedAirCallCommittedSensibleOutputInputs;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirUnitRuntimeState,
};

/// Returns CP330's committed positive supply-mass-flow owner.
pub(in crate::ideal_loads::calc) fn cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate(
    unit: &PurchasedAirUnitRuntimeState,
    witness: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    cp329: PurchasedAirCalcCoolingMixedAirCallCommittedSensibleOutputInputs,
) -> Option<f64> {
    let state = &unit.calc_cooling_supply_mass_flow_positive_guard;
    let latest = state.latest?;
    let supply = latest.supply_mass_flow_rate_kg_per_s?;
    (committed_positive_guard_state_is_consistent(unit, latest, witness)
        && latest.source == SOURCE
        && latest.first_excluded_source == EXCLUDED
        && latest.source_order == ORDER
        && !latest.unit_off_skipped
        && !latest.non_cooling_skipped
        && latest.unit_body_entered
        && latest.predecessor_cooling_call_executed
        && (latest.predecessor_zero_flow_reset_body_entered
            != latest.predecessor_active_guard_false_fallthrough)
        && latest.predecessor_no_outdoor_air_fallback_entered
        && latest.cooling_body_entered
        && latest.supply_mass_flow_rate_read
        && latest.supply_mass_flow_rate_strictly_positive_comparison_evaluated
        && latest.supply_mass_flow_rate_strictly_positive == Some(true)
        && latest.positive_supply_mass_flow_body_entered
        && !latest.active_guard_false_fallthrough
        && supply > 0.0
        && supply.to_bits() == cp329.supply_mass_flow_rate_kg_per_s.to_bits())
    .then_some(supply)
}

fn committed_positive_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    latest: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    witness: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_positive_guard;
    let Some(transition_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_body_entry_count))
    else {
        return false;
    };
    let Some(active_partition) = state
        .positive_supply_mass_flow_body_entry_count
        .checked_add(state.active_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(source_sites) = state
        .cooling_body_entry_count
        .checked_mul(2)
        .and_then(|count| count.checked_add(state.positive_supply_mass_flow_body_entry_count))
    else {
        return false;
    };
    state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count == unit.calc_entry.call_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
                .transition_count
        && transition_partition == state.transition_count
        && active_partition == state.cooling_body_entry_count
        && state.source_site_execution_count == source_sites
        && state.supply_mass_flow_rate_read_count == state.cooling_body_entry_count
        && state.supply_mass_flow_rate_strictly_positive_comparison_count
            == state.cooling_body_entry_count
        && state.witnessed_positive_supply_mass_flow_body_entry_count
            == state.positive_supply_mass_flow_body_entry_count
        && state.witnessed_active_guard_false_fallthrough_count
            == state.active_guard_false_fallthrough_count
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.latest_route == Some(Route::PositiveSupplyMassFlowBodyEntered)
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && latest.parent_call_ordinal == state.transition_count
        && snapshots_match_bit_exact(latest, witness)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp330_flow_owner_hot_path_has_no_recursive_exact_validation() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        assert!(!hot.contains("completed_"));
        assert!(!hot.contains("snapshot_is_exact"));
        assert!(!hot.contains("predecessor_route("));
    }
}
