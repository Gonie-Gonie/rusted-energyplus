use super::*;
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_case_break,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
};

#[test]
fn public_direct_none_is_numeric_lazy_and_records_only_a_completed_skip() {
    for minimum in [
        f64::from_bits(0x7ff8_0000_0000_0367),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let completed = completed_cp367_case(minimum);
        assert!(completed.is_some(), "CP367 direct fixture must remain lazy");
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let result = advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        );
        assert!(result.is_ok(), "CP368 must not read any humidity owner");
        let Ok(snapshot) = result else {
            return;
        };
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot.dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
        );
        let Some(state) = runtime
            .units
            .get(&system.id)
            .map(|unit| &unit.calc_cooling_default_supply_humidity_ratio_case_break)
        else {
            return;
        };
        assert_eq!(
            state.dehumidification_control_default_supply_humidity_ratio_case_break_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn private_csh_delegates_cp367_and_still_skips_the_untyped_default() {
    for minimum in [-0.0, 0.0, 0.0077] {
        let completed = completed_cp367_case(minimum);
        assert!(completed.is_some(), "finite CP367 fixture must complete");
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let direct = advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        );
        assert!(direct.is_ok());
        let Ok(direct) = direct else {
            return;
        };
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let private =
            private_default_supply_humidity_ratio_case_break_csh_counterfactual_from_direct_release(
                &runtime, unit, &system, direct,
            );
        assert!(
            private.is_some(),
            "canonical private CP367 must reconstruct"
        );
        let Some(private) = private else {
            return;
        };
        assert!(
            private.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
        );
        assert!(
            !private.dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
        );
        assert!(
            private_default_supply_humidity_ratio_case_break_csh_counterfactual_links_to_direct_release(
                &runtime, unit, &system, direct, private,
            )
        );

        let mut forged = private;
        forged.dehumidification_control_humidistat_case_completed_skip = true;
        assert!(
            !private_default_supply_humidity_ratio_case_break_csh_counterfactual_links_to_direct_release(
                &runtime, unit, &system, direct, forged,
            )
        );
    }
}

#[test]
fn private_csh_rejects_nonfinite_cp365_owner_after_lazy_direct_release() {
    for minimum in [
        f64::from_bits(0x7ff8_0000_0000_0367),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let completed = completed_cp367_case(minimum);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let direct = advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        );
        assert!(direct.is_ok());
        let Ok(direct) = direct else {
            return;
        };
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        assert!(
            private_default_supply_humidity_ratio_case_break_csh_counterfactual_from_direct_release(
                &runtime, unit, &system, direct,
            )
            .is_none()
        );
    }
}

#[test]
fn corruption_replay_and_nonzero_source_counter_reject_transactionally() {
    let Some((mut runtime, system, mut predecessor)) = completed_cp367_case(0.0077) else {
        return;
    };
    predecessor
        .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed =
        true;
    let before = runtime.clone();
    let result = advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        predecessor,
    );
    assert!(matches!(
        result,
        Err(
            super::super::PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakError::
                CoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshotMismatch { .. }
        )
    ));
    assert_eq!(runtime, before);

    let Some((mut runtime, system, predecessor)) = completed_cp367_case(0.0077) else {
        return;
    };
    assert!(
        advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let Some((mut runtime, system, predecessor)) = completed_cp367_case(0.0077) else {
        return;
    };
    let Some(unit) = runtime.units.get_mut(&system.id) else {
        return;
    };
    unit.calc_cooling_default_supply_humidity_ratio_case_break
        .dehumidification_control_default_supply_humidity_ratio_case_break_count = 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn completed_cp367_case(
    minimum: f64,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot,
)> {
    let (mut runtime, mut system, cp355) = completed_cp355_case(-1_000.0, 1.0, false)?;
    system.minimum_cooling_supply_air_humidity_ratio = minimum;
    let cp356 =
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            cp355,
        )
        .ok()?;
    let cp357 =
        advance_direct_no_oa_calc_cooling_constant_shr_case_break(&mut runtime, &system, cp356)
            .ok()?;
    let cp358 =
        advance_direct_no_oa_calc_cooling_humidistat_case_entry(&mut runtime, &system, cp357)
            .ok()?;
    let cp359 = advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp358,
    )
    .ok()?;
    let cp360 =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            cp359,
        )
        .ok()?;
    let cp361 =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut runtime,
            &system,
            cp360,
        )
        .ok()?;
    let cp362 = advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit(
        &mut runtime,
        &system,
        cp361,
    )
    .ok()?;
    let cp363 =
        advance_direct_no_oa_calc_cooling_humidistat_case_break(&mut runtime, &system, cp362)
            .ok()?;
    let cp364 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry(
        &mut runtime,
        &system,
        cp363,
    )
    .ok()?;
    let cp365 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
        &mut runtime,
        &system,
        cp364,
    )
    .ok()?;
    let cp366 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        cp365,
    )
    .ok()?;
    let cp367 =
        advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            cp366,
        )
        .ok()?;
    Some((runtime, system, cp367))
}
