use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState,
    cooling_humidistat_case_break_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp362_then_cp363_before_numerical_and_does_not_feed_result() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP363 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor =
            output.calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
        let snapshot = output.calculation_cooling_humidistat_case_break;
        assert_eq!(
            (
                snapshot.system,
                snapshot.parent_call_ordinal,
                snapshot.controlled_zone,
            ),
            (
                predecessor.system,
                predecessor.parent_call_ordinal,
                predecessor.controlled_zone,
            )
        );
        assert!(cooling_humidistat_case_break_snapshot_is_exact_direct_release(snapshot));
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        );
        assert!(!snapshot.dehumidification_control_humidistat_case_exited_via_break);
        assert!(
            !snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        );

        let summary = purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary(
            &runtime,
            output.initialization.system,
        );
        assert!(summary.is_ok(), "CP363 lifecycle summary must be available");
        let Ok(summary) = summary else {
            return;
        };
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(
            summary
                .state
                .dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            summary
                .state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            0
        );
        assert_eq!(
            summary
                .state
                .dehumidification_control_humidistat_case_break_count,
            0
        );
        assert_eq!(summary.state.source_site_execution_count, 0);

        let cp345 = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
        let numerical = output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio;
        assert_eq!(
            cp345.assigned_supply_humidity_ratio.map(f64::to_bits),
            Some(numerical.to_bits())
        );
        assert!(numerical.is_finite());
    }
}

#[test]
fn binding_cp363_is_complete_skip_for_direct_none() {
    for (load, availability, capacity, expected) in [
        (3_000.0, 0.0, None, (true, false, false, false)),
        (0.0, 1.0, None, (false, true, false, false)),
        (3_000.0, 1.0, Some(-0.0), (false, false, true, false)),
        (3_000.0, 1.0, Some(1.0), (false, false, false, true)),
    ] {
        let limit = if capacity.is_some() {
            IdealLoadsLimit::LimitCapacity
        } else {
            IdealLoadsLimit::NoLimit
        };
        let case = run_case(limit, capacity, load, availability);
        assert!(case.is_some(), "CP363 route fixture must succeed");
        let Some((_, output)) = case else {
            return;
        };
        let snapshot = output.calculation_cooling_humidistat_case_break;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert!(cooling_humidistat_case_break_snapshot_is_exact_direct_release(snapshot));
        assert!(!snapshot.dehumidification_control_humidistat_case_exited_via_break);
    }
}

#[test]
fn binding_rejects_corrupt_cp362_without_mutation() {
    let case = run_case(IdealLoadsLimit::NoLimit, None, 3_000.0, 1.0);
    assert!(case.is_some(), "CP363 corruption fixture must succeed");
    let Some((mut runtime, output)) = case else {
        return;
    };
    let predecessor = output.calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
    let system_id = output.initialization.system;
    let (model, _) = fixture(|typed| {
        typed.ideal_loads_air_systems[0].dehumidification_control_type =
            DehumidificationControlType::None;
        typed.ideal_loads_air_systems[0].minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    });
    let system = &model.typed.ideal_loads_air_systems[0];

    let unit = runtime.units.get_mut(&system_id);
    assert!(unit.is_some(), "CP363 runtime unit must exist");
    let Some(unit) = unit else {
        return;
    };
    unit.calc_cooling_humidistat_case_break =
        PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState::new(system_id);
    runtime.clear_cooling_humidistat_case_break_latest_witness(system_id);

    let mut canonical_pending = runtime.clone();
    assert!(
        crate::ideal_loads::binding::cooling_humidistat_case_break::advance_cooling_humidistat_case_break(
            &mut canonical_pending,
            system,
            predecessor,
        )
        .is_ok()
    );

    let unit = runtime.units.get_mut(&system_id);
    assert!(unit.is_some(), "CP362 retained unit must exist");
    let Some(unit) = unit else {
        return;
    };
    let latest = unit
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest
        .as_mut();
    assert!(latest.is_some(), "CP362 retained snapshot must exist");
    let Some(latest) = latest else {
        return;
    };
    latest.source_order = &[];
    let before = runtime.clone();

    assert!(
        crate::ideal_loads::binding::cooling_humidistat_case_break::advance_cooling_humidistat_case_break(
            &mut runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
