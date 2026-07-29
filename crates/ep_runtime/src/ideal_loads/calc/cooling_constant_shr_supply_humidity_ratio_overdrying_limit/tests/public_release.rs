use super::completed_cp353_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit,
    cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle_summary,
};

#[test]
fn public_inherited_routes_are_exact_complete_null_skips() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 1.0, false, (false, false, false, true)),
        (-100_000.0, 1.0, true, (false, false, false, true)),
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
    ] {
        let completed = completed_cp353_case(demand, availability, capacity);
        assert!(completed.is_some());
        let Some((mut runtime, system, _, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP354 direct release");
        assert!(
            cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert_complete_null(snapshot);
        let summary =
            purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle_summary(
                &runtime,
                system.id,
            )
            .expect("CP354 lifecycle");
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }
}

#[test]
fn supplied_corruption_and_replay_reject_transactionally() {
    let completed = completed_cp353_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, _, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let completed = completed_cp353_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, _, predecessor)) = completed else {
        return;
    };
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn assert_complete_null(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed
    );
    assert!(!snapshot.supply_humidity_ratio_for_overdrying_limit_minimum_read);
    assert!(!snapshot.supply_temperature_for_humidity_ratio_inversion_read);
    assert!(!snapshot.supply_enthalpy_for_humidity_ratio_inversion_read);
    assert!(!snapshot.psychrometric_supply_humidity_ratio_evaluated);
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(!snapshot.supply_humidity_ratio_assignment_performed);
    assert_eq!(
        [
            snapshot.supply_humidity_ratio_before_overdrying_limit,
            snapshot.supply_temperature_c,
            snapshot.supply_enthalpy_j_per_kg,
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ],
        [None; 7]
    );
}
