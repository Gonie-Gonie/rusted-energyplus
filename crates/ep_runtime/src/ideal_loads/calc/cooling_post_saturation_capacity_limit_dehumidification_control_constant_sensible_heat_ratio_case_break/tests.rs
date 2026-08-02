//! CP393 source boundary, route, carrier, corruption, and release tests.

use super::*;

mod corruption;
pub(in crate::ideal_loads::calc) mod fixtures;
mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp393_boundary_and_single_break_site_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_BREAK_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2285",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2288",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_BREAK_SOURCE_ORDER,
        &["exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-via-break"],
    );
}
