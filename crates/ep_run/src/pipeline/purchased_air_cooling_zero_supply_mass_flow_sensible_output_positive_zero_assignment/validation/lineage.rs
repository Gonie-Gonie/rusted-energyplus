//! Bounded CP427-to-CP428 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SENSIBLE_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SENSIBLE_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SENSIBLE_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot as Predecessor,
    cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_predecessor_cp427_snapshot,
};

use crate::pipeline::purchased_air_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(
        cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_predecessor_cp427_snapshot(
            snapshot,
        ),
    ) == predecessor_json(predecessor)
        && local_shape_is_exact(snapshot, predecessor)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let assignment =
        predecessor.cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed;
    provenance_is_exact(
        snapshot.source,
        snapshot.first_excluded_source,
        snapshot.source_order,
    ) && snapshot.cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed
        == predecessor
            .cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed
        && snapshot.cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_executed
            == assignment
        && snapshot.cp427_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp427_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp427_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cooling_sensible_output_positive_zero_assignment_performed == assignment
        && same(
            snapshot.predecessor_cp427_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp427_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp427_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
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
        && if assignment {
            positive_zero_is_exact(snapshot.assigned_cooling_sensible_output_w)
        } else {
            snapshot.assigned_cooling_sensible_output_w.is_none()
        }
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == SOURCE && first_excluded_source == EXCLUDED && source_order == ORDER
}

pub(super) fn positive_zero_is_exact(value: Option<f64>) -> bool {
    value.is_some_and(|value| value.to_bits() == 0.0_f64.to_bits())
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
