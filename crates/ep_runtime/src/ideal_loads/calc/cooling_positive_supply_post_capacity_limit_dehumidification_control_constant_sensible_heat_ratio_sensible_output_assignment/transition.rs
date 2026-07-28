//! Pure CP349-to-CP350 constant-SHR sensible-output assignment transition.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput
{
    /// Same-call CP330-owned final positive supply mass flow.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Same-call CP329-owned mixed-air temperature.
    pub mixed_air_temperature_c: f64,
    /// Branch-selected final supply temperature owned by CP334 or CP344.
    pub supply_temperature_c: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput,
    >,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
>{
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    if !next_transition_fits(state, route) {
        return None;
    }
    let prepared = prepare_values(route, predecessor, active_input)?;

    state.transition_count += 1;
    match route {
        Route::UnitOff => state.unit_off_skip_count += 1,
        Route::NonCooling => state.non_cooling_skip_count += 1,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
            state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        }
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            state.dehumidification_control_none_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_none_case_completed_skip_count += 1;
        }
        Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count +=
                1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
                    .len();
            state.supply_mass_flow_rate_read_count += 1;
            state.cp_air_read_count += 1;
            state.supply_mass_flow_rate_times_cp_air_calculation_count += 1;
            state.mixed_air_temperature_read_count += 1;
            state.supply_temperature_read_count += 1;
            state.mixed_air_minus_supply_temperature_calculation_count += 1;
            state.cooling_sensible_output_calculation_count += 1;
            state.cooling_sensible_output_assignment_write_count += 1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count +=
                1;
        }
        Route::DehumidificationControlHumidistatCaseSelectedSkip => {
            state.dehumidification_control_humidistat_case_selected_skip_count += 1;
            state.witnessed_dehumidification_control_humidistat_case_selected_skip_count += 1;
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => {
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count +=
                1;
        }
    }

    let none_skip = route == Route::DehumidificationControlNoneCaseCompletedSkip;
    let assignment =
        route == Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned;
    let humidistat_skip = route == Route::DehumidificationControlHumidistatCaseSelectedSkip;
    let constant_supply_skip =
        route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;
    let snapshot = PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_selected_skip: predecessor
            .dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip: none_skip,
        dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed:
            assignment,
        dehumidification_control_humidistat_case_selected_skip: humidistat_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply_skip,
        supply_mass_flow_rate_read: assignment,
        supply_mass_flow_rate_kg_per_s: prepared.supply_mass_flow_rate_kg_per_s,
        cp_air_read: assignment,
        cp_air_j_per_kg_k: prepared.cp_air_j_per_kg_k,
        supply_mass_flow_rate_times_cp_air_calculated: assignment,
        supply_mass_flow_rate_times_cp_air_w_per_k: prepared
            .supply_mass_flow_rate_times_cp_air_w_per_k,
        mixed_air_temperature_read: assignment,
        mixed_air_temperature_c: prepared.mixed_air_temperature_c,
        supply_temperature_read: assignment,
        supply_temperature_c: prepared.supply_temperature_c,
        mixed_air_minus_supply_temperature_calculated: assignment,
        mixed_air_minus_supply_temperature_k: prepared.mixed_air_minus_supply_temperature_k,
        cooling_sensible_output_calculated: assignment,
        calculated_cooling_sensible_output_w: prepared.calculated_cooling_sensible_output_w,
        cooling_sensible_output_assigned: assignment,
        cooling_sensible_output_w: prepared.calculated_cooling_sensible_output_w,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let local_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        )
        + usize::from(predecessor.dehumidification_control_humidistat_case_selected_skip)
        + usize::from(
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        );
    if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && local_count == 0
    {
        return Some(Route::UnitOff);
    }
    if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && local_count == 0
    {
        return Some(Route::NonCooling);
    }
    if !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && predecessor.positive_guard_false_fallthrough_skipped
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
        && local_count == 0
    {
        return Some(Route::PositiveGuardFalseFallthrough);
    }
    if !active_prefix(predecessor) || local_count != 1 {
        return None;
    }
    match predecessor.predecessor_dehumidification_control_type? {
        DehumidificationControlType::None
            if predecessor.predecessor_dehumidification_control_none_case_completed
                && predecessor.dehumidification_control_none_case_completed_skip =>
        {
            Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        }
        DehumidificationControlType::ConstantSensibleHeatRatio
            if predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
                && predecessor
                    .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
                && predecessor.cp_air_assigned =>
        {
            Some(Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned)
        }
        DehumidificationControlType::Humidistat
            if predecessor.dehumidification_control_humidistat_case_selected_skip =>
        {
            Some(Route::DehumidificationControlHumidistatCaseSelectedSkip)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =>
        {
            Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
        }
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn predecessor_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.psychrometric_cp_air_result_j_per_kg_k,
        right.psychrometric_cp_air_result_j_per_kg_k,
    ) && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.cp_air_j_per_kg_k = None;
    }
    values_match && left == right
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState,
    route: Route,
) -> bool {
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    match route {
        Route::UnitOff => state.unit_off_skip_count.checked_add(1).is_some(),
        Route::NonCooling => state.non_cooling_skip_count.checked_add(1).is_some(),
        Route::PositiveGuardFalseFallthrough => {
            checked_pair(
                state.positive_guard_false_fallthrough_skip_count,
                state.witnessed_positive_guard_false_fallthrough_skip_count,
            )
        }
        Route::DehumidificationControlNoneCaseCompletedSkip => checked_pair(
            state.dehumidification_control_none_case_completed_skip_count,
            state.witnessed_dehumidification_control_none_case_completed_skip_count,
        ),
        Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned => {
            let counters = [
                state
                    .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
                state.supply_mass_flow_rate_read_count,
                state.cp_air_read_count,
                state.supply_mass_flow_rate_times_cp_air_calculation_count,
                state.mixed_air_temperature_read_count,
                state.supply_temperature_read_count,
                state.mixed_air_minus_supply_temperature_calculation_count,
                state.cooling_sensible_output_calculation_count,
                state.cooling_sensible_output_assignment_write_count,
                state
                    .witnessed_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
            ];
            counters
                .into_iter()
                .all(|counter| counter.checked_add(1).is_some())
                && state
                    .source_site_execution_count
                    .checked_add(
                        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
                            .len(),
                    )
                    .is_some()
        }
        Route::DehumidificationControlHumidistatCaseSelectedSkip => checked_pair(
            state.dehumidification_control_humidistat_case_selected_skip_count,
            state.witnessed_dehumidification_control_humidistat_case_selected_skip_count,
        ),
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => checked_pair(
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    }
}

pub(in crate::ideal_loads::calc) fn input_fits_route(
    route: Route,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput,
    >,
) -> bool {
    prepare_values(route, predecessor, active_input).is_some()
}

fn prepare_values(
    route: Route,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput,
    >,
) -> Option<PreparedValues> {
    if route != Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned {
        return active_input.is_none().then_some(PreparedValues::empty());
    }
    let input = active_input?;
    let cp_air = predecessor.cp_air_j_per_kg_k?;
    if input.supply_mass_flow_rate_kg_per_s <= 0.0
        || input.supply_mass_flow_rate_kg_per_s.is_nan()
        || !cp_air.is_finite()
    {
        return None;
    }
    let first_product = input.supply_mass_flow_rate_kg_per_s * cp_air;
    let difference = input.mixed_air_temperature_c - input.supply_temperature_c;
    let output = first_product * difference;
    Some(PreparedValues {
        supply_mass_flow_rate_kg_per_s: Some(input.supply_mass_flow_rate_kg_per_s),
        cp_air_j_per_kg_k: Some(cp_air),
        supply_mass_flow_rate_times_cp_air_w_per_k: Some(first_product),
        mixed_air_temperature_c: Some(input.mixed_air_temperature_c),
        supply_temperature_c: Some(input.supply_temperature_c),
        mixed_air_minus_supply_temperature_k: Some(difference),
        calculated_cooling_sensible_output_w: Some(output),
    })
}

fn active_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor
            .predecessor_dehumidification_control_type
            .is_some()
}

fn inactive_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    !predecessor.predecessor_cooling_body_entered
        && !predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor
            .predecessor_dehumidification_control_type
            .is_none()
}

fn checked_pair(left: usize, right: usize) -> bool {
    left.checked_add(1).is_some() && right.checked_add(1).is_some()
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[derive(Clone, Copy)]
struct PreparedValues {
    supply_mass_flow_rate_kg_per_s: Option<f64>,
    cp_air_j_per_kg_k: Option<f64>,
    supply_mass_flow_rate_times_cp_air_w_per_k: Option<f64>,
    mixed_air_temperature_c: Option<f64>,
    supply_temperature_c: Option<f64>,
    mixed_air_minus_supply_temperature_k: Option<f64>,
    calculated_cooling_sensible_output_w: Option<f64>,
}

impl PreparedValues {
    const fn empty() -> Self {
        Self {
            supply_mass_flow_rate_kg_per_s: None,
            cp_air_j_per_kg_k: None,
            supply_mass_flow_rate_times_cp_air_w_per_k: None,
            mixed_air_temperature_c: None,
            supply_temperature_c: None,
            mixed_air_minus_supply_temperature_k: None,
            calculated_cooling_sensible_output_w: None,
        }
    }
}
