use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState as State,
    advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_state as advance,
    cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release,
};
use super::release::snapshots_match_bit_exact_for_test;
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState as Cp353State,
    active_operands_from_retained_owners_for_test as cp353_active_operands,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state as advance_cp353,
    completed_cp352_case,
    private_active_predecessor as private_active_cp352_predecessor,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot as Cp352Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit,
};
use crate::psychrometrics::{energyplus_psy_h_fn_tdb_w, energyplus_psy_w_fn_tdb_h};

mod public_release;
mod release_corruption;

const U: Route = Route::UnitOff;
const N: Route = Route::NonCooling;
const P: Route = Route::PositiveGuardFalseFallthrough;
const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted;
const H: Route = Route::DehumidificationControlHumidistatCaseSelectedSkip;
const CSH: Route = Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;

pub(in crate::ideal_loads::calc) fn completed_cp353_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Cp352Snapshot,
    Predecessor,
)> {
    let (mut runtime, system, cp352) =
        completed_cp352_case(cooling_demand_w, overall_availability, capacity_limit)?;
    let cp353 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
            &mut runtime,
            &system,
            cp352,
        )
        .ok()?;
    Some((runtime, system, cp352, cp353))
}

pub(in crate::ideal_loads::calc) fn private_active_cp353_predecessor(
    direct_cp352: Cp352Snapshot,
    runtime: &PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
) -> Option<Predecessor> {
    let private_cp352 =
        private_active_cp352_predecessor(direct_cp352, runtime, system)?;
    let unit = runtime.units.get(&system.id)?;
    let operands = cp353_active_operands(
        runtime,
        unit,
        system,
        private_cp352,
    )?;
    let mut state = Cp353State::new(system.id);
    advance_cp353(&mut state, private_cp352, Some(operands))
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
    let temperature = 12.0;
    let pre_limit = 40_000.0;
    let psychrometric = energyplus_psy_h_fn_tdb_w(temperature, 1.0e-5);
    let maximum = if pre_limit > psychrometric {
        pre_limit
    } else {
        psychrometric
    };
    Predecessor {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed:
            route == Q,
        predecessor_dehumidification_control_humidistat_case_selected_skip: route == H,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == CSH,
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed:
            route == Q,
        dehumidification_control_humidistat_case_selected_skip: route == H,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route == CSH,
        supply_enthalpy_for_overdrying_limit_maximum_read: route == Q,
        supply_enthalpy_before_overdrying_limit_j_per_kg: (route == Q).then_some(pre_limit),
        supply_temperature_for_minimum_humidity_ratio_enthalpy_read: route == Q,
        supply_temperature_c: (route == Q).then_some(temperature),
        psychrometric_minimum_supply_enthalpy_evaluated: route == Q,
        psychrometric_minimum_supply_enthalpy_j_per_kg: (route == Q)
            .then_some(psychrometric),
        source_shaped_two_argument_maximum_evaluated: route == Q,
        maximum_supply_enthalpy_j_per_kg: (route == Q).then_some(maximum),
        supply_enthalpy_assignment_performed: route == Q,
        assigned_supply_enthalpy_j_per_kg: (route == Q).then_some(maximum),
        resulting_supply_enthalpy_j_per_kg: (route == Q).then_some(maximum),
    }
}

const fn operands(
    humidity_ratio: f64,
    temperature: f64,
    enthalpy: f64,
) -> ActiveOperands {
    ActiveOperands {
        supply_humidity_ratio_before_overdrying_limit: humidity_ratio,
        supply_temperature_c: temperature,
        supply_enthalpy_j_per_kg: enthalpy,
    }
}

#[test]
fn source_boundary_six_sites_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2222"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2224"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit-minimum",
            "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
            "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
            "evaluate-psy-w-fn-tdb-h-for-constant-sensible-heat-ratio-overdrying-limit",
            "apply-source-shaped-two-argument-minimum-for-constant-sensible-heat-ratio-overdrying-limit",
            "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
        ]
    );

    let routes = [U, N, P, C0, Q, H, CSH];
    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in routes.into_iter().enumerate() {
        let active = (route == Q).then_some(operands(0.02, 12.0, 30_000.0));
        assert!(advance(&mut state, predecessor(route, index + 1), active).is_some());
    }
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.dehumidification_control_none_case_completed_skip_count, 1);
    assert_eq!(
        state
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_humidistat_case_selected_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 6);
    assert_eq!(
        [
            state.supply_humidity_ratio_for_overdrying_limit_minimum_read_count,
            state.supply_temperature_for_humidity_ratio_inversion_read_count,
            state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
            state.psychrometric_supply_humidity_ratio_evaluation_count,
            state.source_shaped_two_argument_minimum_evaluation_count,
            state.supply_humidity_ratio_assignment_write_count,
        ],
        [1; 6]
    );
}

#[test]
fn active_transition_uses_canonical_inversion_and_source_shaped_minimum() {
    let active = operands(0.02, 12.0, 30_000.0);
    let expected_psychrometric =
        energyplus_psy_w_fn_tdb_h(active.supply_temperature_c, active.supply_enthalpy_j_per_kg);
    let expected_minimum = source_shaped_two_argument_minimum(
        active.supply_humidity_ratio_before_overdrying_limit,
        expected_psychrometric,
    );
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let snapshot = advance(&mut state, predecessor(Q, 1), Some(active)).unwrap();

    assert!(snapshot
        .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed);
    assert_eq!(
        snapshot
            .supply_humidity_ratio_before_overdrying_limit
            .unwrap()
            .to_bits(),
        active
            .supply_humidity_ratio_before_overdrying_limit
            .to_bits()
    );
    assert_eq!(
        snapshot.supply_temperature_c.unwrap().to_bits(),
        active.supply_temperature_c.to_bits()
    );
    assert_eq!(
        snapshot.supply_enthalpy_j_per_kg.unwrap().to_bits(),
        active.supply_enthalpy_j_per_kg.to_bits()
    );
    assert_eq!(
        snapshot
            .psychrometric_supply_humidity_ratio
            .unwrap()
            .to_bits(),
        expected_psychrometric.to_bits()
    );
    for value in [
        snapshot.minimum_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ] {
        assert_eq!(value.unwrap().to_bits(), expected_minimum.to_bits());
    }
    assert!(snapshot.supply_humidity_ratio_for_overdrying_limit_minimum_read);
    assert!(snapshot.supply_temperature_for_humidity_ratio_inversion_read);
    assert!(snapshot.supply_enthalpy_for_humidity_ratio_inversion_read);
    assert!(snapshot.psychrometric_supply_humidity_ratio_evaluated);
    assert!(snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(snapshot.supply_humidity_ratio_assignment_performed);
}

#[test]
fn inactive_routes_are_complete_null_and_direct_none_is_exact() {
    for (ordinal, route) in [U, N, P, C0, H, CSH].into_iter().enumerate() {
        let mut state = State::new(IdealLoadsAirSystemId(7));
        let snapshot = advance(&mut state, predecessor(route, ordinal + 1), None).unwrap();
        assert!(!snapshot.supply_humidity_ratio_for_overdrying_limit_minimum_read);
        assert!(!snapshot.supply_temperature_for_humidity_ratio_inversion_read);
        assert!(!snapshot.supply_enthalpy_for_humidity_ratio_inversion_read);
        assert!(!snapshot.psychrometric_supply_humidity_ratio_evaluated);
        assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
        assert!(!snapshot.supply_humidity_ratio_assignment_performed);
        assert!(
            [
                snapshot.supply_humidity_ratio_before_overdrying_limit,
                snapshot.supply_temperature_c,
                snapshot.supply_enthalpy_j_per_kg,
                snapshot.psychrometric_supply_humidity_ratio,
                snapshot.minimum_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ]
            .into_iter()
            .all(|value| value.is_none())
        );
        assert_eq!(
            cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release(
                snapshot
            ),
            route != H && route != CSH
        );
    }
}

#[test]
fn source_shaped_minimum_preserves_right_bias_and_ieee_bits() {
    assert_eq!(
        source_shaped_two_argument_minimum(-0.0, 0.0).to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(
        source_shaped_two_argument_minimum(0.0, -0.0).to_bits(),
        (-0.0f64).to_bits()
    );
    let right_nan = f64::from_bits(0x7ff8_0000_0000_00a5);
    assert_eq!(
        source_shaped_two_argument_minimum(1.0, right_nan).to_bits(),
        right_nan.to_bits()
    );
    let left_nan = f64::from_bits(0x7ff8_0000_0000_00b6);
    assert_eq!(
        source_shaped_two_argument_minimum(left_nan, 2.0).to_bits(),
        2.0f64.to_bits()
    );
    assert_eq!(
        source_shaped_two_argument_minimum(f64::INFINITY, f64::NEG_INFINITY),
        f64::NEG_INFINITY
    );
}

#[test]
fn canonical_humidity_ratio_inversion_keeps_floor_and_ieee_behavior() {
    assert_eq!(
        energyplus_psy_w_fn_tdb_h(20.0, 0.0).to_bits(),
        1.0e-5f64.to_bits()
    );
    assert_eq!(
        energyplus_psy_w_fn_tdb_h(-0.0, -0.0).to_bits(),
        0.0f64.to_bits()
    );
    assert!(energyplus_psy_w_fn_tdb_h(f64::NAN, 30_000.0).is_nan());
    assert_eq!(1.0e-5f64.to_bits(), 0x3ee4_f8b5_88e3_68f1);
}

#[test]
fn bit_exact_snapshot_matching_distinguishes_nan_payloads() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let snapshot = advance(
        &mut state,
        predecessor(Q, 1),
        Some(operands(0.02, 12.0, 30_000.0)),
    )
    .unwrap();
    let mut same = snapshot;
    let mut different = snapshot;
    same.supply_humidity_ratio_before_overdrying_limit =
        Some(f64::from_bits(0x7ff8_0000_0000_0011));
    different.supply_humidity_ratio_before_overdrying_limit =
        Some(f64::from_bits(0x7ff8_0000_0000_0022));
    let mut same_copy = same;
    assert!(snapshots_match_bit_exact_for_test(same, same_copy));
    same_copy.supply_humidity_ratio_before_overdrying_limit =
        Some(f64::from_bits(0x7ff8_0000_0000_0011));
    assert!(!snapshots_match_bit_exact_for_test(same_copy, different));
}

#[test]
fn active_counter_overflow_is_transactional() {
    macro_rules! reject_overflow {
        ($field:ident) => {{
            let mut state = State::new(IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(
                advance(
                    &mut state,
                    predecessor(Q, 1),
                    Some(operands(0.02, 12.0, 30_000.0)),
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }

    reject_overflow!(transition_count);
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count
    );
    reject_overflow!(source_site_execution_count);
    reject_overflow!(supply_humidity_ratio_for_overdrying_limit_minimum_read_count);
    reject_overflow!(supply_temperature_for_humidity_ratio_inversion_read_count);
    reject_overflow!(supply_enthalpy_for_humidity_ratio_inversion_read_count);
    reject_overflow!(psychrometric_supply_humidity_ratio_evaluation_count);
    reject_overflow!(source_shaped_two_argument_minimum_evaluation_count);
    reject_overflow!(supply_humidity_ratio_assignment_write_count);
    reject_overflow!(
        witnessed_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count
    );
}
