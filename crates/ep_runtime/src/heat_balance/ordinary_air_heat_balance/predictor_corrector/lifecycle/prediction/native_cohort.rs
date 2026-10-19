//! One complete actual-Native entry-state cohort for two production Source methods.
//! No parent predictor, psychrometric cache, sums, corrector or full-run claim.
use super::*;
use crate::heat_balance::ordinary_equipment::{OrdinaryEquipment, OrdinaryEquipmentInput, SourceEquipmentInputOwners};
use crate::heat_balance::selected_input_processor::{SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
    SourceInputStatisticsInput, SourceInputStatisticsValue};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_sizing::SelectedSizingManager;
use crate::heat_balance::startup_integrity::SourceBranchNodeConnections;
use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
use super::super::controls::SourceZoneTstatSetpoint;
use serde_json::Value;
use std::{collections::BTreeMap, error::Error as StdError, io};

include!("fixtures/native_prediction_b.rs");
type Result<T = ()> = std::result::Result<T, Box<dyn StdError>>;
fn invalid(message: &str) -> io::Error { io::Error::new(io::ErrorKind::InvalidData, message) }
fn object(value: &Value) -> Result<&serde_json::Map<String, Value>> { Ok(value.as_object().ok_or_else(|| invalid("fixture object"))?) }
fn array(value: &Value) -> Result<&Vec<Value>> { Ok(value.as_array().ok_or_else(|| invalid("fixture array"))?) }
fn member<'a>(value: &'a Value, name: &str) -> Result<&'a Value> { Ok(object(value)?.get(name).ok_or_else(|| invalid(name))?) }
fn text(value: &Value) -> Result<&str> { Ok(value.as_str().ok_or_else(|| invalid("fixture string"))?) }
fn integer(value: &Value) -> Result<i32> { Ok(i32::try_from(value.as_i64().ok_or_else(|| invalid("fixture integer"))?)?) }
fn count(value: &Value) -> Result<usize> { Ok(usize::try_from(value.as_u64().ok_or_else(|| invalid("fixture count"))?)?) }
fn snapshot(fixture: &Value, indexed: &Value) -> Result<Value> {
    // Only expand losslessly interned captured field objects; no arithmetic,
    // clock/history reconstruction, default values or expected answers added.
    let tables=member(fixture,"state_tables")?;
    Ok(Value::Object(object(indexed)?.iter().map(|(name,index)| {
        let table=array(member(tables,name)?)?;
        Ok((name.clone(),table.get(count(index)?).ok_or_else(||invalid("state table index"))?.clone()))
    }).collect::<Result<serde_json::Map<_,_>>>()?))
}
fn boolean(value: &Value) -> Result<bool> { Ok(value.as_bool().ok_or_else(|| invalid("fixture bool"))?) }
fn bool_integer(value: &Value) -> Result<bool> {
    match integer(value)? { 0 => Ok(false), 1 => Ok(true), _ => Err(invalid("Native bool transport must be0/1").into()) }
}
fn real(value: &Value) -> Result<f64> {
    let bits = text(value)?;
    if bits.len() != 16 || !bits.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {
        return Err(invalid("exact16hex native binary64").into());
    }
    Ok(f64::from_bits(u64::from_str_radix(bits, 16)?))
}
fn four(value: &Value) -> Result<[f64; 4]> {
    let values = array(value)?;
    if values.len() != 4 { return Err(invalid("actual four history slots").into()); }
    Ok([real(&values[0])?, real(&values[1])?, real(&values[2])?, real(&values[3])?])
}
fn assert_real(actual: f64, expected: &Value, context: &str, name: &str) -> Result {
    assert_eq!(actual.to_bits(), real(expected)?.to_bits(), "{context}: {name}");
    Ok(())
}
fn raw_value(value: &Value) -> Result<SourceInputStatisticsValue> {
    Ok(match value {
        Value::Null => SourceInputStatisticsValue::Null,
        Value::Bool(v) => SourceInputStatisticsValue::Bool(*v),
        Value::Number(v) => SourceInputStatisticsValue::Number(v.to_string()),
        Value::String(v) => SourceInputStatisticsValue::String(v.clone()),
        Value::Array(v) => SourceInputStatisticsValue::Array(v.iter().map(raw_value).collect::<Result<Vec<_>>>()?),
        Value::Object(v) => SourceInputStatisticsValue::Object(v.iter().map(|(k,v)| Ok((k.clone(),raw_value(v)?))).collect::<Result<BTreeMap<_,_>>>()?),
    })
}
fn json_size(value: &Value) -> usize {
    match value { Value::Null => 0, Value::Array(v) => v.len(), Value::Object(v) => v.len(), _ => 1 }
}
fn input(fixture: &Value) -> Result<SelectedInputProcessor> {
    let mut definitions = Vec::new();
    for row in array(member(fixture, "definitions")?)? {
        let row = array(row)?;
        if row.len() != 6 { return Err(invalid("query definition width").into()); }
        definitions.push(SourceObjectDefinition { object_type: text(&row[0])?.into(),
            alpha_fields: count(&row[1])?, numeric_fields: count(&row[2])?,
            alpha_extensions: count(&row[3])?, numeric_extensions: count(&row[4])?,
            extension: if row[5].is_null() { None } else { Some(text(&row[5])?.into()) } });
    }
    let mut families = BTreeMap::new();
    let mut raw = SourceInputStatisticsInput::default();
    for (family, objects) in object(member(fixture, "raw")?)? {
        let mut shapes = Vec::new();
        let mut retained = BTreeMap::new();
        for (name, fields) in object(objects)? {
            shapes.push(SourceObjectShape { object_name: name.clone(), field_sizes: object(fields)?.iter().map(|(k,v)| (k.clone(),json_size(v))).collect() });
            retained.insert(name.clone(), object(fields)?.iter().map(|(k,v)| Ok((k.clone(),raw_value(v)?))).collect::<Result<BTreeMap<_,_>>>()?);
        }
        families.insert(family.clone(), SourceInputFamily::Objects(shapes));
        raw.objects.insert(family.clone(), retained);
        raw.object_order.insert(family.clone(), array(member(member(fixture,"object_order")?,family)?)?.iter().map(|v| Ok(text(v)?.to_owned())).collect::<Result<Vec<_>>>()?);
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    input.retain_source_input_statistics(raw)?;
    Ok(input)
}
fn identity_model() -> ep_model::TypedModel {
    use ep_model::*;
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = 4;
    model.zones.push(Zone { id: ZoneId(0), name: NormalizedName::new("ZONE ONE"),
        direction_of_relative_north_deg: 0.0, origin: Point3 { x_m: 0.0, y_m: 0.0, z_m: 0.0 },
        zone_type: 1, multiplier: 1, list_multiplier: 1, list_group: None,
        ceiling_height: AutoOrNumber::AutoCalculate, volume: AutoOrNumber::AutoCalculate, floor_area: AutoOrNumber::AutoCalculate,
        inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(InsideSurfaceConvectionAlgorithm::Tarp),
        outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(OutsideSurfaceConvectionAlgorithm::Doe2),
        is_part_of_total_floor_area: true, is_nominal_controlled: true, linked_outdoor_air_node: None, spaces: vec![SpaceId(0)] });
    model.spaces.push(Space { id: SpaceId(0), name: NormalizedName::new("ZONE ONE"), zone: ZoneId(0),
        ceiling_height: AutoOrNumber::AutoCalculate, volume: AutoOrNumber::AutoCalculate, floor_area: AutoOrNumber::AutoCalculate,
        space_type: NormalizedName::new("GENERAL"), space_type_id: SpaceTypeId(0), tags: Vec::new(), origin: SpaceOrigin::AutoZoneDefault });
    model
}
fn error_state() -> SourceErrorState {
    // These ancillary diagnostic operands are never read by either method.
    SourceErrorState { output_error_header: true, version_string: String::new(), idd_version_string: String::new(),
        doing_input_processing: false, print_console_output: false, kickoff_simulation: false,
        developer_flag: false, warmup_flag: false, doing_sizing: false,
        tracking: SourceErrorTracking { ask_for_plant_check_on_abort: false, exit_during_simulations: false,
            match_counts: [0;20], abort_processing: false, ask_for_connections_report: false,
            ask_for_surfaces_report: false, total_multiplied_windows: 0, total_coincident_vertices: 0,
            total_degenerate_surfaces: 0, total_receiving_non_convex_surfaces: 0, total_casting_non_convex_surfaces: 0,
            total_severe_errors: 0, total_severe_errors_during_warmup: 0, total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(), total_warning_errors: 0, total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0 } }
}
macro_rules! seed_reals {
    ($target:expr,$raw:expr,{$($name:literal => $field:ident),* $(,)?}) => {{
        $(($target).$field = real(member($raw,$name)?)?;)*
    }};
}
macro_rules! check_reals {
    ($target:expr,$raw:expr,$context:expr,{$($name:literal => $field:ident),* $(,)?}) => {{
        $(assert_real(($target).$field,member($raw,$name)?,$context,$name)?;)*
    }};
}
fn seed_air(air: &mut SourceZoneSpaceHeatBalanceData, raw: &Value) -> Result {
    seed_reals!(air,raw,{"MAT"=>mat,"airHumRat"=>air_hum_rat,"T1"=>t1,"AirPowerCap"=>air_power_cap,
        "tempDepLoad"=>temp_dep_load,"tempIndLoad"=>temp_ind_load,"setPointLast"=>set_point_last});
    air.xmat=four(member(raw,"XMAT")?)?; air.dsxmat=four(member(raw,"DSXMAT")?)?;
    air.w_prev_zone_ts=four(member(raw,"WPrevZoneTS")?)?; air.dsw_prev_zone_ts=four(member(raw,"DSWPrevZoneTS")?)?;
    air.ztm=four(member(raw,"ZTM")?)?; air.w_prev_zone_ts_temp=four(member(raw,"WPrevZoneTSTemp")?)?;
    Ok(())
}
fn check_air(air: &SourceZoneSpaceHeatBalanceData, raw: &Value, context: &str) -> Result {
    check_reals!(air,raw,context,{"MAT"=>mat,"airHumRat"=>air_hum_rat,"T1"=>t1,"AirPowerCap"=>air_power_cap,
        "tempDepLoad"=>temp_dep_load,"tempIndLoad"=>temp_ind_load,"setPointLast"=>set_point_last});
    for (name,values) in [("XMAT",air.xmat),("DSXMAT",air.dsxmat),("WPrevZoneTS",air.w_prev_zone_ts),
        ("DSWPrevZoneTS",air.dsw_prev_zone_ts),("ZTM",air.ztm),("WPrevZoneTSTemp",air.w_prev_zone_ts_temp)] {
        let expected=four(member(raw,name)?)?;
        assert_eq!(values.map(f64::to_bits),expected.map(f64::to_bits),"{context}: {name}");
    }
    Ok(())
}
fn sequence(raw: &Value) -> Result<Option<Vec<f64>>> {
    if boolean(member(raw,"allocated")?)? {
        let values=array(member(raw,"values")?)?;
        assert_eq!(values.len(),count(member(raw,"size")?)?);
        Ok(Some(values.iter().map(real).collect::<Result<Vec<_>>>()?))
    } else {
        assert!(member(raw,"values")?.is_null());
        Ok(None)
    }
}
fn seed_demand(row: &mut SourceSystemSensibleDemand, raw: &Value) -> Result {
    row.base.num_zone_equipment=integer(member(raw,"NumZoneEquipment")?)?;
    row.base.stage_num=integer(member(raw,"StageNum")?)?;
    seed_reals!(&mut row.base,raw,{"RemainingOutputRequired"=>remaining_output_required,
        "UnadjRemainingOutputRequired"=>unadjusted_remaining_output_required,"TotalOutputRequired"=>total_output_required,
        "SupplyAirAdjustFactor"=>supply_air_adjust_factor});
    seed_reals!(row,raw,{"OutputRequiredToHeatingSP"=>output_required_to_heating_sp,"OutputRequiredToCoolingSP"=>output_required_to_cooling_sp,
        "RemainingOutputReqToHeatSP"=>remaining_output_to_heat_sp,"RemainingOutputReqToCoolSP"=>remaining_output_to_cool_sp,
        "UnadjRemainingOutputReqToHeatSP"=>unadjusted_remaining_output_to_heat_sp,"UnadjRemainingOutputReqToCoolSP"=>unadjusted_remaining_output_to_cool_sp,
        "predictedRate"=>predicted_rate,"predictedHSPRate"=>predicted_heating_sp_rate,"predictedCSPRate"=>predicted_cooling_sp_rate,
        "airSysHeatRate"=>air_sys_heat_rate,"airSysCoolRate"=>air_sys_cool_rate,"airSysHeatEnergy"=>air_sys_heat_energy,"airSysCoolEnergy"=>air_sys_cool_energy});
    row.sequenced_output_required=sequence(member(raw,"SequencedOutputRequired")?)?;
    row.sequenced_output_to_heating_sp=sequence(member(raw,"SequencedOutputRequiredToHeatingSP")?)?;
    row.sequenced_output_to_cooling_sp=sequence(member(raw,"SequencedOutputRequiredToCoolingSP")?)?;
    Ok(())
}
fn check_demand(row: &SourceSystemSensibleDemand, raw: &Value, context: &str) -> Result {
    assert_eq!(row.base.num_zone_equipment,integer(member(raw,"NumZoneEquipment")?)?,"{context}: NumZoneEquipment");
    assert_eq!(row.base.stage_num,integer(member(raw,"StageNum")?)?,"{context}: StageNum");
    check_reals!(&row.base,raw,context,{"RemainingOutputRequired"=>remaining_output_required,
        "UnadjRemainingOutputRequired"=>unadjusted_remaining_output_required,"TotalOutputRequired"=>total_output_required,
        "SupplyAirAdjustFactor"=>supply_air_adjust_factor});
    check_reals!(row,raw,context,{"OutputRequiredToHeatingSP"=>output_required_to_heating_sp,"OutputRequiredToCoolingSP"=>output_required_to_cooling_sp,
        "RemainingOutputReqToHeatSP"=>remaining_output_to_heat_sp,"RemainingOutputReqToCoolSP"=>remaining_output_to_cool_sp,
        "UnadjRemainingOutputReqToHeatSP"=>unadjusted_remaining_output_to_heat_sp,"UnadjRemainingOutputReqToCoolSP"=>unadjusted_remaining_output_to_cool_sp,
        "predictedRate"=>predicted_rate,"predictedHSPRate"=>predicted_heating_sp_rate,"predictedCSPRate"=>predicted_cooling_sp_rate,
        "airSysHeatRate"=>air_sys_heat_rate,"airSysCoolRate"=>air_sys_cool_rate,"airSysHeatEnergy"=>air_sys_heat_energy,"airSysCoolEnergy"=>air_sys_cool_energy});
    for (name,values) in [("SequencedOutputRequired",&row.sequenced_output_required),
        ("SequencedOutputRequiredToHeatingSP",&row.sequenced_output_to_heating_sp),("SequencedOutputRequiredToCoolingSP",&row.sequenced_output_to_cooling_sp)] {
        let expected=sequence(member(raw,name)?)?;
        assert_eq!(values.as_ref().map(|v|v.iter().map(|x|x.to_bits()).collect::<Vec<_>>()),
            expected.as_ref().map(|v|v.iter().map(|x|x.to_bits()).collect::<Vec<_>>()),"{context}: {name}");
    }
    Ok(())
}
fn seed_state(air: &mut SourceZoneSpaceHeatBalanceData, controller: &mut SourceZoneAirPredictorCorrector,
    owners: &mut SourceZoneAirOwners<'_,'_>, raw: &Value) -> Result {
    seed_air(air,member(raw,"air")?)?;
    let zone=owners.zones.zones.as_mut().and_then(|rows|rows.first_mut()).ok_or_else(||invalid("actual Source Zone"))?;
    let z=member(raw,"zone")?;
    assert_eq!(zone.name,text(member(z,"Name")?)?);
    assert_eq!(zone.system_zone_node_number,integer(member(z,"SystemZoneNodeNumber")?)?);
    zone.is_controlled=boolean(member(z,"IsControlled")?)?;
    zone.multiplier=integer(member(z,"Multiplier")?)?; zone.list_multiplier=integer(member(z,"ListMultiplier")?)?;
    zone.has_adjusted_return_temperature_by_ite=boolean(member(z,"HasAdjustedReturnTempByITE")?)?;
    zone.adjusted_return_temperature_by_ite=real(member(z,"AdjustedReturnTempByITE")?)?;
    let flags=member(raw,"flags")?;
    assert_eq!(integer(member(flags,"ZoneAirSolutionAlgo")?)?,0,"actual Native ThirdOrder enum");
    owners.control.zone_air_solution=SelectedZoneAirSolution::ThirdOrder;
    controller.flags.number_stage_zones=integer(member(flags,"NumStageCtrZone")?)?;
    controller.subsystems.room_air.any_non_mixing=boolean(member(flags,"anyNonMixingRoomAirModel")?)?;
    let stage=member(flags,"StageZoneLogic")?;
    controller.controls.stage_zone_logic=if boolean(member(stage,"allocated")?)? {
        Some(vec![bool_integer(member(stage,"value")?)?])
    } else { assert!(member(stage,"value")?.is_null()); None };
    let c=member(raw,"clock")?;
    // Caller-owned clock is supplied separately when constructing the borrow.
    assert_eq!(owners.clock.begin_sim_flag,boolean(member(c,"BeginSimFlag")?)?);
    owners.hvac.number_system_time_steps=integer(member(c,"NumOfSysTimeSteps")?)?;
    owners.hvac.number_system_time_steps_last_zone=integer(member(c,"NumOfSysTimeStepsLastZoneTimeStep")?)?;
    owners.hvac.time_step_sys=real(member(c,"TimeStepSys")?)?;
    let n=member(raw,"node")?;
    let node=owners.equipment.source_zone_air_node_mut(integer(member(raw,"node_ordinal")?)?)?;
    *node.temperature=real(member(n,"Temp")?)?; *node.humidity_ratio=real(member(n,"HumRat")?)?;
    *node.enthalpy=real(member(n,"Enthalpy")?)?; *node.temperature_setpoint=real(member(n,"TempSetPoint")?)?;
    let f=member(raw,"fan")?; let set=member(f,"setpoints")?;
    owners.fan.temp_tstat_air=Some(vec![real(member(f,"TempTstatAir")?)?]);
    owners.fan.thermostat.load_correction_factor=Some(vec![real(member(f,"LoadCorrectionFactor")?)?]);
    owners.fan.thermostat.temp_control_type=Some(vec![integer(member(f,"TempControlType")?)?]);
    owners.fan.thermostat.zone_tstat_setpoints=Some(vec![SourceZoneTstatSetpoint { setpt:real(member(set,"setpt")?)?,
        setpt_lo:real(member(set,"setptLo")?)?,setpt_hi:real(member(set,"setptHi")?)?,..Default::default() }]);
    let d=member(raw,"demand")?;
    controller.demand.setback=Some(vec![bool_integer(member(d,"Setback")?)?]);
    controller.demand.deadband_or_setback=Some(vec![bool_integer(member(d,"DeadBandOrSetback")?)?]);
    controller.demand.current_deadband_or_setback=Some(vec![bool_integer(member(d,"CurDeadBandOrSetback")?)?]);
    let row=controller.demand.zones.as_mut().and_then(|rows|rows.first_mut()).ok_or_else(||invalid("actual Source Demand"))?;
    seed_demand(row,member(d,"sensible")?)?;
    Ok(())
}
fn check_state(air: &SourceZoneSpaceHeatBalanceData, controller: &SourceZoneAirPredictorCorrector,
    owners: &mut SourceZoneAirOwners<'_,'_>, raw: &Value, context: &str) -> Result {
    check_air(air,member(raw,"air")?,context)?;
    let node=owners.equipment.source_zone_air_node_mut(integer(member(raw,"node_ordinal")?)?)?;
    let n=member(raw,"node")?;
    assert_real(*node.temperature,member(n,"Temp")?,context,"Node.Temp")?;
    assert_real(*node.humidity_ratio,member(n,"HumRat")?,context,"Node.HumRat")?;
    assert_real(*node.enthalpy,member(n,"Enthalpy")?,context,"Node.Enthalpy")?;
    assert_real(*node.temperature_setpoint,member(n,"TempSetPoint")?,context,"Node.TempSetPoint")?;
    let f=member(raw,"fan")?;let set=member(f,"setpoints")?;
    let temp=owners.fan.temp_tstat_air.as_ref().and_then(|rows|rows.first()).ok_or_else(||invalid("TempTstatAir"))?;
    assert_real(*temp,member(f,"TempTstatAir")?,context,"TempTstatAir")?;
    let correction=owners.fan.thermostat.load_correction_factor.as_ref().and_then(|rows|rows.first()).ok_or_else(||invalid("LoadCorrectionFactor"))?;
    assert_real(*correction,member(f,"LoadCorrectionFactor")?,context,"LoadCorrectionFactor")?;
    let kind=owners.fan.thermostat.temp_control_type.as_ref().and_then(|rows|rows.first()).ok_or_else(||invalid("TempControlType"))?;
    assert_eq!(*kind,integer(member(f,"TempControlType")?)?,"{context}: TempControlType");
    let values=owners.fan.thermostat.zone_tstat_setpoints.as_ref().and_then(|rows|rows.first()).ok_or_else(||invalid("zoneTstatSetpts"))?;
    check_reals!(values,set,context,{"setpt"=>setpt,"setptLo"=>setpt_lo,"setptHi"=>setpt_hi});
    let d=member(raw,"demand")?;
    assert_eq!(controller.demand.setback.as_deref(),Some(&[bool_integer(member(d,"Setback")?)?][..]),"{context}: Setback");
    assert_eq!(controller.demand.deadband_or_setback.as_deref(),Some(&[bool_integer(member(d,"DeadBandOrSetback")?)?][..]),"{context}: DeadBand");
    assert_eq!(controller.demand.current_deadband_or_setback.as_deref(),Some(&[bool_integer(member(d,"CurDeadBandOrSetback")?)?][..]),"{context}: CurDeadBand");
    check_demand(controller.demand.zones.as_ref().and_then(|rows|rows.first()).ok_or_else(||invalid("actual Demand row"))?,member(d,"sensible")?,context)?;
    Ok(())
}

#[test]
fn all_native_b_prediction_entries_match_source_rollback_and_sensible_bits() -> Result {
    let fixture:Value=serde_json::from_str(NATIVE_PREDICTION_COHORT_JSON)?;
    let cases=array(member(&fixture,"cases")?)?;
    assert_eq!(cases.len(),1168);
    let mut input=input(&fixture)?;
    let inventory=object(member(&fixture,"raw")?)?.iter().map(|(family,objects)|Ok((family.as_str(),object(objects)?.len()))).collect::<Result<Vec<_>>>()?;
    let equipment_input=OrdinaryEquipmentInput::from_source_family_inventory(inventory,4)?;
    let model=identity_model();
    let mut equipment=OrdinaryEquipment::new_for_source(&model,equipment_input)?;
    let mut globals=SelectedHeatBalanceGlobals::default(); globals.num_of_zones=1;globals.num_spaces=1;
    let mut zones=SelectedZoneInputOwners::default();
    zones.zones=Some(vec![SelectedZoneData { name:"ZONE ONE".into(),typed_zone_id:Some(ep_model::ZoneId(0)),space_indexes:vec![1],num_spaces:1,..Default::default() }]);
    let mut schedules=SelectedScheduleManager::default(); schedules.init_constant_state(4)?;
    let mut sizing=SelectedSizingManager::default();let mut branch=SourceBranchNodeConnections::default();
    let mut gains=SelectedInternalHeatGains::default();
    equipment.get_zone_equipment_source(&mut input,equipment_input,false,false,&globals,&mut gains,
        SourceEquipmentInputOwners { zones:zones.zones.as_deref_mut().ok_or_else(||invalid("Zone allocation"))?,
            schedules:&mut schedules,sizing:&mut sizing,branch:Some(&mut branch),source_is_epjson:false })?;
    let mut controller=SourceZoneAirPredictorCorrector::default();
    let space_controls=SelectedHeatBalanceSpaceControls::default();
    controller.demand.allocate_source_demands(&globals,&space_controls)?;
    let mut fan=crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays::default();
    let mut control=SelectedHeatBalanceInputState::default();
    let geometry=SurfaceGeometryInputState::default();let surface_arrays=SourceSurfaceHeatBalanceArrays::default();
    let environment=WeatherEnvironmentState::default();
    let faults=crate::heat_balance::source_initialization::SourceFaultsManager::default();
    let availability=crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability::default();
    let mut psychrometrics=SourcePsychrometricAccess::default();let mut shortcut=SourceInputProcessorShortCut::default();
    let mut state=error_state();let mut console=Vec::new();
    let mut backend=SourceErrorBackend { state:&mut state,error_stream:None,console:&mut console,out_unit1:None,out_unit2:None,
        f_message:None,message_callback:None,error_callback:None,sqlite:None };
    let source_is_epjson=false;
    let mut shortened=0;
    let mut groups=BTreeMap::new();
    let mut first_physical_address=None;
    for (index,case) in cases.iter().enumerate() {
        assert_eq!(count(member(case,"parent")?)?,index+1,"all actual parent IDs retained");
        let context=format!("Native parent{}",index+1);
        let entry=snapshot(&fixture,member(case,"update_entry")?)?;let clock_row=member(&entry,"clock")?;
        let mut clock=WeatherGlobalState::default(); clock.begin_sim_flag=boolean(member(clock_row,"BeginSimFlag")?)?;
        let mut hvac=SelectedHvacGlobals::default();
        let mut air=SourceZoneSpaceHeatBalanceData::default();
        let mut owners=SourceZoneAirOwners { zones:&mut zones,equipment:&mut equipment,geometry:&geometry,surface_arrays:&surface_arrays,
            fan:&mut fan,gains:&gains,control:&mut control,space_controls:&space_controls,globals:&globals,clock:&clock,
            environment:&environment,hvac:&mut hvac,input:&mut input,shortcut:&mut shortcut,source_is_epjson:&source_is_epjson,
            faults:&faults,schedules:&mut schedules,availability:&availability,psychrometrics:&mut psychrometrics,backend:&mut backend };
        seed_state(&mut air,&mut controller,&mut owners,&entry)?;
        let ordinal=integer(member(&entry,"node_ordinal")?)?;
        let address={let node=owners.equipment.source_zone_air_node_mut(ordinal)?;std::ptr::from_ref(&*node.temperature) as usize};
        if let Some(first)=first_physical_address { assert_eq!(address,first,"same physical Node retained across cohort"); }
        else { first_physical_address=Some(address); }
        let args=member(case,"update_arguments")?;
        let shorten=bool_integer(member(args,"ShortenTimeStepSys")?)?;
        let use_history=bool_integer(member(args,"UseZoneTimeStepHistory")?)?;
        if shorten { shortened+=1; }
        *groups.entry((boolean(member(clock_row,"WarmupFlag")?)?,boolean(member(clock_row,"KickOffSimulation")?)?,shorten,use_history)).or_insert(0_usize)+=1;
        air.update_source_temperatures(1,shorten,use_history,real(member(args,"PriorTimeStep")?)?,&controller,&mut owners)?;
        // Assert rollback/history outputs BEFORE supplying later coefficient inputs.
        check_state(&air,&controller,&mut owners,&snapshot(&fixture,member(case,"update_exit")?)?,&format!("{context} update exit"))?;
        let sensible_entry=snapshot(&fixture,member(case,"sensible_entry")?)?;
        let coefficients=member(&sensible_entry,"air")?;
        // Only the real parent-computed method-entry coefficient boundary is supplied.
        air.air_power_cap=real(member(coefficients,"AirPowerCap")?)?;
        air.temp_dep_load=real(member(coefficients,"tempDepLoad")?)?;
        air.temp_ind_load=real(member(coefficients,"tempIndLoad")?)?;
        check_state(&air,&controller,&mut owners,&sensible_entry,&format!("{context} same-state sensible entry"))?;
        air.calculate_source_predicted_sensible(real(member(member(case,"sensible_arguments")?,"RAFNFrac")?)?,1,&mut controller,&mut owners)?;
        check_state(&air,&controller,&mut owners,&snapshot(&fixture,member(case,"sensible_exit")?)?,&format!("{context} sensible exit"))?;
        let after={let node=owners.equipment.source_zone_air_node_mut(ordinal)?;std::ptr::from_ref(&*node.temperature) as usize};
        assert_eq!(after,address,"{context}: both methods keep same actual Node row");
    }
    assert_eq!(shortened,77);
    assert_eq!(groups,BTreeMap::from([((false,false,false,false),57),((false,false,false,true),96),
        ((false,false,true,false),10),((true,false,false,false),359),((true,false,false,true),576),
        ((true,false,true,false),67),((true,true,false,true),3)]));
    assert!(console.is_empty(),"the two bounded methods did not emit diagnostics");
    Ok(())
}
