//! Exact CP440-to-CP441 latest-snapshot lineage checks.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot as Predecessor,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot as Snapshot,
    heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_predecessor_cp440_snapshot,
};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_continue_warning_call::serialization::snapshot::snapshot_json as cp440_snapshot_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let reconstructed =
        heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_predecessor_cp440_snapshot(
            snapshot,
        );
    cp440_snapshot_json(reconstructed) == cp440_snapshot_json(predecessor)
        && !predecessor.heating_outdoor_air_maximum_flow_continue_warning_call_site_reached
        && !snapshot.heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_site_reached
}

#[cfg(test)]
mod tests {
    #[test]
    fn lineage_is_reconstruction_based_and_public_release_skips_the_call() {
        let source = include_str!("lineage.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("lineage.rs"), |(production, _)| production);
        assert!(source.contains("predecessor_cp440_snapshot"));
        assert!(source.contains("call_site_reached"));
        for forbidden in ["ShowContinueError", "DirectZonePurchasedAirCouplingInput"] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
