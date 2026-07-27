use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit};

use super::{
    PURCHASED_AIR_SUPPLY_TEMPERATURE_UNIT_C, PurchasedAirRuntimeState,
    PurchasedAirSupplyTemperatureDiagnostic, PurchasedAirSupplyTemperatureDiagnosticKind,
    PurchasedAirSupplyTemperatureDiagnosticRegistry, PurchasedAirSupplyTemperatureGateTrace,
    PurchasedAirSupplyTemperatureInitialMessageApi, init_purchased_air_runtime,
    lifecycle_tests::{SYSTEM, context, finite_flow_system, single_manager_plan, topology},
    purchased_air_init_lifecycle_summary,
    state::PurchasedAirUnitRuntimeState,
    supply_temperature_diagnostic::{
        PurchasedAirSupplyTemperatureDiagnosticContext, advance_supply_temperature_diagnostics,
    },
};

#[test]
fn recurring_registry_preserves_first_detail_asymmetry_and_cooling_first_order() {
    let mut system = finite_flow_system();
    system.cooling_limit = IdealLoadsLimit::NoLimit;
    system.heating_limit = IdealLoadsLimit::NoLimit;
    system.minimum_cooling_supply_air_temperature_c = 22.001;
    system.maximum_heating_supply_air_temperature_c = 19.999;
    let plan = single_manager_plan();
    let mut state = PurchasedAirRuntimeState::default();

    let first = init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(true))
        .expect("first active diagnostic call");
    assert_eq!(
        first.transition.cooling_supply_temperature_gate,
        PurchasedAirSupplyTemperatureGateTrace {
            outer_condition_met: true,
            overall_availability_read_site_visited: true,
            mode_availability_read_site_visited: true,
            active: true,
        }
    );
    assert_eq!(
        first.transition.heating_supply_temperature_gate,
        first.transition.cooling_supply_temperature_gate
    );
    assert!(first.transition.cooling_supply_temperature_first_diagnostic);
    assert!(first.transition.heating_supply_temperature_first_diagnostic);
    assert_eq!(first.transition.supply_temperature_diagnostics_emitted, 2);
    assert_eq!(
        first
            .transition
            .supply_temperature_characterized_severe_error_count_increment,
        3
    );

    let first_summary =
        purchased_air_init_lifecycle_summary(&state, SYSTEM).expect("first diagnostic summary");
    assert_eq!(
        first_summary.supply_temperature_registered_recurring_diagnostic_count,
        2
    );
    assert_eq!(first_summary.supply_temperature_diagnostic_event_count, 2);
    assert_eq!(
        first_summary.supply_temperature_characterized_severe_error_count_increment,
        3
    );
    assert_eq!(first_summary.cooling_supply_temperature_error_index, 1);
    assert_eq!(first_summary.heating_supply_temperature_error_index, 2);
    assert_eq!(
        first_summary.cooling_supply_temperature_first_diagnostic_count,
        1
    );
    assert_eq!(
        first_summary.heating_supply_temperature_first_diagnostic_count,
        1
    );
    assert_eq!(
        first_summary.supply_temperature_diagnostics,
        vec![
            PurchasedAirSupplyTemperatureDiagnostic {
                system: SYSTEM,
                registry_registration_ordinal: 1,
                first_init_call_ordinal: 1,
                last_init_call_ordinal: 1,
                source_order_ordinal: 1,
                kind: PurchasedAirSupplyTemperatureDiagnosticKind::CoolingMinimumAboveSetpoint,
                recurring_index: 1,
                first_detailed_diagnostic_count: 1,
                initial_message_api:
                    PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereError,
                first_detail_primary_message_count: 1,
                first_detail_continue_message_count: 5,
                first_detail_timestamp_count: 1,
                recurring_severe_call_count: 1,
                characterized_severe_error_count_increment: 2,
                latest_supply_temperature_c: 22.001,
                latest_thermostat_setpoint_c: 22.0,
                recurring_minimum_c: 22.001,
                recurring_maximum_c: 22.001,
                temperature_unit: PURCHASED_AIR_SUPPLY_TEMPERATURE_UNIT_C,
            },
            PurchasedAirSupplyTemperatureDiagnostic {
                system: SYSTEM,
                registry_registration_ordinal: 2,
                first_init_call_ordinal: 1,
                last_init_call_ordinal: 1,
                source_order_ordinal: 2,
                kind: PurchasedAirSupplyTemperatureDiagnosticKind::HeatingMaximumBelowSetpoint,
                recurring_index: 2,
                first_detailed_diagnostic_count: 1,
                initial_message_api:
                    PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereMessage,
                first_detail_primary_message_count: 1,
                first_detail_continue_message_count: 5,
                first_detail_timestamp_count: 1,
                recurring_severe_call_count: 1,
                characterized_severe_error_count_increment: 1,
                latest_supply_temperature_c: 19.999,
                latest_thermostat_setpoint_c: 20.0,
                recurring_minimum_c: 19.999,
                recurring_maximum_c: 19.999,
                temperature_unit: PURCHASED_AIR_SUPPLY_TEMPERATURE_UNIT_C,
            },
        ]
    );

    let mut off = context(true);
    off.overall_availability = 0.0;
    let inactive = init_purchased_air_runtime(&mut state, &plan, &topology(), &system, off)
        .expect("inactive diagnostic gap");
    assert!(
        inactive
            .transition
            .cooling_supply_temperature_gate
            .outer_condition_met
    );
    assert!(
        inactive
            .transition
            .cooling_supply_temperature_gate
            .overall_availability_read_site_visited
    );
    assert!(
        inactive
            .transition
            .cooling_supply_temperature_gate
            .mode_availability_read_site_visited
    );
    assert!(!inactive.transition.cooling_supply_temperature_gate.active);
    assert_eq!(
        inactive.transition.supply_temperature_diagnostics_emitted,
        0
    );

    system.minimum_cooling_supply_air_temperature_c = 23.0;
    system.maximum_heating_supply_air_temperature_c = 19.0;
    let recurring =
        init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(true))
            .expect("recurring diagnostic call");
    assert!(
        !recurring
            .transition
            .cooling_supply_temperature_first_diagnostic
    );
    assert!(
        !recurring
            .transition
            .heating_supply_temperature_first_diagnostic
    );
    assert_eq!(
        recurring.transition.supply_temperature_diagnostics_emitted,
        2
    );
    assert_eq!(
        recurring
            .transition
            .supply_temperature_characterized_severe_error_count_increment,
        2
    );

    let repeated =
        purchased_air_init_lifecycle_summary(&state, SYSTEM).expect("recurring diagnostic summary");
    assert_eq!(
        repeated.supply_temperature_registered_recurring_diagnostic_count,
        2
    );
    assert_eq!(repeated.supply_temperature_diagnostic_event_count, 4);
    assert_eq!(
        repeated.supply_temperature_characterized_severe_error_count_increment,
        5
    );
    assert_eq!(
        repeated.cooling_supply_temperature_first_diagnostic_count,
        1
    );
    assert_eq!(
        repeated.heating_supply_temperature_first_diagnostic_count,
        1
    );
    assert_eq!(repeated.cooling_supply_temperature_warning_count, 2);
    assert_eq!(repeated.heating_supply_temperature_warning_count, 2);
    assert_eq!(repeated.supply_temperature_diagnostics.len(), 2);
    let cooling_recurring = repeated.supply_temperature_diagnostics[0];
    assert_eq!(cooling_recurring.registry_registration_ordinal, 1);
    assert_eq!(cooling_recurring.first_init_call_ordinal, 1);
    assert_eq!(cooling_recurring.last_init_call_ordinal, 3);
    assert_eq!(cooling_recurring.source_order_ordinal, 1);
    assert_eq!(cooling_recurring.recurring_index, 1);
    assert_eq!(cooling_recurring.first_detailed_diagnostic_count, 1);
    assert_eq!(
        cooling_recurring.initial_message_api,
        PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereError
    );
    assert_eq!(cooling_recurring.first_detail_primary_message_count, 1);
    assert_eq!(cooling_recurring.first_detail_continue_message_count, 5);
    assert_eq!(cooling_recurring.first_detail_timestamp_count, 1);
    assert_eq!(cooling_recurring.recurring_severe_call_count, 2);
    assert_eq!(
        cooling_recurring.characterized_severe_error_count_increment,
        3
    );
    assert_eq!(cooling_recurring.latest_supply_temperature_c, 23.0);
    assert_eq!(cooling_recurring.recurring_minimum_c, 22.001);
    assert_eq!(cooling_recurring.recurring_maximum_c, 23.0);
    let heating_recurring = repeated.supply_temperature_diagnostics[1];
    assert_eq!(heating_recurring.registry_registration_ordinal, 2);
    assert_eq!(heating_recurring.source_order_ordinal, 2);
    assert_eq!(heating_recurring.recurring_index, 2);
    assert_eq!(heating_recurring.recurring_severe_call_count, 2);
    assert_eq!(heating_recurring.recurring_minimum_c, 19.0);
    assert_eq!(heating_recurring.recurring_maximum_c, 19.999);
}

#[test]
fn recurring_registry_allocates_globally_and_reuses_each_units_indices() {
    let second_system_id = IdealLoadsAirSystemId(1);
    let mut first_system = finite_flow_system();
    first_system.cooling_limit = IdealLoadsLimit::NoLimit;
    first_system.heating_limit = IdealLoadsLimit::NoLimit;
    first_system.minimum_cooling_supply_air_temperature_c = 23.0;
    first_system.maximum_heating_supply_air_temperature_c = 19.0;
    let mut second_system = first_system.clone();
    second_system.id = second_system_id;

    let mut registry = PurchasedAirSupplyTemperatureDiagnosticRegistry::default();
    let mut first_unit = PurchasedAirUnitRuntimeState::new(SYSTEM, None);
    first_unit.init_call_count = 1;
    let mut second_unit = PurchasedAirUnitRuntimeState::new(second_system_id, None);
    second_unit.init_call_count = 1;
    let diagnostic_context = PurchasedAirSupplyTemperatureDiagnosticContext {
        cooling_setpoint_c: 22.0,
        heating_setpoint_c: 20.0,
        overall_availability: 1.0,
        cooling_availability: 1.0,
        heating_availability: 1.0,
    };

    advance_supply_temperature_diagnostics(
        &mut registry,
        &mut first_unit,
        &first_system,
        diagnostic_context,
    );
    advance_supply_temperature_diagnostics(
        &mut registry,
        &mut second_unit,
        &second_system,
        diagnostic_context,
    );

    assert_eq!(first_unit.cooling_supply_temperature_error_index, 1);
    assert_eq!(first_unit.heating_supply_temperature_error_index, 2);
    assert_eq!(second_unit.cooling_supply_temperature_error_index, 3);
    assert_eq!(second_unit.heating_supply_temperature_error_index, 4);
    assert_eq!(registry.registered_recurring_diagnostic_count, 4);
    assert_eq!(registry.event_count, 4);
    assert_eq!(
        registry
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.system)
            .collect::<Vec<_>>(),
        vec![SYSTEM, SYSTEM, second_system_id, second_system_id]
    );

    first_unit.init_call_count = 2;
    advance_supply_temperature_diagnostics(
        &mut registry,
        &mut first_unit,
        &first_system,
        diagnostic_context,
    );
    assert_eq!(registry.registered_recurring_diagnostic_count, 4);
    assert_eq!(registry.event_count, 6);
    assert_eq!(registry.diagnostics.len(), 4);
    assert_eq!(registry.diagnostics[0].recurring_severe_call_count, 2);
    assert_eq!(registry.diagnostics[1].recurring_severe_call_count, 2);
    assert_eq!(registry.diagnostics[2].recurring_severe_call_count, 1);
    assert_eq!(registry.diagnostics[3].recurring_severe_call_count, 1);
}
