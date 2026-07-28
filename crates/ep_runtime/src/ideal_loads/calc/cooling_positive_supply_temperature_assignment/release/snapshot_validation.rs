//! Exact CP332 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_temperature_assignment_executed;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_temperature_assignment_executed;
    let guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_temperature_assignment_executed;
    let assigned = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.supply_temperature_assignment_executed;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(guard_false)
            + usize::from(assigned)
            == 1
        && if assigned {
            assigned_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

fn assigned_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let Some(zone_cooling_setpoint_load) = snapshot.zone_cooling_setpoint_load_w else {
        return false;
    };
    let Some(cp_air) = snapshot.cp_air_j_per_kg_k else {
        return false;
    };
    let Some(supply_mass_flow_rate) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(retained_denominator) = snapshot.cp_air_times_supply_mass_flow_rate_w_per_k else {
        return false;
    };
    let Some(retained_quotient) = snapshot.zone_cooling_setpoint_load_over_denominator_c else {
        return false;
    };
    let Some(zone_node_temperature) = snapshot.zone_node_temperature_c else {
        return false;
    };
    let Some(calculated) = snapshot.calculated_supply_temperature_c else {
        return false;
    };
    let Some(assigned) = snapshot.supply_temperature_c else {
        return false;
    };

    let denominator = cp_air * supply_mass_flow_rate;
    let quotient = zone_cooling_setpoint_load / denominator;
    let expected_supply_temperature = quotient + zone_node_temperature;

    snapshot.zone_cooling_setpoint_load_read
        && zone_cooling_setpoint_load.is_finite()
        && snapshot.cp_air_read
        && cp_air.is_finite()
        && cp_air > 0.0
        && snapshot.supply_mass_flow_rate_read
        && supply_mass_flow_rate > 0.0
        && snapshot.cp_air_times_supply_mass_flow_rate_calculated
        && retained_denominator.to_bits() == denominator.to_bits()
        && snapshot.zone_cooling_setpoint_load_over_denominator_calculated
        && retained_quotient.to_bits() == quotient.to_bits()
        && snapshot.zone_node_temperature_read
        && zone_node_temperature.is_finite()
        && snapshot.supply_temperature_calculated
        && calculated.to_bits() == expected_supply_temperature.to_bits()
        && snapshot.supply_temperature_assigned
        && assigned.to_bits() == calculated.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    !snapshot.zone_cooling_setpoint_load_read
        && snapshot.zone_cooling_setpoint_load_w.is_none()
        && !snapshot.cp_air_read
        && snapshot.cp_air_j_per_kg_k.is_none()
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.cp_air_times_supply_mass_flow_rate_calculated
        && snapshot
            .cp_air_times_supply_mass_flow_rate_w_per_k
            .is_none()
        && !snapshot.zone_cooling_setpoint_load_over_denominator_calculated
        && snapshot
            .zone_cooling_setpoint_load_over_denominator_c
            .is_none()
        && !snapshot.zone_node_temperature_read
        && snapshot.zone_node_temperature_c.is_none()
        && !snapshot.supply_temperature_calculated
        && snapshot.calculated_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assigned
        && snapshot.supply_temperature_c.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.zone_cooling_setpoint_load_w,
        right.zone_cooling_setpoint_load_w,
    ) && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k)
        && option_bits_match(
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        )
        && option_bits_match(
            left.cp_air_times_supply_mass_flow_rate_w_per_k,
            right.cp_air_times_supply_mass_flow_rate_w_per_k,
        )
        && option_bits_match(
            left.zone_cooling_setpoint_load_over_denominator_c,
            right.zone_cooling_setpoint_load_over_denominator_c,
        )
        && option_bits_match(left.zone_node_temperature_c, right.zone_node_temperature_c)
        && option_bits_match(
            left.calculated_supply_temperature_c,
            right.calculated_supply_temperature_c,
        )
        && option_bits_match(left.supply_temperature_c, right.supply_temperature_c);
    for snapshot in [&mut left, &mut right] {
        snapshot.zone_cooling_setpoint_load_w = None;
        snapshot.cp_air_j_per_kg_k = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.cp_air_times_supply_mass_flow_rate_w_per_k = None;
        snapshot.zone_cooling_setpoint_load_over_denominator_c = None;
        snapshot.zone_node_temperature_c = None;
        snapshot.calculated_supply_temperature_c = None;
        snapshot.supply_temperature_c = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
