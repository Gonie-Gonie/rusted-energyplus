//! Bounded CP423-to-CP424 latest-snapshot lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Snapshot,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_predecessor_cp423_snapshot,
};

use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(
        cooling_supply_mass_flow_positive_guard_else_branch_entry_predecessor_cp423_snapshot(
            snapshot,
        ),
    ) == predecessor_json(predecessor)
        && snapshot.cooling_supply_mass_flow_positive_guard_else_branch_entered
            == predecessor.positive_guard_false_fallthrough_skipped
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

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
