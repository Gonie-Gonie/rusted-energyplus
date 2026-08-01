//! Native IEEE grouping, signed-zero, infinity, NaN, and non-operand tests.

use super::*;

#[test]
fn raw_three_step_grouping_preserves_ieee_bits() {
    for (ratio, flow) in [
        (-0.0, 1.0),
        (f64::INFINITY, 1.0),
        (f64::NEG_INFINITY, f64::INFINITY),
        (f64::from_bits(0x7ff8_0000_0000_0042), 1.0),
        (0.7, f64::MAX),
    ] {
        let chain = fixtures::chain(
            3,
            1,
            true,
            Some(D::ConstantSensibleHeatRatio),
            1,
            ratio,
            18.0,
            flow,
        );
        let snapshot = characterize(chain);
        let mixed = snapshot.mixed_air_temperature_c.expect("mixed");
        let sensible = snapshot.cooling_sensible_output_w.expect("sensible");
        let cp_air = snapshot.cp_air_j_per_kg_k.expect("CpAir");
        let denominator = cp_air * flow;
        let drop = sensible / denominator;
        let expected = mixed - drop;
        assert_eq!(
            snapshot
                .cp_air_times_supply_mass_flow_rate_w_per_k
                .map(f64::to_bits),
            Some(denominator.to_bits())
        );
        assert_eq!(
            snapshot
                .cooling_sensible_output_over_air_capacity_rate_k
                .map(f64::to_bits),
            Some(drop.to_bits())
        );
        assert_eq!(
            snapshot.calculated_supply_temperature_c.map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert_eq!(
            snapshot.assigned_supply_temperature_c.map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_is_exact(snapshot));
    }
}

#[test]
fn cp385_enthalpy_and_cp379_preexisting_temperature_are_carried_nonoperands() {
    let low = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.71,
        -40.0,
        1.0,
    );
    let high = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.71,
        80.0,
        1.0,
    );
    let low_snapshot = characterize(low);
    let high_snapshot = characterize(high);

    assert_ne!(
        low_snapshot
            .preexisting_supply_temperature_c
            .map(f64::to_bits),
        high_snapshot
            .preexisting_supply_temperature_c
            .map(f64::to_bits)
    );
    assert_eq!(
        low_snapshot
            .resulting_supply_temperature_c
            .map(f64::to_bits),
        high_snapshot
            .resulting_supply_temperature_c
            .map(f64::to_bits)
    );
    assert_eq!(
        low_snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        low.cp388
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits)
    );
    assert_eq!(
        high_snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        high.cp388
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits)
    );
}

fn characterize(
    chain: fixtures::Chain,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot{
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_characterization(
        chain.cp388,
        chain.cp379,
        Some(chain.formula_owners.mixed_air_owner),
        Some(chain.formula_owners.supply_mass_flow_owner),
        Some(chain.cp387),
    )
    .expect("private CP389")
}
