//! CP392 exhaustive route and checked-accounting tests.

use super::*;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

#[test]
fn cp392_preserves_thirty_routes_and_assigns_exactly_three() {
    let chains = fixtures::all_chains();
    let mut state = State::new(chains[0].cp391.system);
    let snapshots: Vec<_> = chains
        .iter()
        .map(|chain| advance(&mut state, chain.cp391).expect("CP392 route"))
        .collect();

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(state.assignment_count(), 3);
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 12);
    assert_eq!(state.cp391_supply_temperature_state_owner_count, 27);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 27);
    assert_eq!(state.cp391_supply_enthalpy_state_owner_count, 17);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 17);
    for count in active_counters(&state) {
        assert_eq!(count, 3);
    }
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| exact_direct(**snapshot))
            .count(),
        11,
    );

    for (index, (chain, snapshot)) in chains.into_iter().zip(snapshots).enumerate() {
        assert!(exact(snapshot));
        assert_eq!(exact_direct(snapshot), matches!(index, 0..=8 | 20 | 24));
        let has_temperature = index >= 3;
        let has_enthalpy = matches!(index, 5 | 8 | 11 | 14 | 17..=29);
        assert_eq!(
            snapshot.cp391_retained_supply_temperature_state_owned,
            has_temperature
        );
        assert_eq!(
            snapshot.cp391_retained_supply_enthalpy_state_owned,
            has_enthalpy
        );
        assert_bits_eq(
            snapshot.resulting_supply_temperature_c,
            chain.cp391.resulting_supply_temperature_c,
        );
        assert_bits_eq(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            chain.cp391.resulting_supply_enthalpy_j_per_kg,
        );

        if matches!(index, 18 | 22 | 28) {
            let temperature = chain
                .cp391
                .resulting_supply_temperature_c
                .expect("temperature");
            let enthalpy = chain
                .cp391
                .resulting_supply_enthalpy_j_per_kg
                .expect("enthalpy");
            let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
            assert_eq!(
                snapshot.supply_temperature_c.map(f64::to_bits),
                Some(temperature.to_bits())
            );
            assert_eq!(
                snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
                Some(enthalpy.to_bits())
            );
            assert_eq!(
                snapshot
                    .psychrometric_supply_humidity_ratio
                    .map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert_eq!(
                snapshot.assigned_supply_humidity_ratio.map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert_eq!(
                snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
                Some(expected.to_bits())
            );
        } else {
            assert_eq!(
                [
                    snapshot.supply_temperature_c,
                    snapshot.supply_enthalpy_j_per_kg,
                    snapshot.psychrometric_supply_humidity_ratio,
                    snapshot.assigned_supply_humidity_ratio,
                    snapshot.resulting_supply_humidity_ratio,
                ],
                [None; 5],
            );
        }
    }
}

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let chain = active_chain();
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[18] = usize::MAX,
        |state| state.cp391_supply_temperature_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        |state| state.cp391_supply_enthalpy_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state| {
            state.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count = usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX,
        |state| state.supply_temperature_owned_read_count = usize::MAX,
        |state| state.supply_temperature_for_humidity_ratio_inversion_read_count = usize::MAX,
        |state| state.supply_enthalpy_owned_read_count = usize::MAX,
        |state| state.supply_enthalpy_for_humidity_ratio_inversion_read_count = usize::MAX,
        |state| state.psychrometric_supply_humidity_ratio_evaluation_count = usize::MAX,
        |state| state.supply_humidity_ratio_assignment_write_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(chain.cp391.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, chain.cp391).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn inactive_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(3, 1, false, None, 1, 0.7, 18.0, 1.0);
    let mut state = State::new(chain.cp391.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, chain.cp391).is_none());
    assert_eq!(state, before);
}

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState;

fn advance(state: &mut State, predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot>{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_state(state, predecessor)
}

fn active_chain() -> fixtures::Chain {
    fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    )
}

fn active_counters(state: &State) -> [usize; 6] {
    [
        state.supply_temperature_owned_read_count,
        state.supply_temperature_for_humidity_ratio_inversion_read_count,
        state.supply_enthalpy_owned_read_count,
        state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        state.psychrometric_supply_humidity_ratio_evaluation_count,
        state.supply_humidity_ratio_assignment_write_count,
    ]
}

fn assert_bits_eq(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}

fn exact(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact(snapshot)
}

fn exact_direct(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(snapshot)
}

trait AssignmentCount {
    fn assignment_count(&self) -> usize;
}

impl AssignmentCount for State {
    fn assignment_count(&self) -> usize {
        self.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count
    }
}
