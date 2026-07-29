use super::completed_cp354_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit,
    cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle_summary,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn public_inherited_routes_are_exact_complete_null_skips() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 1.0, false, (false, false, false, true)),
        (-100_000.0, 1.0, true, (false, false, false, true)),
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
    ] {
        let completed = completed_cp354_case(demand, availability, capacity);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP355 direct release");
        assert!(
            cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot_is_exact_direct_release(
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
            purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle_summary(
                &runtime,
                system.id,
            )
            .expect("CP355 lifecycle");
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }
}

#[test]
fn supplied_corruption_identity_forge_and_replay_reject_transactionally() {
    let completed = completed_cp354_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let completed = completed_cp354_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, mut system, predecessor)) = completed else {
        return;
    };
    system.id = IdealLoadsAirSystemId(system.id.0.wrapping_add(100));
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let completed = completed_cp354_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, mut system, predecessor)) = completed else {
        return;
    };
    system.minimum_cooling_supply_air_humidity_ratio = f64::from_bits(0x7ff8_0000_0000_00a5);
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn assert_complete_null(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed
    );
    assert!(!snapshot.supply_humidity_ratio_for_minimum_limit_maximum_read);
    assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read);
    assert!(!snapshot.source_shaped_two_argument_maximum_evaluated);
    assert!(!snapshot.supply_humidity_ratio_assignment_performed);
    assert_eq!(
        [
            snapshot.supply_humidity_ratio_before_minimum_limit,
            snapshot.minimum_cooling_supply_air_humidity_ratio,
            snapshot.maximum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ],
        [None; 5]
    );
}
