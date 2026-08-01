//! CP375 public direct release and canonical private selected-`None` tests.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError as Error,
    active_none_operands_from_retained_cp345_for_test,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Cp374Snapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release,
};

#[test]
fn cp375_direct_is_owner_free_and_private_none_uses_exact_cp345_owner() {
    let (mut runtime, system, cp374) = completed_cp374_case();
    let mut skipped_owner_poison = system.clone();
    skipped_owner_poison.maximum_heating_supply_air_humidity_ratio = f64::NAN;
    let direct = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
        &mut runtime,
        &skipped_owner_poison,
        cp374,
    )
    .expect("CP375 direct");

    assert!(
        cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
            direct,
        )
    );
    assert_eq!(
        direct.predecessor_resulting_supply_humidity_ratio_for_humidification,
        None
    );
    assert_eq!(
        direct.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        None
    );
    assert_eq!(
        direct.supply_humidity_ratio_for_humidification_for_supply_maximum,
        None
    );
    assert_eq!(direct.resulting_supply_humidity_ratio, None);

    let before = runtime.clone();
    let unit = runtime.units.get(&system.id).expect("known CP375 unit");
    let private = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        0.0,
        0.008,
    )
    .expect("canonical private selected-None CP375");
    let owner = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest
        .and_then(|owner| owner.assigned_supply_humidity_ratio)
        .expect("same-call CP345 owner with no intervening write on the None branch");
    let right = private
        .predecessor_resulting_supply_humidity_ratio_for_humidification
        .expect("same-call CP374 owner");
    let expected = if owner < right { right } else { owner };
    assert_eq!(
        private
            .purchased_air_supply_humidity_ratio_before_humidification_supply_maximum
            .map(f64::to_bits),
        Some(owner.to_bits())
    );
    assert_eq!(
        private.resulting_supply_humidity_ratio.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert!(
        private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_links_to_direct_release(
            &runtime,
            unit,
            &system,
            direct,
            private,
            0.0,
            0.008,
        )
    );

    let private_cp374 = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release(
        &runtime,
        unit,
        &system,
        cp374,
        0.0,
        0.008,
    )
    .expect("private CP374 None predecessor");
    let operands = active_none_operands_from_retained_cp345_for_test(
        &runtime,
        unit,
        &system,
        private_cp374,
    )
    .expect("validated CP345 owner");
    assert_eq!(operands.purchased_air_supply_humidity_ratio.to_bits(), owner.to_bits());
    assert_eq!(runtime, before);

    let mut forged_runtime = runtime.clone();
    let mut forged = forged_runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(system.id)
        .expect("CP345 witness");
    forged.assigned_supply_humidity_ratio = Some(f64::from_bits(owner.to_bits() ^ 1));
    forged_runtime
        .set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
            forged,
        );
    let forged_unit = forged_runtime.units.get(&system.id).expect("known unit");
    assert!(
        active_none_operands_from_retained_cp345_for_test(
            &forged_runtime,
            forged_unit,
            &system,
            private_cp374,
        )
        .is_none()
    );
}

#[test]
fn cp375_public_release_rejects_forged_predecessor_and_replay_transactionally() {
    let (mut runtime, system, cp374) = completed_cp374_case();
    let mut forged = cp374;
    forged.parent_call_ordinal = forged.parent_call_ordinal.saturating_add(1);
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
            &mut runtime,
            &system,
            forged,
        ),
        Err(Error::CoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshotMismatch { .. })
    ));
    assert_eq!(runtime, before);

    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
        &mut runtime,
        &system,
        cp374,
    )
    .expect("first CP375 direct release");
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
            &mut runtime,
            &system,
            cp374,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn completed_cp374_case() -> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp374Snapshot) {
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
    (runtime, system, cp374)
}
