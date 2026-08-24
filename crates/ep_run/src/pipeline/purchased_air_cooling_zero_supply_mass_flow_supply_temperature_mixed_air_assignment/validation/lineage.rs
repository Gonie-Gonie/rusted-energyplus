//! Bounded CP426-to-CP427 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot as Snapshot,
    cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_predecessor_cp426_snapshot,
};

use crate::pipeline::purchased_air_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(
        cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_predecessor_cp426_snapshot(
            snapshot,
        ),
    ) == predecessor_json(predecessor)
        && local_shape_is_exact(snapshot, predecessor)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let assignment = predecessor
        .cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed;
    provenance_is_exact(
        snapshot.source,
        snapshot.first_excluded_source,
        snapshot.source_order,
    ) && snapshot.cooling_supply_mass_flow_positive_guard_else_branch_entered
        == predecessor.cooling_supply_mass_flow_positive_guard_else_branch_entered
        && snapshot.cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed
            == predecessor.cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed
        && snapshot.cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed
            == assignment
        && snapshot.cp426_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp426_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp426_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cp329_retained_mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_owned_read
            == assignment
        && snapshot.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_read
            == assignment
        && snapshot.zero_supply_mass_flow_supply_temperature_mixed_air_assignment_performed
            == assignment
        && same(snapshot.predecessor_cp426_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        && same(snapshot.predecessor_cp426_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        && same(snapshot.predecessor_cp426_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        && same(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        && same(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        && if assignment {
            active_temperature_chain_is_exact(
                snapshot.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c,
                snapshot.assigned_supply_temperature_from_mixed_air_c,
                snapshot.resulting_supply_temperature_c,
            )
        } else {
            snapshot.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c.is_none()
                && snapshot.assigned_supply_temperature_from_mixed_air_c.is_none()
                && same(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        }
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == SOURCE && first_excluded_source == EXCLUDED && source_order == ORDER
}

pub(super) fn active_temperature_chain_is_exact(
    rhs: Option<f64>,
    assigned: Option<f64>,
    resulting: Option<f64>,
) -> bool {
    match (rhs, assigned, resulting) {
        (Some(rhs), Some(assigned), Some(resulting)) => {
            rhs.to_bits() == assigned.to_bits() && assigned.to_bits() == resulting.to_bits()
        }
        _ => false,
    }
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
