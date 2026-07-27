use ep_model::{IdealLoadsAirSystem, IdealLoadsLimit};

use super::{
    PurchasedAirInitCallContext, PurchasedAirRuntimeState, init_purchased_air_runtime,
    lifecycle_tests::{SYSTEM, context, finite_flow_system, single_manager_plan, topology},
    purchased_air_init_lifecycle_summary,
};

#[test]
fn warning_predicates_preserve_strict_setpoint_limit_and_availability_gates() {
    let evaluate = |system: &IdealLoadsAirSystem, context: PurchasedAirInitCallContext| {
        let mut state = PurchasedAirRuntimeState::default();
        let plan = single_manager_plan();
        let snapshot = init_purchased_air_runtime(&mut state, &plan, &topology(), system, context)
            .expect("warning predicate initialization");
        let summary = purchased_air_init_lifecycle_summary(&state, SYSTEM)
            .expect("warning predicate lifecycle");
        (
            snapshot.transition.cooling_supply_temperature_warning,
            snapshot.transition.heating_supply_temperature_warning,
            summary.cooling_supply_temperature_warning_count,
            summary.heating_supply_temperature_warning_count,
        )
    };

    let mut warning_system = finite_flow_system();
    warning_system.cooling_limit = IdealLoadsLimit::NoLimit;
    warning_system.heating_limit = IdealLoadsLimit::NoLimit;
    warning_system.minimum_cooling_supply_air_temperature_c = 22.001;
    warning_system.maximum_heating_supply_air_temperature_c = 19.999;
    assert_eq!(evaluate(&warning_system, context(true)), (true, true, 1, 1));

    let mut equal = warning_system.clone();
    equal.minimum_cooling_supply_air_temperature_c = 22.0;
    equal.maximum_heating_supply_air_temperature_c = 20.0;
    assert_eq!(evaluate(&equal, context(true)), (false, false, 0, 0));

    let mut zero_setpoints = context(true);
    zero_setpoints.cooling_setpoint_c = 0.0;
    zero_setpoints.heating_setpoint_c = 0.0;
    assert_eq!(
        evaluate(&warning_system, zero_setpoints),
        (false, false, 0, 0)
    );

    let mut cooling_off = context(true);
    cooling_off.cooling_availability = 0.0;
    assert_eq!(evaluate(&warning_system, cooling_off), (false, true, 0, 1));
    let mut heating_off = context(true);
    heating_off.heating_availability = 0.0;
    assert_eq!(evaluate(&warning_system, heating_off), (true, false, 1, 0));
    let mut unit_off = context(true);
    unit_off.overall_availability = 0.0;
    assert_eq!(evaluate(&warning_system, unit_off), (false, false, 0, 0));
    let mut nan_availability = context(true);
    nan_availability.overall_availability = f64::NAN;
    nan_availability.cooling_availability = f64::NAN;
    nan_availability.heating_availability = f64::NAN;
    assert_eq!(
        evaluate(&warning_system, nan_availability),
        (true, true, 1, 1)
    );

    let mut limited = warning_system.clone();
    limited.cooling_limit = IdealLoadsLimit::LimitFlowRateAndCapacity;
    limited.heating_limit = IdealLoadsLimit::LimitFlowRateAndCapacity;
    assert_eq!(evaluate(&limited, context(true)), (false, false, 0, 0));

    let mut repeated_state = PurchasedAirRuntimeState::default();
    let repeated_plan = single_manager_plan();
    for _ in 0..2 {
        init_purchased_air_runtime(
            &mut repeated_state,
            &repeated_plan,
            &topology(),
            &warning_system,
            context(true),
        )
        .expect("recurring warning call");
    }
    let repeated = purchased_air_init_lifecycle_summary(&repeated_state, SYSTEM)
        .expect("recurring warning lifecycle");
    assert_eq!(repeated.cooling_supply_temperature_warning_count, 2);
    assert_eq!(repeated.heating_supply_temperature_warning_count, 2);
}
