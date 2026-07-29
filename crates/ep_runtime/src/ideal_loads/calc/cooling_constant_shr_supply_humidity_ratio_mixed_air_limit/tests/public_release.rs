use super::completed_cp355_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle_summary,
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
        let completed = completed_cp355_case(demand, availability, capacity);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let mixed_air_before = runtime
            .units
            .get(&system.id)
            .and_then(|unit| unit.calc_cooling_mixed_air_call.latest);
        let mixed_air_witness_before = runtime.cooling_mixed_air_call_latest_witness(system.id);
        let snapshot =
            advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP356 direct release");
        assert!(
            cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
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
        assert!(match (
            mixed_air_before,
            runtime
                .units
                .get(&system.id)
                .and_then(|unit| unit.calc_cooling_mixed_air_call.latest),
        ) {
            (Some(before), Some(after)) =>
                cooling_mixed_air_call_snapshots_match_bit_exact(before, after),
            (None, None) => true,
            _ => false,
        });
        assert!(match (
            mixed_air_witness_before,
            runtime.cooling_mixed_air_call_latest_witness(system.id),
        ) {
            (Some(before), Some(after)) =>
                cooling_mixed_air_call_snapshots_match_bit_exact(before, after),
            (None, None) => true,
            _ => false,
        });
        let summary =
            purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle_summary(
                &runtime,
                system.id,
            )
            .expect("CP356 lifecycle");
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }
}

#[test]
fn supplied_corruption_identity_forge_and_replay_reject_transactionally() {
    let completed = completed_cp355_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let completed = completed_cp355_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, mut system, predecessor)) = completed else {
        return;
    };
    system.id = IdealLoadsAirSystemId(system.id.0.wrapping_add(100));
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let completed = completed_cp355_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn assert_complete_null(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed
    );
    assert!(!snapshot.supply_humidity_ratio_for_mixed_air_limit_minimum_read);
    assert!(!snapshot.mixed_air_humidity_ratio_for_minimum_read);
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(!snapshot.supply_humidity_ratio_assignment_performed);
    assert_eq!(
        [
            snapshot.supply_humidity_ratio_before_mixed_air_limit,
            snapshot.mixed_air_humidity_ratio,
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ],
        [None; 5]
    );
}
