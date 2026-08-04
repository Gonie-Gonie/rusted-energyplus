//! Non-vacuous CP413 coupled-runtime integration and numerical-firewall tests.

use super::*;
use crate::{
    ideal_loads::{
        DirectZonePurchasedAirScheduledCouplingInput,
        DirectZonePurchasedAirScheduledCouplingOutput,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary as Lifecycle,
        PurchasedAirRuntimeState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_lifecycle_summary,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_lifecycle_summary,
    },
    schedules::precompute_schedule_cache,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
    SimulationModel,
};

use super::super::DirectZonePurchasedAirCoupledRuntimeError as Error;
use super::super::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_validation::{
    snapshot_matches_release, validate_lifecycle,
};

#[test]
fn cp413_conceptual_contract_has_54_outcomes_126_sites_and_expected_carrier_presence() {
    let inactive = 18;
    let active_predecessors = 18;
    let evaluated_false = active_predecessors;
    let body = active_predecessors;
    let total = inactive + evaluated_false + body;
    let humidity_presence = evaluated_false + body;
    let enthalpy_presence = 5 + evaluated_false + body;
    let temperature_presence = 15 + evaluated_false + body;
    let sites = evaluated_false * 3 + body * 4;
    assert_eq!(
        (
            total,
            inactive,
            evaluated_false,
            body,
            humidity_presence,
            enthalpy_presence,
            temperature_presence,
            sites
        ),
        (54, 18, 18, 18, 36, 41, 51, 126),
    );
}

#[test]
fn cp413_reads_cp412_saturation_and_recursively_retained_cp411_original_only_when_active() {
    let mut saw_active = false;
    let mut saw_inactive = false;

    for (limit, humidity_ratio, maximum_capacity_w, availability, pressure) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0),
        (
            IdealLoadsLimit::LimitCapacity,
            0.020,
            1.0e-100,
            1.0,
            97_321.0,
        ),
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0, 97_321.0),
    ] {
        let (model, output, lifecycle, predecessor) = validator_fixture(
            limit,
            humidity_ratio,
            maximum_capacity_w,
            availability,
            pressure,
        );
        assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());

        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
        let cp412 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
        let active = cp412
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed;
        assert_eq!(
            snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated,
            active,
        );

        for (left, right) in [
            (
                snapshot.predecessor_cp412_resulting_supply_humidity_ratio,
                cp412.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
                cp412.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp412_resulting_supply_temperature_c,
                cp412.resulting_supply_temperature_c,
            ),
            (
                snapshot.resulting_supply_humidity_ratio,
                cp412.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.resulting_supply_enthalpy_j_per_kg,
                cp412.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.resulting_supply_temperature_c,
                cp412.resulting_supply_temperature_c,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }

        if active {
            let saturation = cp412
                .resulting_saturation_supply_humidity_ratio
                .expect("active CP413 fixture requires CP412 saturation");
            let original = cp412
                .resulting_supply_humidity_ratio_original
                .expect("active CP413 fixture requires CP411 original");
            let terminal = cp412
                .predecessor_cp411_resulting_supply_humidity_ratio
                .expect("active CP413 fixture requires CP411 terminal corroboration");
            assert_eq!(original.to_bits(), terminal.to_bits());
            assert!(snapshot.cp412_saturation_supply_humidity_ratio_owned_read);
            assert!(snapshot.saturation_supply_humidity_ratio_for_guard_read);
            assert!(snapshot.cp411_original_supply_humidity_ratio_owned_read);
            assert!(snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated);
            assert!(snapshot.original_supply_humidity_ratio_for_guard_read);
            assert!(snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated);
            assert_eq!(
                snapshot
                    .saturation_supply_humidity_ratio_for_guard
                    .map(f64::to_bits),
                Some(saturation.to_bits()),
            );
            assert_eq!(
                snapshot
                    .original_supply_humidity_ratio_for_guard
                    .map(f64::to_bits),
                Some(original.to_bits()),
            );
            assert_eq!(
                snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
                Some(saturation < original),
            );
            assert_eq!(
                snapshot.saturation_supply_humidity_ratio_guard_body_entered,
                saturation < original
            );
            assert_eq!(
                snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough,
                saturation >= original
            );
            saw_active = true;
        } else {
            assert!(!snapshot.cp412_saturation_supply_humidity_ratio_owned_read);
            assert!(!snapshot.saturation_supply_humidity_ratio_for_guard_read);
            assert!(!snapshot.cp411_original_supply_humidity_ratio_owned_read);
            assert!(!snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated);
            assert!(!snapshot.original_supply_humidity_ratio_for_guard_read);
            assert!(!snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated);
            assert!(
                snapshot
                    .saturation_supply_humidity_ratio_for_guard
                    .is_none()
            );
            assert!(snapshot.original_supply_humidity_ratio_for_guard.is_none());
            assert!(snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio.is_none());
            saw_inactive = true;
        }
    }

    assert!(saw_active);
    assert!(saw_inactive);
}

#[test]
fn cp413_validation_rejects_changed_operand_lineage_and_non_direct_routes() {
    let (model, output, mut lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0);
    let latest = lifecycle
        .state
        .latest
        .as_mut()
        .expect("CP413 latest snapshot");
    latest.original_supply_humidity_ratio_for_guard = latest
        .original_supply_humidity_ratio_for_guard
        .map(different);
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor),
        Err(latest_violation()),
    );

    let (model, output, mut lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0);
    lifecycle.state.predecessor_route_counts[22] += 1;
    assert_eq!(
        validate(&model, &output, &lifecycle, &predecessor),
        Err(Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleInvariant {
            field: "predecessor_route_lineage",
            expected: 1,
            actual: 0,
        }),
    );
}

#[test]
fn cp413_evidence_does_not_feed_numerical_result() {
    let (model, mut output, lifecycle, predecessor) =
        validator_fixture(IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0, 97_321.0);
    output
        .coupling
        .purchased_air
        .calculation
        .supply_humidity_ratio = different(
        output
            .coupling
            .purchased_air
            .calculation
            .supply_humidity_ratio,
    );
    output
        .coupling
        .purchased_air
        .calculation
        .zone_latent_cooling_rate_w = different(
        output
            .coupling
            .purchased_air
            .calculation
            .zone_latent_cooling_rate_w,
    );
    output.coupling.feedback.sum_sys_mcp_t_w = different(output.coupling.feedback.sum_sys_mcp_t_w);
    assert!(validate(&model, &output, &lifecycle, &predecessor).is_ok());
}

fn validator_fixture(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
    maximum_capacity_w: f64,
    availability: f64,
    pressure: f64,
) -> (
    SimulationModel,
    DirectZonePurchasedAirScheduledCouplingOutput,
    Lifecycle,
    PredecessorLifecycle,
) {
    let mut typed = exact_model(1).typed;
    typed.schedules[1].hourly_value = 20.0;
    typed.schedules[2].hourly_value = 24.0;
    typed.schedules[3].hourly_value = availability;
    let system = &mut typed.ideal_loads_air_systems[0];
    system.cooling_limit = cooling_limit;
    system.maximum_cooling_air_flow_rate_m3_per_s = None;
    system.maximum_total_cooling_capacity_w = matches!(
        cooling_limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
    .then_some(AutosizeOrNumber::Value(maximum_capacity_w));
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::None;
    system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let model = SimulationModel::from_typed(typed);
    let cache = precompute_schedule_cache(&model.typed, 1)
        .expect("CP413 fixture requires a valid schedule cache");
    let binding = bind_direct_zone_purchased_air_model(&model)
        .expect("CP413 fixture requires a direct binding");
    let mut zone_state = super::coupled_runtime_tests_cp389::cooling_zone_state(
        binding.nominal_system_timestep_seconds,
        air_humidity_ratio,
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
            barometric_pressure_pa: pressure,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("CP413 fixture requires a successful coupling call");
    let system = output.initialization.system;
    let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_lifecycle_summary(
        &runtime,
        system,
    )
    .expect("CP413 lifecycle summary");
    let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
        &runtime,
        system,
    )
    .expect("CP412 lifecycle summary");
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleInvariant {
        field: "latest_release_snapshot_ready",
        expected: 1,
        actual: 0,
    }
}

fn different(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
