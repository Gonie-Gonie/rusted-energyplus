use super::completed_cp352_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary,
};

#[test]
fn public_none_and_inherited_routes_are_complete_null_no_read_skips() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 1.0, false, (false, false, false, true)),
        (-100_000.0, 1.0, true, (false, false, false, true)),
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
    ] {
        let completed = completed_cp352_case(demand, availability, capacity);
        assert!(completed.is_some());
        let Some((mut runtime, system, predecessor)) = completed else {
            return;
        };
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
                &mut runtime,
                &system,
                predecessor,
            );
        assert!(snapshot.is_ok());
        let Ok(snapshot) = snapshot else {
            return;
        };
        assert!(
            cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
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
        assert_complete_null_skip(snapshot);
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let state = &unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit;
        assert_eq!(state.transition_count, 1);
        assert_eq!(
            state
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
        for count in [
            state.supply_enthalpy_for_overdrying_limit_maximum_read_count,
            state.supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count,
            state.psychrometric_minimum_supply_enthalpy_evaluation_count,
            state.source_shaped_two_argument_maximum_evaluation_count,
            state.supply_enthalpy_assignment_write_count,
        ] {
            assert_eq!(count, 0);
        }
    }
}

#[test]
fn lifecycle_summary_and_replay_are_exact_and_transactional() {
    let completed = completed_cp352_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    let Ok(snapshot) = snapshot else {
        return;
    };
    let summary =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary(
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
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
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
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
    );
    assert!(!snapshot.supply_enthalpy_for_overdrying_limit_maximum_read);
    assert!(
        snapshot
            .supply_enthalpy_before_overdrying_limit_j_per_kg
            .is_none()
    );
    assert!(!snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read);
    assert!(snapshot.supply_temperature_c.is_none());
    assert!(!snapshot.psychrometric_minimum_supply_enthalpy_evaluated);
    assert!(
        snapshot
            .psychrometric_minimum_supply_enthalpy_j_per_kg
            .is_none()
    );
    assert!(!snapshot.source_shaped_two_argument_maximum_evaluated);
    assert!(snapshot.maximum_supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.supply_enthalpy_assignment_performed);
    assert!(snapshot.assigned_supply_enthalpy_j_per_kg.is_none());
    assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
}
