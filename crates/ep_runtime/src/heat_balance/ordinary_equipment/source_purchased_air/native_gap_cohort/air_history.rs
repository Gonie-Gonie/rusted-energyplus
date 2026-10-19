//! Full production history methods on the retained genuine P1 factory owners.
//! No exit/history restore; only named external air and pressure inputs are written.
use super::*;
use hb::ordinary_air_heat_balance::SourceZoneSpaceHeatBalanceData;
use hb::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirOwners;

fn air(row: &SourceZoneSpaceHeatBalanceData) -> Value {
    let mut out = Map::new();
    out.insert("MAT".into(), number(row.mat));
    out.insert("MRT".into(), number(row.mrt));
    out.insert("ZTAV".into(), number(row.ztav));
    out.insert("ZT".into(), number(row.zt));
    out.insert("ZTAVComf".into(), number(row.ztav_comf));
    out.insert("XMPT".into(), number(row.xmpt));
    out.insert(
        "XMAT".into(),
        Value::Array(row.xmat.iter().copied().map(number).collect()),
    );
    out.insert(
        "DSXMAT".into(),
        Value::Array(row.dsxmat.iter().copied().map(number).collect()),
    );
    out.insert("TMX".into(), number(row.tmx));
    out.insert("TM2".into(), number(row.tm2));
    out.insert("T1".into(), number(row.t1));
    out.insert("airHumRat".into(), number(row.air_hum_rat));
    out.insert("airHumRatAvg".into(), number(row.air_hum_rat_avg));
    out.insert("airHumRatTemp".into(), number(row.air_hum_rat_temp));
    out.insert("airHumRatAvgComf".into(), number(row.air_hum_rat_avg_comf));
    out.insert(
        "WPrevZoneTS".into(),
        Value::Array(row.w_prev_zone_ts.iter().copied().map(number).collect()),
    );
    out.insert(
        "DSWPrevZoneTS".into(),
        Value::Array(row.dsw_prev_zone_ts.iter().copied().map(number).collect()),
    );
    out.insert("WTimeMinusP".into(), number(row.w_time_minus_p));
    out.insert("WMX".into(), number(row.wmx));
    out.insert("WM2".into(), number(row.wm2));
    out.insert("W1".into(), number(row.w1));
    out.insert(
        "ZTM".into(),
        Value::Array(row.ztm.iter().copied().map(number).collect()),
    );
    out.insert(
        "WPrevZoneTSTemp".into(),
        Value::Array(
            row.w_prev_zone_ts_temp
                .iter()
                .copied()
                .map(number)
                .collect(),
        ),
    );
    out.insert("SumIntGain".into(), number(row.sum_int_gain));
    out.insert("SumHA".into(), number(row.sum_ha));
    out.insert("SumHATsurf".into(), number(row.sum_ha_tsurf));
    out.insert("SumHATref".into(), number(row.sum_ha_tref));
    out.insert("SumMCp".into(), number(row.sum_m_cp));
    out.insert("SumMCpT".into(), number(row.sum_m_cp_t));
    out.insert("SumSysMCp".into(), number(row.sum_sys_m_cp));
    out.insert("SumSysMCpT".into(), number(row.sum_sys_m_cp_t));
    out.insert(
        "SumIntGainExceptPeople".into(),
        number(row.sum_int_gain_except_people),
    );
    out.insert("SumHmAW".into(), number(row.sum_hm_aw));
    out.insert("SumHmARa".into(), number(row.sum_hm_a_ra));
    out.insert("SumHmARaW".into(), number(row.sum_hm_a_ra_w));
    out.insert("SumHmARaZ".into(), number(row.sum_hm_a_ra_z));
    out.insert("TempDepCoef".into(), number(row.temp_dep_coef));
    out.insert("TempIndCoef".into(), number(row.temp_ind_coef));
    out.insert("TempHistoryTerm".into(), number(row.temp_history_term));
    out.insert("MCPI".into(), number(row.mcpi));
    out.insert("MCPTI".into(), number(row.mcpti));
    out.insert("MCPV".into(), number(row.mcpv));
    out.insert("MCPTV".into(), number(row.mcptv));
    out.insert("MCPM".into(), number(row.mcpm));
    out.insert("MCPTM".into(), number(row.mcptm));
    out.insert("MCPE".into(), number(row.mcpe));
    out.insert("EAMFL".into(), number(row.eamfl));
    out.insert("EAMFLxHumRat".into(), number(row.eamf_lx_hum_rat));
    out.insert("MCPTE".into(), number(row.mcpte));
    out.insert("MCPC".into(), number(row.mcpc));
    out.insert("CTMFL".into(), number(row.ctmfl));
    out.insert("MCPTC".into(), number(row.mcptc));
    out.insert("ThermChimAMFL".into(), number(row.therm_chim_amfl));
    out.insert("MCPTThermChim".into(), number(row.mcpt_therm_chim));
    out.insert("MCPThermChim".into(), number(row.mcp_therm_chim));
    out.insert("latentGain".into(), number(row.latent_gain));
    out.insert(
        "latentGainExceptPeople".into(),
        number(row.latent_gain_except_people),
    );
    out.insert("OAMFL".into(), number(row.oamfl));
    out.insert("VAMFL".into(), number(row.vamfl));
    out.insert(
        "NonAirSystemResponse".into(),
        number(row.non_air_system_response),
    );
    out.insert("SysDepZoneLoads".into(), number(row.sys_dep_zone_loads));
    out.insert(
        "SysDepZoneLoadsLagged".into(),
        number(row.sys_dep_zone_loads_lagged),
    );
    out.insert("MDotCPOA".into(), number(row.m_dot_cpoa));
    out.insert("MDotOA".into(), number(row.m_dot_oa));
    out.insert("MixingMAT".into(), number(row.mixing_mat));
    out.insert("MixingHumRat".into(), number(row.mixing_hum_rat));
    out.insert(
        "MixingMassFlowZone".into(),
        number(row.mixing_mass_flow_zone),
    );
    out.insert(
        "MixingMassFlowXHumRat".into(),
        number(row.mixing_mass_flow_x_hum_rat),
    );
    out.insert("setPointLast".into(), number(row.set_point_last));
    out.insert("tempIndLoad".into(), number(row.temp_ind_load));
    out.insert("tempDepLoad".into(), number(row.temp_dep_load));
    out.insert("airRelHum".into(), number(row.air_rel_hum));
    out.insert("AirPowerCap".into(), number(row.air_power_cap));
    out.insert(
        "hmThermalMassMultErrIndex".into(),
        json!(row.hm_thermal_mass_mult_err_index),
    );
    out.insert("address".into(), pointer(row));
    Value::Object(out)
}
fn state(factory: &Factory) -> Result<Value> {
    let mut zones = shape(&factory.air.zones);
    zones["items"] = Value::Array(
        factory
            .air
            .zones
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|row| air(&row.air))
            .collect(),
    );
    let mut spaces = shape(&factory.air.spaces);
    spaces["items"] = Value::Array(
        factory
            .air
            .spaces
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|row| air(&row.air))
            .collect(),
    );
    let room = &factory.controller.subsystems.room_air;
    let mut models = shape(&room.air_models);
    models["items"]=Value::Array(room.air_models.as_deref().unwrap_or(&[]).iter().map(|row|json!({
        "address":pointer(row),"Name":row.name,"ZoneName":row.zone_name,"ZonePtr":row.zone_ptr,
        "AirModel":row.model,"TempCoupleScheme":row.coupling_scheme,"SimAirModel":row.simulate_air_model})).collect());
    let zone_metadata=factory.zones.zones.as_deref().ok_or_else(||invalid("real getter Zone metadata"))?.iter().enumerate().map(|(at,row)|json!({
        "address":pointer(row),"ordinal":at+1,"Name":row.name,"Multiplier":row.multiplier,"ListMultiplier":row.list_multiplier,"spaceIndexes":row.space_indexes})).collect::<Vec<_>>();
    let algorithm = match factory.control.zone_air_solution {
        hb::selected_heat_balance_input::SelectedZoneAirSolution::ThirdOrder => 0,
        hb::selected_heat_balance_input::SelectedZoneAirSolution::Euler => 2,
    }; // Pinned DataHeatBalance.hh100-107; these inputs genuinely select ThirdOrder0.
    Ok(
        json!({"P1_owners":snapshot(factory)?,"zoneHeatBalance":zones,"spaceHeatBalance":spaces,"AirModel":models,
        "anyNonMixingRoomAirModel":room.any_non_mixing,"GetAirModelData":room.get_air_model_data,
        "doSpaceHeatBalance":factory.space_controls.current,"doSpaceHeatBalanceSimulation":factory.space_controls.simulation,
        "doSpaceHeatBalanceSizing":factory.space_controls.sizing,"ZoneAirSolutionAlgo":algorithm,
        "NumOfZones":factory.globals.num_of_zones,"Zone_metadata":zone_metadata,
        "RoomAir_owner_address":pointer(room),"predictor_owner_address":pointer(&factory.controller)}),
    )
}
fn primary_inputs(factory: &mut Factory, input: &Value) -> Result {
    let input = object(input)?;
    for key in input.keys() {
        if !matches!(
            key.as_str(),
            "MAT" | "ZT" | "ZTAV" | "airHumRatAvg" | "airHumRatTemp" | "pressure"
        ) {
            return Err(invalid("history accepts only real external caller inputs, no histories or computed answers").into());
        }
    }
    let row = factory
        .air
        .zones
        .as_mut()
        .and_then(|rows| rows.get_mut(index(factory.zone).ok()?))
        .ok_or_else(|| invalid("same actual history zone"))?;
    if let Some(v) = input.get("MAT") {
        row.air.mat = real(v)?;
    }
    if let Some(v) = input.get("ZT") {
        row.air.zt = real(v)?;
    }
    if let Some(v) = input.get("ZTAV") {
        row.air.ztav = real(v)?;
    }
    if let Some(v) = input.get("airHumRatAvg") {
        row.air.air_hum_rat_avg = real(v)?;
    }
    if let Some(v) = input.get("airHumRatTemp") {
        row.air.air_hum_rat_temp = real(v)?;
    }
    if let Some(v) = input.get("pressure") {
        factory.weather.environment.current_weather.out_baro_press = real(v)?;
    }
    Ok(())
}
fn call(factory: &mut Factory, operation: &str) -> Result {
    let mut backend = SourceErrorBackend {
        state: &mut factory.error_state,
        error_stream: Some(&mut factory.error),
        console: &mut factory.console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let source_is_epjson = false;
    let mut owners = SourceZoneAirOwners {
        zones: &mut factory.zones,
        equipment: &mut factory.equipment,
        geometry: &factory.ancillary.surface_geometry_input,
        surface_arrays: &factory.surface.arrays,
        fan: &mut factory.fan,
        gains: &factory.ancillary.selected_internal_heat_gains,
        control: &mut factory.control,
        space_controls: &factory.space_controls,
        globals: &factory.globals,
        clock: &factory.weather.global,
        environment: &factory.weather.environment,
        hvac: &mut factory.startup.hvac_globals,
        input: &mut factory.input,
        shortcut: &mut factory.shortcut,
        source_is_epjson: &source_is_epjson,
        faults: &factory.ancillary.source_faults_manager,
        schedules: &mut factory.schedules,
        availability: &factory.hvac_manager.ancillary.availability,
        psychrometrics: &mut factory.psych,
        backend: &mut backend,
    };
    match operation {
        "push_zone" => factory
            .air
            .push_source_zone_timestep_histories(&factory.controller, &mut owners)?,
        "push_system" => factory
            .air
            .push_source_system_timestep_histories(&factory.controller, &owners)?,
        "revert_zone" => factory
            .air
            .revert_source_zone_timestep_histories(&factory.controller, &owners)?,
        _ => return Err(invalid("unknown real full history operation").into()),
    }
    Ok(())
}
#[derive(Default)]
struct HistoryIdentities {
    pairs: BTreeMap<(String, String), String>,
    raw_bits: usize,
}
impl HistoryIdentities {
    fn check(&mut self, source: &Value, native: &Value, path: &str) -> Result {
        match (source, native) {
            (Value::Object(a), Value::Object(b)) => {
                assert_eq!(
                    a.keys().collect::<Vec<_>>(),
                    b.keys().collect::<Vec<_>>(),
                    "{path} full observed keyset"
                );
                if a.len() == 1 && a.contains_key("value_bits") {
                    self.raw_bits += 1;
                }
                for (k, v) in a {
                    let expected = b
                        .get(k)
                        .ok_or_else(|| invalid("same observed history key"))?;
                    if matches!(
                        k.as_str(),
                        "address"
                            | "Which"
                            | "owner_address"
                            | "state_address"
                            | "RoomAir_owner_address"
                            | "predictor_owner_address"
                    ) {
                        match (v, expected) {
                            (Value::Null, Value::Null) => {}
                            (Value::String(actual), Value::String(expected)) => {
                                assert!(
                                    actual.starts_with("0x") && expected.starts_with("0x"),
                                    "{path} true pointer transport"
                                );
                                let kind = if k != "address" {
                                    k.as_str()
                                } else if path.contains("zoneHeatBalance.items") {
                                    "ZoneAir"
                                } else if path.contains("spaceHeatBalance.items") {
                                    "SpaceAir"
                                } else if path.contains("AirModel.items") {
                                    "AirModel"
                                } else if path.contains("Zone_metadata") {
                                    "Zone"
                                } else if path.contains(".Node.items") {
                                    "Node"
                                } else if path.contains(".PurchAir.items") {
                                    "Unit"
                                } else if path.contains(".availability.items") {
                                    "Availability"
                                } else if path.ends_with(".config") {
                                    "Config"
                                } else if path.ends_with(".demand") {
                                    "Demand"
                                } else if path.ends_with(".moisture") {
                                    "Moisture"
                                } else {
                                    "Schedule"
                                };
                                let key = (kind.to_owned(), expected.clone());
                                if let Some(old) = self.pairs.get(&key) {
                                    assert_eq!(old, actual, "{path} same real persistent owner");
                                } else {
                                    assert!(
                                        !self.pairs.iter().any(
                                            |((old_kind, old_native), old_source)| old_kind == kind
                                                && old_native != expected
                                                && old_source == actual
                                        ),
                                        "{path} distinct logical rows remain distinct"
                                    );
                                    self.pairs.insert(key, actual.clone());
                                }
                            }
                            _ => return Err(invalid("actual history pointer/null shape").into()),
                        }
                    } else {
                        self.check(v, expected, &format!("{path}.{k}"))?;
                    }
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                assert_eq!(a.len(), b.len(), "{path} whole array extent");
                for (i, (a, b)) in a.iter().zip(b).enumerate() {
                    self.check(a, b, &format!("{path}[{i}]"))?;
                }
            }
            _ => assert_eq!(source, native, "{path} strict observed value"),
        }
        Ok(())
    }
}
pub(super) fn run(fixture: &Value) -> Result<usize> {
    let ingress: Value = serde_json::from_str(input::SOURCE_PURCHASED_AIR_INPUT_JSON)?;
    let definitions = member(&ingress, "definitions")?;
    let factories = array(member(&ingress, "factories")?)?;
    let named = member(member(fixture, "named_inputs")?, "air_history")?;
    let expected = member(
        member(member(fixture, "native_output")?, "groups")?,
        "air_history",
    )?;
    assert_eq!(
        member(named, "schema")?,
        "src06-air-history-original-core-input.v1"
    );
    assert_eq!(
        member(expected, "schema")?,
        "src06-air-history-original-core-output.v1"
    );
    let cases = array(member(named, "cases")?)?;
    let outputs = array(member(expected, "cases")?)?;
    assert_eq!(cases.len(), 4);
    assert_eq!(outputs.len(), 4);
    let mut calls = 0;
    let mut raw_bits = 0;
    for (case, expected) in cases.iter().zip(outputs) {
        assert_eq!(member(case, "name")?, member(expected, "name")?);
        assert_eq!(member(case, "origin")?, member(expected, "origin")?);
        let native_factory = member(case, "factory")?;
        let factory_name = text(member(native_factory, "name")?)?;
        let definition = factories
            .iter()
            .find(|f| f.get("name").and_then(Value::as_str) == Some(factory_name))
            .ok_or_else(|| invalid("actual history IDF factory ingress"))?;
        assert_eq!(
            member(definition, "steps")?,
            member(native_factory, "steps")?
        );
        let mut factory = Box::new(Factory::new(definitions, definition)?);
        let mut ids = HistoryIdentities::default();
        let name = text(member(case, "name")?)?;
        assert!(factory.controller.subsystems.room_air.air_models.is_none());
        ids.check(
            &state(&factory)?,
            member(expected, "before_real_RoomAir_getter")?,
            &format!("{name}.before_RoomAir"),
        )?;
        factory
            .controller
            .subsystems
            .room_air
            .get_source_model_parameters(
                &factory.zones,
                &factory.globals,
                &factory.space_controls,
                &factory.input,
                &mut factory.shortcut,
                &mut factory.eio,
            )?;
        assert!(
            !factory.controller.subsystems.room_air.any_non_mixing
                && !factory.space_controls.current
        );
        assert_eq!(
            factory.control.zone_air_solution,
            hb::selected_heat_balance_input::SelectedZoneAirSolution::ThirdOrder
        );
        assert!(
            factory
                .controller
                .subsystems
                .room_air
                .air_models
                .as_deref()
                .ok_or_else(|| invalid("genuine RoomAir getter allocation"))?
                .iter()
                .all(|row| row.model == 1)
        );
        ids.check(
            &state(&factory)?,
            member(expected, "after_real_RoomAir_getter")?,
            &format!("{name}.after_RoomAir"),
        )?;
        let steps = array(member(case, "steps")?)?;
        let exits = array(member(expected, "steps")?)?;
        assert_eq!(steps.len(), 10);
        assert_eq!(exits.len(), 10);
        for (at, (step, expected)) in steps.iter().zip(exits).enumerate() {
            assert_eq!(member(step, "name")?, member(expected, "name")?);
            assert_eq!(member(step, "operation")?, member(expected, "operation")?);
            assert_eq!(member(step, "inputs")?, member(expected, "named_inputs")?);
            assert_eq!(count(member(expected, "ordinal")?)?, at + 1);
            primary_inputs(&mut factory, member(step, "inputs")?)?;
            ids.check(
                &state(&factory)?,
                member(expected, "entry")?,
                &format!("{name}.step{}.entry", at + 1),
            )?;
            call(&mut factory, text(member(step, "operation")?)?)?;
            ids.check(
                &state(&factory)?,
                member(expected, "exit")?,
                &format!("{name}.step{}.exit", at + 1),
            )?;
            calls += 1;
        }
        raw_bits += ids.raw_bits;
    }
    assert_eq!(calls, 40);
    assert!(raw_bits > 0);
    println!(
        "Air-history genuine persistent calls={calls}; strict raw-bit state observations={raw_bits}; four getter factories; no history/exit restore; private cache unobserved"
    );
    Ok(calls)
}
impl OrdinaryEquipment {
    pub(crate) fn run_native_gap_air_history_cohort(fixture: &Value) -> Result<usize> {
        run(fixture)
    }
}

// Test-only sibling access: the accepted history observer/caller/checker above
// stays whole and remains the only implementation of these shared operations.
pub(super) fn multiplier_state(factory: &Factory) -> Result<Value> { state(factory) }
pub(super) fn multiplier_history_call(factory: &mut Factory, operation: &str) -> Result { call(factory, operation) }
#[derive(Default)]
pub(super) struct MultiplierObservations(HistoryIdentities);
impl MultiplierObservations {
    pub(super) fn check(&mut self, source: &Value, native: &Value, path: &str) -> Result { self.0.check(source, native, path) }
    pub(super) fn raw_bits(&self) -> usize { self.0.raw_bits }
}
