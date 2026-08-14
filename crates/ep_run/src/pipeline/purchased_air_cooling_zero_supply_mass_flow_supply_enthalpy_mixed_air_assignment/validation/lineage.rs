//! Bounded CP424-to-CP425 latest-snapshot lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot as Snapshot,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_predecessor_cp424_snapshot,
};

use crate::pipeline::purchased_air_cooling_supply_mass_flow_positive_guard_else_branch_entry::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(
        cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_predecessor_cp424_snapshot(
            snapshot,
        ),
    ) == predecessor_json(predecessor)
        && local_shape_is_exact(snapshot, predecessor)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let assignment = predecessor.cooling_supply_mass_flow_positive_guard_else_branch_entered;
    snapshot.cooling_supply_mass_flow_positive_guard_else_branch_entered == assignment
        && snapshot.cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_executed
            == assignment
        && snapshot.cp424_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp424_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp424_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cp329_retained_mixed_air_enthalpy_owned_read == assignment
        && snapshot.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read == assignment
        && snapshot.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_performed
            == assignment
        && same(
            snapshot.predecessor_cp424_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp424_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp424_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && same(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && if assignment {
            active_enthalpy_chain_is_exact(
                snapshot.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg,
                snapshot.assigned_supply_enthalpy_from_mixed_air_j_per_kg,
                snapshot.resulting_supply_enthalpy_j_per_kg,
            )
        } else {
            snapshot
                .mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg
                .is_none()
                && snapshot
                    .assigned_supply_enthalpy_from_mixed_air_j_per_kg
                    .is_none()
                && same(
                    snapshot.resulting_supply_enthalpy_j_per_kg,
                    predecessor.resulting_supply_enthalpy_j_per_kg,
                )
        }
}

pub(super) fn active_enthalpy_chain_is_exact(
    mixed_air_rhs: Option<f64>,
    assigned: Option<f64>,
    resulting: Option<f64>,
) -> bool {
    match (mixed_air_rhs, assigned, resulting) {
        (Some(mixed_air_rhs), Some(assigned), Some(resulting)) => {
            mixed_air_rhs.to_bits() == assigned.to_bits()
                && assigned.to_bits() == resulting.to_bits()
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
