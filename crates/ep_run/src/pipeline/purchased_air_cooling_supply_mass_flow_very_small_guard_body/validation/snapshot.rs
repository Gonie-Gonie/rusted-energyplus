//! Exact direct-lane shape checks for one CP328 snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    if !snapshot.cooling_body_entered {
        return !snapshot.predecessor_zero_flow_reset_body_entered
            && !snapshot.predecessor_active_guard_false_fallthrough
            && !snapshot.zero_flow_reset_body_entered
            && snapshot.body_skipped
            && !snapshot.active_guard_false_fallthrough
            && snapshot
                .predecessor_supply_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.supply_mass_flow_rate_positive_zero_assignment_performed
            && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
            && snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none()
            && usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
                == 1
            && snapshot.unit_body_entered == snapshot.non_cooling_skipped
            && predecessor.supply_mass_flow_rate_kg_per_s.is_none();
    }

    let Some(predecessor_supply) = predecessor.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let body_entered = predecessor.zero_flow_reset_body_entered;
    snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.predecessor_zero_flow_reset_body_entered == body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && snapshot.zero_flow_reset_body_entered == body_entered
        && snapshot.body_skipped != body_entered
        && snapshot.active_guard_false_fallthrough == predecessor.active_guard_false_fallthrough
        && option_has_bits(
            snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
            predecessor_supply,
        )
        && if body_entered {
            snapshot.supply_mass_flow_rate_positive_zero_assignment_performed
                && option_has_bits(snapshot.assigned_supply_mass_flow_rate_kg_per_s, 0.0)
                && option_has_bits(snapshot.resulting_supply_mass_flow_rate_kg_per_s, 0.0)
        } else {
            !snapshot.supply_mass_flow_rate_positive_zero_assignment_performed
                && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
                && option_has_bits(
                    snapshot.resulting_supply_mass_flow_rate_kg_per_s,
                    predecessor_supply,
                )
        }
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    };

    use super::*;

    fn predecessor(
        supply: f64,
        body_entered: bool,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_ems_supply_mass_flow_override_body_entered: false,
            predecessor_ems_supply_mass_flow_override_body_skipped: true,
            predecessor_ems_disabled_fallthrough: true,
            predecessor_supply_mass_flow_limit_body_entered: false,
            predecessor_supply_mass_flow_limit_body_skipped: true,
            predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(supply),
            hvac_very_small_mass_flow_read: true,
            hvac_very_small_mass_flow_source: Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE),
            hvac_very_small_mass_flow_kg_per_s: Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S),
            supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated: true,
            supply_mass_flow_rate_at_or_below_very_small_mass_flow: Some(body_entered),
            zero_flow_reset_body_entered: body_entered,
            active_guard_false_fallthrough: !body_entered,
        }
    }

    fn snapshot(
        predecessor: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
        let body_entered = predecessor.zero_flow_reset_body_entered;
        let supply = predecessor.supply_mass_flow_rate_kg_per_s;
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
            system: predecessor.system,
            parent_call_ordinal: predecessor.parent_call_ordinal,
            controlled_zone: predecessor.controlled_zone,
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_ems_supply_mass_flow_override_body_entered: false,
            predecessor_ems_supply_mass_flow_override_body_skipped: true,
            predecessor_ems_disabled_fallthrough: true,
            predecessor_supply_mass_flow_limit_body_entered: false,
            predecessor_supply_mass_flow_limit_body_skipped: true,
            predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: true,
            predecessor_zero_flow_reset_body_entered: body_entered,
            predecessor_active_guard_false_fallthrough: !body_entered,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            zero_flow_reset_body_entered: body_entered,
            body_skipped: !body_entered,
            active_guard_false_fallthrough: !body_entered,
            predecessor_supply_mass_flow_rate_kg_per_s: supply,
            supply_mass_flow_rate_positive_zero_assignment_performed: body_entered,
            assigned_supply_mass_flow_rate_kg_per_s: body_entered.then_some(0.0),
            resulting_supply_mass_flow_rate_kg_per_s: supply
                .map(|supply| if body_entered { 0.0 } else { supply }),
        }
    }

    #[test]
    fn active_shape_uses_cp327_route_and_preserves_exact_result_bits() {
        for (supply, body_entered) in [
            (-0.0, true),
            (ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, true),
            (f64::INFINITY, false),
            (f64::NAN, false),
        ] {
            let predecessor = predecessor(supply, body_entered);
            assert!(snapshot_shape(&snapshot(&predecessor), &predecessor));
        }
    }

    #[test]
    fn active_shape_rejects_negative_zero_assignment_corruption() {
        let predecessor = predecessor(-0.0, true);
        let mut snapshot = snapshot(&predecessor);
        snapshot.assigned_supply_mass_flow_rate_kg_per_s = Some(-0.0);
        snapshot.resulting_supply_mass_flow_rate_kg_per_s = Some(-0.0);
        assert!(!snapshot_shape(&snapshot, &predecessor));
    }
}
