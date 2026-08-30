//! Bounded CP432-to-CP433 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Predecessor,
    heating_mode_guard_else_branch_entry_predecessor_cp432_snapshot,
};

use crate::pipeline::purchased_air_heating_operating_mode_heat_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(heating_mode_guard_else_branch_entry_predecessor_cp432_snapshot(snapshot))
        == predecessor_json(predecessor)
        && provenance_is_exact(
            snapshot.source,
            snapshot.first_excluded_source,
            snapshot.source_order,
        )
        && snapshot.heating_mode_guard_else_branch_entered
            == predecessor.heating_mode_guard_false_fallthrough
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == SOURCE && first_excluded_source == EXCLUDED && source_order == ORDER
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
