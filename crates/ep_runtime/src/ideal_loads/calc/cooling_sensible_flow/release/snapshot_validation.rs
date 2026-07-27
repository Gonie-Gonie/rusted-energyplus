//! Exact direct-release validation for CP318 snapshots.

use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingSensibleFlowRetainedRoute, PurchasedAirCalcCoolingSensibleFlowSnapshot,
};

pub(in crate::ideal_loads) fn cooling_sensible_flow_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered;
    let cooling = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered;
    let direct_predecessor_shape = !snapshot.predecessor_maximum_cooling_flow_body_sibling_skipped
        && !snapshot.predecessor_economizer_condition_fallthrough_skipped
        && !snapshot.predecessor_economizer_calculation_body_executed
        && snapshot.predecessor_no_economizer_outer_guard_fallthrough_skipped == cooling;

    provenance
        && direct_predecessor_shape
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(cooling) == 1
        && if cooling {
            active_sites_are_exact(snapshot)
        } else {
            skipped_sites_are_exact(snapshot)
        }
}

pub(super) fn cooling_sensible_flow_snapshot_route(
    snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot,
) -> Option<PurchasedAirCalcCoolingSensibleFlowRetainedRoute> {
    if !cooling_sensible_flow_snapshot_is_exact_direct_release(snapshot) {
        return None;
    }
    if snapshot.unit_off_skipped {
        Some(PurchasedAirCalcCoolingSensibleFlowRetainedRoute::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(PurchasedAirCalcCoolingSensibleFlowRetainedRoute::NonCooling)
    } else if snapshot.delta_temperature_body_entered {
        Some(PurchasedAirCalcCoolingSensibleFlowRetainedRoute::CandidateAssigned)
    } else {
        Some(PurchasedAirCalcCoolingSensibleFlowRetainedRoute::DeltaTemperatureFallthrough)
    }
}

fn active_sites_are_exact(snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot) -> bool {
    let Some(humidity_ratio) = snapshot.zone_humidity_ratio else {
        return false;
    };
    let Some(minimum_supply) = snapshot.minimum_cooling_supply_air_temperature_c else {
        return false;
    };
    let Some(zone_temperature) = snapshot.zone_temperature_c else {
        return false;
    };
    if !humidity_ratio.is_finite() || !minimum_supply.is_finite() || !zone_temperature.is_finite() {
        return false;
    }
    let cp_air = energyplus_psy_cp_air_fn_w(humidity_ratio);
    let delta_temperature = minimum_supply - zone_temperature;
    let delta_satisfied =
        delta_temperature < -PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C;
    let common = snapshot.supply_mass_flow_rate_for_cool_reset_assigned
        && option_f64_has_bits(snapshot.reset_supply_mass_flow_rate_for_cool_kg_per_s, 0.0)
        && snapshot.cooling_on_read
        && snapshot.cooling_on == Some(true)
        && snapshot.cooling_on_body_entered
        && snapshot.zone_humidity_ratio_read
        && snapshot.psychrometric_cp_air_evaluated
        && option_f64_has_bits(snapshot.psychrometric_cp_air_result_j_per_kg_k, cp_air)
        && snapshot.cp_air_assigned
        && option_f64_has_bits(snapshot.cp_air_j_per_kg_k, cp_air)
        && snapshot.minimum_cooling_supply_air_temperature_read
        && snapshot.zone_temperature_read
        && snapshot.delta_temperature_calculated
        && option_f64_has_bits(snapshot.delta_temperature_c, delta_temperature)
        && snapshot.delta_temperature_assigned
        && option_f64_has_bits(snapshot.assigned_delta_temperature_c, delta_temperature)
        && snapshot.delta_temperature_for_gate_read
        && option_f64_has_bits(snapshot.delta_temperature_for_gate_c, delta_temperature)
        && snapshot.delta_temperature_comparison_evaluated
        && snapshot.delta_temperature_below_negative_small_temp_diff == Some(delta_satisfied)
        && snapshot.delta_temperature_body_entered == delta_satisfied;
    common
        && if delta_satisfied {
            assigned_sites_are_exact(snapshot, cp_air, delta_temperature)
        } else {
            downstream_sites_are_skipped(snapshot)
                && option_f64_has_bits(
                    snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
                    0.0,
                )
        }
}

fn assigned_sites_are_exact(
    snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    cp_air: f64,
    delta_temperature: f64,
) -> bool {
    let Some(load) = snapshot.zone_cooling_setpoint_load_w else {
        return false;
    };
    if !load.is_finite() {
        return false;
    }
    let first_division = load / cp_air;
    let supply_flow = first_division / delta_temperature;
    snapshot.zone_cooling_setpoint_load_read
        && snapshot.cp_air_for_first_division_read
        && option_f64_has_bits(snapshot.cp_air_for_first_division_j_per_kg_k, cp_air)
        && snapshot.zone_cooling_setpoint_load_over_cp_air_calculated
        && option_f64_has_bits(
            snapshot.zone_cooling_setpoint_load_over_cp_air_kg_k_per_s,
            first_division,
        )
        && snapshot.delta_temperature_for_second_division_read
        && option_f64_has_bits(
            snapshot.delta_temperature_for_second_division_c,
            delta_temperature,
        )
        && snapshot.supply_mass_flow_rate_for_cool_calculated
        && option_f64_has_bits(
            snapshot.calculated_supply_mass_flow_rate_for_cool_kg_per_s,
            supply_flow,
        )
        && snapshot.supply_mass_flow_rate_for_cool_assigned
        && option_f64_has_bits(
            snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
            supply_flow,
        )
        && option_f64_has_bits(
            snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            supply_flow,
        )
}

fn skipped_sites_are_exact(snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot) -> bool {
    !snapshot.supply_mass_flow_rate_for_cool_reset_assigned
        && snapshot
            .reset_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
        && !snapshot.cooling_on_read
        && snapshot.cooling_on.is_none()
        && !snapshot.cooling_on_body_entered
        && !snapshot.zone_humidity_ratio_read
        && snapshot.zone_humidity_ratio.is_none()
        && !snapshot.psychrometric_cp_air_evaluated
        && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
        && !snapshot.cp_air_assigned
        && snapshot.cp_air_j_per_kg_k.is_none()
        && !snapshot.minimum_cooling_supply_air_temperature_read
        && snapshot.minimum_cooling_supply_air_temperature_c.is_none()
        && !snapshot.zone_temperature_read
        && snapshot.zone_temperature_c.is_none()
        && !snapshot.delta_temperature_calculated
        && snapshot.delta_temperature_c.is_none()
        && !snapshot.delta_temperature_assigned
        && snapshot.assigned_delta_temperature_c.is_none()
        && !snapshot.delta_temperature_for_gate_read
        && snapshot.delta_temperature_for_gate_c.is_none()
        && !snapshot.delta_temperature_comparison_evaluated
        && snapshot
            .delta_temperature_below_negative_small_temp_diff
            .is_none()
        && !snapshot.delta_temperature_body_entered
        && downstream_sites_are_skipped(snapshot)
        && snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
}

fn downstream_sites_are_skipped(snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot) -> bool {
    !snapshot.zone_cooling_setpoint_load_read
        && snapshot.zone_cooling_setpoint_load_w.is_none()
        && !snapshot.cp_air_for_first_division_read
        && snapshot.cp_air_for_first_division_j_per_kg_k.is_none()
        && !snapshot.zone_cooling_setpoint_load_over_cp_air_calculated
        && snapshot
            .zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
            .is_none()
        && !snapshot.delta_temperature_for_second_division_read
        && snapshot.delta_temperature_for_second_division_c.is_none()
        && !snapshot.supply_mass_flow_rate_for_cool_calculated
        && snapshot
            .calculated_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_cool_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
}

fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
