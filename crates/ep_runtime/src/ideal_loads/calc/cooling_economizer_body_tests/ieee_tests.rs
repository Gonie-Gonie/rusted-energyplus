use super::*;

#[test]
fn objexx_min_max_preserve_nan_and_signed_zero_operand_order() {
    let (nan_snapshot, _) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
        zone_cooling_setpoint_load_w: f64::NAN,
        cooling_limit: IdealLoadsLimit::LimitFlowRate,
        maximum_cooling_air_mass_flow_rate_kg_per_s: 2.0,
        outdoor_air_mass_flow_rate_kg_per_s: -1.0,
        ..base_input()
    });
    assert!(
        nan_snapshot
            .calculated_supply_mass_flow_rate_kg_per_s
            .is_some_and(f64::is_nan)
    );
    assert!(
        nan_snapshot
            .nonnegative_supply_mass_flow_rate_kg_per_s
            .is_some_and(f64::is_nan),
        "Objexx max(NaN, 0) returns its first operand"
    );
    assert!(nan_snapshot.maximum_flow_clamp_body_entered);
    assert!(nan_snapshot.supply_mass_flow_rate_for_clamp_read);
    assert!(
        nan_snapshot
            .supply_mass_flow_rate_for_clamp_kg_per_s
            .is_some_and(f64::is_nan)
    );
    assert!(nan_snapshot.inner_max_evaluated);
    assert!(nan_snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read);
    assert_bits(
        nan_snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s,
        2.0,
    );
    assert_bits(nan_snapshot.clamped_supply_mass_flow_rate_kg_per_s, 2.0);
    assert!(nan_snapshot.outer_min_evaluated);
    assert!(nan_snapshot.clamped_supply_mass_flow_rate_assigned);
    assert_bits(nan_snapshot.resulting_supply_mass_flow_rate_kg_per_s, 2.0);

    let (negative_zero_snapshot, _) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
        zone_cooling_setpoint_load_w: 0.0,
        cooling_limit: IdealLoadsLimit::LimitFlowRate,
        maximum_cooling_air_mass_flow_rate_kg_per_s: 2.0,
        outdoor_air_mass_flow_rate_kg_per_s: -1.0,
        ..base_input()
    });
    assert_bits(
        negative_zero_snapshot.calculated_supply_mass_flow_rate_kg_per_s,
        -0.0,
    );
    assert_bits(
        negative_zero_snapshot.nonnegative_supply_mass_flow_rate_kg_per_s,
        -0.0,
    );
    assert_bits(
        negative_zero_snapshot.clamped_supply_mass_flow_rate_kg_per_s,
        -0.0,
    );
    assert_bits(
        negative_zero_snapshot.resulting_supply_mass_flow_rate_kg_per_s,
        -0.0,
    );

    let (negative_snapshot, _) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
        zone_cooling_setpoint_load_w: 1.0,
        cooling_limit: IdealLoadsLimit::LimitFlowRate,
        maximum_cooling_air_mass_flow_rate_kg_per_s: 2.0,
        outdoor_air_mass_flow_rate_kg_per_s: -1.0,
        ..base_input()
    });
    assert!(
        negative_snapshot
            .calculated_supply_mass_flow_rate_kg_per_s
            .is_some_and(|value| value < 0.0)
    );
    assert_bits(
        negative_snapshot.nonnegative_supply_mass_flow_rate_kg_per_s,
        0.0,
    );
    assert_bits(
        negative_snapshot.clamped_supply_mass_flow_rate_kg_per_s,
        0.0,
    );

    let (infinity_snapshot, _) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
        zone_cooling_setpoint_load_w: f64::NEG_INFINITY,
        cooling_limit: IdealLoadsLimit::LimitFlowRate,
        maximum_cooling_air_mass_flow_rate_kg_per_s: 2.0,
        outdoor_air_mass_flow_rate_kg_per_s: -1.0,
        ..base_input()
    });
    assert_bits(
        infinity_snapshot.calculated_supply_mass_flow_rate_kg_per_s,
        f64::INFINITY,
    );
    assert_bits(
        infinity_snapshot.clamped_supply_mass_flow_rate_kg_per_s,
        2.0,
    );

    for maximum in [-0.0, f64::NAN, f64::NEG_INFINITY] {
        let (snapshot, state) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
            zone_cooling_setpoint_load_w: f64::NAN,
            cooling_limit: IdealLoadsLimit::LimitFlowRate,
            maximum_cooling_air_mass_flow_rate_kg_per_s: maximum,
            ..base_input()
        });
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_positive,
            Some(false)
        );
        assert!(!snapshot.maximum_flow_clamp_body_entered);
        assert!(!snapshot.supply_mass_flow_rate_for_clamp_read);
        assert!(snapshot.supply_mass_flow_rate_for_clamp_kg_per_s.is_none());
        assert!(!snapshot.inner_max_evaluated);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read);
        assert!(
            snapshot
                .maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s
                .is_none()
        );
        assert!(!snapshot.outer_min_evaluated);
        assert!(!snapshot.clamped_supply_mass_flow_rate_assigned);
        assert!(!snapshot.supply_mass_flow_rate_clamped);
        assert!(
            snapshot
                .resulting_supply_mass_flow_rate_kg_per_s
                .is_some_and(f64::is_nan)
        );
        assert_eq!(
            state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count,
            0
        );
        assert_eq!(state.maximum_flow_clamp_body_entry_count, 0);
        assert_eq!(state.supply_mass_flow_rate_for_clamp_read_count, 0);
        assert_eq!(state.inner_max_evaluation_count, 0);
        assert_eq!(state.outer_min_evaluation_count, 0);
        assert_eq!(state.clamped_supply_mass_flow_rate_assignment_count, 0);
        assert_eq!(state.supply_mass_flow_rate_clamp_count, 0);
    }
}
