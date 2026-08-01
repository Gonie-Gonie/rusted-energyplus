//! CP373 raw binary64 arithmetic characterization.

use super::*;

#[test]
fn cp373_both_humidistat_and_none_routes_use_raw_divide_then_add_ieee_bits() {
    for selector in [
        DehumidificationControlType::None,
        DehumidificationControlType::Humidistat,
    ] {
        for (demand, flow, zone_humidity) in [
            (-0.0, 2.0, 0.0),
            (f64::from_bits(1), f64::from_bits(1), f64::from_bits(1)),
            (f64::INFINITY, f64::INFINITY, -0.0),
            (f64::from_bits(0x7ff8_0000_0000_0373), 2.0, 0.004),
        ] {
            let predecessor = active_cp372(selector, demand);
            let mut state = State::new(predecessor.system);
            let snapshot = advance(
                &mut state,
                predecessor,
                Some(ActiveOperands {
                    supply_mass_flow_rate_kg_per_s: flow,
                    zone_node_humidity_ratio: zone_humidity,
                }),
            )
            .expect("active CP373 assignment");
            let quotient = demand / flow;
            let calculated = quotient + zone_humidity;

            assert_eq!(
                snapshot
                    .moisture_demand_derived_supply_humidity_ratio
                    .expect("quotient")
                    .to_bits(),
                quotient.to_bits()
            );
            assert_eq!(
                snapshot
                    .resulting_supply_humidity_ratio_for_humidification
                    .expect("result")
                    .to_bits(),
                calculated.to_bits()
            );
            assert_eq!(state.source_site_execution_count, 6);
            assert_eq!(
                state.supply_humidity_ratio_for_humidification_assignment_count,
                1
            );
        }
    }
}
