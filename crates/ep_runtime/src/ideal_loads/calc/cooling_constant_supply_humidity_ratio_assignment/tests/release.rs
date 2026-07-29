use super::*;
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_case_break,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
};

#[test]
fn public_direct_none_never_reads_or_validates_typed_owner() {
    for minimum in [
        f64::from_bits(0x7ff8_0000_0000_0365),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let completed = completed_cp364_case();
        assert!(completed.is_some(), "CP364 fixture must complete");
        let Some((mut runtime, mut system, predecessor)) = completed else {
            return;
        };
        system.minimum_cooling_supply_air_humidity_ratio = minimum;
        let before = runtime.clone();
        let result =
            advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(
            result.is_ok(),
            "direct None must not validate the unexecuted owner"
        );
        let Ok(snapshot) = result else {
            return;
        };
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_read);
        assert!(snapshot.minimum_cooling_supply_air_humidity_ratio.is_none());
        assert!(!snapshot.supply_humidity_ratio_assigned);
        assert!(snapshot.assigned_supply_humidity_ratio.is_none());
        assert!(snapshot.resulting_supply_humidity_ratio.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .map(|unit| {
                    unit.calc_cooling_constant_supply_humidity_ratio_assignment
                        .transition_count
                }),
            Some(
                before
                    .units
                    .get(&system.id)
                    .map_or(1, |unit| {
                        unit.calc_cooling_constant_supply_humidity_ratio_assignment
                            .transition_count
                            + 1
                    })
            )
        );
    }
}

#[test]
fn private_canonical_assignment_copies_finite_typed_owner_bit_exact() {
    for minimum in [-0.0, 0.0, 0.0077, 0.999_999] {
        let completed = completed_cp364_case();
        assert!(completed.is_some(), "CP364 fixture must complete");
        let Some((mut runtime, mut system, predecessor)) = completed else {
            return;
        };
        system.minimum_cooling_supply_air_humidity_ratio = minimum;
        let direct =
            advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(direct.is_ok(), "CP365 direct anchor must complete");
        let Ok(direct) = direct else {
            return;
        };
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let private =
            private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release(
                &runtime, unit, &system, direct,
            );
        assert!(private.is_some(), "finite private owner must be admitted");
        let Some(private) = private else {
            return;
        };
        assert!(
            private.dehumidification_control_constant_supply_humidity_ratio_assignment_executed
        );
        assert!(private.minimum_cooling_supply_air_humidity_ratio_read);
        assert!(private.supply_humidity_ratio_assigned);
        for value in [
            private.minimum_cooling_supply_air_humidity_ratio,
            private.assigned_supply_humidity_ratio,
            private.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(minimum.to_bits()));
        }
        assert!(
            private_constant_supply_humidity_ratio_assignment_counterfactual_links_to_direct_release(
                &runtime, unit, &system, direct, private,
            )
        );
        let mut forged = private;
        forged.resulting_supply_humidity_ratio = Some(f64::from_bits(minimum.to_bits() ^ 1));
        assert!(
            !private_constant_supply_humidity_ratio_assignment_counterfactual_links_to_direct_release(
                &runtime, unit, &system, direct, forged,
            )
        );
    }
}

#[test]
fn private_canonical_assignment_rejects_nonfinite_typed_owner() {
    for minimum in [
        f64::from_bits(0x7ff8_0000_0000_0365),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let completed = completed_cp364_case();
        assert!(completed.is_some(), "CP364 fixture must complete");
        let Some((mut runtime, mut system, predecessor)) = completed else {
            return;
        };
        system.minimum_cooling_supply_air_humidity_ratio = minimum;
        let direct =
            advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(
            direct.is_ok(),
            "direct None must remain lazy for a nonfinite owner"
        );
        let Ok(direct) = direct else {
            return;
        };
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        assert!(
            private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release(
                &runtime, unit, &system, direct,
            )
            .is_none(),
            "private retained-owner admission must be finite-only"
        );
    }
}

fn completed_cp364_case() -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
)> {
    let (mut runtime, system, cp355) = completed_cp355_case(-1_000.0, 1.0, false)?;
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
    Some((runtime, system, cp364))
}
