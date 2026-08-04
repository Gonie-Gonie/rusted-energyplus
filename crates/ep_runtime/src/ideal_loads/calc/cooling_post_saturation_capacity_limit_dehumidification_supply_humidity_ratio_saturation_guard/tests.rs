//! CP413 boundary, route, IEEE, release-shape, and overflow tests.

use ep_model::{DehumidificationControlType as D, IdealLoadsAirSystemId, ZoneId};

use super::transition::routes::{RetainedRoute, predecessor_index_is_split};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as Cp412ActiveInput,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_state as advance_cp411,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state as advance_cp412,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Cp410Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as Cp411State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as Cp412State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE as CP410_EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE as CP410_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER as CP410_ORDER,
};

mod exhaustive;
mod ieee;
mod overflow;
mod release;

#[test]
fn cp413_boundary_and_four_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2315",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2316",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER,
        &[
            "read-local-saturation-supply-humidity-ratio-for-saturation-guard",
            "read-local-original-supply-humidity-ratio-for-saturation-guard",
            "compare-local-saturation-supply-humidity-ratio-strictly-less-than-local-original-supply-humidity-ratio",
            "enter-saturation-supply-humidity-ratio-guard-body-if-comparison-satisfied",
        ],
    );
}

#[test]
fn option_presence_cannot_forge_cp413_reachability() {
    use super::{
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
        advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance,
    };
    let inactive = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 3)
        .expect("temperature-present inactive route");
    let predecessor = predecessor_for_route(inactive, 1);
    let snapshot = advance(&mut State::new(predecessor.system), predecessor)
        .expect("route-derived skip");
    assert!(!snapshot
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated);
    assert!(snapshot.saturation_supply_humidity_ratio_for_guard.is_none());
    assert!(snapshot.original_supply_humidity_ratio_for_guard.is_none());
}

pub(super) fn all_routes() -> Vec<RetainedRoute> {
    let mut routes = Vec::new();
    for predecessor_index in 0..30 {
        let active = matches!(predecessor_index, 18..=29);
        if predecessor_index_is_split(predecessor_index) {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: true,
                predecessor_maximum_capacity_assignment_executed: false,
                active,
                body_entered: false,
            });
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: true,
                active,
                body_entered: false,
            });
        } else {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: false,
                active,
                body_entered: false,
            });
        }
    }
    routes
}

pub(super) fn predecessor_for_route(route: RetainedRoute, ordinal: usize) -> Predecessor {
    predecessor_with_original(route, ordinal, 0.001)
}

pub(super) fn predecessor_for_outcome(
    route: RetainedRoute,
    ordinal: usize,
    body_entered: bool,
) -> Predecessor {
    if !route.active {
        return predecessor_for_route(route, ordinal);
    }
    predecessor_with_original(route, ordinal, if body_entered { 0.03 } else { 0.001 })
}

pub(super) fn predecessor_with_original(
    route: RetainedRoute,
    ordinal: usize,
    original: f64,
) -> Predecessor {
    predecessor_with_operands(route, ordinal, original, 18.0, 101_325.0)
}

pub(super) fn predecessor_with_operands(
    route: RetainedRoute,
    ordinal: usize,
    original: f64,
    temperature: f64,
    pressure: f64,
) -> Predecessor {
    let mut cp410 = cp410_predecessor_for_route(route, ordinal);
    if route.active {
        let original = Some(original);
        cp410.predecessor_cp409_resulting_supply_humidity_ratio = original;
        cp410.resulting_supply_humidity_ratio = original;
        let temperature = Some(temperature);
        cp410.predecessor_cp409_resulting_supply_temperature_c = temperature;
        cp410.resulting_supply_temperature_c = temperature;
    }
    let mut cp411_state = Cp411State::new(cp410.system);
    let cp411 = advance_cp411(&mut cp411_state, cp410).expect("valid CP411 predecessor");
    let mut cp412_state = Cp412State::new(cp411.system);
    let input = route.active.then_some(Cp412ActiveInput {
        outdoor_barometric_pressure_pa: pressure,
    });
    advance_cp412(&mut cp412_state, cp411, input).expect("valid CP412 predecessor")
}

fn cp410_predecessor_for_route(route: RetainedRoute, ordinal: usize) -> Cp410Snapshot {
    let index = route.predecessor_index;
    let mut snapshot = base_cp410_predecessor();
    snapshot.parent_call_ordinal = ordinal;
    snapshot.unit_off_skipped = index == 0;
    snapshot.non_cooling_skipped = index == 1;
    snapshot.positive_guard_false_fallthrough_skipped = index == 2;
    if index >= 3 {
        set_lineage(&mut snapshot, lineage(index));
        set_stage(&mut snapshot, if index < 18 { (index - 3) % 3 } else { 3 });
    }
    if index >= 18 {
        set_switch_case(&mut snapshot, index, route);
    }
    set_carriers(&mut snapshot, index, ordinal);
    snapshot
}

fn base_cp410_predecessor() -> Cp410Snapshot {
    Cp410Snapshot {
        source: CP410_SOURCE,
        first_excluded_source: CP410_EXCLUDED,
        source_order: CP410_ORDER,
        system: IdealLoadsAirSystemId(412),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(412),
        unit_off_skipped: true,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: false,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: false,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: false,
        predecessor_capacity_limit_body_entered: false,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: false,
        predecessor_dehumidification_body_entered: false,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: false,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: false,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: false,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed: false,
        predecessor_supply_enthalpy_assignment_executed: false,
        predecessor_dehumidification_control_type_read: false,
        predecessor_dehumidification_control_type: None,
        predecessor_dehumidification_control_switch_dispatched: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
            false,
        predecessor_dehumidification_control_humidistat_case_entered: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed:
            false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered:
            false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough:
            false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed:
            false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break:
            false,
        predecessor_cp409_resulting_supply_humidity_ratio: None,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: None,
        predecessor_cp409_resulting_supply_temperature_c: None,
        dehumidification_control_default_case_exited_via_break: false,
        resulting_supply_humidity_ratio: None,
        resulting_supply_enthalpy_j_per_kg: None,
        resulting_supply_temperature_c: None,
    }
}

fn lineage(index: usize) -> usize {
    match index {
        3..=17 => (index - 3) / 3,
        18..=21 => 0,
        22..=25 => 1,
        26 => 2,
        27 => 3,
        28..=29 => 4,
        _ => 0,
    }
}

fn set_lineage(snapshot: &mut Cp410Snapshot, lineage: usize) {
    snapshot.heating_availability_guard_false_fallthrough = lineage == 0;
    snapshot.humidification_control_guard_false_fallthrough = lineage == 1;
    snapshot.dehumidification_control_humidistat_maximum_assignment_executed = lineage == 2;
    snapshot.dehumidification_control_none_maximum_assignment_executed = lineage == 3;
    snapshot.dehumidification_control_guard_false_fallthrough = lineage == 4;
}

fn set_stage(snapshot: &mut Cp410Snapshot, stage: usize) {
    snapshot.predecessor_capacity_limit_guard_evaluated = true;
    if stage == 0 {
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough = true;
        return;
    }
    snapshot.predecessor_capacity_limit_body_entered = true;
    snapshot.predecessor_dehumidification_guard_evaluated = true;
    if stage == 1 {
        snapshot.predecessor_dehumidification_guard_false_fallthrough = true;
        return;
    }
    snapshot.predecessor_dehumidification_body_entered = true;
    snapshot.predecessor_dehumidification_total_output_assignment_executed = true;
    snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated = true;
    if stage == 2 {
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough = true;
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough = true;
        return;
    }
    snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered = true;
    snapshot.dehumidification_total_output_maximum_capacity_assignment_executed = true;
}

fn set_switch_case(snapshot: &mut Cp410Snapshot, index: usize, route: RetainedRoute) {
    let selector = match index {
        18 | 22 | 28 => D::ConstantSensibleHeatRatio,
        19 | 23 | 26 => D::Humidistat,
        20 | 24 | 27 => D::None,
        _ => D::ConstantSupplyHumidityRatio,
    };
    let constant_shr = matches!(index, 18 | 22 | 28);
    let humidistat = matches!(index, 19 | 23 | 26);
    let shared = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
    snapshot.predecessor_supply_enthalpy_assignment_executed = true;
    snapshot.predecessor_dehumidification_control_type_read = true;
    snapshot.predecessor_dehumidification_control_type = Some(selector);
    snapshot.predecessor_dehumidification_control_switch_dispatched = true;
    snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered =
        constant_shr;
    snapshot
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break =
        constant_shr;
    snapshot.predecessor_dehumidification_control_humidistat_case_entered = humidistat;
    snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed = humidistat;
    snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break = humidistat;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered = shared;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough = route.predecessor_guard_false_fallthrough;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed = route.predecessor_maximum_capacity_assignment_executed;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break = shared;
}

fn set_carriers(snapshot: &mut Cp410Snapshot, index: usize, ordinal: usize) {
    let ordinal = ordinal as u64;
    let humidity =
        matches!(index, 18..=29).then(|| f64::from_bits(0x3f80_0000_0000_0000 + ordinal));
    let enthalpy = matches!(index, 5 | 8 | 11 | 14 | 17..=29)
        .then(|| f64::from_bits(0x40e0_0000_0000_0000 + ordinal));
    let temperature =
        (index >= 3).then(|| f64::from_bits(0x4032_0000_0000_0000 + ordinal));
    snapshot.predecessor_cp409_resulting_supply_humidity_ratio = humidity;
    snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.predecessor_cp409_resulting_supply_temperature_c = temperature;
    snapshot.resulting_supply_humidity_ratio = humidity;
    snapshot.resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.resulting_supply_temperature_c = temperature;
}
