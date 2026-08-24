//! Sealed CP426 route and mixed-air-humidity-ratio capability for CP427.

use super::super::transition::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentRetainedRoute as Route,
    cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_route_from_committed_predecessor,
};
use super::prefix::predecessor_cp425_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{
    retained_route_matches_snapshot_bounded, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_committed_latest_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP426's bounded committed route without recursive derivation.
pub(in crate::ideal_loads::calc) fn cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
    cp329_witness: Option<Cp329Snapshot>,
) -> Option<Route> {
    let state =
        &unit.calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp425_snapshot(latest);
    let predecessor_route =
        cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_committed_latest_route(
            unit,
            predecessor,
            cp329_witness,
        )?;
    let expected_route =
        cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
    let assigned_humidity_ratio_is_exact = if route.assignment_executed {
        let committed = cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio(
            unit,
            cp329_witness?,
        )?;
        [
            latest
                .mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment,
            latest.assigned_supply_humidity_ratio_from_mixed_air,
            latest.resulting_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| value.is_some_and(|value| value.to_bits() == committed.to_bits()))
    } else {
        cp329_witness.is_none()
            && latest
                .mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment
                .is_none()
            && latest.assigned_supply_humidity_ratio_from_mixed_air.is_none()
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
        && assigned_humidity_ratio_is_exact
        && state_counts_are_consistent(state)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment,
        ))
    .then_some(route)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp426_committed_route_hot_path_is_bounded() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        for forbidden in [
            "completed_",
            "snapshot_is_exact",
            "private_characterization",
            "snapshot_route(",
            "calc_cooling_mixed_air_call.latest",
        ] {
            assert!(!hot.contains(forbidden), "{forbidden}");
        }
        assert_eq!(
            hot.matches("supply_enthalpy_mixed_air_assignment_committed_latest_route(")
                .count(),
            1,
        );
        assert_eq!(
            hot.matches("mixed_air_call_committed_latest_mixed_air_humidity_ratio(")
                .count(),
            1,
        );
    }
}
