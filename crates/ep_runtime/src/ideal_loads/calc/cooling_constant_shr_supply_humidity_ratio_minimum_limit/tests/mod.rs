use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::release::snapshots_match_bit_exact_for_test;
use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRuntimeState as State,
    advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit_state as advance,
    cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot_is_exact_direct_release,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_overdrying_limit::{
    completed_cp353_case,
    private_active_counterfactual_from_direct_release as private_active_cp354_from_direct,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

mod public_release;
mod release_corruption;

const U: Route = Route::UnitOff;
const N: Route = Route::NonCooling;
const P: Route = Route::PositiveGuardFalseFallthrough;
const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMinimumLimitExecuted;
const H: Route = Route::DehumidificationControlHumidistatCaseSelectedSkip;
const CSH: Route = Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;

pub(in crate::ideal_loads::calc) fn completed_cp354_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Predecessor,
)> {
    let (mut runtime, system, _, cp353) =
        completed_cp353_case(cooling_demand_w, overall_availability, capacity_limit)?;
    let cp354 =
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
            &mut runtime,
            &system,
            cp353,
        )
        .ok()?;
    Some((runtime, system, cp354))
}

pub(super) fn private_active_cp354_predecessor(
    runtime: &PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    direct: Predecessor,
) -> Option<Predecessor> {
    let unit = runtime.units.get(&system.id)?;
    private_active_cp354_from_direct(runtime, unit, system, direct)
}

fn predecessor(route: Route, ordinal: usize) -> Predecessor {
    let active = matches!(route, C0 | Q | H | CSH);
    let selector = match route {
        C0 => Some(DehumidificationControlType::None),
        Q => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        H => Some(DehumidificationControlType::Humidistat),
        CSH => Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
        _ => None,
    };
    let before = 0.02;
    let temperature = 12.0;
    let enthalpy = 30_000.0;
    let psychrometric = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
    let minimum = if before < psychrometric {
        before
    } else {
        psychrometric
    };
    Predecessor {
        source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(7),
        parent_call_ordinal: ordinal,
        controlled_zone: ZoneId(9),
        unit_body_entered: route != U,
        predecessor_cooling_body_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_no_outdoor_air_fallback_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_positive_supply_mass_flow_body_entered: active,
        unit_off_skipped: route == U,
        non_cooling_skipped: route == N,
        positive_guard_false_fallthrough_skipped: route == P,
        predecessor_dehumidification_control_type: selector,
        predecessor_dehumidification_control_none_case_completed_skip: route == C0,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed:
            route == Q,
        predecessor_dehumidification_control_humidistat_case_selected_skip: route == H,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == CSH,
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed:
            route == Q,
        dehumidification_control_humidistat_case_selected_skip: route == H,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route == CSH,
        supply_humidity_ratio_for_overdrying_limit_minimum_read: route == Q,
        supply_humidity_ratio_before_overdrying_limit: (route == Q).then_some(before),
        supply_temperature_for_humidity_ratio_inversion_read: route == Q,
        supply_temperature_c: (route == Q).then_some(temperature),
        supply_enthalpy_for_humidity_ratio_inversion_read: route == Q,
        supply_enthalpy_j_per_kg: (route == Q).then_some(enthalpy),
        psychrometric_supply_humidity_ratio_evaluated: route == Q,
        psychrometric_supply_humidity_ratio: (route == Q).then_some(psychrometric),
        source_shaped_two_argument_minimum_evaluated: route == Q,
        minimum_supply_humidity_ratio: (route == Q).then_some(minimum),
        supply_humidity_ratio_assignment_performed: route == Q,
        assigned_supply_humidity_ratio: (route == Q).then_some(minimum),
        resulting_supply_humidity_ratio: (route == Q).then_some(minimum),
    }
}

const fn operands(minimum: f64) -> ActiveOperands {
    ActiveOperands {
        minimum_cooling_supply_air_humidity_ratio: minimum,
    }
}

#[test]
fn source_boundary_four_sites_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2224"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2226"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
            "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
            "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-minimum-limit",
            "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit",
        ]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let active = (route == Q).then_some(operands(0.0077));
        assert!(advance(&mut state, predecessor(route, index + 1), active).is_some());
    }
    assert_eq!(
        [
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
            state
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count,
            state.dehumidification_control_humidistat_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ],
        [1; 7]
    );
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(
        [
            state.supply_humidity_ratio_for_minimum_limit_maximum_read_count,
            state.minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count,
            state.source_shaped_two_argument_maximum_evaluation_count,
            state.supply_humidity_ratio_assignment_write_count,
        ],
        [1; 4]
    );
}

#[test]
fn active_transition_uses_cp354_left_and_typed_minimum_right() {
    let predecessor = predecessor(Q, 1);
    let right = 0.0077;
    let left = predecessor.resulting_supply_humidity_ratio.unwrap();
    let expected = source_shaped_two_argument_maximum(left, right);
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, Some(operands(right))).unwrap();
    assert_eq!(
        snapshot
            .supply_humidity_ratio_before_minimum_limit
            .unwrap()
            .to_bits(),
        left.to_bits()
    );
    assert_eq!(
        snapshot
            .minimum_cooling_supply_air_humidity_ratio
            .unwrap()
            .to_bits(),
        right.to_bits()
    );
    for value in [
        snapshot.maximum_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ] {
        assert_eq!(value.unwrap().to_bits(), expected.to_bits());
    }
}

#[test]
fn source_shaped_maximum_preserves_left_bias_and_ieee_bits() {
    assert_eq!(
        source_shaped_two_argument_maximum(0.0077, 0.0077).to_bits(),
        0.0077f64.to_bits()
    );
    assert_eq!(
        source_shaped_two_argument_maximum(-0.0, 0.0).to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        source_shaped_two_argument_maximum(0.0, -0.0).to_bits(),
        0.0f64.to_bits()
    );
    let right_nan = f64::from_bits(0x7ff8_0000_0000_00a5);
    assert_eq!(
        source_shaped_two_argument_maximum(1.0, right_nan).to_bits(),
        1.0f64.to_bits()
    );
    let left_nan = f64::from_bits(0x7ff8_0000_0000_00b6);
    assert_eq!(
        source_shaped_two_argument_maximum(left_nan, 2.0).to_bits(),
        left_nan.to_bits()
    );
    assert_eq!(
        source_shaped_two_argument_maximum(f64::NEG_INFINITY, f64::INFINITY),
        f64::INFINITY
    );
}

#[test]
fn inactive_routes_are_complete_null_and_direct_none_is_exact() {
    for (ordinal, route) in [U, N, P, C0, H, CSH].into_iter().enumerate() {
        let mut state = State::new(IdealLoadsAirSystemId(7));
        let snapshot = advance(&mut state, predecessor(route, ordinal + 1), None).unwrap();
        assert!(!snapshot.supply_humidity_ratio_for_minimum_limit_maximum_read);
        assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read);
        assert!(!snapshot.source_shaped_two_argument_maximum_evaluated);
        assert!(!snapshot.supply_humidity_ratio_assignment_performed);
        assert_eq!(
            [
                snapshot.supply_humidity_ratio_before_minimum_limit,
                snapshot.minimum_cooling_supply_air_humidity_ratio,
                snapshot.maximum_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ],
            [None; 5]
        );
        assert_eq!(
            cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot_is_exact_direct_release(
                snapshot
            ),
            route != H && route != CSH
        );
    }
}

#[test]
fn bit_exact_snapshot_matching_and_active_overflow_are_transactional() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let snapshot = advance(&mut state, predecessor(Q, 1), Some(operands(0.0077))).unwrap();
    let mut forged = snapshot;
    forged.minimum_cooling_supply_air_humidity_ratio = Some(f64::from_bits(0x7ff8_0000_0000_0011));
    assert!(snapshots_match_bit_exact_for_test(snapshot, snapshot));
    assert!(!snapshots_match_bit_exact_for_test(snapshot, forged));

    macro_rules! reject_overflow {
        ($field:ident) => {{
            let mut state = State::new(IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(advance(&mut state, predecessor(Q, 1), Some(operands(0.0077))).is_none());
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count);
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count
    );
    reject_overflow!(source_site_execution_count);
    reject_overflow!(supply_humidity_ratio_for_minimum_limit_maximum_read_count);
    reject_overflow!(minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count);
    reject_overflow!(source_shaped_two_argument_maximum_evaluation_count);
    reject_overflow!(supply_humidity_ratio_assignment_write_count);
    reject_overflow!(
        witnessed_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count
    );
}
