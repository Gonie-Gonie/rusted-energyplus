use super::super::{
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    snapshot_is_exact_source_characterization(snapshot)
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && snapshot.predecessor_ems_disabled_fallthrough == snapshot.cooling_body_entered
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    mut right: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    let float_fields_match = option_bits_match(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.hvac_very_small_mass_flow_kg_per_s,
        right.hvac_very_small_mass_flow_kg_per_s,
    );
    left.supply_mass_flow_rate_kg_per_s = None;
    right.supply_mass_flow_rate_kg_per_s = None;
    left.hvac_very_small_mass_flow_kg_per_s = None;
    right.hvac_very_small_mass_flow_kg_per_s = None;
    float_fields_match && left == right
}

pub(super) fn snapshot_is_exact_source_characterization(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER;
    let predecessor_limit_route_is_exact = if snapshot.cooling_body_entered {
        snapshot.predecessor_supply_mass_flow_limit_body_skipped
            != snapshot.predecessor_supply_mass_flow_limit_body_entered
            && snapshot.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
                == snapshot.predecessor_supply_mass_flow_limit_body_skipped
    } else {
        !snapshot.predecessor_supply_mass_flow_limit_body_entered
            && snapshot.predecessor_supply_mass_flow_limit_body_skipped
            && !snapshot.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
    };
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered;
    let cooling = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered;

    provenance
        && predecessor_limit_route_is_exact
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(cooling) == 1
        && if cooling {
            active_fields_are_exact(snapshot)
        } else {
            skipped_fields_are_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> Option<Route> {
    if !snapshot_is_exact_source_characterization(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.zero_flow_reset_body_entered {
        Some(Route::ZeroFlowResetBodyEntered)
    } else {
        Some(Route::ActiveGuardFalseFallthrough)
    }
}

fn active_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    let Some(supply) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(threshold) = snapshot.hvac_very_small_mass_flow_kg_per_s else {
        return false;
    };
    let expected = supply <= threshold;

    snapshot.supply_mass_flow_rate_read
        && snapshot.hvac_very_small_mass_flow_read
        && snapshot.hvac_very_small_mass_flow_source
            == Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE)
        && threshold.to_bits() == ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S.to_bits()
        && snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated
        && snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow == Some(expected)
        && snapshot.zero_flow_reset_body_entered == expected
        && snapshot.active_guard_false_fallthrough != expected
}

fn skipped_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.hvac_very_small_mass_flow_read
        && snapshot.hvac_very_small_mass_flow_source.is_none()
        && snapshot.hvac_very_small_mass_flow_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated
        && snapshot
            .supply_mass_flow_rate_at_or_below_very_small_mass_flow
            .is_none()
        && !snapshot.zero_flow_reset_body_entered
        && !snapshot.active_guard_false_fallthrough
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
