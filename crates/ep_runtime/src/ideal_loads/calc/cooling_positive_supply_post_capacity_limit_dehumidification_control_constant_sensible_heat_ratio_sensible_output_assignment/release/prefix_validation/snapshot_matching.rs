//! Bit-exact matching for CP350 retained active-operand owners.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot as Cp344Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot as Cp345Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot as Cp334Snapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as Cp330Snapshot,
};

pub(super) fn cp345_snapshots_match(mut left: Cp345Snapshot, mut right: Cp345Snapshot) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.assigned_supply_humidity_ratio,
        right.assigned_supply_humidity_ratio,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
    }
    values_match && left == right
}

pub(super) fn cp330_snapshots_match(mut left: Cp330Snapshot, mut right: Cp330Snapshot) -> bool {
    let values_match = option_bits_match(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    );
    left.supply_mass_flow_rate_kg_per_s = None;
    right.supply_mass_flow_rate_kg_per_s = None;
    values_match && left == right
}

pub(super) fn cp334_snapshots_match(mut left: Cp334Snapshot, mut right: Cp334Snapshot) -> bool {
    let values_match = [
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
    }
    values_match && left == right
}

pub(super) fn cp344_snapshots_match(mut left: Cp344Snapshot, mut right: Cp344Snapshot) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
        option_bits_match(
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_temperature_c = None;
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
