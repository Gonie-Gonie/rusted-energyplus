use crate::ideal_loads::calc::cooling_capacity_zero_flow_reset_committed_latest_maximum_total_cooling_capacity as committed_capacity;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard::tests::release_fixture::completed_cp340_case;

#[test]
fn cp321_capacity_owner_accepts_exact_commit_and_rejects_metadata_value_and_witness_forgeries() {
    let (runtime, system, _) = completed_cp340_case(-1_000.0, 1.0, true);
    let witness = runtime
        .cooling_capacity_zero_flow_reset_latest_witness(system.id)
        .expect("CP321 witness");
    let capacity = committed_capacity(
        runtime.units.get(&system.id).expect("unit"),
        witness,
    )
    .expect("sealed capacity");
    assert_eq!(
        capacity.to_bits(),
        witness
            .maximum_total_cooling_capacity_w
            .expect("owned capacity")
            .to_bits(),
    );

    let mut cases = Vec::new();
    let mut missing = runtime.clone();
    missing
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_capacity_zero_flow_reset
        .latest = None;
    cases.push((missing, witness));

    let mut forged_witness = witness;
    forged_witness.maximum_total_cooling_capacity_w = Some(f64::from_bits(capacity.to_bits() ^ 1));
    cases.push((runtime.clone(), forged_witness));

    let mut count = runtime.clone();
    count
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_capacity_zero_flow_reset
        .transition_count += 1;
    cases.push((count, witness));

    let mut ordinal = runtime.clone();
    ordinal
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_capacity_zero_flow_reset
        .latest_transition_ordinal = Some(0);
    cases.push((ordinal, witness));

    let mut route = runtime.clone();
    route
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_capacity_zero_flow_reset
        .latest_route = Some(
        super::super::PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::CandidatesZeroed,
    );
    cases.push((route, witness));

    let mut value = runtime.clone();
    let unit = value.units.get_mut(&system.id).expect("unit");
    let latest = unit
        .calc_cooling_capacity_zero_flow_reset
        .latest
        .as_mut()
        .expect("latest");
    latest.maximum_total_cooling_capacity_w = Some(f64::from_bits(capacity.to_bits() ^ 1));
    let value_witness = *latest;
    cases.push((value, value_witness));

    for (case, forged) in cases {
        assert!(committed_capacity(
            case.units.get(&system.id).expect("unit"),
            forged,
        )
        .is_none());
    }
}
