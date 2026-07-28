use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

pub(super) fn very_small_guard_body_links_to_guard(
    body: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    guard: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    body.system == guard.system
        && body.parent_call_ordinal == guard.parent_call_ordinal
        && body.controlled_zone == guard.controlled_zone
        && body.unit_body_entered == guard.unit_body_entered
        && body.predecessor_cooling_body_entered == guard.cooling_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_entered
            == guard.predecessor_ems_supply_mass_flow_override_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_skipped
            == guard.predecessor_ems_supply_mass_flow_override_body_skipped
        && body.predecessor_ems_disabled_fallthrough == guard.predecessor_ems_disabled_fallthrough
        && body.predecessor_supply_mass_flow_limit_body_entered
            == guard.predecessor_supply_mass_flow_limit_body_entered
        && body.predecessor_supply_mass_flow_limit_body_skipped
            == guard.predecessor_supply_mass_flow_limit_body_skipped
        && body.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
            == guard.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
        && body.predecessor_zero_flow_reset_body_entered == guard.zero_flow_reset_body_entered
        && body.predecessor_active_guard_false_fallthrough == guard.active_guard_false_fallthrough
        && body.unit_off_skipped == guard.unit_off_skipped
        && body.non_cooling_skipped == guard.non_cooling_skipped
        && body.cooling_body_entered == guard.cooling_body_entered
        && body.zero_flow_reset_body_entered == guard.zero_flow_reset_body_entered
        && body.active_guard_false_fallthrough == guard.active_guard_false_fallthrough
        && option_bits_match(
            body.predecessor_supply_mass_flow_rate_kg_per_s,
            guard.supply_mass_flow_rate_kg_per_s,
        )
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
