//! Bit-exact CP415-to-CP416 latest-snapshot lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Predecessor,
    psychrometrics::energyplus_psy_w_fn_tdb_h,
};
use serde_json::{Map, Value};

use crate::pipeline::{
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment::serialization::snapshot::snapshot_json,
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit::serialization::snapshot::snapshot_json as predecessor_json,
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
        && local_shape_is_exact(snapshot, predecessor)
}

fn inherited_fields_match(snapshot: &Map<String, Value>, predecessor: &Map<String, Value>) -> bool {
    predecessor.iter().all(|(key, expected)| {
        let inherited_key = match key.as_str() {
            "source" | "first_excluded_source" | "source_order" => return true,
            "resulting_supply_humidity_ratio" => {
                "predecessor_cp415_resulting_supply_humidity_ratio"
            }
            "resulting_supply_humidity_ratio_ieee_bits" => {
                "predecessor_cp415_resulting_supply_humidity_ratio_ieee_bits"
            }
            "resulting_supply_enthalpy_j_per_kg" => {
                "predecessor_cp415_resulting_supply_enthalpy_j_per_kg"
            }
            "resulting_supply_enthalpy_j_per_kg_ieee_bits" => {
                "predecessor_cp415_resulting_supply_enthalpy_j_per_kg_ieee_bits"
            }
            "resulting_supply_temperature_c" => "predecessor_cp415_resulting_supply_temperature_c",
            "resulting_supply_temperature_c_ieee_bits" => {
                "predecessor_cp415_resulting_supply_temperature_c_ieee_bits"
            }
            key => key,
        };
        snapshot.get(inherited_key) == Some(expected)
    })
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed;
    let temperature = active
        .then_some(predecessor.resulting_supply_temperature_c)
        .flatten();
    let enthalpy = active
        .then_some(predecessor.resulting_supply_enthalpy_j_per_kg)
        .flatten();
    let psychrometric = temperature
        .zip(enthalpy)
        .map(|(temperature, enthalpy)| energyplus_psy_w_fn_tdb_h(temperature, enthalpy));

    snapshot
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed
        == active
        && snapshot.cp415_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp415_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp415_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cp415_retained_supply_temperature_owned_read == active
        && snapshot.supply_temperature_for_humidity_ratio_inversion_read == active
        && option_bits_match(snapshot.supply_temperature_c, temperature)
        && snapshot.cp415_retained_supply_enthalpy_owned_read == active
        && snapshot.supply_enthalpy_for_humidity_ratio_inversion_read == active
        && option_bits_match(snapshot.supply_enthalpy_j_per_kg, enthalpy)
        && snapshot.psychrometric_supply_humidity_ratio_evaluated == active
        && option_bits_match(snapshot.psychrometric_supply_humidity_ratio, psychrometric)
        && snapshot.supply_humidity_ratio_assignment_performed == active
        && option_bits_match(snapshot.assigned_supply_humidity_ratio, psychrometric)
        && option_bits_match(
            snapshot.resulting_supply_humidity_ratio,
            psychrometric.or(predecessor.resulting_supply_humidity_ratio),
        )
        && option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && direct_subset_values_are_valid(active, temperature, enthalpy, psychrometric)
}

fn direct_subset_values_are_valid(
    active: bool,
    temperature: Option<f64>,
    enthalpy: Option<f64>,
    psychrometric: Option<f64>,
) -> bool {
    if !active {
        return temperature.is_none() && enthalpy.is_none() && psychrometric.is_none();
    }
    temperature.is_some_and(f64::is_finite)
        && enthalpy.is_some_and(f64::is_finite)
        && psychrometric.is_some_and(f64::is_finite)
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_subset_rejects_nonfinite_operands_and_result() {
        assert!(direct_subset_values_are_valid(
            true,
            Some(14.0),
            Some(40_000.0),
            Some(0.01),
        ));
        for values in [
            (Some(f64::NAN), Some(40_000.0), Some(0.01)),
            (Some(14.0), Some(f64::INFINITY), Some(0.01)),
            (Some(14.0), Some(40_000.0), Some(f64::NAN)),
        ] {
            assert!(!direct_subset_values_are_valid(
                true, values.0, values.1, values.2,
            ));
        }
        assert!(direct_subset_values_are_valid(false, None, None, None));
    }

    #[test]
    fn option_bits_distinguish_signed_zero() {
        assert!(option_bits_match(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_match(Some(-0.0), Some(0.0)));
    }
}
