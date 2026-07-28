//! Fail-closed validation helpers for CP332 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirCalcEntrySnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state.supply_temperature_assignment_count;
    let source_sites = checked_product(
        assignments,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len(),
        "source-site count",
    )?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "zone_cooling_setpoint_load_read_count",
            assignments,
            state.zone_cooling_setpoint_load_read_count,
        ),
        ("cp_air_read_count", assignments, state.cp_air_read_count),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "cp_air_times_supply_mass_flow_rate_calculation_count",
            assignments,
            state.cp_air_times_supply_mass_flow_rate_calculation_count,
        ),
        (
            "zone_cooling_setpoint_load_over_denominator_calculation_count",
            assignments,
            state.zone_cooling_setpoint_load_over_denominator_calculation_count,
        ),
        (
            "zone_node_temperature_read_count",
            assignments,
            state.zone_node_temperature_read_count,
        ),
        (
            "supply_temperature_calculation_count",
            assignments,
            state.supply_temperature_calculation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            assignments,
            state.supply_temperature_assignment_write_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling positive-supply temperature assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    entry: &PurchasedAirCalcEntrySnapshot,
    sensible_flow: &PurchasedAirCalcCoolingSensibleFlowSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    positive_guard: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let assignment_expected = predecessor.cp_air_assignment_executed;
    if snapshot.supply_temperature_assignment_executed != assignment_expected
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
    {
        return false;
    }
    if !assignment_expected {
        return skipped_source_shape(snapshot);
    }

    let Some(load) = snapshot.zone_cooling_setpoint_load_w else {
        return false;
    };
    let Some(cp_air) = snapshot.cp_air_j_per_kg_k else {
        return false;
    };
    let Some(mass_flow) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(zone_node_temperature) = snapshot.zone_node_temperature_c else {
        return false;
    };
    let denominator = cp_air * mass_flow;
    let load_temperature = load / denominator;
    let supply_temperature = load_temperature + zone_node_temperature;

    snapshot.zone_cooling_setpoint_load_read
        && same_option(
            snapshot.zone_cooling_setpoint_load_w,
            Some(entry.demand.remaining_output_req_to_cool_sp_w),
        )
        && snapshot.cp_air_read
        && same_option(snapshot.cp_air_j_per_kg_k, predecessor.cp_air_j_per_kg_k)
        && snapshot.supply_mass_flow_rate_read
        && same_option(
            snapshot.supply_mass_flow_rate_kg_per_s,
            positive_guard.supply_mass_flow_rate_kg_per_s,
        )
        && snapshot.cp_air_times_supply_mass_flow_rate_calculated
        && same_option(
            snapshot.cp_air_times_supply_mass_flow_rate_w_per_k,
            Some(denominator),
        )
        && snapshot.zone_cooling_setpoint_load_over_denominator_calculated
        && same_option(
            snapshot.zone_cooling_setpoint_load_over_denominator_c,
            Some(load_temperature),
        )
        && snapshot.zone_node_temperature_read
        && same_option(
            snapshot.zone_node_temperature_c,
            sensible_flow.zone_temperature_c,
        )
        && same_option(
            snapshot.zone_node_temperature_c,
            mixed_air.recirculation_temperature_c,
        )
        && same_option(
            snapshot.zone_node_temperature_c,
            mixed_air.mixed_air_temperature_c,
        )
        && snapshot.supply_temperature_calculated
        && same_option(
            snapshot.calculated_supply_temperature_c,
            Some(supply_temperature),
        )
        && snapshot.supply_temperature_assigned
        && same_option(snapshot.supply_temperature_c, Some(supply_temperature))
}

fn skipped_source_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
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

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply temperature assignment {label} overflowed"
        )
    })
}

fn same_option(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.supply_temperature_assignment_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }
}
