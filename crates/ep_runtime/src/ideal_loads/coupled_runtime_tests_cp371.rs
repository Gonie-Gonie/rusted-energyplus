use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{DehumidificationControlType, HumidificationControlType, SimulationModel, ZoneId};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_validation::{
    expected_snapshot, validate_lifecycle,
};

#[test]
fn cp371_lifecycle_matches_cp370_and_does_not_feed_numerical_result() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP371 validator fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };
    assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let predecessor_snapshot =
        output.calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
    let snapshot = output
        .calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;

    assert_eq!(state.transition_count, 1);
    assert_eq!(state.transition_count, predecessor_state.transition_count);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state.heating_on_read_count,
        predecessor_state.heating_on_read_count
    );
    assert_eq!(
        state.humidification_control_type_read_count,
        predecessor_state.humidification_control_type_read_count
    );
    assert_eq!(
        state.humidification_control_guard_false_fallthrough_count,
        predecessor_state.humidification_control_guard_false_fallthrough_count
    );
    for count in [
        state.dehumidification_control_type_first_read_count,
        state.dehumidification_control_type_humidistat_comparison_count,
        state.dehumidification_control_type_humidistat_match_count,
        state.dehumidification_control_type_second_read_count,
        state.dehumidification_control_type_none_comparison_count,
        state.dehumidification_control_type_none_match_count,
        state.dehumidification_control_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_count,
        state.source_site_execution_count,
    ] {
        assert_eq!(count, 0);
    }

    assert!(snapshot.predecessor_humidification_control_type_read);
    assert_eq!(
        snapshot.predecessor_humidification_control_type,
        Some(HumidificationControlType::None)
    );
    assert_eq!(
        snapshot.predecessor_humidification_control_type_humidistat,
        Some(false)
    );
    assert!(!snapshot.predecessor_humidification_control_body_entered);
    assert!(snapshot.predecessor_humidification_control_guard_false_fallthrough);
    assert!(!snapshot.dehumidification_control_type_first_read);
    assert_eq!(snapshot.first_dehumidification_control_type, None);
    assert_eq!(snapshot.dehumidification_control_type_humidistat, None);
    assert!(!snapshot.dehumidification_control_type_second_read);
    assert_eq!(snapshot.second_dehumidification_control_type, None);
    assert_eq!(snapshot.dehumidification_control_type_none, None);
    assert!(!snapshot.dehumidification_control_body_entered);
    assert!(!snapshot.dehumidification_control_guard_false_fallthrough);
    assert_eq!(snapshot, expected_snapshot(predecessor_snapshot));

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
        Some(numerical_result.to_bits())
    );
}

#[test]
fn cp371_direct_site_count_and_latest_corruption_are_rejected() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP371 corruption fixture");
    let Some((model, output, lifecycle, predecessor)) = fixture else {
        return;
    };

    for (field, mutate) in [
        (
            "direct_dehumidification_control_type_first_read_count",
            set_first_read_one as fn(&mut Lifecycle),
        ),
        (
            "direct_dehumidification_control_type_humidistat_comparison_count",
            set_humidistat_comparison_one as fn(&mut Lifecycle),
        ),
        (
            "direct_dehumidification_control_type_humidistat_match_count",
            set_humidistat_match_one as fn(&mut Lifecycle),
        ),
        (
            "direct_dehumidification_control_type_second_read_count",
            set_second_read_one as fn(&mut Lifecycle),
        ),
        (
            "direct_dehumidification_control_type_none_comparison_count",
            set_none_comparison_one as fn(&mut Lifecycle),
        ),
        (
            "direct_dehumidification_control_type_none_match_count",
            set_none_match_one as fn(&mut Lifecycle),
        ),
        (
            "direct_dehumidification_control_body_entry_count",
            set_body_one as fn(&mut Lifecycle),
        ),
        (
            "direct_dehumidification_control_guard_false_fallthrough_count",
            set_false_one as fn(&mut Lifecycle),
        ),
        (
            "direct_source_site_execution_count",
            set_source_one as fn(&mut Lifecycle),
        ),
    ] {
        let mut corrupt = lifecycle.clone();
        mutate(&mut corrupt);
        assert_eq!(
            validate(&model, &output, &corrupt, &predecessor),
            Err(Error::CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleInvariant {
                field,
                expected: 0,
                actual: 1,
            })
        );
    }

    let mut latest = lifecycle.clone();
    latest
        .state
        .latest
        .as_mut()
        .expect("CP371 latest")
        .dehumidification_control_type_first_read = true;
    assert_eq!(
        validate(&model, &output, &latest, &predecessor),
        Err(latest_violation())
    );

    let mut predecessor_latest = predecessor.clone();
    predecessor_latest
        .state
        .latest
        .as_mut()
        .expect("CP370 predecessor latest")
        .humidification_control_type_humidistat = Some(true);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor_latest),
        Err(latest_violation())
    );
}

fn latest_violation() -> Error {
    Error::CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn set_first_read_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_type_first_read_count = 1;
}
fn set_humidistat_comparison_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_type_humidistat_comparison_count = 1;
}
fn set_humidistat_match_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_type_humidistat_match_count = 1;
}
fn set_second_read_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_type_second_read_count = 1;
}
fn set_none_comparison_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_type_none_comparison_count = 1;
}
fn set_none_match_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_type_none_match_count = 1;
}
fn set_body_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.dehumidification_control_body_entry_count = 1;
}
fn set_false_one(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .dehumidification_control_guard_false_fallthrough_count = 1;
}
fn set_source_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.source_site_execution_count = 1;
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
    typed.ideal_loads_air_systems[0].humidification_control_type = HumidificationControlType::None;
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
        purchased_air_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .ok()?;
    let predecessor =
        purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle_summary(
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
    let binding = bind_direct_zone_purchased_air_model(model).expect("CP371 binding");
    validate_lifecycle(lifecycle, predecessor, 1, output, &binding)
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
