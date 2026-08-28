//! Bounded CP429-to-CP430 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot as Predecessor,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Snapshot,
    heating_or_no_load_case_entry_predecessor_cp429_snapshot,
};

use crate::pipeline::purchased_air_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(heating_or_no_load_case_entry_predecessor_cp429_snapshot(
        snapshot,
    )) == predecessor_json(predecessor)
        && provenance_is_exact(
            snapshot.source,
            snapshot.first_excluded_source,
            snapshot.source_order,
        )
        && snapshot.heating_or_no_load_case_entered == predecessor.non_cooling_skipped
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
