use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    mixed_air_owner: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot {
    let active = predecessor.dehumidification_control_switch_dispatched
        && predecessor.dehumidification_control_type == Some(DehumidificationControlType::None);
    let mixed_air_humidity_ratio = active
        .then_some(mixed_air_owner.mixed_air_humidity_ratio)
        .flatten();

    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_capacity_limit_guard_false_fallthrough:
            predecessor.predecessor_capacity_limit_guard_false_fallthrough,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough:
            predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
        predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
            predecessor
                .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
        predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed:
            predecessor
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed,
        predecessor_assigned_supply_humidity_ratio:
            predecessor.predecessor_assigned_supply_humidity_ratio,
        predecessor_dehumidification_control_type_read:
            predecessor.dehumidification_control_type_read,
        predecessor_dehumidification_control_type: predecessor.dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched:
            predecessor.dehumidification_control_switch_dispatched,
        dehumidification_control_none_case_entered: active,
        mixed_air_humidity_ratio_read: active,
        mixed_air_humidity_ratio,
        supply_humidity_ratio_assignment_performed: active,
        assigned_supply_humidity_ratio: mixed_air_humidity_ratio,
        resulting_supply_humidity_ratio: mixed_air_humidity_ratio,
        dehumidification_control_none_case_exited_via_break: active,
    }
}
