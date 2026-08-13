use super::release_fixture::completed_cp340_case;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity as committed_capacity;

#[test]
fn cp340_capacity_corroborator_rejects_latest_route_count_ordinal_witness_and_bit_drift() {
    let (runtime, system, witness) = completed_cp340_case(-1_000.0, 1.0, true);
    let cp321 = runtime
        .cooling_capacity_zero_flow_reset_latest_witness(system.id)
        .expect("CP321");
    assert!(committed_capacity(
        runtime.units.get(&system.id).expect("unit"),
        cp321,
        witness,
    )
    .is_some());

    let mut cases = Vec::new();
    let mut missing = runtime.clone();
    missing
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest = None;
    cases.push((missing, witness));

    let mut forged_witness = witness;
    forged_witness.maximum_total_cooling_capacity_w = forged_witness
        .maximum_total_cooling_capacity_w
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    cases.push((runtime.clone(), forged_witness));

    let mut count = runtime.clone();
    count
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .transition_count += 1;
    cases.push((count, witness));

    let mut ordinal = runtime.clone();
    ordinal
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, witness));

    let mut route = runtime.clone();
    route
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest_route = Some(
        super::super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::UnitOff,
    );
    cases.push((route, witness));

    let mut active_route = runtime.clone();
    let route = active_route
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest_route
        .as_mut()
        .expect("route");
    *route = match *route {
        super::super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::CapacityLimitSensibleOutputAdjustmentBodyEntered => {
            super::super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::CapacityLimitSensibleOutputGuardFalseFallthrough
        }
        _ => {
            super::super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::CapacityLimitSensibleOutputAdjustmentBodyEntered
        }
    };
    cases.push((active_route, witness));

    let mut latest = runtime.clone();
    latest
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest
        .as_mut()
        .expect("latest")
        .maximum_total_cooling_capacity_w = Some(-1.0);
    cases.push((latest, witness));

    for (case, forged) in cases {
        assert!(committed_capacity(
            case.units.get(&system.id).expect("unit"),
            cp321,
            forged,
        )
        .is_none());
    }
}
