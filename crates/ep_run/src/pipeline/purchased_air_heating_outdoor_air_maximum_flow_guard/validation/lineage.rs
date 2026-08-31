//! Bounded CP434-to-CP435 latest-snapshot lineage validation.

use ep_model::IdealLoadsLimit;
use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot as Predecessor,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Snapshot,
    heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot,
};

use crate::pipeline::purchased_air_heating_operating_mode_deadband_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    maximum_heating_air_mass_flow_rate_kg_per_s: f64,
) -> bool {
    predecessor_json(heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot(snapshot))
        == predecessor_json(predecessor)
        && provenance_is_exact(
            snapshot.source,
            snapshot.first_excluded_source,
            snapshot.source_order,
        )
        && same(
            snapshot.predecessor_cp434_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp434_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp434_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && snapshot.cp434_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp434_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp434_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && same(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && guard_shape_is_exact(snapshot, maximum_heating_air_mass_flow_rate_kg_per_s)
}

fn guard_shape_is_exact(
    snapshot: Snapshot,
    maximum_heating_air_mass_flow_rate_kg_per_s: f64,
) -> bool {
    let evaluated = predecessor_heating_case_entered(snapshot);
    if snapshot.heating_outdoor_air_maximum_flow_guard_evaluated != evaluated {
        return false;
    }
    if !evaluated {
        return skipped_shape(snapshot);
    }
    let Some(limit) = snapshot.heating_limit_flow_rate_value else {
        return false;
    };
    let first_match = limit == IdealLoadsLimit::LimitFlowRate;
    let second_evaluated = !first_match;
    let second_match = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_limit_active = first_match || second_match;
    if !snapshot.heating_limit_flow_rate_comparison_evaluated
        || snapshot.heating_limit_flow_rate_comparison_satisfied != Some(first_match)
        || snapshot.heating_limit_flow_rate_and_capacity_comparison_evaluated != second_evaluated
        || snapshot.heating_limit_flow_rate_and_capacity_value != second_evaluated.then_some(limit)
        || snapshot.heating_limit_flow_rate_and_capacity_comparison_satisfied
            != second_evaluated.then_some(second_match)
        || snapshot.heating_flow_limit_active != Some(flow_limit_active)
        || snapshot.heating_flow_limit_selector_rejected == flow_limit_active
    {
        return false;
    }
    if !flow_limit_active {
        return no_strict_comparison_shape(snapshot)
            && !snapshot.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated
            && snapshot.heating_outdoor_air_maximum_flow_guard_false_fallthrough;
    }
    snapshot.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated
        && snapshot.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit
        && option_has_bits(
            snapshot.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s,
            0.0,
        )
        && snapshot.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit
        && option_has_bits(
            snapshot.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s,
            maximum_heating_air_mass_flow_rate_kg_per_s,
        )
        && snapshot
            .outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated
        && snapshot
            .outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate
            == Some(false)
        && !snapshot.maximum_heating_flow_body_entered
        && snapshot.heating_outdoor_air_maximum_flow_guard_false_fallthrough
}

fn predecessor_heating_case_entered(snapshot: Snapshot) -> bool {
    heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot(snapshot)
        .heating_or_no_load_case_entered
}

fn skipped_shape(snapshot: Snapshot) -> bool {
    !snapshot.heating_limit_flow_rate_comparison_evaluated
        && snapshot.heating_limit_flow_rate_value.is_none()
        && snapshot
            .heating_limit_flow_rate_comparison_satisfied
            .is_none()
        && !snapshot.heating_limit_flow_rate_and_capacity_comparison_evaluated
        && snapshot
            .heating_limit_flow_rate_and_capacity_value
            .is_none()
        && snapshot
            .heating_limit_flow_rate_and_capacity_comparison_satisfied
            .is_none()
        && snapshot.heating_flow_limit_active.is_none()
        && !snapshot.heating_flow_limit_selector_rejected
        && !snapshot.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated
        && no_strict_comparison_shape(snapshot)
        && !snapshot.heating_outdoor_air_maximum_flow_guard_false_fallthrough
}

fn no_strict_comparison_shape(snapshot: Snapshot) -> bool {
    !snapshot.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit
        && snapshot
            .outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s
            .is_none()
        && !snapshot.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit
        && snapshot
            .maximum_heating_air_mass_flow_rate_for_guard_kg_per_s
            .is_none()
        && !snapshot
            .outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated
        && snapshot
            .outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate
            .is_none()
        && !snapshot.maximum_heating_flow_body_entered
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == SOURCE && first_excluded_source == EXCLUDED && source_order == ORDER
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{EXCLUDED, ORDER, SOURCE, provenance_is_exact};

    #[test]
    fn snapshot_provenance_rejects_each_coordinated_field_forgery() {
        assert!(provenance_is_exact(SOURCE, EXCLUDED, ORDER));
        assert!(!provenance_is_exact("forged source", EXCLUDED, ORDER));
        assert!(!provenance_is_exact(SOURCE, "forged exclusion", ORDER));
        assert!(!provenance_is_exact(SOURCE, EXCLUDED, &["forged order"]));
    }
}
