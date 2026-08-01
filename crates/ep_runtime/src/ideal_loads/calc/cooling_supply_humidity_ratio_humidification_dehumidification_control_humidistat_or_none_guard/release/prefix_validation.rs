//! CP370 lineage and selected dehumidification-control provenance validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state,
};
use super::snapshot_validation::snapshots_match_exact;
use crate::ideal_loads::calc::{
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent,
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshots_match_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release,
};

pub(super) fn guard_links_to_predecessor(
    guard: Snapshot,
    predecessor: Predecessor,
    control: DehumidificationControlType,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state(
        &mut state,
        predecessor,
        control,
    )
    .is_some_and(|expected| snapshots_match_exact(expected, guard))
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == predecessor.system
        && cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshots_match_exact(
            retained,
            predecessor,
        )
        && cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshots_match_exact(
            witness,
            predecessor,
        )
        && cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(
            predecessor,
        )
        && crate::ideal_loads::calc::completed_direct_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn dehumidification_control_type_provenance_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let owner = system.dehumidification_control_type;
    let Some(cp346) = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
        .latest
    else {
        return false;
    };
    let Some(cp346_witness) = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    let active = predecessor.predecessor_positive_supply_mass_flow_body_entered;
    let inherited_owner_matches = !active
        || (predecessor.predecessor_dehumidification_control_type == Some(owner)
            && predecessor.dehumidification_control_none_case_completed_skip
            && !predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            && !predecessor.dehumidification_control_humidistat_case_completed_skip
            && !predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip);
    owner == DehumidificationControlType::None
        && PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER
            == &PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER[6..11]
        && cp346.system == predecessor.system
        && cp346.parent_call_ordinal == predecessor.parent_call_ordinal
        && cp346.controlled_zone == predecessor.controlled_zone
        && cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(cp346)
        && completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent(
            runtime,
            unit,
            system,
            cp346,
            Some(cp346_witness),
        )
        && inherited_owner_matches
}