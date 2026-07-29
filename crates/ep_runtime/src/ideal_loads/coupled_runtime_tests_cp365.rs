use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle_summary,
    purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_constant_supply_humidity_ratio_assignment_validation::{
    expected_snapshot, snapshots_match_bit_exact, validate_lifecycle,
};

#[test]
fn cp365_lifecycle_matches_cp364_as_a_complete_null_direct_skip() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP365 validator fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };
    assert!(
        validate(&model, &output, &lifecycle, &predecessor).is_ok(),
        "canonical CP365 coupled lifecycle"
    );
    let state = &lifecycle.state;
    let snapshot = output.calculation_cooling_constant_supply_humidity_ratio_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_assignment_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert_eq!(
        state.minimum_cooling_supply_air_humidity_ratio_read_count,
        0
    );
    assert_eq!(state.supply_humidity_ratio_assignment_count, 0);
    assert!(snapshot.dehumidification_control_none_case_completed_skip);
    assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_read);
    assert!(snapshot.minimum_cooling_supply_air_humidity_ratio.is_none());
    assert!(!snapshot.supply_humidity_ratio_assigned);
    assert!(snapshot.assigned_supply_humidity_ratio.is_none());
    assert!(snapshot.resulting_supply_humidity_ratio.is_none());

    let expected =
        expected_snapshot(output.calculation_cooling_constant_supply_humidity_ratio_case_entry);
    assert!(snapshots_match_bit_exact(&snapshot, &expected));

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
        "CP365 lifecycle evidence must not feed the actual coupling result"
    );
}

#[test]
fn cp365_route_source_latest_and_predecessor_corruption_are_rejected() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP365 corruption fixture");
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

    let mut active = lifecycle.clone();
    let mut active_predecessor = predecessor.clone();
    active
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    active_predecessor
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    active
        .state
        .dehumidification_control_constant_supply_humidity_ratio_assignment_count = 1;
    active.state.source_site_execution_count = 2;
    active
        .state
        .minimum_cooling_supply_air_humidity_ratio_read_count = 1;
    active.state.supply_humidity_ratio_assignment_count = 1;
    active_predecessor
        .state
        .dehumidification_control_constant_supply_humidity_ratio_case_entry_count = 1;
    assert_invariant(
        validate(&model, &output, &active, &active_predecessor),
        "direct_constant_supply_humidity_ratio_assignment_count",
        0,
        1,
    );

    let mut source = lifecycle.clone();
    source.state.source_site_execution_count = 1;
    assert_invariant(
        validate(&model, &output, &source, &predecessor),
        "source_site_execution_count",
        0,
        1,
    );

    let mut read = lifecycle.clone();
    read.state
        .minimum_cooling_supply_air_humidity_ratio_read_count = 1;
    assert_invariant(
        validate(&model, &output, &read, &predecessor),
        "minimum_cooling_supply_air_humidity_ratio_read_count",
        0,
        1,
    );

    let mut latest = lifecycle.clone();
    let latest_snapshot = latest.state.latest.as_mut();
    assert!(latest_snapshot.is_some(), "CP365 latest snapshot");
    let Some(latest_snapshot) = latest_snapshot else {
        return;
    };
    latest_snapshot.parent_call_ordinal = latest_snapshot.parent_call_ordinal.wrapping_add(1);
    assert_invariant(
        validate(&model, &output, &latest, &predecessor),
        "latest_release_snapshot_ready",
        1,
        0,
    );

    let mut predecessor_identity = predecessor.clone();
    let predecessor_latest = predecessor_identity.state.latest.as_mut();
    assert!(predecessor_latest.is_some(), "CP364 predecessor snapshot");
    let Some(predecessor_latest) = predecessor_latest else {
        return;
    };
    predecessor_latest.controlled_zone = ZoneId(999);
    assert_invariant(
        validate(&model, &output, &lifecycle, &predecessor_identity),
        "latest_release_snapshot_ready",
        1,
        0,
    );
}

#[test]
fn cp365_snapshot_matching_rejects_signed_zero_numeric_corruption() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP365 exact-bit fixture");
    let Some((_, output, _, _)) = fixture else {
        return;
    };
    let mut negative_zero =
        expected_snapshot(output.calculation_cooling_constant_supply_humidity_ratio_case_entry);
    negative_zero.minimum_cooling_supply_air_humidity_ratio_read = true;
    negative_zero.minimum_cooling_supply_air_humidity_ratio = Some(-0.0);
    negative_zero.supply_humidity_ratio_assigned = true;
    negative_zero.assigned_supply_humidity_ratio = Some(-0.0);
    negative_zero.resulting_supply_humidity_ratio = Some(-0.0);
    let mut positive_zero = negative_zero;
    positive_zero.minimum_cooling_supply_air_humidity_ratio = Some(0.0);
    positive_zero.assigned_supply_humidity_ratio = Some(0.0);
    positive_zero.resulting_supply_humidity_ratio = Some(0.0);
    assert_eq!(negative_zero, positive_zero);
    assert!(!snapshots_match_bit_exact(&negative_zero, &positive_zero));
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
        purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .ok()?;
    let predecessor =
        purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle_summary(
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
    assert!(binding.is_ok(), "CP365 binding");
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
            Error::CalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            }
        )
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
