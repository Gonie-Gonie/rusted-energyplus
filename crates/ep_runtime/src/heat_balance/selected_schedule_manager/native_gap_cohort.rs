//! One shared process-main cohort child. Expectations enter only from actual Native output.
use super::minute::{
    SourceScheduleInterpolation, populate_source_day_from_minutes, process_source_interval_fields,
};
use super::{SelectedDaySchedule, SelectedScheduleManager};
use crate::heat_balance::error_backend::{SourceErrorBackend, SourceErrorState, SourceErrorTracking};
use serde_json::Value;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn array(v: &Value) -> Result<&Vec<Value>> { v.as_array().ok_or_else(|| "Native group array absent".into()) }
fn real(v: &Value) -> Result<f64> {
    let text=v["value_bits"].as_str().ok_or("raw f64 bits absent")?;
    if text.len()!=16 || !text.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {
        return Err("raw f64 encoding invalid".into());
    }
    let value=f64::from_bits(u64::from_str_radix(text,16)?);
    if !value.is_finite() { return Err("nonfinite named input outside authored cohort".into()); }
    Ok(value)
}
fn bits(actual:f64, expected:&Value, name:&str)->Result<()> {
    let expected=expected["value_bits"].as_str().ok_or("expected actual Native f64 absent")?;
    if format!("{:016x}",actual.to_bits())!=expected {
        return Err(format!("{name}: Source {:016x} != actual Native {expected}",actual.to_bits()).into());
    }
    Ok(())
}
fn values(actual:&[f64],expected:&Value,name:&str)->Result<()> {
    let expected=array(expected)?;
    if actual.len()!=expected.len() { return Err(format!("{name}: actual array extent differs").into()); }
    for (a,b) in actual.iter().zip(expected) { bits(*a,b,name)?; }
    Ok(())
}
fn cold_errors()->SourceErrorState {
    SourceErrorState {
        output_error_header:true,
        // Actual fresh Native DataStringGlobals std::string constructors;
        // neither narrow factory initializes or substitutes either version.
        version_string:String::new(),idd_version_string:String::new(),
        doing_input_processing:false,print_console_output:true,kickoff_simulation:false,
        developer_flag:false,warmup_flag:false,doing_sizing:false,
        tracking:SourceErrorTracking {
            ask_for_plant_check_on_abort:false,exit_during_simulations:false,
            ask_for_connections_report:false,ask_for_surfaces_report:false,
            total_multiplied_windows:0,total_coincident_vertices:0,total_degenerate_surfaces:0,
            total_receiving_non_convex_surfaces:0,total_casting_non_convex_surfaces:0,
            match_counts:[0;20],abort_processing:false,total_severe_errors:0,
            total_severe_errors_during_warmup:0,total_severe_errors_during_sizing:0,
            last_severe_error:String::new(),total_warning_errors:0,
            total_warning_errors_during_warmup:0,total_warning_errors_during_sizing:0,
        },
    }
}

impl SelectedScheduleManager {
    /// All named minute cases, actual retained DaySchedule and real backend.
    pub(crate) fn run_native_gap_schedule_minute(input:&Value,native:&Value)->Result<usize> {
        let inputs=array(&input["schedule_minute"])?;
        let actual=array(&native["schedule_minute"])?;
        if inputs.len()!=actual.len() { return Err("minute cohort case count differs".into()); }
        let mut calls=0;
        for (case,oracle) in inputs.iter().zip(actual) {
            if case["name"]!=oracle["name"] || case!=&oracle["input"] {
                return Err("minute actual Native primary-input identity differs".into());
            }
            let steps=usize::try_from(case["steps_per_hour"].as_u64().ok_or("steps absent")?)?;
            if steps==0 || steps>60 || 60%steps!=0 { return Err("minute integral step domain".into()); }
            let (mode,ordinal)=match case["mode"].as_str().ok_or("minute mode absent")? {
                "No"=>(SourceScheduleInterpolation::No,0),
                "Average"=>(SourceScheduleInterpolation::Average,1),
                "Linear"=>(SourceScheduleInterpolation::Linear,2),
                _=>return Err("minute mode invalid".into()),
            };
            // Native DaySchedule constructor, then std::vector resize(24*steps).
            let mut day=SelectedDaySchedule {name:String::new(),number:0,is_used:false,
                min_value:0.0,max_value:0.0,is_min_max_set:false,schedule_type_number:None,
                interpolation:ordinal,timestep_values:vec![0.0;24*steps],sum_timestep_values:0.0};
            let mut state=cold_errors();let mut error_bytes=Vec::new();let mut console=Vec::new();
            let source_calls=array(&case["calls"])?;let native_calls=array(&oracle["calls"])?;
            if source_calls.len()!=native_calls.len() { return Err("minute call count differs".into()); }
            for (call,expected) in source_calls.iter().zip(native_calls) {
                if call!=&expected["primary_call"] { return Err("minute raw call identity differs".into()); }
                // Assert the actual retained state BEFORE the next call. It is
                // never replaced by a prior expected Native exit.
                values(&day.timestep_values,&expected["before"]["tsVals"],"day entry tsVals")?;
                bits(day.sum_timestep_values,&expected["before"]["sumTsVals"],"day entry sum")?;
                let untils=array(&call["untils"])?.iter().map(|v|v.as_str().map(str::to_owned).ok_or("minute Until input absent")).collect::<std::result::Result<Vec<_>,_>>()?;
                let scalar_values=array(&call["values"])?.iter().map(real).collect::<Result<Vec<_>>>()?;
                let mut minute_values=[f64::NAN;1440];let mut set_minutes=[true;1440];let mut errors_found=false;
                {
                    let mut backend=SourceErrorBackend {state:&mut state,error_stream:Some(&mut error_bytes),
                        console:&mut console,out_unit1:None,out_unit2:None,f_message:None,
                        message_callback:None,error_callback:None,sqlite:None};
                    process_source_interval_fields(&untils,&scalar_values,untils.len(),scalar_values.len(),
                        &mut minute_values,&mut set_minutes,&mut errors_found,
                        case["name"].as_str().ok_or("day input name absent")?,"NativeGapBatch",mode,
                        i32::try_from(60/steps)?,&mut backend)?;
                }
                if expected["ErrorsFound"].as_bool()!=Some(errors_found) { return Err("minute ErrorsFound differs".into()); }
                values(&minute_values,&expected["minute"],"full1440 minute output")?;
                let expected_set=array(&expected["set_minutes"])?;
                if expected_set.len()!=1440 || set_minutes.iter().zip(expected_set).any(|(a,b)|Some(*a)!=b.as_bool()) {
                    return Err("full1440 set-minute output differs".into());
                }
                if !errors_found { populate_source_day_from_minutes(&mut day,&minute_values,steps)?; }
                if expected["populate_called"].as_bool()!=Some(!errors_found) { return Err("populate call-order differs".into()); }
                values(&day.timestep_values,&expected["tsVals"],"day exit tsVals")?;
                bits(day.sum_timestep_values,&expected["sumTsVals"],"retained day exit sum")?;
                let diagnostics=&expected["diagnostics"];let t=&state.tracking;
                for (name,value) in [("warnings",t.total_warning_errors),("severe",t.total_severe_errors),
                    ("warmup_warnings",t.total_warning_errors_during_warmup),("warmup_severe",t.total_severe_errors_during_warmup),
                    ("sizing_warnings",t.total_warning_errors_during_sizing),("sizing_severe",t.total_severe_errors_during_sizing)] {
                    if diagnostics[name].as_i64()!=Some(i64::from(value)) { return Err(format!("minute diagnostic counter {name} differs").into()); }
                }
                if diagnostics["recurring"].as_i64()!=Some(0) { return Err("actual Native minute unexpectedly used recurrence".into()); }
                let source_text=std::str::from_utf8(&error_bytes)?;
                let native_text=diagnostics["text"].as_str().ok_or("Native diagnostics text absent")?;
                for text in [source_text,native_text] {
                    if !text.is_empty() {
                        let (header,body)=text.split_once('\n').ok_or("complete minute diagnostic header absent")?;
                        if header!="Program Version,," || body.contains("Program Version,") { return Err("minute cold constructor header/schema/position differs".into()); }
                    }
                }
                if state.output_error_header!=source_text.is_empty() { return Err("minute actual header latch differs".into()); }
                if source_text!=native_text { return Err("minute entire raw diagnostic text differs".into()); }
                calls+=1;
            }
        }
        Ok(calls)
    }
}
