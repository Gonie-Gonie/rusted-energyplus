//! Bit-exact CP404 coupled snapshot equality.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Snapshot;

pub(super) fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = options_have_exact_bits(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }

    let values_match = compare_clear!(predecessor_cp397_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp397_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp397_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp398_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp398_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp398_resulting_supply_temperature_c)
        && compare_clear!(predecessor_mixed_air_humidity_ratio)
        && compare_clear!(predecessor_psychrometric_cp_air_result_j_per_kg_k)
        && compare_clear!(predecessor_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_cp399_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp399_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp399_resulting_supply_temperature_c)
        && compare_clear!(predecessor_supply_mass_flow_rate_kg_per_s)
        && compare_clear!(predecessor_cp400_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_supply_mass_flow_rate_times_cp_air_w_per_k)
        && compare_clear!(predecessor_mixed_air_temperature_c)
        && compare_clear!(predecessor_supply_temperature_c)
        && compare_clear!(predecessor_mixed_air_minus_supply_temperature_k)
        && compare_clear!(predecessor_calculated_cooling_sensible_output_w)
        && compare_clear!(predecessor_cooling_sensible_output_w)
        && compare_clear!(predecessor_cp400_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp400_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp400_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cooling_total_output_w)
        && compare_clear!(predecessor_cp401_cooling_sensible_output_w)
        && compare_clear!(predecessor_calculated_cooling_latent_output_w)
        && compare_clear!(predecessor_cooling_latent_output_w)
        && compare_clear!(predecessor_cp401_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp401_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp401_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp402_cooling_latent_output_w)
        && compare_clear!(predecessor_maximum_total_cooling_capacity_w)
        && compare_clear!(predecessor_cp402_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp402_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp402_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp403_mixed_air_temperature_c)
        && compare_clear!(predecessor_cp403_assigned_supply_temperature_c)
        && compare_clear!(predecessor_cp403_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp403_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp403_resulting_supply_temperature_c)
        && compare_clear!(supply_temperature_c)
        && compare_clear!(supply_enthalpy_j_per_kg)
        && compare_clear!(psychrometric_supply_humidity_ratio)
        && compare_clear!(assigned_supply_humidity_ratio)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);

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
