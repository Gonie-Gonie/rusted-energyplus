use super::{C0, CSH, H, N, P, Q, U, operands, predecessor};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState as State,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_state as advance,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn every_counter_overflow_rejects_without_mutation() {
    macro_rules! reject_overflow {
        ($field:ident, $route:expr) => {{
            let mut state = State::new(IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(
                advance(
                    &mut state,
                    predecessor($route, 1, 0.006),
                    operands($route, 0.0077),
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count, H);
    reject_overflow!(unit_off_skip_count, U);
    reject_overflow!(non_cooling_skip_count, N);
    reject_overflow!(positive_guard_false_fallthrough_skip_count, P);
    reject_overflow!(witnessed_positive_guard_false_fallthrough_skip_count, P);
    reject_overflow!(dehumidification_control_none_case_completed_skip_count, C0);
    reject_overflow!(
        witnessed_dehumidification_control_none_case_completed_skip_count,
        C0
    );
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        Q
    );
    reject_overflow!(
        witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        Q
    );
    reject_overflow!(
        dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count,
        H
    );
    reject_overflow!(
        witnessed_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count,
        H
    );
    reject_overflow!(
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        CSH
    );
    reject_overflow!(
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        CSH
    );
    reject_overflow!(source_site_execution_count, H);
    reject_overflow!(
        supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count,
        H
    );
    reject_overflow!(
        minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count,
        H
    );
    reject_overflow!(source_shaped_two_argument_maximum_evaluation_count, H);
    reject_overflow!(
        supply_humidity_ratio_for_dehumidification_assignment_count,
        H
    );
}

#[test]
fn four_site_increment_preflight_rejects_max_minus_three() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    state.source_site_execution_count = usize::MAX - 3;
    let before = state.clone();
    assert!(advance(&mut state, predecessor(H, 1, 0.006), operands(H, 0.0077),).is_none());
    assert_eq!(state, before);
}
