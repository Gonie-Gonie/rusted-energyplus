//! CP397 source boundary, route, carrier, corruption, and release tests.

use super::*;

mod corruption;
pub(in crate::ideal_loads::calc) mod fixtures;
mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp397_boundary_and_single_none_case_entry_site_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2290",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2294",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_ENTRY_SOURCE_ORDER,
        &["enter-purchased-air-post-saturation-capacity-limit-dehumidification-control-none-case"],
    );
}
