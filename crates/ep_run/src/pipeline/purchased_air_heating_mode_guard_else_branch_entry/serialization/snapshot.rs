//! Lossless JSON serialization for one CP433 heating-mode-guard else-entry snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    heating_mode_guard_else_branch_entry_predecessor_cp432_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_heating_operating_mode_heat_assignment::serialization::snapshot::snapshot_json as cp432_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
) -> Value {
    let predecessor = heating_mode_guard_else_branch_entry_predecessor_cp432_snapshot(snapshot);
    let mut value = cp432_snapshot_json(predecessor);
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
        "heating_mode_guard_else_branch_entered".to_string(),
        json!(snapshot.heating_mode_guard_else_branch_entered),
    );
    value
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_source_preserves_cp432_prefix_and_extends_one_marker() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp432_snapshot_json(predecessor)"));
        assert!(!source.contains("target.remove"));
        assert_eq!(
            source
                .matches("\"heating_mode_guard_else_branch_entered\"")
                .count(),
            1
        );
        for forbidden in [
            "DirectZonePurchasedAirCouplingInput",
            "numerical_dto",
            "Deadband",
            "calculation.mode",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
