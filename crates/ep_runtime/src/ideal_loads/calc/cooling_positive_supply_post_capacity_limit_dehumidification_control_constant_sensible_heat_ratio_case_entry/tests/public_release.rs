use super::completed_cp347_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle_summary,
};

#[test]
fn public_active_routes_complete_skip_the_constant_shr_entry_site() {
    for (demand, capacity) in [(-1_000.0, false), (-1_000.0, true), (-100_000.0, true)] {
        let completed = completed_cp347_case(demand, 1.0, capacity);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(snapshot.is_ok());
        let Ok(snapshot) = snapshot else {
            return;
        };
        assert!(snapshot.predecessor_dehumidification_control_none_case_completed);
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_case_entered);
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let state = &unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry;
        assert_eq!(
            state.dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
        assert_eq!(
            state.dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count,
            0
        );
    }
}

#[test]
fn public_u_n_p_routes_also_skip_the_case_entry_site() {
    for (demand, availability, u, n, p) in [
        (-1_000.0, 0.0, true, false, false),
        (1.0, 1.0, false, true, false),
        (-1.0e-40, 1.0, false, false, true),
    ] {
        let completed = completed_cp347_case(demand, availability, true);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(snapshot.is_ok());
        let Ok(snapshot) = snapshot else {
            return;
        };
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            (u, n, p)
        );
        assert!(!snapshot.dehumidification_control_none_case_completed_skip);
        assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_case_entered);
    }
}

#[test]
fn lifecycle_summary_reports_skip_and_duplicate_is_rejected() {
    let completed = completed_cp347_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    let Ok(snapshot) = snapshot else {
        return;
    };
    let summary =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle_summary(
            &runtime,
            system.id,
        );
    assert!(summary.is_ok());
    let Ok(summary) = summary else {
        return;
    };
    assert_eq!(summary.state.latest, Some(snapshot));
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
