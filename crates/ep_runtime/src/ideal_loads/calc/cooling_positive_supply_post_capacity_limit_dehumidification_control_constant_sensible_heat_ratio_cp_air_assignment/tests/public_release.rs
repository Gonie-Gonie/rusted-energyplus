use super::completed_cp348_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle_summary,
};

#[test]
fn public_none_route_complete_skips_all_three_cp349_sites() {
    for (demand, capacity) in [(-1_000.0, false), (-1_000.0, true), (-100_000.0, true)] {
        let completed = completed_cp348_case(demand, 1.0, capacity);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(snapshot.is_ok());
        let Ok(snapshot) = snapshot else {
            return;
        };
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        );
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(!snapshot.psychrometric_cp_air_evaluated);
        assert!(snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none());
        assert!(!snapshot.cp_air_assigned);
        assert!(snapshot.cp_air_j_per_kg_k.is_none());
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let state = &unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment;
        assert_eq!(
            state.dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            state.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn public_u_n_p_routes_also_skip_every_cp349_value() {
    for (demand, availability, u, n, p) in [
        (-1_000.0, 0.0, true, false, false),
        (1.0, 1.0, false, true, false),
        (-1.0e-40, 1.0, false, false, true),
    ] {
        let completed = completed_cp348_case(demand, availability, true);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
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
        assert!(
            !snapshot
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        );
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(snapshot.cp_air_j_per_kg_k.is_none());
    }
}

#[test]
fn lifecycle_summary_reports_skip_and_duplicate_is_transactional() {
    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    let Ok(snapshot) = snapshot else {
        return;
    };
    let summary =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle_summary(
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
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
