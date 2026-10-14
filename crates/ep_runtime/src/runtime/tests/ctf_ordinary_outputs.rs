
include!("ctf_ordinary_process_reference.rs");

fn ctf_ordinary_output_test_state() -> crate::heat_balance::error_backend::SourceErrorState {
    use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
    SourceErrorState { output_error_header: true, version_string: "authored Rust caller".into(),
        idd_version_string: "authored typed input".into(), doing_input_processing: false,
        print_console_output: true, kickoff_simulation: false, developer_flag: false,
        warmup_flag: false, doing_sizing: false, tracking: SourceErrorTracking { ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
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

    // Authored Rust custody fixture: exercise real final-copy allocation and
    // the same move/reverse APIs used by the selected constructor error path.
    // This does not observe Native arrays or certify a full getter return.
    {
        use crate::heat_balance::selected_heat_balance_input::surface::{
            SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners, SurfaceTmpInput,
        };
        use crate::heat_balance::selected_heat_balance_input::{
            SelectedHeatBalanceInputState, zone::{SelectedZoneData, SelectedZoneInputOwners},
        };
        use crate::heat_balance::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
        let model = SimulationModel::from_typed(ctf_ordinary_output_test_model());
        let total = model.typed.surfaces.len();
        assert_eq!(total, 6);
        let (schedule_cache, mut profile) =
            crate::schedules::precompute_hour_only_internal_gain_schedule_cache_profiled(&model.typed)?;
        let (mut reports, mut state, mut error, mut console, mut eio) =
            (CtfInitializationReports::default(), ctf_ordinary_output_test_state(), Vec::new(), Vec::new(), Vec::new());
        let mut context = CtfInitializationContext { reports: &mut reports, eio: &mut eio,
            backend: SourceErrorBackend { state: &mut state, error_stream: Some(&mut error),
                console: &mut console, out_unit1: None, out_unit2: None, f_message: None,
                message_callback: None, error_callback: None, sqlite: None } };
        let mut physical = crate::heat_balance::initialization::initialize_heat_balance_state_with_initialization_context_and_equipment_profiled(
            &model, 23.0, &[], &schedule_cache, &mut profile, &mut context, Default::default(),
        )?;
        let mut raw = SurfaceGeometryInputState {
            total_surfaces: i32::try_from(total)?, surfaces_read: total, next_object: total,
            surface_tmp: Some(model.typed.surfaces.iter().map(|surface| SurfaceTmpInput {
                name: surface.name.0.clone(), class: match surface.surface_type {
                    SurfaceType::Wall => SurfaceClass::Wall, SurfaceType::Floor => SurfaceClass::Floor,
                    SurfaceType::Roof | SurfaceType::Ceiling => SurfaceClass::Roof,
                }, construction: 1, zone: 1, space: 1, heat_transfer_surface: true,
                exterior_solar: true, exterior_wind: true, sides: 4,
                vertex: Some(surface.vertices.iter().map(|p| Some([p.x_m, p.y_m, p.z_m])).collect()),
                ..Default::default()
            }).collect()), ..Default::default()
        };
        let mut zones = SelectedZoneInputOwners {
            zones: Some(vec![SelectedZoneData { typed_zone_id: Some(model.typed.zones[0].id),
                space_indexes: vec![1], num_spaces: 1, ..Default::default() }]),
            spaces: Some(vec![Default::default()]), ..Default::default()
        };
        let space = &mut zones.spaces.as_mut().ok_or("authored actual Space allocation")?[0];
        space.typed_space_id = Some(model.typed.spaces[0].id);
        space.zone_num = 1;
        let mut definitions = vec![
            crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: "SurfaceProperty:ExteriorNaturalVentedCavity".into(),
                alpha_fields: 3, numeric_fields: 8, alpha_extensions: 1,
                numeric_extensions: 0, extension: Some("surface".into()),
            },
        ];
        for (name, a, n, ae, ne, extension) in [
            ("NodeList", 1, 0, 1, 0, Some("nodes")),
            ("ZoneHVAC:EquipmentList", 2, 0, 4, 2, Some("equipment")),
            ("ZoneHVAC:EquipmentConnections", 8, 0, 0, 0, None),
            ("SpaceHVAC:EquipmentConnections", 7, 0, 0, 0, None),
            ("AirLoopHVAC:SupplyPath", 2, 0, 2, 0, Some("components")),
            ("AirLoopHVAC:ReturnPath", 2, 0, 2, 0, Some("components")),
        ] {
            definitions.push(crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: name.into(), alpha_fields: a, numeric_fields: n,
                alpha_extensions: ae, numeric_extensions: ne, extension: extension.map(str::to_owned),
            });
        }
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, std::collections::BTreeMap::new())?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        let mut control = SelectedHeatBalanceInputState { any_ctf: true, construction_order: vec![ConstructionId(0)],
            construction_usage: vec![Default::default()], ..Default::default() };
        let globals = crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals {
            num_spaces: 1, num_of_zones: 1, ..Default::default()
        };
        let environment = crate::weather::day::WeatherEnvironmentState::default();
        let mut owners = SurfaceGetterOwners { model: &model.typed, input: &mut input,
            shortcut: &mut shortcut, zones: &mut zones, control: &mut control,
            globals: &globals, environment: &environment };
        let mut errors = false;
        // Authored checkpoint only, not a GetVertices/World producer certificate.
        // This cube intentionally has inward floor/roof winding. Do not bypass
        // the real warning/reversal IO by calling GetVertices on these rows.
        for (index, row) in raw.surface_tmp.as_mut().ok_or("authored checkpoint rows")?.iter_mut().enumerate() {
            let points = row.vertex.as_ref().ok_or("authored checkpoint vertices")?.iter().map(|vertex| {
                let [x_m, y_m, z_m] = vertex.ok_or("authored written checkpoint vertex")?;
                Ok(ep_model::Point3 { x_m, y_m, z_m })
            }).collect::<Result<Vec<_>, &str>>()?;
            row.base_surface = i32::try_from(index + 1)?;
            raw.coordinate.max_vertices_per_surface = raw.coordinate.max_vertices_per_surface.max(points.len());
            let normal = crate::geometry::source_normal_area(&points);
            let orientation = crate::geometry::orientation_from_normal(&points, normal.newell_normal);
            row.vertex_progress.normal_area = Some(normal);
            row.vertex_progress.orientation_return = Some(orientation);
            row.vertex_progress.properties = Some(crate::geometry::finish_get_vertices_properties(normal, orientation));
        }
        // A non-default layer bound proves AllocateSurfaceWindows uses its operand.
        raw.initialize_coordinate_owners(&owners)?;
        raw.copy_final_opaque_surfaces(&mut owners, &mut context.backend, &mut errors, 3)?;
        raw.build_opaque_surface_lists(&mut owners, &mut context.backend, errors)?;
        assert!(raw.final_surfaces.as_ref().ok_or("actual pre-override Surface algorithm rows")?.iter()
            .all(|surface| surface.row.heat_transfer_algorithm == crate::heat_balance::selected_heat_balance_input::SelectedHeatTransferModel::Invalid));
        raw.report_surface_input_diagnostics(&mut context.backend)?;
        raw.read_empty_surface_boundary_inputs(&mut owners, &mut context.backend, context.eio, &mut errors)?;
        assert_eq!(owners.shortcut.current_module_object, "SurfaceProperty:HeatTransferAlgorithm:Construction");
        assert!(matches!(owners.input.family("SurfaceProperty:ExteriorNaturalVentedCavity"),
            Some(crate::heat_balance::selected_input_processor::SourceInputFamily::Null)));
        assert_eq!(raw.total_ext_vented_cavities, 0);
        assert!(raw.ext_vented_cavities.as_ref().is_some_and(Vec::is_empty));
        assert!(raw.final_surfaces.as_ref().ok_or("actual Surface algorithm rows")?.iter()
            .all(|surface| surface.row.heat_transfer_algorithm == crate::heat_balance::selected_heat_balance_input::SelectedHeatTransferModel::Ctf));
        assert!(owners.control.construction_usage[0].is_used_ctf);
        // Authored callback host tests the existing OP interface; actual A uses
        // the real retained filesystem MTD host. No OS success is inferred here.
        struct EnclosureTestHost { calls: usize, fail: bool }
        impl crate::heat_balance::source_output_registration::SourceOutputFileHost for EnclosureTestHost {
            fn ensure_mtd_open(&mut self) -> std::io::Result<()> {
                self.calls += 1;
                if self.fail { return Err(std::io::Error::other("authored enclosure MTD failure")); }
                Ok(())
            }
        }
        let radiation_input = crate::heat_balance::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory([], false)?;
        for (floor_only, fail) in [(false, false), (true, false), (false, true)] {
            let zone = &mut owners.zones.zones.as_mut().ok_or("actual enclosure Zone")?[0];
            zone.name = "authored Source Zone".into();
            zone.radiant_enclosure_first = -1; zone.radiant_enclosure_last = -1;
            let space = &mut owners.zones.spaces.as_mut().ok_or("actual enclosure Space")?[0];
            space.name = "authored Source Space".into(); space.floor_area = 25.0;
            space.radiant_enclosure_num = 0;
            let mut radiation = crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(radiation_input);
            let mut registration = crate::heat_balance::source_output_registration::SourceOutputRegistration::default();
            let prescan = crate::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            let mut host = EnclosureTestHost { calls: 0, fail };
            let mut registration_eio = Vec::new();
            let mut output = crate::heat_balance::radiation::ordinary::EnclosureOutputOwners {
                output: &mut registration, prescan: &prescan, minimum_report_frequency: "",
                minutes_in_time_step: 15, host: &mut host, eio: &mut registration_eio,
            };
            // Scoped authored class inputs cover the otherwise independent
            // floor-only grouping branch; no geometry/Native endpoint is inferred.
            let mut enclosure_surfaces = raw.clone();
            if floor_only {
                for surface in enclosure_surfaces.final_surfaces.as_mut().ok_or("authored floor-only rows")? {
                    surface.row.class = SurfaceClass::Floor;
                }
            }
            let result = radiation.setup_radiant_enclosures(&enclosure_surfaces, &mut owners, &mut output);
            assert_eq!(radiation.num_radiant_enclosures(), 1);
            assert_eq!(radiation.radiant_enclosures()[0].space_nums, [1]);
            assert_eq!(radiation.radiant_enclosures()[0].space_names, ["authored Source Space"]);
            assert_eq!(radiation.radiant_enclosures()[0].floor_area.to_bits(), 25.0_f64.to_bits());
            assert_eq!(radiation.radiant_enclosures()[0].mean_radiant_temperature.to_bits(), 0.0_f64.to_bits());
            assert_eq!(owners.zones.spaces.as_ref().ok_or("retained enclosure Space")?[0].radiant_enclosure_num, 1);
            let zone = &owners.zones.zones.as_ref().ok_or("retained enclosure Zone")?[0];
            if fail {
                assert!(result.is_err());
                assert_eq!((zone.radiant_enclosure_first, zone.radiant_enclosure_last), (-1, -1));
                assert!(registration.output_initialized);
                assert_eq!(registration.num_real_variables_setup, 0);
            } else {
                result?;
                assert_eq!((zone.radiant_enclosure_first, zone.radiant_enclosure_last), (1, 1));
                assert_eq!(registration.num_real_variables_setup, 1);
                assert_eq!(registration.dictionary_variables[0].name, "Enclosure Mean Radiant Temperature");
                assert_eq!(registration.dictionary_variables[0].key_out_var_nums.len(), 0,
                    "unmatched Source registration does not fabricate an MRT variable handle");
                let allocation = radiation.radiant_enclosures().as_ptr();
                radiation.bind_initialized_solar_geometry(&model.typed, &physical.surfaces, radiation_input)?;
                assert_eq!(radiation.radiant_enclosures().as_ptr(), allocation);
                assert_eq!(radiation.radiant_enclosures()[0].name,
                    if floor_only { "authored Source Zone" } else { "authored Source Space" });
                assert_eq!(radiation.radiant_enclosures()[0].floor_area.to_bits(), 25.0_f64.to_bits());
                context.reports.ordinary_solar_radiation = Some(radiation);
                let moved = context.reports.ordinary_solar_radiation.take().ok_or("lost same radiation container")?;
                assert_eq!(moved.radiant_enclosures().as_ptr(), allocation);
            }
            assert_eq!(host.calls, 1);
        }
        // Reuse this authored owner fixture for the newly reached zero suffix.
        assert!(raw.final_surfaces.as_ref().ok_or("actual pre-suffix Surface")?.iter()
            .all(|surface| surface.representative_calc_surface_num == -1));
        assert!(raw.window_arrays.as_ref().ok_or("actual pre-suffix window arrays")?
            .surf_active_construction.iter().all(|&value| value == 0));
        raw.finish_selected_surface_input(&mut owners, &mut context.backend, &mut errors)?;
        assert_eq!(owners.shortcut.current_module_object, "SurfaceProperty:LocalEnvironment");
        assert_eq!((raw.total_surface_ground_properties, raw.total_surface_local_environments), (0, 0));
        assert!(owners.input.find_exact_family("SurfaceProperty:GroundSurfaces").is_none());
        assert!(raw.ground_surface_properties.is_none() && raw.surrounding_surface_properties.is_none()
            && raw.surface_local_environments.is_none());
        assert!(raw.selected_get_surface_data_returned);
        assert_eq!(raw.final_surfaces.as_ref().ok_or("actual representative rows")?.iter()
            .map(|surface| surface.representative_calc_surface_num).collect::<Vec<_>>(), (1..=i32::try_from(total)?).collect::<Vec<_>>());
        assert_eq!(raw.window_arrays.as_ref().ok_or("actual active Construction owner")?
            .surf_active_construction, vec![1; total]);
        let mut failed_setup = raw.clone();
        assert!(failed_setup.release_setup_coordinates_if_errors(true));
        assert!(failed_setup.coordinate.cos_zone_relative.is_none() && failed_setup.coordinate.sin_zone_relative.is_none());
        assert!(raw.coordinate.cos_zone_relative.is_some() && raw.coordinate.sin_zone_relative.is_some());
        let clock = crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::from_family_inventory([], model.typed.timestep.number_of_timesteps_per_hour as usize)?;
        let mut equipment = crate::heat_balance::ordinary_equipment::OrdinaryEquipment::new_for_source(&model.typed, clock)?;
        let space_controls = crate::heat_balance::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls::default();
        let old_module = owners.shortcut.current_module_object.clone();
        equipment.get_zone_equipment_source(owners.input, clock, space_controls.sizing, space_controls.simulation)?;
        assert_eq!(owners.shortcut.current_module_object, old_module);
        for family in ["NodeList", "ZoneHVAC:EquipmentList", "ZoneHVAC:EquipmentConnections", "SpaceHVAC:EquipmentConnections", "AirLoopHVAC:SupplyPath", "AirLoopHVAC:ReturnPath"] {
            assert!(matches!(owners.input.find_exact_family(family), Some(crate::heat_balance::selected_input_processor::SourceInputFamily::Null)));
        }
        raw.finish_zero_window_setup(owners.input, owners.shortcut, errors)?;
        assert_eq!(owners.shortcut.current_module_object, "WindowProperty:StormWindow");
        assert_eq!(raw.total_storm_windows, 0);
        assert!(raw.storm_windows.is_none());
        assert!(raw.coordinate.cos_zone_relative.is_none() && raw.coordinate.sin_zone_relative.is_none());
        assert!(raw.process_vertices_scratch.one_time);
        assert!(raw.process_vertices_scratch.x.is_none());
        // Constructor-zero X0 is checked before the actual Process writer.
        assert!(raw.parallel_arrays.as_ref().ok_or("pre-Process parallel owner")?.x0.iter()
            .all(|value| value.to_bits() == 0.0_f64.to_bits()));
        raw.process_opaque_setup_surfaces(owners.zones, owners.control)?;
        assert!(!raw.process_vertices_scratch.one_time);
        let process_arrays = raw.parallel_arrays.as_ref().ok_or("actual processed parallel owner")?;
        let process_rows = raw.final_surfaces.as_ref().ok_or("actual processed final owner")?;
        for (row, shade) in process_rows.iter().zip(&process_arrays.shade_vertices) {
            assert!(row.row.vertices_processed && !row.row.is_shadowing);
            assert_eq!(row.row.shape, crate::geometry::SourceProcessShape::Rectangle);
            assert_eq!(shade.n_vert, row.row.sides);
            let vertices = row.row.vertex.as_ref().ok_or("actual Process vertex owner")?;
            for (component, values) in [shade.xv.as_ref(),shade.yv.as_ref(),shade.zv.as_ref()].into_iter().enumerate() {
                let values = values.ok_or("actual allocated ShadeV component")?;
                assert_eq!(values.len(),vertices.len());
                assert_eq!(values.iter().map(|v|v.to_bits()).collect::<Vec<_>>(),vertices.iter().map(|v|v.ok_or("actual written vertex").map(|p|p[component].to_bits())).collect::<Result<Vec<_>,_>>()?);
            }
        }
        assert_eq!(raw.surface_lists.all_exterior_solar_and_shading, raw.surface_lists.all_exterior_solar);
        assert_eq!(raw.setup_shading.last,-1);
        for zone in owners.zones.zones.as_ref().ok_or("actual Setup reset zones")? {
            assert!(!zone.has_window);
            assert_eq!([zone.exterior_window_area,zone.exterior_gross_wall_area,zone.exterior_net_wall_area,zone.total_surface_area],[0.0;4]);
        }
        for space in owners.zones.spaces.as_ref().ok_or("actual Setup reset spaces")? {
            assert_eq!([space.exterior_window_area,space.total_surface_area],[0.0;2]);
        }
        // Authored negative operands exercise reached failure custody, without
        // supplying Native AFTER values or broadening positive raw admission.
        let debug = SelectedInputProcessor::from_admitted_input(vec![
            crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: "DETAILEDWWR_DEBUG".into(), alpha_fields: 0, numeric_fields: 0,
                alpha_extensions: 0, numeric_extensions: 0, extension: None,
            }], std::collections::BTreeMap::from([("DETAILEDWWR_DEBUG".into(),
                crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(vec![
                    crate::heat_balance::selected_input_processor::SourceObjectShape {
                        object_name: "authored debug section".into(), field_sizes: Default::default(),
                    }]))]))?;
        assert_eq!(debug.get_num_sections_found("DETAILEDWWR_DEBUG")?, 1);
        let mut debug_zones = owners.zones.clone();
        let before_debug = debug_zones.clone();
        assert!(matches!(raw.accumulate_opaque_setup_areas(&debug, &mut debug_zones, owners.control),
            Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("positive DetailedWWR debug OutputFile"))));
        assert_eq!(debug_zones, before_debug, "debug header failure precedes every area write");
        let mut window_rows = raw.clone();
        let first = &mut window_rows.final_surfaces.as_mut().ok_or("authored area rows")?[0].row;
        first.class = SurfaceClass::Floor; // Reach the distinct geometric prefix in this test-owned row.
        let area = first.vertex_progress.properties.ok_or("authored retained Area")?.area_m2;
        let gross = first.vertex_progress.normal_area.ok_or("authored retained GrossArea")?.gross_area_m2;
        let mut window_control = owners.control.clone();
        window_control.construction_usage[0].type_is_window = true;
        let mut window_zones = owners.zones.clone();
        let mut expected_prefix = window_zones.clone();
        let zone = &mut expected_prefix.zones.as_mut().ok_or("authored prefix Zone")?[0];
        zone.total_surface_area += area;
        zone.geometric_floor_area += gross;
        expected_prefix.spaces.as_mut().ok_or("authored prefix Space")?[0].total_surface_area += area;
        assert!(matches!(window_rows.accumulate_opaque_setup_areas(owners.input, &mut window_zones, &window_control),
            Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("positive area window/frame branch"))));
        assert_eq!(window_zones, expected_prefix, "window boundary retains only the reached totals/geometric prefix");
        raw.bind_typed_surfaces(&owners)?;
        let mut height_zones = owners.zones.clone();
        let detailed_wwr = raw.accumulate_opaque_setup_areas(owners.input, &mut height_zones, owners.control)?;
        assert!(!detailed_wwr && raw.err_count == 0);
        raw.prepare_opaque_setup_heights(&mut height_zones, false, detailed_wwr, &mut context.backend)?;
        let zone = &height_zones.zones.as_ref().ok_or("actual automatic height Zone")?[0];
        assert_eq!(zone.ceiling_height.to_bits(), 1.0_f64.to_bits());
        assert!(!zone.ceiling_height_entered, "automatic positive height is not user-entered");
        // A real warning stream failure must preserve the prior entered flag
        // write, without assigning the automatic height or fabricating ErrCount.
        let mut warning_rows = raw.clone();
        warning_rows.err_count = 1;
        let mut warning_zones = height_zones.clone();
        warning_zones.zones.as_mut().ok_or("authored entered height Zone")?[0].ceiling_height = 2.0;
        let mut expected_failure = warning_zones.clone();
        expected_failure.zones.as_mut().ok_or("authored warning prefix")?[0].ceiling_height_entered = true;
        let (mut warning_state, mut warning_console, mut failing) =
            (ctf_ordinary_output_test_state(), Vec::new(), OrdinaryFailingEio);
        let failure = warning_rows.prepare_opaque_setup_heights(&mut warning_zones, false, detailed_wwr,
            &mut SourceErrorBackend { state: &mut warning_state, error_stream: Some(&mut failing),
                console: &mut warning_console, out_unit1: None, out_unit2: None, f_message: None,
                message_callback: None, error_callback: None, sqlite: None });
        assert!(matches!(failure, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(_))));
        assert_eq!(warning_zones, expected_failure);
        assert_eq!(warning_rows.err_count, 1);
        assert_eq!(warning_state.tracking.total_warning_errors, 1);
        // All six authored cube faces are inward-wound. Reverse every copied
        // Vertex list; cached Newell values stay unchanged so548 recomputes them.
        let mut volume_rows = raw.clone();
        for surface in volume_rows.final_surfaces.as_mut().ok_or("authored volume rows")? {
            surface.row.vertex.as_mut().ok_or("authored volume vertices")?.reverse();
        }
        let mut volume_zones = height_zones.clone();
        volume_zones.zones.as_mut().ok_or("authored floor-present checkpoint")?[0].has_floor = true;
        let before_volume = volume_zones.clone();
        let returned = volume_rows.calculate_opaque_setup_volumes(owners.input, &mut volume_zones, false, &mut context.backend)?;
        assert_eq!(returned.len(), 1);
        assert_eq!(returned[0].returned.volume_m3.to_bits(), 1.0_f64.to_bits());
        assert_eq!(returned[0].returned.diagnostics.sum_face_areas_m2.to_bits(), 6.0_f64.to_bits());
        assert_eq!(volume_zones.spaces.as_ref().ok_or("actual computed Space volume")?[0].volume.to_bits(), 1.0_f64.to_bits());
        assert_eq!(volume_zones.spaces.as_ref().ok_or("actual computed fraction")?[0].fraction_zone_volume.to_bits(), 1.0_f64.to_bits());
        let show_debug = SelectedInputProcessor::from_admitted_input(vec![
            crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: "SHOWZONESURFACES_DEBUG".into(), alpha_fields: 0, numeric_fields: 0,
                alpha_extensions: 0, numeric_extensions: 0, extension: None,
            }], std::collections::BTreeMap::from([("SHOWZONESURFACES_DEBUG".into(),
                crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(vec![
                    crate::heat_balance::selected_input_processor::SourceObjectShape {
                        object_name: "authored debug section".into(), field_sizes: Default::default(),
                    }]))]))?;
        let mut debug_volume = before_volume.clone();
        assert!(matches!(volume_rows.calculate_opaque_setup_volumes(&show_debug, &mut debug_volume, false, &mut context.backend),
            Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("positive SHOWZONESURFACES debug OutputFile"))));
        assert_eq!(debug_volume, volume_zones, "first debug print follows Zone/Space/fraction writes");
        let mut retained_user_volume = before_volume.clone();
        retained_user_volume.zones.as_mut().ok_or("authored positive user volume")?[0].volume = 1.01;
        let retained = volume_rows.calculate_opaque_setup_volumes(owners.input, &mut retained_user_volume, false, &mut context.backend)?;
        assert_eq!(retained[0].returned.volume_m3.to_bits(), 1.01_f64.to_bits());
        assert_eq!(retained_user_volume.spaces.as_ref().ok_or("user-priority Space volume")?[0].volume.to_bits(), 1.01_f64.to_bits());
        let mut user_volume = before_volume;
        user_volume.zones.as_mut().ok_or("authored user volume")?[0].volume = 2.0;
        let before_failure = user_volume.clone();
        let (mut note_state, mut note_console, mut note_failure) =
            (ctf_ordinary_output_test_state(), Vec::new(), OrdinaryFailingEio);
        let failure = volume_rows.calculate_opaque_setup_volumes(owners.input, &mut user_volume, false,
            &mut SourceErrorBackend { state: &mut note_state, error_stream: Some(&mut note_failure),
                console: &mut note_console, out_unit1: None, out_unit2: None, f_message: None,
                message_callback: None, error_callback: None, sqlite: None });
        assert!(matches!(failure, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(_))));
        assert_eq!(volume_rows.err_count5, 1, "counter increment precedes failed informational note");
        assert_eq!(user_volume, before_failure, "user volume and pending Space fields survive note failure");
        assert_eq!(note_state.tracking.total_warning_errors, 0);
        // Authored checkpoints use the genuine earlier centroid producer and
        // actual550 suffix, without Native AFTER injection or new test bodies.
        let mut bounds_rows = raw.clone();
        bounds_rows.calculate_final_centroids(&mut context.backend, &environment)?;
        bounds_rows.final_surfaces.as_mut().ok_or("authored self-adjacency")?[0].row.exterior_boundary = 1;
        let mut bounds_zones = volume_zones.clone();
        assert!(bounds_rows.adjacent_zone.is_none());
        assert_eq!(bounds_zones.zones.as_ref().ok_or("ctor-zero counts")?[0].num_surfaces, 0);
        bounds_rows.prepare_opaque_setup_zone_bounds(&mut bounds_zones, 1, &mut context.backend, &mut errors)?;
        let zone = &bounds_zones.zones.as_ref().ok_or("actual bounds Zone")?[0];
        assert_eq!(zone.centroid_m, Some([0.5; 3]));
        assert_eq!(zone.bounds_m, [0.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
        assert_eq!((zone.num_surfaces, zone.num_sub_surfaces, zone.num_shading_surfaces), (6, 0, 0));
        let mut expected_adjacent = vec![0; total]; expected_adjacent[0] = 1;
        assert_eq!(bounds_rows.adjacent_zone, Some(expected_adjacent.clone()));
        bounds_rows.prepare_opaque_setup_zone_bounds(&mut bounds_zones, 1, &mut context.backend, &mut errors)?;
        assert_eq!(bounds_zones.zones.as_ref().ok_or("cumulative counts")?[0].num_surfaces, 12);
        let before_adjacency_failure = bounds_zones.clone();
        bounds_rows.adjacent_zone = Some(vec![-7; total]);
        bounds_rows.final_surfaces.as_mut().ok_or("authored missing adjacency")?[1].row.exterior_boundary = i32::try_from(total + 1)?;
        let failure = bounds_rows.prepare_opaque_setup_zone_bounds(&mut bounds_zones, 1, &mut context.backend, &mut errors);
        assert!(matches!(failure, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("actual adjacent Surface association"))));
        assert_eq!(bounds_zones, before_adjacency_failure, "completed centroid/bounds and prior volume retained; counts not reached");
        assert_eq!(bounds_rows.adjacent_zone, Some(expected_adjacent), "new dimension and first entry retained at invalid second association");
        assert!(!errors);
        assert!(raw.surface_tmp.is_none());
        let source_ids = raw.final_surfaces.as_ref().ok_or("missing final Surface")?
            .iter().map(|row| row.typed_id.ok_or("missing final physical ID")).collect::<Result<Vec<_>, _>>()?;
        assert_eq!(source_ids, [SurfaceId(2), SurfaceId(3), SurfaceId(4), SurfaceId(5), SurfaceId(0), SurfaceId(1)]);
        // Authored constructor-type owner; actual Material type producers are exercised by A-24H.
        let source_numbers = (1..=i32::try_from(total)?).collect::<Vec<_>>();
        let zone = &zones.zones.as_ref().ok_or("authored Source list Zone")?[0];
        for list in [&raw.surface_lists.all_ht, &raw.surface_lists.all_ht_non_window,
            &raw.surface_lists.all_exterior_solar, &raw.surface_lists.all_shadow_possible_obstructions,
            &zone.ht_surface_list, &zone.ht_non_window_surface_list] {
            assert_eq!(list, &source_numbers);
        }
        assert!(raw.surface_lists.all_interzone.is_empty() && zone.iz_surface_list.is_empty());
        assert!(raw.surface_lists.all_ht_window.is_empty() && zone.ht_window_surface_list.is_empty());
        assert_eq!(raw.surface_lists.filters[0], source_numbers);
        assert_eq!(raw.surface_lists.filters[2], [1, 2, 3, 4]);
        assert_eq!(raw.surface_lists.filters[3], [6]);
        assert_eq!(raw.surface_lists.filters[4], [5]);
        // Literal2788 appends exterior Roof to AllInteriorCeilings.
        assert_eq!(raw.surface_lists.filters[9], [6]);
        let arrays = raw.parallel_arrays.as_ref().ok_or("missing real parallel allocation")?;
        assert_eq!(arrays.surf_penumbra_id, vec![-1; total]);
        assert_eq!(arrays.surf_t_air_ref, vec![-1; total]);
        assert_eq!(arrays.x0.len(), total);
        let windows = raw.window_arrays.as_ref().ok_or("missing actual window allocation")?;
        assert_eq!((windows.rows, windows.equivalent_layer_columns, windows.solid_window_columns), (total, 7, 3));
        assert_eq!((windows.surf_win_a.len(), windows.surf_win_acf_overlap.len()), (total * 7, total * 3));
        assert!(windows.surf_win_a.iter().chain(&windows.surf_win_acf_overlap).all(|value| value.to_bits() == 0.0_f64.to_bits()));
        let derived = raw.surface_windows.as_ref().ok_or("missing real SurfaceWindowCalc allocation")?;
        assert_eq!(derived.len(), total);
        assert_eq!(raw.surface_shades.as_ref().ok_or("missing real SurfaceShade allocation")?.len(), total);
        for row in derived {
            assert!(row.reference_points.is_none());
            assert!(row.complex_fenestration.states.is_none());
            assert_eq!(row.complex_fenestration.current_state, 1);
            assert_eq!(row.theta_face[0].to_bits(), 296.15_f64.to_bits());
            assert!(row.theta_face[1..].iter().all(|value| value.to_bits() == 0.0_f64.to_bits()));
            assert_eq!(row.outside_projection_sunlit_multiplier[0].to_bits(), 1.0_f64.to_bits());
            assert!(row.outside_projection_sunlit_multiplier[1..].iter().all(|value| value.to_bits() == 0.0_f64.to_bits()));
        }
        let outdoor = raw.parallel_arrays.as_mut().and_then(|arrays| arrays.outdoor.as_mut()).ok_or("missing Source outdoor arrays")?;
        for (index, owner) in outdoor.iter_mut().enumerate() {
            let marker = f64::from(u32::try_from(index)?);
            owner.row.dry_bulb_c = 101.0 + marker;
            owner.row.wet_bulb_c = 201.0 + marker;
            owner.row.wind_speed_m_per_s = if index == 0 { -0.0 } else { 301.0 + marker };
            owner.row.wind_direction_deg = 401.0 + marker;
            owner.manager_invoked = index % 2 == 0;
        }
        raw.bind_outdoor_geometry()?;
        let bits = |owner: &crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentState| {
            [owner.row.dry_bulb_c, owner.row.wet_bulb_c, owner.row.wind_speed_m_per_s, owner.row.wind_direction_deg].map(f64::to_bits)
        };
        let original = raw.parallel_arrays.as_ref().and_then(|arrays| arrays.outdoor.as_ref()).ok_or("missing Source custody")?;
        let allocation = original.as_ptr();
        let expected = original.iter().map(bits).collect::<Vec<_>>();
        assert_eq!(expected[0][2], (-0.0_f64).to_bits());
        context.reports.surface_geometry_input = raw;
        // Missing cardinality and duplicate physical IDs must both reject before take.
        for duplicate in [false, true] {
            let mut invalid = model.typed.clone();
            if duplicate { invalid.surfaces[0].id = invalid.surfaces[1].id; } else { invalid.surfaces.pop(); }
            assert!(context.take_selected_world_outdoor_rows(&invalid).is_err());
            let arrays = context.reports.surface_geometry_input.parallel_arrays.as_ref().ok_or("lost parallel arrays on rejection")?;
            let retained = arrays.outdoor.as_ref().ok_or("invalid mapping moved Source owner")?;
            assert_eq!(retained.as_ptr(), allocation);
            assert_eq!(retained.iter().map(bits).collect::<Vec<_>>(), expected);
            assert!(arrays.outdoor_to_model_swaps.is_empty());
        }
        let moved = context.take_selected_world_outdoor_rows(&model.typed)?;
        assert_eq!(moved.as_ptr(), allocation, "take permutes the original allocation without cloning rows");
        assert!(context.reports.surface_geometry_input.parallel_arrays.as_ref().ok_or("lost arrays after take")?.outdoor.is_none());
        assert_eq!(physical.surfaces.len(), moved.len());
        for ((surface, input), owner) in physical.surfaces.iter_mut().zip(&model.typed.surfaces).zip(moved) {
            assert_eq!(surface.surface_id, input.id);
            let source = source_ids.iter().position(|id| *id == surface.surface_id).ok_or("missing fixture physical association")?;
            assert_eq!(bits(&owner), expected[source]);
            assert_eq!(owner.manager_invoked, source % 2 == 0);
            surface.outdoor_environment = owner;
            assert_eq!(bits(&surface.outdoor_environment), expected[source]);
        }
        // A real post-take guard error exercises reverse custody, not a claimed
        // Native/Solar rollback endpoint or another fabricated caller phase.
        assert!(context.take_selected_world_outdoor_rows(&model.typed).is_err());
        context.restore_selected_world_outdoor_rows(physical.surfaces);
        let arrays = context.reports.surface_geometry_input.parallel_arrays.as_ref().ok_or("lost arrays at restore")?;
        let restored = arrays.outdoor.as_ref().ok_or("lost actual outdoor values at restore")?;
        assert_eq!(restored.iter().map(bits).collect::<Vec<_>>(), expected);
        assert!(restored.iter().enumerate().all(|(index, owner)| owner.manager_invoked == (index % 2 == 0)));
        assert!(arrays.outdoor_to_model_swaps.is_empty());
        assert_eq!(context.reports.surface_geometry_input.final_surfaces.as_ref().ok_or("lost final Surface IDs")?
            .iter().map(|row| row.typed_id).collect::<Vec<_>>(), source_ids.iter().copied().map(Some).collect::<Vec<_>>());
        // Authored reached Material:NoMass checkpoint, not Native R/U output.
        // The actual Construction getter must accumulate both repeated layers.
        use crate::heat_balance::selected_heat_balance_input::{
            SelectedHeatBalanceInput, ConstructionMaterialOwners,
        };
        use crate::heat_balance::selected_material_data::{SelectedMaterialData, SourceMaterialGroup};
        use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
        let materials = SelectedMaterialData { materials: vec![MaterialId(0)], nominal_r: vec![1.0],
            material_map: std::collections::BTreeMap::from([("R1".into(), 1)]),
            material_groups: std::collections::BTreeMap::from([(MaterialId(0), SourceMaterialGroup::Regular)]),
            ..Default::default() };
        let mut layered = model.typed.clone(); layered.constructions[0].layers.push(MaterialId(0));
        let mut construction_input = SelectedInputProcessor::from_admitted_input(vec![
            crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: "Construction".into(), alpha_fields: 11, numeric_fields: 0,
                alpha_extensions: 0, numeric_extensions: 0, extension: None,
            }], std::collections::BTreeMap::from([("Construction".into(),
                crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(vec![
                    crate::heat_balance::selected_input_processor::SourceObjectShape {
                        object_name: "Wall".into(), field_sizes: Default::default(),
                    }]))]))?;
        let construction_raw = SelectedHeatBalanceInput { source_is_epjson: true,
            objects: std::collections::BTreeMap::from([("Construction".into(), vec![ProjectInputObject {
                object_name: "Wall".into(), idf_max_fields: None,
                fields: std::collections::BTreeMap::from([("outside_layer".into(), ProjectInputValue::String("R1".into())),
                    ("layer_2".into(), ProjectInputValue::String("R1".into()))]),
            }])]), ..Default::default() };
        let mut nominal = SelectedHeatBalanceInputState::default();
        assert!(nominal.nominal_r_for_u.is_none() && nominal.nominal_u.is_none()
            && nominal.nominal_u_before_adjusted.is_none() && nominal.coefficient_adjustment_ratio.is_none());
        let mut construction_shortcut = SourceInputProcessorShortCut::default();
        construction_shortcut.allocate_from_input(&construction_input)?;
        nominal.read_regular_construction_arguments(&construction_raw,
            ConstructionMaterialOwners { model: &layered, data: &materials }, &mut construction_input,
            &mut construction_shortcut, &mut String::new(), &mut context.backend, &mut errors)?;
        assert_eq!(nominal.nominal_r_for_u, Some(vec![2.0]));
        assert_eq!(nominal.nominal_u, Some(vec![0.5]));
        assert_eq!(nominal.nominal_u_before_adjusted.as_ref().ok_or("actual U-before dimension")?[0].to_bits(), 0.0_f64.to_bits());
        assert_eq!(nominal.coefficient_adjustment_ratio, Some(vec![1.0]));
        // Same authored owners exercise report-ID order, referenced-class film
        // selection, and the actual table-before-EIO partial-failure boundary.
        let mut report_owner = CtfInitializationReports {
            registry: context.reports.registry.clone(),
            surface_geometry_input: context.reports.surface_geometry_input.clone(),
            selected_zone_input: Some(bounds_zones.clone()), selected_heat_balance_globals: Some(globals.clone()),
            heat_balance_input_state: nominal.clone(), ..Default::default()
        };
        let first = usize::try_from(report_owner.surface_geometry_input.all_surface_list_report_order[0] - 1)?;
        let rows = report_owner.surface_geometry_input.final_surfaces.as_mut().ok_or("actual report rows")?;
        assert_eq!(rows[first].row.class, SurfaceClass::Wall);
        let floor = rows.iter().position(|row| row.row.class == SurfaceClass::Floor).ok_or("authored referenced Floor")?;
        rows[first].row.exterior_boundary = i32::try_from(floor + 1)?;
        report_owner.selected_zone_input.as_mut().and_then(|owner| owner.zones.as_mut())
            .ok_or("actual report Zone")?[0].name = "AUTHORED ZONE".into();
        let mut report_eio = Vec::new();
        report_owner.write_selected_setup_geometry_reports(&mut report_eio)?;
        assert_eq!(report_owner.table.num_table_entry, i32::try_from(total)?);
        let entries = report_owner.table.entries.as_ref().ok_or("actual film entries")?;
        assert_eq!(entries[0].original_real_entry.to_bits(), (1.0_f64 / (0.1197548 + 2.0 + 0.1620212)).to_bits());
        let expected_names = report_owner.surface_geometry_input.all_surface_list_report_order.iter()
            .map(|number| report_owner.surface_geometry_input.final_surfaces.as_ref().ok_or("report allocation")?
                .get(usize::try_from(number - 1)?).map(|row| row.row.name.as_str()).ok_or("report association".into()))
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
        assert_eq!(entries[..total].iter().map(|entry| entry.object_name.as_str()).collect::<Vec<_>>(), expected_names);
        let report_text = String::from_utf8(report_eio)?;
        assert_eq!(report_text.lines().count(), 6);
        assert!(report_text.contains(" Shading Summary,0,0,0\n"));
        assert!(report_text.contains(" Zone Summary,1,6,0\n"));
        assert!(report_text.contains(" Zone Information, AUTHORED ZONE,0.0,0.00,0.00,0.00,0.50,0.50,0.50,"));
        assert!(report_text.ends_with(",12,0,0,Yes\n"));
        let first_header = report_text.lines().next().ok_or("first report header")?.to_owned() + "\n";
        let row = &mut report_owner.surface_geometry_input.final_surfaces.as_mut().ok_or("invalid construction report row")?[first].row;
        row.construction = 0; row.exterior_boundary = -3; // No Compute call, so OSC is not reached.
        let mut prefix = vec![0_u8; first_header.len()];
        let failure = report_owner.write_selected_setup_geometry_reports(&mut std::io::Cursor::new(prefix.as_mut_slice()));
        assert!(matches!(failure, Err(crate::RuntimeError::WarmupOutput(error)) if matches!(*error, crate::error::WarmupOutputError::Io(_))));
        assert_eq!(prefix, first_header.as_bytes());
        assert_eq!(report_owner.table.num_table_entry, i32::try_from(2 * total)?);
        assert_eq!(report_owner.table.entries.as_ref().ok_or("retained second film entries")?[total].original_real_entry.to_bits(), 0.0_f64.to_bits());
        nominal.nominal_r_for_u = Some(vec![0.0]); nominal.construction_usage[0].type_is_window = true;
        let retained = nominal.clone();
        let (mut zero_state, mut zero_console, mut zero_failure) =
            (ctf_ordinary_output_test_state(), Vec::new(), OrdinaryFailingEio);
        let failure = nominal.finish_selected_construction_nominal_u(&materials,
            &mut SourceErrorBackend { state: &mut zero_state, error_stream: Some(&mut zero_failure),
                console: &mut zero_console, out_unit1: None, out_unit2: None, f_message: None,
                message_callback: None, error_callback: None, sqlite: None }, &mut errors);
        assert!(matches!(failure, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(_))));
        assert_eq!(nominal, retained, "zero-R failed severe precedes ErrorsFound and Type* writes");
        assert!(!errors); assert_eq!(zero_state.tracking.total_severe_errors, 1);
        let reached_eio = String::from_utf8(eio)?;
        assert_eq!(reached_eio.matches("! <Surface Heat Transfer Algorithm>").count(), 1);
        assert!(reached_eio.contains("Surface Heat Transfer Algorithm, CTF - ConductionTransferFunction,200,0.10,1000.0\n"));
    }
    check_setup_process_native_reference_if_requested()?;
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
