//! CP425 coupled-runtime accounting and no-feed contracts.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentLifecycleSummary as Lifecycle,
        PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRuntimeState,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_supply_mass_flow_positive_guard_else_branch_entry_lifecycle_summary,
        purchased_air_calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_lifecycle_summary,
    },
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType,
    IdealLoadsAirSystemId, IdealLoadsLimit, SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp425_conceptual_contract_has_59_outcomes_58_inactive_one_assignment_and_two_sites() {
    assert_eq!(
        (59 - 1, 1, 2, 19, 40, 36, 41, 1, 42, 56),
        (58, 1, 2, 19, 40, 36, 41, 1, 42, 56)
    );
}

#[test]
fn cp425_snapshot_schema_is_exactly_275_99_2_1_and_current_binding_is_130() {
    let source =
        include_str!("calc/cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment.rs");
    let snapshot = source
        .split_once(
            "pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP425"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP425 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 275);
    let names = fields
        .iter()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(names.len(), 275);
    assert_eq!(snapshot.matches("Option<f64>").count(), 99);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 101, 1);
    assert_eq!(
        include_str!("binding/scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        130
    );
}

#[test]
fn cp425_new_state_has_two_zeroed_lossless_route_partitions() {
    let state =
        PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    for values in [
        state.predecessor_route_counts,
        state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp425_binding_and_pipeline_keep_numerical_dto_unchanged() {
    let binding = include_str!("binding.rs");
    let pipeline = include_str!("../../../ep_run/src/pipeline.rs");
    let marker = "cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment";
    assert!(binding.contains(marker));
    assert!(pipeline.contains(&format!("{marker}_lifecycle")));
    for forbidden in [
        "coupling.zone_sensible_cooling_rate_w = calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment",
        "DirectZonePurchasedAirCouplingInput { calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment",
    ] {
        assert!(!binding.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp425_validation_has_no_numerical_coupling_dto_feed() {
    let (model, output, lifecycle, predecessor) = validator_fixture();
    let mut enthalpy = output;
    enthalpy
        .coupling
        .purchased_air
        .calculation
        .supply_enthalpy_j_per_kg = different(
        enthalpy
            .coupling
            .purchased_air
            .calculation
            .supply_enthalpy_j_per_kg,
    );
    assert!(validate(&model, &enthalpy, &lifecycle, &predecessor).is_ok());

    let mut sensible = output;
    sensible
        .coupling
        .purchased_air
        .calculation
        .zone_sensible_cooling_rate_w = different(
        sensible
            .coupling
            .purchased_air
            .calculation
            .zone_sensible_cooling_rate_w,
    );
    assert!(validate(&model, &sensible, &lifecycle, &predecessor).is_ok());

    let mut report = output;
    report
        .coupling
        .purchased_air
        .report
        .zone_total_cooling_rate_w = different(
        report
            .coupling
            .purchased_air
            .report
            .zone_total_cooling_rate_w,
    );
    assert!(validate(&model, &report, &lifecycle, &predecessor).is_ok());

    let mut feedback = output;
    feedback.coupling.feedback.sum_sys_mcp_t_w =
        different(feedback.coupling.feedback.sum_sys_mcp_t_w);
    assert!(validate(&model, &feedback, &lifecycle, &predecessor).is_ok());
}

#[test]
fn cp425_integration_roots_stay_within_historical_caps() {
    let state = include_str!("init/state.rs");
    let witnesses = include_str!("init/state/witnesses.rs");
    let calc = include_str!("calc.rs");
    assert!(state.lines().filter(|line| !line.trim().is_empty()).count() <= 380);
    assert!(witnesses.lines().count() <= 276);
    assert!(calc.lines().count() <= 99);
}

fn validator_fixture() -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
) {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 20.0;
    typed.schedules[2].hourly_value = 24.0;
    typed.schedules[3].hourly_value = 1.0;
    let system = &mut typed.ideal_loads_air_systems[0];
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_cooling_air_flow_rate_m3_per_s = None;
    system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(500.0));
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::None;
    system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1).expect("CP425 schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP425 direct binding");
    let mut zone_state = super::coupled_runtime_tests_cp389::cooling_zone_state(
        binding.nominal_system_timestep_seconds,
        0.020,
    );
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
    .expect("CP425 coupling");
    let system = output.initialization.system;
    let lifecycle =
        purchased_air_calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP425 lifecycle");
    let predecessor =
        purchased_air_calc_cooling_supply_mass_flow_positive_guard_else_branch_entry_lifecycle_summary(
            &runtime, system,
        )
        .expect("CP424 lifecycle");
    (model, output, lifecycle, predecessor)
}

fn validate(
    model: &SimulationModel,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), Error> {
    let binding = bind_direct_zone_purchased_air_model(model).map_err(|_| latest_violation())?;
    assert!(snapshot_matches_release(output, 1, &binding));
    validate_lifecycle(lifecycle, predecessor, 1, output, &binding)
}

fn latest_violation() -> Error {
    Error::CalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
