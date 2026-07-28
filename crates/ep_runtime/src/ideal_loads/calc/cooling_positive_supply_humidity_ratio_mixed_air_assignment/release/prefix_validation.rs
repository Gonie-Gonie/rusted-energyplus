//! CP329/CP334-to-CP335 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};

pub(super) fn humidity_assignment_links_to_temperature_mixed_air_limit(
    assignment: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.supply_humidity_ratio_mixed_air_assignment_executed
            == predecessor.supply_temperature_mixed_air_limit_executed
}

pub(in crate::ideal_loads::calc) fn active_operand_links_to_retained_prefix(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    mixed_air_witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    mixed_air_humidity_ratio: Option<f64>,
) -> bool {
    predecessor.supply_temperature_mixed_air_limit_executed
        && predecessor.supply_temperature_assignment_performed
        && mixed_air.system == predecessor.system
        && mixed_air.parent_call_ordinal == predecessor.parent_call_ordinal
        && mixed_air.controlled_zone == predecessor.controlled_zone
        && mixed_air.cooling_call_executed
        && mixed_air.no_outdoor_air_fallback_entered
        && mixed_air.mixed_air_humidity_ratio_assigned
        && cooling_mixed_air_call_snapshots_match_bit_exact(mixed_air, mixed_air_witness)
        && options_match_bits(mixed_air_humidity_ratio, mixed_air.mixed_air_humidity_ratio)
        && mixed_air_humidity_ratio
            .is_some_and(|humidity_ratio| humidity_ratio.is_finite() && humidity_ratio >= 0.0)
}

pub(super) fn temperature_mixed_air_limit_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let values_match = options_match_bits(
        left.supply_temperature_before_mixed_air_limit_c,
        right.supply_temperature_before_mixed_air_limit_c,
    ) && options_match_bits(
        left.mixed_air_temperature_c,
        right.mixed_air_temperature_c,
    ) && options_match_bits(
        left.minimum_supply_temperature_c,
        right.minimum_supply_temperature_c,
    ) && options_match_bits(
        left.assigned_supply_temperature_c,
        right.assigned_supply_temperature_c,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
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
