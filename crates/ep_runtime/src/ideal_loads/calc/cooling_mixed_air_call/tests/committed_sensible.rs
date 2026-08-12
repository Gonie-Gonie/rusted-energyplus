use super::super::{
    PurchasedAirCalcCoolingMixedAirCallRetainedRoute,
    cooling_mixed_air_call_committed_latest_sensible_output_inputs,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
};

#[test]
fn cp329_sensible_owner_accepts_exact_flow_and_temperature_bits() {
    let (runtime, system, snapshot) = completed_case();
    let inputs = cooling_mixed_air_call_committed_latest_sensible_output_inputs(
        runtime.units.get(&system).expect("unit"),
        snapshot,
    )
    .expect("sealed CP329 sensible inputs");
    assert_eq!(
        inputs.supply_mass_flow_rate_kg_per_s.to_bits(),
        snapshot
            .supply_mass_flow_rate_kg_per_s
            .expect("flow")
            .to_bits(),
    );
    assert_eq!(
        inputs.mixed_air_temperature_c.to_bits(),
        snapshot.mixed_air_temperature_c.expect("mixed T").to_bits(),
    );
}

#[test]
fn cp329_sensible_owner_rejects_latest_witness_count_route_ordinal_and_value_forgeries() {
    let (runtime, system, snapshot) = completed_case();
    let mut cases = Vec::new();

    let mut latest = runtime.clone();
    latest
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_mixed_air_call
        .latest = None;
    cases.push((latest, snapshot));

    let mut forged_witness = snapshot;
    forged_witness.mixed_air_temperature_c = forged_witness
        .mixed_air_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    cases.push((runtime.clone(), forged_witness));

    let mut count = runtime.clone();
    count
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_mixed_air_call
        .transition_count += 1;
    cases.push((count, snapshot));

    let mut route = runtime.clone();
    route
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_mixed_air_call
        .latest_route = Some(PurchasedAirCalcCoolingMixedAirCallRetainedRoute::UnitOff);
    cases.push((route, snapshot));

    let mut ordinal = runtime.clone();
    ordinal
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_mixed_air_call
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, snapshot));

    let mut value = runtime.clone();
    let latest = value
        .units
        .get_mut(&system)
        .expect("unit")
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("latest");
    latest.supply_mass_flow_rate_kg_per_s = latest
        .supply_mass_flow_rate_kg_per_s
        .map(|flow| f64::from_bits(flow.to_bits() ^ 1));
    let value_witness = *latest;
    cases.push((value, value_witness));

    for (case, witness) in cases {
        assert!(
            cooling_mixed_air_call_committed_latest_sensible_output_inputs(
                case.units.get(&system).expect("unit"),
                witness,
            )
            .is_none()
        );
    }
}

fn completed_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystemId,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case();
    let snapshot = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP329");
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
        .transition_count = snapshot.parent_call_ordinal;
    (runtime, system.id, snapshot)
}
