use super::super::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute,
    cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::{
    cooling_mixed_air_call_committed_latest_sensible_output_inputs, release_tests::release_case,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};

#[test]
fn cp330_flow_owner_accepts_exact_positive_flow_bits() {
    let (runtime, system, cp329, cp330) = completed_case();
    let unit = runtime.units.get(&system).expect("unit");
    let cp329_inputs = cooling_mixed_air_call_committed_latest_sensible_output_inputs(unit, cp329)
        .expect("sealed CP329 inputs");
    let flow = cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate(
        unit,
        cp330,
        cp329_inputs,
    )
    .expect("sealed CP330 flow");
    assert_eq!(
        flow.to_bits(),
        cp330
            .supply_mass_flow_rate_kg_per_s
            .expect("flow")
            .to_bits(),
    );
}

#[test]
fn cp330_flow_owner_rejects_latest_witness_count_route_ordinal_and_value_forgeries() {
    let (runtime, system, cp329, cp330) = completed_case();
    let cp329_inputs = cooling_mixed_air_call_committed_latest_sensible_output_inputs(
        runtime.units.get(&system).expect("unit"),
        cp329,
    )
    .expect("sealed CP329 inputs");
    let mut cases = Vec::new();

    let mut latest = runtime.clone();
    latest
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .latest = None;
    cases.push((latest, cp330));

    let mut witness = cp330;
    witness.supply_mass_flow_rate_kg_per_s = witness
        .supply_mass_flow_rate_kg_per_s
        .map(|flow| f64::from_bits(flow.to_bits() ^ 1));
    cases.push((runtime.clone(), witness));

    let mut count = runtime.clone();
    count
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .transition_count += 1;
    cases.push((count, cp330));

    let mut route = runtime.clone();
    route
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .latest_route = Some(
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::ActiveGuardFalseFallthrough,
    );
    cases.push((route, cp330));

    let mut ordinal = runtime.clone();
    ordinal
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, cp330));

    let mut value = runtime.clone();
    let latest = value
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .as_mut()
        .expect("latest");
    latest.supply_mass_flow_rate_kg_per_s = Some(-1.0);
    let value_witness = *latest;
    cases.push((value, value_witness));

    for (case, witness) in cases {
        assert!(
            cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate(
                case.units.get(&system).expect("unit"),
                witness,
                cp329_inputs,
            )
            .is_none()
        );
    }
}

fn completed_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystemId,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case();
    let cp329 = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP329");
    let cp330 = advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
        &mut runtime,
        &system,
        cp329,
    )
    .expect("CP330");
    assert!(cp330.positive_supply_mass_flow_body_entered);
    let unit = runtime.units.get_mut(&system.id).expect("unit");
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
        .transition_count = cp329.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .transition_count = cp330.parent_call_ordinal;
    (runtime, system.id, cp329, cp330)
}
