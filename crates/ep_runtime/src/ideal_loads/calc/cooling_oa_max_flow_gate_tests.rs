use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, NodeId, ZoneId};

use crate::zone_equipment::ZoneSysEnergyDemand;

use super::{
    cooling_entry_gate::{
        PurchasedAirCalcCoolingEntryGateRuntimeState, PurchasedAirTemperatureControlType,
        advance_cooling_entry_gate_state,
    },
    cooling_oa_max_flow_gate::{
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
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
fn unit_off_and_active_non_cooling_skip_every_site_in_distinct_partitions() {
    let unit_off = cooling_entry_predecessor(0.0, -1.0);
    let active_non_cooling = cooling_entry_predecessor(1.0, 1.0);
    assert!(!unit_off.unit_body_entered);
    assert!(!unit_off.cooling_body_entered);
    assert!(active_non_cooling.unit_body_entered);
    assert!(!active_non_cooling.cooling_body_entered);
    let mut state = PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(SYSTEM);

    let unit_off_snapshot = advance_cooling_oa_max_flow_gate_state(
        &mut state,
        unit_off,
        IdealLoadsLimit::LimitFlowRate,
        f64::NAN,
        f64::NAN,
    );
    assert_skipped(unit_off_snapshot);
    assert!(unit_off_snapshot.unit_off_skipped);
    assert!(!unit_off_snapshot.non_cooling_skipped);

    let non_cooling_snapshot = advance_cooling_oa_max_flow_gate_state(
        &mut state,
        active_non_cooling,
        IdealLoadsLimit::LimitFlowRateAndCapacity,
        f64::NAN,
        f64::NAN,
    );
    assert_skipped(non_cooling_snapshot);
    assert!(!non_cooling_snapshot.unit_off_skipped);
    assert!(non_cooling_snapshot.non_cooling_skipped);

    assert_eq!(state.transition_count, 2);
    assert_eq!(state.source_execution_count, 0);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
}

#[test]
fn selector_or_and_mass_flow_and_short_circuits_follow_source_order() {
    let predecessor = cooling_entry_predecessor(1.0, -1.0);

    let mut flow_state = PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(SYSTEM);
    let flow = advance_cooling_oa_max_flow_gate_state(
        &mut flow_state,
        predecessor,
        IdealLoadsLimit::LimitFlowRate,
        1.0,
        0.0,
    );
    assert_eq!(
        flow.cooling_limit_flow_rate_value,
        Some(IdealLoadsLimit::LimitFlowRate)
    );
    assert_eq!(
        flow.cooling_limit_flow_rate_comparison_satisfied,
        Some(true)
    );
    assert!(
        !flow.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        "the first true operand must short-circuit `||`"
    );
    assert!(!flow.cooling_limit_flow_rate_and_capacity_read);
    assert_eq!(flow.cooling_limit_flow_rate_and_capacity_value, None);
    assert_eq!(
        flow.cooling_limit_flow_rate_and_capacity_comparison_satisfied,
        None
    );
    assert_eq!(flow.cooling_flow_limit_active, Some(true));
    assert_mass_operands(flow, 1.0, 0.0, true);
    assert_eq!(flow_state.cooling_limit_flow_rate_match_count, 1);

    let mut combined_state = PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(SYSTEM);
    let combined = advance_cooling_oa_max_flow_gate_state(
        &mut combined_state,
        predecessor,
        IdealLoadsLimit::LimitFlowRateAndCapacity,
        1.0,
        0.0,
    );
    assert_eq!(
        combined.cooling_limit_flow_rate_comparison_satisfied,
        Some(false)
    );
    assert!(combined.cooling_limit_flow_rate_and_capacity_comparison_evaluated);
    assert!(combined.cooling_limit_flow_rate_and_capacity_read);
    assert_eq!(
        combined.cooling_limit_flow_rate_and_capacity_value,
        Some(IdealLoadsLimit::LimitFlowRateAndCapacity)
    );
    assert_eq!(
        combined.cooling_limit_flow_rate_and_capacity_comparison_satisfied,
        Some(true)
    );
    assert_eq!(combined.cooling_flow_limit_active, Some(true));
    assert_mass_operands(combined, 1.0, 0.0, true);
    assert_eq!(combined_state.cooling_limit_flow_rate_match_count, 0);
    assert_eq!(
        combined_state.cooling_limit_flow_rate_and_capacity_comparison_count,
        1
    );
    assert_eq!(
        combined_state.cooling_limit_flow_rate_and_capacity_match_count,
        1
    );

    for limit in [IdealLoadsLimit::NoLimit, IdealLoadsLimit::LimitCapacity] {
        let mut state = PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(SYSTEM);
        let snapshot = advance_cooling_oa_max_flow_gate_state(
            &mut state,
            predecessor,
            limit,
            f64::NAN,
            f64::NAN,
        );
        assert_eq!(
            snapshot.cooling_limit_flow_rate_comparison_satisfied,
            Some(false)
        );
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity_comparison_satisfied,
            Some(false)
        );
        assert_eq!(snapshot.cooling_flow_limit_active, Some(false));
        assert!(!snapshot.outdoor_air_mass_flow_rate_read);
        assert_eq!(snapshot.outdoor_air_mass_flow_rate_kg_per_s, None);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_read);
        assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
        assert!(!snapshot.strict_mass_flow_comparison_evaluated);
        assert_eq!(snapshot.outdoor_air_mass_flow_above_maximum, None);
        assert!(!snapshot.maximum_cooling_flow_body_entered);
        assert_eq!(state.strict_mass_flow_comparison_count, 0);
        assert_eq!(state.active_fallthrough_count, 1);
    }
}

#[test]
fn strict_greater_than_characterizes_nan_signed_zero_and_infinity() {
    let predecessor = cooling_entry_predecessor(1.0, -1.0);
    for (outdoor_air, maximum, expected) in [
        (f64::NAN, 0.0, false),
        (0.0, f64::NAN, false),
        (0.0, -0.0, false),
        (-0.0, 0.0, false),
        (-0.0, -0.0, false),
        (1.0, 1.0, false),
        (f64::INFINITY, 0.0, true),
        (0.0, f64::INFINITY, false),
        (0.0, f64::NEG_INFINITY, true),
        (f64::NEG_INFINITY, 0.0, false),
    ] {
        let mut state = PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(SYSTEM);
        let snapshot = advance_cooling_oa_max_flow_gate_state(
            &mut state,
            predecessor,
            IdealLoadsLimit::LimitFlowRate,
            outdoor_air,
            maximum,
        );
        assert!(snapshot.strict_mass_flow_comparison_evaluated);
        assert_eq!(
            snapshot
                .outdoor_air_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(outdoor_air.to_bits())
        );
        assert_eq!(
            snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(maximum.to_bits())
        );
        assert_eq!(
            snapshot.outdoor_air_mass_flow_above_maximum,
            Some(expected),
            "{outdoor_air:?} > {maximum:?}"
        );
        assert_eq!(snapshot.maximum_cooling_flow_body_entered, expected);
        assert_eq!(
            state.strict_mass_flow_comparison_satisfied_count,
            usize::from(expected)
        );
        assert_eq!(
            state.maximum_cooling_flow_body_entry_count,
            usize::from(expected)
        );
        assert_eq!(state.active_fallthrough_count, usize::from(!expected));
    }
}

fn assert_skipped(snapshot: super::PurchasedAirCalcCoolingOaMaxFlowGateSnapshot) {
    assert_eq!(
        snapshot.source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
    );
    assert_eq!(
        snapshot.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER
    );
    assert!(!snapshot.cooling_limit_flow_rate_comparison_evaluated);
    assert!(!snapshot.cooling_limit_flow_rate_read);
    assert_eq!(snapshot.cooling_limit_flow_rate_value, None);
    assert_eq!(snapshot.cooling_limit_flow_rate_comparison_satisfied, None);
    assert!(!snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated);
    assert!(!snapshot.cooling_limit_flow_rate_and_capacity_read);
    assert_eq!(snapshot.cooling_limit_flow_rate_and_capacity_value, None);
    assert_eq!(
        snapshot.cooling_limit_flow_rate_and_capacity_comparison_satisfied,
        None
    );
    assert_eq!(snapshot.cooling_flow_limit_active, None);
    assert!(!snapshot.outdoor_air_mass_flow_rate_read);
    assert_eq!(snapshot.outdoor_air_mass_flow_rate_kg_per_s, None);
    assert!(!snapshot.maximum_cooling_air_mass_flow_rate_read);
    assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
    assert!(!snapshot.strict_mass_flow_comparison_evaluated);
    assert_eq!(snapshot.outdoor_air_mass_flow_above_maximum, None);
    assert!(!snapshot.maximum_cooling_flow_body_entered);
}

fn assert_mass_operands(
    snapshot: super::PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    outdoor_air: f64,
    maximum: f64,
    expected_comparison: bool,
) {
    assert!(snapshot.outdoor_air_mass_flow_rate_read);
    assert_eq!(
        snapshot
            .outdoor_air_mass_flow_rate_kg_per_s
            .map(f64::to_bits),
        Some(outdoor_air.to_bits())
    );
    assert!(snapshot.maximum_cooling_air_mass_flow_rate_read);
    assert_eq!(
        snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .map(f64::to_bits),
        Some(maximum.to_bits())
    );
    assert!(snapshot.strict_mass_flow_comparison_evaluated);
    assert_eq!(
        snapshot.outdoor_air_mass_flow_above_maximum,
        Some(expected_comparison)
    );
    assert_eq!(
        snapshot.maximum_cooling_flow_body_entered,
        expected_comparison
    );
}

fn cooling_entry_predecessor(
    overall_availability: f64,
    cooling_demand_w: f64,
) -> super::PurchasedAirCalcCoolingEntryGateSnapshot {
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
    advance_cooling_entry_gate_state(
        &mut cooling_entry_state,
        entry,
        minimum_oa,
        PurchasedAirTemperatureControlType::DualHeatCool,
    )
}
