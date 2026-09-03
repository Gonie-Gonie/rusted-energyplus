//! Exact CP438-to-CP439 latest-snapshot lineage checks.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot as Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as Predecessor,
    heating_outdoor_air_maximum_flow_first_warning_call_predecessor_cp438_snapshot,
};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_first_warning_counter_increment::serialization::snapshot::snapshot_json as cp438_snapshot_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let reconstructed =
        heating_outdoor_air_maximum_flow_first_warning_call_predecessor_cp438_snapshot(snapshot);
    cp438_snapshot_json(reconstructed) == cp438_snapshot_json(predecessor)
        && !predecessor.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
        && !snapshot.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
}

#[cfg(test)]
mod tests {
    #[test]
    fn lineage_is_reconstruction_based_and_public_release_skips_the_call() {
        let source = include_str!("lineage.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("lineage.rs"), |(production, _)| production);
        assert!(source.contains("predecessor_cp438_snapshot"));
        assert!(source.contains("call_site_reached"));
        for forbidden in ["ShowWarningError", "DirectZonePurchasedAirCouplingInput"] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
