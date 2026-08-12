use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
};

#[test]
fn cp420_binding_contract_maps_line_2331_in_eight_sites() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2331",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        8,
    );
    assert_ne!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    );
}

#[test]
fn cp420_binding_is_between_cp419_and_unchanged_numerical_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =")
        .expect("CP419 scheduled binding");
    let assignment = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =")
        .expect("CP420 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling")
        .expect("unchanged numerical coupling");
    assert!(predecessor < assignment && assignment < coupling);
}
