//! CP329/CP333-to-CP334 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn mixed_air_limit_links_to_minimum_limit(
    limit: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> bool {
    limit.system == predecessor.system
        && limit.parent_call_ordinal == predecessor.parent_call_ordinal
        && limit.controlled_zone == predecessor.controlled_zone
        && limit.unit_body_entered == predecessor.unit_body_entered
        && limit.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && limit.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && limit.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && limit.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && limit.unit_off_skipped == predecessor.unit_off_skipped
        && limit.non_cooling_skipped == predecessor.non_cooling_skipped
        && limit.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && limit.supply_temperature_mixed_air_limit_executed
            == predecessor.supply_temperature_minimum_limit_executed
}

pub(in crate::ideal_loads::calc) fn active_operands_link_to_retained_prefix(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    supply_temperature_before_mixed_air_limit_c: Option<f64>,
    mixed_air_temperature_c: Option<f64>,
) -> bool {
    predecessor.supply_temperature_minimum_limit_executed
        && predecessor.supply_temperature_assignment_performed
        && mixed_air.system == predecessor.system
        && mixed_air.parent_call_ordinal == predecessor.parent_call_ordinal
        && mixed_air.controlled_zone == predecessor.controlled_zone
        && mixed_air.cooling_call_executed
        && mixed_air.no_outdoor_air_fallback_entered
        && mixed_air.mixed_air_temperature_assigned
        && options_match_bits(
            supply_temperature_before_mixed_air_limit_c,
            predecessor.assigned_supply_temperature_c,
        )
        && options_match_bits(mixed_air_temperature_c, mixed_air.mixed_air_temperature_c)
        && mixed_air_temperature_c.is_some_and(f64::is_finite)
}

pub(super) fn minimum_limit_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> bool {
    let values_match = options_match_bits(
        left.supply_temperature_before_minimum_limit_c,
        right.supply_temperature_before_minimum_limit_c,
    ) && options_match_bits(
        left.minimum_cooling_supply_air_temperature_c,
        right.minimum_cooling_supply_air_temperature_c,
    ) && options_match_bits(
        left.maximum_supply_temperature_c,
        right.maximum_supply_temperature_c,
    ) && options_match_bits(
        left.assigned_supply_temperature_c,
        right.assigned_supply_temperature_c,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_before_minimum_limit_c = None;
        snapshot.minimum_cooling_supply_air_temperature_c = None;
        snapshot.maximum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
    }
    values_match && left == right
}

fn options_match_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
