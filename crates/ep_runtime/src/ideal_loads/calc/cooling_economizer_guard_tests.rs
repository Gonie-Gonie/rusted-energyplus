use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, NodeId, OutdoorAirEconomizerType, ZoneId};

use crate::zone_equipment::ZoneSysEnergyDemand;

use super::{
    cooling_economizer_guard::{
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
        PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
        PurchasedAirCalcCoolingEconomizerGuardSnapshot, advance_cooling_economizer_guard_state,
    },
    cooling_entry_gate::{
        PurchasedAirCalcCoolingEntryGateRuntimeState, PurchasedAirTemperatureControlType,
        advance_cooling_entry_gate_state,
    },
    cooling_oa_max_flow_body::{
        PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
        PurchasedAirCalcCoolingOaMaxFlowBodySnapshot, advance_cooling_oa_max_flow_body_state,
    },
    cooling_oa_max_flow_gate::{
        PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState, advance_cooling_oa_max_flow_gate_state,
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
fn line_2082_characterizes_all_typed_economizer_values_without_body_effects() {
    let predecessor = cp314_predecessor(1.0, -1.0, IdealLoadsLimit::NoLimit, f64::NAN, f64::NAN);
    assert!(predecessor.active_guard_false_economizer_fallthrough);

    for (economizer_type, expected_comparison) in [
        (OutdoorAirEconomizerType::NoEconomizer, false),
        (OutdoorAirEconomizerType::DifferentialDryBulb, true),
        (OutdoorAirEconomizerType::DifferentialEnthalpy, true),
    ] {
        let mut state = PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(SYSTEM);
        let snapshot =
            advance_cooling_economizer_guard_state(&mut state, predecessor, economizer_type);

        assert_eq!(
            snapshot.source,
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        );
        assert_eq!(
            snapshot.first_excluded_source,
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        );
        assert_eq!(
            snapshot.source_order,
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER
        );
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert!(snapshot.economizer_guard_evaluated);
        assert!(snapshot.economizer_type_read);
        assert_eq!(snapshot.economizer_type, Some(economizer_type));
        assert!(snapshot.no_economizer_comparison_evaluated);
        assert_eq!(
            snapshot.economizer_not_no_economizer,
            Some(expected_comparison)
        );
        assert_eq!(snapshot.economizer_body_entered, expected_comparison);
        assert_eq!(snapshot.no_economizer_fallthrough, !expected_comparison);
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.guard_evaluation_count, 1);
        assert_eq!(state.economizer_type_read_count, 1);
        assert_eq!(state.no_economizer_comparison_count, 1);
        assert_eq!(
            state.economizer_body_entry_count,
            usize::from(expected_comparison)
        );
        assert_eq!(
            state.no_economizer_fallthrough_count,
            usize::from(!expected_comparison)
        );
    }
}

#[test]
fn unit_off_non_cooling_and_true_sibling_body_are_distinct_complete_skips() {
    let predecessors = [
        cp314_predecessor(0.0, -1.0, IdealLoadsLimit::LimitFlowRate, 2.0, 1.0),
        cp314_predecessor(1.0, 1.0, IdealLoadsLimit::LimitFlowRate, 2.0, 1.0),
        cp314_predecessor(1.0, -1.0, IdealLoadsLimit::LimitFlowRate, 2.0, 1.0),
    ];
    assert!(predecessors[0].unit_off_skipped);
    assert!(predecessors[1].non_cooling_skipped);
    assert!(predecessors[2].predecessor_maximum_cooling_flow_body_entered);
    let economizer_types = [
        OutdoorAirEconomizerType::NoEconomizer,
        OutdoorAirEconomizerType::DifferentialDryBulb,
        OutdoorAirEconomizerType::DifferentialEnthalpy,
    ];
    let mut state = PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(SYSTEM);

    let snapshots: [PurchasedAirCalcCoolingEconomizerGuardSnapshot; 3] =
        std::array::from_fn(|index| {
            advance_cooling_economizer_guard_state(
                &mut state,
                predecessors[index],
                economizer_types[index],
            )
        });

    assert!(snapshots[0].unit_off_skipped);
    assert!(snapshots[1].non_cooling_skipped);
    assert!(snapshots[2].maximum_cooling_flow_body_sibling_skipped);
    for snapshot in snapshots {
        assert_guard_skipped(snapshot);
    }
    assert_eq!(state.transition_count, 3);
    assert_eq!(state.guard_evaluation_count, 0);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.maximum_cooling_flow_body_sibling_skip_count, 1);
    assert_eq!(state.economizer_type_read_count, 0);
    assert_eq!(state.no_economizer_comparison_count, 0);
    assert_eq!(state.economizer_body_entry_count, 0);
    assert_eq!(state.no_economizer_fallthrough_count, 0);
}

fn assert_guard_skipped(snapshot: PurchasedAirCalcCoolingEconomizerGuardSnapshot) {
    assert_eq!(
        usize::from(snapshot.unit_off_skipped)
            + usize::from(snapshot.non_cooling_skipped)
            + usize::from(snapshot.maximum_cooling_flow_body_sibling_skipped),
        1
    );
    assert!(!snapshot.economizer_guard_evaluated);
    assert!(!snapshot.economizer_type_read);
    assert_eq!(snapshot.economizer_type, None);
    assert!(!snapshot.no_economizer_comparison_evaluated);
    assert_eq!(snapshot.economizer_not_no_economizer, None);
    assert!(!snapshot.economizer_body_entered);
    assert!(!snapshot.no_economizer_fallthrough);
}

fn cp314_predecessor(
    overall_availability: f64,
    cooling_demand_w: f64,
    limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingOaMaxFlowBodySnapshot {
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
    let gate = advance_cooling_oa_max_flow_gate_state(
        &mut gate_state,
        cooling_entry,
        limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
    );
    let mut body_state = PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState::new(SYSTEM);
    advance_cooling_oa_max_flow_body_state(
        &mut body_state,
        gate,
        outdoor_air_mass_flow_rate_kg_per_s,
        1.0,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
    )
}
