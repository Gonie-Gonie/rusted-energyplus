//! CP391 source boundary, route, IEEE, corruption, and release tests.

use ep_model::DehumidificationControlType as D;

use super::*;

mod corruption;
pub(in crate::ideal_loads::calc) mod fixtures;
mod ieee;
mod release_corruption;
mod routes;

#[test]
fn cp391_boundaries_and_physical_five_site_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2283",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2284",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
        &[
            "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit-maximum",
            "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-overdrying-limit-enthalpy",
            "evaluate-psy-h-fn-tdb-w-at-minimum-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
            "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-overdrying-limit",
            "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit",
        ],
    );
}
