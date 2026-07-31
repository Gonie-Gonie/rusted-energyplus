//! Persistent CP367 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_mixed_air_assignment::transition::{
    next_transition_fits as pure_next_transition_fits, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_constant_supply_humidity_ratio_case_break
            .system
            == system
        && unit
            .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_constant_supply_humidity_ratio_case_break
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
    selector: DehumidificationControlType,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit.calc_cooling_default_supply_humidity_ratio_mixed_air_assignment;
    let prior = &unit.calc_cooling_constant_supply_humidity_ratio_case_break;
    state_is_consistent(state, witness, predecessor.system, selector)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_count(state.unit_off_skip_count, route == Route::UnitOff)
            == Some(prior.unit_off_skip_count)
        && pending_count(state.non_cooling_skip_count, route == Route::NonCooling)
            == Some(prior.non_cooling_skip_count)
        && pending_count(
            state.positive_guard_false_fallthrough_skip_count,
            route == Route::PositiveGuardFalseFallthrough,
        ) == Some(prior.positive_guard_false_fallthrough_skip_count)
        && pending_count(
            state.dehumidification_control_none_case_completed_skip_count,
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        ) == Some(prior.dehumidification_control_none_case_completed_skip_count)
        && pending_count(
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            route == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
        ) == Some(
            prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        )
        && pending_count(
            state.dehumidification_control_humidistat_case_completed_skip_count,
            route == Route::DehumidificationControlHumidistatCaseCompletedSkip,
        ) == Some(prior.dehumidification_control_humidistat_case_completed_skip_count)
        && pending_count(
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            route
                == Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip,
        ) == Some(
            prior.dehumidification_control_constant_supply_humidity_ratio_case_break_count,
        )
}

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor) -> bool {
    predecessor_route(predecessor).is_some_and(|route| pure_next_transition_fits(state, route))
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
    selector: DehumidificationControlType,
) -> bool {
    let state = &unit.calc_cooling_default_supply_humidity_ratio_mixed_air_assignment;
    let prior = &unit.calc_cooling_constant_supply_humidity_ratio_case_break;
    state_is_consistent(state, witness, snapshot.system, selector)
        && state.transition_count == prior.transition_count
        && state.unit_off_skip_count == prior.unit_off_skip_count
        && state.non_cooling_skip_count == prior.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == prior.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completed_skip_count
            == prior.dehumidification_control_none_case_completed_skip_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        && state.dehumidification_control_humidistat_case_completed_skip_count
            == prior.dehumidification_control_humidistat_case_completed_skip_count
        && state
            .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
            == prior.dehumidification_control_constant_supply_humidity_ratio_case_break_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_exact(latest, snapshot))
}

pub(in crate::ideal_loads) fn cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_metadata_is_consistent(
    state: &State,
    expected_ordinal: usize,
) -> bool {
    expected_ordinal > 0
        && state.transition_count == expected_ordinal
        && state.latest_transition_ordinal == Some(expected_ordinal)
        && state
            .latest
            .is_some_and(|latest| latest_route_is_counted(state, latest))
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
    selector: DehumidificationControlType,
) -> bool {
    let Some(route_partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ]) else {
        return false;
    };
    let Some(selected) = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ]) else {
        return false;
    };
    let Some(recursively_witnessed) = checked_sum(&[
        state.witnessed_dehumidification_control_none_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.witnessed_dehumidification_control_humidistat_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ]) else {
        return false;
    };
    let c0 = state.dehumidification_control_none_case_completed_skip_count;
    let q = state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count;
    let h = state.dehumidification_control_humidistat_case_completed_skip_count;
    let csh =
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count;
    let selector_partition_matches = c0
        == usize::from(selector == DehumidificationControlType::None) * selected
        && q == usize::from(selector == DehumidificationControlType::ConstantSensibleHeatRatio)
            * selected
        && h == usize::from(selector == DehumidificationControlType::Humidistat) * selected
        && csh
            == usize::from(selector == DehumidificationControlType::ConstantSupplyHumidityRatio)
                * selected;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && selected == recursively_witnessed
        && selector_partition_matches
        && PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
            .len()
            == 2
        && state.mixed_air_humidity_ratio_read_count == 0
        && state.supply_humidity_ratio_assignment_count == 0
        && state.source_site_execution_count == 0
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_dehumidification_control_none_case_completed_skip_count == c0
        && state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == q
        && state.witnessed_dehumidification_control_humidistat_case_completed_skip_count == h
        && state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
            == csh;
    if !counters_match {
        return false;
    }
    match (state.transition_count, state.latest, witness) {
        (0, None, None) => {
            state.latest_route.is_none() && state.latest_transition_ordinal.is_none()
        }
        (count, Some(latest), Some(witness)) => {
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && latest_route_is_counted(state, latest)
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && snapshots_match_exact(latest, witness)
                && (!latest.unit_body_entered
                    || latest.predecessor_dehumidification_control_type.is_none()
                    || latest.predecessor_dehumidification_control_type == Some(selector))
        }
        _ => false,
    }
}

fn latest_route_is_counted(state: &State, latest: Snapshot) -> bool {
    let Some(route) = snapshot_route(latest) else {
        return false;
    };
    state.latest_route == Some(route) && route_count(state, route) > 0
}

const fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            state.dehumidification_control_none_case_completed_skip_count
        }
        Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip => {
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        }
        Route::DehumidificationControlHumidistatCaseCompletedSkip => {
            state.dehumidification_control_humidistat_case_completed_skip_count
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip => {
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
        }
    }
}

fn pending_count(count: usize, applies: bool) -> Option<usize> {
    count.checked_add(usize::from(applies))
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |total, value| total.checked_add(*value))
}

#[cfg(test)]
mod tests {
    use ep_model::ZoneId;

    use super::*;
    use crate::ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    };

    #[test]
    fn latest_route_counter_transfer_is_rejected_without_state_mutation() {
        let system = IdealLoadsAirSystemId(367);
        let latest = Snapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
            system,
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(9),
            unit_body_entered: false,
            predecessor_cooling_body_entered: false,
            predecessor_no_outdoor_air_fallback_entered: false,
            predecessor_positive_supply_mass_flow_body_entered: false,
            unit_off_skipped: true,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            predecessor_dehumidification_control_type: None,
            predecessor_dehumidification_control_none_case_completed_skip: false,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
                false,
            predecessor_dehumidification_control_humidistat_case_completed_skip: false,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break:
                false,
            dehumidification_control_none_case_completed_skip: false,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            dehumidification_control_humidistat_case_completed_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
            dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed:
                false,
        };
        let mut state = State::new(system);
        state.transition_count = 1;
        state.unit_off_skip_count = 1;
        state.latest = Some(latest);
        state.latest_route = Some(Route::UnitOff);
        state.latest_transition_ordinal = Some(1);
        assert!(state_is_consistent(
            &state,
            Some(latest),
            system,
            DehumidificationControlType::None,
        ));
        assert!(
            cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_metadata_is_consistent(
                &state, 1,
            )
        );
        let mut zero_ordinal_latest = state.clone();
        zero_ordinal_latest.transition_count = 0;
        zero_ordinal_latest.latest_transition_ordinal = Some(0);
        assert!(
            !cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_metadata_is_consistent(
                &zero_ordinal_latest,
                0,
            )
        );

        state.unit_off_skip_count = 0;
        state.non_cooling_skip_count = 1;
        let before_validation = state.clone();
        assert!(!state_is_consistent(
            &state,
            Some(latest),
            system,
            DehumidificationControlType::None,
        ));
        assert!(
            !cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_metadata_is_consistent(
                &state, 1,
            )
        );
        assert_eq!(state, before_validation);
    }
}
