//! One actual-Native cohort: original predictor children plus whole corrector.
//! No parent predictor/full-engine or Native property cache/recurrence trace claim.
use super::super::controls::SourceZoneTstatSetpoint;
use super::*;
use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
use crate::heat_balance::ordinary_equipment::{
    OrdinaryEquipment, OrdinaryEquipmentInput, SourceEquipmentInputOwners,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceInputStatisticsInput, SourceInputStatisticsValue,
    SourceObjectDefinition, SourceObjectShape,
};
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_sizing::SelectedSizingManager;
use crate::heat_balance::startup_integrity::SourceBranchNodeConnections;
use serde_json::Value;
use std::{collections::BTreeMap, error::Error as StdError, io};

include!("fixtures/native_prediction_b.rs");
type Result<T = ()> = std::result::Result<T, Box<dyn StdError>>;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn object(value: &Value) -> Result<&serde_json::Map<String, Value>> {
    Ok(value.as_object().ok_or_else(|| invalid("fixture object"))?)
}
fn array(value: &Value) -> Result<&Vec<Value>> {
    Ok(value.as_array().ok_or_else(|| invalid("fixture array"))?)
}
fn member<'a>(value: &'a Value, name: &str) -> Result<&'a Value> {
    Ok(object(value)?.get(name).ok_or_else(|| invalid(name))?)
}
fn text(value: &Value) -> Result<&str> {
    Ok(value.as_str().ok_or_else(|| invalid("fixture string"))?)
}
fn integer(value: &Value) -> Result<i32> {
    Ok(i32::try_from(
        value.as_i64().ok_or_else(|| invalid("fixture integer"))?,
    )?)
}
fn count(value: &Value) -> Result<usize> {
    Ok(usize::try_from(
        value.as_u64().ok_or_else(|| invalid("fixture count"))?,
    )?)
}
fn snapshot(fixture: &Value, indexed: &Value) -> Result<Value> {
    // Only expand losslessly interned captured field objects; no arithmetic,
    // clock/history reconstruction, default values or expected answers added.
    let tables = member(fixture, "state_tables")?;
    Ok(Value::Object(
        object(indexed)?
            .iter()
            .map(|(name, index)| {
                let table = array(member(tables, name)?)?;
                Ok((
                    name.clone(),
                    table
                        .get(count(index)?)
                        .ok_or_else(|| invalid("state table index"))?
                        .clone(),
                ))
            })
            .collect::<Result<serde_json::Map<_, _>>>()?,
    ))
}
fn boolean(value: &Value) -> Result<bool> {
    Ok(value.as_bool().ok_or_else(|| invalid("fixture bool"))?)
}
fn bool_integer(value: &Value) -> Result<bool> {
    match integer(value)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(invalid("Native bool transport must be0/1").into()),
    }
}
fn real(value: &Value) -> Result<f64> {
    let bits = text(value)?;
    if bits.len() != 16
        || !bits
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(invalid("exact16hex native binary64").into());
    }
    Ok(f64::from_bits(u64::from_str_radix(bits, 16)?))
}
fn four(value: &Value) -> Result<[f64; 4]> {
    let values = array(value)?;
    if values.len() != 4 {
        return Err(invalid("actual four history slots").into());
    }
    Ok([
        real(&values[0])?,
        real(&values[1])?,
        real(&values[2])?,
        real(&values[3])?,
    ])
}
fn assert_real(actual: f64, expected: &Value, context: &str, name: &str) -> Result {
    assert_eq!(
        actual.to_bits(),
        real(expected)?.to_bits(),
        "{context}: {name}"
    );
    Ok(())
}
fn raw_value(value: &Value) -> Result<SourceInputStatisticsValue> {
    Ok(match value {
        Value::Null => SourceInputStatisticsValue::Null,
        Value::Bool(v) => SourceInputStatisticsValue::Bool(*v),
        Value::Number(v) => SourceInputStatisticsValue::Number(v.to_string()),
        Value::String(v) => SourceInputStatisticsValue::String(v.clone()),
        Value::Array(v) => {
            SourceInputStatisticsValue::Array(v.iter().map(raw_value).collect::<Result<Vec<_>>>()?)
        }
        Value::Object(v) => SourceInputStatisticsValue::Object(
            v.iter()
                .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                .collect::<Result<BTreeMap<_, _>>>()?,
        ),
    })
}
fn json_size(value: &Value) -> usize {
    match value {
        Value::Null => 0,
        Value::Array(v) => v.len(),
        Value::Object(v) => v.len(),
        _ => 1,
    }
}
fn input(fixture: &Value) -> Result<SelectedInputProcessor> {
    let mut definitions = Vec::new();
    for row in array(member(fixture, "definitions")?)? {
        let row = array(row)?;
        if row.len() != 6 {
            return Err(invalid("query definition width").into());
        }
        definitions.push(SourceObjectDefinition {
            object_type: text(&row[0])?.into(),
            alpha_fields: count(&row[1])?,
            numeric_fields: count(&row[2])?,
            alpha_extensions: count(&row[3])?,
            numeric_extensions: count(&row[4])?,
            extension: if row[5].is_null() {
                None
            } else {
                Some(text(&row[5])?.into())
            },
        });
    }
    let mut families = BTreeMap::new();
    let mut raw = SourceInputStatisticsInput::default();
    for (family, objects) in object(member(fixture, "raw")?)? {
        let mut shapes = Vec::new();
        let mut retained = BTreeMap::new();
        for (name, fields) in object(objects)? {
            shapes.push(SourceObjectShape {
                object_name: name.clone(),
                field_sizes: object(fields)?
                    .iter()
                    .map(|(k, v)| (k.clone(), json_size(v)))
                    .collect(),
            });
            retained.insert(
                name.clone(),
                object(fields)?
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                    .collect::<Result<BTreeMap<_, _>>>()?,
            );
        }
        families.insert(family.clone(), SourceInputFamily::Objects(shapes));
        raw.objects.insert(family.clone(), retained);
        raw.object_order.insert(
            family.clone(),
            array(member(member(fixture, "object_order")?, family)?)?
                .iter()
                .map(|v| Ok(text(v)?.to_owned()))
                .collect::<Result<Vec<_>>>()?,
        );
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    input.retain_source_input_statistics(raw)?;
    Ok(input)
}
fn identity_model() -> ep_model::TypedModel {
    use ep_model::*;
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = 4;
    model.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("ZONE ONE"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
        floor_area: AutoOrNumber::AutoCalculate,
        inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: true,
        linked_outdoor_air_node: None,
        spaces: vec![SpaceId(0)],
    });
    model.spaces.push(Space {
        id: SpaceId(0),
        name: NormalizedName::new("ZONE ONE"),
        zone: ZoneId(0),
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
        floor_area: AutoOrNumber::AutoCalculate,
        space_type: NormalizedName::new("GENERAL"),
        space_type_id: SpaceTypeId(0),
        tags: Vec::new(),
        origin: SpaceOrigin::AutoZoneDefault,
    });
    model
}
fn error_state() -> SourceErrorState {
    // These ancillary diagnostic operands are never read by either method.
    SourceErrorState {
        output_error_header: true,
        version_string: String::new(),
        idd_version_string: String::new(),
        doing_input_processing: false,
        print_console_output: false,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            match_counts: [0; 20],
            abort_processing: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
            total_severe_errors: 0,
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
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
    air.xmat = four(member(raw, "XMAT")?)?;
    air.dsxmat = four(member(raw, "DSXMAT")?)?;
    air.w_prev_zone_ts = four(member(raw, "WPrevZoneTS")?)?;
    air.dsw_prev_zone_ts = four(member(raw, "DSWPrevZoneTS")?)?;
    air.ztm = four(member(raw, "ZTM")?)?;
    air.w_prev_zone_ts_temp = four(member(raw, "WPrevZoneTSTemp")?)?;
    Ok(())
}
fn check_air(air: &SourceZoneSpaceHeatBalanceData, raw: &Value, context: &str) -> Result {
    check_reals!(air,raw,context,{"MAT"=>mat,"airHumRat"=>air_hum_rat,"T1"=>t1,"AirPowerCap"=>air_power_cap,
        "tempDepLoad"=>temp_dep_load,"tempIndLoad"=>temp_ind_load,"setPointLast"=>set_point_last});
    for (name, values) in [
        ("XMAT", air.xmat),
        ("DSXMAT", air.dsxmat),
        ("WPrevZoneTS", air.w_prev_zone_ts),
        ("DSWPrevZoneTS", air.dsw_prev_zone_ts),
        ("ZTM", air.ztm),
        ("WPrevZoneTSTemp", air.w_prev_zone_ts_temp),
    ] {
        let expected = four(member(raw, name)?)?;
        assert_eq!(
            values.map(f64::to_bits),
            expected.map(f64::to_bits),
            "{context}: {name}"
        );
    }
    Ok(())
}
fn sequence(raw: &Value) -> Result<Option<Vec<f64>>> {
    if boolean(member(raw, "allocated")?)? {
        let values = array(member(raw, "values")?)?;
        assert_eq!(values.len(), count(member(raw, "size")?)?);
        Ok(Some(values.iter().map(real).collect::<Result<Vec<_>>>()?))
    } else {
        assert!(member(raw, "values")?.is_null());
        Ok(None)
    }
}
fn seed_demand(row: &mut SourceSystemSensibleDemand, raw: &Value) -> Result {
    row.base.num_zone_equipment = integer(member(raw, "NumZoneEquipment")?)?;
    row.base.stage_num = integer(member(raw, "StageNum")?)?;
    seed_reals!(&mut row.base,raw,{"RemainingOutputRequired"=>remaining_output_required,
        "UnadjRemainingOutputRequired"=>unadjusted_remaining_output_required,"TotalOutputRequired"=>total_output_required,
        "SupplyAirAdjustFactor"=>supply_air_adjust_factor});
    seed_reals!(row,raw,{"OutputRequiredToHeatingSP"=>output_required_to_heating_sp,"OutputRequiredToCoolingSP"=>output_required_to_cooling_sp,
        "RemainingOutputReqToHeatSP"=>remaining_output_to_heat_sp,"RemainingOutputReqToCoolSP"=>remaining_output_to_cool_sp,
        "UnadjRemainingOutputReqToHeatSP"=>unadjusted_remaining_output_to_heat_sp,"UnadjRemainingOutputReqToCoolSP"=>unadjusted_remaining_output_to_cool_sp,
        "predictedRate"=>predicted_rate,"predictedHSPRate"=>predicted_heating_sp_rate,"predictedCSPRate"=>predicted_cooling_sp_rate,
        "airSysHeatRate"=>air_sys_heat_rate,"airSysCoolRate"=>air_sys_cool_rate,"airSysHeatEnergy"=>air_sys_heat_energy,"airSysCoolEnergy"=>air_sys_cool_energy});
    row.sequenced_output_required = sequence(member(raw, "SequencedOutputRequired")?)?;
    row.sequenced_output_to_heating_sp =
        sequence(member(raw, "SequencedOutputRequiredToHeatingSP")?)?;
    row.sequenced_output_to_cooling_sp =
        sequence(member(raw, "SequencedOutputRequiredToCoolingSP")?)?;
    Ok(())
}
fn check_demand(row: &SourceSystemSensibleDemand, raw: &Value, context: &str) -> Result {
    assert_eq!(
        row.base.num_zone_equipment,
        integer(member(raw, "NumZoneEquipment")?)?,
        "{context}: NumZoneEquipment"
    );
    assert_eq!(
        row.base.stage_num,
        integer(member(raw, "StageNum")?)?,
        "{context}: StageNum"
    );
    check_reals!(&row.base,raw,context,{"RemainingOutputRequired"=>remaining_output_required,
        "UnadjRemainingOutputRequired"=>unadjusted_remaining_output_required,"TotalOutputRequired"=>total_output_required,
        "SupplyAirAdjustFactor"=>supply_air_adjust_factor});
    check_reals!(row,raw,context,{"OutputRequiredToHeatingSP"=>output_required_to_heating_sp,"OutputRequiredToCoolingSP"=>output_required_to_cooling_sp,
        "RemainingOutputReqToHeatSP"=>remaining_output_to_heat_sp,"RemainingOutputReqToCoolSP"=>remaining_output_to_cool_sp,
        "UnadjRemainingOutputReqToHeatSP"=>unadjusted_remaining_output_to_heat_sp,"UnadjRemainingOutputReqToCoolSP"=>unadjusted_remaining_output_to_cool_sp,
        "predictedRate"=>predicted_rate,"predictedHSPRate"=>predicted_heating_sp_rate,"predictedCSPRate"=>predicted_cooling_sp_rate,
        "airSysHeatRate"=>air_sys_heat_rate,"airSysCoolRate"=>air_sys_cool_rate,"airSysHeatEnergy"=>air_sys_heat_energy,"airSysCoolEnergy"=>air_sys_cool_energy});
    for (name, values) in [
        ("SequencedOutputRequired", &row.sequenced_output_required),
        (
            "SequencedOutputRequiredToHeatingSP",
            &row.sequenced_output_to_heating_sp,
        ),
        (
            "SequencedOutputRequiredToCoolingSP",
            &row.sequenced_output_to_cooling_sp,
        ),
    ] {
        let expected = sequence(member(raw, name)?)?;
        assert_eq!(
            values
                .as_ref()
                .map(|v| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>()),
            expected
                .as_ref()
                .map(|v| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>()),
            "{context}: {name}"
        );
    }
    Ok(())
}
fn seed_state(
    air: &mut SourceZoneSpaceHeatBalanceData,
    controller: &mut SourceZoneAirPredictorCorrector,
    owners: &mut SourceZoneAirOwners<'_, '_>,
    raw: &Value,
) -> Result {
    seed_air(air, member(raw, "air")?)?;
    let zone = owners
        .zones
        .zones
        .as_mut()
        .and_then(|rows| rows.first_mut())
        .ok_or_else(|| invalid("actual Source Zone"))?;
    let z = member(raw, "zone")?;
    assert_eq!(zone.name, text(member(z, "Name")?)?);
    assert_eq!(
        zone.system_zone_node_number,
        integer(member(z, "SystemZoneNodeNumber")?)?
    );
    zone.is_controlled = boolean(member(z, "IsControlled")?)?;
    zone.multiplier = integer(member(z, "Multiplier")?)?;
    zone.list_multiplier = integer(member(z, "ListMultiplier")?)?;
    zone.has_adjusted_return_temperature_by_ite =
        boolean(member(z, "HasAdjustedReturnTempByITE")?)?;
    zone.adjusted_return_temperature_by_ite = real(member(z, "AdjustedReturnTempByITE")?)?;
    let flags = member(raw, "flags")?;
    assert_eq!(
        integer(member(flags, "ZoneAirSolutionAlgo")?)?,
        0,
        "actual Native ThirdOrder enum"
    );
    owners.control.zone_air_solution = SelectedZoneAirSolution::ThirdOrder;
    controller.flags.number_stage_zones = integer(member(flags, "NumStageCtrZone")?)?;
    controller.subsystems.room_air.any_non_mixing =
        boolean(member(flags, "anyNonMixingRoomAirModel")?)?;
    let stage = member(flags, "StageZoneLogic")?;
    controller.controls.stage_zone_logic = if boolean(member(stage, "allocated")?)? {
        Some(vec![bool_integer(member(stage, "value")?)?])
    } else {
        assert!(member(stage, "value")?.is_null());
        None
    };
    let c = member(raw, "clock")?;
    // Caller-owned clock is supplied separately when constructing the borrow.
    assert_eq!(
        owners.clock.begin_sim_flag,
        boolean(member(c, "BeginSimFlag")?)?
    );
    owners.hvac.number_system_time_steps = integer(member(c, "NumOfSysTimeSteps")?)?;
    owners.hvac.number_system_time_steps_last_zone =
        integer(member(c, "NumOfSysTimeStepsLastZoneTimeStep")?)?;
    owners.hvac.time_step_sys = real(member(c, "TimeStepSys")?)?;
    let n = member(raw, "node")?;
    let node = owners
        .equipment
        .source_zone_air_node_mut(integer(member(raw, "node_ordinal")?)?)?;
    *node.temperature = real(member(n, "Temp")?)?;
    *node.humidity_ratio = real(member(n, "HumRat")?)?;
    *node.enthalpy = real(member(n, "Enthalpy")?)?;
    *node.temperature_setpoint = real(member(n, "TempSetPoint")?)?;
    let f = member(raw, "fan")?;
    let set = member(f, "setpoints")?;
    owners.fan.temp_tstat_air = Some(vec![real(member(f, "TempTstatAir")?)?]);
    owners.fan.thermostat.load_correction_factor =
        Some(vec![real(member(f, "LoadCorrectionFactor")?)?]);
    owners.fan.thermostat.temp_control_type = Some(vec![integer(member(f, "TempControlType")?)?]);
    owners.fan.thermostat.zone_tstat_setpoints = Some(vec![SourceZoneTstatSetpoint {
        setpt: real(member(set, "setpt")?)?,
        setpt_lo: real(member(set, "setptLo")?)?,
        setpt_hi: real(member(set, "setptHi")?)?,
        ..Default::default()
    }]);
    let d = member(raw, "demand")?;
    controller.demand.setback = Some(vec![bool_integer(member(d, "Setback")?)?]);
    controller.demand.deadband_or_setback =
        Some(vec![bool_integer(member(d, "DeadBandOrSetback")?)?]);
    controller.demand.current_deadband_or_setback =
        Some(vec![bool_integer(member(d, "CurDeadBandOrSetback")?)?]);
    let row = controller
        .demand
        .zones
        .as_mut()
        .and_then(|rows| rows.first_mut())
        .ok_or_else(|| invalid("actual Source Demand"))?;
    seed_demand(row, member(d, "sensible")?)?;
    Ok(())
}
fn check_state(
    air: &SourceZoneSpaceHeatBalanceData,
    controller: &SourceZoneAirPredictorCorrector,
    owners: &mut SourceZoneAirOwners<'_, '_>,
    raw: &Value,
    context: &str,
) -> Result {
    check_air(air, member(raw, "air")?, context)?;
    let node = owners
        .equipment
        .source_zone_air_node_mut(integer(member(raw, "node_ordinal")?)?)?;
    let n = member(raw, "node")?;
    assert_real(*node.temperature, member(n, "Temp")?, context, "Node.Temp")?;
    assert_real(
        *node.humidity_ratio,
        member(n, "HumRat")?,
        context,
        "Node.HumRat",
    )?;
    assert_real(
        *node.enthalpy,
        member(n, "Enthalpy")?,
        context,
        "Node.Enthalpy",
    )?;
    assert_real(
        *node.temperature_setpoint,
        member(n, "TempSetPoint")?,
        context,
        "Node.TempSetPoint",
    )?;
    let f = member(raw, "fan")?;
    let set = member(f, "setpoints")?;
    let temp = owners
        .fan
        .temp_tstat_air
        .as_ref()
        .and_then(|rows| rows.first())
        .ok_or_else(|| invalid("TempTstatAir"))?;
    assert_real(*temp, member(f, "TempTstatAir")?, context, "TempTstatAir")?;
    let correction = owners
        .fan
        .thermostat
        .load_correction_factor
        .as_ref()
        .and_then(|rows| rows.first())
        .ok_or_else(|| invalid("LoadCorrectionFactor"))?;
    assert_real(
        *correction,
        member(f, "LoadCorrectionFactor")?,
        context,
        "LoadCorrectionFactor",
    )?;
    let kind = owners
        .fan
        .thermostat
        .temp_control_type
        .as_ref()
        .and_then(|rows| rows.first())
        .ok_or_else(|| invalid("TempControlType"))?;
    assert_eq!(
        *kind,
        integer(member(f, "TempControlType")?)?,
        "{context}: TempControlType"
    );
    let values = owners
        .fan
        .thermostat
        .zone_tstat_setpoints
        .as_ref()
        .and_then(|rows| rows.first())
        .ok_or_else(|| invalid("zoneTstatSetpts"))?;
    check_reals!(values,set,context,{"setpt"=>setpt,"setptLo"=>setpt_lo,"setptHi"=>setpt_hi});
    let d = member(raw, "demand")?;
    assert_eq!(
        controller.demand.setback.as_deref(),
        Some(&[bool_integer(member(d, "Setback")?)?][..]),
        "{context}: Setback"
    );
    assert_eq!(
        controller.demand.deadband_or_setback.as_deref(),
        Some(&[bool_integer(member(d, "DeadBandOrSetback")?)?][..]),
        "{context}: DeadBand"
    );
    assert_eq!(
        controller.demand.current_deadband_or_setback.as_deref(),
        Some(&[bool_integer(member(d, "CurDeadBandOrSetback")?)?][..]),
        "{context}: CurDeadBand"
    );
    check_demand(
        controller
            .demand
            .zones
            .as_ref()
            .and_then(|rows| rows.first())
            .ok_or_else(|| invalid("actual Demand row"))?,
        member(d, "sensible")?,
        context,
    )?;
    Ok(())
}

fn all_native_b_prediction_entries_match_source_rollback_and_sensible_bits() -> Result {
    let fixture: Value = serde_json::from_str(NATIVE_PREDICTION_COHORT_JSON)?;
    let cases = array(member(&fixture, "cases")?)?;
    assert_eq!(cases.len(), 1168);
    let mut input = input(&fixture)?;
    let inventory = object(member(&fixture, "raw")?)?
        .iter()
        .map(|(family, objects)| Ok((family.as_str(), object(objects)?.len())))
        .collect::<Result<Vec<_>>>()?;
    let equipment_input = OrdinaryEquipmentInput::from_source_family_inventory(inventory, 4)?;
    let model = identity_model();
    let mut equipment = OrdinaryEquipment::new_for_source(&model, equipment_input)?;
    let mut globals = SelectedHeatBalanceGlobals::default();
    globals.num_of_zones = 1;
    globals.num_spaces = 1;
    let mut zones = SelectedZoneInputOwners::default();
    zones.zones = Some(vec![SelectedZoneData {
        name: "ZONE ONE".into(),
        typed_zone_id: Some(ep_model::ZoneId(0)),
        space_indexes: vec![1],
        num_spaces: 1,
        ..Default::default()
    }]);
    let mut schedules = SelectedScheduleManager::default();
    schedules.init_constant_state(4)?;
    let mut sizing = SelectedSizingManager::default();
    let mut branch = SourceBranchNodeConnections::default();
    let mut gains = SelectedInternalHeatGains::default();
    equipment.get_zone_equipment_source(
        &mut input,
        equipment_input,
        false,
        false,
        &globals,
        &mut gains,
        SourceEquipmentInputOwners {
            zones: zones
                .zones
                .as_deref_mut()
                .ok_or_else(|| invalid("Zone allocation"))?,
            schedules: &mut schedules,
            sizing: &mut sizing,
            branch: Some(&mut branch),
            source_is_epjson: false,
        },
    )?;
    let mut controller = SourceZoneAirPredictorCorrector::default();
    let space_controls = SelectedHeatBalanceSpaceControls::default();
    controller
        .demand
        .allocate_source_demands(&globals, &space_controls)?;
    let mut fan=crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays::default();
    let mut control = SelectedHeatBalanceInputState::default();
    let geometry = SurfaceGeometryInputState::default();
    let surface_arrays = SourceSurfaceHeatBalanceArrays::default();
    let environment = WeatherEnvironmentState::default();
    let faults = crate::heat_balance::source_initialization::SourceFaultsManager::default();
    let availability=crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability::default();
    let mut psychrometrics = SourcePsychrometricAccess::default();
    let mut shortcut = SourceInputProcessorShortCut::default();
    let mut state = error_state();
    let mut console = Vec::new();
    let mut backend = SourceErrorBackend {
        state: &mut state,
        error_stream: None,
        console: &mut console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let source_is_epjson = false;
    let mut recurring_errors =
        crate::heat_balance::source_initialization::SourceRecurringErrorState::default();
    let mut shortened = 0;
    let mut groups = BTreeMap::new();
    let mut first_physical_address = None;
    for (index, case) in cases.iter().enumerate() {
        assert_eq!(
            count(member(case, "parent")?)?,
            index + 1,
            "all actual parent IDs retained"
        );
        let context = format!("Native parent{}", index + 1);
        let entry = snapshot(&fixture, member(case, "update_entry")?)?;
        let clock_row = member(&entry, "clock")?;
        let mut clock = WeatherGlobalState::default();
        clock.begin_sim_flag = boolean(member(clock_row, "BeginSimFlag")?)?;
        let mut hvac = SelectedHvacGlobals::default();
        let mut air = SourceZoneSpaceHeatBalanceData::default();
        let mut owners = SourceZoneAirOwners {
            zones: &mut zones,
            equipment: &mut equipment,
            geometry: &geometry,
            surface_arrays: &surface_arrays,
            fan: &mut fan,
            gains: &gains,
            control: &mut control,
            space_controls: &space_controls,
            globals: &globals,
            clock: &clock,
            environment: &environment,
            hvac: &mut hvac,
            input: &mut input,
            shortcut: &mut shortcut,
            source_is_epjson: &source_is_epjson,
            faults: &faults,
            schedules: &mut schedules,
            availability: &availability,
            psychrometrics: &mut psychrometrics,
            backend: &mut backend,
            recurring_errors: &mut recurring_errors,
        };
        seed_state(&mut air, &mut controller, &mut owners, &entry)?;
        let ordinal = integer(member(&entry, "node_ordinal")?)?;
        let address = {
            let node = owners.equipment.source_zone_air_node_mut(ordinal)?;
            std::ptr::from_ref(&*node.temperature) as usize
        };
        if let Some(first) = first_physical_address {
            assert_eq!(address, first, "same physical Node retained across cohort");
        } else {
            first_physical_address = Some(address);
        }
        let args = member(case, "update_arguments")?;
        let shorten = bool_integer(member(args, "ShortenTimeStepSys")?)?;
        let use_history = bool_integer(member(args, "UseZoneTimeStepHistory")?)?;
        if shorten {
            shortened += 1;
        }
        *groups
            .entry((
                boolean(member(clock_row, "WarmupFlag")?)?,
                boolean(member(clock_row, "KickOffSimulation")?)?,
                shorten,
                use_history,
            ))
            .or_insert(0_usize) += 1;
        air.update_source_temperatures(
            1,
            shorten,
            use_history,
            real(member(args, "PriorTimeStep")?)?,
            &controller,
            &mut owners,
        )?;
        // Assert rollback/history outputs BEFORE supplying later coefficient inputs.
        check_state(
            &air,
            &controller,
            &mut owners,
            &snapshot(&fixture, member(case, "update_exit")?)?,
            &format!("{context} update exit"),
        )?;
        let sensible_entry = snapshot(&fixture, member(case, "sensible_entry")?)?;
        let coefficients = member(&sensible_entry, "air")?;
        // Only the real parent-computed method-entry coefficient boundary is supplied.
        air.air_power_cap = real(member(coefficients, "AirPowerCap")?)?;
        air.temp_dep_load = real(member(coefficients, "tempDepLoad")?)?;
        air.temp_ind_load = real(member(coefficients, "tempIndLoad")?)?;
        check_state(
            &air,
            &controller,
            &mut owners,
            &sensible_entry,
            &format!("{context} same-state sensible entry"),
        )?;
        air.calculate_source_predicted_sensible(
            real(member(member(case, "sensible_arguments")?, "RAFNFrac")?)?,
            1,
            &mut controller,
            &mut owners,
        )?;
        check_state(
            &air,
            &controller,
            &mut owners,
            &snapshot(&fixture, member(case, "sensible_exit")?)?,
            &format!("{context} sensible exit"),
        )?;
        let after = {
            let node = owners.equipment.source_zone_air_node_mut(ordinal)?;
            std::ptr::from_ref(&*node.temperature) as usize
        };
        assert_eq!(
            after, address,
            "{context}: both methods keep same actual Node row"
        );
    }
    assert_eq!(shortened, 77);
    assert_eq!(
        groups,
        BTreeMap::from([
            ((false, false, false, false), 57),
            ((false, false, false, true), 96),
            ((false, false, true, false), 10),
            ((true, false, false, false), 359),
            ((true, false, false, true), 576),
            ((true, false, true, false), 67),
            ((true, true, false, true), 3)
        ])
    );
    run_corrector_cohort(
        &fixture,
        &mut equipment,
        &mut input,
        &mut zones,
        &mut schedules,
        &mut controller,
        &mut fan,
        &mut gains,
        &globals,
        &mut backend,
        &mut recurring_errors,
    )?;
    assert!(
        console.is_empty(),
        "the two bounded methods did not emit diagnostics"
    );
    Ok(())
}

// Whole corrector numeric/observed-owner cohort. Cache hit/miss and recurrence history
// are unobserved in Native21 and are not reconstructed or asserted here.
fn corrector_snapshot(fixture: &Value, indexed: &Value) -> Result<Value> {
    let tables = member(fixture, "corrector_state_tables")?;
    Ok(Value::Object(
        object(indexed)?
            .iter()
            .map(|(name, index)| {
                Ok((
                    name.clone(),
                    array(member(tables, name)?)?
                        .get(count(index)?)
                        .ok_or_else(|| invalid("corrector component index"))?
                        .clone(),
                ))
            })
            .collect::<Result<serde_json::Map<_, _>>>()?,
    ))
}
fn seed_corrector_air(air: &mut SourceZoneSpaceHeatBalanceData, raw: &Value) -> Result {
    seed_reals!(air,raw,{"MAT"=>mat,
        "MRT"=>mrt,
        "ZTAV"=>ztav,
        "ZT"=>zt,
        "ZTAVComf"=>ztav_comf,
        "XMPT"=>xmpt,
        "TMX"=>tmx,
        "TM2"=>tm2,
        "T1"=>t1,
        "airHumRat"=>air_hum_rat,
        "airHumRatAvg"=>air_hum_rat_avg,
        "airHumRatTemp"=>air_hum_rat_temp,
        "airHumRatAvgComf"=>air_hum_rat_avg_comf,
        "WTimeMinusP"=>w_time_minus_p,
        "WMX"=>wmx,
        "WM2"=>wm2,
        "W1"=>w1,
        "SumIntGain"=>sum_int_gain,
        "SumHA"=>sum_ha,
        "SumHATsurf"=>sum_ha_tsurf,
        "SumHATref"=>sum_ha_tref,
        "SumMCp"=>sum_m_cp,
        "SumMCpT"=>sum_m_cp_t,
        "SumSysMCp"=>sum_sys_m_cp,
        "SumSysMCpT"=>sum_sys_m_cp_t,
        "SumIntGainExceptPeople"=>sum_int_gain_except_people,
        "SumHmAW"=>sum_hm_aw,
        "SumHmARa"=>sum_hm_a_ra,
        "SumHmARaW"=>sum_hm_a_ra_w,
        "SumHmARaZ"=>sum_hm_a_ra_z,
        "TempDepCoef"=>temp_dep_coef,
        "TempIndCoef"=>temp_ind_coef,
        "TempHistoryTerm"=>temp_history_term,
        "MCPI"=>mcpi,
        "MCPTI"=>mcpti,
        "MCPV"=>mcpv,
        "MCPTV"=>mcptv,
        "MCPM"=>mcpm,
        "MCPTM"=>mcptm,
        "MCPE"=>mcpe,
        "EAMFL"=>eamfl,
        "EAMFLxHumRat"=>eamf_lx_hum_rat,
        "MCPTE"=>mcpte,
        "MCPC"=>mcpc,
        "CTMFL"=>ctmfl,
        "MCPTC"=>mcptc,
        "ThermChimAMFL"=>therm_chim_amfl,
        "MCPTThermChim"=>mcpt_therm_chim,
        "MCPThermChim"=>mcp_therm_chim,
        "latentGain"=>latent_gain,
        "latentGainExceptPeople"=>latent_gain_except_people,
        "OAMFL"=>oamfl,
        "VAMFL"=>vamfl,
        "NonAirSystemResponse"=>non_air_system_response,
        "SysDepZoneLoads"=>sys_dep_zone_loads,
        "SysDepZoneLoadsLagged"=>sys_dep_zone_loads_lagged,
        "MDotCPOA"=>m_dot_cpoa,
        "MDotOA"=>m_dot_oa,
        "MixingMAT"=>mixing_mat,
        "MixingHumRat"=>mixing_hum_rat,
        "MixingMassFlowZone"=>mixing_mass_flow_zone,
        "MixingMassFlowXHumRat"=>mixing_mass_flow_x_hum_rat,
        "setPointLast"=>set_point_last,
        "tempIndLoad"=>temp_ind_load,
        "tempDepLoad"=>temp_dep_load,
        "airRelHum"=>air_rel_hum,
        "AirPowerCap"=>air_power_cap});
    air.xmat = four(member(raw, "XMAT")?)?;
    air.dsxmat = four(member(raw, "DSXMAT")?)?;
    air.w_prev_zone_ts = four(member(raw, "WPrevZoneTS")?)?;
    air.dsw_prev_zone_ts = four(member(raw, "DSWPrevZoneTS")?)?;
    air.ztm = four(member(raw, "ZTM")?)?;
    air.w_prev_zone_ts_temp = four(member(raw, "WPrevZoneTSTemp")?)?;
    Ok(())
}
fn check_corrector_air(air: &SourceZoneSpaceHeatBalanceData, raw: &Value, context: &str) -> Result {
    check_reals!(air,raw,context,{"MAT"=>mat,
        "MRT"=>mrt,
        "ZTAV"=>ztav,
        "ZT"=>zt,
        "ZTAVComf"=>ztav_comf,
        "XMPT"=>xmpt,
        "TMX"=>tmx,
        "TM2"=>tm2,
        "T1"=>t1,
        "airHumRat"=>air_hum_rat,
        "airHumRatAvg"=>air_hum_rat_avg,
        "airHumRatTemp"=>air_hum_rat_temp,
        "airHumRatAvgComf"=>air_hum_rat_avg_comf,
        "WTimeMinusP"=>w_time_minus_p,
        "WMX"=>wmx,
        "WM2"=>wm2,
        "W1"=>w1,
        "SumIntGain"=>sum_int_gain,
        "SumHA"=>sum_ha,
        "SumHATsurf"=>sum_ha_tsurf,
        "SumHATref"=>sum_ha_tref,
        "SumMCp"=>sum_m_cp,
        "SumMCpT"=>sum_m_cp_t,
        "SumSysMCp"=>sum_sys_m_cp,
        "SumSysMCpT"=>sum_sys_m_cp_t,
        "SumIntGainExceptPeople"=>sum_int_gain_except_people,
        "SumHmAW"=>sum_hm_aw,
        "SumHmARa"=>sum_hm_a_ra,
        "SumHmARaW"=>sum_hm_a_ra_w,
        "SumHmARaZ"=>sum_hm_a_ra_z,
        "TempDepCoef"=>temp_dep_coef,
        "TempIndCoef"=>temp_ind_coef,
        "TempHistoryTerm"=>temp_history_term,
        "MCPI"=>mcpi,
        "MCPTI"=>mcpti,
        "MCPV"=>mcpv,
        "MCPTV"=>mcptv,
        "MCPM"=>mcpm,
        "MCPTM"=>mcptm,
        "MCPE"=>mcpe,
        "EAMFL"=>eamfl,
        "EAMFLxHumRat"=>eamf_lx_hum_rat,
        "MCPTE"=>mcpte,
        "MCPC"=>mcpc,
        "CTMFL"=>ctmfl,
        "MCPTC"=>mcptc,
        "ThermChimAMFL"=>therm_chim_amfl,
        "MCPTThermChim"=>mcpt_therm_chim,
        "MCPThermChim"=>mcp_therm_chim,
        "latentGain"=>latent_gain,
        "latentGainExceptPeople"=>latent_gain_except_people,
        "OAMFL"=>oamfl,
        "VAMFL"=>vamfl,
        "NonAirSystemResponse"=>non_air_system_response,
        "SysDepZoneLoads"=>sys_dep_zone_loads,
        "SysDepZoneLoadsLagged"=>sys_dep_zone_loads_lagged,
        "MDotCPOA"=>m_dot_cpoa,
        "MDotOA"=>m_dot_oa,
        "MixingMAT"=>mixing_mat,
        "MixingHumRat"=>mixing_hum_rat,
        "MixingMassFlowZone"=>mixing_mass_flow_zone,
        "MixingMassFlowXHumRat"=>mixing_mass_flow_x_hum_rat,
        "setPointLast"=>set_point_last,
        "tempIndLoad"=>temp_ind_load,
        "tempDepLoad"=>temp_dep_load,
        "airRelHum"=>air_rel_hum,
        "AirPowerCap"=>air_power_cap});
    assert_eq!(
        air.xmat.map(f64::to_bits),
        four(member(raw, "XMAT")?)?.map(f64::to_bits),
        "{context}: XMAT"
    );
    assert_eq!(
        air.dsxmat.map(f64::to_bits),
        four(member(raw, "DSXMAT")?)?.map(f64::to_bits),
        "{context}: DSXMAT"
    );
    assert_eq!(
        air.w_prev_zone_ts.map(f64::to_bits),
        four(member(raw, "WPrevZoneTS")?)?.map(f64::to_bits),
        "{context}: WPrevZoneTS"
    );
    assert_eq!(
        air.dsw_prev_zone_ts.map(f64::to_bits),
        four(member(raw, "DSWPrevZoneTS")?)?.map(f64::to_bits),
        "{context}: DSWPrevZoneTS"
    );
    assert_eq!(
        air.ztm.map(f64::to_bits),
        four(member(raw, "ZTM")?)?.map(f64::to_bits),
        "{context}: ZTM"
    );
    assert_eq!(
        air.w_prev_zone_ts_temp.map(f64::to_bits),
        four(member(raw, "WPrevZoneTSTemp")?)?.map(f64::to_bits),
        "{context}: WPrevZoneTSTemp"
    );
    Ok(())
}
fn captured_node_values(raw: &Value) -> Result<[f64; 5]> {
    Ok([
        real(member(raw, "Temp")?)?,
        real(member(raw, "MassFlowRate")?)?,
        real(member(raw, "HumRat")?)?,
        real(member(raw, "Enthalpy")?)?,
        real(member(raw, "TempSetPoint")?)?,
    ])
}
fn restore_corrector_surfaces(
    zones: &mut SelectedZoneInputOwners,
    geometry: &mut SurfaceGeometryInputState,
    arrays: &mut SourceSurfaceHeatBalanceArrays,
    gains: &mut SelectedInternalHeatGains,
    spaces: &Value,
) -> Result {
    use crate::heat_balance::selected_heat_balance_input::surface::SurfaceClass;
    let spaces = array(spaces)?;
    zones.spaces = Some(vec![Default::default(); spaces.len()]);
    gains.allocations.space_devices = Some(vec![Default::default(); spaces.len()]);
    let mut surfaces = Vec::new();
    let mut h = Vec::new();
    let mut temp = Vec::new();
    let mut bulk = Vec::new();
    for (at, raw) in spaces.iter().enumerate() {
        assert_eq!(integer(member(raw, "index")?)?, i32::try_from(at + 1)?);
        let v = member(raw, "value")?;
        let space = zones
            .spaces
            .as_mut()
            .and_then(|rows| rows.get_mut(at))
            .ok_or_else(|| invalid("captured Space row"))?;
        space.volume = real(member(v, "Volume")?)?;
        space.fraction_zone_volume = real(member(v, "fracZoneVolume")?)?;
        space.system_zone_node_number = integer(member(v, "SystemZoneNodeNumber")?)?;
        space.ht_surface_first = integer(member(v, "HTSurfaceFirst")?)?;
        space.ht_surface_last = integer(member(v, "HTSurfaceLast")?)?;
        for raw in array(member(v, "surfaces")?)? {
            assert_eq!(
                integer(member(raw, "index")?)?,
                i32::try_from(surfaces.len() + 1)?
            );
            let v = member(raw, "value")?;
            // Exact pinned Native SurfaceClass1/2/3; all actual cohort surfaces opaque.
            let class = match integer(member(v, "Class")?)? {
                1 => SurfaceClass::Wall,
                2 => SurfaceClass::Floor,
                3 => SurfaceClass::Roof,
                _ => return Err(invalid("cohort actual opaque Native class").into()),
            };
            let reference = integer(member(member(v, "SurfTAirRef")?, "value")?)?;
            surfaces.push((real(member(v, "Area")?)?, class, reference));
            h.push(real(member(member(v, "SurfHConvInt")?, "value")?)?);
            temp.push(real(member(member(v, "SurfTempInTmp")?, "value")?)?);
            bulk.push(real(member(member(v, "SurfTempEffBulkAir")?, "value")?)?);
        }
        let raw_gain = member(member(v, "gains")?, "value")?;
        let devices = member(raw_gain, "device")?;
        assert!(boolean(member(devices, "allocated")?)?);
        let owner = gains
            .allocations
            .space_devices
            .as_mut()
            .and_then(|rows| rows.get_mut(at))
            .ok_or_else(|| invalid("same gain allocation"))?;
        owner.number_of_devices = count(member(raw_gain, "numberOfDevices")?)?;
        owner.max_number_of_devices = count(member(raw_gain, "maxNumberOfDevices")?)?;
        assert_eq!(
            owner.max_number_of_devices,
            count(member(devices, "size")?)?
        );
        owner.devices = Some(vec![Default::default(); owner.max_number_of_devices]);
        assert_eq!(
            array(member(devices, "values")?)?.len(),
            owner.number_of_devices
        );
        for (number, raw) in array(member(devices, "values")?)?.iter().enumerate() {
            assert_eq!(count(member(raw, "index")?)?, number + 1);
            let device = owner
                .devices
                .as_mut()
                .and_then(|rows| rows.get_mut(number))
                .ok_or_else(|| invalid("captured active gain device"))?;
            device.component_name = text(member(raw, "CompObjectName")?)?.into();
            device.component_type = integer(member(raw, "CompType")?)?;
            device.return_air_node = integer(member(raw, "ReturnAirNodeNum")?)?;
            device.cached_rates[0] = real(member(raw, "ConvectGainRate")?)?;
            device.cached_rates[1] = real(member(raw, "ReturnAirConvGainRate")?)?;
        }
    }
    geometry.restore_native_cohort_opaque_surfaces(&surfaces);
    arrays.surf_h_conv_int = Some(h);
    arrays.surf_temp_in_tmp = Some(temp);
    arrays.surf_temp_eff_bulk_air = Some(bulk);
    Ok(())
}
fn run_corrector_cohort(
    fixture: &Value,
    equipment: &mut OrdinaryEquipment,
    input: &mut SelectedInputProcessor,
    zones: &mut SelectedZoneInputOwners,
    schedules: &mut SelectedScheduleManager,
    controller: &mut SourceZoneAirPredictorCorrector,
    fan:&mut crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays,
    gains: &mut SelectedInternalHeatGains,
    globals: &SelectedHeatBalanceGlobals,
    backend: &mut SourceErrorBackend<'_>,
    recurring_errors: &mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
) -> Result {
    let cases = array(member(fixture, "corrector_cases")?)?;
    assert_eq!(cases.len(), 1168);
    assert_eq!(count(member(fixture, "corrector_case_count")?)?, 1168);
    let mut geometry = SurfaceGeometryInputState::default();
    let mut surface_arrays = SourceSurfaceHeatBalanceArrays::default();
    let mut control = SelectedHeatBalanceInputState::default();
    let space_controls = SelectedHeatBalanceSpaceControls::default();
    let faults = crate::heat_balance::source_initialization::SourceFaultsManager::default();
    let availability=crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability::default();
    let mut shortcut = SourceInputProcessorShortCut::default();
    let mut psychrometrics = SourcePsychrometricAccess::default();
    let source_is_epjson = false;
    let mut positive = 0;
    let mut first_addresses = None;
    // Run the genuine zero-input RoomAir getter; do not forge its input-read latch.
    controller
        .subsystems
        .room_air
        .manage_source_air_model(1, input, &mut shortcut, backend)?;
    crate::psychrometrics::with_fresh_psychrometric_state(|| -> Result {
        for (at, case) in cases.iter().enumerate() {
            assert_eq!(count(member(case, "invocation")?)?, at + 1);
            let context = format!("Native corrector{}", at + 1);
            let entry = corrector_snapshot(fixture, member(case, "entry")?)?;
            let expected = corrector_snapshot(fixture, member(case, "exit")?)?;
            let spaces = array(member(
                member(fixture, "corrector_state_tables")?,
                "spaces",
            )?)?
            .get(count(member(case, "sums_inputs")?)?)
            .ok_or_else(|| invalid("captured first-consumed Sums inputs"))?;
            restore_corrector_surfaces(zones, &mut geometry, &mut surface_arrays, gains, spaces)?;
            let flags = member(&entry, "flags")?;
            let z = member(&entry, "zone")?;
            let f = member(&entry, "fan")?;
            assert!(!boolean(member(flags, "doSpaceHeatBalance")?)?);
            control.do_latent_sizing = boolean(member(flags, "DoLatentSizing")?)?;
            assert_eq!(
                globals.hybrid_model_people_count,
                boolean(member(flags, "FlagHybridModel_PC")?)?
            );
            controller.subsystems.hybrid_model.active = boolean(member(flags, "FlagHybridModel")?)?;
            controller.subsystems.duct_loss.simulated = boolean(member(flags, "DuctLossSimu")?)?;
            let afn = &mut controller.subsystems.airflow_network;
            afn.multizone_always_simulated = boolean(member(flags, "multizone_always_simulated")?)?;
            afn.control_type = integer(member(flags, "AFN_control_type")?)?;
            afn.fan_activated = boolean(member(flags, "AirflowNetworkFanActivated")?)?;
            afn.distribution_simulated = boolean(member(flags, "distribution_simulated")?)?;
            let room = member(flags, "roomAir")?;
            controller.subsystems.room_air.air_models = Some(vec![Default::default()]);
            let model = controller
                .subsystems
                .room_air
                .air_models
                .as_mut()
                .and_then(|rows| rows.first_mut())
                .ok_or_else(|| invalid("actual RoomAir row"))?;
            model.model = integer(member(room, "AirModel")?)?;
            model.simulate_air_model = boolean(member(room, "SimAirModel")?)?;
            let c = member(&entry, "clock")?;
            let mut clock = WeatherGlobalState::default();
            clock.begin_sim_flag = boolean(member(c, "BeginSimFlag")?)?;
            let mut environment = WeatherEnvironmentState::default();
            environment.current_weather.out_baro_press = real(member(c, "OutBaroPress")?)?;
            environment.current_weather.out_hum_rat = real(member(c, "OutHumRat")?)?;
            let mut hvac = SelectedHvacGlobals::default();
            hvac.time_step_sys_seconds = real(member(c, "TimeStepSysSec")?)?;
            let mut air = SourceZoneSpaceHeatBalanceData::default();
            let mut owners = SourceZoneAirOwners {
                zones,
                equipment,
                geometry: &geometry,
                surface_arrays: &surface_arrays,
                fan,
                gains,
                control: &mut control,
                space_controls: &space_controls,
                globals,
                clock: &clock,
                environment: &environment,
                hvac: &mut hvac,
                input,
                shortcut: &mut shortcut,
                source_is_epjson: &source_is_epjson,
                faults: &faults,
                schedules,
                availability: &availability,
                psychrometrics: &mut psychrometrics,
                backend,
                recurring_errors,
            };
            seed_state(&mut air, controller, &mut owners, &entry)?;
            seed_corrector_air(&mut air, member(&entry, "air")?)?;
            let zone = owners
                .zones
                .zones
                .as_mut()
                .and_then(|rows| rows.first_mut())
                .ok_or_else(|| invalid("same real Zone"))?;
            seed_reals!(zone,z,{"Volume"=>volume,"ZoneVolCapMultpSens"=>sensible_capacity_multiplier,
                "ZoneVolCapMultpMoist"=>moisture_capacity_multiplier,"OutDryBulbTemp"=>dry_bulb});
            zone.no_heat_to_return_air = boolean(member(z, "NoHeatToReturnAir")?)?;
            zone.is_return_plenum = boolean(member(z, "IsReturnPlenum")?)?;
            zone.is_supply_plenum = boolean(member(z, "IsSupplyPlenum")?)?;
            zone.leakage_parallel_piu_numbers = array(member(z, "leakageParallelPIUNums")?)?
                .iter()
                .map(real)
                .collect::<Result<Vec<_>>>()?;
            zone.space_indexes = array(member(z, "spaceIndexes")?)?
                .iter()
                .map(integer)
                .collect::<Result<Vec<_>>>()?;
            owners.fan.sum_conv_ht_rad_sys = Some(vec![real(member(f, "SumConvHTRadSys")?)?]);
            owners.fan.sum_conv_pool = Some(vec![real(member(f, "SumConvPool")?)?]);
            owners.fan.sum_latent_ht_rad_sys = Some(vec![real(member(f, "SumLatentHTRadSys")?)?]);
            owners.fan.sum_latent_pool = Some(vec![real(member(f, "SumLatentPool")?)?]);
            let cfg = member(&entry, "config")?;
            let inlet = member(cfg, "InletNode")?;
            let inlets = array(member(inlet, "values")?)?;
            let real_cfg = owners.equipment.source_zone_equipment_config(1)?;
            assert_eq!(
                *real_cfg.is_controlled(),
                boolean(member(cfg, "IsControlled")?)?
            );
            assert_eq!(
                *real_cfg.number_inlet_nodes(),
                count(member(cfg, "NumInletNodes")?)?
            );
            assert_eq!(inlets.len(), *real_cfg.number_inlet_nodes());
            for raw in inlets {
                let slot = count(member(raw, "slot")?)?;
                let ordinal = integer(member(raw, "NodeNum")?)?;
                assert_eq!(
                    real_cfg.inlet_node_number(slot).copied(),
                    Some(usize::try_from(ordinal)?)
                );
            }
            let zone_ordinal = integer(member(&entry, "node_ordinal")?)?;
            owners.equipment.restore_native_cohort_node(
                zone_ordinal,
                text(member(&entry, "node_name")?)?,
                captured_node_values(member(&entry, "node")?)?,
            )?;
            let mut has_positive = false;
            let mut addresses = Vec::new();
            for raw in inlets {
                let p = member(raw, "physical")?;
                let ordinal = integer(member(p, "index")?)?;
                let v = member(p, "value")?;
                let values = captured_node_values(v)?;
                has_positive |= values[1] > 0.0;
                owners.equipment.restore_native_cohort_node(
                    ordinal,
                    text(member(p, "NodeID")?)?,
                    values,
                )?;
                let n = owners.equipment.source_zone_air_node_mut(ordinal)?;
                addresses.push(std::ptr::from_ref(&*n.temperature) as usize);
            }
            let zone_address = {
                let n = owners.equipment.source_zone_air_node_mut(zone_ordinal)?;
                std::ptr::from_ref(&*n.temperature) as usize
            };
            addresses.push(zone_address);
            assert!(
                addresses[..addresses.len() - 1]
                    .iter()
                    .all(|a| *a != zone_address)
            );
            if let Some(first) = &first_addresses {
                assert_eq!(
                    &addresses, first,
                    "same genuine physical rows across cohort"
                );
            } else {
                first_addresses = Some(addresses);
            }
            if has_positive {
                positive += 1;
            }
            let use_history = bool_integer(member(
                member(case, "arguments")?,
                "useZoneTimeStepHistory",
            )?)?;
            let change =
                air.correct_source_zone_temperature(1, use_history, controller, &mut owners)?;
            check_state(&air, controller, &mut owners, &expected, &context)?;
            check_corrector_air(&air, member(&expected, "air")?, &context)?;
            assert_real(
                change,
                member(member(case, "return_value")?, "tempChange")?,
                &context,
                "actual returned temperature change",
            )?;
            let expected_node = member(&expected, "node")?;
            assert_real(
                *owners
                    .equipment
                    .source_zone_air_node_mut(zone_ordinal)?
                    .mass_flow_rate,
                member(expected_node, "MassFlowRate")?,
                &context,
                "same ZoneNode.Mdot",
            )?;
            for raw in inlets {
                let p = member(raw, "physical")?;
                let n = owners
                    .equipment
                    .source_zone_air_node_mut(integer(member(p, "index")?)?)?;
                let v = member(p, "value")?;
                assert_real(
                    *n.temperature,
                    member(v, "Temp")?,
                    &context,
                    "inlet Temp unchanged",
                )?;
                assert_real(
                    *n.mass_flow_rate,
                    member(v, "MassFlowRate")?,
                    &context,
                    "inlet Mdot unchanged",
                )?;
                assert_real(
                    *n.humidity_ratio,
                    member(v, "HumRat")?,
                    &context,
                    "inlet W unchanged",
                )?;
                assert_real(
                    *n.enthalpy,
                    member(v, "Enthalpy")?,
                    &context,
                    "inlet H unchanged",
                )?;
                assert_real(
                    *n.temperature_setpoint,
                    member(v, "TempSetPoint")?,
                    &context,
                    "inlet Tset unchanged",
                )?;
            }
        }
        assert_eq!(positive, 810);
        Ok(())
    })
}

// Dedicated stable Cargo harness=false entry; this is the same cohort body.
impl SourceZoneAirPredictorCorrector {
    pub(crate) fn run_native_controlled_air_cohort() -> Result {
        all_native_b_prediction_entries_match_source_rollback_and_sensible_bits()
    }
}

// One consolidated target; the original complete air cohort above is unchanged.
mod report_cohort;
impl SourceZoneAirPredictorCorrector {
    pub(crate) fn run_native_system_report_cohort() -> Result<(usize, usize, usize)> {
        report_cohort::run()
    }
}

// Test-only full predictSystemLoad parent. Existing 2336-body cohort is unchanged.
mod full_predictor;
impl SourceZoneAirPredictorCorrector {
    pub(crate) fn run_native_full_predictor_parent_cohort() -> Result<usize> {
        full_predictor::run()
    }
}
