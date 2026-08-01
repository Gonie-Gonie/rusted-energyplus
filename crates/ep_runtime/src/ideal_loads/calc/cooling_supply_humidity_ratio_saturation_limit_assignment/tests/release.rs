//! CP378 public release, lineage, and commit-atomicity tests.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError as Error,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
    completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent,
    cooling_supply_humidity_ratio_saturation_limit_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_saturation_limit_assignment_characterization,
};
use super::predecessor_for_route;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Cp377Snapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment,
};

#[test]
fn cp378_public_direct_assigns_the_source_shaped_minimum_and_completes() {
    let (mut runtime, system, cp377) = completed_cp377_case();
    let left = cp377
        .predecessor_resulting_supply_humidity_ratio_original
        .expect("CP376 original");
    let right = cp377
        .resulting_saturation_supply_humidity_ratio
        .expect("CP377 saturation");
    let expected = source_shaped_two_argument_minimum(left, right);
    let snapshot =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .expect("CP378 direct release");

    assert!(
        cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert_eq!(
        snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
        Some(expected.to_bits()),
    );
    let unit = runtime.units.get(&system.id).expect("known unit");
    let witness =
        runtime.cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(system.id);
    assert!(
        completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    );
    assert!(
        cooling_supply_humidity_ratio_saturation_limit_assignment_latest_metadata_is_consistent(
            unit, 1,
        )
    );
}

#[test]
fn cp378_rejects_transitive_cp376_bit_drift_transactionally() {
    let (mut runtime, system, cp377) = completed_cp377_case();
    let mut forged = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(system.id)
        .expect("CP376 witness");
    forged.resulting_supply_humidity_ratio_original = forged
        .resulting_supply_humidity_ratio_original
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime.set_cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(
        system.id, forged,
    );
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn cp378_rejects_forged_cp377_carried_original_and_replay_transactionally() {
    let (mut runtime, system, mut cp377) = completed_cp377_case();
    cp377.predecessor_resulting_supply_humidity_ratio_original = cp377
        .predecessor_resulting_supply_humidity_ratio_original
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_supply_humidity_ratio_saturation_assignment
        .latest = Some(cp377);
    runtime
        .set_cooling_supply_humidity_ratio_saturation_assignment_latest_witness(system.id, cp377);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, cp377) = completed_cp377_case();
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
        &mut runtime,
        &system,
        cp377,
    )
    .expect("first CP378 release");
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn cp378_counter_overflow_preserves_runtime_state_and_witness() {
    let (mut runtime, system, cp377) = completed_cp377_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .purchased_air_supply_humidity_ratio_saturation_limit_assignment_count = usize::MAX;
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        ),
        Err(Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp378_snapshot_validation_recomputes_and_complete_null_skips_remain_null() {
    let predecessor = predecessor_for_route(4, 0.008);
    let exact = private_cooling_supply_humidity_ratio_saturation_limit_assignment_characterization(
        predecessor,
    )
    .expect("exact characterization");
    let mut corrupted = exact;
    corrupted.minimum_supply_humidity_ratio_after_saturation_limit = corrupted
        .minimum_supply_humidity_ratio_after_saturation_limit
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        !cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            corrupted,
        )
    );

    for route in 0..3 {
        let skip =
            private_cooling_supply_humidity_ratio_saturation_limit_assignment_characterization(
                predecessor_for_route(route, 0.0),
            )
            .expect("complete-null skip");
        assert!(skip.resulting_supply_humidity_ratio.is_none());
        assert!(
            cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
                skip,
            )
        );
    }
}

fn completed_cp377_case() -> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp377Snapshot) {
    let (mut runtime, system, cp370) =
        completed_cp370_case_for_cp372_test().expect("CP370 fixture");
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .expect("CP371 direct");
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp371,
    )
    .expect("CP372 direct");
    let cp373 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
        &mut runtime,
        &system,
        cp372,
    )
    .expect("CP373 direct");
    let cp374 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
        &mut runtime,
        &system,
        cp373,
    )
    .expect("CP374 direct");
    let cp375 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
        &mut runtime,
        &system,
        cp374,
    )
    .expect("CP375 direct");
    let cp376 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .expect("CP376 direct");
    let cp377 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(
        &mut runtime,
        &system,
        cp376,
        101_325.0,
    )
    .expect("CP377 direct");
    (runtime, system, cp377)
}
