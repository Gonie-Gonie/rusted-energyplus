//! Exhaustive CP409-snapshot-to-CP410 compression regression.

use ep_model::DehumidificationControlType as D;

use super::{State, all_routes, inactive_predecessor};
use crate::ideal_loads::calc::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_route as predecessor_snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_route as snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Snapshot,
};

#[test]
fn all_thirty_six_valid_cp409_snapshots_advance_through_cp410_compression_bit_exact() {
    let expected_routes = all_routes();
    let mut state = State::new(inactive_predecessor().system);

    for (logical_index, expected_route) in expected_routes.iter().copied().enumerate() {
        let predecessor = predecessor_for_route(expected_route, logical_index + 1);
        let Some(cp409_route) = predecessor_snapshot_route(predecessor) else {
            assert!(
                predecessor_snapshot_route(predecessor).is_some(),
                "CP409 predecessor route {logical_index} must be exact"
            );
            continue;
        };
        assert_eq!(
            cp409_route.predecessor_index,
            expected_route.predecessor_index
        );
        assert_eq!(
            cp409_route.predecessor_guard_false_fallthrough,
            expected_route.predecessor_guard_false_fallthrough
        );
        assert_eq!(
            cp409_route.predecessor_maximum_capacity_assignment_executed,
            expected_route.predecessor_maximum_capacity_assignment_executed
        );
        assert_eq!(
            cp409_route.active,
            expected_route.predecessor_shared_case_break_executed
        );

        let Some(snapshot) = advance(&mut state, predecessor) else {
            assert!(
                advance(&mut State::new(predecessor.system), predecessor).is_some(),
                "CP410 transition {logical_index} must succeed"
            );
            continue;
        };
        let Some(reconstructed) = snapshot_route(snapshot) else {
            assert!(
                snapshot_route(snapshot).is_some(),
                "CP410 compressed route {logical_index} must reconstruct"
            );
            continue;
        };
        assert_eq!(reconstructed, expected_route);
        assert!(!snapshot.dehumidification_control_default_case_exited_via_break);
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
            expected_route.predecessor_shared_case_break_executed
        );
        assert_carriers_are_bit_exact(predecessor, snapshot);
    }

    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 36);
    assert_eq!(state.dehumidification_control_default_case_break_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
}

fn predecessor_for_route(route: super::RetainedRoute, ordinal: usize) -> Predecessor {
    let index = route.predecessor_index;
    let mut snapshot = inactive_predecessor();
    snapshot.parent_call_ordinal = ordinal;
    snapshot.unit_off_skipped = index == 0;
    snapshot.non_cooling_skipped = index == 1;
    snapshot.positive_guard_false_fallthrough_skipped = index == 2;

    if index >= 3 {
        set_lineage(&mut snapshot, lineage(index));
        set_stage(&mut snapshot, if index < 18 { (index - 3) % 3 } else { 3 });
    }
    if index >= 18 {
        set_switch_case(&mut snapshot, index, route);
    }
    set_carriers(&mut snapshot, index, ordinal);
    snapshot
}

fn lineage(index: usize) -> usize {
    match index {
        3..=17 => (index - 3) / 3,
        18..=21 => 0,
        22..=25 => 1,
        26 => 2,
        27 => 3,
        28..=29 => 4,
        _ => 0,
    }
}

fn set_lineage(snapshot: &mut Predecessor, lineage: usize) {
    snapshot.heating_availability_guard_false_fallthrough = lineage == 0;
    snapshot.humidification_control_guard_false_fallthrough = lineage == 1;
    snapshot.dehumidification_control_humidistat_maximum_assignment_executed = lineage == 2;
    snapshot.dehumidification_control_none_maximum_assignment_executed = lineage == 3;
    snapshot.dehumidification_control_guard_false_fallthrough = lineage == 4;
}

fn set_stage(snapshot: &mut Predecessor, stage: usize) {
    snapshot.predecessor_capacity_limit_guard_evaluated = true;
    if stage == 0 {
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough = true;
        return;
    }

    snapshot.predecessor_capacity_limit_body_entered = true;
    snapshot.predecessor_dehumidification_guard_evaluated = true;
    if stage == 1 {
        snapshot.predecessor_dehumidification_guard_false_fallthrough = true;
        return;
    }

    snapshot.predecessor_dehumidification_body_entered = true;
    snapshot.predecessor_dehumidification_total_output_assignment_executed = true;
    snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated = true;
    if stage == 2 {
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough = true;
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough = true;
        return;
    }

    snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered = true;
    snapshot.dehumidification_total_output_maximum_capacity_assignment_executed = true;
}

fn set_switch_case(snapshot: &mut Predecessor, index: usize, route: super::RetainedRoute) {
    let selector = match index {
        18 | 22 | 28 => D::ConstantSensibleHeatRatio,
        19 | 23 | 26 => D::Humidistat,
        20 | 24 | 27 => D::None,
        _ => D::ConstantSupplyHumidityRatio,
    };
    let constant_shr = matches!(index, 18 | 22 | 28);
    let humidistat = matches!(index, 19 | 23 | 26);
    let shared = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);

    snapshot.predecessor_supply_enthalpy_assignment_executed = true;
    snapshot.predecessor_dehumidification_control_type_read = true;
    snapshot.predecessor_dehumidification_control_type = Some(selector);
    snapshot.predecessor_dehumidification_control_switch_dispatched = true;
    snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered =
        constant_shr;
    snapshot
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break =
        constant_shr;
    snapshot.predecessor_dehumidification_control_humidistat_case_entered = humidistat;
    snapshot
        .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed =
        humidistat;
    snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break = humidistat;
    snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered = shared;
    snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough =
        route.predecessor_guard_false_fallthrough;
    snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed =
        route.predecessor_maximum_capacity_assignment_executed;
    snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break = shared;
}

fn set_carriers(snapshot: &mut Predecessor, index: usize, ordinal: usize) {
    let ordinal = ordinal as u64;
    let humidity =
        matches!(index, 18..=29).then(|| f64::from_bits(0x3f80_0000_0000_0000 + ordinal));
    let enthalpy = matches!(index, 5 | 8 | 11 | 14 | 17..=29)
        .then(|| f64::from_bits(0xc0e0_0000_0000_0000 + ordinal));
    let temperature = (index >= 3).then(|| f64::from_bits(0x7ff8_0000_0000_1000 + ordinal));
    snapshot.predecessor_cp408_resulting_supply_humidity_ratio = humidity;
    snapshot.predecessor_cp408_resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.predecessor_cp408_resulting_supply_temperature_c = temperature;
    snapshot.resulting_supply_humidity_ratio = humidity;
    snapshot.resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.resulting_supply_temperature_c = temperature;
}

fn assert_carriers_are_bit_exact(predecessor: Predecessor, snapshot: Snapshot) {
    for (cp409, retained, resulting) in [
        (
            predecessor.resulting_supply_humidity_ratio,
            snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg,
            snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            predecessor.resulting_supply_temperature_c,
            snapshot.predecessor_cp409_resulting_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
        ),
    ] {
        assert_eq!(cp409.map(f64::to_bits), retained.map(f64::to_bits));
        assert_eq!(cp409.map(f64::to_bits), resulting.map(f64::to_bits));
    }
}
