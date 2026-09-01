//! Bounded CP435-to-CP436 latest-snapshot lineage validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Predecessor,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot,
};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_guard::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    standard_air_density_kg_per_m3: f64,
) -> bool {
    predecessor_json(
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot(
            snapshot,
        ),
    ) == predecessor_json(predecessor)
        && snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && same(
            snapshot.predecessor_cp435_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp435_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp435_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && snapshot.cp435_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp435_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp435_retained_supply_temperature_state_owned
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
        && assignment_shape_is_exact(snapshot, predecessor, standard_air_density_kg_per_m3)
}

fn assignment_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    standard_air_density_kg_per_m3: f64,
) -> bool {
    let executed = predecessor.maximum_heating_flow_body_entered;
    if snapshot.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed != executed {
        return false;
    }
    if !executed {
        return !snapshot.cp435_retained_outdoor_air_mass_flow_rate_owned_read
            && !snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read
            && snapshot
                .outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s
                .is_none()
            && !snapshot.begin_environment_standard_air_density_owned_read
            && !snapshot.standard_air_density_for_outdoor_air_volume_flow_division_read
            && snapshot
                .standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3
                .is_none()
            && !snapshot.outdoor_air_mass_flow_rate_standard_air_density_division_evaluated
            && snapshot
                .calculated_outdoor_air_volume_flow_rate_m3_per_s
                .is_none()
            && !snapshot.local_outdoor_air_volume_flow_rate_assignment_performed
            && snapshot
                .assigned_outdoor_air_volume_flow_rate_m3_per_s
                .is_none();
    }
    let Some(mass_flow) =
        predecessor.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s
    else {
        return false;
    };
    let calculated = mass_flow / standard_air_density_kg_per_m3;
    snapshot.cp435_retained_outdoor_air_mass_flow_rate_owned_read
        && snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read
        && option_has_bits(
            snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s,
            mass_flow,
        )
        && snapshot.begin_environment_standard_air_density_owned_read
        && snapshot.standard_air_density_for_outdoor_air_volume_flow_division_read
        && option_has_bits(
            snapshot.standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3,
            standard_air_density_kg_per_m3,
        )
        && snapshot.outdoor_air_mass_flow_rate_standard_air_density_division_evaluated
        && option_has_bits(
            snapshot.calculated_outdoor_air_volume_flow_rate_m3_per_s,
            calculated,
        )
        && snapshot.local_outdoor_air_volume_flow_rate_assignment_performed
        && option_has_bits(
            snapshot.assigned_outdoor_air_volume_flow_rate_m3_per_s,
            calculated,
        )
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
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
        assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2363");
        assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2364");
        assert_eq!(ORDER.len(), 4);
    }
}
