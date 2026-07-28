use super::completed_cp346_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary,
};

#[test]
fn public_g_f_l_routes_execute_the_exact_none_case_and_preserve_owner_bits() {
    for (demand, capacity) in [(-1_000.0, false), (-1_000.0, true), (-100_000.0, true)] {
        let (mut runtime, system, predecessor) =
            completed_cp346_case(demand, 1.0, capacity).expect("CP346 active test prefix");
        let owner_bits = runtime
            .units
            .get(&system.id)
            .and_then(|unit| unit.calc_cooling_mixed_air_call.latest)
            .and_then(|owner| owner.mixed_air_humidity_ratio)
            .map(f64::to_bits);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP347 active route must complete");
        assert!(snapshot.dehumidification_control_none_case_entered);
        assert!(snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.supply_humidity_ratio_assignment_performed);
        assert!(snapshot.dehumidification_control_none_case_exited_via_break);
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            owner_bits
        );
        assert_eq!(
            snapshot.assigned_supply_humidity_ratio.map(f64::to_bits),
            owner_bits
        );
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
            owner_bits
        );
        let unit = runtime.units.get(&system.id).expect("known unit");
        let state = &unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
        assert_eq!(state.dehumidification_control_none_case_completion_count, 1);
        assert_eq!(state.source_site_execution_count, 4);
        assert_eq!(state.dehumidification_control_none_case_entry_count, 1);
        assert_eq!(state.mixed_air_humidity_ratio_read_count, 1);
        assert_eq!(state.supply_humidity_ratio_assignment_count, 1);
        assert_eq!(state.dehumidification_control_none_case_break_count, 1);
    }
}

#[test]
fn public_u_n_p_routes_skip_all_none_case_sites() {
    for (demand, availability, u, n, p) in [
        (-1_000.0, 0.0, true, false, false),
        (1.0, 1.0, false, true, false),
        (-1.0e-40, 1.0, false, false, true),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp346_case(demand, availability, true).expect("CP346 skip test prefix");
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP347 skip route must complete");
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            (u, n, p)
        );
        assert!(!snapshot.dehumidification_control_none_case_entered);
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(!snapshot.supply_humidity_ratio_assignment_performed);
        assert!(snapshot.assigned_supply_humidity_ratio.is_none());
        assert!(snapshot.resulting_supply_humidity_ratio.is_none());
        assert!(!snapshot.dehumidification_control_none_case_exited_via_break);
    }
}

#[test]
fn lifecycle_summary_reports_completion_and_duplicate_is_rejected() {
    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 lifecycle test prefix");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP347 must complete");
    let summary =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary(
            &runtime,
            system.id,
        )
        .expect("CP347 summary must exist");
    assert_eq!(summary.state.latest, Some(snapshot));
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
}
