use super::*;

#[test]
fn public_condition_rejects_entry_prefix_and_initialization_corruption_transactionally() {
    let (runtime, system, predecessor) = release_fixture();

    for stage in 0..3 {
        let mut corruption = runtime.clone();
        let unit = corruption.units.get_mut(&SYSTEM).expect("selected unit");
        match stage {
            0 => unit.calc_entry.latest = None,
            1 => unit.calc_minimum_oa_prefix.latest = None,
            2 => unit.calc_cooling_entry_gate.latest = None,
            _ => unreachable!(),
        }
        assert_runtime_invariant_without_mutation(corruption, &system, predecessor);
    }

    let wrong_system = IdealLoadsAirSystemId(SYSTEM.0 + 1);
    for stage in 0..3 {
        let mut corruption = runtime.clone();
        let unit = corruption.units.get_mut(&SYSTEM).expect("selected unit");
        match stage {
            0 => {
                unit.calc_entry
                    .latest
                    .as_mut()
                    .expect("retained CP310")
                    .system = wrong_system;
            }
            1 => {
                unit.calc_minimum_oa_prefix
                    .latest
                    .as_mut()
                    .expect("retained CP311")
                    .system = wrong_system;
            }
            2 => {
                unit.calc_cooling_entry_gate
                    .latest
                    .as_mut()
                    .expect("retained CP312")
                    .system = wrong_system;
            }
            _ => unreachable!(),
        }
        assert_runtime_invariant_without_mutation(corruption, &system, predecessor);
    }

    let wrong_zone = ZoneId(ZONE.0 + 1);
    for stage in 0..3 {
        let mut corruption = runtime.clone();
        let unit = corruption.units.get_mut(&SYSTEM).expect("selected unit");
        match stage {
            0 => {
                unit.calc_entry
                    .latest
                    .as_mut()
                    .expect("retained CP310")
                    .controlled_zone = wrong_zone;
            }
            1 => {
                unit.calc_minimum_oa_prefix
                    .latest
                    .as_mut()
                    .expect("retained CP311")
                    .controlled_zone = wrong_zone;
            }
            2 => {
                unit.calc_cooling_entry_gate
                    .latest
                    .as_mut()
                    .expect("retained CP312")
                    .controlled_zone = wrong_zone;
            }
            _ => unreachable!(),
        }
        assert_runtime_invariant_without_mutation(corruption, &system, predecessor);
    }

    for stage in 0..3 {
        let mut corruption = runtime.clone();
        let unit = corruption.units.get_mut(&SYSTEM).expect("selected unit");
        match stage {
            0 => {
                unit.calc_entry
                    .latest
                    .as_mut()
                    .expect("retained CP310")
                    .force_off_applied = true;
            }
            1 => {
                unit.calc_minimum_oa_prefix
                    .latest
                    .as_mut()
                    .expect("retained CP311")
                    .ems_override_applied = true;
            }
            2 => {
                unit.calc_cooling_entry_gate
                    .latest
                    .as_mut()
                    .expect("retained CP312")
                    .single_heat_blocked = true;
            }
            _ => unreachable!(),
        }
        assert_runtime_invariant_without_mutation(corruption, &system, predecessor);
    }

    for stage in 0..3 {
        let mut corruption = runtime.clone();
        let unit = corruption.units.get_mut(&SYSTEM).expect("selected unit");
        match stage {
            0 => unit.calc_entry.reset_count += 1,
            1 => unit.calc_minimum_oa_prefix.psychrometric_call_count += 1,
            2 => unit.calc_cooling_entry_gate.single_heat_block_count += 1,
            _ => unreachable!(),
        }
        assert_runtime_invariant_without_mutation(corruption, &system, predecessor);
    }

    for field in 0..10 {
        let mut corruption = runtime.clone();
        let unit = corruption.units.get_mut(&SYSTEM).expect("selected unit");
        match field {
            0 => unit.topology_plan = None,
            1 => unit.equipment_list = None,
            2 => unit.supply_node = None,
            3 => unit.recirculation_node = None,
            4 => unit.controlled_zone = None,
            5 => unit.one_time_latched = false,
            6 => unit.standard_air_density_kg_per_m3 = Some(f64::INFINITY),
            7 => unit.maximum_cooling_air_mass_flow_rate_kg_per_s = f64::INFINITY,
            8 => unit.maximum_heating_air_mass_flow_rate_kg_per_s = 1.0,
            9 => unit.maximum_cooling_air_mass_flow_rate_kg_per_s = 1.0,
            _ => unreachable!(),
        }
        assert_initialization_not_ready_without_mutation(corruption, &system, predecessor);
    }

    let mut wrong_model_system = system.clone();
    wrong_model_system.id = wrong_system;
    let mut unchanged = runtime;
    let before = unchanged.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_condition(
            &mut unchanged,
            &wrong_model_system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingEconomizerConditionError::SystemIdentityMismatch {
                expected: SYSTEM,
                actual: wrong_system,
            },
        ),
    );
    assert_eq!(unchanged, before);
}

fn assert_runtime_invariant_without_mutation(
    mut runtime: PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, system, predecessor,),
        Err(
            PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation {
                system: SYSTEM,
            },
        ),
    );
    assert_eq!(runtime, before);
}

fn assert_initialization_not_ready_without_mutation(
    mut runtime: PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, system, predecessor,),
        Err(
            PurchasedAirCalcCoolingEconomizerConditionError::InitializationNotReady {
                system: SYSTEM,
            },
        ),
    );
    assert_eq!(runtime, before);
}
