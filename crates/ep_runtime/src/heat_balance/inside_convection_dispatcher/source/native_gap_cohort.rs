//! Full real InitExt named lifetimes, cold getters retained; no geometry producer claim.
use super::*;
use crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentState;
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::error_backend::{SourceErrorState,SourceErrorTracking};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
use crate::weather::day::WeatherEnvironmentState;
use ep_model::{MaterialSurfaceRoughness as Roughness,OutsideSurfaceConvectionAlgorithm as Outside};
use serde_json::Value;
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
fn array(v:&Value)->Result<&Vec<Value>> {v.as_array().ok_or_else(||"Native InitExt array absent".into())}
fn real(v:&Value)->Result<f64> {
    let s=v["value_bits"].as_str().ok_or("raw InitExt primary absent")?;
    if s.len()!=16 || !s.bytes().all(|c|c.is_ascii_digit()||(b'a'..=b'f').contains(&c)){return Err("raw input encoding".into());}
    let x=f64::from_bits(u64::from_str_radix(s,16)?);if !x.is_finite(){return Err("nonfinite declared primary".into());}Ok(x)
}
fn bits(x:f64,v:&Value,name:&str)->Result<()> {
    if v["value_bits"].as_str()!=Some(format!("{:016x}",x.to_bits()).as_str()){return Err(format!("InitExt {name} actual bits differ").into());}Ok(())
}
fn cold_errors()->SourceErrorState {
    SourceErrorState {output_error_header:true,version_string:String::new(),idd_version_string:String::new(),
        doing_input_processing:false,print_console_output:true,kickoff_simulation:false,developer_flag:false,warmup_flag:false,doing_sizing:false,
        tracking:SourceErrorTracking {ask_for_plant_check_on_abort:false,exit_during_simulations:false,ask_for_connections_report:false,
            ask_for_surfaces_report:false,total_multiplied_windows:0,total_coincident_vertices:0,total_degenerate_surfaces:0,
            total_receiving_non_convex_surfaces:0,total_casting_non_convex_surfaces:0,match_counts:[0;20],abort_processing:false,
            total_severe_errors:0,total_severe_errors_during_warmup:0,total_severe_errors_during_sizing:0,last_severe_error:String::new(),
            total_warning_errors:0,total_warning_errors_during_warmup:0,total_warning_errors_during_sizing:0}}
}
impl SourceInsideConvection {
    fn assert_native_gap_external(&self,geometry:&SurfaceGeometryInputState,zones:&SelectedZoneInputOwners,state:&SourceErrorState,bytes:&[u8],e:&Value)->Result<()> {
        let surface_model=geometry.parallel_arrays.as_ref().and_then(|p|p.exterior_convection.first()).ok_or("same actual exterior model owner")?.model;
        let zone=zones.zones.as_ref().and_then(|r|r.first()).ok_or("same actual zone model owner")?;
        let zone_model=match zone.outside_algorithm {Outside::SimpleCombined=>3,Outside::Tarp=>4,Outside::MoWitt=>6,Outside::Doe2=>7,Outside::AdaptiveConvectionAlgorithm=>9};
        if e["GetUserSuppliedConvectionCoeffs"].as_bool()!=Some(self.state.get_user_input)
            || e["actual_surface_model"].as_i64()!=Some(i64::from(surface_model)) || e["actual_zone_model"].as_i64()!=Some(zone_model) {
            return Err("InitExt actual flag/model differs".into());
        }
        for (i,key) in ["inside_user_curves","outside_user_curves","inside_user_model_storage","outside_user_model_storage"].iter().enumerate(){
            let v=&self.state.user_arrays[i];let native=&e[*key];
            if native["allocated"].as_bool()!=Some(v.is_some()) || native["size"].as_u64()!=Some(v.as_ref().map_or(0,Vec::len) as u64){return Err(format!("InitExt {key} storage differs").into());}
            if i<2 {
                // True Array1D declarations retain explicit 1..0 bounds even
                // before allocation; EPVector model rows expose no l/u API.
                if native["lower"].as_i64()!=Some(1)||native["upper"].as_i64()!=Some(v.as_ref().map_or(0,Vec::len) as i64){return Err("InitExt Array1D bounds schema differs".into());}
            } else if native.get("lower").is_some()||native.get("upper").is_some(){return Err("InitExt EPVector must have no Objexx bounds fields".into());}
        }
        for (i,key) in ["user_inside_models","user_outside_models"].iter().enumerate(){
            if e[*key].as_u64()!=Some(self.state.user_model_counts[i] as u64){return Err("InitExt model count differs".into());}
        }
        let diag=&e["diagnostics"];
        if diag["warnings"].as_i64()!=Some(i64::from(state.tracking.total_warning_errors))
            || diag["severe"].as_i64()!=Some(i64::from(state.tracking.total_severe_errors))
            || diag["recurring"].as_i64()!=Some(0) || diag["text"].as_str()!=Some(std::str::from_utf8(bytes)?) {
            return Err("entire cold InitExt diagnostics differ".into());
        }
        if state.output_error_header!=bytes.is_empty(){return Err("InitExt error header latch differs".into());}
        Ok(())
    }
    pub(crate) fn native_gap_external_convection_cases(input:&Value,native:&Value)->Result<usize> {
        let cases=array(&input["exterior_coefficients"])?;let expected=array(&native["exterior_coefficients"])?;
        if cases.len()!=expected.len(){return Err("InitExt cases extent differs".into());}let mut count=0;
        for(c,e)in cases.iter().zip(expected){
            if c!=&e["input"]||c["name"]!=e["name"]{return Err("InitExt whole primary input differs".into());}
            let name=c["name"].as_str().ok_or("InitExt case name")?;
            let(model,algorithm)=match c["model"].as_str(){Some("DOE2")=>(7,Outside::Doe2),Some("ASHRAESimple")=>(3,Outside::SimpleCombined),_=>return Err("actual selected model".into())};
            let wind=c["wind_exposed"].as_bool().ok_or("wind input")?;
            let mut geometry=SurfaceGeometryInputState::native_gap_cold_external_row(name,wind,real(&c["gross_area"])?,real(&c["height"])?,real(&c["cos_tilt"])?,real(&c["azimuth"])?,real(&c["view_factor_sky"])?,real(&c["view_factor_ground"])?,model)?;
            let mut zones=SelectedZoneInputOwners::default();let mut zone=SelectedZoneData::default();zone.typed_zone_id=Some(ep_model::ZoneId(0));zone.outside_algorithm=algorithm;zones.zones=Some(vec![zone]);
            let mut module=Self::new(&geometry,&zones)?;
            let mut environment=WeatherEnvironmentState::default();
            let mut physical=vec![SourceSurfaceOutdoorState {surface_id:ep_model::SurfaceId(0),outdoor_environment:SurfaceOutdoorEnvironmentState::allocate(0.0,wind)}];
            let mut arrays=SourceSurfaceHeatBalanceArrays::default();
            // Output-only canaries, installed once; no exits are restored between calls.
            arrays.surf_h_conv_ext=Some(vec![f64::NAN]);arrays.surf_h_sky_ext=Some(vec![f64::NAN]);arrays.surf_h_grd_ext=Some(vec![f64::NAN]);
            arrays.surf_h_air_ext=Some(vec![f64::NAN]);arrays.surf_h_srd_surf_ext=Some(vec![f64::NAN]);arrays.surf_win_coeff_adj_ratio=Some(vec![f64::NAN]);
            let input_owner=SelectedInputProcessor::from_admitted_input(Vec::new(),std::collections::BTreeMap::new())?;let control=SelectedHeatBalanceInputState::default();let reports=ReportInputs::default();let mut scan=ReportScanState::default();
            let mut errors=cold_errors();let mut bytes=Vec::new();let mut console=Vec::new();
            let native_identity=|state:&Value|->Result<(u64,u64)> {
                Ok((state["state_address"].as_u64().filter(|v|*v>0).ok_or("actual Native nonnull state identity")?,
                    state["surface_address"].as_u64().filter(|v|*v>0).ok_or("actual Native nonnull Surface identity")?))
            };
            let initial_identity=native_identity(&e["initial"])?;
            let actual_source_surface=geometry.final_surfaces.as_ref().and_then(|r|r.first()).ok_or("actual Source Surface row")? as *const _;
            let actual_source_physical=physical.as_ptr();
            module.assert_native_gap_external(&geometry,&zones,&errors,&bytes,&e["initial"])?;
            let roughness=match c["roughness"].as_u64(){Some(0)=>Roughness::VeryRough,Some(1)=>Roughness::Rough,Some(2)=>Roughness::MediumRough,Some(3)=>Roughness::MediumSmooth,Some(4)=>Roughness::Smooth,Some(5)=>Roughness::VerySmooth,_=>return Err("roughness enum input".into())};
            let steps=array(&c["steps"])?;let calls=array(&e["calls"])?;if steps.len()!=calls.len(){return Err("InitExt repeated calls extent".into());}
            for(step,call)in steps.iter().zip(calls){
                if step!=&call["primary_step"]{return Err("InitExt step primary differs".into());}
                physical[0].outdoor_environment.row.dry_bulb_c=real(&step["dry_bulb"])?;
                physical[0].outdoor_environment.row.wind_speed_m_per_s=real(&step["wind_speed"])?;
                physical[0].outdoor_environment.row.wind_direction_deg=real(&step["wind_direction"])?;
                environment.current_weather.sky_temp_kelvin=real(&step["sky_temperature_kelvin"])?;
                geometry.parallel_arrays.as_mut().ok_or("actual convection metadata")?.surf_air_sky_rad_split[0]=real(&step["air_sky_split"])?;
                arrays.surf_win_coeff_adj_ratio.as_mut().ok_or("actual adjustment array")?[0]=real(&step["adjustment"])?;
                if native_identity(&call["before"])?!=initial_identity||native_identity(&call["after"])?!=initial_identity{return Err("Native retained InitExt state/Surface identity differs".into());}
                module.assert_native_gap_external(&geometry,&zones,&errors,&bytes,&call["before"])?;
                let mut backend=SourceErrorBackend {state:&mut errors,error_stream:Some(&mut bytes),console:&mut console,out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None};
                let mut owners=super::SourceExternalConvectionOwners {geometry:&mut geometry,zones:&zones,control:&control,arrays:&mut arrays,physical:&physical,environment:&environment,input:&input_owner,reports:&reports,scan:&mut scan,backend:&mut backend};
                module.initialize_external(super::SourceExternalConvectionCall {surface:1,h_movable_insulation:real(&step["HMovInsul"])?,roughness,absorptance:real(&step["absorptance"])?,outside_temperature:real(&step["surface_temperature"])?},&mut owners)?;
                module.assert_native_gap_external(&geometry,&zones,&errors,&bytes,&call["after"])?;
                if geometry.final_surfaces.as_ref().and_then(|r|r.first()).ok_or("same Source Surface row")? as *const _!=actual_source_surface
                    ||physical.as_ptr()!=actual_source_physical{return Err("Source retained Surface/physical owner identity differs".into());}
                for(value,key)in [(&arrays.surf_h_conv_ext,"HExt"),(&arrays.surf_h_sky_ext,"HSky"),(&arrays.surf_h_grd_ext,"HGround"),(&arrays.surf_h_air_ext,"HAir"),(&arrays.surf_h_srd_surf_ext,"HSrdSurf")]{bits(value.as_ref().ok_or("actual output storage")?[0],&call[key],key)?;}
                count+=1;
            }
        }Ok(count)
    }
}
