//! CP390 source boundary, route, IEEE, and corruption tests.

use ep_model::DehumidificationControlType as D;

use super::*;

mod corruption;
mod fixtures;
mod ieee;
mod release_corruption;
mod routes;

#[test]
fn cp390_boundaries_and_physical_four_site_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2281",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2283",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-minimum",
            "read-purchased-air-mixed-air-temperature-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-purchased-air-supply-temperature",
        ],
    );
}
