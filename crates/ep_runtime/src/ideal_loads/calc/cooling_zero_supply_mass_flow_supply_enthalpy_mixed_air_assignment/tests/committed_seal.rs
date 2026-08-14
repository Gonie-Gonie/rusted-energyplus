//! Focused CP425 committed-route and independent CP329-witness tests.

use super::*;
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_mixed_air_enthalpy,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_committed_latest_route as committed,
    cp422_all_snapshots_for_successor_tests, cp424_fixture_unit_for_successor_tests,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot as Cp425Snapshot,
    PurchasedAirUnitRuntimeState, moist_air_enthalpy_j_per_kg,
};

#[test]
fn cp425_committed_seal_accepts_active_route_only_with_independent_cp329_witness() {
    let (unit, snapshot, route, cp329_witness) = active_fixture();
    assert_eq!(committed(&unit, snapshot, Some(cp329_witness)), Some(route));
    assert!(committed(&unit, snapshot, None).is_none());
}

#[test]
fn cp425_committed_seal_accepts_inactive_route_without_acquiring_cp329_owner() {
    let (unit, snapshot, route, cp329_witness) = fixture(false);
    assert_eq!(committed(&unit, snapshot, None), Some(route));
    assert!(committed(&unit, snapshot, Some(cp329_witness)).is_none());
}

#[test]
fn cp425_committed_seal_rejects_route_witness_count_and_identity_forgeries() {
    let (unit, snapshot, route, cp329_witness) = active_fixture();
    let owner = Some(cp329_witness);
    let mut cases = Vec::new();

    let mut logical = unit.clone();
    logical
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .logical_index = 3;
    cases.push((logical, snapshot, owner));
    let mut active = unit.clone();
    active
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .active = false;
    cases.push((active, snapshot, owner));
    let mut predecessor_assignment = unit.clone();
    predecessor_assignment
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .predecessor_assignment_executed ^= true;
    cases.push((predecessor_assignment, snapshot, owner));
    let mut predecessor_entered = unit.clone();
    predecessor_entered
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .predecessor_entered = false;
    cases.push((predecessor_entered, snapshot, owner));
    let mut assignment = unit.clone();
    assignment
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest_route
        .as_mut()
        .expect("route")
        .assignment_executed = false;
    cases.push((assignment, snapshot, owner));

    let mut witness = snapshot;
    witness.assigned_supply_enthalpy_from_mixed_air_j_per_kg =
        witness.assigned_supply_enthalpy_from_mixed_air_j_per_kg.map(flip);
    cases.push((unit.clone(), witness, owner));
    let mut latest_missing = unit.clone();
    latest_missing
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest = None;
    cases.push((latest_missing, snapshot, owner));
    let mut route_missing = unit.clone();
    route_missing
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest_route = None;
    cases.push((route_missing, snapshot, owner));
    let mut transition = unit.clone();
    transition
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .transition_count += 1;
    cases.push((transition, snapshot, owner));
    let mut init_count = unit.clone();
    init_count.init_call_count += 1;
    cases.push((init_count, snapshot, owner));
    let mut ordinal = unit.clone();
    ordinal
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, snapshot, owner));
    let mut route_count = unit.clone();
    route_count
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .predecessor_route_counts[route.logical_index] += 1;
    cases.push((route_count, snapshot, owner));
    let mut assignment_count = unit.clone();
    assignment_count
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_count += 1;
    cases.push((assignment_count, snapshot, owner));
    let mut site_count = unit.clone();
    site_count
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .source_site_execution_count += 1;
    cases.push((site_count, snapshot, owner));
    let mut predecessor_count = unit.clone();
    predecessor_count
        .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
        .predecessor_route_counts[route.logical_index] += 1;
    cases.push((predecessor_count, snapshot, owner));

    let mut coordinated_ordinal = unit.clone();
    let forged_ordinal = snapshot.parent_call_ordinal.wrapping_add(1);
    coordinated_ordinal
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest
        .as_mut()
        .expect("latest")
        .parent_call_ordinal = forged_ordinal;
    let mut ordinal_witness = snapshot;
    ordinal_witness.parent_call_ordinal = forged_ordinal;
    cases.push((coordinated_ordinal, ordinal_witness, owner));

    let mut coordinated_system = unit.clone();
    let forged_system = ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1));
    coordinated_system
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest
        .as_mut()
        .expect("latest")
        .system = forged_system;
    let mut system_witness = snapshot;
    system_witness.system = forged_system;
    cases.push((coordinated_system, system_witness, owner));

    let mut coordinated_zone = unit.clone();
    let forged_zone = ep_model::ZoneId(snapshot.controlled_zone.0.wrapping_add(1));
    coordinated_zone.controlled_zone = Some(forged_zone);
    coordinated_zone
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest
        .as_mut()
        .expect("latest")
        .controlled_zone = forged_zone;
    let mut zone_witness = snapshot;
    zone_witness.controlled_zone = forged_zone;
    cases.push((coordinated_zone, zone_witness, owner));

    let mut state_system = unit.clone();
    state_system
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .system = forged_system;
    cases.push((state_system, snapshot, owner));

    for (index, (forged, witness, owner)) in cases.into_iter().enumerate() {
        assert!(
            committed(&forged, witness, owner).is_none(),
            "forgery {index}",
        );
    }
}

#[test]
fn coordinated_cp329_latest_and_cp425_value_forgery_fails_private_witness_anchor() {
    let (unit, _, _, cp329_witness) = active_fixture();
    let mut forged = unit.clone();
    let cp329 = forged
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329 latest");
    let temperature = cp329.recirculation_temperature_c.expect("temperature");
    let humidity = cp329.recirculation_humidity_ratio.expect("humidity") + 0.000_001;
    let enthalpy = moist_air_enthalpy_j_per_kg(temperature, humidity);
    cp329.recirculation_humidity_ratio = Some(humidity);
    cp329.mixed_air_humidity_ratio = Some(humidity);
    cp329.recirculation_enthalpy_projection_j_per_kg = Some(enthalpy);
    cp329.mixed_air_enthalpy_projection_j_per_kg = Some(enthalpy);

    let latest = forged
        .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
        .latest
        .as_mut()
        .expect("CP425 latest");
    latest.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg =
        Some(enthalpy);
    latest.assigned_supply_enthalpy_from_mixed_air_j_per_kg = Some(enthalpy);
    latest.resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    let coordinated_cp425 = *latest;

    assert!(committed(
        &forged,
        coordinated_cp425,
        Some(cp329_witness),
    )
    .is_none());
}

fn active_fixture() -> (
    PurchasedAirUnitRuntimeState,
    Cp425Snapshot,
    Route,
    Cp329Snapshot,
) {
    fixture(true)
}

fn fixture(expect_active: bool) -> (
    PurchasedAirUnitRuntimeState,
    Cp425Snapshot,
    Route,
    Cp329Snapshot,
) {
    let mut fixture = None;
    for mut cp422 in cp422_all_snapshots_for_successor_tests() {
        cp422.parent_call_ordinal = 1;
        let (mut unit, cp424, _) = cp424_fixture_unit_for_successor_tests(cp422);
        let route = route_for(cp424);
        if route.assignment_executed != expect_active
            || unit.calc_cooling_mixed_air_call.latest.is_none()
        {
            continue;
        }
        let cp329_witness = unit.calc_cooling_mixed_air_call.latest.expect("CP329");
        let enthalpy = route.assignment_executed.then(|| {
            cooling_mixed_air_call_committed_latest_mixed_air_enthalpy(&unit, cp329_witness)
                .expect("committed CP329 enthalpy")
        });
        let mut state = State::new(cp424.system);
        let snapshot = advance_validated(&mut state, cp424, route, enthalpy)
            .expect("CP425 active");
        unit.calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment = state;
        fixture = Some((unit, snapshot, route, cp329_witness));
        break;
    }
    fixture.expect("active CP425 fixture")
}

fn flip(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
