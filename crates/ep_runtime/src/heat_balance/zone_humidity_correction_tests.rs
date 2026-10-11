use super::*;

// Literal input-only equal-working-no-inlet owner; these are no Native answers.
fn fixture() -> (ZoneAirInitializationState, ZoneHumidityCorrectionContext) {
    let state = ZoneAirInitializationState {
        zt: 24.0,
        mat: 18.0,
        ztav: 21.0,
        air_hum_rat: 0.008,
        air_hum_rat_avg: 0.0075,
        air_hum_rat_temp: 0.006,
        w1: 0.005,
        wmx: 0.004,
        wm2: 0.003,
        w_time_minus_p: 0.002,
        air_rel_hum: 37.0,
        w_prev_zone_ts: [0.019, 0.018, 0.017, 0.016],
        dsw_prev_zone_ts: [0.013, 0.012, 0.011, 0.010],
        w_prev_zone_ts_temp: [0.008, 0.008, 0.008, 0.009],
        ..ZoneAirInitializationState::default()
    };
    let context = ZoneHumidityCorrectionContext {
        zone_id: 1,
        space_id: 0,
        zone_name: "DECLARED-DIRECT-MEMBER-ZONE".into(),
        solution_algorithm: 0,
        room_air_model: 1,
        do_space_heat_balance: false,
        do_latent_sizing: false,
        is_controlled: false,
        is_return_plenum: false,
        is_supply_plenum: false,
        leakage_parallel_piu_count: 0,
        afn_control_type: 0,
        afn_multizone_always_simulated: false,
        afn_fan_activated: false,
        afn_distribution_simulated: false,
        duct_loss_simulated: false,
        hybrid_model: false,
        multiplier: 1,
        list_multiplier: 1,
        volume: 100.0,
        moisture_capacitance_multiplier: 1.0,
        outdoor_pressure: 101325.0,
        outdoor_humidity_ratio: 0.006,
        time_step_sys_seconds: 900.0,
        time_step_sys: 0.25,
        time_step_zone: 0.25,
        inlet_node_ids: vec![],
        system_zone_node_number: 0,
    };
    (state, context)
}

fn node(id: i32, mass: f64, humidity: f64) -> ZoneHumidityNode {
    ZoneHumidityNode {
        node_id: id,
        name: format!("DECLARED-NODE-{id}"),
        mass_flow_rate: mass,
        temperature: 19.0,
        humidity_ratio: humidity,
        enthalpy: 12345.0,
    }
}

#[test]
fn retains_full_four_slot_current_average_and_temperature_owners() {
    let (mut state, context) = fixture();
    let before = state;
    let mut psych = EnergyPlusPsychrometricsState::default();
    let returned = correct_selected_zone_humidity_ratio(
        &mut state,
        &ZoneHumiditySourceTerms::default(),
        &context,
        &mut [],
        &mut psych,
    )
    .unwrap();
    assert_eq!(returned.system_node_written, None);
    assert_ne!(
        state.air_hum_rat_temp.to_bits(),
        before.air_hum_rat_temp.to_bits()
    );
    state.air_hum_rat_temp = before.air_hum_rat_temp;
    assert_eq!(state, before);
}

#[test]
fn passed_psychrometric_instance_and_original_working_owner_persist_across_calls() {
    let (mut state, context) = fixture();
    let mut psych = EnergyPlusPsychrometricsState::default();
    let first = correct_selected_zone_humidity_ratio(
        &mut state,
        &ZoneHumiditySourceTerms::default(),
        &context,
        &mut [],
        &mut psych,
    )
    .unwrap();
    let first_temp = state.air_hum_rat_temp.to_bits();
    let cache = psych.final_caches();
    assert!(!cache.psat.is_empty());
    let second = correct_selected_zone_humidity_ratio(
        &mut state,
        &ZoneHumiditySourceTerms::default(),
        &context,
        &mut [],
        &mut psych,
    )
    .unwrap();
    assert_eq!(state.air_hum_rat_temp.to_bits(), first_temp);
    assert_eq!(
        second.saturation_call.state_before,
        first.saturation_call.state_after
    );
    assert_eq!(psych.final_caches(), cache);
    assert_eq!(state.w_prev_zone_ts_temp[3].to_bits(), 0.009_f64.to_bits());
}

#[test]
fn feedback_write_is_independent_of_controlled_inlet_branch() {
    let (mut state, mut context) = fixture();
    context.system_zone_node_number = 1;
    let mut nodes = vec![node(1, 0.0, 0.009)];
    nodes[0].enthalpy = -321.0; // Actual uncontrolled-with-feedback-node input.
    let before = nodes[0].clone();
    let call = correct_selected_zone_humidity_ratio(
        &mut state,
        &ZoneHumiditySourceTerms::default(),
        &context,
        &mut nodes,
        &mut EnergyPlusPsychrometricsState::default(),
    )
    .unwrap();
    assert_eq!(call.system_node_written, Some(1));
    assert_eq!(
        nodes[0].humidity_ratio.to_bits(),
        state.air_hum_rat_temp.to_bits()
    );
    assert_ne!(nodes[0].enthalpy.to_bits(), before.enthalpy.to_bits());
    nodes[0].humidity_ratio = before.humidity_ratio;
    nodes[0].enthalpy = before.enthalpy;
    assert_eq!(nodes[0], before);
}

#[test]
fn selected_stored_solution_has_no_helper_humidity_floor() {
    let (mut state, context) = fixture();
    state.w_prev_zone_ts_temp = [0.000001; 4]; // Actual subfloor-working input.
    correct_selected_zone_humidity_ratio(
        &mut state,
        &ZoneHumiditySourceTerms::default(),
        &context,
        &mut [],
        &mut EnergyPlusPsychrometricsState::default(),
    )
    .unwrap();
    assert!(state.air_hum_rat_temp > 0.0 && state.air_hum_rat_temp < 0.00001);
}

#[test]
fn ordered_lower_then_saturation_guards_remain_distinct() {
    let (mut low, context) = fixture();
    low.w_prev_zone_ts_temp = [0.001, 0.010, 0.001, 0.007];
    correct_selected_zone_humidity_ratio(
        &mut low,
        &ZoneHumiditySourceTerms::default(),
        &context,
        &mut [],
        &mut EnergyPlusPsychrometricsState::default(),
    )
    .unwrap();
    assert_eq!(low.air_hum_rat_temp.to_bits(), 0.0_f64.to_bits());
    let (mut high, cold) = fixture();
    high.zt = 5.0;
    high.w_prev_zone_ts_temp = [0.040, 0.040, 0.040, 0.015];
    let call = correct_selected_zone_humidity_ratio(
        &mut high,
        &ZoneHumiditySourceTerms::default(),
        &cold,
        &mut [],
        &mut EnergyPlusPsychrometricsState::default(),
    )
    .unwrap();
    assert_eq!(
        high.air_hum_rat_temp.to_bits(),
        call.saturation_call.result.to_bits()
    );
}

#[test]
fn declared_inlet_order_and_nonfeedback_node_owners_are_retained() {
    let (mut state, mut context) = fixture();
    context.is_controlled = true;
    context.inlet_node_ids = vec![3, 1, 2];
    context.system_zone_node_number = 4;
    let mut nodes = vec![
        node(1, 0.125, 0.003),
        node(2, 0.5, 0.016),
        node(3, 0.00000001, 0.007),
        node(4, 0.0, 0.009),
    ];
    let before = nodes.clone();
    let context_before = context.clone();
    correct_selected_zone_humidity_ratio(
        &mut state,
        &ZoneHumiditySourceTerms::default(),
        &context,
        &mut nodes,
        &mut EnergyPlusPsychrometricsState::default(),
    )
    .unwrap();
    assert_eq!(&nodes[..3], &before[..3]);
    assert_eq!(context, context_before);
    assert_eq!(
        nodes[3].humidity_ratio.to_bits(),
        state.air_hum_rat_temp.to_bits()
    );
}

#[test]
fn rejects_scope_and_int_product_before_state_node_or_cache_mutation() {
    let (mut state, mut context) = fixture();
    let before = state;
    let mut psych = EnergyPlusPsychrometricsState::default();
    context.multiplier = i32::MAX;
    context.list_multiplier = 2;
    assert!(matches!(
        correct_selected_zone_humidity_ratio(
            &mut state,
            &ZoneHumiditySourceTerms::default(),
            &context,
            &mut [],
            &mut psych
        ),
        Err(ZoneHumidityCorrectionUnavailable::Scope(_))
    ));
    assert_eq!(state, before);
    assert!(psych.final_caches().psat.is_empty());
    context.multiplier = 1;
    context.list_multiplier = 1;
    let sources = ZoneHumiditySourceTerms {
        latent_gain: 1.0,
        ..ZoneHumiditySourceTerms::default()
    };
    assert!(matches!(
        correct_selected_zone_humidity_ratio(&mut state, &sources, &context, &mut [], &mut psych),
        Err(ZoneHumidityCorrectionUnavailable::Scope(_))
    ));
    assert_eq!(state, before);
    assert!(psych.final_caches().psat.is_empty());
}

#[test]
fn negative_density_backend_is_explicitly_unavailable_before_member_writes() {
    let (mut state, context) = fixture();
    state.zt = -300.0; // Source-only unsupported-route probe, not a Native fixture.
    let before = state;
    let mut psych = EnergyPlusPsychrometricsState::default();
    assert!(matches!(
        correct_selected_zone_humidity_ratio(
            &mut state,
            &ZoneHumiditySourceTerms::default(),
            &context,
            &mut [],
            &mut psych
        ),
        Err(ZoneHumidityCorrectionUnavailable::NegativeDensityDiagnostic)
    ));
    assert_eq!(state, before);
    assert!(psych.final_caches().psat.is_empty());
}
