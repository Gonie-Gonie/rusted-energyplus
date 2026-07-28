//! Exact direct-lane shape checks for one CP327 snapshot.

use ep_runtime::{
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    predecessor_supply: Option<f64>,
) -> bool {
    if !snapshot.cooling_body_entered {
        return !snapshot.supply_mass_flow_rate_read
            && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.hvac_very_small_mass_flow_read
            && snapshot.hvac_very_small_mass_flow_source.is_none()
            && snapshot.hvac_very_small_mass_flow_kg_per_s.is_none()
            && !snapshot
                .supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated
            && snapshot
                .supply_mass_flow_rate_at_or_below_very_small_mass_flow
                .is_none()
            && !snapshot.zero_flow_reset_body_entered
            && !snapshot.active_guard_false_fallthrough
            && usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
                == 1
            && snapshot.unit_body_entered == snapshot.non_cooling_skipped
            && predecessor_supply.is_none();
    }

    let Some(predecessor_supply) = predecessor_supply else {
        return false;
    };
    let comparison = predecessor_supply <= ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S;
    snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.supply_mass_flow_rate_read
        && option_has_bits(snapshot.supply_mass_flow_rate_kg_per_s, predecessor_supply)
        && snapshot.hvac_very_small_mass_flow_read
        && snapshot.hvac_very_small_mass_flow_source
            == Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE)
        && option_has_bits(
            snapshot.hvac_very_small_mass_flow_kg_per_s,
            ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S,
        )
        && snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated
        && snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow == Some(comparison)
        && snapshot.zero_flow_reset_body_entered == comparison
        && snapshot.active_guard_false_fallthrough != comparison
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    };

    use super::*;

    fn active_snapshot(supply: f64) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
        let comparison = supply <= ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S;
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
            supply_mass_flow_rate_at_or_below_very_small_mass_flow: Some(comparison),
            zero_flow_reset_body_entered: comparison,
            active_guard_false_fallthrough: !comparison,
        }
    }

    #[test]
    fn active_shape_covers_threshold_edges_and_nan_false_fallthrough() {
        for supply in [
            -0.0,
            0.0,
            ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S,
            f64::from_bits(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S.to_bits() + 1),
            -1.0,
            f64::NAN,
        ] {
            assert!(snapshot_shape(&active_snapshot(supply), Some(supply)));
        }
    }

    #[test]
    fn active_shape_rejects_cp326_supply_bit_drift() {
        let snapshot = active_snapshot(-0.0);
        assert!(!snapshot_shape(&snapshot, Some(0.0)));
    }
}
