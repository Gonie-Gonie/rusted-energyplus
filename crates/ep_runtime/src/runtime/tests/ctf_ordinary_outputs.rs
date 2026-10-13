
fn ctf_ordinary_output_test_state() -> crate::heat_balance::error_backend::SourceErrorState {
    use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
    SourceErrorState { output_error_header: true, version_string: "authored Rust caller".into(),
        idd_version_string: "authored typed input".into(), doing_input_processing: false,
        print_console_output: true, kickoff_simulation: false, developer_flag: false,
        warmup_flag: false, doing_sizing: false, tracking: SourceErrorTracking {
            match_counts:[0;20],abort_processing:false,total_severe_errors:0,last_severe_error:String::new(),
            total_severe_errors_during_warmup:0,total_severe_errors_during_sizing:0,
            total_warning_errors:0,total_warning_errors_during_warmup:0,total_warning_errors_during_sizing:0,
        } }
}

fn ctf_ordinary_output_test_model() -> TypedModel {
    let mut model=cube_model();
    model.ctf_report_names.materials=vec![(MaterialId(0),"R1".into())];
    model.ctf_report_names.constructions=vec![(ConstructionId(0),"Wall".into())];
    model
}

#[test]
fn ordinary_cache_uses_real_registry_columns_and_postlude_before_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{CtfInitializationContext,CtfInitializationReports};
    use crate::heat_balance::error_backend::SourceErrorBackend;
    let model=ctf_ordinary_output_test_model();
    let mut reports=CtfInitializationReports::default();
    let mut state=ctf_ordinary_output_test_state();
    let (mut error,mut console,mut eio)=(Vec::new(),Vec::new(),Vec::new());
    let mut context=CtfInitializationContext { reports:&mut reports,eio:&mut eio,
        backend:SourceErrorBackend {state:&mut state,error_stream:Some(&mut error),console:&mut console,
            out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None} };
    context.prepare_registry(&model)?;
    let mut cache=crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
        &model,&std::collections::BTreeMap::new(),&mut context)?;
    assert!(cache.take_public_surface_lifecycle().transpose().map_err(crate::RuntimeError::CtfSurfaceLifecycle)?.is_some());
    assert!(reports.progress.postlude_returned);
    assert_eq!(reports.progress.layer_report_declared_indices,[0]);
    assert_eq!(reports.progress.construction_scan_result,Some(false));
    let column=reports.registry.opaque_layer_columns()[0];
    let entries=reports.table.entries.as_ref().ok_or_else(|| std::io::Error::other("missing real table owner"))?;
    assert_eq!(reports.table.num_table_entry,1);
    assert_eq!(entries[0].index_column,column);
    assert_eq!(entries[0].object_name,"Wall");
    assert_eq!(entries[0].char_entry,"R1");
    assert!(error.is_empty());assert!(eio.is_empty());
    // Authored typed input: exercise the real optional Solar initializer,
    // not an observed Native owner endpoint or another weather simulation.
    for (surface_count, success) in [(1_usize, true), (4, false)] {
        let mut typed = ctf_ordinary_output_test_model();
        typed.zones[0].ceiling_height = AutoOrNumber::Value(1.0);
        typed.zones[0].floor_area = AutoOrNumber::Value(1.0);
        let wall = typed.surfaces.iter().find(|surface| surface.surface_type == SurfaceType::Wall)
            .cloned().ok_or("missing authored Wall fixture")?;
        typed.surfaces = (0..surface_count).map(|index| {
            let mut surface = wall.clone();
            surface.id = SurfaceId(u32::try_from(index)?);
            surface.name = NormalizedName::new(&format!("authored parallel wall {index}"));
            Ok::<_, Box<dyn std::error::Error>>(surface)
        }).collect::<Result<Vec<_>, _>>()?;
        let mut inside = typed.materials[0].clone();
        inside.id = MaterialId(1);
        inside.name = NormalizedName::new("authored inside layer");
        let ep_model::MaterialDefinition::NoMass(material) = &mut inside.definition else {
            return Err("expected authored opaque inside material".into());
        };
        material.surface.solar_absorptance = 0.25;
        typed.materials.push(inside);
        typed.ctf_report_names.materials.push((MaterialId(1), "authored inside layer".into()));
        typed.constructions[0].layers.push(MaterialId(1));
        typed.other_equipment[0].fraction_radiant = 0.25;
        let model = SimulationModel::from_typed(typed);
        let (schedule_cache, mut profile) =
            crate::schedules::precompute_hour_only_internal_gain_schedule_cache_profiled(&model.typed)?;
        assert_eq!(profile.cache_entry_count, 1);
        let (mut reports, mut state, mut error, mut console, mut eio) =
            (CtfInitializationReports::default(), ctf_ordinary_output_test_state(),
             Vec::new(), Vec::new(), Vec::new());
        let result = {
            let mut context = CtfInitializationContext { reports: &mut reports, eio: &mut eio,
                backend: SourceErrorBackend { state: &mut state, error_stream: Some(&mut error),
                    console: &mut console, out_unit1: None, out_unit2: None, f_message: None,
                    message_callback: None, error_callback: None, sqlite: None } };
            crate::heat_balance::initialization::initialize_heat_balance_state_with_initialization_context_and_equipment_profiled(
                &model, 23.0, &[], &schedule_cache, &mut profile, &mut context,
                crate::heat_balance::initialization::OrdinaryInitializationInputs {
                    equipment: None,
                    source: None,
                    radiation: Some(crate::heat_balance::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory([], true)?),
                },
            )
        };
        assert!(reports.progress.postlude_returned);
        let owner = if success {
            let mut output = result?;
            assert!(reports.ordinary_solar_radiation.is_none(), "successful owner must move into State");
            let surface = &output.surfaces[0];
            assert_eq!(surface.inside_solar_absorptance.to_bits(), 0.25_f64.to_bits());
            assert_eq!(surface.solar_absorptance.to_bits(), 0.75_f64.to_bits());
            assert_eq!(output.zones[0].convective_internal_gain_w.to_bits(), 9.0_f64.to_bits());
            assert!((surface.inside_radiant_internal_gain_w_per_m2 - 3.0).abs() < 1.0e-12);
            assert_eq!(profile.initialization.cached_value_lookup_count, 2,
                "both actual gain consumers run after successful InitSolar");
            // Authored Rust failure-custody payload, not Native getter endpoints.
            // Preserve the actual initialized Surface/Solar allocations by move.
            use crate::heat_balance::selected_heat_balance_input::zone::{SelectedZoneData, SelectedZoneInputOwners};
            output.selected_zone_input = Some(SelectedZoneInputOwners {
                zones: Some(vec![SelectedZoneData {
                    typed_zone_id: Some(model.typed.zones[0].id),
                    name: model.typed.zones[0].name.0.clone(),
                    dry_bulb: -101.0, wet_bulb: -99.0, wind_speed: -0.0,
                    ..Default::default()
                }]), ..Default::default()
            });
            output.selected_heat_balance_globals = Some(Default::default());
            let surface_allocation = output.surfaces.as_ptr();
            let zone_allocation = output.selected_zone_input.as_ref().and_then(|owner| owner.zones.as_ref())
                .ok_or("authored custody Zone allocation")?.as_ptr();
            assert!(reports.failed_heat_balance_state.is_none());
            {
                let mut context = CtfInitializationContext { reports: &mut reports, eio: &mut eio,
                    backend: SourceErrorBackend { state: &mut state, error_stream: Some(&mut error),
                        console: &mut console, out_unit1: None, out_unit2: None, f_message: None,
                        message_callback: None, error_callback: None, sqlite: None } };
                crate::heat_balance::convection::surface_environment::ordinary::retain_failed_state(
                    output, Some(&mut context),
                );
            }
            assert!(reports.selected_zone_input.is_none());
            assert!(reports.selected_heat_balance_globals.is_none());
            let mut failed = reports.failed_heat_balance_state.take().ok_or("missing moved failed State")?;
            assert_eq!(failed.surfaces.as_ptr(), surface_allocation);
            let rows = failed.selected_zone_input.as_ref().and_then(|owner| owner.zones.as_ref())
                .ok_or("failed State lost Zone allocation")?;
            assert_eq!(rows.as_ptr(), zone_allocation);
            assert_eq!([rows[0].dry_bulb, rows[0].wet_bulb], [-101.0, -99.0]);
            assert_eq!(rows[0].wind_speed.to_bits(), (-0.0_f64).to_bits());
            assert!(failed.selected_heat_balance_globals.is_some());
            failed.ordinary_solar_radiation.take().ok_or("failed State lost moved actual Solar owner")?
        } else {
            use crate::heat_balance::radiation::ordinary::{OrdinaryRadiationError, OrdinaryRadiationUnavailable};
            assert!(matches!(result, Err(RuntimeError::OrdinaryRadiation(error))
                if matches!(*error, OrdinaryRadiationError::Unavailable(OrdinaryRadiationUnavailable::NumericDomain))));
            assert_eq!(profile.initialization.cached_value_lookup_count, 0,
                "neither convective nor radiant gain consumer may precede the failed Solar prefix");
            reports.ordinary_solar_radiation.take().ok_or("missing retained partial Solar owner")?
        };
        assert_eq!(owner.is_initialized(), success);
        assert_eq!(owner.enclosures().len(), 1);
        let enclosure = &owner.enclosures()[0];
        assert_eq!(enclosure.id, 1);
        assert_eq!(enclosure.zone_id, model.typed.zones[0].id);
        assert_eq!(enclosure.space_ids, [model.typed.spaces[0].id]);
        assert_eq!(enclosure.surface_ptr,
            model.typed.surfaces.iter().map(|surface| surface.id).collect::<Vec<_>>());
        assert_eq!(enclosure.surface_indexes, (0..surface_count).collect::<Vec<_>>());
        assert_eq!(enclosure.surface_report_nums, (0..surface_count).collect::<Vec<_>>());
        assert_eq!(owner.back_pointers(), (0..surface_count).map(|index| (1, 1, index + 1)).collect::<Vec<_>>());
        assert_eq!(enclosure.area, vec![1.0; surface_count]);
        assert_eq!(enclosure.absorptance, vec![0.25; surface_count]);
        assert_eq!(enclosure.azimuth.len(), surface_count);
        assert_eq!(enclosure.tilt.len(), surface_count);
        assert!(enclosure.azimuth.iter().all(|value| value.to_bits() == enclosure.azimuth[0].to_bits()));
        assert!(enclosure.tilt.iter().all(|value| value.to_bits() == enclosure.tilt[0].to_bits()));
        assert_eq!(enclosure.f.len(), surface_count * surface_count);
        assert!(enclosure.f.iter().all(|value| value.to_bits() == 0.0_f64.to_bits()),
            "N1 skips numeric work; buffered-warning rejection retains allocated +0 F");
        let text = std::str::from_utf8(&eio)?;
        assert_eq!(text.matches("! <Solar View Factor Check Values>").count(), 1);
        assert_eq!(text.lines().filter(|line| line.starts_with("Solar View Factor Check Values,")).count(), usize::from(success));
        if success {
            assert!(text.contains(&format!("Solar View Factor Check Values,{},0,0,0,-1,0,0", model.typed.spaces[0].name.0)));
        }
        assert_eq!(state.tracking.total_warning_errors, 0,
            "buffered approximate warnings are unavailable, not emitted as Original mid-loop effects");
        assert_eq!(state.tracking.total_severe_errors, 0);
        assert!(error.is_empty());
    }
    Ok(())
}

#[test]
fn ordinary_loading_error_retains_layers_and_real_final_fatal_prefix() -> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{CtfInitializationContext,CtfInitializationReports};
    use crate::heat_balance::error_backend::SourceErrorBackend;
    let mut model=ctf_ordinary_output_test_model();
    if let ep_model::MaterialDefinition::NoMass(material)=&mut model.materials[0].definition {
        material.thermal_resistance_m2_k_per_w=1.0e-9;
    }
    let mut reports=CtfInitializationReports::default();let mut state=ctf_ordinary_output_test_state();
    let (mut error,mut console,mut eio)=(Vec::new(),Vec::new(),Vec::new());
    let result={
        let mut context=CtfInitializationContext {reports:&mut reports,eio:&mut eio,
            backend:SourceErrorBackend {state:&mut state,error_stream:Some(&mut error),console:&mut console,
                out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None}};
        context.prepare_registry(&model)?;
        crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
            &model,&std::collections::BTreeMap::new(),&mut context)
    };
    assert!(matches!(result,Err(crate::RuntimeError::CtfInitialization(_))));
    assert_eq!(state.tracking.total_severe_errors,1);
    assert!(!state.tracking.abort_processing);
    assert_eq!(reports.progress.layer_report_declared_indices,[0]);
    assert!(reports.progress.final_fatal_invoked);
    assert!(!reports.progress.postlude_returned);
    let text=String::from_utf8(error)?;
    assert!(text.contains("Material=R1R Value below lowest allowed value"));
    assert!(text.contains("Program terminated for reasons listed (InitConductionTransferFunctions)"));
    Ok(())
}

#[test]
fn ordinary_missing_reached_name_preserves_earlier_thick_material_severe() -> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{CtfInitializationContext,CtfInitializationReports};
    use crate::heat_balance::error_backend::SourceErrorBackend;
    let mut model=ctf_ordinary_output_test_model();
    model.ctf_report_names.materials.clear();
    model.materials[0].definition=ep_model::MaterialDefinition::Regular(ep_model::RegularMaterial {
        roughness:MaterialSurfaceRoughness::Rough,thickness_m:4.0,conductivity_w_per_m_k:1.0,
        density_kg_per_m3:1000.0,specific_heat_j_per_kg_k:1000.0,thermal_resistance_m2_k_per_w:4.0,
        surface:ep_model::OpaqueSurfaceProperties::default(),
    });
    let mut reports=CtfInitializationReports::default();let mut state=ctf_ordinary_output_test_state();
    let (mut error,mut console,mut eio)=(Vec::new(),Vec::new(),Vec::new());
    let result={
        let mut context=CtfInitializationContext {reports:&mut reports,eio:&mut eio,
            backend:SourceErrorBackend {state:&mut state,error_stream:Some(&mut error),console:&mut console,
                out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None}};
        context.prepare_registry(&model)?;
        crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
            &model,&std::collections::BTreeMap::new(),&mut context)
    };
    assert!(matches!(result,Err(crate::RuntimeError::CtfInitialization(_))));
    assert_eq!(state.tracking.total_severe_errors,1);
    assert!(!state.tracking.abort_processing);
    assert!(!reports.progress.final_fatal_invoked);
    assert!(String::from_utf8(error)?.contains("Material too thick for CTF calculation"));
    Ok(())
}

struct OrdinaryFailingEio;
impl std::io::Write for OrdinaryFailingEio {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::from_raw_os_error(5))
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

#[test]
fn ordinary_eio_failure_remains_discoverable_through_runtime_source_chain() -> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{CtfInitializationContext, CtfInitializationError, CtfInitializationReports};
    use crate::heat_balance::ctf_init_postlude::CtfInitPostludeError;
    use crate::heat_balance::ctf_reports::CtfReportError;
    use crate::heat_balance::error_backend::SourceErrorBackend;
    use std::error::Error;
    let mut model = ctf_ordinary_output_test_model();
    model.report_inputs.objects.push(ep_model::ReportInputObject {
        object_name: "authored direct report request".into(),
        object_type: ep_model::ReportInputObjectType::Constructions,
        alphas: ["Constructions".into(), String::new(), String::new()],
        alpha_blank: [false, true, true], default_applied: [false; 3],
        num_alphas: 1, is_epjson: true,
    });
    let mut reports = CtfInitializationReports::default();
    let mut state = ctf_ordinary_output_test_state();
    let (mut error, mut console) = (Vec::new(), Vec::new());
    let mut eio = OrdinaryFailingEio;
    let mut context = CtfInitializationContext { reports: &mut reports, eio: &mut eio,
        backend: SourceErrorBackend { state: &mut state, error_stream: Some(&mut error), console: &mut console,
            out_unit1: None, out_unit2: None, f_message: None, message_callback: None, error_callback: None, sqlite: None } };
    context.prepare_registry(&model)?;
    let result = crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
        &model, &std::collections::BTreeMap::new(), &mut context);
    let outer = match result { Err(error) => error, Ok(_) => return Err("EIO write unexpectedly succeeded".into()) };
    let stored = match &outer {
        crate::RuntimeError::CtfInitialization(error) => match error.as_ref() {
            CtfInitializationError::Postlude(CtfInitPostludeError::Report(CtfReportError::Io(error))) => error,
            _ => return Err("wrong initialization failure".into()),
        },
        _ => return Err("wrong runtime failure".into()),
    };
    let discovered = outer.source().and_then(Error::source).and_then(Error::source)
        .and_then(Error::source).and_then(|error| error.downcast_ref::<std::io::Error>())
        .ok_or("actual EIO cause missing from RuntimeError source chain")?;
    assert!(std::ptr::eq(stored, discovered));
    assert_eq!(discovered.raw_os_error(), Some(5));
    assert_eq!(reports.progress.construction_scan_result, Some(true));
    assert_eq!(reports.progress.layer_report_declared_indices, [0]);
    assert!(!reports.progress.report_headers_written && !reports.progress.final_fatal_invoked);
    assert!(!reports.progress.postlude_returned && error.is_empty());
    Ok(())
}
