use super::*;
use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingInput, DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary as OwnerLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
    couple_model_bound_direct_zone_purchased_air,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary,
};
use crate::schedules::precompute_schedule_cache;
use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, SimulationModel,
    ZoneId,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_supply_humidity_ratio_pre_saturation_original_assignment_validation::{
    owner_snapshots_match_exact_bits, predecessor_snapshots_match_exact_bits,
    snapshot_matches_release, snapshots_match_exact_bits, validate_lifecycle,
};

#[test]
fn cp376_lifecycle_follows_cp375_copies_cp347_and_does_not_feed_numerical_result() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP376 validator fixture");
    let Some((model, output, lifecycle, predecessor, owner)) = fixture else {
        return;
    };
    assert!(validate(&model, &output, &lifecycle, &predecessor, &owner).is_ok());

    let state = &lifecycle.state;
    let snapshot =
        output.calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    let owner_snapshot = output
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.transition_count, predecessor.state.transition_count);
    assert_eq!(
        state.humidification_control_guard_false_fallthrough_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(state.cp347_none_case_owner_count, 1);
    for count in [
        state.cp375_maximum_assignment_owner_count,
        state.cp356_constant_shr_owner_count,
        state.cp362_humidistat_owner_count,
        state.cp365_constant_supply_humidity_ratio_owner_count,
    ] {
        assert_eq!(count, 0);
    }
    assert!(snapshot.humidification_control_guard_false_fallthrough);
    assert!(snapshot.cp347_none_case_owned_read);
    assert!(!snapshot.cp375_maximum_assignment_owned_read);
    let owner_bits = owner_snapshot
        .resulting_supply_humidity_ratio
        .map(f64::to_bits);
    assert!(owner_bits.is_some(), "CP347 source-last-writer bits");
    let Some(owner_bits) = owner_bits else {
        return;
    };
    for value in [
        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        snapshot.assigned_supply_humidity_ratio_original,
        snapshot.resulting_supply_humidity_ratio_original,
    ] {
        assert_eq!(value.map(f64::to_bits), Some(owner_bits));
    }

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
    );
}

#[test]
fn cp376_counter_owner_and_latest_corruption_are_rejected() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP376 corruption fixture");
    let Some((model, output, lifecycle, predecessor, owner)) = fixture else {
        return;
    };

    for mutate in [
        set_source_one as fn(&mut Lifecycle),
        set_read_zero,
        set_assignment_zero,
        set_cp347_owner_zero,
        set_cp375_owner_one,
    ] {
        let mut corrupt = lifecycle.clone();
        mutate(&mut corrupt);
        assert!(validate(&model, &output, &corrupt, &predecessor, &owner).is_err());
    }

    let mut latest = lifecycle.clone();
    let Some(latest_snapshot) = latest.state.latest.as_mut() else {
        return;
    };
    latest_snapshot.cp347_none_case_owned_read = false;
    assert_eq!(
        validate(&model, &output, &latest, &predecessor, &owner),
        Err(latest_violation()),
    );

    let mut owner_latest = owner.clone();
    let Some(owner_snapshot) = owner_latest.state.latest.as_mut() else {
        return;
    };
    let Some(value) = owner_snapshot.resulting_supply_humidity_ratio else {
        return;
    };
    owner_snapshot.resulting_supply_humidity_ratio = Some(f64::from_bits(value.to_bits() ^ 1));
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor, &owner_latest),
        Err(latest_violation()),
    );
}

#[test]
fn cp376_latest_output_snapshot_comparison_preserves_ieee_bits() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP376 IEEE comparison fixture");
    let Some((_model, _output, lifecycle, predecessor, owner)) = fixture else {
        return;
    };
    assert!(predecessor.state.latest.is_some(), "CP375 latest");
    let Some(mut cp375_left) = predecessor.state.latest else {
        return;
    };
    let mut cp375_right = cp375_left;
    cp375_left.maximum_supply_humidity_ratio = Some(0.0);
    cp375_right.maximum_supply_humidity_ratio = Some(-0.0);
    assert!(!predecessor_snapshots_match_exact_bits(
        cp375_left,
        cp375_right,
    ));

    assert!(owner.state.latest.is_some(), "CP347 latest");
    let Some(mut cp347_left) = owner.state.latest else {
        return;
    };
    let mut cp347_right = cp347_left;
    cp347_left.resulting_supply_humidity_ratio = Some(0.0);
    cp347_right.resulting_supply_humidity_ratio = Some(-0.0);
    assert!(!owner_snapshots_match_exact_bits(cp347_left, cp347_right));

    assert!(lifecycle.state.latest.is_some(), "CP376 latest");
    let Some(mut cp376_left) = lifecycle.state.latest else {
        return;
    };
    let mut cp376_right = cp376_left;
    cp376_left.assigned_supply_humidity_ratio_original = Some(0.0);
    cp376_right.assigned_supply_humidity_ratio_original = Some(-0.0);
    assert!(!snapshots_match_exact_bits(cp376_left, cp376_right));

    let nan = f64::from_bits(0x7ff8_0000_0000_0376);
    cp376_left.assigned_supply_humidity_ratio_original = Some(nan);
    cp376_right.assigned_supply_humidity_ratio_original = Some(nan);
    assert!(snapshots_match_exact_bits(cp376_left, cp376_right));
    cp376_right.assigned_supply_humidity_ratio_original = Some(f64::from_bits(nan.to_bits() ^ 1));
    assert!(!snapshots_match_exact_bits(cp376_left, cp376_right));
}

#[test]
fn cp376_release_match_rejects_coordinated_foreign_predecessor_identities() {
    let fixture = validator_fixture();
    assert!(fixture.is_some(), "CP376 identity fixture");
    let Some((model, output, _lifecycle, _predecessor, _owner)) = fixture else {
        return;
    };
    let binding = bind_direct_zone_purchased_air_model(&model);
    assert!(binding.is_ok(), "direct binding");
    let Ok(binding) = binding else {
        return;
    };
    assert!(snapshot_matches_release(&output, 1, &binding));

    let mut foreign = output;
    let predecessor = &mut foreign
        .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
    predecessor.system = IdealLoadsAirSystemId(1);
    predecessor.controlled_zone = ZoneId(1);
    predecessor.parent_call_ordinal = 2;
    let owner = &mut foreign
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    owner.system = IdealLoadsAirSystemId(1);
    owner.controlled_zone = ZoneId(1);
    owner.parent_call_ordinal = 2;

    assert!(!snapshot_matches_release(&foreign, 1, &binding));
}

fn set_source_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.source_site_execution_count = 1;
}

fn set_read_zero(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .purchased_air_supply_humidity_ratio_before_saturation_limit_read_count = 0;
}

fn set_assignment_zero(lifecycle: &mut Lifecycle) {
    lifecycle
        .state
        .local_original_supply_humidity_ratio_before_saturation_limit_assignment_count = 0;
}

fn set_cp347_owner_zero(lifecycle: &mut Lifecycle) {
    lifecycle.state.cp347_none_case_owner_count = 0;
}

fn set_cp375_owner_one(lifecycle: &mut Lifecycle) {
    lifecycle.state.cp375_maximum_assignment_owner_count = 1;
}

fn latest_violation() -> Error {
    Error::CalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn validator_fixture() -> Option<(
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
    OwnerLifecycle,
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
        purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .ok()?;
    let predecessor = purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .ok()?;
    let owner = purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .ok()?;
    Some((model, output, lifecycle, predecessor, owner))
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    owner: &OwnerLifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    validate_lifecycle(lifecycle, predecessor, owner, 1, output, &binding)
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
