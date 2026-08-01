//! CP374 public direct release and canonical private selected-`None` tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError as Error,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
    active_operands_from_selected_typed_owner_for_test,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release,
};

#[test]
fn cp374_direct_is_numeric_free_and_private_none_uses_typed_owner_transactionally() {
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
    let mut skipped_rhs_poison = system.clone();
    skipped_rhs_poison.maximum_heating_supply_air_humidity_ratio = f64::NAN;
    let direct = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
        &mut runtime,
        &skipped_rhs_poison,
        cp373,
    )
    .expect("CP374 direct");

    assert!(
        cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(
            direct,
        )
    );
    assert_eq!(
        direct.predecessor_resulting_supply_humidity_ratio_for_humidification,
        None
    );
    assert_eq!(
        direct.supply_humidity_ratio_for_humidification_before_maximum_limit,
        None
    );
    assert_eq!(direct.maximum_heating_supply_air_humidity_ratio, None);
    assert_eq!(direct.resulting_supply_humidity_ratio_for_humidification, None);

    let before = runtime.clone();
    for (demand, zone_humidity_ratio, right) in [
        (0.0, 0.010, 0.008),
        (0.0, 0.007, 0.008),
        (-0.0, -0.0, 0.0),
        (0.0, 0.0, -0.0),
        (f64::from_bits(0x7ff8_0000_0000_0374), 0.0, 0.008),
    ] {
        let mut selected_system = system.clone();
        selected_system.maximum_heating_supply_air_humidity_ratio = right;
        let unit = runtime.units.get(&system.id).expect("known CP374 unit");
        let private = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release(
            &runtime,
            unit,
            &selected_system,
            direct,
            demand,
            zone_humidity_ratio,
        )
        .expect("canonical private selected-None CP374");
        let left = private
            .predecessor_resulting_supply_humidity_ratio_for_humidification
            .expect("CP373 left operand");
        let expected = if left < right { left } else { right };
        assert_eq!(
            private
                .maximum_heating_supply_air_humidity_ratio
                .map(f64::to_bits),
            Some(right.to_bits())
        );
        assert_eq!(
            private
                .resulting_supply_humidity_ratio_for_humidification
                .map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert!(
            private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_links_to_direct_release(
                &runtime,
                unit,
                &selected_system,
                direct,
                private,
                demand,
                zone_humidity_ratio,
            )
        );
    }

    let unit = runtime.units.get(&system.id).expect("known CP374 unit");
    let private_predecessor = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release(
        &runtime,
        unit,
        &system,
        cp373,
        0.0,
        0.008,
    )
    .expect("canonical private CP373 predecessor");
    let operands = active_operands_from_selected_typed_owner_for_test(
        unit,
        &system,
        private_predecessor,
    )
    .expect("finite selected typed owner");
    assert_eq!(
        operands
            .maximum_heating_supply_air_humidity_ratio
            .to_bits(),
        system
            .maximum_heating_supply_air_humidity_ratio
            .to_bits()
    );

    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut invalid_owner = system.clone();
        invalid_owner.maximum_heating_supply_air_humidity_ratio = invalid;
        let unit = runtime.units.get(&system.id).expect("known CP374 unit");
        assert!(
            active_operands_from_selected_typed_owner_for_test(
                unit,
                &invalid_owner,
                private_predecessor,
            )
            .is_none()
        );
        assert!(
            private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release(
                &runtime,
                unit,
                &invalid_owner,
                direct,
                0.0,
                0.008,
            )
            .is_none()
        );
    }
    assert_eq!(runtime, before);
}

#[test]
fn cp374_public_release_rejects_forged_predecessor_and_replay_transactionally() {
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

    let mut forged = cp373;
    forged.parent_call_ordinal = forged.parent_call_ordinal.saturating_add(1);
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
            &mut runtime,
            &system,
            forged,
        ),
        Err(Error::CoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshotMismatch { .. })
    ));
    assert_eq!(runtime, before);

    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
        &mut runtime,
        &system,
        cp373,
    )
    .expect("first CP374 direct release");
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
            &mut runtime,
            &system,
            cp373,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
