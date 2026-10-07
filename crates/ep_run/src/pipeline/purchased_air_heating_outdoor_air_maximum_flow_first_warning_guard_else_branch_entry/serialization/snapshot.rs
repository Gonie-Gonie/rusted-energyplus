//! Lossless JSON serialization for one CP442 first-warning guard else-entry snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot,
    heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call::serialization::snapshot::snapshot_json as cp441_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot,
) -> Value {
    let predecessor =
        heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot(
            snapshot,
        );
    let mut value = cp441_snapshot_json(predecessor);
    let Value::Object(target) = &mut value else {
        return Value::Null;
    };
    target.insert("source".to_string(), json!(snapshot.source));
    target.insert(
        "first_excluded_source".to_string(),
        json!(snapshot.first_excluded_source),
    );
    target.insert("source_order".to_string(), json!(snapshot.source_order));
    target.insert(
        "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered".to_string(),
        json!(snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered),
    );
    value
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_source_preserves_cp441_json_and_appends_only_the_entry_marker() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp441_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.insert(").count(), 4);
        assert!(
            source.contains(
                "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered"
            )
        );
        for forbidden in ["target.remove", "ieee_bits", "json_number"] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
