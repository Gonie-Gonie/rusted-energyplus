use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Snapshot,
    PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary as PredecessorLifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    cooling_constant_supply_humidity_ratio_case_entry_snapshot_links_to_predecessor,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle_summary,
    purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_constant_supply_humidity_ratio_case_entry_validation::{
    expected_snapshot, validate_lifecycle,
};

#[test]
fn cp364_lifecycle_matches_outputs_and_cp363() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP364 validator fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };
    assert!(
        validate(&model, &output, &lifecycle, &predecessor).is_ok(),
        "canonical CP364 coupled lifecycle"
    );
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(
        lifecycle
            .state
            .dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        lifecycle
            .state
            .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        0
    );
    assert_eq!(
        lifecycle
            .state
            .dehumidification_control_humidistat_case_completed_skip_count,
        0
    );
    assert_eq!(lifecycle.state.source_site_execution_count, 0);
    assert!(
        output
            .calculation_cooling_constant_supply_humidity_ratio_case_entry
            .dehumidification_control_none_case_completed_skip
    );
    assert!(
        !output
            .calculation_cooling_constant_supply_humidity_ratio_case_entry
            .dehumidification_control_humidistat_case_completed_skip
    );
}

#[test]
fn cp364_route_source_latest_and_predecessor_corruption_are_rejected() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP364 corruption fixture must succeed");
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

    let mut q = lifecycle.clone();
    let mut q_predecessor = predecessor.clone();
    q.state
        .dehumidification_control_none_case_completed_skip_count = 0;
    q_predecessor
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    q.state
        .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count = 1;
    q_predecessor
        .state
        .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count = 1;
    assert_invariant(
        validate(&model, &output, &q, &q_predecessor),
        "direct_constant_sensible_heat_ratio_case_completed_skip_count",
        0,
        1,
    );

    let mut h = lifecycle.clone();
    let mut h_predecessor = predecessor.clone();
    h.state
        .dehumidification_control_none_case_completed_skip_count = 0;
    h_predecessor
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    h.state
        .dehumidification_control_humidistat_case_completed_skip_count = 1;
    h_predecessor
        .state
        .dehumidification_control_humidistat_case_break_count = 1;
    assert_invariant(
        validate(&model, &output, &h, &h_predecessor),
        "direct_humidistat_case_completed_skip_count",
        0,
        1,
    );

    let mut csh = lifecycle.clone();
    let mut csh_predecessor = predecessor.clone();
    csh.state
        .dehumidification_control_none_case_completed_skip_count = 0;
    csh_predecessor
        .state
        .dehumidification_control_none_case_completed_skip_count = 0;
    csh.state
        .dehumidification_control_constant_supply_humidity_ratio_case_entry_count = 1;
    csh.state.source_site_execution_count = 1;
    csh_predecessor
        .state
        .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count = 1;
    assert_invariant(
        validate(&model, &output, &csh, &csh_predecessor),
        "direct_constant_supply_humidity_ratio_case_entry_count",
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

    let mut replay = lifecycle.clone();
    replay.state.transition_count = 2;
    replay
        .state
        .dehumidification_control_none_case_completed_skip_count = 2;
    assert_invariant(
        validate(&model, &output, &replay, &predecessor),
        "transition_count",
        1,
        2,
    );

    let mut latest_route = lifecycle.clone();
    let latest = latest_route.state.latest.as_mut();
    assert!(latest.is_some(), "CP364 latest route snapshot must exist");
    let Some(latest) = latest else {
        return;
    };
    forge_latest_route_to_unit_off(latest);
    assert_latest_rejected(&model, &output, &latest_route, &predecessor);

    let mut latest_ordinal = lifecycle.clone();
    let latest = latest_ordinal.state.latest.as_mut();
    assert!(latest.is_some(), "CP364 latest ordinal snapshot must exist");
    let Some(latest) = latest else {
        return;
    };
    latest.parent_call_ordinal = latest.parent_call_ordinal.wrapping_add(1);
    assert_latest_rejected(&model, &output, &latest_ordinal, &predecessor);

    let mut predecessor_link = lifecycle.clone();
    let latest = predecessor_link.state.latest.as_mut();
    assert!(
        latest.is_some(),
        "CP364 predecessor-link snapshot must exist"
    );
    let Some(latest) = latest else {
        return;
    };
    latest.controlled_zone = ZoneId(999);
    assert_latest_rejected(&model, &output, &predecessor_link, &predecessor);
}

#[test]
fn cp364_expected_snapshot_maps_valid_q_h_and_csh_predecessor_routes() {
    let fixture = validator_fixture();
    assert!(
        fixture.is_some(),
        "CP364 route-mapping fixture must succeed"
    );
    let Some((_, output, _, _)) = fixture else {
        return;
    };
    let base = output.calculation_cooling_humidistat_case_break;

    for (selector, expected) in [
        (
            DehumidificationControlType::ConstantSensibleHeatRatio,
            (false, true, false, false),
        ),
        (
            DehumidificationControlType::Humidistat,
            (false, false, true, false),
        ),
        (
            DehumidificationControlType::ConstantSupplyHumidityRatio,
            (false, false, false, true),
        ),
    ] {
        let mut predecessor = base;
        predecessor.predecessor_dehumidification_control_type = Some(selector);
        predecessor.predecessor_dehumidification_control_none_case_completed_skip = false;
        predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =
            expected.1;
        predecessor
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed =
            expected.2;
        predecessor
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =
            expected.3;
        predecessor.dehumidification_control_none_case_completed_skip = expected.0;
        predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =
            expected.1;
        predecessor.dehumidification_control_humidistat_case_exited_via_break = expected.2;
        predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =
            expected.3;

        assert!(
            predecessor.unit_body_entered
                && predecessor.predecessor_cooling_body_entered
                && predecessor.predecessor_no_outdoor_air_fallback_entered
                && predecessor.predecessor_positive_supply_mass_flow_body_entered
                && !predecessor.unit_off_skipped
                && !predecessor.non_cooling_skipped
                && !predecessor.positive_guard_false_fallthrough_skipped
        );
        assert_eq!(
            (
                predecessor.predecessor_dehumidification_control_none_case_completed_skip,
                predecessor
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
                predecessor
                    .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed,
                predecessor
                    .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
            ),
            expected
        );
        let snapshot = expected_snapshot(predecessor);
        assert_eq!(
            snapshot.predecessor_dehumidification_control_type,
            Some(selector)
        );
        assert!(
            cooling_constant_supply_humidity_ratio_case_entry_snapshot_links_to_predecessor(
                snapshot,
                predecessor,
            )
        );
        assert_eq!(
            (
                snapshot.dehumidification_control_none_case_completed_skip,
                snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
                snapshot.dehumidification_control_humidistat_case_completed_skip,
                snapshot.dehumidification_control_constant_supply_humidity_ratio_case_entered,
            ),
            expected
        );
        assert_eq!(
            (
                snapshot
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
                snapshot
                    .predecessor_dehumidification_control_humidistat_case_exited_via_break,
                snapshot
                    .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
            ),
            (expected.1, expected.2, expected.3)
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
        purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .ok()?;
    let predecessor = purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary(
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
    assert!(binding.is_ok(), "CP364 binding");
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
            Error::CalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleInvariant {
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

fn forge_latest_route_to_unit_off(snapshot: &mut Snapshot) {
    snapshot.unit_body_entered = false;
    snapshot.predecessor_cooling_body_entered = false;
    snapshot.predecessor_no_outdoor_air_fallback_entered = false;
    snapshot.predecessor_positive_supply_mass_flow_body_entered = false;
    snapshot.unit_off_skipped = true;
    snapshot.non_cooling_skipped = false;
    snapshot.positive_guard_false_fallthrough_skipped = false;
    snapshot.predecessor_dehumidification_control_type = None;
    snapshot.predecessor_dehumidification_control_none_case_completed_skip = false;
    snapshot
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =
        false;
    snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break = false;
    snapshot
        .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =
        false;
    snapshot.dehumidification_control_none_case_completed_skip = false;
    snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip = false;
    snapshot.dehumidification_control_humidistat_case_completed_skip = false;
    snapshot.dehumidification_control_constant_supply_humidity_ratio_case_entered = false;
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
