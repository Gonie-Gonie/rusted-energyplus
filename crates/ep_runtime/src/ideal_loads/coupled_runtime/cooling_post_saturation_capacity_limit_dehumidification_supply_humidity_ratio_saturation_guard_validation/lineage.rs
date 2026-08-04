//! Bit-exact CP413 coupled snapshot equality.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Snapshot;

pub(super) fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = option_bits_match(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let values_match = compare_clear!(predecessor_cp409_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp409_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp409_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp410_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp410_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp410_resulting_supply_temperature_c)
        && compare_clear!(purchased_air_supply_humidity_ratio_before_saturation_check)
        && compare_clear!(assigned_supply_humidity_ratio_original)
        && compare_clear!(resulting_supply_humidity_ratio_original)
        && compare_clear!(predecessor_cp411_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp411_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp411_resulting_supply_temperature_c)
        && compare_clear!(supply_temperature_for_saturation_humidity_ratio_c)
        && compare_clear!(outdoor_barometric_pressure_pa)
        && compare_clear!(saturation_supply_humidity_ratio)
        && compare_clear!(assigned_saturation_supply_humidity_ratio)
        && compare_clear!(resulting_saturation_supply_humidity_ratio)
        && compare_clear!(predecessor_cp412_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp412_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp412_resulting_supply_temperature_c)
        && compare_clear!(saturation_supply_humidity_ratio_for_guard)
        && compare_clear!(original_supply_humidity_ratio_for_guard)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
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
    fn option_bits_distinguish_signed_zero() {
        assert!(option_bits_match(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_match(Some(-0.0), Some(0.0)));
    }
}
