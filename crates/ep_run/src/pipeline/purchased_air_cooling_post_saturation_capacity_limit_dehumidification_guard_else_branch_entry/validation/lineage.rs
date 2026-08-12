//! Bit-exact CP417-to-CP418 latest-snapshot lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Predecessor,
};
use serde_json::{Map, Value};

use crate::pipeline::{
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry::serialization::snapshot::snapshot_json,
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment::serialization::snapshot::snapshot_json as predecessor_json,
};

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let snapshot_value = snapshot_json(snapshot);
    let predecessor_value = predecessor_json(predecessor);
    let (Some(snapshot_map), Some(predecessor_map)) =
        (snapshot_value.as_object(), predecessor_value.as_object())
    else {
        return false;
    };
    inherited_fields_match(snapshot_map, predecessor_map)
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
            == predecessor.predecessor_dehumidification_guard_false_fallthrough
}

fn inherited_fields_match(snapshot: &Map<String, Value>, predecessor: &Map<String, Value>) -> bool {
    predecessor.iter().all(|(key, expected)| {
        if matches!(
            key.as_str(),
            "source" | "first_excluded_source" | "source_order"
        ) {
            true
        } else {
            snapshot.get(key) == Some(expected)
        }
    })
}
