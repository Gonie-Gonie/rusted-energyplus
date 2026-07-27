use super::*;
use crate::ideal_loads::calc::cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand as Operand,
    purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle_summary,
};

fn run_case(
    limit: IdealLoadsLimit,
    capacity: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = matches!(
            limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        )
        .then_some(AutosizeOrNumber::Value(0.25));
        system.maximum_total_cooling_capacity_w = capacity.map(AutosizeOrNumber::Value);
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
    .expect("source-ordered CP322 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_assigns_the_exact_source_shaped_maximum() {
    for (limit, capacity, zeroed) in [
        (IdealLoadsLimit::NoLimit, None, false),
        (IdealLoadsLimit::LimitFlowRate, None, false),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), true),
        (IdealLoadsLimit::LimitCapacity, Some(900.0), false),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, Some(0.0), true),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            Some(900.0),
            false,
        ),
    ] {
        let (runtime, output) = run_case(limit, capacity, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_capacity_zero_flow_reset;
        let maximum = output.calculation_cooling_supply_mass_flow_maximum;
        assert!(cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(maximum));
        assert!(maximum.cooling_body_entered);
        assert_eq!(
            maximum
                .outdoor_air_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(0.0_f64.to_bits())
        );
        assert_option_bits_eq(
            maximum.supply_mass_flow_rate_for_cool_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        );
        assert_option_bits_eq(
            maximum.supply_mass_flow_rate_for_dehumidification_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        );
        assert_option_bits_eq(
            maximum.supply_mass_flow_rate_for_humidification_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        );
        assert_option_bits_eq(
            maximum.assigned_supply_mass_flow_rate_kg_per_s,
            maximum.maximum_supply_mass_flow_rate_kg_per_s,
        );
        assert_option_bits_eq(
            maximum.resulting_supply_mass_flow_rate_kg_per_s,
            maximum.maximum_supply_mass_flow_rate_kg_per_s,
        );
        if zeroed {
            assert_eq!(maximum.final_winner, Some(Operand::PositiveZeroFloor));
            assert_eq!(
                maximum
                    .maximum_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                Some(0.0_f64.to_bits())
            );
        }

        let state = purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP322 lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 1);
        assert_eq!(state.maximum_evaluation_count, 1);
        assert_eq!(state.supply_mass_flow_rate_assignment_count, 1);
    }
}

#[test]
fn scheduled_binding_skips_every_line_2155_site_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            independent_load_w,
            availability,
        );
        let maximum = output.calculation_cooling_supply_mass_flow_maximum;
        assert!(cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(maximum));
        assert_eq!(maximum.unit_off_skipped, unit_off);
        assert_eq!(maximum.non_cooling_skipped, non_cooling);
        assert!(!maximum.cooling_body_entered);
        assert!(!maximum.outdoor_air_mass_flow_rate_read);
        assert!(!maximum.supply_mass_flow_rate_for_cool_read);
        assert!(!maximum.positive_zero_vs_outdoor_air_comparison_evaluated);
        assert!(!maximum.supply_mass_flow_rate_assigned);
        assert!(maximum.resulting_supply_mass_flow_rate_kg_per_s.is_none());

        let state = purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP322 skip lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.maximum_evaluation_count, 0);
    }
}

fn assert_option_bits_eq(actual: Option<f64>, expected: Option<f64>) {
    assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
}
