use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

pub(super) fn very_small_guard_links_to_limit_body(
    guard: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    body: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    guard.system == body.system
        && guard.parent_call_ordinal == body.parent_call_ordinal
        && guard.controlled_zone == body.controlled_zone
        && guard.unit_body_entered == body.unit_body_entered
        && guard.predecessor_cooling_body_entered == body.cooling_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_entered
            == body.predecessor_ems_supply_mass_flow_override_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_skipped
            == body.predecessor_ems_supply_mass_flow_override_body_skipped
        && guard.predecessor_ems_disabled_fallthrough == body.predecessor_ems_disabled_fallthrough
        && guard.predecessor_supply_mass_flow_limit_body_entered
            == body.supply_mass_flow_limit_body_entered
        && guard.predecessor_supply_mass_flow_limit_body_skipped == body.body_skipped
        && guard.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
            == body.active_guard_false_fallthrough
        && guard.unit_off_skipped == body.unit_off_skipped
        && guard.non_cooling_skipped == body.non_cooling_skipped
        && guard.cooling_body_entered == body.cooling_body_entered
        && if guard.cooling_body_entered {
            option_bits_match(
                guard.supply_mass_flow_rate_kg_per_s,
                body.resulting_supply_mass_flow_rate_kg_per_s,
            )
        } else {
            guard.supply_mass_flow_rate_kg_per_s.is_none()
                && body.resulting_supply_mass_flow_rate_kg_per_s.is_none()
        }
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
