use super::*;
use crate::psychrometrics::production_trace;

fn supplied_observation() -> ZoneAirInitializationObservation {
    let owner = ZoneAirInitializationState {
        mat: -0.0,
        xmat: [17.0, 18.0, 19.0, 1234.5],
        ..Default::default()
    };
    ZoneAirInitializationObservation {
        zone_id: ZoneId(7),
        zone_name: "SUPPLIED".into(),
        out_hum_rat: -0.0,
        caller_temperature_inputs: ZoneAirCallerTemperatureInputs {
            mat: -0.0,
            ztav: 31.0,
            xmat: [17.0, 18.0, 19.0],
            dsxmat: [20.0, 21.0, 22.0],
        },
        constructor: owner,
        after_bulk: owner,
        before_begin: owner,
        after_begin: owner,
        guard: ZoneAirEnvironmentInvocation {
            begin_environment: true,
            my_environment_before: true,
            eligible_before: true,
            my_environment_after: false,
            initializer_invocations: 3,
        },
        handoff: LegacyZoneAirInitializationProjection {
            mean_air_temperature_c: -0.0,
            zone_timestep_average_air_temperature_c: 31.0,
            air_humidity_ratio: 0.004,
            zone_timestep_average_air_humidity_ratio: -0.0,
            previous_mean_air_temperatures_c: [17.0, 18.0, 19.0],
            previous_system_mean_air_temperatures_c: [20.0, 21.0, 22.0],
            previous_air_humidity_ratios: [0.001, 0.002, -0.0],
            previous_system_air_humidity_ratios: [0.005, 0.006, 0.007],
            third_order_temp_independent_load_w: 41.0,
            third_order_temp_dependent_load_w_per_k: 42.0,
            air_power_cap_w_per_k: 43.0,
        },
    }
}

#[test]
fn copies_supplied_bits_fourth_slot_and_actual_scope_without_reconstructing_owner() {
    let (_, psych_trace) = production_trace::capture(true, || {
        production_trace::in_phase("supplied-snapshot", || {
            let _scope = production_trace::zone_step(5, 2, 4, 900.0);
            let (caller_line, trace) = capture(true, || {
                let caller_line = line!() + 1;
                record(supplied_observation());
                caller_line
            });
            let trace = trace.expect("active trace");
            assert_eq!(trace.total_initializer_count, 1);
            let row = &trace.initializer_observations[0];
            assert_eq!(row.caller.line(), caller_line);
            assert_eq!(row.phase, "supplied-snapshot");
            let cursor = row
                .context
                .expect("actual supplied scope")
                .zone_timestep
                .expect("cursor");
            assert_eq!((cursor.hour_index, cursor.zone_timestep), (5, 2));
            assert_eq!(row.observation.zone_id, ZoneId(7));
            assert_eq!(row.observation.out_hum_rat.to_bits(), (-0.0_f64).to_bits());
            assert_eq!(
                row.observation.after_begin.xmat[3].to_bits(),
                1234.5_f64.to_bits()
            );
            assert_eq!(
                row.observation.handoff.mean_air_temperature_c.to_bits(),
                (-0.0_f64).to_bits()
            );
            assert_eq!(row.observation.handoff.air_power_cap_w_per_k, 43.0);
            assert_eq!(row.observation.guard.initializer_invocations, 3);
        });
    });
    assert!(psych_trace.is_some());
    assert!(!is_active());
}

#[test]
#[allow(
    clippy::panic,
    reason = "Exercise observer restoration when its caller unwinds."
)]
fn disabled_nested_unwinding_and_zero_index_entry_filter_preserve_execution() {
    let model = ep_model::SimulationModel::from_typed(ep_model::TypedModel::default());
    let mut state = super::super::initialize_heat_balance_state(&model, 23.0).expect("empty state");
    let (_, outer) = capture(true, || {
        record(supplied_observation());
        record_timestep_entry(&state);
        state.timestep_index = 1;
        record_timestep_entry(&state);
        let (_, inner) = capture(true, || record(supplied_observation()));
        assert_eq!(inner.expect("inner").total_initializer_count, 1);
        let panic = std::panic::catch_unwind(|| {
            capture(true, || {
                record(supplied_observation());
                panic!("observer restoration");
            })
        });
        assert!(panic.is_err());
        record(supplied_observation());
    });
    let outer = outer.expect("outer");
    assert_eq!(outer.total_initializer_count, 2);
    assert_eq!(outer.total_timestep_entry_call_count, 2);
    assert_eq!(outer.zero_index_timestep_entry_count, 1);
    assert_eq!(outer.timestep_entry_observations.len(), 1);
    assert_eq!(outer.timestep_entry_observations[0].timestep_index, 0);
    assert_eq!(state.timestep_index, 1);
    let (value, disabled) = capture(false, || {
        record(supplied_observation());
        record_timestep_entry(&state);
        -0.0_f64
    });
    assert_eq!(value.to_bits(), (-0.0_f64).to_bits());
    assert!(disabled.is_none());
    assert!(!is_active());
}
