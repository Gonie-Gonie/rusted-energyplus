use super::completed_cp350_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle_summary,
};

#[test]
fn public_none_and_inherited_routes_are_complete_null_skips() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 1.0, false, (false, false, false, true)),
        (-100_000.0, 1.0, true, (false, false, false, true)),
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
    ] {
        let completed = completed_cp350_case(demand, availability, capacity);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment(
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
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert_complete_null_skip(snapshot);
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let state = &unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment;
        assert_eq!(state.transition_count, 1);
        assert_eq!(
            state
                .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
        assert_eq!(
            state
                .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count,
            unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment
                .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count
        );
    }
}

#[test]
fn lifecycle_summary_and_replay_are_exact_and_transactional() {
    let completed = completed_cp350_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    let Ok(snapshot) = snapshot else {
        return;
    };
    let summary =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle_summary(
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
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn assert_complete_null_skip(
    snapshot:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed
    );
    assert!(!snapshot.cooling_sensible_output_read);
    assert!(snapshot.cooling_sensible_output_w.is_none());
    assert!(!snapshot.cooling_sensible_heat_ratio_read);
    assert!(snapshot.cooling_sensible_heat_ratio.is_none());
    assert!(!snapshot.cooling_total_output_calculated);
    assert!(snapshot.calculated_cooling_total_output_w.is_none());
    assert!(!snapshot.cooling_total_output_assigned);
    assert!(snapshot.cooling_total_output_w.is_none());
}
