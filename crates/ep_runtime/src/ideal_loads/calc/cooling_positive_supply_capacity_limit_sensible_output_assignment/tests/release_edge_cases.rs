use super::release_fixture::{
    active_case, completed_cp338_case, completed_cp338_case_with_zone_temperature,
};
use super::super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment,
};

fn assert_rejected_without_cp339_commit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) {
    let before_state = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .clone();
    let before_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            system.id,
        );
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(
        runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment,
        before_state
    );
    assert_eq!(
        runtime
            .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                system.id,
            ),
        before_witness
    );
}

#[test]
fn public_active_complete_lineage_allows_infinite_flow_times_zero_delta_nan() {
    let (mut runtime, system, predecessor) = completed_cp338_case_with_zone_temperature(
        -f64::MAX,
        1.0,
        true,
        0.008,
        13.000_02,
    );
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        unit.calc_cooling_supply_mass_flow_positive_guard
            .latest
            .and_then(|snapshot| snapshot.supply_mass_flow_rate_kg_per_s)
            .is_some_and(|value| value == f64::INFINITY)
    );
    assert_eq!(
        unit.calc_cooling_mixed_air_call
            .latest
            .and_then(|snapshot| snapshot.mixed_air_enthalpy_projection_j_per_kg)
            .map(f64::to_bits),
        unit.calc_cooling_positive_supply_enthalpy_assignment
            .latest
            .and_then(|snapshot| snapshot.supply_enthalpy_j_per_kg)
            .map(f64::to_bits)
    );

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP339 derived NaN is admitted");
    assert!(
        snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .is_some_and(|value| value.to_bits() == 0.0_f64.to_bits())
    );
    assert!(
        snapshot
            .cooling_sensible_output_w
            .is_some_and(f64::is_nan)
    );
    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
            &runtime,
            runtime.units.get(&system.id).expect("known unit"),
            &system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                    system.id,
                ),
        )
    );
}

#[test]
fn public_skip_does_not_require_available_active_operand_values() {
    let (mut runtime, system, predecessor) =
        completed_cp338_case(-1_000.0, 0.0, true, 0.008);
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        unit.calc_cooling_supply_mass_flow_positive_guard
            .latest
            .is_some_and(|snapshot| snapshot.supply_mass_flow_rate_kg_per_s.is_none())
    );
    assert!(
        unit.calc_cooling_mixed_air_call
            .latest
            .is_some_and(|snapshot| snapshot.mixed_air_enthalpy_projection_j_per_kg.is_none())
    );
    assert!(
        unit.calc_cooling_positive_supply_enthalpy_assignment
            .latest
            .is_some_and(|snapshot| snapshot.supply_enthalpy_j_per_kg.is_none())
    );

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP339 unit-off skip");
    assert!(snapshot.unit_off_skipped);
    assert!(!snapshot.capacity_limit_sensible_output_assignment_executed);
}

#[test]
fn nan_flow_and_nonfinite_enthalpy_lineage_are_transactionally_rejected() {
    for corruption in 0..3 {
        let (mut runtime, system, predecessor) = active_case();
        match corruption {
            0 => {
                let mut snapshot = runtime
                    .cooling_supply_mass_flow_positive_guard_latest_witness(system.id)
                    .expect("CP330 witness");
                snapshot.supply_mass_flow_rate_kg_per_s =
                    Some(f64::from_bits(0x7ff8_0000_0000_00a1));
                runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_supply_mass_flow_positive_guard
                    .latest = Some(snapshot);
                runtime.set_cooling_supply_mass_flow_positive_guard_latest_witness(
                    system.id,
                    snapshot,
                );
            }
            1 => {
                let mut snapshot = runtime
                    .cooling_mixed_air_call_latest_witness(system.id)
                    .expect("CP329 witness");
                snapshot.mixed_air_enthalpy_projection_j_per_kg =
                    Some(f64::INFINITY);
                runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_mixed_air_call
                    .latest = Some(snapshot);
                runtime.set_cooling_mixed_air_call_latest_witness(system.id, snapshot);
            }
            2 => {
                let mut snapshot = runtime
                    .cooling_positive_supply_enthalpy_assignment_latest_witness(
                        system.id,
                    )
                    .expect("CP336 witness");
                snapshot.supply_enthalpy_j_per_kg = Some(f64::NEG_INFINITY);
                runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_positive_supply_enthalpy_assignment
                    .latest = Some(snapshot);
                runtime
                    .set_cooling_positive_supply_enthalpy_assignment_latest_witness(
                        system.id,
                        snapshot,
                    );
            }
            _ => unreachable!(),
        }
        assert_rejected_without_cp339_commit(&mut runtime, &system, predecessor);
    }
}
