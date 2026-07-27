use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneId};

use crate::zone_equipment::ZoneSysEnergyDemand;

use super::{
    cooling_entry_gate::{
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER,
        PurchasedAirCalcCoolingEntryGateRuntimeState, PurchasedAirTemperatureControlType,
        advance_cooling_entry_gate_state,
    },
    lifecycle::{
        PurchasedAirAvailabilityStatus, PurchasedAirCalcEntryContext,
        PurchasedAirCalcEntryRuntimeState, PurchasedAirCalcEntrySnapshot, advance_entry_state,
    },
    minimum_oa_prefix::{
        PurchasedAirCalcMinimumOaPrefixRuntimeState, PurchasedAirCalcMinimumOaPrefixSnapshot,
        advance_minimum_oa_prefix_state,
    },
    types::IdealLoadsSensibleMode,
};

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);
const ZONE: ZoneId = ZoneId(3);

#[test]
fn unit_off_skips_every_cooling_entry_site() {
    let (entry, mut minimum_oa) = predecessors(0.0, f64::NAN, 1.0, 1.0);
    minimum_oa.minimum_outdoor_air_sensible_output_w = Some(f64::NAN);
    let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);

    let snapshot = advance_cooling_entry_gate_state(
        &mut state,
        entry,
        minimum_oa,
        PurchasedAirTemperatureControlType::DualHeatCool,
    );

    assert_eq!(
        snapshot.source,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
    );
    assert_eq!(
        snapshot.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER
    );
    assert!(!snapshot.unit_body_entered);
    assert_eq!(snapshot.minimum_outdoor_air_sensible_output_w, None);
    assert_eq!(snapshot.cooling_setpoint_demand_w, None);
    assert!(!snapshot.sensible_comparison_evaluated);
    assert_eq!(snapshot.sensible_comparison_satisfied, None);
    assert!(!snapshot.temperature_control_type_read);
    assert_eq!(snapshot.temperature_control_type, None);
    assert_eq!(snapshot.temperature_control_type_permits_cooling, None);
    assert!(!snapshot.single_heat_blocked);
    assert!(!snapshot.cooling_body_entered);
    assert_eq!(snapshot.assigned_operating_mode, None);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.source_execution_count, 0);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.sensible_comparison_count, 0);
    assert_eq!(state.temperature_control_type_read_count, 0);
    assert_eq!(state.active_fallthrough_count, 0);
    assert_eq!(state.latest, Some(snapshot));
}

#[test]
fn negative_and_both_zero_cooling_thresholds_enter_inclusively() {
    for (minimum_oa_sensible_output_w, cooling_demand_w) in [
        (0.0, -500.0),
        (0.0, 0.0),
        (0.0, -0.0),
        (-0.0, 0.0),
        (-0.0, -0.0),
    ] {
        let (entry, mut minimum_oa) = predecessors(1.0, cooling_demand_w, 1.0, 1.0);
        minimum_oa.minimum_outdoor_air_sensible_output_w = Some(minimum_oa_sensible_output_w);
        let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);

        let snapshot = advance_cooling_entry_gate_state(
            &mut state,
            entry,
            minimum_oa,
            PurchasedAirTemperatureControlType::DualHeatCool,
        );

        assert!(snapshot.sensible_comparison_evaluated);
        assert_eq!(snapshot.sensible_comparison_satisfied, Some(true));
        assert!(snapshot.temperature_control_type_read);
        assert_eq!(
            snapshot.temperature_control_type,
            Some(PurchasedAirTemperatureControlType::DualHeatCool)
        );
        assert_eq!(
            snapshot.temperature_control_type_permits_cooling,
            Some(true)
        );
        assert!(snapshot.cooling_body_entered);
        assert_eq!(
            snapshot.assigned_operating_mode,
            Some(IdealLoadsSensibleMode::Cooling)
        );
        assert_eq!(state.sensible_comparison_satisfied_count, 1);
        assert_eq!(state.temperature_control_type_read_count, 1);
        assert_eq!(state.cooling_body_entry_count, 1);
        assert_eq!(state.operating_mode_assignment_count, 1);
        assert_eq!(state.active_fallthrough_count, 0);
    }
}

#[test]
fn positive_and_nan_thresholds_short_circuit_before_thermostat_read() {
    for cooling_demand_w in [500.0, f64::NAN] {
        let (entry, minimum_oa) = predecessors(1.0, cooling_demand_w, 1.0, 1.0);
        let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);

        let snapshot = advance_cooling_entry_gate_state(
            &mut state,
            entry,
            minimum_oa,
            PurchasedAirTemperatureControlType::SingleHeat,
        );

        assert_eq!(snapshot.sensible_comparison_satisfied, Some(false));
        assert!(!snapshot.temperature_control_type_read);
        assert_eq!(snapshot.temperature_control_type, None);
        assert!(!snapshot.single_heat_blocked);
        assert!(!snapshot.cooling_body_entered);
        assert_eq!(snapshot.assigned_operating_mode, None);
        assert_eq!(state.temperature_control_type_read_count, 0);
        assert_eq!(state.single_heat_block_count, 0);
        assert_eq!(state.active_fallthrough_count, 1);
    }
}

#[test]
fn nan_minimum_oa_sensible_output_short_circuits_before_thermostat_read() {
    let (entry, mut minimum_oa) = predecessors(1.0, -500.0, 1.0, 1.0);
    minimum_oa.minimum_outdoor_air_sensible_output_w = Some(f64::NAN);
    let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);

    let snapshot = advance_cooling_entry_gate_state(
        &mut state,
        entry,
        minimum_oa,
        PurchasedAirTemperatureControlType::SingleHeat,
    );

    assert_eq!(snapshot.sensible_comparison_satisfied, Some(false));
    assert!(!snapshot.temperature_control_type_read);
    assert_eq!(snapshot.temperature_control_type, None);
    assert!(!snapshot.single_heat_blocked);
    assert!(!snapshot.cooling_body_entered);
    assert_eq!(snapshot.assigned_operating_mode, None);
    assert_eq!(state.temperature_control_type_read_count, 0);
    assert_eq!(state.single_heat_block_count, 0);
    assert_eq!(state.active_fallthrough_count, 1);
}

#[test]
fn exact_single_heat_alone_blocks_a_satisfied_numeric_gate() {
    for control_type in [
        PurchasedAirTemperatureControlType::Invalid,
        PurchasedAirTemperatureControlType::Uncontrolled,
        PurchasedAirTemperatureControlType::SingleCool,
        PurchasedAirTemperatureControlType::SingleHeatCool,
        PurchasedAirTemperatureControlType::DualHeatCool,
    ] {
        let (entry, minimum_oa) = predecessors(1.0, -1.0, 1.0, 1.0);
        let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);
        let snapshot =
            advance_cooling_entry_gate_state(&mut state, entry, minimum_oa, control_type);
        assert!(snapshot.cooling_body_entered, "{control_type:?}");
        assert!(!snapshot.single_heat_blocked);
    }

    let (entry, minimum_oa) = predecessors(1.0, -1.0, 1.0, 1.0);
    let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);
    let snapshot = advance_cooling_entry_gate_state(
        &mut state,
        entry,
        minimum_oa,
        PurchasedAirTemperatureControlType::SingleHeat,
    );
    assert_eq!(snapshot.sensible_comparison_satisfied, Some(true));
    assert!(snapshot.temperature_control_type_read);
    assert_eq!(
        snapshot.temperature_control_type_permits_cooling,
        Some(false)
    );
    assert!(snapshot.single_heat_blocked);
    assert!(!snapshot.cooling_body_entered);
    assert_eq!(snapshot.assigned_operating_mode, None);
    assert_eq!(state.single_heat_block_count, 1);
    assert_eq!(state.active_fallthrough_count, 1);
}

#[test]
fn heating_and_cooling_availability_do_not_gate_the_line_2046_predicate() {
    let (entry, minimum_oa) = predecessors(1.0, -1.0, 0.0, 0.0);
    assert!(entry.unit_body_entered);
    assert!(!entry.heating_on);
    assert!(!entry.cooling_on);
    let mut state = PurchasedAirCalcCoolingEntryGateRuntimeState::new(SYSTEM);

    let snapshot = advance_cooling_entry_gate_state(
        &mut state,
        entry,
        minimum_oa,
        PurchasedAirTemperatureControlType::DualHeatCool,
    );

    assert!(snapshot.cooling_body_entered);
    assert_eq!(state.source_execution_count, 1);
}

fn predecessors(
    overall_availability: f64,
    cooling_demand_w: f64,
    heating_availability: f64,
    cooling_availability: f64,
) -> (
    PurchasedAirCalcEntrySnapshot,
    PurchasedAirCalcMinimumOaPrefixSnapshot,
) {
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
            heating_availability,
            cooling_availability,
        },
    );
    let mut minimum_oa_state = PurchasedAirCalcMinimumOaPrefixRuntimeState::new(SYSTEM);
    let minimum_oa =
        advance_minimum_oa_prefix_state(&mut entry_state, &mut minimum_oa_state, entry);
    (entry, minimum_oa)
}
