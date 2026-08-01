//! Native IEEE multiplication and non-operand retention tests.

use super::*;

#[test]
fn signed_zero_nan_and_infinity_are_preserved_without_local_gates() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        99.0,
        50_000.0,
        0.008,
    );
    for ratio in [
        -0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0042),
    ] {
        let system = fixtures::selected_system(chain, ratio);
        let snapshot = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
            &system,
            chain.cp387,
            Some(chain.cp384),
            Some(chain.cp385),
        )
        .expect("native IEEE CP388");
        let total = chain.cp384.resulting_cooling_total_output_w.expect("owner");
        assert_eq!(
            snapshot.cooling_sensible_heat_ratio.map(f64::to_bits),
            Some(ratio.to_bits()),
        );
        assert_eq!(
            snapshot
                .calculated_cooling_sensible_output_w
                .map(f64::to_bits),
            Some((total * ratio).to_bits()),
        );
        assert_eq!(
            snapshot.cooling_sensible_output_w.map(f64::to_bits),
            snapshot
                .calculated_cooling_sensible_output_w
                .map(f64::to_bits),
        );
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact(snapshot));
    }
}

#[test]
fn cp387_cp_air_and_cp385_enthalpy_are_retained_but_not_operands() {
    let low = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        99.0,
        50_000.0,
        0.004,
    );
    let high = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        99.0,
        60_000.0,
        0.020,
    );
    let low_snapshot = characterize(low, 0.71);
    let high_snapshot = characterize(high, 0.71);

    assert_ne!(
        low_snapshot.predecessor_cp_air_j_per_kg_k.map(f64::to_bits),
        high_snapshot
            .predecessor_cp_air_j_per_kg_k
            .map(f64::to_bits),
    );
    assert_ne!(
        low_snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        high_snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
    );
    assert_eq!(
        low_snapshot.cooling_sensible_output_w.map(f64::to_bits),
        high_snapshot.cooling_sensible_output_w.map(f64::to_bits),
    );
    for (chain, snapshot) in [(low, low_snapshot), (high, high_snapshot)] {
        assert_eq!(
            snapshot.predecessor_cp_air_j_per_kg_k.map(f64::to_bits),
            chain.cp387.cp_air_j_per_kg_k.map(f64::to_bits),
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            chain
                .cp385
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
        );
    }
}

fn characterize(
    chain: fixtures::Chain,
    ratio: f64,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot{
    let system = fixtures::selected_system(chain, ratio);
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
        &system,
        chain.cp387,
        Some(chain.cp384),
        Some(chain.cp385),
    )
    .expect("private CP388")
}
