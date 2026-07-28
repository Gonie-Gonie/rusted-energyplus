use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment::tests::release_fixture::completed_cp342_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

pub(super) fn completed_cp343_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) {
    let (mut runtime, system, predecessor) =
        completed_cp342_case(cooling_demand_w, overall_availability, capacity_limit);
    let assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP343");
    (runtime, system, assignment)
}

pub(super) fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) {
    let selected = predecessor.system;
    let before_state = runtime
        .units
        .get(&selected)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .clone();
    let before_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            selected,
        );
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
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
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
        before_state
    );
    assert_eq!(
        runtime
            .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
                selected,
            ),
        before_witness
    );
}

#[test]
fn public_true_route_uses_only_cp343_result_and_same_call_cp329_owner() {
    let (mut runtime, system, predecessor) = completed_cp343_case(-100_000.0, 1.0, true);
    assert!(
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed
    );
    let left = predecessor
        .resulting_supply_temperature_c
        .expect("CP343 current SupplyTemp");
    let right = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .and_then(|snapshot| snapshot.mixed_air_temperature_c)
        .expect("CP329 current MixedAirTemp");
    let expected = source_shaped_two_argument_minimum(left, right);

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP344");
    assert_eq!(
        snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
        Some(left.to_bits())
    );
    assert_eq!(
        snapshot
            .supply_temperature_before_mixed_air_limit_c
            .map(f64::to_bits),
        Some(left.to_bits())
    );
    assert_eq!(
        snapshot.mixed_air_temperature_c.map(f64::to_bits),
        Some(right.to_bits())
    );
    assert_eq!(
        snapshot.minimum_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert!(
        snapshot
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
    );

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn public_sensible_guard_false_preserves_cp343_result_and_reads_no_cp329_operand() {
    let (mut runtime, system, predecessor) = completed_cp343_case(-1_000.0, 1.0, true);
    assert!(predecessor.capacity_limit_sensible_output_guard_false_fallthrough);
    let left = predecessor
        .resulting_supply_temperature_c
        .expect("CP343 preserved SupplyTemp");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP344 false route");
    assert_eq!(
        snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
        Some(left.to_bits())
    );
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(left.to_bits())
    );
    assert!(!snapshot.supply_temperature_for_minimum_read);
    assert!(snapshot.supply_temperature_before_mixed_air_limit_c.is_none());
    assert!(!snapshot.mixed_air_temperature_for_minimum_read);
    assert!(snapshot.mixed_air_temperature_c.is_none());
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(!snapshot.supply_temperature_assignment_performed);
    assert_eq!(
        runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .source_site_execution_count,
        0
    );
}

#[test]
fn all_four_inherited_skips_have_no_cp344_numeric_evidence() {
    for (demand, availability, capacity) in [
        (-1_000.0, 0.0, true),
        (1.0, 1.0, true),
        (-1.0e-40, 1.0, true),
        (-1_000.0, 1.0, false),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp343_case(demand, availability, capacity);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP344 inherited skip");
        assert!(snapshot.preexisting_supply_temperature_c.is_none());
        assert!(
            snapshot
                .supply_temperature_before_mixed_air_limit_c
                .is_none()
        );
        assert!(snapshot.mixed_air_temperature_c.is_none());
        assert!(snapshot.minimum_supply_temperature_c.is_none());
        assert!(snapshot.assigned_supply_temperature_c.is_none());
        assert!(snapshot.resulting_supply_temperature_c.is_none());
    }
}

#[test]
fn lifecycle_summary_reports_the_completed_cp344_state() {
    let (mut runtime, system, predecessor) = completed_cp343_case(-100_000.0, 1.0, true);
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP344");
    let summary =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime,
            system.id,
        )
        .expect("summary");
    assert_eq!(summary.source, snapshot.source);
    assert_eq!(summary.first_excluded_source, snapshot.first_excluded_source);
    assert_eq!(summary.state.latest, Some(snapshot));
    assert_eq!(summary.state.transition_count, 1);
}
