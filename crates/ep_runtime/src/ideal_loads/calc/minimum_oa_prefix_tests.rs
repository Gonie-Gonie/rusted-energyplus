use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneId};

use crate::zone_equipment::ZoneSysEnergyDemand;

use super::{
    lifecycle::{
        PurchasedAirAvailabilityStatus, PurchasedAirCalcEntryContext,
        PurchasedAirCalcEntryRuntimeState, advance_entry_state,
    },
    minimum_oa_prefix::{
        PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
        PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER,
        PurchasedAirCalcMinimumOaPrefixRuntimeState, advance_minimum_oa_prefix_state,
    },
};

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(4);
const ZONE: ZoneId = ZoneId(2);

#[test]
fn active_no_oa_prefix_rewrites_retained_minimum_and_zeros_both_effects() {
    let mut entry_state = PurchasedAirCalcEntryRuntimeState::new(SYSTEM);
    entry_state.minimum_outdoor_air_mass_flow_rate_kg_per_s = 0.7;
    let entry = advance_entry_state(&mut entry_state, context(1.0, 1.0, 1.0));
    let mut state = PurchasedAirCalcMinimumOaPrefixRuntimeState::new(SYSTEM);

    let snapshot = advance_minimum_oa_prefix_state(&mut entry_state, &mut state, entry);

    assert_eq!(snapshot.source, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE);
    assert_eq!(
        snapshot.minimum_oa_child_source,
        PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER
    );
    assert_eq!(snapshot.parent_call_ordinal, 1);
    assert_eq!(snapshot.controlled_zone, ZONE);
    assert!(snapshot.unit_body_entered);
    assert!(snapshot.zone_heat_balance_reference_bound);
    assert!(snapshot.minimum_oa_child_called);
    assert!(snapshot.minimum_oa_child_no_outdoor_air_route);
    assert_eq!(
        snapshot.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert!(snapshot.retained_minimum_outdoor_air_write_performed);
    assert!(snapshot.ems_override_flag_read);
    assert_eq!(snapshot.ems_override_enabled, Some(false));
    assert!(!snapshot.ems_override_applied);
    assert_eq!(
        snapshot.working_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert!(snapshot.outdoor_air_flag_read);
    assert_eq!(snapshot.outdoor_air_enabled, Some(false));
    assert!(snapshot.no_outdoor_air_zero_branch_entered);
    assert_eq!(snapshot.psychrometric_call_count, 0);
    assert_eq!(snapshot.minimum_outdoor_air_sensible_output_w, Some(0.0));
    assert_eq!(
        snapshot.minimum_outdoor_air_moisture_output_kg_per_s,
        Some(0.0)
    );
    assert_eq!(entry_state.minimum_outdoor_air_mass_flow_rate_kg_per_s, 0.0);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.source_execution_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.zone_heat_balance_reference_count, 1);
    assert_eq!(state.minimum_oa_child_call_count, 1);
    assert_eq!(state.minimum_oa_child_no_outdoor_air_count, 1);
    assert_eq!(state.retained_minimum_outdoor_air_write_count, 1);
    assert_eq!(state.ems_override_flag_read_count, 1);
    assert_eq!(state.ems_override_apply_count, 0);
    assert_eq!(state.outdoor_air_flag_read_count, 1);
    assert_eq!(state.outdoor_air_effect_count, 0);
    assert_eq!(state.no_outdoor_air_zero_branch_count, 1);
    assert_eq!(state.psychrometric_call_count, 0);
    assert_eq!(state.latest, Some(snapshot));
}

#[test]
fn unit_off_skips_child_ems_predicate_and_outdoor_air_branch() {
    let mut entry_state = PurchasedAirCalcEntryRuntimeState::new(SYSTEM);
    let entry = advance_entry_state(&mut entry_state, context(0.0, 1.0, 1.0));
    let mut state = PurchasedAirCalcMinimumOaPrefixRuntimeState::new(SYSTEM);

    let snapshot = advance_minimum_oa_prefix_state(&mut entry_state, &mut state, entry);

    assert!(!snapshot.unit_body_entered);
    assert!(!snapshot.zone_heat_balance_reference_bound);
    assert!(!snapshot.minimum_oa_child_called);
    assert!(!snapshot.minimum_oa_child_no_outdoor_air_route);
    assert_eq!(
        snapshot.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        None
    );
    assert!(!snapshot.retained_minimum_outdoor_air_write_performed);
    assert!(!snapshot.ems_override_flag_read);
    assert_eq!(snapshot.ems_override_enabled, None);
    assert!(!snapshot.ems_override_applied);
    assert_eq!(snapshot.working_outdoor_air_mass_flow_rate_kg_per_s, None);
    assert!(!snapshot.outdoor_air_flag_read);
    assert_eq!(snapshot.outdoor_air_enabled, None);
    assert!(!snapshot.no_outdoor_air_zero_branch_entered);
    assert_eq!(snapshot.minimum_outdoor_air_sensible_output_w, None);
    assert_eq!(snapshot.minimum_outdoor_air_moisture_output_kg_per_s, None);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.source_execution_count, 0);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.minimum_oa_child_call_count, 0);
    assert_eq!(state.ems_override_flag_read_count, 0);
    assert_eq!(state.outdoor_air_flag_read_count, 0);
    assert_eq!(state.no_outdoor_air_zero_branch_count, 0);
}

#[test]
fn heat_and_cool_off_do_not_block_the_unit_on_prefix() {
    let mut entry_state = PurchasedAirCalcEntryRuntimeState::new(SYSTEM);
    let entry = advance_entry_state(&mut entry_state, context(1.0, 0.0, 0.0));
    assert!(entry.unit_on);
    assert!(!entry.heating_on);
    assert!(!entry.cooling_on);
    let mut state = PurchasedAirCalcMinimumOaPrefixRuntimeState::new(SYSTEM);

    let snapshot = advance_minimum_oa_prefix_state(&mut entry_state, &mut state, entry);

    assert!(snapshot.unit_body_entered);
    assert!(snapshot.minimum_oa_child_called);
    assert!(snapshot.no_outdoor_air_zero_branch_entered);
    assert_eq!(state.source_execution_count, 1);
}

fn context(
    overall_availability: f64,
    heating_availability: f64,
    cooling_availability: f64,
) -> PurchasedAirCalcEntryContext {
    PurchasedAirCalcEntryContext {
        controlled_zone: ZONE,
        supply_node: NodeId(10),
        zone_node: NodeId(11),
        outdoor_air_node: None,
        recirculation_node: NodeId(12),
        demand: ZoneSysEnergyDemand::from_output_required_setpoint_loads(ZONE, 1.0, -1.0),
        zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
        overall_availability,
        heating_availability,
        cooling_availability,
    }
}
