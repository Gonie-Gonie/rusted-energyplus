use super::*;
use crate::ideal_loads::{
    DirectZonePurchasedAirCouplingOutput,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::{
    run_case, run_case_with_pressure,
};

#[test]
fn binding_orders_cp377_then_cp378_and_reconciles_without_feeding_the_calculation() {
    for pressure in [101_325.0, 2_000_000.0] {
        let Some((runtime, output)) =
            run_case_with_pressure(IdealLoadsLimit::NoLimit, None, 3_000.0, 1.0, Some(pressure))
        else {
            return;
        };
        let predecessor = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
        let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
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
            ),
        );
        assert!(
            cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            snapshot
                .predecessor_resulting_supply_humidity_ratio_original
                .map(f64::to_bits),
            predecessor
                .predecessor_resulting_supply_humidity_ratio_original
                .map(f64::to_bits),
        );
        assert_eq!(
            snapshot
                .predecessor_resulting_saturation_supply_humidity_ratio
                .map(f64::to_bits),
            predecessor
                .resulting_saturation_supply_humidity_ratio
                .map(f64::to_bits),
        );
        let result_bits = snapshot
            .resulting_supply_humidity_ratio
            .expect("active CP378 result")
            .to_bits();
        for value in [
            snapshot.minimum_supply_humidity_ratio_after_saturation_limit,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(result_bits));
        }
        assert_eq!(
            output
                .coupling
                .purchased_air
                .calculation
                .supply_humidity_ratio
                .to_bits(),
            result_bits,
        );
        assert_eq!(
            output
                .coupling
                .purchased_air
                .supply_node_update
                .humidity_ratio
                .to_bits(),
            result_bits,
        );
        assert_eq!(
            output
                .coupling
                .purchased_air
                .report
                .supply_humidity_ratio
                .to_bits(),
            result_bits,
        );
        let state =
            purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP378 lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.source_site_execution_count, 4);
    }
}

#[test]
fn binding_exercises_original_and_saturation_selection_nonvacuously() {
    let Some((_runtime, original_output)) = run_case_with_pressure(
        IdealLoadsLimit::NoLimit,
        None,
        3_000.0,
        1.0,
        Some(101_325.0),
    ) else {
        return;
    };
    let original =
        original_output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let left = original
        .original_supply_humidity_ratio_before_saturation_limit
        .expect("original operand");
    let right = original
        .saturation_supply_humidity_ratio_for_limit
        .expect("saturation operand");
    assert!(
        left < right,
        "normal pressure must select the original operand"
    );
    assert_eq!(
        original.resulting_supply_humidity_ratio.map(f64::to_bits),
        Some(left.to_bits()),
    );

    let Some((_runtime, saturation_output)) = run_case_with_pressure(
        IdealLoadsLimit::NoLimit,
        None,
        3_000.0,
        1.0,
        Some(2_000_000.0),
    ) else {
        return;
    };
    let saturation =
        saturation_output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let left = saturation
        .original_supply_humidity_ratio_before_saturation_limit
        .expect("original operand");
    let right = saturation
        .saturation_supply_humidity_ratio_for_limit
        .expect("saturation operand");
    assert!(
        right < left,
        "high pressure must select the saturation operand"
    );
    assert_eq!(
        saturation.resulting_supply_humidity_ratio.map(f64::to_bits),
        Some(right.to_bits()),
    );
}

#[test]
fn binding_reconciliation_rejects_each_corrupted_projection_without_overwriting_it() {
    let Some((_runtime, output)) = run_case(IdealLoadsLimit::NoLimit, None, 3_000.0, 1.0) else {
        return;
    };
    let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    for (field, corrupt) in [
        (
            "coupling.purchased_air.calculation.supply_humidity_ratio",
            corrupt_calculation as fn(&mut DirectZonePurchasedAirCouplingOutput),
        ),
        (
            "coupling.purchased_air.supply_node_update.humidity_ratio",
            corrupt_node,
        ),
        (
            "coupling.purchased_air.report.supply_humidity_ratio",
            corrupt_report,
        ),
    ] {
        let mut coupling = output.coupling;
        corrupt(&mut coupling);
        assert_eq!(
            reconcile_cooling_supply_humidity_ratio_saturation_limit_assignment(
                snapshot,
                &coupling,
            ),
            Err(
                DirectZonePurchasedAirScheduledCouplingError::
                    CalculationCoolingSupplyHumidityRatioSaturationLimitAssignmentNumericalInvariant {
                        field,
                    },
            ),
        );
    }
}

#[test]
fn binding_cp378_u_n_and_p_remain_complete_null_and_do_not_reconcile_numerics() {
    for (cooling_limit, capacity, load, availability) in [
        (IdealLoadsLimit::NoLimit, None, 3_000.0, 0.0),
        (IdealLoadsLimit::NoLimit, None, 0.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), 3_000.0, 1.0),
    ] {
        let Some((_runtime, output)) = run_case(cooling_limit, capacity, load, availability) else {
            return;
        };
        let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
        assert!(
            snapshot.unit_off_skipped
                || snapshot.non_cooling_skipped
                || snapshot.positive_guard_false_fallthrough_skipped
        );
        assert!(snapshot.resulting_supply_humidity_ratio.is_none());
        let mut coupling = output.coupling;
        coupling.purchased_air.calculation.supply_humidity_ratio = f64::NAN;
        coupling.purchased_air.supply_node_update.humidity_ratio = f64::INFINITY;
        coupling.purchased_air.report.supply_humidity_ratio = f64::NEG_INFINITY;
        assert_eq!(
            reconcile_cooling_supply_humidity_ratio_saturation_limit_assignment(
                snapshot, &coupling,
            ),
            Ok(()),
        );
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

fn corrupt_calculation(coupling: &mut DirectZonePurchasedAirCouplingOutput) {
    coupling.purchased_air.calculation.supply_humidity_ratio =
        different(coupling.purchased_air.calculation.supply_humidity_ratio);
}

fn corrupt_node(coupling: &mut DirectZonePurchasedAirCouplingOutput) {
    coupling.purchased_air.supply_node_update.humidity_ratio =
        different(coupling.purchased_air.supply_node_update.humidity_ratio);
}

fn corrupt_report(coupling: &mut DirectZonePurchasedAirCouplingOutput) {
    coupling.purchased_air.report.supply_humidity_ratio =
        different(coupling.purchased_air.report.supply_humidity_ratio);
}
