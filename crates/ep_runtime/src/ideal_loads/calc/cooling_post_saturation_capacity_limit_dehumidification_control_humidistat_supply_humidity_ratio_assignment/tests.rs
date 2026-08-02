//! CP395 source boundary, route, carrier, arithmetic, corruption, and release tests.

use super::*;

mod corruption;
pub(in crate::ideal_loads::calc) mod fixtures;
mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp395_boundary_and_four_assignment_sites_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2288",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2289",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-humidistat-humidity-ratio-inversion",
            "read-local-supply-enthalpy-for-humidistat-humidity-ratio-inversion",
            "evaluate-psy-w-fn-tdb-h-for-humidistat-capacity-limit",
            "assign-purchased-air-supply-humidity-ratio-for-humidistat-capacity-limit",
        ],
    );
}
