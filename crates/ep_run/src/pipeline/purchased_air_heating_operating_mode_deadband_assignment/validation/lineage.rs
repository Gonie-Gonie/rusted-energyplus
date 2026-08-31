//! Bounded CP433-to-CP434 latest-snapshot lineage validation.

use ep_runtime::{
    IdealLoadsSensibleMode,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot as Snapshot,
    heating_operating_mode_deadband_assignment_predecessor_cp433_snapshot,
};

use crate::pipeline::purchased_air_heating_mode_guard_else_branch_entry::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let assignment = predecessor.heating_mode_guard_else_branch_entered;
    predecessor_json(
        heating_operating_mode_deadband_assignment_predecessor_cp433_snapshot(snapshot),
    ) == predecessor_json(predecessor)
        && provenance_is_exact(
            snapshot.source,
            snapshot.first_excluded_source,
            snapshot.source_order,
        )
        && same(
            snapshot.predecessor_cp433_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp433_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp433_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && snapshot.heating_mode_guard_else_branch_entered == assignment
        && snapshot.heating_operating_mode_deadband_assignment_executed == assignment
        && snapshot.cp433_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp433_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp433_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.heating_operating_mode_deadband_assignment_performed == assignment
        && snapshot.assigned_heating_operating_mode_deadband
            == assignment.then_some(IdealLoadsSensibleMode::Deadband)
        && !(assignment && predecessor.heating_operating_mode_heat_assignment_executed)
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
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == SOURCE && first_excluded_source == EXCLUDED && source_order == ORDER
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
