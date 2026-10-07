//! Exact CP441-to-CP442 latest-snapshot lineage checks.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot as Predecessor,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot as Snapshot,
    heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot,
};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call::serialization::snapshot::snapshot_json as cp441_snapshot_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let reconstructed =
        heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot(
            snapshot,
        );
    cp441_snapshot_json(reconstructed) == cp441_snapshot_json(predecessor)
        && snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered
            == predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
}

#[cfg(test)]
mod tests {
    #[test]
    fn lineage_is_reconstruction_based_and_marker_aliases_guard_false_fallthrough() {
        let source = include_str!("lineage.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("lineage.rs"), |(production, _)| production);
        assert!(source.contains("predecessor_cp441_snapshot"));
        assert!(source.contains("first_warning_guard_false_fallthrough"));
        assert!(source.contains("SOURCE"));
        assert!(source.contains("EXCLUDED"));
        assert!(source.contains("ORDER"));
        for forbidden in ["ShowContinueError", "DirectZonePurchasedAirCouplingInput"] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
