use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, NodeId, ZoneId};

use crate::zone_equipment::ZoneSysEnergyDemand;

use super::{
    cooling_entry_gate::{
        PurchasedAirCalcCoolingEntryGateRuntimeState, PurchasedAirTemperatureControlType,
        advance_cooling_entry_gate_state,
    },
    cooling_oa_max_flow_body::{
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
        PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
        PurchasedAirCalcCoolingOaMaxFlowBodySnapshot, advance_cooling_oa_max_flow_body_state,
    },
    cooling_oa_max_flow_gate::{
        PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
        PurchasedAirCalcCoolingOaMaxFlowGateSnapshot, advance_cooling_oa_max_flow_gate_state,
    },
    lifecycle::{
        PurchasedAirAvailabilityStatus, PurchasedAirCalcEntryContext,
        PurchasedAirCalcEntryRuntimeState, advance_entry_state,
    },
    minimum_oa_prefix::{
        PurchasedAirCalcMinimumOaPrefixRuntimeState, advance_minimum_oa_prefix_state,
    },
};

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);
const ZONE: ZoneId = ZoneId(3);

#[test]
fn body_skip_partitions_expose_no_sites_or_nonfinite_values() {
    let predecessors = [
        cp313_predecessor(0.0, -1.0, IdealLoadsLimit::LimitFlowRate, 2.0, 1.0),
        cp313_predecessor(1.0, 1.0, IdealLoadsLimit::LimitFlowRate, 2.0, 1.0),
        cp313_predecessor(1.0, -1.0, IdealLoadsLimit::LimitFlowRate, 1.0, 1.0),
    ];
    let mut state = PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState::new(SYSTEM);
    let snapshots = predecessors.map(|predecessor| {
        advance_cooling_oa_max_flow_body_state(
            &mut state,
            predecessor,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
        )
    });

    assert!(snapshots[0].unit_off_skipped);
    assert!(snapshots[1].non_cooling_skipped);
    assert!(snapshots[2].active_guard_false_economizer_fallthrough);
    for snapshot in snapshots {
        assert_skipped(snapshot);
    }
    assert_eq!(state.transition_count, 3);
    assert_eq!(state.body_skip_count, 3);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.active_guard_false_economizer_fallthrough_count, 1);
    assert_eq!(state.body_entry_count, 0);
    assert_eq!(state.outdoor_air_mass_flow_rate_read_count, 0);
    assert_eq!(state.standard_air_density_read_count, 0);
    assert_eq!(state.characterized_total_warning_error_increment_count, 0);
}

#[test]
fn first_body_entry_reaches_first_warning_sites_and_clamps() {
    let predecessor = cp313_predecessor(1.0, -1.0, IdealLoadsLimit::LimitFlowRate, 4.0, 1.0);
    assert!(predecessor.maximum_cooling_flow_body_entered);
    let mut state = PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState::new(SYSTEM);

    let snapshot =
        advance_cooling_oa_max_flow_body_state(&mut state, predecessor, 4.0, 2.0, 0.5, 1.0);

    assert_eq!(
        snapshot.source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
    );
    assert_eq!(
        snapshot.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        snapshot.recurring_warning_child_source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER
    );
    assert!(!snapshot.body_skipped);
    assert_eq!(snapshot.outdoor_air_volume_flow_rate_m3_per_s, Some(2.0));
    assert_eq!(snapshot.warning_counter_before, Some(0));
    assert_eq!(snapshot.first_warning_predicate_satisfied, Some(true));
    assert!(snapshot.first_warning_branch_entered);
    assert!(snapshot.warning_counter_incremented);
    assert_eq!(snapshot.warning_counter_after, Some(1));
    assert!(snapshot.first_warning_call_site_reached);
    assert!(snapshot.maximum_cooling_air_volume_flow_rate_read);
    assert_eq!(
        snapshot.maximum_cooling_air_volume_flow_rate_m3_per_s,
        Some(0.5)
    );
    assert!(snapshot.continue_warning_call_site_reached);
    assert!(snapshot.continue_warning_timestamp_call_site_reached);
    assert!(!snapshot.recurring_warning_branch_entered);
    assert_eq!(snapshot.characterized_recurring_warning_index_before, None);
    assert_eq!(snapshot.characterized_recurring_warning_index_after, None);
    assert!(snapshot.characterized_total_warning_error_incremented);
    assert!(snapshot.outdoor_air_mass_flow_clamp_assignment_performed);
    assert_eq!(
        snapshot.outdoor_air_mass_flow_rate_after_clamp_kg_per_s,
        Some(1.0)
    );
    assert_eq!(state.outdoor_air_flow_max_cooling_output_error_count, 1);
    assert_eq!(state.outdoor_air_flow_max_cooling_output_index, 0);
    assert_eq!(state.first_warning_branch_count, 1);
    assert_eq!(state.recurring_warning_branch_count, 0);
    assert_eq!(state.characterized_total_warning_error_increment_count, 1);
}

#[test]
fn later_entries_allocate_then_reuse_recurring_index_and_update_max_only() {
    let first = cp313_predecessor(1.0, -1.0, IdealLoadsLimit::LimitFlowRate, 4.0, 1.0);
    let second = cp313_predecessor(1.0, -1.0, IdealLoadsLimit::LimitFlowRate, 6.0, 1.0);
    let third = cp313_predecessor(1.0, -1.0, IdealLoadsLimit::LimitFlowRate, 2.0, 1.0);
    let fourth = cp313_predecessor(1.0, -1.0, IdealLoadsLimit::LimitFlowRate, 8.0, 1.0);
    let mut state = PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState::new(SYSTEM);
    let _first = advance_cooling_oa_max_flow_body_state(&mut state, first, 4.0, 2.0, 0.5, 1.0);

    let allocation = advance_cooling_oa_max_flow_body_state(&mut state, second, 6.0, 2.0, 0.5, 1.0);
    assert_eq!(allocation.warning_counter_before, Some(1));
    assert_eq!(allocation.first_warning_predicate_satisfied, Some(false));
    assert!(allocation.recurring_warning_branch_entered);
    assert!(allocation.recurring_warning_call_site_reached);
    assert_eq!(
        allocation.recurring_warning_report_maximum_input_m3_per_s,
        Some(3.0)
    );
    assert!(allocation.characterized_recurring_warning_index_allocated_on_call);
    assert!(!allocation.characterized_recurring_warning_index_reused_on_call);
    assert_eq!(
        allocation.characterized_recurring_warning_index_before,
        Some(0)
    );
    assert_eq!(
        allocation.characterized_recurring_warning_index_after,
        Some(1)
    );
    assert_eq!(
        allocation.characterized_recurring_warning_occurrence_ordinal,
        Some(1)
    );
    assert_eq!(
        allocation.characterized_recurring_warning_report_maximum_m3_per_s,
        Some(3.0)
    );
    assert!(!allocation.maximum_cooling_air_volume_flow_rate_read);
    assert_eq!(
        allocation.maximum_cooling_air_volume_flow_rate_m3_per_s,
        None
    );

    let reuse = advance_cooling_oa_max_flow_body_state(&mut state, third, 2.0, 2.0, 0.5, 1.0);
    assert!(!reuse.characterized_recurring_warning_index_allocated_on_call);
    assert!(reuse.characterized_recurring_warning_index_reused_on_call);
    assert_eq!(reuse.characterized_recurring_warning_index_before, Some(1));
    assert_eq!(reuse.characterized_recurring_warning_index_after, Some(1));
    assert_eq!(
        reuse.recurring_warning_report_maximum_input_m3_per_s,
        Some(1.0)
    );
    assert_eq!(
        reuse.characterized_recurring_warning_occurrence_ordinal,
        Some(2)
    );
    assert_eq!(
        reuse.characterized_recurring_warning_report_maximum_m3_per_s,
        Some(3.0)
    );
    assert_eq!(
        reuse.outdoor_air_mass_flow_rate_after_clamp_kg_per_s,
        Some(1.0)
    );
    let higher = advance_cooling_oa_max_flow_body_state(&mut state, fourth, 8.0, 2.0, 0.5, 1.0);
    assert!(higher.characterized_recurring_warning_index_reused_on_call);
    assert_eq!(
        higher.characterized_recurring_warning_report_maximum_m3_per_s,
        Some(4.0)
    );
    assert_eq!(state.outdoor_air_flow_max_cooling_output_index, 1);
    assert_eq!(
        state.characterized_recurring_warning_index_allocation_count,
        1
    );
    assert_eq!(state.characterized_recurring_warning_index_reuse_count, 2);
    assert_eq!(state.characterized_recurring_warning_occurrence_count, 3);
    assert_eq!(
        state.characterized_recurring_warning_report_maximum_m3_per_s,
        Some(4.0)
    );
    assert_eq!(state.characterized_total_warning_error_increment_count, 4);
    assert_eq!(state.outdoor_air_mass_flow_clamp_assignment_count, 4);
}

fn assert_skipped(snapshot: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot) {
    assert!(snapshot.body_skipped);
    assert!(!snapshot.outdoor_air_mass_flow_rate_read);
    assert_eq!(
        snapshot.outdoor_air_mass_flow_rate_before_clamp_kg_per_s,
        None
    );
    assert!(!snapshot.standard_air_density_read);
    assert_eq!(snapshot.standard_air_density_kg_per_m3, None);
    assert!(!snapshot.outdoor_air_volume_flow_rate_calculated);
    assert_eq!(snapshot.outdoor_air_volume_flow_rate_m3_per_s, None);
    assert!(!snapshot.warning_counter_read);
    assert_eq!(snapshot.warning_counter_before, None);
    assert_eq!(snapshot.first_warning_predicate_satisfied, None);
    assert!(!snapshot.first_warning_branch_entered);
    assert!(!snapshot.recurring_warning_branch_entered);
    assert!(!snapshot.maximum_cooling_air_mass_flow_rate_read);
    assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
    assert!(!snapshot.outdoor_air_mass_flow_clamp_assignment_performed);
    assert_eq!(
        snapshot.outdoor_air_mass_flow_rate_after_clamp_kg_per_s,
        None
    );
}

fn cp313_predecessor(
    overall_availability: f64,
    cooling_demand_w: f64,
    limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingOaMaxFlowGateSnapshot {
    let mut entry_state = PurchasedAirCalcEntryRuntimeState::new(SYSTEM);
    let entry = advance_entry_state(
        &mut entry_state,
        PurchasedAirCalcEntryContext {
            controlled_zone: ZONE,
            supply_node: NodeId(10),
            zone_node: NodeId(11),
            outdoor_air_node: None,
            recirculation_node: NodeId(12),
            demand: ZoneSysEnergyDemand::from_output_required_setpoint_loads(
                ZONE,
                1.0,
                cooling_demand_w,
            ),
            zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
            overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    );
    let mut minimum_oa_state = PurchasedAirCalcMinimumOaPrefixRuntimeState::new(SYSTEM);
    let minimum_oa =
        advance_minimum_oa_prefix_state(&mut entry_state, &mut minimum_oa_state, entry);
    let mut cooling_entry_state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);
    let cooling_entry = advance_cooling_entry_gate_state(
        &mut cooling_entry_state,
        entry,
        minimum_oa,
        PurchasedAirTemperatureControlType::DualHeatCool,
    );
    let mut gate_state = PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(SYSTEM);
    advance_cooling_oa_max_flow_gate_state(
        &mut gate_state,
        cooling_entry,
        limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
    )
}
