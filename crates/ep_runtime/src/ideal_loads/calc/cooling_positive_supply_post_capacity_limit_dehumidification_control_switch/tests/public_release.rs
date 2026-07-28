use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary,
};
use ep_model::DehumidificationControlType;

pub(super) fn completed_cp345_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) {
    let (mut runtime, system, predecessor) =
        completed_cp344_case(cooling_demand_w, overall_availability, capacity_limit);
    let assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP345");
    (runtime, system, assignment)
}

#[test]
fn public_g_f_l_routes_read_only_the_typed_none_selector_and_dispatch() {
    for (demand, capacity) in [(-1_000.0, false), (-1_000.0, true), (-100_000.0, true)] {
        let (mut runtime, system, predecessor) = completed_cp345_case(demand, 1.0, capacity);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP346");
        assert!(snapshot.dehumidification_control_type_read);
        assert_eq!(
            snapshot.dehumidification_control_type,
            Some(DehumidificationControlType::None)
        );
        assert!(snapshot.dehumidification_control_switch_dispatched);
        let state = &runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
        assert_eq!(state.dehumidification_control_switch_count, 1);
        assert_eq!(state.dehumidification_control_none_case_selection_count, 1);
        assert_eq!(state.source_site_execution_count, 2);
    }
}

#[test]
fn public_u_n_p_routes_skip_cp346_while_p_retains_its_earlier_cp319_read() {
    for (demand, availability, cp319_read, p) in [
        (-1_000.0, 0.0, false, false),
        (1.0, 1.0, false, false),
        (-1.0e-40, 1.0, true, true),
    ] {
        let (mut runtime, system, predecessor) = completed_cp345_case(demand, availability, true);
        let cp319 = runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_dehumidification_flow
            .latest
            .expect("CP319");
        assert_eq!(cp319.dehumidification_control_type_read, cp319_read);
        if p {
            assert_eq!(
                cp319.dehumidification_control_type,
                Some(DehumidificationControlType::None)
            );
            assert!(predecessor.positive_guard_false_fallthrough_skipped);
        }
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP346 skip");
        assert!(!snapshot.dehumidification_control_type_read);
        assert!(snapshot.dehumidification_control_type.is_none());
        assert!(!snapshot.dehumidification_control_switch_dispatched);
    }
}

#[test]
fn lifecycle_summary_reports_completed_cp346_and_duplicate_is_rejected() {
    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP346");
    let summary =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary(
            &runtime,
            system.id,
        )
        .expect("summary");
    assert_eq!(summary.state.latest, Some(snapshot));
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
}
