//! Bit-exact CP406 coupled snapshot equality.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Snapshot;

pub(super) fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = options_have_exact_bits(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }

    let values_match = compare_clear!(predecessor_cp405_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp405_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp405_resulting_supply_temperature_c)
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
