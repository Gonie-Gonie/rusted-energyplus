use ep_model::{
    DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit,
    ZoneId,
};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput as ActiveInput,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::{
    completed_cp370_case_for_cp372_test,
    completed_cp370_case_with_capacity_limit_for_later_test,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Cp379Snapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
};

mod overflow;
mod release_corruption;
mod routes;

#[test]
fn cp380_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2264",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2266",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        &[
            "read-cooling-limit-for-post-saturation-capacity-comparison",
            "compare-cooling-limit-equal-to-capacity-for-post-saturation-capacity-guard",
            "read-cooling-limit-for-post-saturation-flow-rate-and-capacity-comparison-after-first-false",
            "compare-cooling-limit-equal-to-flow-rate-and-capacity-for-post-saturation-capacity-guard",
            "enter-post-saturation-capacity-limit-body-if-compound-condition-satisfied",
        ],
    );
}

#[test]
fn cp380_lineage_validation_uses_only_bounded_committed_seals() {
    let source = include_str!("../release/prefix_validation.rs");
    let predecessor_validation = source
        .split("pub(super) fn direct_predecessor_is_retained_and_complete")
        .nth(1)
        .expect("CP379 predecessor validator")
        .split("pub(super) fn direct_selector_lineage_is_retained_and_complete")
        .next()
        .expect("bounded CP379 validator body");

    assert!(predecessor_validation.contains(
        "cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent"
    ));
    assert!(!predecessor_validation.contains(
        "completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent"
    ));

    let selector_validation = source
        .split("pub(super) fn direct_selector_lineage_is_retained_and_complete")
        .nth(1)
        .expect("CP337 selector validator")
        .split("fn selector_lineage_matches_predecessor")
        .next()
        .expect("bounded CP337 validator body");
    assert!(selector_validation.contains(
        "cooling_positive_supply_capacity_limit_guard_committed_latest_snapshot_is_consistent"
    ));
    assert!(!selector_validation.contains(
        "completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent"
    ));
}

pub(in crate::ideal_loads::calc) fn active_input(limit: IdealLoadsLimit) -> Option<ActiveInput> {
    Some(ActiveInput {
        cooling_limit: limit,
        cp337_same_call_selector_lineage_corroborated: true,
    })
}

pub(in crate::ideal_loads::calc) fn predecessor_for_route(
    route: usize,
    ordinal: usize,
) -> Cp379Snapshot {
    let active = route >= 3;
    let selector = match route {
        3 | 4 | 6 => Some(DehumidificationControlType::None),
        5 => Some(DehumidificationControlType::Humidistat),
        7 => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        _ => None,
    };
    Cp379Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(1),
        parent_call_ordinal: ordinal,
        controlled_zone: ZoneId(2),
        unit_off_skipped: route == 0,
        non_cooling_skipped: route == 1,
        positive_guard_false_fallthrough_skipped: route == 2,
        heating_availability_guard_false_fallthrough: route == 3,
        humidification_control_guard_false_fallthrough: route == 4,
        dehumidification_control_humidistat_maximum_assignment_executed: route == 5,
        dehumidification_control_none_maximum_assignment_executed: route == 6,
        dehumidification_control_guard_false_fallthrough: route == 7,
        predecessor_dehumidification_control_type: selector,
        predecessor_supply_humidity_ratio_saturation_limit_assignment_performed: active,
        predecessor_resulting_supply_humidity_ratio: active.then_some(0.008),
        cp377_supply_temperature_owned_read: active,
        cp334_supply_temperature_mixed_air_limit_owned_read: active,
        cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: false,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: active,
        purchased_air_supply_temperature_for_post_saturation_enthalpy_read: active,
        supply_temperature_c: active.then_some(14.0),
        purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read: active,
        supply_humidity_ratio: active.then_some(0.008),
        psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated: active,
        psychrometric_supply_enthalpy_j_per_kg: active.then_some(34_300.0),
        local_supply_enthalpy_after_saturation_limit_assignment_performed: active,
        assigned_supply_enthalpy_j_per_kg: active.then_some(34_300.0),
        resulting_supply_enthalpy_j_per_kg: active.then_some(34_300.0),
    }
}

pub(super) fn completed_cp379_case()
-> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp379Snapshot) {
    let (mut runtime, system, cp370) =
        completed_cp370_case_for_cp372_test().expect("CP370 fixture");
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .expect("CP371 direct");
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp371,
    )
    .expect("CP372 direct");
    let cp373 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
        &mut runtime,
        &system,
        cp372,
    )
    .expect("CP373 direct");
    let cp374 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
        &mut runtime,
        &system,
        cp373,
    )
    .expect("CP374 direct");
    let cp375 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
        &mut runtime,
        &system,
        cp374,
    )
    .expect("CP375 direct");
    let cp376 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .expect("CP376 direct");
    let cp377 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(
        &mut runtime,
        &system,
        cp376,
        101_325.0,
    )
    .expect("CP377 direct");
    let cp378 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .expect("CP378 direct");
    let cp379 = advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
        &mut runtime,
        &system,
        cp378,
    )
    .expect("CP379 direct");
    (runtime, system, cp379)
}

pub(in crate::ideal_loads::calc) fn completed_cp380_case(
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
)> {
    let (mut runtime, system, cp370) =
        completed_cp370_case_with_capacity_limit_for_later_test(capacity_limit)?;
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .ok()?;
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp371,
    )
    .ok()?;
    let cp373 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
        &mut runtime,
        &system,
        cp372,
    )
    .ok()?;
    let cp374 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
        &mut runtime,
        &system,
        cp373,
    )
    .ok()?;
    let cp375 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
        &mut runtime,
        &system,
        cp374,
    )
    .ok()?;
    let cp376 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .ok()?;
    let cp377 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(
        &mut runtime,
        &system,
        cp376,
        101_325.0,
    )
    .ok()?;
    let cp378 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .ok()?;
    let cp379 = advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
        &mut runtime,
        &system,
        cp378,
    )
    .ok()?;
    let cp380 =
        super::super::advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
            &mut runtime,
            &system,
            cp379,
        )
        .ok()?;
    Some((runtime, system, cp380))
}
