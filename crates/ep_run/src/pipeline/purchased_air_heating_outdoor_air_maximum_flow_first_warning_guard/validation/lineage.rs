//! Bounded CP436-to-CP437 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Predecessor,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Snapshot,
    heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot,
};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_body_volume_flow_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor_json(
        heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot(snapshot),
    ) == predecessor_json(predecessor)
        && snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && same(
            snapshot.predecessor_cp436_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp436_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp436_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && snapshot.cp436_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp436_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp436_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
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
        && guard_shape_is_exact(snapshot, predecessor)
}

fn guard_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let evaluated = predecessor.local_outdoor_air_volume_flow_rate_assignment_performed;
    if snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated != evaluated {
        return false;
    }
    if !evaluated {
        return !snapshot.outdoor_air_flow_maximum_heating_output_error_count_state_owned
            && !snapshot.outdoor_air_flow_maximum_heating_output_error_count_read
            && snapshot
                .outdoor_air_flow_maximum_heating_output_error_count_before
                .is_none()
            && !snapshot
                .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated
            && snapshot
                .outdoor_air_flow_maximum_heating_output_error_count_less_than_one
                .is_none()
            && !snapshot.heating_outdoor_air_maximum_flow_first_warning_branch_entered
            && !snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough;
    }
    let before = snapshot.outdoor_air_flow_maximum_heating_output_error_count_before;
    let expected = before.map(|count| count < 1);
    snapshot.outdoor_air_flow_maximum_heating_output_error_count_state_owned
        && snapshot.outdoor_air_flow_maximum_heating_output_error_count_read
        && before.is_some()
        && snapshot
            .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated
        && snapshot.outdoor_air_flow_maximum_heating_output_error_count_less_than_one == expected
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_branch_entered
            == (expected == Some(true))
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
            == (expected == Some(false))
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
    use super::{EXCLUDED, ORDER, SOURCE};

    #[test]
    fn exact_provenance_constants_are_locked() {
        assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2364");
        assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2365");
        assert_eq!(ORDER.len(), 3);
    }
}
