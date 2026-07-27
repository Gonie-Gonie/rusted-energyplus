use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand as Operand,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot, PurchasedAirCalcMinimumOaPrefixSnapshot,
};

type Winner = (Operand, f64);

pub(super) fn calculation_cooling_supply_mass_flow_maximum_snapshot(
    minimum_oa: PurchasedAirCalcMinimumOaPrefixSnapshot,
    predecessor: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let outdoor_air = minimum_oa
        .working_outdoor_air_mass_flow_rate_kg_per_s
        .filter(|_| cooling);
    let cool = predecessor
        .resulting_supply_mass_flow_rate_for_cool_kg_per_s
        .filter(|_| cooling);
    let dehumidification = predecessor
        .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
        .filter(|_| cooling);
    let humidification = predecessor
        .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
        .filter(|_| cooling);
    let tree = outdoor_air
        .zip(cool)
        .zip(dehumidification)
        .zip(humidification)
        .map(
            |(((outdoor_air, cool), dehumidification), humidification)| {
                let positive_zero_outdoor_air = source_pair(
                    (Operand::PositiveZeroFloor, 0.0),
                    (Operand::OutdoorAir, outdoor_air),
                );
                let cooling_dehumidification = source_pair(
                    (Operand::Cooling, cool),
                    (Operand::Dehumidification, dehumidification),
                );
                let leading_candidate_pair =
                    source_pair(positive_zero_outdoor_air.1, cooling_dehumidification.1);
                let final_pair = source_pair(
                    leading_candidate_pair.1,
                    (Operand::Humidification, humidification),
                );
                (
                    positive_zero_outdoor_air,
                    cooling_dehumidification,
                    leading_candidate_pair,
                    final_pair,
                )
            },
        );

    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        outdoor_air_mass_flow_rate_read: cooling,
        outdoor_air_mass_flow_rate_kg_per_s: outdoor_air,
        supply_mass_flow_rate_for_cool_read: cooling,
        supply_mass_flow_rate_for_cool_kg_per_s: cool,
        supply_mass_flow_rate_for_dehumidification_read: cooling,
        supply_mass_flow_rate_for_dehumidification_kg_per_s: dehumidification,
        supply_mass_flow_rate_for_humidification_read: cooling,
        supply_mass_flow_rate_for_humidification_kg_per_s: humidification,
        positive_zero_vs_outdoor_air_comparison_evaluated: cooling,
        positive_zero_less_than_outdoor_air: tree.map(|value| value.0.0),
        positive_zero_outdoor_air_winner: tree.map(|value| value.0.1.0),
        positive_zero_outdoor_air_maximum_kg_per_s: tree.map(|value| value.0.1.1),
        cooling_vs_dehumidification_comparison_evaluated: cooling,
        cooling_less_than_dehumidification: tree.map(|value| value.1.0),
        cooling_dehumidification_winner: tree.map(|value| value.1.1.0),
        cooling_dehumidification_maximum_kg_per_s: tree.map(|value| value.1.1.1),
        leading_vs_candidate_pair_comparison_evaluated: cooling,
        leading_less_than_candidate_pair: tree.map(|value| value.2.0),
        leading_candidate_pair_winner: tree.map(|value| value.2.1.0),
        leading_candidate_pair_maximum_kg_per_s: tree.map(|value| value.2.1.1),
        leading_vs_humidification_comparison_evaluated: cooling,
        leading_less_than_humidification: tree.map(|value| value.3.0),
        final_winner: tree.map(|value| value.3.1.0),
        maximum_supply_mass_flow_rate_kg_per_s: tree.map(|value| value.3.1.1),
        supply_mass_flow_rate_assigned: cooling,
        assigned_supply_mass_flow_rate_kg_per_s: tree.map(|value| value.3.1.1),
        resulting_supply_mass_flow_rate_kg_per_s: tree.map(|value| value.3.1.1),
    }
}

fn source_pair(left: Winner, right: Winner) -> (bool, Winner) {
    let right_wins = left.1 < right.1;
    (right_wins, if right_wins { right } else { left })
}
