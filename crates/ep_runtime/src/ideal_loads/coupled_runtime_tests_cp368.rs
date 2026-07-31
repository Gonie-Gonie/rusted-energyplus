use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_default_supply_humidity_ratio_case_break_lifecycle_summary,
    purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_default_supply_humidity_ratio_case_break_validation::{
    expected_snapshot, validate_lifecycle,
};

#[test]
fn cp368_lifecycle_matches_outputs_and_cp367_without_feeding_numerical_result() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP368 validator fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };
    assert!(
        validate(&model, &output, &lifecycle, &predecessor).is_ok(),
        "canonical CP368 coupled lifecycle"
    );
    let state = &lifecycle.state;
    let snapshot = output.calculation_cooling_default_supply_humidity_ratio_case_break;
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        0
    );
    assert_eq!(
        state.dehumidification_control_default_supply_humidity_ratio_case_break_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(snapshot.dehumidification_control_none_case_completed_skip);
    assert!(!snapshot.dehumidification_control_default_supply_humidity_ratio_case_exited_via_break);
    assert_eq!(
        snapshot,
        expected_snapshot(
            output.calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment
        )
    );

    let numerical_owner = output
        .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .assigned_supply_humidity_ratio;
    let numerical_result = output
        .coupling
        .purchased_air
        .supply_node_update
        .humidity_ratio;
    assert_eq!(
        numerical_owner.map(f64::to_bits),
        Some(numerical_result.to_bits()),
        "CP368 control-only evidence must not feed the coupling result"
    );
}

#[test]
fn cp368_route_source_latest_and_predecessor_corruption_are_rejected() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP368 corruption fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };

    let mut route_overflow = lifecycle.clone();
    route_overflow.state.unit_off_skip_count = usize::MAX;
    route_overflow.state.non_cooling_skip_count = 1;
    assert_invariant(
        validate(&model, &output, &route_overflow, &predecessor),
        "transition_partition_overflow",
        usize::MAX,
        1,
    );

    for (field, mutate) in [
        (
            "default_supply_humidity_ratio_case_break_count",
            set_break_count as fn(&mut Lifecycle),
        ),
        (
            "source_site_execution_count",
            set_source_count as fn(&mut Lifecycle),
        ),
    ] {
        let mut corrupt = lifecycle.clone();
        mutate(&mut corrupt);
        assert_invariant(
            validate(&model, &output, &corrupt, &predecessor),
            field,
            0,
            1,
        );
    }

    for (field, mutate) in [
        (
            "predecessor_mixed_air_humidity_ratio_read_count",
            set_predecessor_read_count as fn(&mut PredecessorLifecycle),
        ),
        (
            "predecessor_supply_humidity_ratio_assignment_count",
            set_predecessor_assignment_count as fn(&mut PredecessorLifecycle),
        ),
        (
            "predecessor_source_site_execution_count",
            set_predecessor_source_count as fn(&mut PredecessorLifecycle),
        ),
    ] {
        let mut corrupt = predecessor.clone();
        mutate(&mut corrupt);
        assert_invariant(validate(&model, &output, &lifecycle, &corrupt), field, 0, 1);
    }

    let mut predecessor_partition = predecessor.clone();
    predecessor_partition.state.unit_off_skip_count = 1;
    predecessor_partition.state.non_cooling_skip_count = 1;
    predecessor_partition
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    assert_invariant(
        validate(&model, &output, &lifecycle, &predecessor_partition),
        "predecessor_transition_partition",
        1,
        2,
    );

    let mut predecessor_overflow = predecessor.clone();
    predecessor_overflow.state.unit_off_skip_count = usize::MAX;
    predecessor_overflow.state.non_cooling_skip_count = 1;
    predecessor_overflow
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    assert_invariant(
        validate(&model, &output, &lifecycle, &predecessor_overflow),
        "transition_partition_overflow",
        usize::MAX,
        1,
    );

    let mut uncounted_latest_route = lifecycle.clone();
    uncounted_latest_route.state.unit_off_skip_count = 1;
    uncounted_latest_route
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    let mut matching_predecessor_counts = predecessor.clone();
    matching_predecessor_counts.state.unit_off_skip_count = 1;
    matching_predecessor_counts
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    assert_latest_rejected(
        &model,
        &output,
        &uncounted_latest_route,
        &matching_predecessor_counts,
    );

    let mut latest = lifecycle.clone();
    let latest_snapshot = latest.state.latest.as_mut();
    assert!(latest_snapshot.is_some(), "CP368 latest snapshot");
    let Some(latest_snapshot) = latest_snapshot else {
        return;
    };
    latest_snapshot.parent_call_ordinal = latest_snapshot.parent_call_ordinal.wrapping_add(1);
    assert_latest_rejected(&model, &output, &latest, &predecessor);

    let mut predecessor_corrupt = predecessor.clone();
    let predecessor_latest = predecessor_corrupt.state.latest.as_mut();
    assert!(predecessor_latest.is_some(), "CP367 predecessor snapshot");
    let Some(predecessor_latest) = predecessor_latest else {
        return;
    };
    predecessor_latest.first_excluded_source = "corrupt";
    assert_latest_rejected(&model, &output, &lifecycle, &predecessor_corrupt);
}

fn set_break_count(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_default_supply_humidity_ratio_case_break_count = 1;
}

fn set_source_count(lifecycle: &mut Lifecycle) {
    lifecycle.state.source_site_execution_count = 1;
}

fn set_predecessor_read_count(lifecycle: &mut PredecessorLifecycle) {
    lifecycle.state.mixed_air_humidity_ratio_read_count = 1;
}

fn set_predecessor_assignment_count(lifecycle: &mut PredecessorLifecycle) {
    lifecycle.state.supply_humidity_ratio_assignment_count = 1;
}

fn set_predecessor_source_count(lifecycle: &mut PredecessorLifecycle) {
    lifecycle.state.source_site_execution_count = 1;
}

#[test]
fn cp368_expected_snapshot_maps_all_typed_selected_routes_to_default_break_skip() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP368 route-mapping fixture");
    let Some((_, output, _, _)) = fixture else {
        return;
    };
    let base = output.calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment;

    for control in [
        DehumidificationControlType::None,
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let mut predecessor = base;
        predecessor.predecessor_dehumidification_control_type = Some(control);
        predecessor.predecessor_dehumidification_control_none_case_completed_skip =
            control == DehumidificationControlType::None;
        predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =
            control == DehumidificationControlType::ConstantSensibleHeatRatio;
        predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip =
            control == DehumidificationControlType::Humidistat;
        predecessor
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break =
            control == DehumidificationControlType::ConstantSupplyHumidityRatio;
        predecessor.dehumidification_control_none_case_completed_skip =
            control == DehumidificationControlType::None;
        predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =
            control == DehumidificationControlType::ConstantSensibleHeatRatio;
        predecessor.dehumidification_control_humidistat_case_completed_skip =
            control == DehumidificationControlType::Humidistat;
        predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip =
            control == DehumidificationControlType::ConstantSupplyHumidityRatio;
        predecessor
            .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed =
            false;

        let snapshot = expected_snapshot(predecessor);
        assert_eq!(
            (
                snapshot.dehumidification_control_none_case_completed_skip,
                snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
                snapshot.dehumidification_control_humidistat_case_completed_skip,
                snapshot
                    .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
            ),
            (
                control == DehumidificationControlType::None,
                control == DehumidificationControlType::ConstantSensibleHeatRatio,
                control == DehumidificationControlType::Humidistat,
                control == DehumidificationControlType::ConstantSupplyHumidityRatio,
            )
        );
        assert!(
            !snapshot.dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
        );
    }
}

fn validator_fixture() -> Option<(
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
)> {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 20.0;
    typed.schedules[2].hourly_value = 24.0;
    typed.schedules[3].hourly_value = 1.0;
    typed.ideal_loads_air_systems[0].dehumidification_control_type =
        DehumidificationControlType::None;
    typed.ideal_loads_air_systems[0].minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1).ok()?;
    let binding = bind_direct_zone_purchased_air_model(&model).ok()?;
    let mut zone_state = cooling_zone_state(binding.nominal_system_timestep_seconds);
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
    .ok()?;
    let lifecycle =
        purchased_air_calc_cooling_default_supply_humidity_ratio_case_break_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .ok()?;
    let predecessor =
        purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .ok()?;
    Some((model, output, lifecycle, predecessor))
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model);
    assert!(binding.is_ok(), "CP368 binding");
    let Ok(binding) = binding else {
        return Ok(());
    };
    validate_lifecycle(lifecycle, predecessor, 1, output, &binding)
}

fn assert_invariant(
    result: Result<(), Error>,
    field: &'static str,
    expected: usize,
    actual: usize,
) {
    assert_eq!(
        result,
        Err(
            Error::CalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleInvariant {
                field,
                expected,
                actual,
            }
        )
    );
}

fn assert_latest_rejected(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) {
    assert_invariant(
        validate(model, output, lifecycle, predecessor),
        "latest_release_snapshot_ready",
        1,
        0,
    );
}

fn cooling_zone_state(system_timestep_seconds: f64) -> ZoneHeatBalanceState {
    ZoneHeatBalanceState {
        zone_id: ZoneId(0),
        zone_name: "ZONE ONE".to_string(),
        mean_air_temperature_c: 22.0,
        zone_timestep_average_air_temperature_c: 22.0,
        previous_mean_air_temperatures_c: [0.0; 3],
        previous_system_mean_air_temperatures_c: [0.0; 3],
        previous_system_timestep_count: 1,
        air_humidity_ratio: 0.008,
        zone_timestep_average_air_humidity_ratio: 0.008,
        previous_air_humidity_ratios: [0.008; 3],
        previous_system_air_humidity_ratios: [0.008; 3],
        use_zone_timestep_history: false,
        shorten_timestep_sys: false,
        prior_timestep_seconds: system_timestep_seconds,
        volume_m3: 100.0,
        air_heat_capacity_j_per_k: 0.0,
        convective_internal_gain_w: 0.0,
        opaque_surface_conductance_w_per_k: 100.0,
        opaque_surface_heat_gain_w: 0.0,
        opaque_surface_outside_conduction_w: 0.0,
        sum_ha_w_per_k: 100.0,
        sum_hat_surf_w: 3_000.0,
        sum_hat_ref_w: 0.0,
        sum_mcp_w_per_k: 0.0,
        sum_mcp_t_w: 0.0,
        sum_sys_mcp_w_per_k: 7.0,
        sum_sys_mcp_t_w: 11.0,
        system_dependent_zone_loads_lagged_w: 0.0,
        zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
        system_timestep_average_surface_convection_report_w: None,
        system_timestep_average_air_storage_report_w: None,
    }
}
