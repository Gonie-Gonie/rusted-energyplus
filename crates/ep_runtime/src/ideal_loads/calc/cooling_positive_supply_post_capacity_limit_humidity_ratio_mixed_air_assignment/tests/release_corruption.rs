use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput,
    release::next_transition_fits_for_test,
};
use super::public_release::{assert_rejected_transactionally, completed_cp344_case};

fn flipped(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

#[test]
fn supplied_cp344_numeric_bit_drift_is_rejected_transactionally() {
    let (mut runtime, system, mut predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    predecessor.resulting_supply_temperature_c =
        predecessor.resulting_supply_temperature_c.map(flipped);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn retained_cp344_latest_drift_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let unit = runtime.units.get_mut(&system.id).expect("known unit");
    let mut drift = predecessor;
    drift.minimum_supply_temperature_c = drift.minimum_supply_temperature_c.map(flipped);
    unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .latest = Some(drift);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn private_cp344_witness_drift_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let mut drift = predecessor;
    drift.assigned_supply_temperature_c = drift.assigned_supply_temperature_c.map(flipped);
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            system.id,
            drift,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn malformed_cp344_double_active_route_is_rejected_even_when_all_copies_match() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let mut malformed = predecessor;
    malformed.capacity_limit_sensible_output_guard_false_fallthrough = true;
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .latest = Some(malformed);
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            system.id,
            malformed,
        );
    assert_rejected_transactionally(&mut runtime, &system, malformed);
}

#[test]
fn cp329_public_owner_corruption_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let owner = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .expect("CP329");
    let mut drift = owner;
    drift.mixed_air_humidity_ratio = drift.mixed_air_humidity_ratio.map(flipped);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest = Some(drift);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn cp329_private_owner_corruption_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let mut drift = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .expect("CP329");
    drift.mixed_air_humidity_ratio = drift.mixed_air_humidity_ratio.map(flipped);
    runtime.set_cooling_mixed_air_call_latest_witness(system.id, drift);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn cp335_public_corroboration_corruption_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let corroboration = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest
        .expect("CP335");
    let mut drift = corroboration;
    drift.assigned_supply_humidity_ratio = drift.assigned_supply_humidity_ratio.map(flipped);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest = Some(drift);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn cp335_private_corroboration_corruption_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let mut drift = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest
        .expect("CP335");
    drift.mixed_air_humidity_ratio = drift.mixed_air_humidity_ratio.map(flipped);
    runtime.set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        system.id, drift,
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn cp337_active_guard_false_counter_corruption_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-1_000.0, 1.0, false);
    assert!(predecessor.capacity_limit_guard_false_fallthrough_skipped);
    let counter = &mut runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_guard
        .active_guard_false_fallthrough_count;
    assert_eq!(*counter, 1);
    *counter = 2;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn nonfinite_owner_and_matching_corroboration_cannot_bypass_recursive_release() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let payload = f64::from_bits(0x7ff8_0000_0000_0042);
    let mut owner = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .expect("CP329");
    owner.mixed_air_humidity_ratio = Some(payload);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest = Some(owner);
    runtime.set_cooling_mixed_air_call_latest_witness(system.id, owner);

    let mut corroboration = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest
        .expect("CP335");
    corroboration.mixed_air_humidity_ratio = Some(payload);
    corroboration.assigned_supply_humidity_ratio = Some(payload);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest = Some(corroboration);
    runtime.set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        system.id,
        corroboration,
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn every_active_counter_increment_is_preflighted() {
    let (runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let baseline = runtime.units.get(&system.id).expect("known unit");
    let active_input = Some(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput {
            mixed_air_humidity_ratio: 0.008,
        },
    );

    macro_rules! assert_overflow_rejected {
        ($field:ident) => {{
            let mut unit = baseline.clone();
            unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
                .$field = usize::MAX;
            assert!(!next_transition_fits_for_test(
                &unit,
                predecessor,
                active_input
            ));
        }};
    }

    assert_overflow_rejected!(transition_count);
    assert_overflow_rejected!(post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count);
    assert_overflow_rejected!(source_site_execution_count);
    assert_overflow_rejected!(mixed_air_humidity_ratio_read_count);
    assert_overflow_rejected!(supply_humidity_ratio_assignment_count);
    assert_overflow_rejected!(
        witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count
    );
    assert_overflow_rejected!(
        assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
    );
    assert_overflow_rejected!(
        witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count
    );

    macro_rules! assert_route_counter_overflow {
        ($demand:expr, $availability:expr, $capacity:expr, $field:ident, $active:expr) => {{
            let (runtime, system, predecessor) =
                completed_cp344_case($demand, $availability, $capacity);
            let mut unit = runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .clone();
            unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
                .$field = usize::MAX;
            let active_input = $active.then_some(
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput {
                    mixed_air_humidity_ratio: 0.008,
                },
            );
            assert!(!next_transition_fits_for_test(
                &unit,
                predecessor,
                active_input,
            ));
        }};
    }

    assert_route_counter_overflow!(-1_000.0, 0.0, true, unit_off_skip_count, false);
    assert_route_counter_overflow!(1.0, 1.0, true, non_cooling_skip_count, false);
    assert_route_counter_overflow!(
        -1.0e-40,
        1.0,
        true,
        positive_guard_false_fallthrough_skip_count,
        false
    );
    assert_route_counter_overflow!(
        -1.0e-40,
        1.0,
        true,
        witnessed_positive_guard_false_fallthrough_skip_count,
        false
    );
    assert_route_counter_overflow!(
        -1_000.0,
        1.0,
        false,
        assignment_after_capacity_limit_guard_false_fallthrough_count,
        true
    );
    assert_route_counter_overflow!(
        -1_000.0,
        1.0,
        false,
        witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count,
        true
    );
    assert_route_counter_overflow!(
        -1_000.0,
        1.0,
        true,
        assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        true
    );
    assert_route_counter_overflow!(
        -1_000.0,
        1.0,
        true,
        witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        true
    );
}

#[test]
fn malformed_retained_counter_state_rejects_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .assignment_after_capacity_limit_guard_false_fallthrough_count = 1;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn source_site_multiplication_overflow_state_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp344_case(-100_000.0, 1.0, true);
    let state = &mut runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
    state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count = usize::MAX;
    state
        .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count =
        usize::MAX;
    state.witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count =
        usize::MAX;
    state
        .witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count =
        usize::MAX;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}
