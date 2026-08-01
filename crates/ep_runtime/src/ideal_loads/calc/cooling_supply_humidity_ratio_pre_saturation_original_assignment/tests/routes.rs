//! CP376 eight-route and five-owner partition tests.

use ep_model::DehumidificationControlType;

use super::super::{
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state as advance,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner as Owner,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
};
use super::release::completed_cp375_case;
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Predecessor;

#[test]
fn cp376_retains_all_eight_routes_and_executes_exactly_five_positive_copies() {
    let (_, _, direct) = completed_cp375_case();
    let mut state = State::new(direct.system);
    for route in 0..8 {
        let predecessor = predecessor_for_route(direct, route, 0.008 + route as f64 * 0.0001);
        let input = input_for_route(route, 0.007 + route as f64 * 0.0001);
        let snapshot = advance(&mut state, predecessor, input).expect("valid CP376 route");
        let active = route >= 3;
        assert_eq!(snapshot.purchased_air_supply_humidity_ratio_read, active);
        assert_eq!(
            snapshot.local_supply_humidity_ratio_original_assignment_performed,
            active,
        );
        assert_eq!(
            snapshot
                .assigned_supply_humidity_ratio_original
                .map(f64::to_bits),
            input.map(|input| input.purchased_air_supply_humidity_ratio.to_bits()),
        );
    }
    assert_eq!(state.transition_count, 8);
    assert_eq!(state.source_site_execution_count, 10);
    assert_eq!(
        state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
        5,
    );
    assert_eq!(
        state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count,
        5,
    );
    assert_eq!(state.cp375_maximum_assignment_owner_count, 2);
    assert_eq!(state.cp347_none_case_owner_count, 2);
    assert_eq!(state.cp356_constant_shr_owner_count, 1);
}

#[test]
fn cp376_false_guard_routes_require_the_selector_specific_latest_writer() {
    let (_, _, direct) = completed_cp375_case();
    let cases = [
        (DehumidificationControlType::None, Owner::Cp347NoneCase),
        (
            DehumidificationControlType::ConstantSensibleHeatRatio,
            Owner::Cp356ConstantShr,
        ),
        (
            DehumidificationControlType::Humidistat,
            Owner::Cp362Humidistat,
        ),
        (
            DehumidificationControlType::ConstantSupplyHumidityRatio,
            Owner::Cp365ConstantSupplyHumidityRatio,
        ),
    ];
    for (selector, owner) in cases {
        let mut predecessor = predecessor_for_route(direct, 3, 0.008);
        predecessor.predecessor_dehumidification_control_type = Some(selector);
        let input = ActiveInput {
            purchased_air_supply_humidity_ratio: 0.009,
            owner,
        };
        let mut state = State::new(predecessor.system);
        assert!(advance(&mut state, predecessor, Some(input)).is_some());

        let wrong = ActiveInput {
            owner: Owner::Cp375MaximumAssignment,
            ..input
        };
        let mut state = State::new(predecessor.system);
        assert!(advance(&mut state, predecessor, Some(wrong)).is_none());
        assert_eq!(state, State::new(predecessor.system));
    }
}

#[test]
fn cp376_exact_direct_rejects_private_false_guard_owners() {
    let (_, _, direct) = completed_cp375_case();
    for route in [3, 4] {
        let direct_predecessor = predecessor_for_route(direct, route, 0.008);
        let direct_snapshot =
            private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization(
                direct_predecessor,
                Some(0.009),
                Some(Owner::Cp347NoneCase),
            )
            .expect("direct None/CP347 characterization");
        assert!(
            cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
                direct_snapshot,
            )
        );

        let mut private_predecessor = direct_predecessor;
        private_predecessor.predecessor_dehumidification_control_type =
            Some(DehumidificationControlType::ConstantSensibleHeatRatio);
        let private_snapshot =
            private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization(
                private_predecessor,
                Some(0.009),
                Some(Owner::Cp356ConstantShr),
            )
            .expect("private CSHR/CP356 characterization");
        assert!(
            !cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
                private_snapshot,
            )
        );
    }
}

pub(super) fn predecessor_for_route(
    mut predecessor: Predecessor,
    route: usize,
    cp375_result: f64,
) -> Predecessor {
    predecessor.unit_off_skipped = false;
    predecessor.non_cooling_skipped = false;
    predecessor.positive_guard_false_fallthrough_skipped = false;
    predecessor.predecessor_heating_on_guard_false_fallthrough = false;
    predecessor.predecessor_humidification_control_guard_false_fallthrough = false;
    predecessor
        .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed =
        false;
    predecessor.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed =
        false;
    predecessor.predecessor_dehumidification_control_guard_false_fallthrough = false;
    predecessor.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read = false;
    predecessor.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum = None;
    predecessor.supply_humidity_ratio_for_humidification_for_supply_maximum_read = false;
    predecessor.supply_humidity_ratio_for_humidification_for_supply_maximum = None;
    predecessor.source_shaped_two_argument_maximum_evaluated = false;
    predecessor.maximum_supply_humidity_ratio = None;
    predecessor.purchased_air_supply_humidity_ratio_assignment_performed = false;
    predecessor.assigned_supply_humidity_ratio = None;
    predecessor.resulting_supply_humidity_ratio = None;
    match route {
        0 => {
            predecessor.unit_off_skipped = true;
            predecessor.predecessor_positive_supply_mass_flow_body_entered = false;
            predecessor.predecessor_dehumidification_control_type = None;
        }
        1 => {
            predecessor.non_cooling_skipped = true;
            predecessor.predecessor_positive_supply_mass_flow_body_entered = false;
            predecessor.predecessor_dehumidification_control_type = None;
        }
        2 => {
            predecessor.positive_guard_false_fallthrough_skipped = true;
            predecessor.predecessor_positive_supply_mass_flow_body_entered = false;
            predecessor.predecessor_dehumidification_control_type = None;
        }
        3 => predecessor.predecessor_heating_on_guard_false_fallthrough = true,
        4 => predecessor.predecessor_humidification_control_guard_false_fallthrough = true,
        5 => {
            predecessor.predecessor_dehumidification_control_type =
                Some(DehumidificationControlType::Humidistat);
            predecessor.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed = true;
            set_cp375_active(&mut predecessor, cp375_result);
        }
        6 => {
            predecessor.predecessor_dehumidification_control_type =
                Some(DehumidificationControlType::None);
            predecessor
                .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed =
                true;
            set_cp375_active(&mut predecessor, cp375_result);
        }
        7 => {
            predecessor.predecessor_dehumidification_control_type =
                Some(DehumidificationControlType::ConstantSensibleHeatRatio);
            predecessor.predecessor_dehumidification_control_guard_false_fallthrough = true;
        }
        _ => unreachable!("eight CP376 routes"),
    }
    predecessor
}

pub(super) fn input_for_route(route: usize, value: f64) -> Option<ActiveInput> {
    let owner = match route {
        0..=2 => return None,
        3 | 4 => Owner::Cp347NoneCase,
        5 | 6 => Owner::Cp375MaximumAssignment,
        7 => Owner::Cp356ConstantShr,
        _ => unreachable!("eight CP376 routes"),
    };
    Some(ActiveInput {
        purchased_air_supply_humidity_ratio: if matches!(route, 5 | 6) {
            0.008 + route as f64 * 0.0001
        } else {
            value
        },
        owner,
    })
}

fn set_cp375_active(predecessor: &mut Predecessor, result: f64) {
    predecessor.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read = true;
    predecessor.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum =
        Some(result);
    predecessor.supply_humidity_ratio_for_humidification_for_supply_maximum_read = true;
    predecessor.supply_humidity_ratio_for_humidification_for_supply_maximum = Some(result);
    predecessor.source_shaped_two_argument_maximum_evaluated = true;
    predecessor.maximum_supply_humidity_ratio = Some(result);
    predecessor.purchased_air_supply_humidity_ratio_assignment_performed = true;
    predecessor.assigned_supply_humidity_ratio = Some(result);
    predecessor.resulting_supply_humidity_ratio = Some(result);
}
