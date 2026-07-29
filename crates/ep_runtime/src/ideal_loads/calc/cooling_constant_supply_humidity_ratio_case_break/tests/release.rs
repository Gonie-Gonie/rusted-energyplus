use super::*;
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_case_break,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
};

#[test]
fn public_direct_none_is_lazy_for_nonfinite_typed_owner() {
    for minimum in [
        f64::from_bits(0x7ff8_0000_0000_0366),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let completed = completed_cp365_case(minimum);
        assert!(completed.is_some(), "CP365 fixture must complete lazily");
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let before = runtime.clone();
        let result =
            advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(
            result.is_ok(),
            "direct None must not read or validate the CP365 typed owner"
        );
        let Ok(snapshot) = result else {
            return;
        };
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot
                .dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
        );
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .map(|unit| {
                    unit.calc_cooling_constant_supply_humidity_ratio_case_break
                        .transition_count
                }),
            before.units.get(&system.id).map(|unit| {
                unit.calc_cooling_constant_supply_humidity_ratio_case_break
                    .transition_count
                    + 1
            })
        );
    }
}

#[test]
fn public_direct_routes_skip_break_and_private_csh_uses_only_cp365_bridge() {
    for minimum in [-0.0, 0.0, 0.0077] {
        let completed = completed_cp365_case(minimum);
        assert!(completed.is_some(), "CP365 fixture must complete");
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let direct =
            advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(direct.is_ok(), "CP366 direct anchor must complete");
        let Ok(direct) = direct else {
            return;
        };
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let private =
            private_constant_supply_humidity_ratio_case_break_counterfactual_from_direct_release(
                &runtime, unit, &system, direct,
            );
        assert!(private.is_some(), "finite private CP365 must reconstruct");
        let Some(private) = private else {
            return;
        };
        assert!(
            private
                .dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
        );
        assert!(
            private_constant_supply_humidity_ratio_case_break_counterfactual_links_to_direct_release(
                &runtime, unit, &system, direct, private,
            )
        );
        let mut forged = private;
        forged.dehumidification_control_humidistat_case_completed_skip = true;
        assert!(
            !private_constant_supply_humidity_ratio_case_break_counterfactual_links_to_direct_release(
                &runtime, unit, &system, direct, forged,
            )
        );
    }
}

#[test]
fn private_canonical_break_rejects_nonfinite_owner_after_lazy_direct_release() {
    for minimum in [
        f64::from_bits(0x7ff8_0000_0000_0366),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let completed = completed_cp365_case(minimum);
        assert!(completed.is_some(), "CP365 direct fixture must remain lazy");
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let direct =
            advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(direct.is_ok(), "CP366 direct release must remain lazy");
        let Ok(direct) = direct else {
            return;
        };
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        assert!(
            private_constant_supply_humidity_ratio_case_break_counterfactual_from_direct_release(
                &runtime, unit, &system, direct,
            )
            .is_none()
        );
    }
}

#[test]
fn corruption_identity_replay_and_runtime_forge_reject_without_mutation() {
    let completed = completed_cp365_case(0.0077);
    assert!(completed.is_some(), "CP365 fixture must complete");
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.resulting_supply_humidity_ratio = Some(0.0088);
    let before = runtime.clone();
    let result = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        predecessor,
    );
    assert!(matches!(
        result,
        Err(
            super::super::PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakError::
                CoolingConstantSupplyHumidityRatioAssignmentSnapshotMismatch { .. }
        )
    ));
    assert_eq!(runtime, before);

    let completed = completed_cp365_case(0.0077);
    assert!(completed.is_some(), "CP365 identity fixture must complete");
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.system = ep_model::IdealLoadsAirSystemId(system.id.0 + 1);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let completed = completed_cp365_case(0.0077);
    assert!(completed.is_some(), "CP365 replay fixture must complete");
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let first = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        predecessor,
    );
    assert!(first.is_ok());
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let completed = completed_cp365_case(0.0077);
    assert!(
        completed.is_some(),
        "CP365 runtime-forge fixture must complete"
    );
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let Some(unit) = runtime.units.get_mut(&system.id) else {
        return;
    };
    unit.calc_cooling_constant_supply_humidity_ratio_case_break
        .transition_count = 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn completed_cp365_case(
    minimum: f64,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
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
    let cp364 =
        advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry(
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
    Some((runtime, system, cp365))
}
