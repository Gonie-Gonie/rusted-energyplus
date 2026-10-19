// Test-feature-only passive OutputProcessor owner observation.
use super::*;
use serde_json::{Value, json};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn invalid(s: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, s)
}
fn number(v: f64) -> Value {
    json!({"value_bits":format!("{:016x}",v.to_bits())})
}
fn pointer<T>(v: &T) -> Value {
    json!(format!("{:p}", std::ptr::from_ref(v)))
}
fn ordinal(v: Option<usize>) -> Result<i64> {
    match v {
        Some(n) => Ok(i64::try_from(n)?),
        None => Ok(-1),
    }
}
fn shape<T>(v: &Option<Vec<T>>) -> Value {
    json!({"allocated":v.is_some(),"size":v.as_ref().map_or(0,Vec::len)})
}
fn strings(v: &Option<Vec<String>>) -> Value {
    let mut out = shape(v);
    out["lower"] = json!(1);
    out["upper"] = json!(v.as_ref().map_or(0, Vec::len));
    out["items"] = json!(v.as_deref().unwrap_or(&[]));
    out
}
// These five retained cells correspond to the Native unit's arbitrary named
// lvalues. They are neither Node/weather projections nor reported snapshots.
#[derive(Default)]
pub(crate) struct SourceP4Values {
    pub(crate) zone_average: f64,
    pub(crate) system_average: f64,
    pub(crate) system_energy: f64,
    pub(crate) system_integer: i32,
    pub(crate) zone_sum_integer: i32,
}
impl SourceRealOutputOwners for SourceP4Values {
    fn real_output(
        &self,
        target: SourceRealOutputTarget,
    ) -> std::result::Result<&f64, OrdinaryRadiationError> {
        match target {
            SourceRealOutputTarget::CohortReal { slot: 0 } => Ok(&self.zone_average),
            SourceRealOutputTarget::CohortReal { slot: 1 } => Ok(&self.system_average),
            SourceRealOutputTarget::CohortReal { slot: 2 } => Ok(&self.system_energy),
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
impl SourceIntegerOutputOwners for SourceP4Values {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> std::result::Result<&i32, OrdinaryRadiationError> {
        match target {
            SourceIntegerOutputTarget::CohortInteger { slot: 0 } => Ok(&self.system_integer),
            SourceIntegerOutputTarget::CohortInteger { slot: 1 } => Ok(&self.zone_sum_integer),
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
impl SourceP4Values {
    pub(crate) fn native_p4_observe(&self) -> Value {
        json!({"zone_average":number(self.zone_average),"system_average":number(self.system_average),"system_energy":number(self.system_energy),"system_integer":self.system_integer,"zone_sum_integer":self.zone_sum_integer,
            "addresses":{"zone_average":pointer(&self.zone_average),"system_average":pointer(&self.system_average),"system_energy":pointer(&self.system_energy),"system_integer":pointer(&self.system_integer),"zone_sum_integer":pointer(&self.zone_sum_integer)}})
    }
}
fn unit(v: SourceOutputUnits) -> i32 {
    use SourceOutputUnits as U;
    match v {
        U::Invalid => -1,
        U::Dimensionless => 6,
        U::C => 1,
        U::Pa => 4,
        U::M => 21,
        U::KgPerM3 => 36,
        U::Lux => 14,
        U::LumPerW => 15,
        U::Percentage => 33,
        U::Hr => 16,
        U::KgWaterPerSecond => 49,
        U::KgWaterPerKgDryAir => 2,
        U::JPerKg => 31,
        U::MPerS => 19,
        U::Deg => 34,
        U::M2 => 41,
        U::Rad => 43,
        U::W => 8,
        U::J => 9,
        U::Kg => 11,
        U::WPerM2 => 20,
        U::WPerM2K => 30,
        U::KgPerSecond => 0,
        U::M3PerSecond => 5,
        U::M3 => 10,
        U::Ach => 12,
        U::JPerKgK => 38,
        U::Ppm => 3,
    }
}
fn period(p: &meters::SourceMeterPeriod) -> Value {
    json!({"Value":number(p.value),"MaxVal":number(p.maximum),"MaxValDate":p.maximum_date,"MinVal":number(p.minimum),"MinValDate":p.minimum_date,"Rpt":p.report,"RptFO":p.report_file_only,"RptNum":p.report_number,"accRpt":p.accumulated_report,"accRptFO":p.accumulated_file_only,"accRptNum":p.accumulated_report_number})
}
// Literal passive JSON serialization of the existing same meter-frame cells.
// It calls no report/meter operation and restores no expected values.
fn frame_json(f: &SourceMeterDataFrame, only: bool) -> Result<Value> {
    let mut cols = Vec::new();
    let mut selected = Vec::new();
    for v in f.meter_map.values() {
        if only && v.meter_only {
            continue;
        }
        let units = v.units.ok_or_else(|| invalid("actual meter frame units"))?;
        cols.push(json!({"Variable":v.name,"Units":units.symbol()}));
        selected.push(v);
    }
    if cols.is_empty() {
        return Ok(Value::Null);
    }
    let rows = f
        .timestamps
        .iter()
        .enumerate()
        .map(|(i, time)| {
            let values = selected
                .iter()
                .map(|v| v.values.get(i).map_or(Value::Null, |x| number(*x)))
                .collect::<Vec<_>>();
            let mut row = serde_json::Map::new();
            row.insert(time.clone(), Value::Array(values));
            Value::Object(row)
        })
        .collect::<Vec<_>>();
    Ok(json!({"ReportFrequency":f.report_frequency,"Cols":cols,"Rows":rows}))
}
pub(crate) fn native_p4_results(
    r: &super::super::ordinary_output_control::SourceResultsFrameworkOptions,
) -> Result<Value> {
    if r.time_series_enabled {
        return Err(invalid("P4 genuine disabled variable-frame domain").into());
    }
    let mut frames = Vec::new();
    for f in &r.meter_frames {
        if f.beginning_of_interval {
            return Err(invalid("P4 default timestamp format").into());
        }
        frames.push(json!({"address":pointer(f),"enabled":f.enabled,"scanned":f.variables_scanned,"iso8601":f.iso8601,"beginningOfInterval":f.beginning_of_interval,"whole":frame_json(f,false)?,"without_meter_file_only":frame_json(f,true)?}));
    }
    Ok(
        json!({"owner_address":pointer(r),"timeSeriesEnabled":r.time_series_enabled,"timeSeriesAndTabularEnabled":r.time_series_and_tabular_enabled,"JSONEnabled":r.output_json,"CBOREnabled":r.output_cbor,"MsgPackEnabled":r.output_msg_pack,"RDD":r.report_data_dictionary,"MDD":r.meter_data_dictionary,"meters":frames}),
    )
}
impl SourceOutputRegistration {
    pub(crate) fn native_p4_get_report_input(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        host: &impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
    ) -> std::result::Result<(), OrdinaryRadiationError> {
        self.get_report_variable_input(input, host, eio)
    }
    pub(crate) fn native_p4_observe(
        &self,
        values: &SourceP4Values,
        owners: &LiveTimeStepOwners<'_>,
    ) -> Result<Value> {
        let mut vars = Vec::new();
        for v in &self.out_vars {
            if v.schedule.is_some() {
                return Err(invalid("P4 unselected scheduled request").into());
            }
            let mut row = json!({"address":pointer(v),"ddVarNum":ordinal(v.dictionary_number)?,"varType":v.variable_type as i32,"timeStepType":v.time_step_type as i32,"storeType":v.store_type as i32,
                "Value":number(v.value),"TSValue":number(v.time_step_value),"EITSValue":number(v.external_interface_time_step_value),"StoreValue":number(v.store_value),"NumStored":number(v.num_stored),"Stored":v.stored,"Report":v.report,"tsStored":v.time_step_stored,"thisTSStored":v.this_time_step_stored,"thisTSCount":v.this_time_step_count,"freq":v.frequency as i32,"MaxValue":number(v.max_value),"MinValue":number(v.min_value),"maxValueDate":v.max_value_date,"minValueDate":v.min_value_date,"ReportID":v.report_id,"sched":Value::Null,"ZoneMult":v.zone_multiplier,"ZoneListMult":v.zone_list_multiplier,
                "keyColonName":v.key_colon_name,"keyColonNameUC":v.key_colon_name_upper,"name":v.name,"nameUC":v.name_upper,"key":v.key,"keyUC":v.key_upper,"units":unit(v.units),"unitNameCustomEMS":v.custom_unit_name,"indexGroup":v.index_group,"indexGroupKey":v.index_group_key,"meterNums":v.meter_numbers});
            if let Some(target) = v.which {
                let cell = values.real_output(target)?;
                row["Which"] = pointer(cell);
                row["live_value"] = number(*cell);
                row["same_named_lvalue"] = json!(match target {
                    SourceRealOutputTarget::CohortReal { slot: 0 } => "zone_average",
                    SourceRealOutputTarget::CohortReal { slot: 1 } => "system_average",
                    SourceRealOutputTarget::CohortReal { slot: 2 } => "system_energy",
                    _ => return Err(invalid("P4 real field identity").into()),
                });
            } else if let Some(target) = v.integer_which {
                let cell = values.integer_output(target)?;
                row["Which"] = pointer(cell);
                row["live_value"] = json!(*cell);
                row["same_named_lvalue"] = json!(match target {
                    SourceIntegerOutputTarget::CohortInteger { slot: 0 } => "system_integer",
                    SourceIntegerOutputTarget::CohortInteger { slot: 1 } => "zone_sum_integer",
                    _ => return Err(invalid("P4 integer field identity").into()),
                });
            } else {
                return Err(invalid("P4 actual Which binding").into());
            }
            vars.push(row);
        }
        let dictionary=self.dictionary_variables.iter().map(|d|Ok(json!({"address":pointer(d),"name":d.name,"timeStepType":d.time_step_type as i32,"storeType":d.store_type as i32,"variableType":if d.integer{0}else{1},"Next":ordinal(d.next)?,"ReportedOnDDFile":d.reported_on_dictionary_file,"units":unit(d.units),"keyOutVarNums":d.key_out_var_nums}))).collect::<Result<Vec<_>>>()?;
        let mut requests = Vec::new();
        for r in &self.requested_variables {
            if r.reporting_schedule.is_some() || !r.is_simple_string {
                return Err(invalid("P4 real unscheduled literal request").into());
            }
            requests.push(json!({"address":pointer(r),"key":r.key,"name":r.name,"freq":r.frequency as i32,"sched":Value::Null,"Used":r.used,"is_simple_string":r.is_simple_string}));
        }
        let mut meters = Vec::new();
        for m in &self.meters.meters {
            let rt = m
                .ip_units
                .ok_or_else(|| invalid("P4 actual initialized IP units"))?;
            meters.push(json!({"address":pointer(m),"Name":m.name,"type":m.meter_type as i32,"resource":m.resource as i32,"endUseCat":m.end_use as i32,"EndUseSub":m.end_use_subcategory,"group":m.group as i32,"units":unit(m.units),"RT_forIPUnits":rt as i32,"CurTSValue":number(m.current_time_step_value),"indexGroup":m.index_group,"dstMeterNums":m.destination_meter_numbers,"decMeterNum":m.decrement_meter_number,"srcVarNums":m.source_variable_numbers,"srcMeterNums":m.source_meter_numbers,"periods":m.periods.iter().map(period).collect::<Vec<_>>(),"periodLastSM":period(&m.period_last_simulation),"periodFinYrSM":period(&m.period_final_year_simulation)}));
        }
        let mut categories = shape(&self.end_use_categories);
        categories["items"]=Value::Array(self.end_use_categories.as_deref().unwrap_or(&[]).iter().map(|v|json!({"Name":v.name,"DisplayName":v.display_name,"NumSubcategories":v.num_subcategories,"SubcategoryName":strings(&v.subcategory_names),"numSpaceTypes":v.num_space_types,"spaceTypeName":strings(&v.space_type_names)})).collect());
        let time=[(&self.time_pointers.zone,TimeStepKind::Zone),(&self.time_pointers.system,TimeStepKind::System)].iter().map(|(v,kind)|{
            let mut row=json!({"address":pointer(*v),"TimeStep":Value::Null,"value":Value::Null,"CurMinute":number(v.current_minute),"is_actual_zone_pointer":false,"is_actual_system_pointer":false});
            if v.target.is_some(){let cell=self.time_pointers.time_step(*kind,owners)?;row["TimeStep"]=pointer(cell);row["value"]=number(*cell);row["is_actual_zone_pointer"]=json!(std::ptr::eq(cell,&owners.global.time_step_zone));row["is_actual_system_pointer"]=json!(std::ptr::eq(cell,&owners.hvac.time_step_sys));}
            Ok(row)
        }).collect::<Result<Vec<_>>>()?;
        Ok(
            json!({"owner_address":pointer(self),"NumTotalRVariable":self.num_total_real_variables,"NumOfRVariable":self.num_real_variables,"NumOfRVariable_Setup":self.num_real_variables_setup,"NumOfRVariable_Sum":self.num_real_variables_sum,"NumOfRVariable_Meter":self.num_real_variables_meter,"NumOfIVariable":self.num_integer_variables,"NumOfIVariable_Setup":self.num_integer_variables_setup,"NumTotalIVariable":self.num_total_integer_variables,"NumOfIVariable_Sum":self.num_integer_variables_sum,
            "OutputInitialized":self.output_initialized,"ProduceReportVDD":self.produce_report_vdd as i32,"NumHoursInMonth":self.number_hours_in_month,"NumHoursInSim":self.number_hours_in_simulation,"freqStampReportNums":self.frequency_stamp_report_numbers,"freqTrackingVariables":self.frequency_tracking_variables,"TimeStepZoneSec":number(self.time_step_zone_sec),"ErrorsLogged":self.errors_logged,"isFinalYear":self.is_final_year,"GetOutputInputFlag":self.get_output_input,"minimumReportFreq":self.minimum_report_frequency as i32,"ReportNumberCounter":self.report_number_counter,"LHourP":self.last_hour_printed,"LStartMin":number(self.last_start_minute),"LEndMin":number(self.last_end_minute),"OpaqSurfWarned":self.opaque_surface_warning,"MaxNumSubcategories":self.max_num_subcategories,"maxNumEndUseSpaceTypes":self.max_num_end_use_space_types,"ddOutVarMap":self.dictionary_by_name,"meterMap":self.meters.meter_map,
            "time_values":time,"outVars":vars,"ddOutVars":dictionary,"reqVars":requests,"meters":meters,"meterValues":self.meter_values.iter().map(|x|number(*x)).collect::<Vec<_>>(),"EndUseCategory":categories}),
        )
    }
}
