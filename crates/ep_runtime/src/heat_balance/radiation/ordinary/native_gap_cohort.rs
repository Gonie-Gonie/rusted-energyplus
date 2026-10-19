//! One existing process-main cohort child; all expectations require real Native output.
use super::{OrdinarySolarRadiation, OrdinaryRadiationInput, RadiantRadiationEnclosure};
use super::radiant_exchange::RadiantExchangeCall;
use crate::heat_balance::error_backend::{SourceErrorBackend, SourceErrorState, SourceErrorTracking};
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::{SelectedZoneInputOwners, SelectedZoneData};
use crate::heat_balance::selected_heat_balance_input::{SelectedHeatBalanceInputState, SelectedConstructionUsage};
use serde_json::Value;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn array(v:&Value)->Result<&Vec<Value>> { v.as_array().ok_or_else(||"actual Native array absent".into()) }
fn real(v:&Value)->Result<f64> {
    let s=v["value_bits"].as_str().ok_or("raw f64 input absent")?;
    if s.len()!=16 || !s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) { return Err("raw f64 input invalid".into()); }
    let x=f64::from_bits(u64::from_str_radix(s,16)?);
    if !x.is_finite() { return Err("nonfinite primary input outside cohort".into()); }
    Ok(x)
}
fn bits(v:f64,e:&Value,label:&str)->Result<()> {
    let text=e["value_bits"].as_str().ok_or("actual expected bits absent")?;
    if format!("{:016x}",v.to_bits())!=text { return Err(format!("{label}: Source {:016x} != Native {text}",v.to_bits()).into()); }
    Ok(())
}
fn values(v:&[f64],e:&Value,label:&str)->Result<()> {
    let e=array(e)?;
    if v.len()!=e.len() { return Err(format!("{label}: extent differs").into()); }
    for (a,b) in v.iter().zip(e) { bits(*a,b,label)?; }
    Ok(())
}
fn vector(v:&[f64],e:&Value,label:&str)->Result<()> {
    if e["allocated"].as_bool()!=Some(true) || e["size"].as_u64()!=Some(v.len() as u64)
        || e["lower"].as_i64()!=Some(1) || e["upper"].as_u64()!=Some(v.len() as u64) { return Err(format!("{label}: Native allocation/bounds differ").into()); }
    values(v,&e["values"],label)
}
fn input_matrix(e:&Value)->Result<Vec<f64>> {
    let columns=array(e)?;let n=columns.len();let mut v=Vec::with_capacity(n*n);
    if columns.iter().any(|c|c.as_array().map(Vec::len)!=Some(n)) { return Err("square primary matrix required".into()); }
    // Array2(i,j) physical index is i*n+j, while the passive JSON deliberately
    // groups its second index as columns. This is a storage transport only.
    for i in 0..n { for column in columns { v.push(real(&column[i])?); } }
    Ok(v)
}
fn matrix(v:&[f64],n:usize,e:&Value,label:&str)->Result<()> {
    if e["allocated"].as_bool()!=Some(true) || e["size"].as_u64()!=Some((n*n) as u64)
        || e["lower1"].as_i64()!=Some(1) || e["lower2"].as_i64()!=Some(1)
        || e["upper1"].as_u64()!=Some(n as u64) || e["upper2"].as_u64()!=Some(n as u64) { return Err(format!("{label}: Native matrix bounds differ").into()); }
    let cols=array(&e["columns"])?;
    if cols.len()!=n || v.len()!=n*n || cols.iter().any(|c|c.as_array().map(Vec::len)!=Some(n)) { return Err("matrix observation extent differs".into()); }
    for i in 0..n { for j in 0..n { bits(v[i*n+j],&cols[j][i],label)?; } }
    Ok(())
}
fn scratch(v:Option<&[Option<f64>]>,e:&Value,label:&str)->Result<()> {
    if e["allocated"].as_bool()!=Some(v.is_some()) { return Err(format!("{label}: allocated state differs").into()); }
    let Some(v)=v else {
        if e["size"].as_u64()!=Some(0) || !e["values"].is_null() { return Err("unallocated scratch observation differs".into()); }
        return Ok(());
    };
    if e["size"].as_u64()!=Some(v.len() as u64) { return Err("scratch shape differs".into()); }
    if e["values_defined"].as_bool()==Some(false) {
        if v.iter().any(Option::is_some) || !e["values"].is_null() { return Err("first-kickoff unwritten scratch mask differs".into()); }
        return Ok(());
    }
    if e["values_defined"].as_bool()!=Some(true) { return Err("Native definedness evidence absent".into()); }
    let expected=array(&e["values"])?;
    if expected.len()!=v.len() { return Err("written scratch shape differs".into()); }
    for (value,expected) in v.iter().zip(expected) { bits(value.ok_or("Source scratch producer has not written")?,expected,label)?; }
    Ok(())
}
fn cold_errors()->SourceErrorState {
    SourceErrorState {output_error_header:true,version_string:String::new(),
        idd_version_string:String::new(),doing_input_processing:false,print_console_output:true,
        kickoff_simulation:false,developer_flag:false,warmup_flag:false,doing_sizing:false,
        tracking:SourceErrorTracking {ask_for_plant_check_on_abort:false,exit_during_simulations:false,
            ask_for_connections_report:false,ask_for_surfaces_report:false,total_multiplied_windows:0,
            total_coincident_vertices:0,total_degenerate_surfaces:0,total_receiving_non_convex_surfaces:0,
            total_casting_non_convex_surfaces:0,match_counts:[0;20],abort_processing:false,
            total_severe_errors:0,total_severe_errors_during_warmup:0,total_severe_errors_during_sizing:0,
            last_severe_error:String::new(),total_warning_errors:0,
            total_warning_errors_during_warmup:0,total_warning_errors_during_sizing:0}}
}
fn backend<'a>(state:&'a mut SourceErrorState,bytes:&'a mut Vec<u8>,console:&'a mut Vec<u8>)->SourceErrorBackend<'a> {
    SourceErrorBackend {state,error_stream:Some(bytes),console,out_unit1:None,out_unit2:None,
        f_message:None,message_callback:None,error_callback:None,sqlite:None}
}
fn diagnostics(state:&SourceErrorState,bytes:&[u8],e:&Value)->Result<()> {
    let t=&state.tracking;
    for (k,v) in [("warnings",t.total_warning_errors),("severe",t.total_severe_errors),
        ("warmup_warnings",t.total_warning_errors_during_warmup),("warmup_severe",t.total_severe_errors_during_warmup),
        ("sizing_warnings",t.total_warning_errors_during_sizing),("sizing_severe",t.total_severe_errors_during_sizing)] {
        if e[k].as_i64()!=Some(i64::from(v)) { return Err(format!("diagnostic {k} count differs").into()); }
    }
    if e["recurring"].as_i64()!=Some(0) { return Err("unexpected Native recurring path outside cohort".into()); }
    let source_text=std::str::from_utf8(bytes)?;
    let native_text=e["text"].as_str().ok_or("Native diagnostics absent")?;
    for text in [source_text,native_text] {
        if !text.is_empty() {
            let (header,body)=text.split_once('\n').ok_or("complete diagnostic header absent")?;
            if header!="Program Version,," || body.contains("Program Version,") { return Err("cold constructor diagnostic header/schema/position differs".into()); }
        }
    }
    if state.output_error_header!=source_text.is_empty() { return Err("actual diagnostic header latch differs".into()); }
    if source_text!=native_text { return Err("entire raw diagnostic text differs".into()); }
    Ok(())
}

impl OrdinarySolarRadiation {
    fn assert_native_gap_radiation(
        &self,geometry:&SurfaceGeometryInputState,net:&[f64],state:&SourceErrorState,bytes:&[u8],e:&Value,
    )->Result<()> {
        let r=&self.radiant_exchange;let enclosure=self.radiant.first().ok_or("Source radiant enclosure absent")?;
        let n=usize::try_from(enclosure.num_surfaces)?;
        if e["first_exchange"].as_bool()!=Some(r.first_exchange_call)
            || e["max_enclosure_surfaces"].as_i64()!=Some(i64::from(r.max_num_enclosure_surfaces))
            || e["largest_surface"].as_i64()!=Some(i64::from(r.largest_surface)) { return Err("persistent radiation module fields differ".into()); }
        vector(enclosure.area.as_deref().ok_or("area absent")?,&e["Area"],"Area")?;
        matrix(enclosure.f.as_deref().ok_or("F absent")?,n,&e["F"],"F")?;
        vector(enclosure.emissivity.as_deref().ok_or("emissivity absent")?,&e["Emissivity"],"Emissivity")?;
        matrix(enclosure.script_f.as_deref().ok_or("ScriptF absent")?,n,&e["ScriptF"],"ScriptF")?;
        let pointer=enclosure.source_surface_ptr.as_deref().ok_or("SurfacePtr absent")?;
        let expected=array(&e["SurfacePtr"]["values"])?;
        if e["SurfacePtr"]["allocated"].as_bool()!=Some(true) || expected.len()!=pointer.len()
            || pointer.iter().zip(expected).any(|(a,b)|b.as_i64()!=Some(i64::from(*a))) { return Err("actual Source SurfacePtr differs".into()); }
        scratch(r.surface_temp_rad.as_deref(),&e["SurfaceTempRad"],"SurfaceTempRad")?;
        scratch(r.surface_temp_in_kelvin_fourth.as_deref(),&e["SurfaceTempInKto4th"],"SurfaceTempInKto4th")?;
        scratch(r.surface_emissivity.as_deref(),&e["SurfaceEmiss"],"SurfaceEmiss")?;
        vector(net,&e["NetLW"],"NetLW")?;
        vector(&geometry.window_arrays.as_ref().ok_or("window owner absent")?.surf_win_i_rfrom_parent_zone,&e["IRfromParentZone"],"IRfromParentZone")?;
        diagnostics(state,bytes,&e["diagnostics"])
    }

    /// All corrected named lifetimes, each retained across Fix→ScriptF→every
    /// original exchange. No previous expected exit is ever restored.
    pub(crate) fn run_native_gap_radiation(input:&Value,native:&Value)->Result<usize> {
        let cases=array(&input["radiation"])?;let observed=array(&native["radiation"])?;
        if cases.len()!=observed.len() { return Err("radiation case count differs".into()); }
        let mut count=0;
        for (case,oracle) in cases.iter().zip(observed) {
            if case!=&oracle["input"] || case["name"]!=oracle["name"] { return Err("radiation primary identity differs".into()); }
            let area=array(&case["areas"])?.iter().map(real).collect::<Result<Vec<_>>>()?;
            let emissivity=array(&case["emissivity"])?.iter().map(real).collect::<Result<Vec<_>>>()?;
            let n=area.len();if !(2..=8).contains(&n) || emissivity.len()!=n { return Err("authored enclosure shape differs".into()); }
            let name=case["name"].as_str().ok_or("case name absent")?;
            let f=input_matrix(&case["F_columns"])?;
            let input=OrdinaryRadiationInput::from_admitted_inventory(std::iter::empty::<(&str,usize)>(),false)?;
            let mut owner=Self::for_surface_input(input);
            owner.num_radiant_enclosures=1;
            owner.radiant_exchange.max_num_enclosure_surfaces=i32::try_from(n)?;
            owner.radiant.push(RadiantRadiationEnclosure {name:name.into(),space_nums:vec![1],num_surfaces:i32::try_from(n)?,
                area:Some(area),f:Some(f),emissivity:Some(emissivity),script_f:Some(vec![0.0;n*n]),
                source_surface_ptr:Some((1..=n).map(i32::try_from).collect::<std::result::Result<Vec<_>,_>>()?),..Default::default()});
            let mut zones=SelectedZoneInputOwners::default();
            zones.zones=Some(vec![SelectedZoneData {name:name.into(),space_indexes:vec![1],..Default::default()}]);
            zones.spaces=Some(vec![Default::default()]);
            {
                let space=zones.spaces.as_mut().and_then(|v|v.first_mut()).ok_or("actual Space row absent")?;
                space.name=name.into();
                space.zone_num=1;
            }
            let mut geometry=SurfaceGeometryInputState::native_gap_opaque_radiation_rows(n)?;
            let mut control=SelectedHeatBalanceInputState::default();
            control.construction_usage=owner.radiant[0].emissivity.as_ref().ok_or("emissivity absent")?.iter()
                .map(|v|SelectedConstructionUsage {inside_absorp_thermal:*v,..Default::default()}).collect();
            let mut net=vec![0.0;n];let mut error_state=cold_errors();let mut error_bytes=Vec::new();let mut console=Vec::new();
            let calls=array(&oracle["calls"])?;
            if calls.len()!=2+array(&case["exchanges"])?.len() { return Err("radiation full call count differs".into()); }
            let fix=&calls[0];if fix["operation"]!="FixViewFactors" { return Err("Fix order differs".into()); }
            owner.assert_native_gap_radiation(&geometry,&net,&error_state,&error_bytes,&fix["before"])?;
            let mut checks=super::super::fix_view_factors::SourceViewFactorChecks {original:f64::NAN,fixed:f64::NAN,
                final_value:f64::NAN,iterations:-1,row_sum:f64::NAN};
            {
                let e=&mut owner.radiant[0];
                owner.radiant_exchange.fix_source_view_factors(&mut zones,e.area.as_deref().ok_or("area absent")?,
                    e.f.as_deref_mut().ok_or("F absent")?,name,&e.space_nums,&mut checks,
                    case["any_internal_mass"].as_bool().ok_or("intmass primary absent")?,
                    &mut backend(&mut error_state,&mut error_bytes,&mut console))?;
            }
            for (v,key) in [(checks.original,"OriginalCheckValue"),(checks.fixed,"FixedCheckValue"),
                (checks.final_value,"FinalCheckValue"),(checks.row_sum,"RowSum")] { bits(v,&fix[key],key)?; }
            if fix["NumIterations"].as_i64()!=Some(i64::from(checks.iterations))
                || fix["EnforcedReciprocity"].as_bool()!=Some(zones.zones.as_ref().ok_or("zones absent")?[0].enforced_reciprocity) { return Err("Fix persistent/output arguments differ".into()); }
            owner.assert_native_gap_radiation(&geometry,&net,&error_state,&error_bytes,&fix["after"])?;
            let script=&calls[1];if script["operation"]!="CalcScriptF" { return Err("ScriptF order differs".into()); }
            owner.assert_native_gap_radiation(&geometry,&net,&error_state,&error_bytes,&script["before"])?;
            {
                let e=&mut owner.radiant[0];let mut output=backend(&mut error_state,&mut error_bytes,&mut console);
                let result=super::super::energyplus_scriptf_from_view_factors_with_warning(
                    e.area.as_deref().ok_or("area absent")?,e.f.as_deref().ok_or("F absent")?,e.emissivity.as_deref_mut().ok_or("emissivity absent")?,
                    ||output.show_warning_error("A thermal emissivity above 0.99999 was detected. This is not allowed. Value was reset to 0.99999"))?
                    .ok_or("actual ScriptF route unavailable")?;
                e.script_f.as_mut().ok_or("ScriptF owner absent")?.copy_from_slice(&result);
            }
            owner.assert_native_gap_radiation(&geometry,&net,&error_state,&error_bytes,&script["after"])?;
            count+=2;
            for (step,expected) in array(&case["exchanges"])?.iter().zip(&calls[2..]) {
                if step!=&expected["primary_step"] || expected["operation"]!="CalcInteriorRadExchange" { return Err("exchange primary/order differs".into()); }
                owner.assert_native_gap_radiation(&geometry,&net,&error_state,&error_bytes,&expected["before"])?;
                let temperature=array(&step["temperature"])?.iter().map(real).collect::<Result<Vec<_>>>()?;
                vector(&temperature,&expected["temperature"],"actual supplied temperature")?;
                error_state.kickoff_simulation=step["kickoff"].as_bool().ok_or("kickoff absent")?;
                owner.exchange_source_radiant(&mut geometry,&control,&temperature,
                    &control.construction_usage.iter().map(|v|v.inside_absorp_thermal).collect::<Vec<_>>(),&mut net,
                    RadiantExchangeCall {surface_iterations:i32::try_from(step["surface_iterations"].as_i64().ok_or("iterations absent")?)?,
                        begin_environment:step["begin_environment"].as_bool().ok_or("begin env absent")?,
                        kickoff_simulation:step["kickoff"].as_bool().ok_or("kickoff absent")?,kickoff_sizing:false,
                        any_movable_insulation:false,carroll_method:false},
                    &mut backend(&mut error_state,&mut error_bytes,&mut console))?;
                owner.assert_native_gap_radiation(&geometry,&net,&error_state,&error_bytes,&expected["after"])?;
                count+=1;
            }
        }
        Ok(count)
    }

    /// Existing actual CalcMatrixInverse kernel; no copied LU implementation.
    pub(crate) fn run_native_gap_inverse(input:&Value,native:&Value)->Result<usize> {
        let cases=array(&input["inverse"])?;let observed=array(&native["inverse"])?;
        if cases.len()!=observed.len() { return Err("inverse case count differs".into()); }
        for (case,oracle) in cases.iter().zip(observed) {
            if case!=&oracle["input"] || case["name"]!=oracle["name"] { return Err("inverse primary identity differs".into()); }
            let n=array(&case["A_columns"])?.len();let mut a=input_matrix(&case["A_columns"])?;
            matrix(&a,n,&oracle["A_before"],"actual inverse entry")?;
            let inverse=super::super::invert_square_matrix(&mut a,n).ok_or("actual inverse domain unavailable")?;
            matrix(&a,n,&oracle["A_after"],"actual LU mutation")?;
            matrix(&inverse,n,&oracle["I_after"],"actual returned inverse")?;
        }
        Ok(cases.len())
    }
}

mod absorption;

mod sky_opaque;
