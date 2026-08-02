//! Bit-exact CP403 coupled snapshot equality.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot as Snapshot;

pub(super) fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    let values_match = [
        (
            left.predecessor_cp397_resulting_supply_humidity_ratio,
            right.predecessor_cp397_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp397_resulting_supply_temperature_c,
            right.predecessor_cp397_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_cp398_resulting_supply_humidity_ratio,
            right.predecessor_cp398_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp398_resulting_supply_temperature_c,
            right.predecessor_cp398_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_mixed_air_humidity_ratio,
            right.predecessor_mixed_air_humidity_ratio,
        ),
        (
            left.predecessor_psychrometric_cp_air_result_j_per_kg_k,
            right.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        ),
        (
            left.predecessor_cp_air_j_per_kg_k,
            right.predecessor_cp_air_j_per_kg_k,
        ),
        (
            left.predecessor_cp399_resulting_supply_humidity_ratio,
            right.predecessor_cp399_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp399_resulting_supply_temperature_c,
            right.predecessor_cp399_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_supply_mass_flow_rate_kg_per_s,
            right.predecessor_supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.predecessor_cp400_cp_air_j_per_kg_k,
            right.predecessor_cp400_cp_air_j_per_kg_k,
        ),
        (
            left.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
            right.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
        ),
        (
            left.predecessor_mixed_air_temperature_c,
            right.predecessor_mixed_air_temperature_c,
        ),
        (
            left.predecessor_supply_temperature_c,
            right.predecessor_supply_temperature_c,
        ),
        (
            left.predecessor_mixed_air_minus_supply_temperature_k,
            right.predecessor_mixed_air_minus_supply_temperature_k,
        ),
        (
            left.predecessor_calculated_cooling_sensible_output_w,
            right.predecessor_calculated_cooling_sensible_output_w,
        ),
        (
            left.predecessor_cooling_sensible_output_w,
            right.predecessor_cooling_sensible_output_w,
        ),
        (
            left.predecessor_cp400_resulting_supply_humidity_ratio,
            right.predecessor_cp400_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp400_resulting_supply_temperature_c,
            right.predecessor_cp400_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_cooling_total_output_w,
            right.predecessor_cooling_total_output_w,
        ),
        (
            left.predecessor_cp401_cooling_sensible_output_w,
            right.predecessor_cp401_cooling_sensible_output_w,
        ),
        (
            left.predecessor_calculated_cooling_latent_output_w,
            right.predecessor_calculated_cooling_latent_output_w,
        ),
        (
            left.predecessor_cooling_latent_output_w,
            right.predecessor_cooling_latent_output_w,
        ),
        (
            left.predecessor_cp401_resulting_supply_humidity_ratio,
            right.predecessor_cp401_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp401_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp401_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp401_resulting_supply_temperature_c,
            right.predecessor_cp401_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_cp402_cooling_latent_output_w,
            right.predecessor_cp402_cooling_latent_output_w,
        ),
        (
            left.predecessor_maximum_total_cooling_capacity_w,
            right.predecessor_maximum_total_cooling_capacity_w,
        ),
        (
            left.predecessor_cp402_resulting_supply_humidity_ratio,
            right.predecessor_cp402_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp402_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp402_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp402_resulting_supply_temperature_c,
            right.predecessor_cp402_resulting_supply_temperature_c,
        ),
        (left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        (
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));

    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp397_resulting_supply_temperature_c = None;
        snapshot.predecessor_cp398_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp398_resulting_supply_temperature_c = None;
        snapshot.predecessor_mixed_air_humidity_ratio = None;
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.predecessor_cp_air_j_per_kg_k = None;
        snapshot.predecessor_cp399_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp399_resulting_supply_temperature_c = None;
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s = None;
        snapshot.predecessor_cp400_cp_air_j_per_kg_k = None;
        snapshot.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k = None;
        snapshot.predecessor_mixed_air_temperature_c = None;
        snapshot.predecessor_supply_temperature_c = None;
        snapshot.predecessor_mixed_air_minus_supply_temperature_k = None;
        snapshot.predecessor_calculated_cooling_sensible_output_w = None;
        snapshot.predecessor_cooling_sensible_output_w = None;
        snapshot.predecessor_cp400_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp400_resulting_supply_temperature_c = None;
        snapshot.predecessor_cooling_total_output_w = None;
        snapshot.predecessor_cp401_cooling_sensible_output_w = None;
        snapshot.predecessor_calculated_cooling_latent_output_w = None;
        snapshot.predecessor_cooling_latent_output_w = None;
        snapshot.predecessor_cp401_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp401_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp401_resulting_supply_temperature_c = None;
        snapshot.predecessor_cp402_cooling_latent_output_w = None;
        snapshot.predecessor_maximum_total_cooling_capacity_w = None;
        snapshot.predecessor_cp402_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp402_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp402_resulting_supply_temperature_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
        snapshot.resulting_supply_humidity_ratio = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left == right
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
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
    fn exact_bits_distinguish_signed_zero() {
        assert!(options_have_exact_bits(Some(-0.0), Some(-0.0)));
        assert!(!options_have_exact_bits(Some(-0.0), Some(0.0)));
    }
}
