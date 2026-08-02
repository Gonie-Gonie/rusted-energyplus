use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_lifecycle_summary,
};

#[test]
fn binding_places_cp400_after_cp399_before_unchanged_numerical_coupling() {
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
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;
        let active = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
            active
        );
        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
            active
        );
        assert_eq!(
            snapshot.cp399_retained_supply_humidity_ratio_state_owned,
            predecessor.resulting_supply_humidity_ratio.is_some()
        );
        assert_eq!(
            snapshot.cp399_retained_supply_enthalpy_state_owned,
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        );
        assert_eq!(
            snapshot.cp399_retained_supply_temperature_state_owned,
            predecessor.resulting_supply_temperature_c.is_some()
        );
        for local in [
            snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
            snapshot.cp329_supply_mass_flow_rate_bit_corroborated,
            snapshot.supply_mass_flow_rate_read,
            snapshot.cp399_retained_cp_air_owned_read,
            snapshot.cp_air_read,
            snapshot.supply_mass_flow_rate_times_cp_air_calculated,
            snapshot.cp329_retained_mixed_air_temperature_owned_read,
            snapshot.mixed_air_temperature_read,
            snapshot.cp399_retained_supply_temperature_owned_read,
            snapshot.supply_temperature_read,
            snapshot.mixed_air_minus_supply_temperature_calculated,
            snapshot.cooling_sensible_output_calculated,
            snapshot.cooling_sensible_output_assigned,
        ] {
            assert_eq!(local, active);
        }

        if active {
            let supply_mass_flow_rate = output
                .calculation_cooling_supply_mass_flow_positive_guard
                .supply_mass_flow_rate_kg_per_s
                .expect("active CP400 must read the CP330-owned mass flow");
            let corroborating_flow = output
                .calculation_cooling_mixed_air_call
                .supply_mass_flow_rate_kg_per_s
                .expect("active CP400 must corroborate the CP329 flow");
            let cp_air = predecessor
                .cp_air_j_per_kg_k
                .expect("active CP400 must read the CP399-owned CpAir");
            let mixed_air_temperature = output
                .calculation_cooling_mixed_air_call
                .mixed_air_temperature_c
                .expect("active CP400 must read the CP329-owned mixed-air temperature");
            let supply_temperature = predecessor
                .resulting_supply_temperature_c
                .expect("active CP400 must read the CP399-owned supply temperature");
            let first_product = supply_mass_flow_rate * cp_air;
            let difference = mixed_air_temperature - supply_temperature;
            let sensible_output = first_product * difference;

            for (left, right) in [
                (
                    snapshot.supply_mass_flow_rate_kg_per_s,
                    Some(supply_mass_flow_rate),
                ),
                (Some(corroborating_flow), Some(supply_mass_flow_rate)),
                (snapshot.cp_air_j_per_kg_k, Some(cp_air)),
                (
                    snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
                    Some(first_product),
                ),
                (
                    snapshot.mixed_air_temperature_c,
                    Some(mixed_air_temperature),
                ),
                (snapshot.supply_temperature_c, Some(supply_temperature)),
                (
                    snapshot.mixed_air_minus_supply_temperature_k,
                    Some(difference),
                ),
                (
                    snapshot.calculated_cooling_sensible_output_w,
                    Some(sensible_output),
                ),
                (snapshot.cooling_sensible_output_w, Some(sensible_output)),
            ] {
                assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
            }
        } else {
            for value in [
                snapshot.supply_mass_flow_rate_kg_per_s,
                snapshot.cp_air_j_per_kg_k,
                snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
                snapshot.mixed_air_temperature_c,
                snapshot.supply_temperature_c,
                snapshot.mixed_air_minus_supply_temperature_k,
                snapshot.calculated_cooling_sensible_output_w,
                snapshot.cooling_sensible_output_w,
            ] {
                assert!(value.is_none());
            }
        }
        assert_lossless_carriers(snapshot, predecessor);

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP400 lifecycle");
        let assignments = usize::from(active);
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1 - assignments);
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count,
            assignments
        );
        assert_eq!(lifecycle.state.source_site_execution_count, 8 * assignments);
        let routes = lifecycle.state.predecessor_route_counts;
        let humidity_owner_count = [18, 19, 22, 23, 26, 28]
            .into_iter()
            .map(|index| routes[index])
            .sum::<usize>();
        let enthalpy_owner_count = [5, 8, 11, 14]
            .into_iter()
            .chain(17..=29)
            .map(|index| routes[index])
            .sum::<usize>();
        let temperature_owner_count = (3..=29).map(|index| routes[index]).sum::<usize>();
        assert_eq!(
            lifecycle
                .state
                .cp399_supply_humidity_ratio_state_owner_count,
            humidity_owner_count
        );
        assert_eq!(
            lifecycle
                .state
                .unchanged_supply_humidity_ratio_preservation_count,
            humidity_owner_count
        );
        assert_eq!(
            lifecycle.state.cp399_supply_enthalpy_state_owner_count,
            enthalpy_owner_count
        );
        assert_eq!(
            lifecycle.state.unchanged_supply_enthalpy_preservation_count,
            enthalpy_owner_count
        );
        assert_eq!(
            lifecycle.state.cp399_supply_temperature_state_owner_count,
            temperature_owner_count
        );
        assert_eq!(
            lifecycle
                .state
                .unchanged_supply_temperature_preservation_count,
            temperature_owner_count
        );
        for count in [
            lifecycle.state.supply_mass_flow_rate_owned_read_count,
            lifecycle
                .state
                .supply_mass_flow_rate_bit_corroboration_count,
            lifecycle.state.supply_mass_flow_rate_read_count,
            lifecycle.state.cp_air_owned_read_count,
            lifecycle.state.cp_air_read_count,
            lifecycle
                .state
                .supply_mass_flow_rate_times_cp_air_calculation_count,
            lifecycle.state.mixed_air_temperature_owned_read_count,
            lifecycle.state.mixed_air_temperature_read_count,
            lifecycle.state.supply_temperature_owned_read_count,
            lifecycle.state.supply_temperature_read_count,
            lifecycle
                .state
                .mixed_air_minus_supply_temperature_calculation_count,
            lifecycle.state.cooling_sensible_output_calculation_count,
            lifecycle
                .state
                .cooling_sensible_output_assignment_write_count,
        ] {
            assert_eq!(count, assignments);
        }
        saw_active |= active;
        saw_inactive |= !active;
    }
    assert!(saw_active, "fixture set must execute CP400");
    assert!(saw_inactive, "fixture set must also skip CP400");
}

fn assert_lossless_carriers(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot,
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
            predecessor.predecessor_cp398_resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
            predecessor.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp398_resulting_supply_temperature_c,
            predecessor.predecessor_cp398_resulting_supply_temperature_c,
        ),
        (
            snapshot.predecessor_mixed_air_humidity_ratio,
            predecessor.mixed_air_humidity_ratio,
        ),
        (
            snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
            predecessor.psychrometric_cp_air_result_j_per_kg_k,
        ),
        (
            snapshot.predecessor_cp_air_j_per_kg_k,
            predecessor.cp_air_j_per_kg_k,
        ),
        (
            snapshot.predecessor_cp399_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp399_resulting_supply_temperature_c,
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
