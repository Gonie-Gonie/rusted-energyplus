use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_capacity_w: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.cooling_sensible_heat_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    let mut runtime = PurchasedAirRuntimeState::default();
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("source-ordered CP352 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_completes_cp352_direct_none_route_without_operand_or_numeric_work() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let (runtime, output) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        let predecessor = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment;
        let snapshot = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment;

        assert!(predecessor.dehumidification_control_none_case_completed_skip);
        assert!(
            cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_type,
            Some(DehumidificationControlType::None)
        );
        assert!(snapshot.predecessor_dehumidification_control_none_case_completed_skip);
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert_complete_null(snapshot);

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP352 lifecycle")
            .state;
        assert_counter_shape(&state, false, false, false, true);
    }
}

#[test]
fn scheduled_binding_preserves_cp352_u_n_and_p_routes_as_complete_null() {
    for (
        cooling_limit,
        maximum_capacity_w,
        independent_load_w,
        availability,
        unit_off,
        non_cooling,
        positive_false,
    ) in [
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            0.0,
            true,
            false,
            false,
        ),
        (IdealLoadsLimit::NoLimit, None, 0.0, 1.0, false, true, false),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(-0.0),
            3_000.0,
            1.0,
            false,
            false,
            true,
        ),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            maximum_capacity_w,
            independent_load_w,
            availability,
        );
        let snapshot = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment;
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            positive_false
        );
        assert!(!snapshot.dehumidification_control_none_case_completed_skip);
        assert_complete_null(snapshot);

        let state =
            purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP352 lifecycle")
            .state;
        assert_counter_shape(&state, unit_off, non_cooling, positive_false, false);
    }
}

fn assert_complete_null(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed
    );
    assert!(!snapshot.mixed_air_enthalpy_read);
    assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.cooling_total_output_read);
    assert!(snapshot.cooling_total_output_w.is_none());
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.specific_cooling_output_calculated);
    assert!(snapshot.specific_cooling_output_j_per_kg.is_none());
    assert!(!snapshot.supply_enthalpy_calculated);
    assert!(snapshot.calculated_supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.supply_enthalpy_assigned);
    assert!(snapshot.assigned_supply_enthalpy_j_per_kg.is_none());
    assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
}

fn assert_counter_shape(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState,
    unit_off: bool,
    non_cooling: bool,
    positive_false: bool,
    none_skip: bool,
) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
    assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
    assert_eq!(
        state.positive_guard_false_fallthrough_skip_count,
        usize::from(positive_false)
    );
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        usize::from(none_skip)
    );
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_count,
        0
    );
    assert_eq!(
        state.dehumidification_control_humidistat_case_selected_skip_count,
        0
    );
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert_eq!(state.mixed_air_enthalpy_read_count, 0);
    assert_eq!(state.cooling_total_output_read_count, 0);
    assert_eq!(state.supply_mass_flow_rate_read_count, 0);
    assert_eq!(state.specific_cooling_output_calculation_count, 0);
    assert_eq!(state.supply_enthalpy_calculation_count, 0);
    assert_eq!(state.supply_enthalpy_assignment_write_count, 0);
}
