use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneId};

use crate::zone_equipment::ZoneSysEnergyDemand;

use super::lifecycle::{
    PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS, PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER,
    PurchasedAirAvailabilityStatus, PurchasedAirCalcEntryContext, PurchasedAirCalcEntryError,
    PurchasedAirCalcEntryResetSnapshot, PurchasedAirCalcEntryRuntimeState, advance_entry_state,
    advance_purchased_air_calc_entry, purchased_air_calc_entry_lifecycle_summary,
};
use crate::ideal_loads::PurchasedAirRuntimeState;

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(4);
const ZONE: ZoneId = ZoneId(2);

#[test]
fn entry_resets_then_reads_demand_manager_and_all_schedules() {
    let mut state = PurchasedAirCalcEntryRuntimeState::new(SYSTEM);
    state.minimum_outdoor_air_mass_flow_rate_kg_per_s = 0.4;
    state.economizer_active_time_hours = 3.0;
    state.heat_recovery_active_time_hours = 2.0;
    let demand = ZoneSysEnergyDemand::from_output_required_setpoint_loads(ZONE, 900.0, -700.0);
    let snapshot = advance_entry_state(
        &mut state,
        context(demand, Some(PurchasedAirAvailabilityStatus::ForceOff)),
    );

    assert_eq!(snapshot.source_order, PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER);
    assert_eq!(snapshot.call_ordinal, 1);
    assert_eq!(PurchasedAirCalcEntryResetSnapshot::FIELD_COUNT, 12);
    assert_eq!(PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS.len(), 12);
    assert!(snapshot.reset.all_zero());
    assert_eq!(snapshot.demand.zone, demand.zone);
    assert_eq!(
        snapshot.demand.sensible_input_kind,
        demand.sensible_input_kind
    );
    assert_eq!(
        snapshot.demand.remaining_output_req_to_heat_sp_w,
        demand.remaining_output_req_to_heat_sp_w
    );
    assert_eq!(
        snapshot.demand.remaining_output_req_to_cool_sp_w,
        demand.remaining_output_req_to_cool_sp_w
    );
    assert!(snapshot.unit_defaulted_on);
    assert!(!snapshot.economizer_defaulted_on);
    assert!(snapshot.availability_manager_read_site_visited);
    assert!(snapshot.availability_manager_zone_written);
    assert_eq!(
        snapshot.copied_availability_status,
        Some(PurchasedAirAvailabilityStatus::ForceOff)
    );
    assert!(snapshot.force_off_applied);
    assert!(snapshot.overall_availability_read_site_visited);
    assert!(snapshot.heating_availability_read_site_visited);
    assert!(snapshot.cooling_availability_read_site_visited);
    assert!(!snapshot.unit_on);
    assert!(snapshot.heating_on);
    assert!(snapshot.cooling_on);
    assert!(!snapshot.unit_body_entered);

    assert_eq!(state.call_count, 1);
    assert_eq!(state.reset_count, 1);
    assert_eq!(state.demand_read_count, 1);
    assert_eq!(state.availability_manager_read_count, 1);
    assert_eq!(state.availability_manager_zone_write_count, 1);
    assert_eq!(state.availability_status_copy_count, 1);
    assert_eq!(state.force_off_count, 1);
    assert_eq!(state.overall_availability_read_count, 1);
    assert_eq!(state.heating_availability_read_count, 1);
    assert_eq!(state.cooling_availability_read_count, 1);
    assert_eq!(state.unit_off_count, 1);
    assert_eq!(state.heating_on_count, 1);
    assert_eq!(state.cooling_on_count, 1);
    assert_eq!(state.availability_manager_zone, Some(ZONE));
    assert_eq!(
        state.availability_status,
        PurchasedAirAvailabilityStatus::ForceOff
    );
    assert_eq!(state.minimum_outdoor_air_mass_flow_rate_kg_per_s, 0.0);
    assert_eq!(state.economizer_active_time_hours, 0.0);
    assert_eq!(state.heat_recovery_active_time_hours, 0.0);
    assert_eq!(state.latest, Some(snapshot));
}

#[test]
fn schedule_gates_are_independent_and_nan_is_nominally_on() {
    let mut state = PurchasedAirCalcEntryRuntimeState::new(SYSTEM);
    let demand = ZoneSysEnergyDemand::from_output_required_setpoint_loads(ZONE, 1.0, -1.0);
    let mut first = context(demand, Some(PurchasedAirAvailabilityStatus::CycleOn));
    first.overall_availability = 0.0;
    first.heating_availability = f64::NAN;
    first.cooling_availability = -0.0;
    let first = advance_entry_state(&mut state, first);
    assert!(!first.unit_on);
    assert!(first.heating_on);
    assert!(!first.cooling_on);

    let mut force_off = context(demand, Some(PurchasedAirAvailabilityStatus::ForceOff));
    force_off.overall_availability = f64::NAN;
    let force_off = advance_entry_state(&mut state, force_off);
    assert!(!force_off.unit_on);
    assert!(force_off.force_off_applied);

    let mut unallocated = context(demand, None);
    unallocated.overall_availability = f64::NAN;
    let unallocated = advance_entry_state(&mut state, unallocated);
    assert!(unallocated.unit_on);
    assert!(!unallocated.availability_manager_read_site_visited);
    assert_eq!(
        state.availability_status,
        PurchasedAirAvailabilityStatus::ForceOff
    );

    assert_eq!(state.call_count, 3);
    assert_eq!(state.overall_schedule_off_count, 1);
    assert_eq!(state.force_off_count, 1);
    assert_eq!(state.unit_body_entry_count, 1);
    assert_eq!(state.unit_off_count, 2);
    assert_eq!(state.heating_on_count, 3);
    assert_eq!(state.cooling_on_count, 2);

    let mut modes_off = context(demand, None);
    modes_off.heating_availability = 0.0;
    modes_off.cooling_availability = f64::NEG_INFINITY;
    let modes_off = advance_entry_state(&mut state, modes_off);
    assert!(modes_off.unit_on);
    assert!(!modes_off.heating_on);
    assert!(!modes_off.cooling_on);
    assert!(modes_off.unit_body_entered);
    assert_eq!(state.call_count, 4);
    assert_eq!(state.unit_body_entry_count, 2);
    assert_eq!(state.unit_off_count, 2);
}

#[test]
fn direct_entry_retains_mismatched_zone_and_aliased_nodes_without_validation() {
    let mut state = PurchasedAirCalcEntryRuntimeState::new(SYSTEM);
    let demand_zone = ZoneId(99);
    let demand = ZoneSysEnergyDemand::from_output_required_setpoint_loads(demand_zone, -2.0, 3.0);
    let shared_node = NodeId(7);
    let snapshot = advance_entry_state(
        &mut state,
        PurchasedAirCalcEntryContext {
            controlled_zone: ZONE,
            supply_node: shared_node,
            zone_node: shared_node,
            outdoor_air_node: Some(shared_node),
            recirculation_node: shared_node,
            demand,
            zone_component_availability: Some(PurchasedAirAvailabilityStatus::Invalid),
            overall_availability: f64::INFINITY,
            heating_availability: f64::NEG_INFINITY,
            cooling_availability: f64::INFINITY,
        },
    );

    assert_eq!(snapshot.controlled_zone, ZONE);
    assert_eq!(snapshot.demand.zone, demand_zone);
    assert_eq!(snapshot.supply_node, shared_node);
    assert_eq!(snapshot.zone_node, shared_node);
    assert_eq!(snapshot.outdoor_air_node, Some(shared_node));
    assert_eq!(snapshot.recirculation_node, shared_node);
    assert!(snapshot.unit_on);
    assert!(!snapshot.heating_on);
    assert!(snapshot.cooling_on);
}

#[test]
fn unknown_public_unit_rejects_advance_and_summary_without_mutation() {
    let mut runtime = PurchasedAirRuntimeState::default();
    let before = runtime.clone();
    assert_eq!(
        advance_purchased_air_calc_entry(
            &mut runtime,
            SYSTEM,
            context(
                ZoneSysEnergyDemand::from_output_required_setpoint_loads(ZONE, 1.0, -1.0),
                None
            )
        ),
        Err(PurchasedAirCalcEntryError::UnknownSystem { system: SYSTEM })
    );
    assert_eq!(runtime, before);
    assert_eq!(
        purchased_air_calc_entry_lifecycle_summary(&runtime, SYSTEM),
        Err(PurchasedAirCalcEntryError::UnknownSystem { system: SYSTEM })
    );
}

fn context(
    demand: ZoneSysEnergyDemand,
    zone_component_availability: Option<PurchasedAirAvailabilityStatus>,
) -> PurchasedAirCalcEntryContext {
    PurchasedAirCalcEntryContext {
        controlled_zone: ZONE,
        supply_node: NodeId(10),
        zone_node: NodeId(11),
        outdoor_air_node: None,
        recirculation_node: NodeId(12),
        demand,
        zone_component_availability,
        overall_availability: 1.0,
        heating_availability: 1.0,
        cooling_availability: 1.0,
    }
}
