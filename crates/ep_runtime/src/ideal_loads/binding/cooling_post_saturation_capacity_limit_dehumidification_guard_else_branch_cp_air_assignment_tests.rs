use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
};

#[test]
fn cp419_binding_contract_is_source_ordered_after_cp418() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2330",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2331",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        [
            "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
            "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
            "assign-local-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch",
        ],
    );
}

#[test]
fn cp419_binding_is_scheduled_immediately_after_cp418() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =")
        .expect("CP418 scheduled binding");
    let assignment = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =")
        .expect("CP419 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling")
        .expect("numerical coupling");
    assert!(predecessor < assignment && assignment < coupling);
}
