//! CP376 public direct release tests.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError as Error,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Cp375Snapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
};

#[test]
fn cp376_public_direct_reads_recursively_validated_cp347_and_copies_bits() {
    let (mut runtime, system, cp375) = completed_cp375_case();
    let expected = runtime
        .units
        .get(&system.id)
        .and_then(|unit| {
            unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
                .latest
        })
        .and_then(|owner| owner.resulting_supply_humidity_ratio)
        .expect("retained CP347 owner");
    let snapshot =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .expect("CP376 direct release");

    assert!(
        cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(snapshot.cp347_none_case_owned_read);
    assert!(!snapshot.cp375_maximum_assignment_owned_read);
    assert_eq!(
        snapshot
            .purchased_air_supply_humidity_ratio_before_saturation_check
            .map(f64::to_bits),
        Some(expected.to_bits()),
    );
    assert_eq!(
        snapshot
            .assigned_supply_humidity_ratio_original
            .map(f64::to_bits),
        Some(expected.to_bits()),
    );
    let unit = runtime.units.get(&system.id).expect("known unit");
    let witness = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(system.id);
    assert!(
        completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            witness,
        )
    );
    assert!(
        cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_metadata_is_consistent(
            &unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
            1,
        )
    );
}

#[test]
fn cp376_rejects_cp347_witness_corruption_and_replay_transactionally() {
    let (mut runtime, system, cp375) = completed_cp375_case();
    let mut forged = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
            system.id,
        )
        .expect("CP347 witness");
    forged.resulting_supply_humidity_ratio = forged
        .resulting_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime.set_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
        system.id,
        forged,
    );
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        ),
        Err(Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);

    let (mut runtime, system, cp375) = completed_cp375_case();
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
        &mut runtime,
        &system,
        cp375,
    )
    .expect("first CP376 release");
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn cp376_public_metadata_rejects_owner_counter_redistribution() {
    let (mut runtime, system, cp375) = completed_cp375_case();
    let snapshot =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .expect("CP376 direct release");
    let witness = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(system.id);
    {
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
        state.cp347_none_case_owner_count -= 1;
        state.cp356_constant_shr_owner_count += 1;
        assert!(
            !cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_metadata_is_consistent(
                state,
                1,
            )
        );
    }
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            witness,
        )
    );
}

pub(super) fn completed_cp375_case(
) -> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp375Snapshot) {
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
    (runtime, system, cp375)
}
