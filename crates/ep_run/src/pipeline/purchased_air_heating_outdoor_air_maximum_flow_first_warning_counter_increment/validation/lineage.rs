//! Exact CP437-to-CP438 latest-snapshot lineage checks.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Predecessor,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_predecessor_cp437_snapshot,
};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_first_warning_guard::serialization::snapshot::snapshot_json as cp437_snapshot_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let reconstructed =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_predecessor_cp437_snapshot(
            snapshot,
        );
    cp437_snapshot_json(reconstructed) == cp437_snapshot_json(predecessor)
        && !predecessor.heating_outdoor_air_maximum_flow_first_warning_branch_entered
        && !snapshot.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
        && snapshot.cp437_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp437_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp437_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && !snapshot.cp437_retained_outdoor_air_flow_maximum_heating_output_error_count_state_owned
        && !snapshot.outdoor_air_flow_maximum_heating_output_error_count_increment_performed
        && snapshot
            .assigned_outdoor_air_flow_maximum_heating_output_error_count
            .is_none()
        && option_bits_equal(
            snapshot.predecessor_cp437_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.predecessor_cp437_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.predecessor_cp437_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lineage_is_reconstruction_based_and_excludes_the_warning_call() {
        let source = include_str!("lineage.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("lineage.rs"), |(production, _)| production);
        assert!(source.contains("predecessor_cp437_snapshot"));
        assert!(source.contains("increment_performed"));
        assert!(!source.contains("ShowWarningError"));
    }
}
