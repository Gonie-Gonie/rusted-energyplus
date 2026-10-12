// Genuine Rust production caller regressions, not Native numerical expectations.
#[test]
fn ordinary_a_records_warmup_and_every_run_step_reports_only_complete_first_day()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{CtfInitializationContext,CtfInitializationReports};
    use crate::heat_balance::error_backend::SourceErrorBackend;
    for samples in [1,24,48] {
        let mut typed=ctf_ordinary_output_test_model();
        typed.building=Some(surface_environment_test_building());
        typed.timestep.number_of_timesteps_per_hour=4;
        let model=SimulationModel::from_typed(typed);
        let weather=ProductionWeatherTimestepSeries::from_bytes(
            DECOY_BYTES.to_vec(),configuration(6,30,7,2,FirstHourInterpolationStartingValues::Hour24)?)?;
        let mut options=HeatBalanceSimulationOptions::hourly_samples(samples);
        options.warmup=HeatBalanceWarmupOptions {enabled:true,minimum_days:1,maximum_days:2,
            temperature_convergence_tolerance_delta_c:1000.0,loads_convergence_tolerance_w:1.0};
        let (mut reports,mut backend_state,mut eio,mut error,mut console)=(CtfInitializationReports::default(),ctf_ordinary_output_test_state(),Vec::new(),Vec::new(),Vec::new());
        let mut simulation={
            let mut context=CtfInitializationContext {reports:&mut reports,eio:&mut eio,
                backend:SourceErrorBackend {state:&mut backend_state,error_stream:Some(&mut error),console:&mut console,
                out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None}};
            crate::simulate_heat_balance_zone_air_temperatures_with_production_weather_and_warmup_reporting(
                &model,&weather,options,&mut context,crate::HeatBalanceOrdinaryWarmupReportInputs {detailed:true,advanced_report_variables:false,radiation_input:None,do_pure_load_calc:false,do_space_heat_balance_for_sizing:false,do_space_heat_balance_for_simulation:false,
                    no_user_convection_input:crate::heat_balance::inside_convection_dispatcher::NoUserConvectionInput::new([0;6])?,
                    low_hconv_limit_w_per_m2_k:0.375,
                    equipment_input:Some(crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::from_family_inventory([],4)?)})?
        };
        let equipment=simulation.state.ordinary_equipment.as_mut().ok_or("missing actual geometry equipment owner")?;
        let view=equipment.config_view()?;
        use crate::heat_balance::inside_convection_dispatcher::ZoneEquipmentActiveElements;
        assert_eq!(view.zone_id(0),model.typed.zones[0].id);
        assert_eq!(view.active_elements(0),[false;3]);
        for surface in &simulation.state.surfaces {
            assert!(surface.outdoor_environment.manager_invoked);
            assert_eq!(surface.outdoor_environment.row.centroid_height_m.to_bits(),surface.geometry.centroid_m.z_m.to_bits());
        }
        let temperatures=simulation.state.ordinary_inside_temperatures.as_ref().ok_or("missing ordinary temperature owner")?;
        assert_eq!(temperatures.inside_old.len(),simulation.state.surfaces.len());
        assert_eq!(temperatures.inside_tmp.len(),simulation.state.surfaces.len());
        for (&(id, tmp), surface) in temperatures.inside_tmp.iter().zip(&simulation.state.surfaces) {
            assert_eq!(id,surface.surface_id);
            assert_eq!(tmp.to_bits(),surface.inside_face_temperature_c.to_bits());
        }
        assert_eq!(temperatures.no_user_convection_input.counts(),[0;6]);
        assert_eq!(temperatures.low_hconv_limit_w_per_m2_k.to_bits(),0.375_f64.to_bits());
        for space in temperatures.space_air.rows() {
            assert_eq!(space.mat.to_bits(),simulation.state.zones[0].mean_air_temperature_c.to_bits());
            assert_eq!(space.zt.to_bits(),space.mat.to_bits());
            assert_eq!(space.ztav.to_bits(),23.0_f64.to_bits());
        }
        let owner=simulation.state.warmup_convergence.as_ref().ok_or("missing actual ordinary owner")?;
        let report=owner.report.as_ref().ok_or("missing actual ordinary report allocation")?;
        assert!(!owner.warmup_flag && !backend_state.warmup_flag);
        assert!(owner.checked_day>=1);
        assert_eq!(report.count,samples.min(24)*4);
        assert_eq!(owner.zones[0].sensible_report_timestep,simulation.state.timestep_index.checked_sub(1));
        assert_eq!(owner.zones[0].rolling[0].to_bits(),simulation.state.zones[0].zone_timestep_average_air_temperature_c.to_bits());
        assert_eq!(report.temperature_stddev.iter().all(Option::is_some),samples>=24);
        assert_eq!(report.first_summary_write,samples<24);
        let text=std::str::from_utf8(&eio)?;
        assert_eq!(text.matches(crate::heat_balance::warmup::report::DETAILED_HEADER).count(),1);
        assert_eq!(text.matches(crate::heat_balance::warmup::report::SUMMARY_HEADER).count(),usize::from(samples>=24));
        assert_selected_temperature_resets_and_prefix(&model.typed, &mut simulation.state)?;
    }
    Ok(())
}


// Explicit authored test inputs; production still requires its compiled owner.
fn surface_environment_test_building() -> ep_model::Building {
    ep_model::Building { name: NormalizedName::new("authored surface weather test"),
        north_axis_deg:0.0, terrain:Terrain::Suburbs,
        loads_convergence_tolerance_w:1.0, temperature_convergence_tolerance_delta_c:1000.0,
        solar_distribution:ep_model::SolarDistribution::FullExterior,
        maximum_number_of_warmup_days:2, minimum_number_of_warmup_days:1 }
}

#[test]
fn ordinary_surface_cold_stop_retains_all_bulbs_and_incoming_wind_arrays()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{CtfInitializationContext,CtfInitializationReports};
    use crate::heat_balance::convection::surface_environment::{ordinary,SurfaceOutdoorEnvironmentError};
    use crate::heat_balance::error_backend::{SourceErrorBackend,SourceFatalStop};
    let mut typed=ctf_ordinary_output_test_model();
    typed.building=Some(surface_environment_test_building());
    typed.timestep.number_of_timesteps_per_hour=4;
    let model=SimulationModel::from_typed(typed);
    let weather=ProductionWeatherTimestepSeries::from_bytes(DECOY_BYTES.to_vec(),
        configuration(6,30,7,2,FirstHourInterpolationStartingValues::Hour24)?)?;
    let driver=HeatBalanceWeatherDriver::Production(&weather);
    driver.prepare_initial_phase(crate::weather::day::WeatherDayPhase::Warmup{day:1})?;
    let actual=driver.current_context(0,4,1,FirstHourInterpolationStartingValues::Hour24)?;
    let (mut reports,mut backend_state,mut eio,mut error,mut console)=(CtfInitializationReports::default(),ctf_ordinary_output_test_state(),Vec::new(),Vec::new(),Vec::new());
    let mut context=CtfInitializationContext {reports:&mut reports,eio:&mut eio,
        backend:SourceErrorBackend {state:&mut backend_state,error_stream:Some(&mut error),console:&mut console,
            out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None}};
    let mut state=crate::heat_balance::initialize_heat_balance_state_with_initialization_context(&model,23.0,&mut context)?;
    let timestep=state.timestep_index;
    // Deliberate pre-call writes to actual retained owners, not Native results
    // or altered weather. This invokes the real manager/guard on its state.
    let first=state.surfaces.first_mut().ok_or("missing first actual surface")?;
    first.geometry.centroid_m.z_m=30000.0;
    first.outdoor_environment.row.centroid_height_m=30000.0;
    for (index,surface) in state.surfaces.iter_mut().enumerate() {
        surface.outdoor_environment.row.wind_speed_m_per_s=f64::from_bits(0x7ff8000000000300+index as u64);
        surface.outdoor_environment.row.wind_direction_deg= -0.0;
    }
    let incoming=state.surfaces.iter().map(|surface|
        (surface.outdoor_environment.row.wind_speed_m_per_s.to_bits(),
         surface.outdoor_environment.row.wind_direction_deg.to_bits())).collect::<Vec<_>>();
    let result=ordinary::invoke(&model,&mut state,Some(actual),Some(&mut context));
    assert!(matches!(result,Err(RuntimeError::SurfaceOutdoorEnvironment(error))
        if matches!(*error,SurfaceOutdoorEnvironmentError::Fatal(SourceFatalStop::Fatal(_)))));
    assert!(state.surfaces[0].outdoor_environment.row.dry_bulb_c < -100.0);
    for (surface,(wind,direction)) in state.surfaces.iter().zip(incoming) {
        assert!(surface.outdoor_environment.manager_invoked);
        assert_ne!(surface.outdoor_environment.row.wet_bulb_c.to_bits(),0.0_f64.to_bits());
        assert_eq!(surface.outdoor_environment.row.wind_speed_m_per_s.to_bits(),wind);
        assert_eq!(surface.outdoor_environment.row.wind_direction_deg.to_bits(),direction);
    }
    assert_eq!(state.timestep_index,timestep);
    assert_eq!(context.backend.state.tracking.total_severe_errors,1);
    assert_eq!(context.backend.state.tracking.total_severe_errors_during_warmup,1);
    Ok(())
}

// These authored owner checks run only after the actual A production route above.
// They retain distinct current ZT, MAT, ZTAV and Old inputs, including signed zero.
fn assert_selected_temperature_resets_and_prefix(
    model: &TypedModel,
    state: &mut crate::heat_balance::state::HeatBalanceState,
) -> Result<(), Box<dyn std::error::Error>> {
    let owner = state.ordinary_inside_temperatures.as_mut().ok_or("missing owner")?;
    let original_low_limit = owner.low_hconv_limit_w_per_m2_k;
    let input_prefix = owner.clone();
    let surface_prefix = state.surfaces.iter().map(|surface|
        (surface.inside_face_temperature_c.to_bits(), surface.inside_reference_air_temperature_c.to_bits()))
        .collect::<Vec<_>>();
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        owner.low_hconv_limit_w_per_m2_k = invalid;
        assert!(owner.prepare_inside_entry(model, &state.zones, &mut state.surfaces).is_err());
        assert_eq!(state.surfaces.iter().map(|surface|
            (surface.inside_face_temperature_c.to_bits(), surface.inside_reference_air_temperature_c.to_bits()))
            .collect::<Vec<_>>(), surface_prefix);
    }
    owner.low_hconv_limit_w_per_m2_k = original_low_limit;
    assert_eq!(*owner, input_prefix);
    let original_id = state.surfaces.first().ok_or("missing actual surface")?.surface_id;
    let stale_id = ep_model::SurfaceId(original_id.0.wrapping_add(1));
    assert!(owner.inside_tmp_for_pass(&state.surfaces, &[]).is_err());
    assert_eq!(*owner,input_prefix);
    owner.inside_tmp[0].0 = stale_id;
    let invalid_tmp_prefix = owner.clone();
    let zone_prefix = state.zones.iter().map(|zone|
        (zone.mean_air_temperature_c.to_bits(), zone.zone_timestep_average_air_temperature_c.to_bits()))
        .collect::<Vec<_>>();
    assert!(owner.prepare_inside_entry(model, &state.zones, &mut state.surfaces).is_err());
    assert!(owner.inside_tmp_for_pass(&state.surfaces, state.surface_indexes.ctf_surface_indices()).is_err());
    assert!(owner.begin_environment(&mut state.zones).is_err());
    assert_eq!(*owner,invalid_tmp_prefix);
    assert_eq!(state.zones.iter().map(|zone|
        (zone.mean_air_temperature_c.to_bits(), zone.zone_timestep_average_air_temperature_c.to_bits()))
        .collect::<Vec<_>>(),zone_prefix);
    assert_eq!(state.surfaces.iter().map(|surface|
        (surface.inside_face_temperature_c.to_bits(),surface.inside_reference_air_temperature_c.to_bits()))
        .collect::<Vec<_>>(),surface_prefix);
    owner.inside_tmp[0].0 = original_id;
    state.surfaces[0].surface_id = stale_id;
    owner.inside_old[0].0 = stale_id;
    let old_before_validation = owner.inside_old.iter()
        .map(|&(id, value)| (id, value.to_bits())).collect::<Vec<_>>();
    let surfaces_before_validation = state.surfaces.iter().map(|surface|
        (surface.surface_id, surface.inside_face_temperature_c.to_bits(),
         surface.inside_reference_air_temperature_c.to_bits())).collect::<Vec<_>>();
    assert!(owner.prepare_inside_entry(model, &state.zones, &mut state.surfaces).is_err());
    assert_eq!(owner.inside_old.iter().map(|&(id, value)| (id, value.to_bits()))
        .collect::<Vec<_>>(), old_before_validation);
    assert_eq!(state.surfaces.iter().map(|surface|
        (surface.surface_id, surface.inside_face_temperature_c.to_bits(),
         surface.inside_reference_air_temperature_c.to_bits())).collect::<Vec<_>>(), surfaces_before_validation);
    state.surfaces[0].surface_id = original_id;
    owner.inside_old[0].0 = original_id;
    let last_surface = state.surfaces.pop().ok_or("missing surface prefix")?;
    let last_old = owner.inside_old.pop().ok_or("missing Old prefix")?;
    let last_tmp = owner.inside_tmp.pop().ok_or("missing Tmp prefix")?;
    let tmp_prefix = owner.inside_tmp.iter().map(|&(id,value)| (id,value.to_bits())).collect::<Vec<_>>();
    let old_prefix = owner.inside_old.iter().map(|&(id, value)| (id, value.to_bits())).collect::<Vec<_>>();
    let ref_air_prefix = state.surfaces.iter().map(|surface|
        surface.inside_reference_air_temperature_c.to_bits()).collect::<Vec<_>>();
    assert!(owner.prepare_inside_entry(model, &state.zones, &mut state.surfaces).is_err());
    assert_eq!(owner.inside_old.iter().map(|&(id, value)| (id, value.to_bits()))
        .collect::<Vec<_>>(), old_prefix);
    assert_eq!(state.surfaces.iter().map(|surface| surface.inside_reference_air_temperature_c.to_bits())
        .collect::<Vec<_>>(), ref_air_prefix);
    state.surfaces.push(last_surface);
    assert_eq!(owner.inside_tmp.iter().map(|&(id,value)| (id,value.to_bits())).collect::<Vec<_>>(),tmp_prefix);
    owner.inside_old.push(last_old);
    owner.inside_tmp.push(last_tmp);
    owner.inside_tmp[0].1 = -0.0;
    let zone = state.zones.first_mut().ok_or("missing actual zone")?;
    let surface = state.surfaces.first_mut().ok_or("missing actual surface")?;
    let id = surface.surface_id;
    let old_before = owner.inside_old.clone();
    surface.surface_id = ep_model::SurfaceId(id.0.wrapping_add(1));
    assert!(owner.capture_inside_entry(&state.surfaces).is_err());
    assert_eq!(owner.inside_old, old_before);
    state.surfaces[0].surface_id = id;
    state.surfaces[0].inside_face_temperature_c = -0.0;
    owner.capture_inside_entry(&state.surfaces).map_err(|_| "inside capture unavailable")?;
    assert_eq!(owner.inside_old[0].1.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(owner.inside_tmp[0].1.to_bits(),(-0.0_f64).to_bits());
    state.surfaces[0].inside_face_temperature_c = 39.0;
    assert_eq!(owner.inside_old[0].1.to_bits(), (-0.0_f64).to_bits());

    let zone_id = zone.zone_id;
    zone.mean_air_temperature_c = 41.0;
    zone.zone_timestep_average_air_temperature_c = 15.0;
    owner.retain_solution(zone_id, -0.0).map_err(|_| "solution unavailable")?;
    let spaces_before_commit = owner.space_air.clone();
    owner.commit_zone_mat(&mut state.zones).map_err(|_| "MAT commit unavailable")?;
    assert_eq!(owner.space_air, spaces_before_commit);
    assert_eq!(state.zones[0].mean_air_temperature_c.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(state.zones[0].zone_timestep_average_air_temperature_c, 15.0);
    owner.after_full_correction(&state.zones).map_err(|_| "correction unavailable")?;
    assert_eq!(state.zones[0].mean_air_temperature_c.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(state.zones[0].zone_timestep_average_air_temperature_c, 15.0);
    for space in owner.space_air.rows() {
        assert_eq!(space.mat.to_bits(), (-0.0_f64).to_bits());
        assert_eq!(space.zt.to_bits(), (-0.0_f64).to_bits());
        assert_eq!(space.ztav.to_bits(), 23.0_f64.to_bits());
    }
    let old_id = state.zones[0].zone_id;
    state.zones[0].zone_id = ep_model::ZoneId(old_id.0.wrapping_add(1));
    assert!(owner.begin_environment(&mut state.zones).is_err());
    assert_eq!(state.zones[0].mean_air_temperature_c.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(state.zones[0].zone_timestep_average_air_temperature_c, 15.0);
    state.zones[0].zone_id = old_id;
    owner.begin_environment(&mut state.zones).map_err(|_| "reset unavailable")?;
    assert_eq!(state.zones[0].mean_air_temperature_c, 23.0);
    assert_eq!(state.zones[0].zone_timestep_average_air_temperature_c, 23.0);
    assert_eq!(owner.inside_old[0].1.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(state.surfaces[0].inside_face_temperature_c, 39.0);
    for &(_,tmp) in &owner.inside_tmp {
        assert_eq!(tmp.to_bits(),23.0_f64.to_bits());
    }
    for space in owner.space_air.rows() {
        assert_eq!((space.mat, space.zt, space.ztav), (23.0, 23.0, 23.0));
    }
    state.zones[0].mean_air_temperature_c = 25.0;
    owner.commit_zone_mat(&mut state.zones).map_err(|_| "reset ZT unavailable")?;
    owner.after_full_correction(&state.zones).map_err(|_| "reset space copy unavailable")?;
    assert_eq!(state.zones[0].mean_air_temperature_c, 23.0);
    Ok(())
}
