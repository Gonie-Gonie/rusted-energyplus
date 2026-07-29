use super::{H, operands, predecessor};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState as State,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state as advance,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn division_then_addition_preserves_staged_binary64_evidence() {
    let cases = [
        (-0.002, 0.5, 0.008),
        (-0.0, 1.0, -0.0),
        (0.0, -0.0, 0.0),
        (1.0, f64::INFINITY, -0.0),
        (f64::MAX, f64::MIN_POSITIVE, f64::NEG_INFINITY),
        (f64::INFINITY, f64::INFINITY, 0.007),
        (f64::from_bits(0x7ff8_0000_0000_0042), 1.0, 0.008),
        (1.0, f64::from_bits(0x7ff8_0000_0000_0043), 0.008),
        (1.0, 2.0, f64::from_bits(0x7ff8_0000_0000_0044)),
    ];
    for (demand, flow, zone_humidity) in cases {
        let quotient = demand / flow;
        let calculated = quotient + zone_humidity;
        let snapshot = advance(
            &mut State::new(IdealLoadsAirSystemId(7)),
            predecessor(H, 1, demand),
            operands(H, flow, zone_humidity),
        )
        .expect("private-H CP360");
        assert_bits(
            snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            demand,
        );
        assert_bits(snapshot.supply_mass_flow_rate_kg_per_s, flow);
        assert_bits(
            snapshot.moisture_demand_derived_supply_humidity_ratio,
            quotient,
        );
        assert_bits(snapshot.zone_node_humidity_ratio, zone_humidity);
        assert_bits(
            snapshot.calculated_supply_humidity_ratio_for_dehumidification,
            calculated,
        );
        assert_bits(
            snapshot.assigned_supply_humidity_ratio_for_dehumidification,
            calculated,
        );
        assert_bits(
            snapshot.resulting_supply_humidity_ratio_for_dehumidification,
            calculated,
        );
    }
}

#[test]
fn predecessor_nan_payload_mismatch_is_rejected_without_mutation() {
    let demand = f64::from_bits(0x7ff8_0000_0000_0042);
    let mut predecessor = predecessor(H, 1, demand);
    predecessor.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s =
        Some(f64::from_bits(0x7ff8_0000_0000_0043));
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor, operands(H, 1.0, 0.008)).is_none());
    assert_eq!(state, before);
}

fn assert_bits(actual: Option<f64>, expected: f64) {
    assert_eq!(
        actual.expect("numeric evidence").to_bits(),
        expected.to_bits()
    );
}
