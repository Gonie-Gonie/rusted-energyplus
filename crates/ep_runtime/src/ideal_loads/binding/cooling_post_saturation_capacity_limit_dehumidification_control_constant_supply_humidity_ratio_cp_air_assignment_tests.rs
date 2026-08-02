use super::*;
use crate::{
    ideal_loads::{
        cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

#[test]
fn binding_places_cp399_after_cp398_before_unchanged_numerical_coupling() {
    let mut saw_active = false;
    let mut saw_inactive = false;
    for (limit, humidity_ratio, availability, capacity) in [
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.008, 1.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 500.0),
    ] {
        let (runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
            limit,
            humidity_ratio,
            availability,
            capacity,
        );
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
        let active = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
            active
        );
        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
            active
        );
        for local in [
            snapshot.mixed_air_humidity_ratio_read,
            snapshot.psychrometric_cp_air_evaluated,
            snapshot.cp_air_assigned,
        ] {
            assert_eq!(local, active);
        }

        if active {
            let owner = output
                .calculation_cooling_mixed_air_call
                .mixed_air_humidity_ratio
                .expect("active CP399 must retain the CP329-owned operand");
            let expected = energyplus_psy_cp_air_fn_w(owner);
            assert_eq!(
                snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
                Some(owner.to_bits())
            );
            assert_eq!(
                snapshot
                    .psychrometric_cp_air_result_j_per_kg_k
                    .map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert_eq!(
                snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
                Some(expected.to_bits())
            );
        } else {
            assert!(snapshot.mixed_air_humidity_ratio.is_none());
            assert!(snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none());
            assert!(snapshot.cp_air_j_per_kg_k.is_none());
        }
        assert_lossless_carriers(snapshot, predecessor);

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP399 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(!active)
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count,
            usize::from(active)
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            3 * usize::from(active)
        );
        assert_eq!(
            lifecycle.state.mixed_air_humidity_ratio_read_count,
            usize::from(active)
        );
        assert_eq!(
            lifecycle.state.psychrometric_cp_air_evaluation_count,
            usize::from(active)
        );
        assert_eq!(
            lifecycle.state.cp_air_assignment_write_count,
            usize::from(active)
        );
        saw_active |= active;
        saw_inactive |= !active;
    }
    assert!(saw_active, "fixture set must execute CP399");
    assert!(saw_inactive, "fixture set must also skip CP399");
}

fn assert_lossless_carriers(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot,
) {
    for (left, right) in [
        (
            snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
            predecessor.predecessor_cp397_resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
            predecessor.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp397_resulting_supply_temperature_c,
            predecessor.predecessor_cp397_resulting_supply_temperature_c,
        ),
        (
            snapshot.predecessor_cp398_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp398_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        ),
        (
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        ),
    ] {
        assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
    }
}
