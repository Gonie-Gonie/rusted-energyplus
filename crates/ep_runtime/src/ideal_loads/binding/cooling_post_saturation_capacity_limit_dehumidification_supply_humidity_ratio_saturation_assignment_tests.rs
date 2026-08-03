use super::*;
use crate::{
    ideal_loads::{
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentError as Cp412Error,
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment as advance_cp412,
        cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
    },
    psychrometrics::energyplus_psy_w_fn_tdb_rh_pb,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case_with_pressure;

#[test]
fn binding_places_cp412_after_cp411_and_inactive_routes_ignore_unused_pressure() {
    let mut saw_assignment = false;
    let mut saw_skip = false;

    for (limit, humidity_ratio, availability, capacity) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 500.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 1.0e-100),
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
    ] {
        let (_runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
            limit,
            humidity_ratio,
            availability,
            capacity,
        );
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);

        let active = predecessor
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed;
        assert_eq!(
            snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
            active,
        );
        assert_eq!(
            snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
            active,
        );

        for (left, right) in [
            (
                snapshot.predecessor_cp411_resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp411_resulting_supply_temperature_c,
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

        if active {
            let temperature = predecessor
                .resulting_supply_temperature_c
                .expect("active CP412 binding route requires the CP411 supply temperature");
            let pressure = snapshot
                .outdoor_barometric_pressure_pa
                .expect("active CP412 binding route requires the current barometric pressure");
            let saturation = energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
            assert!(snapshot.cp411_retained_supply_temperature_owned_read);
            assert!(snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read);
            assert!(snapshot.environment_outdoor_barometric_pressure_owned_read);
            assert!(
                snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            );
            assert!(snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated);
            assert_eq!(
                snapshot
                    .supply_temperature_for_saturation_humidity_ratio_c
                    .map(f64::to_bits),
                Some(temperature.to_bits()),
            );
            for value in [
                snapshot.saturation_supply_humidity_ratio,
                snapshot.assigned_saturation_supply_humidity_ratio,
                snapshot.resulting_saturation_supply_humidity_ratio,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(saturation.to_bits()));
            }
            saw_assignment = true;
        } else {
            assert!(!snapshot.cp411_retained_supply_temperature_owned_read);
            assert!(!snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read);
            assert!(!snapshot.environment_outdoor_barometric_pressure_owned_read);
            assert!(
                !snapshot
                    .environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            );
            assert!(!snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated);
            assert!(
                snapshot
                    .supply_temperature_for_saturation_humidity_ratio_c
                    .is_none()
            );
            assert!(snapshot.outdoor_barometric_pressure_pa.is_none());
            assert!(snapshot.saturation_supply_humidity_ratio.is_none());
            assert!(snapshot.assigned_saturation_supply_humidity_ratio.is_none());
            assert!(
                snapshot
                    .resulting_saturation_supply_humidity_ratio
                    .is_none()
            );
            saw_skip = true;
        }

        assert!(
            output
                .coupling
                .purchased_air
                .supply_node_update
                .humidity_ratio
                .is_finite(),
            "CP412 evidence must not replace the numerical PurchasedAir output",
        );
    }

    assert!(saw_assignment);
    assert!(saw_skip);
}

#[test]
fn binding_cp412_inactive_u_n_and_p_ignore_invalid_unused_pressure() {
    for (cooling_limit, maximum_capacity_w, independent_load_w, availability) in [
        (IdealLoadsLimit::NoLimit, None, 3_000.0, 0.0),
        (IdealLoadsLimit::NoLimit, None, 0.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), 3_000.0, 1.0),
    ] {
        for pressure in [f64::NAN, 0.0, -1.0, f64::INFINITY] {
            let (_runtime, output) = run_case_with_pressure(
                cooling_limit,
                maximum_capacity_w,
                independent_load_w,
                availability,
                Some(pressure),
            )
            .expect("inactive CP412 route must ignore invalid unused pressure");
            let snapshot = output
                .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;

            assert!(
                !snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed
            );
            assert!(!snapshot.cp411_retained_supply_temperature_owned_read);
            assert!(!snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read);
            assert!(!snapshot.environment_outdoor_barometric_pressure_owned_read);
            assert!(
                !snapshot
                    .environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
            );
            assert!(!snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated);
            assert!(!snapshot.local_saturation_supply_humidity_ratio_assignment_performed);
            for value in [
                snapshot.supply_temperature_for_saturation_humidity_ratio_c,
                snapshot.outdoor_barometric_pressure_pa,
                snapshot.saturation_supply_humidity_ratio,
                snapshot.assigned_saturation_supply_humidity_ratio,
                snapshot.resulting_saturation_supply_humidity_ratio,
            ] {
                assert!(value.is_none());
            }
        }
    }
}

#[test]
fn binding_cp412_active_invalid_pressure_is_transactional_and_fail_closed() {
    let (runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
        IdealLoadsLimit::LimitCapacity,
        0.020,
        1.0,
        500.0,
    );
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment;
    assert!(
        output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed
    );

    let (model, _) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(500.0));
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.humidification_control_type = HumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = 1.0;
    });
    let system = &model.typed.ideal_loads_air_systems[0];

    for pressure in [f64::NAN, 0.0, -1.0, f64::INFINITY] {
        let mut rejected = runtime.clone();
        let before = rejected.clone();
        assert_eq!(
            advance_cp412(&mut rejected, system, predecessor, pressure),
            Err(Cp412Error::BarometricPressureOutsideDirectSubset {
                system: system.id,
                bits: pressure.to_bits(),
            })
        );
        assert_eq!(rejected, before);
    }
}
