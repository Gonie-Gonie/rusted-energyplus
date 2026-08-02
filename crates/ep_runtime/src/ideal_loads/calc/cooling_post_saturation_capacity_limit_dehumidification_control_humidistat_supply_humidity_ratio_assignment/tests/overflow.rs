//! CP395 checked-counter overflow tests.

use super::*;
use ep_model::DehumidificationControlType as D;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState;

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0);
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[19] = usize::MAX,
        |state| state.cp394_supply_temperature_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        |state| state.cp394_supply_enthalpy_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state| {
            state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count =
                usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX,
        |state| state.supply_temperature_owned_read_count = usize::MAX,
        |state| state.supply_temperature_for_humidity_ratio_inversion_read_count = usize::MAX,
        |state| state.supply_enthalpy_owned_read_count = usize::MAX,
        |state| state.supply_enthalpy_for_humidity_ratio_inversion_read_count = usize::MAX,
        |state| state.psychrometric_supply_humidity_ratio_evaluation_count = usize::MAX,
        |state| state.supply_humidity_ratio_assignment_write_count = usize::MAX,
    ];
    assert_each_overflow_rejected(chain.cp394, setters);
}

#[test]
fn every_retained_humidity_ratio_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[18] = usize::MAX,
        |state| state.inactive_transition_count = usize::MAX,
        |state| state.cp394_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state| state.cp394_supply_temperature_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        |state| state.cp394_supply_enthalpy_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
    ];
    assert_each_overflow_rejected(chain.cp394, setters);
}

#[test]
fn inactive_route_without_carriers_checks_only_its_applicable_counters() {
    let chain = fixtures::chain(0, 0, false, None, 1, 0.7, 18.0, 1.0);
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[0] = usize::MAX,
        |state| state.inactive_transition_count = usize::MAX,
    ];
    assert_each_overflow_rejected(chain.cp394, setters);
}

fn assert_each_overflow_rejected(
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
    setters: &[fn(&mut State)],
) {
    for set_overflow in setters {
        let mut state = State::new(predecessor.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
                &mut state,
                predecessor,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}
