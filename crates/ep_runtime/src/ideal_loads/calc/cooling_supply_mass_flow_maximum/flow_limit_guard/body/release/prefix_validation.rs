use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
};

pub(super) fn limit_body_links_to_guard(
    body: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    guard: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
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
        && body.unit_off_skipped == guard.unit_off_skipped
        && body.non_cooling_skipped == guard.non_cooling_skipped
        && body.cooling_body_entered == guard.cooling_body_entered
        && body.supply_mass_flow_limit_body_entered == guard.supply_mass_flow_limit_body_entered
        && body.active_guard_false_fallthrough == guard.active_guard_false_fallthrough
}

pub(super) fn limit_body_inputs_link_to_supply_maximum_and_cache(
    body: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    maximum: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> bool {
    body.system == maximum.system
        && body.parent_call_ordinal == maximum.parent_call_ordinal
        && body.controlled_zone == maximum.controlled_zone
        && body.cooling_body_entered == maximum.cooling_body_entered
        && if body.cooling_body_entered {
            let Some(retained_supply) = maximum.resulting_supply_mass_flow_rate_kg_per_s else {
                return false;
            };
            let result_matches =
                body.resulting_supply_mass_flow_rate_kg_per_s
                    .is_some_and(|result| {
                        let expected = if body.supply_mass_flow_limit_body_entered {
                            source_min(retained_supply, maximum_cooling_air_mass_flow_rate_kg_per_s)
                        } else {
                            retained_supply
                        };
                        result.to_bits() == expected.to_bits()
                    });
            result_matches
                && if body.supply_mass_flow_limit_body_entered {
                    has_bits(
                        body.supply_mass_flow_rate_before_limit_kg_per_s,
                        Some(retained_supply),
                    ) && has_bits(
                        body.maximum_cooling_air_mass_flow_rate_kg_per_s,
                        Some(maximum_cooling_air_mass_flow_rate_kg_per_s),
                    )
                } else {
                    body.supply_mass_flow_rate_before_limit_kg_per_s.is_none()
                        && body.maximum_cooling_air_mass_flow_rate_kg_per_s.is_none()
                }
        } else {
            maximum.resulting_supply_mass_flow_rate_kg_per_s.is_none()
                && body.resulting_supply_mass_flow_rate_kg_per_s.is_none()
        }
}

fn has_bits(left: Option<f64>, right: Option<f64>) -> bool {
    left.zip(right)
        .is_some_and(|(left, right)| left.to_bits() == right.to_bits())
}

fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}
