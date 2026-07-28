use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment::tests::release_fixture::completed_cp342_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary,
};

pub(super) fn completed_cp344_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
){
    let (mut runtime, system, predecessor) =
        completed_cp342_case(cooling_demand_w, overall_availability, capacity_limit);
    let assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP343");
    let limit =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            assignment,
        )
        .expect("CP344");
    (runtime, system, limit)
}

pub(super) fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) {
    let selected = predecessor.system;
    let before_state = runtime
        .units
        .get(&selected)
        .expect("known unit")
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .clone();
    let before_witness = runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            selected,
        );
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(
        runtime
            .units
            .get(&selected)
            .expect("known unit")
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
        before_state
    );
    assert_eq!(
        runtime
            .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
                selected,
            ),
        before_witness
    );
}

#[test]
fn public_g_f_l_routes_copy_only_same_call_cp329_owner_bits() {
    for (demand, capacity, expected_provenance) in [
        (-1_000.0, false, [1, 0, 0]),
        (-1_000.0, true, [0, 1, 0]),
        (-100_000.0, true, [0, 0, 1]),
    ] {
        let (mut runtime, system, predecessor) = completed_cp344_case(demand, 1.0, capacity);
        let owner_bits = runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_mixed_air_call
            .latest
            .and_then(|snapshot| snapshot.mixed_air_humidity_ratio)
            .map(f64::to_bits)
            .expect("CP329 owner");
        let corroborating_bits = runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
            .latest
            .and_then(|snapshot| snapshot.assigned_supply_humidity_ratio)
            .map(f64::to_bits)
            .expect("CP335 corroboration");
        assert_eq!(corroborating_bits, owner_bits);

        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP345");
        assert!(snapshot.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed);
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(owner_bits)
        );
        assert_eq!(
            snapshot.assigned_supply_humidity_ratio.map(f64::to_bits),
            Some(owner_bits)
        );
        let state = &runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
        assert_eq!(
            [
                state.assignment_after_capacity_limit_guard_false_fallthrough_count,
                state
                    .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
                state
                    .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            ],
            expected_provenance
        );
        assert_eq!(
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
            1
        );
        assert_eq!(state.source_site_execution_count, 2);
    }
}

#[test]
fn public_u_n_p_routes_read_no_humidity_operand() {
    for (demand, availability) in [(-1_000.0, 0.0), (1.0, 1.0), (-1.0e-40, 1.0)] {
        let (mut runtime, system, predecessor) = completed_cp344_case(demand, availability, true);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP345 inherited skip");
        assert!(!snapshot.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed);
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(!snapshot.supply_humidity_ratio_assignment_performed);
        assert!(snapshot.assigned_supply_humidity_ratio.is_none());
    }
}

#[test]
fn lifecycle_summary_reports_completed_cp345_and_duplicate_release_is_transactional() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP345");
    let summary =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary(
            &runtime,
            system.id,
        )
        .expect("summary");
    assert_eq!(summary.source, snapshot.source);
    assert_eq!(
        summary.first_excluded_source,
        snapshot.first_excluded_source
    );
    assert_eq!(summary.state.latest, Some(snapshot));
    assert_eq!(summary.state.transition_count, 1);

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}
