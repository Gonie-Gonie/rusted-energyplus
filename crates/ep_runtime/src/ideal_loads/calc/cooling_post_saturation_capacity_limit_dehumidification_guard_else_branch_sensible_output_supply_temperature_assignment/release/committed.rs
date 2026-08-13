//! Sealed CP423 route capability for its immediate successor.

use super::super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute as Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor,
};
use super::prefix::predecessor_cp422_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{retained_route_matches_snapshot_bounded, snapshots_match_bit_exact};
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_sensible_output_inputs,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_committed_latest_route_and_assigned_cooling_sensible_output,
    cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP423's bounded committed route without recursively deriving it.
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<Route> {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp422_snapshot(latest);
    let (predecessor_route, committed_output) =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_committed_latest_route_and_assigned_cooling_sensible_output(
            unit,
            predecessor,
        )?;
    let expected_route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    let assigned_output_is_exact = if route.assignment_executed {
        let mixed_witness = unit.calc_cooling_mixed_air_call.latest?;
        let cp329 = cooling_mixed_air_call_committed_latest_sensible_output_inputs(unit, mixed_witness)?;
        let flow_witness = unit.calc_cooling_supply_mass_flow_positive_guard.latest?;
        let flow = cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate(
            unit,
            flow_witness,
            cp329,
        )?;
        let cp419_witness = unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
            .latest?;
        let (cp419_route, cp_air) = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air(
            unit,
            cp419_witness,
        )?;
        cp419_route.active
            && cp419_route.logical_index == route.logical_index
            && latest
            .cooling_sensible_output_for_supply_temperature_w
            .zip(committed_output)
            .is_some_and(|(retained, committed)| retained.to_bits() == committed.to_bits())
            && latest
                .mixed_air_temperature_for_sensible_output_supply_temperature_c
                .is_some_and(|retained| retained.to_bits() == cp329.mixed_air_temperature_c.to_bits())
            && latest
                .supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s
                .is_some_and(|retained| retained.to_bits() == flow.to_bits())
            && latest
                .cp_air_for_sensible_output_supply_temperature_j_per_kg_k
                .zip(cp_air)
                .is_some_and(|(retained, committed)| retained.to_bits() == committed.to_bits())
    } else {
        committed_output.is_none()
    };
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && snapshots_match_bit_exact(latest, witness)
        && retained_route_matches_snapshot_bounded(latest, route)
        && route == expected_route
        && assigned_output_is_exact
        && state_counts_are_consistent(state)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment,
        ))
    .then_some(route)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp423_committed_route_hot_path_is_bounded() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        for forbidden in ["completed_", "snapshot_is_exact", "private_characterization", "snapshot_route("] {
            assert!(!hot.contains(forbidden), "{forbidden}");
        }
        assert_eq!(
            hot.matches("maximum_capacity_assignment_committed_latest_route_and_assigned_cooling_sensible_output(").count(),
            1,
        );
    }
}
