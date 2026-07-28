//! CP334/CP335-to-CP336 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn supply_enthalpy_assignment_links_to_humidity_assignment(
    assignment: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
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
        && assignment.supply_enthalpy_assignment_executed
            == predecessor.supply_humidity_ratio_mixed_air_assignment_executed
}

pub(in crate::ideal_loads::calc) fn active_operands_link_to_retained_prefix(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    temperature_assignment: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    temperature_witness: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    supply_temperature_c: Option<f64>,
    supply_humidity_ratio: Option<f64>,
) -> bool {
    predecessor.supply_humidity_ratio_mixed_air_assignment_executed
        && predecessor.supply_humidity_ratio_assignment_performed
        && temperature_assignment.system == predecessor.system
        && temperature_assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && temperature_assignment.controlled_zone == predecessor.controlled_zone
        && temperature_assignment.unit_body_entered == predecessor.unit_body_entered
        && temperature_assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && temperature_assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && temperature_assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && temperature_assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && temperature_assignment.unit_off_skipped == predecessor.unit_off_skipped
        && temperature_assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && temperature_assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && temperature_assignment.supply_temperature_mixed_air_limit_executed
        && temperature_assignment.supply_temperature_assignment_performed
        && temperature_snapshots_match_bit_exact(temperature_assignment, temperature_witness)
        && options_match_bits(
            supply_temperature_c,
            temperature_assignment.assigned_supply_temperature_c,
        )
        && options_match_bits(
            supply_humidity_ratio,
            predecessor.assigned_supply_humidity_ratio,
        )
        && supply_temperature_c.is_some_and(f64::is_finite)
        && supply_humidity_ratio
            .is_some_and(|humidity_ratio| humidity_ratio.is_finite() && humidity_ratio >= 0.0)
}

pub(super) fn humidity_assignment_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let values_match = options_match_bits(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && options_match_bits(
        left.assigned_supply_humidity_ratio,
        right.assigned_supply_humidity_ratio,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn temperature_snapshots_match_bit_exact(
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
