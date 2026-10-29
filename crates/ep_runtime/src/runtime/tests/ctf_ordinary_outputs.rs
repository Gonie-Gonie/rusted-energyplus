include!("ctf_ordinary_process_reference.rs");

fn ctf_ordinary_output_test_state() -> crate::heat_balance::error_backend::SourceErrorState {
    use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
    SourceErrorState {
        output_error_header: true,
        version_string: "authored Rust caller".into(),
        idd_version_string: "authored typed input".into(),
        doing_input_processing: false,
        print_console_output: true,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
            match_counts: [0; 20],
            abort_processing: false,
            total_severe_errors: 0,
            last_severe_error: String::new(),
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

fn ctf_ordinary_output_test_model() -> TypedModel {
    let mut model = cube_model();
    model.ctf_report_names.materials = vec![(MaterialId(0), "R1".into())];
    model.ctf_report_names.constructions = vec![(ConstructionId(0), "Wall".into())];
    model
}

#[test]
fn ordinary_cache_uses_real_registry_columns_and_postlude_before_lifecycle()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{
        CtfInitializationContext, CtfInitializationReports,
    };
    use crate::heat_balance::error_backend::SourceErrorBackend;
    let model = ctf_ordinary_output_test_model();
    let mut reports = CtfInitializationReports::default();
    let mut state = ctf_ordinary_output_test_state();
    let (mut error, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
    let mut context = CtfInitializationContext {
        reports: &mut reports,
        eio: &mut eio,
        backend: SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut error),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        },
    };
    context.prepare_registry(&model)?;
    let mut cache=crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
        &model,&std::collections::BTreeMap::new(),&mut context)?;
    assert!(
        cache
            .take_public_surface_lifecycle()
            .transpose()
            .map_err(crate::RuntimeError::CtfSurfaceLifecycle)?
            .is_some()
    );
    assert!(reports.progress.postlude_returned);
    assert_eq!(reports.progress.layer_report_declared_indices, [0]);
    assert_eq!(reports.progress.construction_scan_result, Some(false));
    let column = reports.registry.opaque_layer_columns()[0];
    let entries = reports
        .table
        .entries
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing real table owner"))?;
    assert_eq!(reports.table.num_table_entry, 1);
    assert_eq!(entries[0].index_column, column);
    assert_eq!(entries[0].object_name, "Wall");
    assert_eq!(entries[0].char_entry.as_slice(), b"R1");
    assert!(error.is_empty());
    assert!(eio.is_empty());
    // Authored typed input: exercise the real optional Solar initializer,
    // not an observed Native owner endpoint or another weather simulation.
    for (surface_count, success) in [(1_usize, true), (4, false)] {
        let mut typed = ctf_ordinary_output_test_model();
        typed.zones[0].ceiling_height = AutoOrNumber::Value(1.0);
        typed.zones[0].floor_area = AutoOrNumber::Value(1.0);
        let wall = typed
            .surfaces
            .iter()
            .find(|surface| surface.surface_type == SurfaceType::Wall)
            .cloned()
            .ok_or("missing authored Wall fixture")?;
        typed.surfaces = (0..surface_count)
            .map(|index| {
                let mut surface = wall.clone();
                surface.id = SurfaceId(u32::try_from(index)?);
                surface.name = NormalizedName::new(&format!("authored parallel wall {index}"));
                Ok::<_, Box<dyn std::error::Error>>(surface)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut inside = typed.materials[0].clone();
        inside.id = MaterialId(1);
        inside.name = NormalizedName::new("authored inside layer");
        let ep_model::MaterialDefinition::NoMass(material) = &mut inside.definition else {
            return Err("expected authored opaque inside material".into());
        };
        material.surface.solar_absorptance = 0.25;
        typed.materials.push(inside);
        typed
            .ctf_report_names
            .materials
            .push((MaterialId(1), "authored inside layer".into()));
        typed.constructions[0].layers.push(MaterialId(1));
        typed.other_equipment[0].fraction_radiant = 0.25;
        let model = SimulationModel::from_typed(typed);
        let (schedule_cache, mut profile) =
            crate::schedules::precompute_hour_only_internal_gain_schedule_cache_profiled(
                &model.typed,
            )?;
        assert_eq!(profile.cache_entry_count, 1);
        let (mut reports, mut state, mut error, mut console, mut eio) = (
            CtfInitializationReports::default(),
            ctf_ordinary_output_test_state(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let result = {
            let mut context = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut error),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            };
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
            assert!(
                reports.ordinary_solar_radiation.is_none(),
                "successful owner must move into State"
            );
            let surface = &output.surfaces[0];
            assert_eq!(
                surface.inside_solar_absorptance.to_bits(),
                0.25_f64.to_bits()
            );
            assert_eq!(surface.solar_absorptance.to_bits(), 0.75_f64.to_bits());
            assert_eq!(
                output.zones[0].convective_internal_gain_w.to_bits(),
                9.0_f64.to_bits()
            );
            assert!((surface.inside_radiant_internal_gain_w_per_m2 - 3.0).abs() < 1.0e-12);
            assert_eq!(
                profile.initialization.cached_value_lookup_count, 2,
                "both actual gain consumers run after successful InitSolar"
            );
            // Authored Rust failure-custody payload, not Native getter endpoints.
            // Preserve the actual initialized Surface/Solar allocations by move.
            use crate::heat_balance::selected_heat_balance_input::zone::{
                SelectedZoneData, SelectedZoneInputOwners,
            };
            output.selected_zone_input = Some(SelectedZoneInputOwners {
                zones: Some(vec![SelectedZoneData {
                    typed_zone_id: Some(model.typed.zones[0].id),
                    name: model.typed.zones[0].name.0.clone(),
                    dry_bulb: -101.0,
                    wet_bulb: -99.0,
                    wind_speed: -0.0,
                    ..Default::default()
                }]),
                ..Default::default()
            });
            output.selected_heat_balance_globals = Some(Default::default());
            let surface_allocation = output.surfaces.as_ptr();
            let zone_allocation = output
                .selected_zone_input
                .as_ref()
                .and_then(|owner| owner.zones.as_ref())
                .ok_or("authored custody Zone allocation")?
                .as_ptr();
            assert!(reports.failed_heat_balance_state.is_none());
            {
                let mut context = CtfInitializationContext {
                    reports: &mut reports,
                    eio: &mut eio,
                    backend: SourceErrorBackend {
                        state: &mut state,
                        error_stream: Some(&mut error),
                        console: &mut console,
                        out_unit1: None,
                        out_unit2: None,
                        f_message: None,
                        message_callback: None,
                        error_callback: None,
                        sqlite: None,
                    },
                };
                crate::heat_balance::convection::surface_environment::ordinary::retain_failed_state(
                    output,
                    Some(&mut context),
                );
            }
            assert!(reports.selected_zone_input.is_none());
            assert!(reports.selected_heat_balance_globals.is_none());
            let mut failed = reports
                .failed_heat_balance_state
                .take()
                .ok_or("missing moved failed State")?;
            assert_eq!(failed.surfaces.as_ptr(), surface_allocation);
            let rows = failed
                .selected_zone_input
                .as_ref()
                .and_then(|owner| owner.zones.as_ref())
                .ok_or("failed State lost Zone allocation")?;
            assert_eq!(rows.as_ptr(), zone_allocation);
            assert_eq!([rows[0].dry_bulb, rows[0].wet_bulb], [-101.0, -99.0]);
            assert_eq!(rows[0].wind_speed.to_bits(), (-0.0_f64).to_bits());
            assert!(failed.selected_heat_balance_globals.is_some());
            failed
                .ordinary_solar_radiation
                .take()
                .ok_or("failed State lost moved actual Solar owner")?
        } else {
            use crate::heat_balance::radiation::ordinary::{
                OrdinaryRadiationError, OrdinaryRadiationUnavailable,
            };
            assert!(matches!(result, Err(RuntimeError::OrdinaryRadiation(error))
                if matches!(*error, OrdinaryRadiationError::Unavailable(OrdinaryRadiationUnavailable::NumericDomain))));
            assert_eq!(
                profile.initialization.cached_value_lookup_count, 0,
                "neither convective nor radiant gain consumer may precede the failed Solar prefix"
            );
            reports
                .ordinary_solar_radiation
                .take()
                .ok_or("missing retained partial Solar owner")?
        };
        assert_eq!(owner.is_initialized(), success);
        assert_eq!(owner.enclosures().len(), 1);
        let enclosure = &owner.enclosures()[0];
        assert_eq!(enclosure.id, 1);
        assert_eq!(enclosure.zone_id, Some(model.typed.zones[0].id));
        assert_eq!(enclosure.space_ids, [model.typed.spaces[0].id]);
        assert_eq!(
            enclosure.surface_ptr,
            model
                .typed
                .surfaces
                .iter()
                .map(|surface| surface.id)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            enclosure.surface_indexes,
            (0..surface_count).collect::<Vec<_>>()
        );
        assert_eq!(
            enclosure.surface_report_nums,
            (0..surface_count).collect::<Vec<_>>()
        );
        assert_eq!(
            owner.back_pointers(),
            (0..surface_count)
                .map(|index| (1, 1, index + 1))
                .collect::<Vec<_>>()
        );
        assert_eq!(enclosure.area, vec![1.0; surface_count]);
        assert_eq!(enclosure.absorptance, vec![0.25; surface_count]);
        assert_eq!(enclosure.azimuth.len(), surface_count);
        assert_eq!(enclosure.tilt.len(), surface_count);
        assert!(
            enclosure
                .azimuth
                .iter()
                .all(|value| value.to_bits() == enclosure.azimuth[0].to_bits())
        );
        assert!(
            enclosure
                .tilt
                .iter()
                .all(|value| value.to_bits() == enclosure.tilt[0].to_bits())
        );
        assert_eq!(enclosure.f.len(), surface_count * surface_count);
        assert!(
            enclosure
                .f
                .iter()
                .all(|value| value.to_bits() == 0.0_f64.to_bits()),
            "N1 skips numeric work; buffered-warning rejection retains allocated +0 F"
        );
        let text = std::str::from_utf8(&eio)?;
        assert_eq!(
            text.matches("! <Solar View Factor Check Values>").count(),
            1
        );
        assert_eq!(
            text.lines()
                .filter(|line| line.starts_with("Solar View Factor Check Values,"))
                .count(),
            usize::from(success)
        );
        if success {
            assert!(text.contains(&format!(
                "Solar View Factor Check Values,{},0,0,0,-1,0,0",
                model.typed.spaces[0].name.0
            )));
        }
        assert_eq!(
            state.tracking.total_warning_errors, 0,
            "buffered approximate warnings are unavailable, not emitted as Original mid-loop effects"
        );
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
            SelectedHeatBalanceInputState,
            zone::{SelectedZoneData, SelectedZoneInputOwners},
        };
        use crate::heat_balance::selected_input_processor::{
            SelectedInputProcessor, SourceInputProcessorShortCut,
        };
        let model = SimulationModel::from_typed(ctf_ordinary_output_test_model());
        let total = model.typed.surfaces.len();
        assert_eq!(total, 6);
        let (schedule_cache, mut profile) =
            crate::schedules::precompute_hour_only_internal_gain_schedule_cache_profiled(
                &model.typed,
            )?;
        let (mut reports, mut state, mut error, mut console, mut eio) = (
            CtfInitializationReports::default(),
            ctf_ordinary_output_test_state(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let mut context = CtfInitializationContext {
            reports: &mut reports,
            eio: &mut eio,
            backend: SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        };
        let mut physical = crate::heat_balance::initialization::initialize_heat_balance_state_with_initialization_context_and_equipment_profiled(
            &model, 23.0, &[], &schedule_cache, &mut profile, &mut context, Default::default(),
        )?;
        let mut raw = SurfaceGeometryInputState {
            total_surfaces: i32::try_from(total)?,
            surfaces_read: total,
            next_object: total,
            surface_tmp: Some(
                model
                    .typed
                    .surfaces
                    .iter()
                    .map(|surface| SurfaceTmpInput {
                        name: surface.name.0.clone(),
                        class: match surface.surface_type {
                            SurfaceType::Wall => SurfaceClass::Wall,
                            SurfaceType::Floor => SurfaceClass::Floor,
                            SurfaceType::Roof | SurfaceType::Ceiling => SurfaceClass::Roof,
                        },
                        construction: 1,
                        zone: 1,
                        space: 1,
                        heat_transfer_surface: true,
                        exterior_solar: true,
                        exterior_wind: true,
                        sides: 4,
                        vertex: Some(
                            surface
                                .vertices
                                .iter()
                                .map(|p| Some([p.x_m, p.y_m, p.z_m]))
                                .collect(),
                        ),
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        };
        let mut zones = SelectedZoneInputOwners {
            zones: Some(vec![SelectedZoneData {
                typed_zone_id: Some(model.typed.zones[0].id),
                space_indexes: vec![1],
                num_spaces: 1,
                ..Default::default()
            }]),
            spaces: Some(vec![Default::default()]),
            ..Default::default()
        };
        let space = &mut zones
            .spaces
            .as_mut()
            .ok_or("authored actual Space allocation")?[0];
        space.typed_space_id = Some(model.typed.spaces[0].id);
        space.zone_num = 1;
        let mut definitions = vec![
            crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: "SurfaceProperty:ExteriorNaturalVentedCavity".into(),
                alpha_fields: 3,
                numeric_fields: 8,
                alpha_extensions: 1,
                numeric_extensions: 0,
                extension: Some("surface".into()),
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
            definitions.push(
                crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                    object_type: name.into(),
                    alpha_fields: a,
                    numeric_fields: n,
                    alpha_extensions: ae,
                    numeric_extensions: ne,
                    extension: extension.map(str::to_owned),
                },
            );
        }
        let mut input = SelectedInputProcessor::from_admitted_input(
            definitions,
            std::collections::BTreeMap::new(),
        )?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        let mut control = SelectedHeatBalanceInputState {
            any_ctf: true,
            construction_order: vec![ConstructionId(0)],
            construction_usage: vec![Default::default()],
            ..Default::default()
        };
        let globals =
            crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals {
                num_spaces: 1,
                num_of_zones: 1,
                ..Default::default()
            };
        let environment = crate::weather::day::WeatherEnvironmentState::default();
        let mut owners = SurfaceGetterOwners {
            model: &model.typed,
            input: &mut input,
            shortcut: &mut shortcut,
            zones: &mut zones,
            control: &mut control,
            globals: &globals,
            environment: &environment,
        };
        let mut errors = false;
        // Declared per-row entry, then the genuine GetVertices implementation.
        // Reuse these actual borrowed owners; this does not certify the separate
        // complete raw getter/canonical binding or a multi-zone admission.
        {
            let saved_zones = owners.zones.clone();
            let saved_control = owners.control.clone();
            let saved_shortcut = owners.shortcut.clone();
            let points = [[0.0, -0.0, 2.0], [0.0, 0.0, 0.0],
                [3.0, 0.0, 0.0], [3.0, -0.0, 2.0]];
            // Fixed expected permutations from Original9248-9276. In particular,
            // a triangle's UpperRightCorner preserves its starting vertex.
            for (sides, orders) in [(3, [[0,1,2,0], [2,0,1,0], [1,2,0,0], [0,1,2,0]]),
                (4, [[0,1,2,3], [3,0,1,2], [2,3,0,1], [1,2,3,0]])] {
                for (corner, order) in orders.into_iter().enumerate() {
                    for ccw in [true, false] {
                        for (world, appendix) in [(true, 0.0), (true, 7.5), (false, 7.5)] {
                            owners.control.building_azimuth = -22.5;
                            owners.control.building_rotation_appendix_g = appendix;
                            let zone = &mut owners.zones.zones.as_mut().ok_or("declared owning Zone")?[0];
                            zone.relative_north = 37.5;
                            zone.origin_m = [12.0, -7.0, 2.25];
                            owners.shortcut.numeric_args = vec![0.0; 2];
                            owners.shortcut.numeric_args.extend(points[..sides].iter().flatten().copied());
                            let mut vertices = SurfaceGeometryInputState {
                                corner: i32::try_from(corner + 1)?, counterclockwise: ccw,
                                world_coordinates: world, pending_vertices: Some(0),
                                surface_tmp: Some(vec![SurfaceTmpInput {
                                    name: "declared transformed wall".into(), class: SurfaceClass::Wall,
                                    zone: 1, sides: i32::try_from(sides)?,
                                    vertex: Some(vec![None; sides]), ..Default::default()
                                }]), ..Default::default()
                            };
                            vertices.initialize_coordinate_owners(&owners)?;
                            vertices.get_world_vertices(0, sides, &mut owners, &mut context.backend)?;
                            assert_eq!(vertices.pending_vertices, None);
                            let row = &vertices.surface_tmp.as_ref().ok_or("actual transformed row")?[0];
                            let actual = row.vertex.as_ref().ok_or("actual transformed vertices")?;
                            for (position, point) in actual.iter().enumerate() {
                                let source = order[position];
                                let source = if ccw || source == 0 { source } else { sides - source };
                                let [x, y, z] = points[source];
                                let rad = std::f64::consts::PI / 180.0;
                                let expected = if world {
                                    let angle = -appendix * rad;
                                    [x * angle.cos() - y * angle.sin(),
                                        x * angle.sin() + y * angle.cos(), z]
                                } else {
                                    let angle = -37.5_f64 * rad;
                                    let xb = x * angle.cos() - y * angle.sin() + 12.0;
                                    let yb = x * angle.sin() + y * angle.cos() - 7.0;
                                    let angle = -(-22.5 + appendix) * rad;
                                    [xb * angle.cos() - yb * angle.sin(),
                                        xb * angle.sin() + yb * angle.cos(), z + 2.25]
                                };
                                assert_eq!(point.ok_or("actual written point")?.map(f64::to_bits),
                                    expected.map(f64::to_bits), "sides{sides} corner{corner} ccw{ccw} world{world}");
                            }
                            assert!((row.gross_area - if sides == 3 { 3.0 } else { 6.0 }).abs() < 1.0e-12);
                            assert!(row.vertex_progress.normal_area.is_some());
                            assert!(row.vertex_progress.properties.ok_or("actual GetVertices properties")?.is_finite());
                        }
                    }
                }
            }
            *owners.zones = saved_zones;
            *owners.control = saved_control;
            *owners.shortcut = saved_shortcut;
        }

        // Authored checkpoint only, not a GetVertices/World producer certificate.
        // This cube intentionally has inward floor/roof winding. Do not bypass
        // the real warning/reversal IO by calling GetVertices on these rows.
        for (index, row) in raw
            .surface_tmp
            .as_mut()
            .ok_or("authored checkpoint rows")?
            .iter_mut()
            .enumerate()
        {
            let points = row
                .vertex
                .as_ref()
                .ok_or("authored checkpoint vertices")?
                .iter()
                .map(|vertex| {
                    let [x_m, y_m, z_m] = vertex.ok_or("authored written checkpoint vertex")?;
                    Ok(ep_model::Point3 { x_m, y_m, z_m })
                })
                .collect::<Result<Vec<_>, &str>>()?;
            row.base_surface = i32::try_from(index + 1)?;
            raw.coordinate.max_vertices_per_surface =
                raw.coordinate.max_vertices_per_surface.max(points.len());
            let normal = crate::geometry::source_normal_area(&points);
            let orientation =
                crate::geometry::orientation_from_normal(&points, normal.newell_normal);
            row.vertex_progress.normal_area = Some(normal);
            row.gross_area = normal.gross_area_m2;
            row.area = row.gross_area;
            row.net_area_shadow_calc = row.area;
            row.vertex_progress.orientation_return = Some(orientation);
            row.vertex_progress.properties = Some(crate::geometry::finish_get_vertices_properties(
                normal,
                orientation,
            ));
        }
        // A non-default layer bound proves AllocateSurfaceWindows uses its operand.
        raw.initialize_coordinate_owners(&owners)?;
        raw.copy_final_opaque_surfaces(&mut owners, &mut context.backend, &mut errors, 3, None, None)?;
        raw.build_opaque_surface_lists(&mut owners, &mut context.backend, errors, None)?;
        // Authored child diagnostic inputs, not a completed Native material or
        // geometry certificate. Real IRT property/getter owners are tested in
        // selected_material_data; these rows isolate warning prefix custody.
        for mode in 0..4 {
            let mut warning_raw = raw.clone();
            warning_raw.surface_lists = Default::default();
            let mut warning_zones = owners.zones.clone();
            for zone in warning_zones.zones.as_mut().ok_or("warning Zone owner")? {
                zone.ht_surface_list.clear(); zone.iz_surface_list.clear();
                zone.ht_non_window_surface_list.clear(); zone.ht_window_surface_list.clear();
            }
            let mut warning_control = owners.control.clone();
            warning_control.construction_usage.get_mut(0).ok_or("warning construction owner")?.type_is_irt = true;
            warning_control.construction_usage[0].outside_roughness = None;
            let mut warning_input = owners.input.clone();
            let mut warning_shortcut = owners.shortcut.clone();
            let mut warning_globals = owners.globals.clone();
            warning_globals.kickoff_simulation = false;
            warning_globals.doing_sizing = mode == 1;
            warning_globals.display_extra_warnings = mode == 1;
            let warning_clock = crate::weather::day::WeatherGlobalState { warmup_flag: true, ..Default::default() };
            let developer = true;
            let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
                kickoff: &warning_globals.kickoff_simulation, developer: &developer,
                warmup: &warning_clock.warmup_flag, doing_sizing: &warning_globals.doing_sizing,
            };
            let mut warning_owners = SurfaceGetterOwners {
                model: owners.model, environment: owners.environment,
                input: &mut warning_input, shortcut: &mut warning_shortcut,
                zones: &mut warning_zones, control: &mut warning_control, globals: &warning_globals,
            };
            // The legacy backend phase cells remain false. Different live phase
            // cells must control the actual Source warning counters instead.
            let mut warning_state = ctf_ordinary_output_test_state();
            let (mut warning_text, mut warning_console) = (Vec::new(), Vec::new());
            let mut broken = OrdinaryFailingEio;
            let error_stream: &mut dyn std::io::Write = if mode == 2 { &mut broken } else { &mut warning_text };
            let mut warning_backend = SourceErrorBackend {
                state: &mut warning_state, error_stream: Some(error_stream), console: &mut warning_console,
                out_unit1: None, out_unit2: None, f_message: None, message_callback: None,
                error_callback: None, sqlite: None,
            };
            let result = warning_raw.build_opaque_surface_lists(
                &mut warning_owners, &mut warning_backend, false, (mode != 3).then_some(&flags),
            );
            let warnings = if mode == 1 { i32::try_from(total)? } else { i32::from(mode != 3) };
            assert_eq!(warning_backend.state.tracking.total_warning_errors, warnings);
            assert_eq!(warning_backend.state.tracking.total_warning_errors_during_sizing, if mode == 1 { warnings } else { 0 });
            assert_eq!(warning_backend.state.tracking.total_warning_errors_during_warmup, if mode == 1 { 0 } else { warnings });
            assert_eq!(warning_raw.surface_lists.all_ht.len(), total, "list effects precede every warning/availability failure");
            if mode >= 2 {
                assert!(result.is_err());
                assert!(warning_raw.surface_lists.filters.iter().all(Vec::is_empty), "filter writes follow successful warning/Continue only");
            } else {
                result?;
                assert!(warning_raw.surface_lists.filters.iter().any(|rows| !rows.is_empty()));
                let text = std::str::from_utf8(&warning_text)?;
                assert_eq!(text.matches(" ** Warning **").count(), usize::try_from(warnings)?);
                assert_eq!(text.contains("For explicit details on each use"), mode == 0);
            }
        }
        // Authored diagnostic child checkpoints, not a Native owner endpoint or
        // weather simulation. The real backend and real Surface methods retain
        // their prefix effects when a requested output record cannot be written.
        struct ExposureRecordWriter {
            bytes: Vec<u8>,
            completed_lines: usize,
            allowed_lines: usize,
        }
        impl std::io::Write for ExposureRecordWriter {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if self.completed_lines >= self.allowed_lines {
                    return Err(std::io::Error::other("authored exposure record failure"));
                }
                self.bytes.extend_from_slice(bytes);
                self.completed_lines += bytes.iter().filter(|&&byte| byte == b'\n').count();
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
        }
        // Real GetHTSurfaceData row parsing: a non-external Sun request warns
        // before reading Wind or allocating/returning the vertex prefix.
        for (sizing, allowed_lines, have_flags) in [
            (false, usize::MAX, true), (true, usize::MAX, true),
            (false, 0, true), (false, 1, true), (false, usize::MAX, false),
        ] {
            use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue as V};
            use crate::heat_balance::selected_input_processor::{SourceInputFamily, SourceObjectDefinition, SourceObjectShape};
            use crate::heat_balance::selected_heat_balance_input::surface::{OpaqueSurfaceInput, SelectedSurfaceInput, SurfaceRowPrefix};
            let surface_name = "AUTHORED SUN WALL";
            let object = ProjectInputObject {
                object_name: surface_name.into(), idf_max_fields: None,
                fields: std::collections::BTreeMap::from([
                    ("surface_type".into(), V::String("Wall".into())),
                    ("construction_name".into(), V::String("Wall".into())),
                    ("zone_name".into(), V::String("AUTHORED SUN ZONE".into())),
                    ("outside_boundary_condition".into(), V::String("Surface".into())),
                    ("outside_boundary_condition_object".into(), V::String("AUTHORED OTHER WALL".into())),
                    ("sun_exposure".into(), V::String("SunExposed".into())),
                    ("wind_exposure".into(), V::String("WindExposed".into())),
                    ("view_factor_to_ground".into(), V::Number(0.5)),
                    ("number_of_vertices".into(), V::Number(4.0)),
                ]),
            };
            let row_input = crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInput {
                source_is_epjson: true,
                surface: SelectedSurfaceInput {
                    geometry_rules: Vec::new(),
                    building_surfaces: vec![OpaqueSurfaceInput {
                        object, idf_max_extensible_fields: None,
                        vertices: vec![[0.0,0.0,1.0],[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,0.0,1.0]],
                    }],
                }, objects: Default::default(),
            };
            let mut row_ip = SelectedInputProcessor::from_admitted_input(
                vec![SourceObjectDefinition {
                    object_type: "BuildingSurface:Detailed".into(), alpha_fields: 9, numeric_fields: 2,
                    alpha_extensions: 0, numeric_extensions: 3, extension: Some("vertices".into()),
                }],
                std::collections::BTreeMap::from([("BuildingSurface:Detailed".into(), SourceInputFamily::Objects(vec![SourceObjectShape {
                    object_name: surface_name.into(), field_sizes: std::collections::BTreeMap::from([("vertices".into(),4)]),
                }]))]),
            )?;
            let mut row_shortcut = SourceInputProcessorShortCut {
                alpha_args: vec![String::new();9], numeric_args: vec![0.0;14],
                alpha_field_names: vec![String::new();9], numeric_field_names: vec![String::new();14],
                alpha_field_blanks: vec![true;9], numeric_field_blanks: vec![true;14],
                ..Default::default()
            };
            let mut row_zones = owners.zones.clone();
            row_zones.zones.as_mut().ok_or("authored Sun Zone")?[0].name = "AUTHORED SUN ZONE".into();
            let mut row_control = owners.control.clone();
            row_control.construction_usage[0].construction_id = Some(row_control.construction_order[0]);
            let mut row_globals = owners.globals.clone();
            row_globals.kickoff_simulation = false; row_globals.doing_sizing = sizing;
            let row_clock = crate::weather::day::WeatherGlobalState { warmup_flag: true, ..Default::default() };
            let developer = true;
            let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
                kickoff: &row_globals.kickoff_simulation, developer: &developer,
                warmup: &row_clock.warmup_flag, doing_sizing: &row_globals.doing_sizing,
            };
            let mut row_owners = SurfaceGetterOwners {
                model: owners.model, environment: owners.environment, input: &mut row_ip, shortcut: &mut row_shortcut,
                zones: &mut row_zones, control: &mut row_control, globals: &row_globals,
            };
            let mut row_state = SurfaceGeometryInputState {
                entered_once: true, total_surfaces: 1, surface_tmp: Some(vec![SurfaceTmpInput::default()]),
                ..Default::default()
            };
            let mut backend_state = ctf_ordinary_output_test_state();
            backend_state.output_error_header = false;
            let (mut writer, mut console) = (ExposureRecordWriter { bytes: Vec::new(), completed_lines: 0, allowed_lines }, Vec::new());
            let mut backend = SourceErrorBackend {
                state: &mut backend_state, error_stream: Some(&mut writer), console: &mut console,
                out_unit1: None, out_unit2: None, f_message: None, message_callback: None,
                error_callback: None, sqlite: None,
            };
            let mut errors = false;
            let result = row_state.read_opaque_row(&row_input, 0, &mut row_owners, &mut backend, &mut errors, have_flags.then_some(&flags));
            let success = have_flags && allowed_lines >= 2;
            assert!(!errors);
            assert_eq!((row_state.next_object,row_state.surfaces_read),(1,1));
            let actual = &row_state.surface_tmp.as_ref().ok_or("retained actual Sun row")?[0];
            assert_eq!((actual.name.as_str(),actual.construction,actual.zone),(surface_name,1,1));
            assert_eq!(actual.exterior_boundary,-999);
            assert!(!actual.exterior_solar, "the Source Sun branch retains its constructor false");
            assert_eq!(actual.exterior_wind,success,"Wind is read only after the successful warning and Continue");
            assert_eq!(row_state.pending_vertices,success.then_some(0));
            assert_eq!(backend.state.tracking.total_warning_errors,i32::from(have_flags));
            assert_eq!(backend.state.tracking.total_warning_errors_during_warmup,i32::from(have_flags && !sizing));
            assert_eq!(backend.state.tracking.total_warning_errors_during_sizing,i32::from(have_flags && sizing));
            if success { assert!(matches!(result?,SurfaceRowPrefix::BeforeVertices { slot:0,sides:4 })); }
            else { assert!(result.is_err()); }
            assert_eq!(writer.completed_lines,if have_flags { allowed_lines.min(2) } else { 0 });
            if success {
                assert_eq!(std::str::from_utf8(&writer.bytes)?,
                    "   ** Warning ** BuildingSurface:Detailed=\"AUTHORED SUN WALL\", sun_exposure=\"SUNEXPOSED\".\n   **   ~~~   ** ..This surface is not exposed to External Environment.  Sun exposure has no effect.\n");
            }
        }
        // Reconciliation starts before typed binding. These natural Source
        // indices1/2 are authored temporary rows, without fabricated typed IDs.
        // The real area/orientation helpers prepare the reversed quad operands.
        for (sun, wind_mask, allowed_lines, have_flags, sizing) in [
            (false,0,usize::MAX,false,false), (false,1,usize::MAX,true,false),
            (false,2,usize::MAX,true,true), (false,3,usize::MAX,true,false),
            (true,0,usize::MAX,true,false), (true,3,usize::MAX,true,false),
            (false,3,0,true,false), (false,3,1,true,false), (false,3,2,true,false),
            (true,3,3,true,false), (false,3,usize::MAX,false,false),
        ] {
            let template = raw.final_surfaces.as_ref().ok_or("authored pair rows")?.iter()
                .find(|row| row.row.class==SurfaceClass::Wall).ok_or("authored pair Wall")?;
            let mut pair_rows = vec![template.clone(),template.clone()];
            for (i,final_row) in pair_rows.iter_mut().enumerate() {
                final_row.typed_id = None;
                let row = &mut final_row.row;
                row.name = if i==0 { "AUTHORED LEFT WALL" } else { "AUTHORED RIGHT WALL" }.into();
                row.zone_name = if i==0 { "AUTHORED LEFT ZONE" } else { "AUTHORED RIGHT ZONE" }.into();
                row.zone = i32::try_from(i+1)?; row.space = row.zone; row.base_surface = row.zone;
                row.exterior_boundary = -999;
                row.exterior_boundary_name = if i==0 { "AUTHORED RIGHT WALL" } else { "AUTHORED LEFT WALL" }.into();
                row.exterior_solar = sun; row.exterior_wind = wind_mask & (1<<i) != 0;
                if i==1 { row.vertex.as_mut().ok_or("authored reverse vertices")?.reverse(); }
                let points = row.vertex.as_ref().ok_or("authored pair vertices")?.iter().map(|v| {
                    let [x_m,y_m,z_m] = v.ok_or("authored written pair vertex")?;
                    Ok(ep_model::Point3 { x_m,y_m,z_m })
                }).collect::<Result<Vec<_>,&str>>()?;
                let normal = crate::geometry::source_normal_area(&points);
                let orientation = crate::geometry::orientation_from_normal(&points,normal.newell_normal);
                row.area = normal.gross_area_m2; row.gross_area = normal.gross_area_m2;
                row.vertex_progress.normal_area = Some(normal);
                row.vertex_progress.orientation_return = Some(orientation);
                row.vertex_progress.properties = Some(crate::geometry::finish_get_vertices_properties(normal,orientation));
            }
            let mut pair_zones = owners.zones.clone();
            let first = pair_zones.zones.as_ref().ok_or("authored pair Zones")?[0].clone();
            pair_zones.zones = Some(vec![first.clone(),first]);
            for (i,z) in pair_zones.zones.as_mut().ok_or("authored pair Zones")?.iter_mut().enumerate() {
                z.name = if i==0 { "AUTHORED LEFT ZONE" } else { "AUTHORED RIGHT ZONE" }.into();
                z.typed_zone_id = None; z.multiplier = 1; z.list_multiplier = 1;
                z.space_indexes = vec![i32::try_from(i+1)?]; z.num_spaces = 1;
            }
            let first = pair_zones.spaces.as_ref().ok_or("authored pair Spaces")?[0].clone();
            pair_zones.spaces = Some(vec![first.clone(),first]);
            for (i,sp) in pair_zones.spaces.as_mut().ok_or("authored pair Spaces")?.iter_mut().enumerate() {
                sp.name = if i==0 { "AUTHORED LEFT ZONE" } else { "AUTHORED RIGHT ZONE" }.into();
                sp.zone_num = i32::try_from(i+1)?; sp.typed_space_id = None;
            }
            let mut pair_control = owners.control.clone();
            pair_control.construction_usage[0].total_layers = 1;
            pair_control.construction_usage[0].layer_points = [0;11];
            pair_control.construction_usage[0].layer_points[0] = 1;
            let mut pair_globals = owners.globals.clone();
            pair_globals.num_of_zones = 2; pair_globals.num_spaces = 2;
            pair_globals.kickoff_simulation = false; pair_globals.doing_sizing = sizing;
            let pair_clock = crate::weather::day::WeatherGlobalState { warmup_flag: true, ..Default::default() };
            let developer = true;
            let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
                kickoff: &pair_globals.kickoff_simulation, developer: &developer,
                warmup: &pair_clock.warmup_flag, doing_sizing: &pair_globals.doing_sizing,
            };
            let mut pair_ip = owners.input.clone(); let mut pair_shortcut = owners.shortcut.clone();
            let mut pair_owners = SurfaceGetterOwners {
                model: owners.model, environment: owners.environment, input: &mut pair_ip, shortcut: &mut pair_shortcut,
                zones: &mut pair_zones, control: &mut pair_control, globals: &pair_globals,
            };
            let mut pair = raw.clone(); pair.moved_surfaces = 2; pair.total_surfaces = 2; pair.final_surfaces = Some(pair_rows);
            let mut backend_state = ctf_ordinary_output_test_state(); backend_state.output_error_header = false;
            let (mut writer,mut console) = (ExposureRecordWriter { bytes:Vec::new(),completed_lines:0,allowed_lines },Vec::new());
            let mut backend = SourceErrorBackend {
                state:&mut backend_state,error_stream:Some(&mut writer),console:&mut console,
                out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
            };
            let result = pair.reconcile_opaque_surface_boundaries(&mut pair_owners,&mut backend,have_flags.then_some(&flags),None);
            let needed = usize::from(sun)*3 + usize::from(wind_mask!=0)*3;
            let success = needed==0 || (have_flags && allowed_lines>=needed);
            let warnings = if !have_flags { 0 } else { usize::from(sun) + usize::from(wind_mask!=0 && (!sun || allowed_lines>=3)) };
            assert_eq!(backend.state.tracking.total_warning_errors,i32::try_from(warnings)?);
            assert_eq!(backend.state.tracking.total_warning_errors_during_warmup,if sizing { 0 } else { i32::try_from(warnings)? });
            assert_eq!(backend.state.tracking.total_warning_errors_during_sizing,if sizing { i32::try_from(warnings)? } else { 0 });
            let rows = pair.final_surfaces.as_ref().ok_or("retained real pair rows")?;
            assert_eq!(rows[0].row.exterior_boundary,2,"current Found write precedes warning/availability failures");
            assert_eq!(rows[1].row.exterior_boundary,if success { 1 } else { -999 },"reciprocal Found write follows all warning effects");
            for (i,row) in rows.iter().enumerate() {
                assert_eq!(row.row.exterior_solar,sun && !(have_flags && allowed_lines>=3));
                assert_eq!(row.row.exterior_wind,(wind_mask & (1<<i)!=0) && !success);
            }
            if success { result?; } else { assert!(result.is_err()); }
            assert_eq!(writer.completed_lines,if have_flags { allowed_lines.min(needed) } else { 0 });
            if success && needed>0 {
                let text = std::str::from_utf8(&writer.bytes)?;
                assert_eq!(text.matches(" ** Warning **").count(),warnings,"the resolved opposing row does not warn again");
                assert_eq!(text.matches("Surface=AUTHORED LEFT WALL, Zone=AUTHORED LEFT ZONE").count(),warnings);
                assert_eq!(text.matches("Surface=AUTHORED RIGHT WALL, Zone=AUTHORED RIGHT ZONE").count(),warnings);
                assert_eq!(text.contains("Interzone surfaces cannot be \"SunExposed\" -- removing SunExposed"),sun);
                assert_eq!(text.contains("Interzone surfaces cannot be \"WindExposed\" -- removing WindExposed"),wind_mask!=0);
            }
        }
        // Authored reached material checkpoints feed the REAL Construction
        // getter below. Its real LayerPoint/MaterialMap association, not a
        // typed-ordinal cast or thermal-property comparison, then reaches the
        // actual reconciliation child. No completed Native/CTF endpoint is claimed.
        let stack_cases: &[(&[i32], &[i32], bool, bool)] = &[
            (&[1], &[1], false, true),
            (&[1,2], &[2,1], false, true),
            (&[1,2,3], &[3,2,1], false, true),
            (&[1,2,3,1,2,3,1,2,3,1], &[1,3,2,1,3,2,1,3,2,1], false, true),
            (&[1,2,1], &[1,2,1], true, true),
            (&[1,2], &[1,2], false, false),
            (&[1,2], &[2,1], true, false),
            (&[1,2], &[3,2,1], false, false),
            (&[1,2], &[2,3], false, false),
        ];
        for &(left, right, same_constructor, succeeds) in stack_cases {
            use crate::heat_balance::selected_heat_balance_input::ConstructionMaterialOwners;
            use crate::heat_balance::selected_material_data::{SelectedMaterialData, SourceMaterialGroup};
            use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
            use crate::heat_balance::selected_input_processor::{SourceObjectDefinition, SourceObjectShape, SourceInputFamily};
            let names = ["STACK MAT ONE", "STACK MAT TWO", "STACK MAT THREE"];
            let mut stack_model = owners.model.clone();
            let original = stack_model.materials[0].clone();
            stack_model.materials = names.iter().enumerate().map(|(index,name)| {
                let mut material = original.clone();
                material.id = MaterialId(u32::try_from(index)?);
                material.name = NormalizedName::new(name);
                Ok(material)
            }).collect::<Result<Vec<_>,std::num::TryFromIntError>>()?;
            stack_model.material_names=Default::default();
            for material in &stack_model.materials {
                assert!(stack_model.material_names.insert(&material.name.0,material.id).is_none());
            }
            assert!(matches!(original.definition,ep_model::MaterialDefinition::NoMass(_)),
                "the authored reached Material checkpoint is genuinely resistance-only");
            stack_model.ctf_report_names.materials = stack_model.materials.iter()
                .zip(names).map(|(material,name)|(material.id,name.into())).collect();
            let original = stack_model.constructions[0].clone();
            stack_model.constructions.clear();
            for (index,(name,points)) in [("STACK LEFT",left),("STACK RIGHT",right)].into_iter().enumerate() {
                let mut construction = original.clone();
                construction.id = ConstructionId(u32::try_from(index)?);
                construction.name = NormalizedName::new(name);
                construction.layers = points.iter().map(|point| u32::try_from(point-1).map(MaterialId))
                    .collect::<Result<Vec<_>,_>>()?;
                construction.outside_layer=construction.layers.first().copied();
                stack_model.constructions.push(construction);
            }
            stack_model.construction_names=Default::default();
            for construction in &stack_model.constructions {
                assert!(stack_model.construction_names.insert(&construction.name.0,construction.id).is_none());
            }
            stack_model.ctf_report_names.constructions = vec![
                (ConstructionId(0),"STACK LEFT".into()),(ConstructionId(1),"STACK RIGHT".into())];
            let mut material_checkpoint = SelectedMaterialData::default();
            for (index,material) in stack_model.materials.iter().enumerate() {
                material_checkpoint.materials.push(material.id);
                material_checkpoint.nominal_r.push(material.thermal_resistance().ok_or("authored reached material resistance")?);
                material_checkpoint.material_map.insert(names[index].into(),i32::try_from(index+1)?);
                material_checkpoint.material_groups.insert(material.id,SourceMaterialGroup::Regular);
                material_checkpoint.resistance_only.insert(material.id,true);
            }
            let mut objects = Vec::new();
            for (name,points) in [("STACK LEFT",left),("STACK RIGHT",right)] {
                let mut fields = std::collections::BTreeMap::new();
                for (index,point) in points.iter().enumerate() {
                    let field = if index==0 { "outside_layer".into() } else { format!("layer_{}",index+1) };
                    fields.insert(field,ProjectInputValue::String(names[usize::try_from(point-1)?].into()));
                }
                objects.push(ProjectInputObject { object_name:name.into(),fields,idf_max_fields:None });
            }
            let construction_raw = crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInput {
                source_is_epjson:true,objects:std::collections::BTreeMap::from([("Construction".into(),objects)]),
                ..Default::default()
            };
            let mut stack_input = SelectedInputProcessor::from_admitted_input(
                vec![SourceObjectDefinition { object_type:"Construction".into(),alpha_fields:11,numeric_fields:0,
                    alpha_extensions:0,numeric_extensions:0,extension:None }],
                std::collections::BTreeMap::from([("Construction".into(),SourceInputFamily::Objects(
                    ["STACK LEFT","STACK RIGHT"].map(|name|SourceObjectShape {
                        object_name:name.into(),field_sizes:Default::default()
                    }).into()))]),
            )?;
            let mut stack_shortcut = SourceInputProcessorShortCut::default();
            stack_shortcut.allocate_from_input(&stack_input)?;
            let mut bounds = crate::heat_balance::selected_simulation_startup::SelectedConstructionBounds::default();
            bounds.set_preconstruction_input_parameters(&stack_input)?;
            let mut stack_control = SelectedHeatBalanceInputState::default();
            let (mut stack_errors,mut stack_console) = (Vec::new(),Vec::new());
            let mut stack_state = ctf_ordinary_output_test_state(); stack_state.output_error_header=false;
            let mut backend = SourceErrorBackend {
                state:&mut stack_state,error_stream:Some(&mut stack_errors),console:&mut stack_console,
                out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
            };
            let mut errors = false;
            stack_control.read_regular_construction_arguments(
                &construction_raw,ConstructionMaterialOwners { model:&stack_model,data:&material_checkpoint,bounds:&bounds },
                &mut stack_input,&mut stack_shortcut,&mut String::new(),&mut backend,&mut errors,
            )?;
            assert!(!errors);
            for (name,expected) in [("STACK LEFT",left),("STACK RIGHT",right)] {
                let row = stack_control.construction_usage.iter().find(|row|row.name==name)
                    .ok_or("actual constructed stack")?;
                assert_eq!(row.total_layers, i32::try_from(expected.len())?);
                assert_eq!(&row.layer_points[..expected.len()],expected);
            }
            let before_warnings = backend.state.tracking.total_warning_errors;
            let template = raw.final_surfaces.as_ref().ok_or("authored stack pair")?.iter()
                .find(|row|row.row.class==SurfaceClass::Wall).ok_or("authored stack Wall")?;
            let mut rows = vec![template.clone(),template.clone()];
            for (index,final_row) in rows.iter_mut().enumerate() {
                final_row.typed_id=None;
                let row=&mut final_row.row;
                row.name=if index==0 { "STACK LEFT WALL" } else { "STACK RIGHT WALL" }.into();
                row.zone_name=if index==0 { "STACK LEFT ZONE" } else { "STACK RIGHT ZONE" }.into();
                row.zone=i32::try_from(index+1)?;row.space=row.zone;row.base_surface=row.zone;
                row.exterior_boundary = -999;
                row.exterior_boundary_name=if index==0 { "STACK RIGHT WALL" } else { "STACK LEFT WALL" }.into();
                let name=if index==0 || same_constructor { "STACK LEFT" } else { "STACK RIGHT" };
                row.construction=i32::try_from(stack_control.construction_usage.iter()
                    .position(|row|row.name==name).ok_or("actual stack construction number")?+1)?;
                row.exterior_solar=false;row.exterior_wind=false;
                if index==1 { row.vertex.as_mut().ok_or("authored stack reverse vertices")?.reverse(); }
                let points=row.vertex.as_ref().ok_or("authored stack vertices")?.iter().map(|vertex| {
                    let [x_m,y_m,z_m]=vertex.ok_or("authored written stack vertex")?;
                    Ok(ep_model::Point3 { x_m,y_m,z_m })
                }).collect::<Result<Vec<_>,&str>>()?;
                let normal=crate::geometry::source_normal_area(&points);
                let orientation=crate::geometry::orientation_from_normal(&points,normal.newell_normal);
                row.area=normal.gross_area_m2;row.gross_area=normal.gross_area_m2;
                row.vertex_progress.normal_area=Some(normal);
                row.vertex_progress.orientation_return=Some(orientation);
                row.vertex_progress.properties=Some(crate::geometry::finish_get_vertices_properties(normal,orientation));
            }
            let mut zones=owners.zones.clone();
            let first=zones.zones.as_ref().ok_or("authored stack Zones")?[0].clone();
            zones.zones=Some(vec![first.clone(),first]);
            for (index,row) in zones.zones.as_mut().ok_or("authored stack Zones")?.iter_mut().enumerate() {
                row.name=if index==0 { "STACK LEFT ZONE" } else { "STACK RIGHT ZONE" }.into();
                row.typed_zone_id=None;row.multiplier=1;row.list_multiplier=1;
                row.num_spaces=1;row.space_indexes=vec![i32::try_from(index+1)?];
            }
            let first=zones.spaces.as_ref().ok_or("authored stack Spaces")?[0].clone();
            zones.spaces=Some(vec![first.clone(),first]);
            for (index,row) in zones.spaces.as_mut().ok_or("authored stack Spaces")?.iter_mut().enumerate() {
                row.name=if index==0 { "STACK LEFT ZONE" } else { "STACK RIGHT ZONE" }.into();
                row.zone_num=i32::try_from(index+1)?;row.typed_space_id=None;
            }
            let mut globals=owners.globals.clone();globals.num_of_zones=2;globals.num_spaces=2;
            let mut pair_owners=SurfaceGetterOwners { model:&stack_model,environment:owners.environment,
                input:&mut stack_input,shortcut:&mut stack_shortcut,zones:&mut zones,control:&mut stack_control,globals:&globals };
            let mut pair=raw.clone();pair.moved_surfaces=2;pair.total_surfaces=2;pair.final_surfaces=Some(rows);
            let nominal_checkpoint=pair.clone();
            let result=pair.reconcile_opaque_surface_boundaries(&mut pair_owners,&mut backend,None,None);
            if succeeds { result?; } else { assert!(result.is_err()); }
            let rows=pair.final_surfaces.as_ref().ok_or("retained actual stack reconciliation rows")?;
            assert_eq!(rows[0].row.exterior_boundary,2);
            assert_eq!(rows[1].row.exterior_boundary,if succeeds { 1 } else { -999 });
            assert_eq!(backend.state.tracking.total_warning_errors,before_warnings,
                "mismatches remain an explicit boundary before unported Native diagnostics");
            // Authored ACTUAL control.nominal_u checkpoints, not full Native
            // inputs or getter-produced numerical observations. The surrounding
            // real Constructor getter supplies all names/IDs/LayerPoints. These
            // variants ensure the child reads this owner rather than deriving U
            // from the unchanged typed materials or the nominal-R array.
            let u_probes: Vec<(Option<Vec<f64>>,bool)> = if left.len()!=right.len() {
                vec![
                    (None,false),(Some(vec![]),false),(Some(vec![0.5]),false),
                    (Some(vec![0.5,0.5]),true),(Some(vec![0.0,0.001]),true),
                    (Some(vec![0.0,f64::from_bits(0.001_f64.to_bits()+1)]),false),
                    (Some(vec![f64::NAN,0.5]),false),(Some(vec![0.5,f64::INFINITY]),false),
                ]
            } else if succeeds { vec![(None,true)] } else { Vec::new() };
            for (nominal_u,expected) in u_probes {
                let mut control=pair_owners.control.clone();control.nominal_u=nominal_u;
                let mut input=pair_owners.input.clone();let mut shortcut=pair_owners.shortcut.clone();
                let mut zones=pair_owners.zones.clone();
                let mut probe_owners=SurfaceGetterOwners { model:pair_owners.model,environment:pair_owners.environment,
                    input:&mut input,shortcut:&mut shortcut,zones:&mut zones,control:&mut control,globals:pair_owners.globals };
                let mut probe=nominal_checkpoint.clone();
                let before=backend.state.tracking.clone();
                let result=probe.reconcile_opaque_surface_boundaries(&mut probe_owners,&mut backend,None,None);
                if expected { result?; } else { assert!(result.is_err()); }
                let rows=probe.final_surfaces.as_ref().ok_or("actual U-gate child prefix")?;
                assert_eq!(rows[0].row.exterior_boundary,2);
                assert_eq!(rows[1].row.exterior_boundary,if expected { 1 } else { -999 });
                assert!(!probe.surface_error,"unsupported invalid-U branches are not fabricated Native Severe returns");
                assert_eq!(backend.state.tracking,before,"accepted unequal counts emit no diagnostic");
            }

            if left==[1,2] && right==[2,1] && !same_constructor {
                // Unit87 authored class-specific child checkpoints retain real
                // getter Construction IDs/LayerPoints, not full Native endpoints.
                for (first_class,second_class,wind,unavailable_azimuth) in [
                    (SurfaceClass::Roof,SurfaceClass::Floor,false,false),
                    (SurfaceClass::Roof,SurfaceClass::Floor,true,false),
                    (SurfaceClass::Floor,SurfaceClass::Roof,true,false),
                    (SurfaceClass::Roof,SurfaceClass::Floor,false,true),
                ] {
                    let up=[[0.0,3.0,2.5],[0.0,0.0,2.5],[4.0,0.0,2.5],[4.0,3.0,2.5]];
                    let mut pair=nominal_checkpoint.clone();
                    let rows=pair.final_surfaces.as_mut().ok_or("horizontal pair checkpoint")?;
                    for (index,row) in rows.iter_mut().enumerate() {
                        row.original_class=if index==0 { first_class } else { second_class };
                        row.row.class=row.original_class;row.row.exterior_wind=wind;
                        let downward=if index==0 { first_class==SurfaceClass::Floor } else { first_class!=SurfaceClass::Floor };
                        let mut vertices=up.to_vec();
                        if downward { vertices.reverse(); }
                        if index==1 { vertices.rotate_left(1); }
                        row.row.vertex=Some(vertices.iter().copied().map(Some).collect());
                        let points=vertices.iter().map(|&[x_m,y_m,z_m]|ep_model::Point3 { x_m,y_m,z_m }).collect::<Vec<_>>();
                        let normal=crate::geometry::source_normal_area(&points);
                        let orientation=crate::geometry::orientation_from_normal(&points,normal.newell_normal);
                        row.row.area=normal.gross_area_m2;row.row.gross_area=normal.gross_area_m2;
                        row.row.vertex_progress.normal_area=Some(normal);
                        row.row.vertex_progress.orientation_return=Some(orientation);
                        let mut properties=crate::geometry::finish_get_vertices_properties(normal,orientation);
                        if unavailable_azimuth {
                            // Authored unconsumed child operands prove Native2037's
                            // Roof/Floor no-read branch, not physical NaN outputs.
                            properties.azimuth_deg=f64::NAN;properties.sin_tilt=f64::NAN;
                        }
                        row.row.vertex_progress.properties=Some(properties);
                    }
                    let tilts=rows.iter().map(|row|row.row.vertex_progress.properties.as_ref()
                        .map(|p|p.tilt_deg).ok_or("horizontal actual tilt")).collect::<Result<Vec<_>,_>>()?;
                    assert_eq!(tilts,if first_class==SurfaceClass::Roof { vec![0.0,180.0] } else { vec![180.0,0.0] });
                    assert_eq!(rows[0].row.area,12.0);assert_eq!(rows[1].row.area,12.0);
                    let mut control=pair_owners.control.clone();control.nominal_u=None;
                    let mut input=pair_owners.input.clone();let mut shortcut=pair_owners.shortcut.clone();
                    let mut zones=pair_owners.zones.clone();let mut globals=pair_owners.globals.clone();
                    globals.display_extra_warnings=true;globals.kickoff_simulation=false;globals.doing_sizing=false;
                    let clock=crate::weather::day::WeatherGlobalState { warmup_flag:true,..Default::default() };
                    let developer=true;
                    let flags=crate::heat_balance::error_backend::SourceDiagnosticFlags {
                        kickoff:&globals.kickoff_simulation,developer:&developer,warmup:&clock.warmup_flag,doing_sizing:&globals.doing_sizing,
                    };
                    let mut class_owners=SurfaceGetterOwners { model:pair_owners.model,environment:pair_owners.environment,
                        input:&mut input,shortcut:&mut shortcut,zones:&mut zones,control:&mut control,globals:&globals };
                    let (mut writer,mut console)=(Vec::new(),Vec::new());
                    let mut state=ctf_ordinary_output_test_state();state.output_error_header=false;
                    let mut backend=SourceErrorBackend {
                        state:&mut state,error_stream:Some(&mut writer),console:&mut console,
                        out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
                    };
                    pair.reconcile_opaque_surface_boundaries(&mut class_owners,&mut backend,wind.then_some(&flags),None)?;
                    let rows=pair.final_surfaces.as_ref().ok_or("retained horizontal pair")?;
                    assert_eq!((rows[0].row.class,rows[1].row.class),(first_class,second_class));
                    assert_eq!((rows[0].row.exterior_boundary,rows[1].row.exterior_boundary),(2,1));
                    assert!(rows.iter().all(|row|!row.row.exterior_wind));
                    for row in rows {
                        assert!(!row.expected_wind_exposure(wind,true)?);
                        if unavailable_azimuth {
                            let properties=row.row.vertex_progress.properties.as_ref().ok_or("retained unavailable azimuth")?;
                            assert!(properties.azimuth_deg.is_nan() && properties.sin_tilt.is_nan());
                        }
                    }
                    assert_eq!(pair.iz_const_diff_message,Some(false));assert!(!pair.surface_error);
                    assert!(class_owners.control.nominal_u.is_none(),"exact reversed IDs require neither U nor groups");
                    assert!(class_owners.control.construction_usage.iter().all(|row|
                        !row.reverse_construction_num_layers_warning && !row.reverse_construction_layers_order_warning));
                    assert_eq!(backend.state.tracking.total_warning_errors,i32::from(wind));
                    assert_eq!(backend.state.tracking.total_warning_errors_during_warmup,i32::from(wind));
                    assert_eq!(backend.state.tracking.total_severe_errors,0);
                    let text=std::str::from_utf8(&writer)?;
                    assert_eq!(text.lines().count(),if wind { 3 } else { 0 });
                    assert_eq!(text.contains("Interzone surfaces cannot be \"WindExposed\""),wind);
                }
            }
            if left==[1] && right==[1] && !same_constructor {
                // Explicitly authored TypeIsIRT/latch entry checkpoints isolate
                // the IRT Wall-only fence; changing these flags is not a completed
                // IRT Material getter/Native endpoint. Real getter one-layer IDs
                // remain untouched. Each negative class relation is otherwise valid.
                for (first_class,second_class,irt_mask,latched,expected) in [
                    (SurfaceClass::Wall,SurfaceClass::Wall,3,false,true),
                    (SurfaceClass::Roof,SurfaceClass::Floor,1,false,false),
                    (SurfaceClass::Roof,SurfaceClass::Floor,2,false,false),
                    (SurfaceClass::Floor,SurfaceClass::Roof,3,false,false),
                    (SurfaceClass::Roof,SurfaceClass::Floor,1,true,false),
                    (SurfaceClass::Floor,SurfaceClass::Roof,2,true,false),
                ] {
                    let mut pair=nominal_checkpoint.clone();
                    for (index,row) in pair.final_surfaces.as_mut().ok_or("IRT class checkpoint")?.iter_mut().enumerate() {
                        row.original_class=if index==0 { first_class } else { second_class };
                        row.row.class=row.original_class;row.row.exterior_wind = !expected;
                    }
                    let mut control=pair_owners.control.clone();control.nominal_u=None;
                    for (index,row) in control.construction_usage.iter_mut().enumerate() {
                        row.type_is_irt=irt_mask & (1<<index)!=0;
                        if row.type_is_irt { row.outside_roughness=None; }
                        row.reverse_construction_layers_order_warning=latched;
                    }
                    let mut input=pair_owners.input.clone();let mut shortcut=pair_owners.shortcut.clone();
                    let mut zones=pair_owners.zones.clone();
                    let mut class_owners=SurfaceGetterOwners { model:pair_owners.model,environment:pair_owners.environment,
                        input:&mut input,shortcut:&mut shortcut,zones:&mut zones,control:&mut control,globals:pair_owners.globals };
                    let (mut writer,mut console)=(Vec::new(),Vec::new());
                    let mut state=ctf_ordinary_output_test_state();state.output_error_header=false;
                    let before=state.tracking.clone();
                    let mut backend=SourceErrorBackend {
                        state:&mut state,error_stream:Some(&mut writer),console:&mut console,
                        out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
                    };
                    let result=pair.reconcile_opaque_surface_boundaries(&mut class_owners,&mut backend,None,None);
                    if expected { result?; } else {
                        assert!(matches!(result,Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable(
                            "existing IRT paired Wall-only domain"))),"IRT class fence precedes persistent-latch skips");
                    }
                    let rows=pair.final_surfaces.as_ref().ok_or("IRT class-fence prefix")?;
                    assert_eq!((rows[0].row.exterior_boundary,rows[1].row.exterior_boundary),(2,if expected { 1 } else { -999 }));
                    for row in rows { assert_eq!(row.row.exterior_wind,!expected); }
                    if expected { assert!(!rows[0].expected_wind_exposure(false,true)?); }
                    else { assert!(rows[0].expected_wind_exposure(true,true).is_err()); }
                    assert_eq!(pair.iz_const_diff_message,Some(false));
                    assert!(!pair.surface_error,"bounded IRT class rejection does not fabricate a Native Severe");
                    assert_eq!(backend.state.tracking,before);
                    assert!(class_owners.control.construction_usage.iter().all(|row|
                        row.reverse_construction_layers_order_warning==latched && !row.reverse_construction_num_layers_warning));
                    assert!(writer.is_empty());
                }
            }
            if left==[1,2] && right==[1,2] && !same_constructor {
                // Unit86 authored child checkpoints: the real getter above supplies
                // Construction names, Source IDs and LayerPoint arrays. Warning
                // effects below are not represented as completed Native endpoints.
                let getter_u=pair_owners.control.nominal_u.as_ref().ok_or("actual getter NominalU")?;
                assert_eq!(getter_u.len(),2);
                assert_eq!(getter_u[0].to_bits(),getter_u[1].to_bits());
                assert!(pair_owners.control.construction_usage.iter().all(|row|
                    !row.reverse_construction_num_layers_warning && !row.reverse_construction_layers_order_warning));
                let expected_records=[
                    "   ** Warning ** GetSurfaceData: Construction STACK LEFT of interzone surface STACK LEFT WALL does not have the same materials in the reverse order as the construction STACK RIGHT of adjacent surface STACK RIGHT WALL\n",
                    "   **   ~~~   ** or the properties of the reversed layers are not correct due to differing layer front and back side values\n",
                    "   **   ~~~   ** ...but Nominal U values are similar, diff=[0.0000] ... simulation proceeds.\n",
                    "   **   ~~~   ** ...if the two zones are expected to have significantly different temperatures, the proper \"reverse\" construction should be created.\n",
                    "   **   ~~~   ** ...this problem for this pair will not be reported again.\n",
                ];
                for (allowed_lines,have_flags,sizing) in [
                    (usize::MAX,true,false),(usize::MAX,true,true),
                    (0,true,false),(1,true,false),(2,true,false),(3,true,false),(4,true,false),
                    (usize::MAX,false,false),
                ] {
                    let mut control=pair_owners.control.clone();
                    let mut input=pair_owners.input.clone();let mut shortcut=pair_owners.shortcut.clone();
                    let mut zones=pair_owners.zones.clone();
                    let mut phase_globals=pair_owners.globals.clone();
                    phase_globals.kickoff_simulation=false;phase_globals.doing_sizing=sizing;
                    let clock=crate::weather::day::WeatherGlobalState { warmup_flag:true,..Default::default() };
                    let developer=true;
                    let flags=crate::heat_balance::error_backend::SourceDiagnosticFlags {
                        kickoff:&phase_globals.kickoff_simulation,developer:&developer,
                        warmup:&clock.warmup_flag,doing_sizing:&phase_globals.doing_sizing,
                    };
                    let mut warning_owners=SurfaceGetterOwners { model:pair_owners.model,environment:pair_owners.environment,
                        input:&mut input,shortcut:&mut shortcut,zones:&mut zones,control:&mut control,globals:&phase_globals };
                    let mut warning_pair=nominal_checkpoint.clone();
                    let mut state=ctf_ordinary_output_test_state();state.output_error_header=false;
                    assert!(!state.warmup_flag && !state.doing_sizing && !state.developer_flag && !state.kickoff_simulation);
                    let (mut writer,mut console)=(ExposureRecordWriter { bytes:Vec::new(),completed_lines:0,allowed_lines },Vec::new());
                    let mut backend=SourceErrorBackend {
                        state:&mut state,error_stream:Some(&mut writer),console:&mut console,
                        out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
                    };
                    let result=warning_pair.reconcile_opaque_surface_boundaries(
                        &mut warning_owners,&mut backend,have_flags.then_some(&flags),Some(&material_checkpoint));
                    let success=have_flags && allowed_lines>=5;
                    if success { result?; } else { assert!(result.is_err()); }
                    let completed=if have_flags { allowed_lines.min(5) } else { 0 };
                    assert_eq!(warning_pair.iz_const_diff_message,Some(completed>=4),
                        "the invocation hint changes only after its Continue returns");
                    let rows=warning_pair.final_surfaces.as_ref().ok_or("warning child Found prefix")?;
                    assert_eq!((rows[0].row.exterior_boundary,rows[1].row.exterior_boundary),(2,if success { 1 } else { -999 }));
                    assert!(!warning_pair.surface_error,"accepted warnings and effect failures do not invent Native Severe");
                    for row in &warning_owners.control.construction_usage {
                        assert_eq!(row.reverse_construction_layers_order_warning,success,
                            "both persistent latches change only after the fifth record returns");
                        assert!(!row.reverse_construction_num_layers_warning);
                    }
                    assert_eq!(backend.state.tracking.total_warning_errors,i32::from(have_flags));
                    assert_eq!(backend.state.tracking.total_warning_errors_during_warmup,i32::from(have_flags && !sizing));
                    assert_eq!(backend.state.tracking.total_warning_errors_during_sizing,i32::from(have_flags && sizing));
                    assert_eq!(backend.state.tracking.total_severe_errors,0);
                    assert_eq!(writer.completed_lines,completed);
                    assert_eq!(writer.bytes,expected_records[..completed].concat().as_bytes(),
                        "all completed warning records precede the failed writer operation");
                    if success {
                        for row in rows { assert!(!row.expected_wind_exposure(true,true)?); }
                    } else {
                        assert!(rows[0].expected_wind_exposure(true,true).is_err(),
                            "an incomplete body supplies no completed-exposure validation expectation");
                    }
                }
                // Deliberately varied ACTUAL NominalU/group owner checkpoints.
                // Missing, nonfinite and over-threshold values stop before warning;
                // unrelated material groups remain genuinely lazy.
                for (nominal_u,group_mode,expected) in [
                    (None,"regular",false),(Some(vec![]),"regular",false),
                    (Some(vec![0.5]),"regular",false),
                    (Some(vec![f64::NAN,0.5]),"regular",false),
                    (Some(vec![0.5,f64::INFINITY]),"regular",false),
                    (Some(vec![0.0,f64::from_bits(0.001_f64.to_bits()+1)]),"regular",false),
                    (Some(vec![0.0,0.001]),"regular",true),
                    (Some(vec![0.5,0.5]),"none",false),
                    (Some(vec![0.5,0.5]),"missing",false),
                    (Some(vec![0.5,0.5]),"glass",false),
                    (Some(vec![0.5,0.5]),"unused-missing",true),
                ] {
                    let mut control=pair_owners.control.clone();control.nominal_u=nominal_u;
                    let mut materials=material_checkpoint.clone();
                    match group_mode {
                        "missing"=>{ materials.material_groups.remove(&MaterialId(0)); }
                        "glass"=>{ materials.material_groups.insert(MaterialId(0),SourceMaterialGroup::Glass); }
                        "unused-missing"=>{ materials.material_groups.remove(&MaterialId(2)); }
                        _=>{}
                    }
                    let mut input=pair_owners.input.clone();let mut shortcut=pair_owners.shortcut.clone();
                    let mut zones=pair_owners.zones.clone();
                    let mut probe_owners=SurfaceGetterOwners { model:pair_owners.model,environment:pair_owners.environment,
                        input:&mut input,shortcut:&mut shortcut,zones:&mut zones,control:&mut control,globals:pair_owners.globals };
                    let mut probe=nominal_checkpoint.clone();
                    let clock=crate::weather::day::WeatherGlobalState { warmup_flag:true,..Default::default() };
                    let developer=true;
                    let flags=crate::heat_balance::error_backend::SourceDiagnosticFlags {
                        kickoff:&pair_owners.globals.kickoff_simulation,developer:&developer,
                        warmup:&clock.warmup_flag,doing_sizing:&pair_owners.globals.doing_sizing,
                    };
                    let (mut writer,mut console)=(Vec::new(),Vec::new());
                    let mut state=ctf_ordinary_output_test_state();state.output_error_header=false;
                    let mut backend=SourceErrorBackend {
                        state:&mut state,error_stream:Some(&mut writer),console:&mut console,
                        out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
                    };
                    let result=probe.reconcile_opaque_surface_boundaries(
                        &mut probe_owners,&mut backend,Some(&flags),(group_mode!="none").then_some(&materials));
                    if expected { result?; } else { assert!(result.is_err()); }
                    let rows=probe.final_surfaces.as_ref().ok_or("U/group availability prefix")?;
                    assert_eq!((rows[0].row.exterior_boundary,rows[1].row.exterior_boundary),(2,if expected { 1 } else { -999 }));
                    assert!(!probe.surface_error);
                    assert_eq!(probe.iz_const_diff_message,Some(expected));
                    assert_eq!(backend.state.tracking.total_warning_errors,i32::from(expected));
                    assert_eq!(backend.state.tracking.total_severe_errors,0);
                    assert!(probe_owners.control.construction_usage.iter().all(|row|
                        row.reverse_construction_layers_order_warning==expected && !row.reverse_construction_num_layers_warning));
                    let text=std::str::from_utf8(&writer)?;
                    assert_eq!(text.lines().count(),if expected { 5 } else { 0 });
                    if expected && group_mode=="regular" {
                        assert!(text.contains("diff=[1.0000E-003]"),
                            "the exact .001 threshold reaches the existing Source .4R formatter");
                    }
                }
                // Native1842/1846 read BOTH construction latches before U/groups,
                // area/exposure and reciprocal writes. These are authored entry
                // checkpoints, not fabricated prior Native warning endpoints.
                for number_latch in [true,false] {
                    let mut control=pair_owners.control.clone();control.nominal_u=None;
                    for row in &mut control.construction_usage {
                        row.reverse_construction_num_layers_warning=number_latch;
                        row.reverse_construction_layers_order_warning = !number_latch;
                    }
                    let mut input=pair_owners.input.clone();let mut shortcut=pair_owners.shortcut.clone();
                    let mut zones=pair_owners.zones.clone();
                    let mut latched_owners=SurfaceGetterOwners { model:pair_owners.model,environment:pair_owners.environment,
                        input:&mut input,shortcut:&mut shortcut,zones:&mut zones,control:&mut control,globals:pair_owners.globals };
                    let mut latched=nominal_checkpoint.clone();latched.iz_const_diff_message=Some(true);
                    for row in latched.final_surfaces.as_mut().ok_or("latched pair")? { row.row.exterior_wind=true; }
                    let (mut writer,mut console)=(Vec::new(),Vec::new());
                    let mut state=ctf_ordinary_output_test_state();state.output_error_header=false;
                    let before=state.tracking.clone();
                    let mut backend=SourceErrorBackend {
                        state:&mut state,error_stream:Some(&mut writer),console:&mut console,
                        out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
                    };
                    latched.reconcile_opaque_surface_boundaries(&mut latched_owners,&mut backend,None,None)?;
                    assert_eq!(latched.iz_const_diff_message,Some(false),"the local resets on each invocation");
                    assert!(!latched.surface_error);
                    let rows=latched.final_surfaces.as_ref().ok_or("latched return")?;
                    assert_eq!((rows[0].row.exterior_boundary,rows[1].row.exterior_boundary),(2,1));
                    for row in rows {
                        assert!(row.row.exterior_wind);
                        assert!(row.expected_wind_exposure(true,true)?);
                    }
                    assert_eq!(backend.state.tracking,before);
                    assert!(latched_owners.control.construction_usage.iter().all(|row|
                        row.reverse_construction_num_layers_warning==number_latch
                        && row.reverse_construction_layers_order_warning!=number_latch));
                    assert!(writer.is_empty());
                }
                // Actual Source traversal groups the two left rows before the two
                // right rows. Reusing real getter constructions exposes the
                // asymmetric skipped/Completed validation path; no IDs are changed.
                for (same_first,wind) in [(false,false),(false,true),(true,false)] {
                    let checkpoint_rows=nominal_checkpoint.final_surfaces.as_ref().ok_or("two-pair checkpoint")?;
                    let left_number=checkpoint_rows[0].row.construction;
                    let right_number=checkpoint_rows[1].row.construction;
                    let mut rows=vec![checkpoint_rows[0].clone(),checkpoint_rows[0].clone(),
                        checkpoint_rows[1].clone(),checkpoint_rows[1].clone()];
                    let names=["STACK LEFT A","STACK LEFT B","STACK RIGHT A","STACK RIGHT B"];
                    for (index,row) in rows.iter_mut().enumerate() {
                        row.row.name=names[index].into();
                        row.row.base_surface=i32::try_from(index+1)?;
                        row.row.exterior_boundary = -999;
                        row.row.exterior_boundary_name=names[[2,3,0,1][index]].into();
                        row.row.construction=if index<2 || (same_first && index==2) { left_number } else { right_number };
                        row.row.exterior_wind=wind;
                    }
                    let mut control=pair_owners.control.clone();
                    let mut input=pair_owners.input.clone();let mut shortcut=pair_owners.shortcut.clone();
                    let mut zones=pair_owners.zones.clone();
                    let mut repeated_owners=SurfaceGetterOwners { model:pair_owners.model,environment:pair_owners.environment,
                        input:&mut input,shortcut:&mut shortcut,zones:&mut zones,control:&mut control,globals:pair_owners.globals };
                    let mut repeated=nominal_checkpoint.clone();repeated.moved_surfaces=4;repeated.total_surfaces=4;
                    repeated.final_surfaces=Some(rows);
                    let clock=crate::weather::day::WeatherGlobalState { warmup_flag:true,..Default::default() };
                    let developer=true;
                    let flags=crate::heat_balance::error_backend::SourceDiagnosticFlags {
                        kickoff:&pair_owners.globals.kickoff_simulation,developer:&developer,
                        warmup:&clock.warmup_flag,doing_sizing:&pair_owners.globals.doing_sizing,
                    };
                    let (mut writer,mut console)=(Vec::new(),Vec::new());
                    let mut state=ctf_ordinary_output_test_state();state.output_error_header=false;
                    let mut backend=SourceErrorBackend {
                        state:&mut state,error_stream:Some(&mut writer),console:&mut console,
                        out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None,
                    };
                    repeated.reconcile_opaque_surface_boundaries(
                        &mut repeated_owners,&mut backend,Some(&flags),Some(&material_checkpoint))?;
                    let warnings=if same_first || wind { 2 } else { 1 };
                    assert_eq!(backend.state.tracking.total_warning_errors,warnings);
                    assert_eq!(backend.state.tracking.total_warning_errors_during_warmup,warnings);
                    assert_eq!(backend.state.tracking.total_severe_errors,0);
                    assert_eq!(repeated.iz_const_diff_message,Some(true));
                    assert!(!repeated.surface_error);
                    assert!(repeated_owners.control.construction_usage.iter().all(|row|
                        row.reverse_construction_layers_order_warning && !row.reverse_construction_num_layers_warning));
                    let rows=repeated.final_surfaces.as_ref().ok_or("reused-construction return")?;
                    assert_eq!(rows.iter().map(|row|row.row.exterior_boundary).collect::<Vec<_>>(),[3,4,1,2]);
                    for (index,row) in rows.iter().enumerate() {
                        let expected=wind && (index==1 || index==3);
                        assert_eq!(row.row.exterior_wind,expected);
                        assert_eq!(row.expected_wind_exposure(wind,true)?,expected,
                            "later reused constructions retain Wind only on their actual skipped rows");
                    }
                    let text=std::str::from_utf8(&writer)?;
                    assert_eq!(text.matches(" ** Warning **").count(),usize::try_from(warnings)?);
                    assert_eq!(text.matches("proper \"reverse\" construction should be created.").count(),1,
                        "the invocation-local hint is emitted once across two distinct warnings");
                    assert_eq!(text.matches("this problem for this pair will not be reported again.").count(),if same_first { 2 } else { 1 });
                    assert_eq!(text.matches("Interzone surfaces cannot be \"WindExposed\"").count(),usize::from(wind));
                    assert_eq!(text.lines().count(),if same_first { 9 } else if wind { 8 } else { 5 });
                }
            }
        }
        assert!(raw.final_surfaces.as_ref().ok_or("actual pre-override Surface algorithm rows")?.iter()
            .all(|surface| surface.row.heat_transfer_algorithm == crate::heat_balance::selected_heat_balance_input::SelectedHeatTransferModel::Invalid));
        raw.report_surface_input_diagnostics(&mut context.backend)?;
        raw.read_empty_surface_boundary_inputs(
            &mut owners,
            &mut context.backend,
            context.eio,
            &mut errors,
        )?;
        assert_eq!(
            owners.shortcut.current_module_object,
            "SurfaceProperty:HeatTransferAlgorithm:Construction"
        );
        assert!(matches!(
            owners
                .input
                .family("SurfaceProperty:ExteriorNaturalVentedCavity"),
            Some(crate::heat_balance::selected_input_processor::SourceInputFamily::Null)
        ));
        assert_eq!(raw.total_ext_vented_cavities, 0);
        assert!(raw.ext_vented_cavities.as_ref().is_some_and(Vec::is_empty));
        assert!(raw.final_surfaces.as_ref().ok_or("actual Surface algorithm rows")?.iter()
            .all(|surface| surface.row.heat_transfer_algorithm == crate::heat_balance::selected_heat_balance_input::SelectedHeatTransferModel::Ctf));
        assert!(owners.control.construction_usage[0].is_used_ctf);
        // Authored callback host tests the existing OP interface; actual A uses
        // the real retained filesystem MTD host. No OS success is inferred here.
        struct EnclosureTestHost {
            calls: usize,
            fail: bool,
        }
        impl crate::heat_balance::source_output_registration::SourceOutputFileHost for EnclosureTestHost {
            fn ensure_mtd_open(&mut self) -> std::io::Result<()> {
                self.calls += 1;
                if self.fail {
                    return Err(std::io::Error::other("authored enclosure MTD failure"));
                }
                Ok(())
            }
        }
        let radiation_input = crate::heat_balance::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory([], false)?;
        let mut source_solar = None;
        for (floor_only, fail) in [(false, false), (true, false), (false, true)] {
            let zone = &mut owners.zones.zones.as_mut().ok_or("actual enclosure Zone")?[0];
            zone.name = "authored Source Zone".into();
            zone.radiant_enclosure_first = -1;
            zone.radiant_enclosure_last = -1;
            let space = &mut owners
                .zones
                .spaces
                .as_mut()
                .ok_or("actual enclosure Space")?[0];
            space.name = "authored Source Space".into();
            space.floor_area = 25.0;
            space.radiant_enclosure_num = 0;
            space.solar_enclosure_num = 0;
            let mut radiation =
                crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(
                    radiation_input,
                );
            assert!(radiation.enclosures().is_empty());
            assert_eq!(radiation.num_solar_enclosures(), 0);
            let mut registration =
                crate::heat_balance::source_output_registration::SourceOutputRegistration::default(
                );
            let prescan =
                crate::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            let mut host = EnclosureTestHost { calls: 0, fail };
            let mut registration_eio = Vec::new();
            let mut output = crate::heat_balance::radiation::ordinary::EnclosureOutputOwners {
                output: &mut registration,
                prescan: &prescan,
                minimum_report_frequency: crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::new("", None),
                backend: &mut context.backend,
                minutes_in_time_step: 15,
                host: &mut host,
                eio: &mut registration_eio,
            };
            // Scoped authored class inputs cover the otherwise independent
            // floor-only grouping branch; no geometry/Native endpoint is inferred.
            let mut enclosure_surfaces = raw.clone();
            if floor_only {
                for surface in enclosure_surfaces
                    .final_surfaces
                    .as_mut()
                    .ok_or("authored floor-only rows")?
                {
                    surface.row.class = SurfaceClass::Floor;
                }
            }
            let result =
                radiation.setup_radiant_enclosures(&enclosure_surfaces, &mut owners, &mut output);
            assert_eq!(radiation.num_radiant_enclosures(), 1);
            assert_eq!(radiation.radiant_enclosures()[0].space_nums, [1]);
            assert_eq!(
                radiation.radiant_enclosures()[0].space_names,
                ["authored Source Space"]
            );
            assert_eq!(
                radiation.radiant_enclosures()[0].floor_area.to_bits(),
                25.0_f64.to_bits()
            );
            assert_eq!(
                radiation.radiant_enclosures()[0]
                    .mean_radiant_temperature
                    .to_bits(),
                0.0_f64.to_bits()
            );
            assert_eq!(
                owners
                    .zones
                    .spaces
                    .as_ref()
                    .ok_or("retained enclosure Space")?[0]
                    .radiant_enclosure_num,
                1
            );
            if fail {
                let zone = &owners
                    .zones
                    .zones
                    .as_ref()
                    .ok_or("retained enclosure Zone")?[0];
                assert!(result.is_err());
                assert_eq!(
                    (zone.radiant_enclosure_first, zone.radiant_enclosure_last),
                    (-1, -1)
                );
                assert!(registration.output_initialized);
                assert_eq!(registration.num_real_variables_setup, 0);
                // Separate authored OP checkpoint: fail the common registration
                // after actual Solar allocation/Space writes, not a runtime retry.
                let mut solar_registration = Default::default();
                let failure = radiation.setup_solar_enclosures(
                    &enclosure_surfaces,
                    &mut owners,
                    &mut crate::heat_balance::radiation::ordinary::EnclosureOutputOwners {
                        output: &mut solar_registration,
                        prescan: &prescan,
                        minimum_report_frequency: crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::new("", None),
                        backend: &mut context.backend,
                        minutes_in_time_step: 15,
                        host: &mut host,
                        eio: &mut registration_eio,
                    },
                );
                assert!(failure.is_err());
                assert_eq!(radiation.num_solar_enclosures(), 1);
                assert_eq!(radiation.enclosures()[0].space_nums, [1]);
                assert!(radiation.enclosures()[0].zone_id.is_none());
                assert_eq!(
                    owners
                        .zones
                        .spaces
                        .as_ref()
                        .ok_or("failed solar Space prefix")?[0]
                        .solar_enclosure_num,
                    1
                );
                let zone = &owners
                    .zones
                    .zones
                    .as_ref()
                    .ok_or("failed solar Zone prefix")?[0];
                assert_eq!(
                    (zone.radiant_enclosure_first, zone.radiant_enclosure_last),
                    (-1, -1)
                );
                assert_eq!(solar_registration.num_real_variables_setup, 0);
                assert!(solar_registration.output_initialized);
            } else {
                result?;
                radiation.setup_solar_enclosures(&enclosure_surfaces, &mut owners, &mut output)?;
                let zone = &owners
                    .zones
                    .zones
                    .as_ref()
                    .ok_or("retained enclosure Zone")?[0];
                assert_eq!(
                    (zone.radiant_enclosure_first, zone.radiant_enclosure_last),
                    (1, 1)
                );
                assert_eq!(
                    registration.num_real_variables_setup, 2,
                    "Solar repeats real MRT registration"
                );
                assert_eq!(
                    registration.dictionary_variables[0].name,
                    "Enclosure Mean Radiant Temperature"
                );
                assert_eq!(
                    registration.dictionary_variables[0].key_out_var_nums.len(),
                    0,
                    "unmatched Source registration does not fabricate an MRT variable handle"
                );
                let allocation = radiation.radiant_enclosures().as_ptr();
                let solar_allocation = radiation.enclosures().as_ptr();
                assert_eq!(radiation.num_solar_enclosures(), 1);
                assert_eq!(
                    radiation.enclosures()[0].space_names,
                    ["authored Source Space"]
                );
                assert_eq!(
                    radiation.enclosures()[0].floor_area.to_bits(),
                    25.0_f64.to_bits()
                );
                assert!(
                    radiation.enclosures()[0].zone_id.is_none()
                        && radiation.enclosures()[0].f.is_empty()
                );
                if !floor_only {
                    source_solar = Some(radiation.clone());
                }
                radiation.bind_initialized_solar_geometry(
                    &model.typed,
                    &physical.surfaces,
                    radiation_input,
                )?;
                assert_eq!(radiation.radiant_enclosures().as_ptr(), allocation);
                assert_eq!(
                    radiation.enclosures().as_ptr(),
                    solar_allocation,
                    "later bind keeps the early Solar Vec"
                );
                assert_eq!(
                    radiation.enclosures()[0].zone_id,
                    Some(model.typed.zones[0].id)
                );
                assert_eq!(
                    radiation.enclosures()[0].name,
                    if floor_only {
                        "authored Source Zone"
                    } else {
                        "authored Source Space"
                    }
                );
                assert_eq!(
                    radiation.enclosures()[0].floor_area.to_bits(),
                    25.0_f64.to_bits()
                );
                assert_eq!(
                    radiation.radiant_enclosures()[0].name,
                    if floor_only {
                        "authored Source Zone"
                    } else {
                        "authored Source Space"
                    }
                );
                assert_eq!(
                    radiation.radiant_enclosures()[0].floor_area.to_bits(),
                    25.0_f64.to_bits()
                );
                context.reports.ordinary_solar_radiation = Some(radiation);
                let moved = context
                    .reports
                    .ordinary_solar_radiation
                    .take()
                    .ok_or("lost same radiation container")?;
                assert_eq!(moved.radiant_enclosures().as_ptr(), allocation);
            }
            assert_eq!(host.calls, if fail { 2 } else { 1 });
        }
        // Reuse this authored owner fixture for the newly reached zero suffix.
        assert!(
            raw.final_surfaces
                .as_ref()
                .ok_or("actual pre-suffix Surface")?
                .iter()
                .all(|surface| surface.representative_calc_surface_num == -1)
        );
        assert!(
            raw.window_arrays
                .as_ref()
                .ok_or("actual pre-suffix window arrays")?
                .surf_active_construction
                .iter()
                .all(|&value| value == 0)
        );
        raw.finish_selected_surface_input(&mut owners, &mut context.backend, &mut errors)?;
        assert_eq!(
            owners.shortcut.current_module_object,
            "SurfaceProperty:LocalEnvironment"
        );
        assert_eq!(
            (
                raw.total_surface_ground_properties,
                raw.total_surface_local_environments
            ),
            (0, 0)
        );
        assert!(
            owners
                .input
                .find_exact_family("SurfaceProperty:GroundSurfaces")
                .is_none()
        );
        assert!(
            raw.ground_surface_properties.is_none()
                && raw.surrounding_surface_properties.is_none()
                && raw.surface_local_environments.is_none()
        );
        assert!(raw.selected_get_surface_data_returned);
        assert_eq!(
            raw.final_surfaces
                .as_ref()
                .ok_or("actual representative rows")?
                .iter()
                .map(|surface| surface.representative_calc_surface_num)
                .collect::<Vec<_>>(),
            (1..=i32::try_from(total)?).collect::<Vec<_>>()
        );
        assert_eq!(
            raw.window_arrays
                .as_ref()
                .ok_or("actual active Construction owner")?
                .surf_active_construction,
            vec![1; total]
        );
        let mut failed_setup = raw.clone();
        assert!(failed_setup.release_setup_coordinates_if_errors(true));
        assert!(
            failed_setup.coordinate.cos_zone_relative.is_none()
                && failed_setup.coordinate.sin_zone_relative.is_none()
        );
        assert!(
            raw.coordinate.cos_zone_relative.is_some()
                && raw.coordinate.sin_zone_relative.is_some()
        );
        let clock =
            crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::from_family_inventory(
                [],
                model.typed.timestep.number_of_timesteps_per_hour as usize,
            )?;
        let mut equipment =
            crate::heat_balance::ordinary_equipment::OrdinaryEquipment::new_for_source(
                &model.typed,
                clock,
            )?;
        let space_controls = crate::heat_balance::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls::default();
        let mut equipment_schedules =
            crate::heat_balance::selected_schedule_manager::SelectedScheduleManager::default();
        let old_module = owners.shortcut.current_module_object.clone();
        equipment.get_zone_equipment_source(
            owners.input,
            clock,
            space_controls.sizing,
            space_controls.simulation,
            owners.globals,
            &mut context.reports.selected_internal_heat_gains,
            crate::heat_balance::ordinary_equipment::SourceEquipmentInputOwners {
                zones: owners.zones.zones.as_deref_mut().ok_or("actual equipment Zone rows")?,
                schedules: &mut equipment_schedules,
                sizing: &mut context.reports.selected_sizing_manager,
                branch: None,
                // Unused: this authored admitted-input fixture has zero controlled Connections.
                source_is_epjson: false,
            },
        )?;
        assert_eq!(owners.shortcut.current_module_object, old_module);
        for family in [
            "NodeList",
            "ZoneHVAC:EquipmentList",
            "ZoneHVAC:EquipmentConnections",
            "SpaceHVAC:EquipmentConnections",
            "AirLoopHVAC:SupplyPath",
            "AirLoopHVAC:ReturnPath",
        ] {
            assert!(matches!(
                owners.input.find_exact_family(family),
                Some(crate::heat_balance::selected_input_processor::SourceInputFamily::Null)
            ));
        }
        raw.finish_zero_window_setup(owners.input, owners.shortcut, errors)?;
        assert_eq!(
            owners.shortcut.current_module_object,
            "WindowProperty:StormWindow"
        );
        assert_eq!(raw.total_storm_windows, 0);
        assert!(raw.storm_windows.is_none());
        assert!(
            raw.coordinate.cos_zone_relative.is_none()
                && raw.coordinate.sin_zone_relative.is_none()
        );
        assert!(raw.process_vertices_scratch.one_time);
        assert!(raw.process_vertices_scratch.x.is_none());
        // Constructor-zero X0 is checked before the actual Process writer.
        assert!(
            raw.parallel_arrays
                .as_ref()
                .ok_or("pre-Process parallel owner")?
                .x0
                .iter()
                .all(|value| value.to_bits() == 0.0_f64.to_bits())
        );
        raw.process_opaque_setup_surfaces(owners.zones, owners.control)?;
        assert!(!raw.process_vertices_scratch.one_time);
        let process_arrays = raw
            .parallel_arrays
            .as_ref()
            .ok_or("actual processed parallel owner")?;
        let process_rows = raw
            .final_surfaces
            .as_ref()
            .ok_or("actual processed final owner")?;
        for (row, shade) in process_rows.iter().zip(&process_arrays.shade_vertices) {
            assert!(row.row.vertices_processed && !row.row.is_shadowing);
            assert_eq!(
                row.row.shape,
                crate::geometry::SourceProcessShape::Rectangle
            );
            assert_eq!(shade.n_vert, row.row.sides);
            let vertices = row
                .row
                .vertex
                .as_ref()
                .ok_or("actual Process vertex owner")?;
            for (component, values) in [shade.xv.as_ref(), shade.yv.as_ref(), shade.zv.as_ref()]
                .into_iter()
                .enumerate()
            {
                let values = values.ok_or("actual allocated ShadeV component")?;
                assert_eq!(values.len(), vertices.len());
                assert_eq!(
                    values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    vertices
                        .iter()
                        .map(|v| v
                            .ok_or("actual written vertex")
                            .map(|p| p[component].to_bits()))
                        .collect::<Result<Vec<_>, _>>()?
                );
            }
        }
        assert_eq!(
            raw.surface_lists.all_exterior_solar_and_shading,
            raw.surface_lists.all_exterior_solar
        );
        assert_eq!(raw.setup_shading.last, -1);
        for zone in owners
            .zones
            .zones
            .as_ref()
            .ok_or("actual Setup reset zones")?
        {
            assert!(!zone.has_window);
            assert_eq!(
                [
                    zone.exterior_window_area,
                    zone.exterior_gross_wall_area,
                    zone.exterior_net_wall_area,
                    zone.total_surface_area
                ],
                [0.0; 4]
            );
        }
        for space in owners
            .zones
            .spaces
            .as_ref()
            .ok_or("actual Setup reset spaces")?
        {
            assert_eq!(
                [space.exterior_window_area, space.total_surface_area],
                [0.0; 2]
            );
        }
        // Authored negative operands exercise reached failure custody, without
        // supplying Native AFTER values or broadening positive raw admission.
        let debug = SelectedInputProcessor::from_admitted_input(
            vec![
                crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                    object_type: "DETAILEDWWR_DEBUG".into(),
                    alpha_fields: 0,
                    numeric_fields: 0,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                },
            ],
            std::collections::BTreeMap::from([(
                "DETAILEDWWR_DEBUG".into(),
                crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(vec![
                    crate::heat_balance::selected_input_processor::SourceObjectShape {
                        object_name: "authored debug section".into(),
                        field_sizes: Default::default(),
                    },
                ]),
            )]),
        )?;
        assert_eq!(debug.get_num_sections_found("DETAILEDWWR_DEBUG")?, 1);
        let mut debug_zones = owners.zones.clone();
        let before_debug = debug_zones.clone();
        assert!(matches!(raw.accumulate_opaque_setup_areas(&debug, &mut debug_zones, owners.control),
            Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("positive DetailedWWR debug OutputFile"))));
        assert_eq!(
            debug_zones, before_debug,
            "debug header failure precedes every area write"
        );
        let mut window_rows = raw.clone();
        let first = &mut window_rows
            .final_surfaces
            .as_mut()
            .ok_or("authored area rows")?[0]
            .row;
        first.class = SurfaceClass::Floor; // Reach the distinct geometric prefix in this test-owned row.
        let area = first
            .vertex_progress
            .properties
            .ok_or("authored retained Area")?
            .area_m2;
        let gross = first
            .vertex_progress
            .normal_area
            .ok_or("authored retained GrossArea")?
            .gross_area_m2;
        let mut window_control = owners.control.clone();
        window_control.construction_usage[0].type_is_window = true;
        let mut window_zones = owners.zones.clone();
        let mut expected_prefix = window_zones.clone();
        let zone = &mut expected_prefix
            .zones
            .as_mut()
            .ok_or("authored prefix Zone")?[0];
        zone.total_surface_area += area;
        zone.geometric_floor_area += gross;
        expected_prefix
            .spaces
            .as_mut()
            .ok_or("authored prefix Space")?[0]
            .total_surface_area += area;
        assert!(matches!(window_rows.accumulate_opaque_setup_areas(owners.input, &mut window_zones, &window_control),
            Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("positive area window/frame branch"))));
        assert_eq!(
            window_zones, expected_prefix,
            "window boundary retains only the reached totals/geometric prefix"
        );
        raw.bind_typed_surfaces(&owners)?;
        let mut height_zones = owners.zones.clone();
        let detailed_wwr =
            raw.accumulate_opaque_setup_areas(owners.input, &mut height_zones, owners.control)?;
        assert!(!detailed_wwr && raw.err_count == 0);
        raw.prepare_opaque_setup_heights(
            &mut height_zones,
            false,
            detailed_wwr,
            &mut context.backend,
        )?;
        let zone = &height_zones
            .zones
            .as_ref()
            .ok_or("actual automatic height Zone")?[0];
        assert_eq!(zone.ceiling_height.to_bits(), 1.0_f64.to_bits());
        assert!(
            !zone.ceiling_height_entered,
            "automatic positive height is not user-entered"
        );
        // A real warning stream failure must preserve the prior entered flag
        // write, without assigning the automatic height or fabricating ErrCount.
        let mut warning_rows = raw.clone();
        warning_rows.err_count = 1;
        let mut warning_zones = height_zones.clone();
        warning_zones
            .zones
            .as_mut()
            .ok_or("authored entered height Zone")?[0]
            .ceiling_height = 2.0;
        let mut expected_failure = warning_zones.clone();
        expected_failure
            .zones
            .as_mut()
            .ok_or("authored warning prefix")?[0]
            .ceiling_height_entered = true;
        let (mut warning_state, mut warning_console, mut failing) = (
            ctf_ordinary_output_test_state(),
            Vec::new(),
            OrdinaryFailingEio,
        );
        let failure = warning_rows.prepare_opaque_setup_heights(
            &mut warning_zones,
            false,
            detailed_wwr,
            &mut SourceErrorBackend {
                state: &mut warning_state,
                error_stream: Some(&mut failing),
                console: &mut warning_console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        );
        assert!(matches!(
            failure,
            Err(
                crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(_)
            )
        ));
        assert_eq!(warning_zones, expected_failure);
        assert_eq!(warning_rows.err_count, 1);
        assert_eq!(warning_state.tracking.total_warning_errors, 1);
        // All six authored cube faces are inward-wound. Reverse every copied
        // Vertex list; cached Newell values stay unchanged so548 recomputes them.
        let mut volume_rows = raw.clone();
        for surface in volume_rows
            .final_surfaces
            .as_mut()
            .ok_or("authored volume rows")?
        {
            surface
                .row
                .vertex
                .as_mut()
                .ok_or("authored volume vertices")?
                .reverse();
        }
        let mut volume_zones = height_zones.clone();
        volume_zones
            .zones
            .as_mut()
            .ok_or("authored floor-present checkpoint")?[0]
            .has_floor = true;
        let before_volume = volume_zones.clone();
        let returned = volume_rows.calculate_opaque_setup_volumes(
            owners.input,
            &mut volume_zones,
            false,
            &mut context.backend,
        )?;
        assert_eq!(returned.len(), 1);
        assert_eq!(returned[0].returned.volume_m3.to_bits(), 1.0_f64.to_bits());
        assert_eq!(
            returned[0].returned.diagnostics.sum_face_areas_m2.to_bits(),
            6.0_f64.to_bits()
        );
        assert_eq!(
            volume_zones
                .spaces
                .as_ref()
                .ok_or("actual computed Space volume")?[0]
                .volume
                .to_bits(),
            1.0_f64.to_bits()
        );
        assert_eq!(
            volume_zones
                .spaces
                .as_ref()
                .ok_or("actual computed fraction")?[0]
                .fraction_zone_volume
                .to_bits(),
            1.0_f64.to_bits()
        );
        let show_debug = SelectedInputProcessor::from_admitted_input(
            vec![
                crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                    object_type: "SHOWZONESURFACES_DEBUG".into(),
                    alpha_fields: 0,
                    numeric_fields: 0,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                },
            ],
            std::collections::BTreeMap::from([(
                "SHOWZONESURFACES_DEBUG".into(),
                crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(vec![
                    crate::heat_balance::selected_input_processor::SourceObjectShape {
                        object_name: "authored debug section".into(),
                        field_sizes: Default::default(),
                    },
                ]),
            )]),
        )?;
        let mut debug_volume = before_volume.clone();
        assert!(matches!(volume_rows.calculate_opaque_setup_volumes(&show_debug, &mut debug_volume, false, &mut context.backend),
            Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("positive SHOWZONESURFACES debug OutputFile"))));
        assert_eq!(
            debug_volume, volume_zones,
            "first debug print follows Zone/Space/fraction writes"
        );
        let mut retained_user_volume = before_volume.clone();
        retained_user_volume
            .zones
            .as_mut()
            .ok_or("authored positive user volume")?[0]
            .volume = 1.01;
        let retained = volume_rows.calculate_opaque_setup_volumes(
            owners.input,
            &mut retained_user_volume,
            false,
            &mut context.backend,
        )?;
        assert_eq!(retained[0].returned.volume_m3.to_bits(), 1.01_f64.to_bits());
        assert_eq!(
            retained_user_volume
                .spaces
                .as_ref()
                .ok_or("user-priority Space volume")?[0]
                .volume
                .to_bits(),
            1.01_f64.to_bits()
        );
        let mut user_volume = before_volume;
        user_volume.zones.as_mut().ok_or("authored user volume")?[0].volume = 2.0;
        let before_failure = user_volume.clone();
        let (mut note_state, mut note_console, mut note_failure) = (
            ctf_ordinary_output_test_state(),
            Vec::new(),
            OrdinaryFailingEio,
        );
        let failure = volume_rows.calculate_opaque_setup_volumes(
            owners.input,
            &mut user_volume,
            false,
            &mut SourceErrorBackend {
                state: &mut note_state,
                error_stream: Some(&mut note_failure),
                console: &mut note_console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        );
        assert!(matches!(
            failure,
            Err(
                crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(_)
            )
        ));
        assert_eq!(
            volume_rows.err_count5, 1,
            "counter increment precedes failed informational note"
        );
        assert_eq!(
            user_volume, before_failure,
            "user volume and pending Space fields survive note failure"
        );
        assert_eq!(note_state.tracking.total_warning_errors, 0);
        // Authored checkpoints use the genuine earlier centroid producer and
        // actual550 suffix, without Native AFTER injection or new test bodies.
        let mut bounds_rows = raw.clone();
        bounds_rows.calculate_final_centroids(&mut context.backend, &environment)?;
        bounds_rows
            .final_surfaces
            .as_mut()
            .ok_or("authored self-adjacency")?[0]
            .row
            .exterior_boundary = 1;
        let mut bounds_zones = volume_zones.clone();
        assert!(bounds_rows.adjacent_zone.is_none());
        assert_eq!(
            bounds_zones.zones.as_ref().ok_or("ctor-zero counts")?[0].num_surfaces,
            0
        );
        bounds_rows.prepare_opaque_setup_zone_bounds(
            &mut bounds_zones,
            1,
            &mut context.backend,
            &mut errors,
        )?;
        let zone = &bounds_zones.zones.as_ref().ok_or("actual bounds Zone")?[0];
        assert_eq!(zone.centroid_m, Some([0.5; 3]));
        assert_eq!(zone.bounds_m, [0.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
        assert_eq!(
            (
                zone.num_surfaces,
                zone.num_sub_surfaces,
                zone.num_shading_surfaces
            ),
            (6, 0, 0)
        );
        let mut expected_adjacent = vec![0; total];
        expected_adjacent[0] = 1;
        assert_eq!(bounds_rows.adjacent_zone, Some(expected_adjacent.clone()));
        bounds_rows.prepare_opaque_setup_zone_bounds(
            &mut bounds_zones,
            1,
            &mut context.backend,
            &mut errors,
        )?;
        assert_eq!(
            bounds_zones.zones.as_ref().ok_or("cumulative counts")?[0].num_surfaces,
            12
        );
        let before_adjacency_failure = bounds_zones.clone();
        bounds_rows.adjacent_zone = Some(vec![-7; total]);
        bounds_rows
            .final_surfaces
            .as_mut()
            .ok_or("authored missing adjacency")?[1]
            .row
            .exterior_boundary = i32::try_from(total + 1)?;
        let failure = bounds_rows.prepare_opaque_setup_zone_bounds(
            &mut bounds_zones,
            1,
            &mut context.backend,
            &mut errors,
        );
        assert!(matches!(failure, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("actual adjacent Surface association"))));
        assert_eq!(
            bounds_zones, before_adjacency_failure,
            "completed centroid/bounds and prior volume retained; counts not reached"
        );
        assert_eq!(
            bounds_rows.adjacent_zone,
            Some(expected_adjacent),
            "new dimension and first entry retained at invalid second association"
        );
        assert!(!errors);
        // The Setup800/801 prefix updates bulbs and preserves both wind fields.
        let mut bulb_zones = bounds_zones
            .zones
            .as_ref()
            .ok_or("authored bulb Zone")?
            .clone();
        bulb_zones[0].wind_speed = -9.0;
        bulb_zones[0].wind_direction = 17.0;
        let mut parameters = environment.outdoor_environment_parameters;
        parameters.site_temperature_gradient_k_per_m = 0.0;
        crate::heat_balance::convection::surface_environment::ordinary::update_zone_out_bulb_temperatures(
            &mut bulb_zones, parameters, 12.0, 10.0, &mut context.backend)?;
        let bulb_fields = |zone: &SelectedZoneData| {
            [
                zone.dry_bulb,
                zone.wet_bulb,
                zone.wind_speed,
                zone.wind_direction,
            ]
            .map(f64::to_bits)
        };
        assert_eq!(
            bulb_fields(&bulb_zones[0]),
            [12.0_f64, 10.0, -9.0, 17.0].map(f64::to_bits)
        );
        let (mut bulb_state, mut bulb_console, mut bulb_failure) = (
            ctf_ordinary_output_test_state(),
            Vec::new(),
            OrdinaryFailingEio,
        );
        let failure = crate::heat_balance::convection::surface_environment::ordinary::update_zone_out_bulb_temperatures(
            &mut bulb_zones, parameters, -101.0, -102.0,
            &mut SourceErrorBackend { state: &mut bulb_state, error_stream: Some(&mut bulb_failure),
                console: &mut bulb_console, out_unit1: None, out_unit2: None, f_message: None,
                message_callback: None, error_callback: None, sqlite: None });
        assert!(matches!(failure, Err(crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentError::Backend(_))));
        assert_eq!(
            bulb_fields(&bulb_zones[0]),
            [-101.0_f64, -102.0, -9.0, 17.0].map(f64::to_bits)
        );
        assert_eq!(bulb_state.tracking.total_severe_errors, 1);
        assert!(raw.surface_tmp.is_none());
        let source_ids = raw
            .final_surfaces
            .as_ref()
            .ok_or("missing final Surface")?
            .iter()
            .map(|row| row.typed_id.ok_or("missing final physical ID"))
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            source_ids,
            [
                SurfaceId(2),
                SurfaceId(3),
                SurfaceId(4),
                SurfaceId(5),
                SurfaceId(0),
                SurfaceId(1)
            ]
        );
        // Authored constructor-type owner; actual Material type producers are exercised by A-24H.
        let source_numbers = (1..=i32::try_from(total)?).collect::<Vec<_>>();
        let zone = &zones.zones.as_ref().ok_or("authored Source list Zone")?[0];
        for list in [
            &raw.surface_lists.all_ht,
            &raw.surface_lists.all_ht_non_window,
            &raw.surface_lists.all_exterior_solar,
            &raw.surface_lists.all_shadow_possible_obstructions,
            &zone.ht_surface_list,
            &zone.ht_non_window_surface_list,
        ] {
            assert_eq!(list, &source_numbers);
        }
        assert!(raw.surface_lists.all_interzone.is_empty() && zone.iz_surface_list.is_empty());
        assert!(
            raw.surface_lists.all_ht_window.is_empty() && zone.ht_window_surface_list.is_empty()
        );
        assert_eq!(raw.surface_lists.filters[0], source_numbers);
        assert_eq!(raw.surface_lists.filters[2], [1, 2, 3, 4]);
        assert_eq!(raw.surface_lists.filters[3], [6]);
        assert_eq!(raw.surface_lists.filters[4], [5]);
        // Literal2788 appends exterior Roof to AllInteriorCeilings.
        assert_eq!(raw.surface_lists.filters[9], [6]);
        let arrays = raw
            .parallel_arrays
            .as_ref()
            .ok_or("missing real parallel allocation")?;
        assert_eq!(arrays.surf_penumbra_id, vec![-1; total]);
        assert_eq!(arrays.surf_t_air_ref, vec![-1; total]);
        assert_eq!(arrays.x0.len(), total);
        let windows = raw
            .window_arrays
            .as_ref()
            .ok_or("missing actual window allocation")?;
        assert_eq!(
            (
                windows.rows,
                windows.equivalent_layer_columns,
                windows.solid_window_columns
            ),
            (total, 7, 3)
        );
        assert_eq!(
            (windows.surf_win_a.len(), windows.surf_win_acf_overlap.len()),
            (total * 7, total * 3)
        );
        assert!(
            windows
                .surf_win_a
                .iter()
                .chain(&windows.surf_win_acf_overlap)
                .all(|value| value.to_bits() == 0.0_f64.to_bits())
        );
        let derived = raw
            .surface_windows
            .as_ref()
            .ok_or("missing real SurfaceWindowCalc allocation")?;
        assert_eq!(derived.len(), total);
        assert_eq!(
            raw.surface_shades
                .as_ref()
                .ok_or("missing real SurfaceShade allocation")?
                .len(),
            total
        );
        for row in derived {
            assert!(row.reference_points.is_none());
            assert!(row.complex_fenestration.states.is_none());
            assert_eq!(row.complex_fenestration.current_state, 1);
            assert_eq!(row.theta_face[0].to_bits(), 296.15_f64.to_bits());
            assert!(
                row.theta_face[1..]
                    .iter()
                    .all(|value| value.to_bits() == 0.0_f64.to_bits())
            );
            assert_eq!(
                row.outside_projection_sunlit_multiplier[0].to_bits(),
                1.0_f64.to_bits()
            );
            assert!(
                row.outside_projection_sunlit_multiplier[1..]
                    .iter()
                    .all(|value| value.to_bits() == 0.0_f64.to_bits())
            );
        }
        let outdoor = raw
            .parallel_arrays
            .as_mut()
            .and_then(|arrays| arrays.outdoor.as_mut())
            .ok_or("missing Source outdoor arrays")?;
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
        let original = raw
            .parallel_arrays
            .as_ref()
            .and_then(|arrays| arrays.outdoor.as_ref())
            .ok_or("missing Source custody")?;
        let allocation = original.as_ptr();
        let expected = original.iter().map(bits).collect::<Vec<_>>();
        assert_eq!(expected[0][2], (-0.0_f64).to_bits());
        context.reports.surface_geometry_input = raw;
        // Missing cardinality and duplicate physical IDs must both reject before take.
        for duplicate in [false, true] {
            let mut invalid = model.typed.clone();
            if duplicate {
                invalid.surfaces[0].id = invalid.surfaces[1].id;
            } else {
                invalid.surfaces.pop();
            }
            assert!(context.take_selected_world_outdoor_rows(&invalid).is_err());
            let arrays = context
                .reports
                .surface_geometry_input
                .parallel_arrays
                .as_ref()
                .ok_or("lost parallel arrays on rejection")?;
            let retained = arrays
                .outdoor
                .as_ref()
                .ok_or("invalid mapping moved Source owner")?;
            assert_eq!(retained.as_ptr(), allocation);
            assert_eq!(retained.iter().map(bits).collect::<Vec<_>>(), expected);
            assert!(arrays.outdoor_to_model_swaps.is_empty());
        }
        let moved = context.take_selected_world_outdoor_rows(&model.typed)?;
        assert_eq!(
            moved.as_ptr(),
            allocation,
            "take permutes the original allocation without cloning rows"
        );
        assert!(
            context
                .reports
                .surface_geometry_input
                .parallel_arrays
                .as_ref()
                .ok_or("lost arrays after take")?
                .outdoor
                .is_none()
        );
        assert_eq!(physical.surfaces.len(), moved.len());
        for ((surface, input), owner) in physical
            .surfaces
            .iter_mut()
            .zip(&model.typed.surfaces)
            .zip(moved)
        {
            assert_eq!(surface.surface_id, input.id);
            let source = source_ids
                .iter()
                .position(|id| *id == surface.surface_id)
                .ok_or("missing fixture physical association")?;
            assert_eq!(bits(&owner), expected[source]);
            assert_eq!(owner.manager_invoked, source % 2 == 0);
            surface.outdoor_environment = owner;
            assert_eq!(bits(&surface.outdoor_environment), expected[source]);
        }
        // A real post-take guard error exercises reverse custody, not a claimed
        // Native/Solar rollback endpoint or another fabricated caller phase.
        assert!(
            context
                .take_selected_world_outdoor_rows(&model.typed)
                .is_err()
        );
        let source_solar_physical = physical.surfaces.clone();
        context.restore_selected_world_outdoor_rows(physical.surfaces);
        let arrays = context
            .reports
            .surface_geometry_input
            .parallel_arrays
            .as_ref()
            .ok_or("lost arrays at restore")?;
        let restored = arrays
            .outdoor
            .as_ref()
            .ok_or("lost actual outdoor values at restore")?;
        assert_eq!(restored.iter().map(bits).collect::<Vec<_>>(), expected);
        assert!(
            restored
                .iter()
                .enumerate()
                .all(|(index, owner)| owner.manager_invoked == (index % 2 == 0))
        );
        assert!(arrays.outdoor_to_model_swaps.is_empty());
        assert_eq!(
            context
                .reports
                .surface_geometry_input
                .final_surfaces
                .as_ref()
                .ok_or("lost final Surface IDs")?
                .iter()
                .map(|row| row.typed_id)
                .collect::<Vec<_>>(),
            source_ids.iter().copied().map(Some).collect::<Vec<_>>()
        );
        // Authored reached Material:NoMass checkpoint, not Native R/U output.
        // The actual Construction getter must accumulate both repeated layers.
        use crate::heat_balance::selected_heat_balance_input::{
            ConstructionMaterialOwners, SelectedHeatBalanceInput,
        };
        use crate::heat_balance::selected_material_data::{
            SelectedMaterialData, SourceMaterialGroup,
        };
        use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
        let materials = SelectedMaterialData {
            materials: vec![MaterialId(0)],
            nominal_r: vec![1.0],
            material_map: std::collections::BTreeMap::from([("R1".into(), 1)]),
            material_groups: std::collections::BTreeMap::from([(
                MaterialId(0),
                SourceMaterialGroup::Regular,
            )]),
            ..Default::default()
        };
        let mut layered = model.typed.clone();
        layered.constructions[0].layers.push(MaterialId(0));
        let mut construction_input = SelectedInputProcessor::from_admitted_input(
            vec![
                crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                    object_type: "Construction".into(),
                    alpha_fields: 11,
                    numeric_fields: 0,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                },
            ],
            std::collections::BTreeMap::from([(
                "Construction".into(),
                crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(vec![
                    crate::heat_balance::selected_input_processor::SourceObjectShape {
                        object_name: "Wall".into(),
                        field_sizes: Default::default(),
                    },
                ]),
            )]),
        )?;
        let construction_raw = SelectedHeatBalanceInput {
            source_is_epjson: true,
            objects: std::collections::BTreeMap::from([(
                "Construction".into(),
                vec![ProjectInputObject {
                    object_name: "Wall".into(),
                    idf_max_fields: None,
                    fields: std::collections::BTreeMap::from([
                        (
                            "outside_layer".into(),
                            ProjectInputValue::String("R1".into()),
                        ),
                        ("layer_2".into(), ProjectInputValue::String("R1".into())),
                    ]),
                }],
            )]),
            ..Default::default()
        };
        let mut nominal = SelectedHeatBalanceInputState::default();
        assert!(
            nominal.nominal_r_for_u.is_none()
                && nominal.nominal_u.is_none()
                && nominal.nominal_u_before_adjusted.is_none()
                && nominal.coefficient_adjustment_ratio.is_none()
        );
        let mut construction_shortcut = SourceInputProcessorShortCut::default();
        construction_shortcut.allocate_from_input(&construction_input)?;
        let mut construction_bounds =
            crate::heat_balance::selected_simulation_startup::SelectedConstructionBounds::default();
        construction_bounds.set_preconstruction_input_parameters(&construction_input)?;
        nominal.read_regular_construction_arguments(
            &construction_raw,
            ConstructionMaterialOwners {
                model: &layered,
                data: &materials,
                bounds: &construction_bounds,
            },
            &mut construction_input,
            &mut construction_shortcut,
            &mut String::new(),
            &mut context.backend,
            &mut errors,
        )?;
        assert_eq!(nominal.nominal_r_for_u, Some(vec![2.0]));
        assert_eq!(nominal.nominal_u, Some(vec![0.5]));
        assert_eq!(
            nominal
                .nominal_u_before_adjusted
                .as_ref()
                .ok_or("actual U-before dimension")?[0]
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(nominal.coefficient_adjustment_ratio, Some(vec![1.0]));
        let material_surface = layered.materials[0]
            .surface_properties()
            .ok_or("authored opaque properties")?;
        let properties = &nominal.construction_usage[0];
        assert_eq!(
            [
                properties.inside_absorp_vis,
                properties.outside_absorp_vis,
                properties.inside_absorp_solar,
                properties.outside_absorp_solar,
                properties.inside_absorp_thermal,
                properties.outside_absorp_thermal,
                properties.reflect_vis_diff_back
            ]
            .map(f64::to_bits),
            [
                material_surface.visible_absorptance,
                material_surface.visible_absorptance,
                material_surface.solar_absorptance,
                material_surface.solar_absorptance,
                material_surface.thermal_absorptance,
                material_surface.thermal_absorptance,
                1.0 - material_surface.visible_absorptance
            ]
            .map(f64::to_bits)
        );
        assert_eq!(
            properties.outside_roughness,
            layered.materials[0].roughness()
        );
        assert_eq!(
            (
                properties.daylt_prop_ptr,
                properties.total_solid_layers,
                properties.total_glass_layers
            ),
            (0, 0, 0)
        );
        assert_eq!(properties.abs_diff_shade.to_bits(), 0.0_f64.to_bits());
        // Both actual early returns preserve every prior Construction field.
        for no_layers in [true, false] {
            let mut early = nominal.clone();
            early.construction_usage[0].type_is_window = true;
            if no_layers {
                early.construction_usage[0].total_layers = 0;
            } else {
                early.construction_usage[0].layer_points[1] = 0;
            }
            let before = early.construction_usage.clone();
            early.finish_selected_construction_nominal_u(
                ConstructionMaterialOwners {
                    model: &layered,
                    data: &materials,
                    bounds: &construction_bounds,
                },
                &mut context.backend,
                &mut errors,
            )?;
            assert_eq!(early.construction_usage, before);
        }
        // Same authored owners exercise report-ID order, referenced-class film
        // selection, and the actual table-before-EIO partial-failure boundary.
        let mut report_owner = CtfInitializationReports {
            registry: context.reports.registry.clone(),
            surface_geometry_input: context.reports.surface_geometry_input.clone(),
            selected_zone_input: Some(bounds_zones.clone()),
            selected_heat_balance_globals: Some(globals.clone()),
            heat_balance_input_state: nominal.clone(),
            ..Default::default()
        };
        let first = usize::try_from(
            report_owner
                .surface_geometry_input
                .all_surface_list_report_order[0]
                - 1,
        )?;
        let rows = report_owner
            .surface_geometry_input
            .final_surfaces
            .as_mut()
            .ok_or("actual report rows")?;
        assert_eq!(rows[first].row.class, SurfaceClass::Wall);
        let floor = rows
            .iter()
            .position(|row| row.row.class == SurfaceClass::Floor)
            .ok_or("authored referenced Floor")?;
        rows[first].row.exterior_boundary = i32::try_from(floor + 1)?;
        report_owner
            .selected_zone_input
            .as_mut()
            .and_then(|owner| owner.zones.as_mut())
            .ok_or("actual report Zone")?[0]
            .name = "AUTHORED ZONE".into();
        let mut report_eio = Vec::new();
        report_owner.write_selected_setup_geometry_reports(&mut report_eio)?;
        assert_eq!(report_owner.table.num_table_entry, i32::try_from(total)?);
        let entries = report_owner
            .table
            .entries
            .as_ref()
            .ok_or("actual film entries")?;
        assert_eq!(
            entries[0].original_real_entry.to_bits(),
            (1.0_f64 / (0.1197548 + 2.0 + 0.1620212)).to_bits()
        );
        let expected_names = report_owner
            .surface_geometry_input
            .all_surface_list_report_order
            .iter()
            .map(|number| {
                report_owner
                    .surface_geometry_input
                    .final_surfaces
                    .as_ref()
                    .ok_or("report allocation")?
                    .get(usize::try_from(number - 1)?)
                    .map(|row| row.row.name.as_str())
                    .ok_or("report association".into())
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
        assert_eq!(
            entries[..total]
                .iter()
                .map(|entry| entry.object_name.as_str())
                .collect::<Vec<_>>(),
            expected_names
        );
        let report_text = String::from_utf8(report_eio)?;
        assert_eq!(report_text.lines().count(), 6);
        assert!(report_text.contains(" Shading Summary,0,0,0\n"));
        assert!(report_text.contains(" Zone Summary,1,6,0\n"));
        assert!(
            report_text
                .contains(" Zone Information, AUTHORED ZONE,0.0,0.00,0.00,0.00,0.50,0.50,0.50,")
        );
        assert!(report_text.ends_with(",12,0,0,Yes\n"));
        let first_header = report_text
            .lines()
            .next()
            .ok_or("first report header")?
            .to_owned()
            + "\n";
        let row = &mut report_owner
            .surface_geometry_input
            .final_surfaces
            .as_mut()
            .ok_or("invalid construction report row")?[first]
            .row;
        row.construction = 0;
        row.exterior_boundary = -3; // No Compute call, so OSC is not reached.
        let mut prefix = vec![0_u8; first_header.len()];
        let failure = report_owner.write_selected_setup_geometry_reports(
            &mut std::io::Cursor::new(prefix.as_mut_slice()),
        );
        assert!(
            matches!(failure, Err(crate::RuntimeError::WarmupOutput(error)) if matches!(*error, crate::error::WarmupOutputError::Io(_)))
        );
        assert_eq!(prefix, first_header.as_bytes());
        assert_eq!(
            report_owner.table.num_table_entry,
            i32::try_from(2 * total)?
        );
        assert_eq!(
            report_owner
                .table
                .entries
                .as_ref()
                .ok_or("retained second film entries")?[total]
                .original_real_entry
                .to_bits(),
            0.0_f64.to_bits()
        );
        // Actual zero getters still query the schema and insert its absent
        // family. This authored unused Construction exercises the real warning.
        let mut scheduled_definition =
            crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: "SurfaceProperty:SolarIncidentInside".into(),
                alpha_fields: 4,
                numeric_fields: 0,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            };
        report_owner.selected_input_processor = Some(SelectedInputProcessor::from_admitted_input(
            vec![scheduled_definition.clone()],
            Default::default(),
        )?);
        let (mut parent_state, mut parent_error, mut parent_console, mut parent_eio) = (
            ctf_ordinary_output_test_state(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let mut parent = CtfInitializationContext {
            reports: &mut report_owner,
            eio: &mut parent_eio,
            backend: SourceErrorBackend {
                state: &mut parent_state,
                error_stream: Some(&mut parent_error),
                console: &mut parent_console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        };
        parent.continue_selected_heat_balance_input_before_solar()?;
        assert_eq!(
            parent
                .reports
                .input_processor_shortcut
                .current_module_object,
            "ComplexFenestrationProperty:SolarAbsorbedLayers"
        );
        assert_eq!(
            (
                parent
                    .reports
                    .surface_geometry_input
                    .total_incident_solar_multipliers,
                parent
                    .reports
                    .surface_geometry_input
                    .total_incident_solar_scheduled_gains,
                parent
                    .reports
                    .surface_geometry_input
                    .total_fenestration_absorbed_solar_scheduled_gains
            ),
            (0, 0, 0)
        );
        assert!(matches!(
            parent
                .reports
                .selected_input_processor
                .as_ref()
                .ok_or("same scheduled IP")?
                .family("SurfaceProperty:SolarIncidentInside"),
            Some(crate::heat_balance::selected_input_processor::SourceInputFamily::Null)
        ));
        assert_eq!(parent.backend.state.tracking.total_warning_errors, 1);
        assert!(!parent.reports.heat_balance_input_errors_found);
        // Authored checkpoint of the real early owner, not a Native numeric fixture.
        // The preceding report-only failure used an intentionally invalid row.
        parent.reports.surface_geometry_input = context.reports.surface_geometry_input.clone();
        parent.reports.ordinary_solar_radiation = Some(
            crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(
                radiation_input,
            ),
        );
        parent.initialize_selected_solar_view_factors(&model.typed)?;
        assert!(
            parent
                .reports
                .ordinary_solar_radiation
                .as_ref()
                .ok_or("zero-enclosure owner")?
                .enclosure_storage()
                .is_none(),
            "zero actual enclosure count preserves unallocated Solar"
        );
        parent.reports.ordinary_solar_radiation = source_solar;
        let solar_allocation = parent
            .reports
            .ordinary_solar_radiation
            .as_ref()
            .ok_or("same early Solar")?
            .enclosures()
            .as_ptr();
        parent.initialize_selected_solar_view_factors(&model.typed)?;
        let solar = parent
            .reports
            .ordinary_solar_radiation
            .as_ref()
            .ok_or("retained early Solar")?;
        let row = &solar.enclosures()[0];
        assert_eq!(solar.enclosures().as_ptr(), solar_allocation);
        assert!(solar.is_initialized());
        assert!(row.source_array_allocated.iter().all(|value| *value));
        assert_eq!(row.num_surfaces, i32::try_from(total)?);
        assert_eq!(
            row.source_surface_ptr
                .as_ref()
                .ok_or("actual Source pointers")?,
            &source_numbers
        );
        assert_eq!(
            row.source_surface_report_nums,
            parent
                .reports
                .surface_geometry_input
                .all_surface_list_report_order
        );
        assert!(
            row.surface_ptr.is_empty(),
            "physical IDs are joined only by the later binder"
        );
        assert!(row.absorptance.iter().all(|value| {
            value.to_bits()
                == parent.reports.heat_balance_input_state.construction_usage[0]
                    .inside_absorp_solar
                    .to_bits()
        }));
        assert!(
            parent
                .reports
                .surface_geometry_input
                .final_surfaces
                .as_ref()
                .ok_or("same final rows")?
                .iter()
                .enumerate()
                .all(|(index, surface)| (
                    surface.row.solar_encl_index,
                    surface.row.rad_encl_index,
                    surface.row.solar_encl_surface_index
                ) == (1, 1, i32::try_from(index + 1).unwrap_or(-1)))
        );
        let retained_f = row.f.clone();
        parent.initialize_selected_solar_view_factors(&model.typed)?;
        assert_eq!(
            parent
                .reports
                .ordinary_solar_radiation
                .as_ref()
                .ok_or("repeated same Source call")?
                .enclosures()[0]
                .source_surface_report_nums,
            parent
                .reports
                .surface_geometry_input
                .all_surface_list_report_order
                .iter()
                .chain(
                    &parent
                        .reports
                        .surface_geometry_input
                        .all_surface_list_report_order
                )
                .copied()
                .collect::<Vec<_>>()
        );
        let retained_f_allocation = parent
            .reports
            .ordinary_solar_radiation
            .as_ref()
            .ok_or("repeated F allocation")?
            .enclosures()[0]
            .f
            .as_ptr();
        parent
            .reports
            .ordinary_solar_radiation
            .as_mut()
            .ok_or("same late Solar")?
            .bind_source_initialized_solar_geometry(
                &model.typed,
                &source_solar_physical,
                radiation_input,
                &parent.reports.surface_geometry_input,
            )?;
        let row = &parent
            .reports
            .ordinary_solar_radiation
            .as_ref()
            .ok_or("late joined Solar")?
            .enclosures()[0];
        assert_eq!(
            row.f, retained_f,
            "late physical join preserves Source numeric writes"
        );
        assert_eq!(
            row.f.as_ptr(),
            retained_f_allocation,
            "late join retains the reached repeated-call F allocation"
        );
        assert_eq!(row.surface_ptr, source_ids);
        // Authored real-getter checkpoint, not a Native/World producer fixture.
        // Built-in Constant schedule is created by its genuine Source producer.
        let mut gains_schedule =
            crate::heat_balance::selected_schedule_manager::SelectedScheduleManager::default();
        gains_schedule.init_constant_state(4)?;
        parent.reports.selected_schedule_manager = Some(gains_schedule);
        parent
            .reports
            .selected_project_data
            .get_or_insert_with(Default::default);
        let zone_name = &mut parent
            .reports
            .selected_zone_input
            .as_mut()
            .and_then(|o| o.zones.as_mut())
            .ok_or("actual gain Zone")?[0]
            .name;
        zone_name.make_ascii_uppercase();
        let zone_name = zone_name.clone();
        let gain_objects = [7.0, -3.0]
            .into_iter()
            .enumerate()
            .map(|(i, level)| ProjectInputObject {
                object_name: format!("Authored gain {i}"),
                idf_max_fields: None,
                fields: std::collections::BTreeMap::from([
                    (
                        "zone_or_zonelist_or_space_or_spacelist_name".into(),
                        ProjectInputValue::String(zone_name.clone()),
                    ),
                    (
                        "schedule_name".into(),
                        ProjectInputValue::String("Constant-1.0".into()),
                    ),
                    ("design_level".into(), ProjectInputValue::Number(level)),
                    ("fraction_latent".into(), ProjectInputValue::Number(0.1)),
                    ("fraction_radiant".into(), ProjectInputValue::Number(0.2)),
                    ("fraction_lost".into(), ProjectInputValue::Number(0.3)),
                    (
                        "end_use_subcategory".into(),
                        ProjectInputValue::String("Mixed Case".into()),
                    ),
                ]),
            })
            .collect::<Vec<_>>();
        let definitions = [
            ("People", 21, 8),
            ("Lights", 8, 9),
            ("ElectricEquipment", 5, 6),
            ("GasEquipment", 5, 7),
            ("HotWaterEquipment", 5, 6),
            ("SteamEquipment", 5, 6),
            ("OtherEquipment", 6, 7),
            ("ElectricEquipment:ITE:AirCooled", 21, 11),
            ("ZoneBaseboard:OutdoorTemperatureControlled", 4, 5),
            ("ZoneContaminantSourceAndSink:CarbonDioxide", 3, 1),
        ]
        .into_iter()
        .map(
            |(name, a, n)| crate::heat_balance::selected_input_processor::SourceObjectDefinition {
                object_type: name.into(),
                alpha_fields: a,
                numeric_fields: n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            },
        )
        .collect();
        parent.reports.selected_input_processor =
            Some(SelectedInputProcessor::from_admitted_input(
                definitions,
                std::collections::BTreeMap::from([(
                    "OtherEquipment".into(),
                    crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(
                        gain_objects
                            .iter()
                            .map(|o| {
                                crate::heat_balance::selected_input_processor::SourceObjectShape {
                                    object_name: o.object_name.clone(),
                                    field_sizes: Default::default(),
                                }
                            })
                            .collect(),
                    ),
                )]),
            )?);
        let raw = parent
            .reports
            .heat_balance_input
            .get_or_insert_with(Default::default);
        raw.source_is_epjson = true;
        raw.objects.insert("OtherEquipment".into(), gain_objects);
        let mut failed_gains = parent.reports.clone();
        let shared_before = parent.reports.input_processor_shortcut.clone();
        let registrations_before = parent.reports.output_registration.num_real_variables_setup;
        let mut gain_host = EnclosureTestHost {
            calls: 0,
            fail: false,
        };
        parent.manage_selected_internal_heat_gains_input(&model.typed, 15, &mut gain_host)?;
        parent.check_selected_kiva_input_guard()?;
        let gains = &parent.reports.selected_internal_heat_gains;
        assert!(!gains.get_input_flag);
        assert!(!gains.errors_found);
        assert_eq!(gains.totals[6], 2);
        assert!(gains.lights_objects.as_ref().is_some_and(Vec::is_empty));
        assert!(
            gains
                .allocations
                .co2_generation
                .as_ref()
                .is_some_and(Vec::is_empty)
        );
        let equipment = gains
            .allocations
            .other_equipment
            .as_ref()
            .ok_or("same gain rows")?;
        assert_eq!(
            equipment.iter().map(|r| r.design_level).collect::<Vec<_>>(),
            [7.0, -3.0]
        );
        assert!(equipment.iter().all(|r| r.fraction_convected.to_bits()
            == (1.0_f64 - (0.1 + 0.2 + 0.3)).to_bits()
            && r.end_use_subcategory == "Mixed Case"
            && r.nominal_min_design_level.to_bits() == r.design_level.to_bits()));
        assert_ne!(
            equipment[0].fraction_convected.to_bits(),
            (1.0_f64 - 0.1 - 0.2 - 0.3).to_bits(),
            "Source grouping differs from unchanged hourly sequential subtraction"
        );
        let devices = &gains
            .allocations
            .space_devices
            .as_ref()
            .ok_or("actual gain devices")?[0];
        assert_eq!(
            (devices.number_of_devices, devices.max_number_of_devices),
            (2, 100)
        );
        let slots = devices.devices.as_ref().ok_or("actual100slots")?;
        assert_eq!(slots.len(), 100);
        assert_ne!(slots[0].targets[0], slots[1].targets[0]);
        assert_ne!(slots[0].targets[0], slots[0].targets[2]);
        assert_eq!(slots[0].targets[1], slots[0].targets[4]);
        assert!(slots[0].targets.iter().all(Option::is_some));
        assert!(slots.iter().all(|row| row.cached_rates == [0.0; 7]));
        assert_eq!(
            parent
                .reports
                .selected_zone_input
                .as_ref()
                .and_then(|o| o.zones.as_ref())
                .ok_or("same gain Zone")?[0]
                .internal_heat_gains
                .to_bits(),
            4.0_f64.to_bits()
        );
        assert_eq!(
            parent.reports.output_registration.num_real_variables_setup - registrations_before,
            60
        );
        assert_eq!(
            parent.reports.input_processor_shortcut, shared_before,
            "IHG has six LOCAL arrays"
        );
        let retained_gains = gains.clone();
        let retained_registration = parent.reports.output_registration.clone();
        parent.manage_selected_internal_heat_gains_input(&model.typed, 15, &mut gain_host)?;
        assert_eq!(parent.reports.selected_internal_heat_gains, retained_gains);
        assert_eq!(
            parent.reports.output_registration, retained_registration,
            "InitOnly repeats skip getter"
        );
        // Unit88 authored input checkpoint: the real Zone getter creates two
        // implicit Spaces, then the real IHG getter resolves opposite object/
        // Zone order. This is not a completed geometry/timestep certificate.
        {
            use crate::heat_balance::selected_heat_balance_input::zone::ZoneGetterOwners;
            use crate::heat_balance::selected_input_processor::{SourceInputFamily,SourceObjectDefinition,SourceObjectShape};
            use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainDictionaryFile;
            use crate::heat_balance::source_output_registration::{SourceInternalGainOutputField as GainField,SourceRealOutputTarget};
            let mut multi_model=model.typed.clone();
            multi_model.timestep.number_of_timesteps_per_hour=4;
            let template=multi_model.zones[0].clone();
            let space_template=multi_model.spaces[0].clone();
            multi_model.zones.clear();multi_model.spaces.clear();multi_model.zone_names=Default::default();
            multi_model.authored_space_names=Default::default();
            for (index,name) in ["ZULU","ALPHA"].into_iter().enumerate() {
                let mut zone=template.clone();zone.id=ZoneId(u32::try_from(index)?);
                zone.name=NormalizedName::new(name);zone.spaces=vec![ep_model::SpaceId(u32::try_from(index)?)];
                zone.inside_convection_algorithm=ep_model::ZoneConvectionAlgorithm::Override(template.inside_convection_algorithm.effective());
                zone.outside_convection_algorithm=ep_model::ZoneConvectionAlgorithm::Override(template.outside_convection_algorithm.effective());
                assert!(multi_model.zone_names.insert(name,zone.id).is_none());
                let mut space=space_template.clone();space.id=ep_model::SpaceId(u32::try_from(index)?);
                space.name=zone.name.clone();space.zone=zone.id;space.origin=ep_model::SpaceOrigin::AutoZoneDefault;
                multi_model.zones.push(zone);multi_model.spaces.push(space);
            }
            let requests=[
                ("ALPHA LOAD","Other Equipment Electricity Energy"),("ZULU LOAD","Other Equipment Electricity Energy"),
                ("ALPHA","Zone Other Equipment Total Heating Rate"),("ZULU","Zone Other Equipment Total Heating Rate"),
                ("ALPHA","Space Other Equipment Total Heating Rate"),("ZULU","Space Other Equipment Total Heating Rate"),
            ];
            multi_model.report_inputs.output_variables=requests.iter().enumerate().map(|(index,(key,name))|
                ep_model::SourceOutputVariableInput { object_name:format!("GAIN REQUEST {index}"),key:(*key).into(),
                    variable_name:(*name).into(),reporting_frequency:"TIMESTEP".into(),schedule_name:String::new(),
                    alpha_blank:[false,false,false,true],default_applied:[false;4],num_alphas:3,is_epjson:true }).collect();
            let zone_objects=["ZULU","ALPHA"].into_iter().map(|name|ProjectInputObject {
                object_name:name.into(),idf_max_fields:None,
                fields:std::collections::BTreeMap::from([
                    ("volume".into(),ProjectInputValue::Number(1.0)),
                    ("zone_inside_convection_algorithm".into(),ProjectInputValue::String("TARP".into())),
                    ("zone_outside_convection_algorithm".into(),ProjectInputValue::String("DOE-2".into())),
                    ("part_of_total_floor_area".into(),ProjectInputValue::String("Yes".into())),
                ]),
            }).collect::<Vec<_>>();
            let gain_objects=[("ALPHA LOAD","ALPHA",300.0),("ZULU LOAD","ZULU",100.0)].into_iter()
                .map(|(name,zone,power)|ProjectInputObject { object_name:name.into(),idf_max_fields:None,
                    fields:std::collections::BTreeMap::from([
                        ("zone_or_zonelist_or_space_or_spacelist_name".into(),ProjectInputValue::String(zone.into())),
                        ("schedule_name".into(),ProjectInputValue::String("Constant-1.0".into())),
                        ("fuel_type".into(),ProjectInputValue::String("Electricity".into())),
                        ("design_level_calculation_method".into(),ProjectInputValue::String("EquipmentLevel".into())),
                        ("design_level".into(),ProjectInputValue::Number(power)),
                        ("fraction_radiant".into(),ProjectInputValue::Number(0.25)),
                    ]) }).collect::<Vec<_>>();
            let raw=crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInput {
                source_is_epjson:true,objects:std::collections::BTreeMap::from([
                    ("Zone".into(),zone_objects.clone()),("OtherEquipment".into(),gain_objects.clone()),
                ]),..Default::default()
            };
            let mut definitions=Vec::new();
            let mut definition_input=failed_gains.selected_input_processor.clone().ok_or("actual IHG definitions")?;
            for family in ["People","Lights","ElectricEquipment","GasEquipment","HotWaterEquipment","SteamEquipment",
                "OtherEquipment","ElectricEquipment:ITE:AirCooled","ZoneBaseboard:OutdoorTemperatureControlled",
                "ZoneContaminantSourceAndSink:CarbonDioxide"] {
                let actual=definition_input.get_object_def_max_args(family)?;
                definitions.push(SourceObjectDefinition { object_type:family.into(),alpha_fields:actual.alphas,numeric_fields:actual.numerics,
                    alpha_extensions:0,numeric_extensions:0,extension:None });
            }
            definitions.push(SourceObjectDefinition { object_type:"Zone".into(),alpha_fields:4,numeric_fields:9,
                alpha_extensions:0,numeric_extensions:0,extension:None });
            definitions.push(SourceObjectDefinition { object_type:"Output:Variable".into(),alpha_fields:4,numeric_fields:0,
                alpha_extensions:0,numeric_extensions:0,extension:None });
            let shape=|rows:&[ProjectInputObject]|SourceInputFamily::Objects(rows.iter().map(|row|SourceObjectShape {
                object_name:row.object_name.clone(),field_sizes:Default::default() }).collect());
            let mut input=SelectedInputProcessor::from_admitted_input(definitions,std::collections::BTreeMap::from([
                ("Zone".into(),shape(&zone_objects)),("OtherEquipment".into(),shape(&gain_objects)),
                ("Output:Variable".into(),SourceInputFamily::Objects(multi_model.report_inputs.output_variables.iter().map(|row|
                    SourceObjectShape { object_name:row.object_name.clone(),field_sizes:Default::default() }).collect())),
            ]))?;
            let mut shortcut=SourceInputProcessorShortCut::default();shortcut.allocate_from_input(&input)?;
            let mut zones=SelectedZoneInputOwners::default();
            let mut globals=crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals::default();
            let mut output=crate::heat_balance::source_output_registration::SourceOutputRegistration::default();
            let mut prescan=crate::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            let (mut state,mut error,mut console,mut eio,mut eso)=(ctf_ordinary_output_test_state(),Vec::new(),Vec::new(),Vec::new(),Vec::new());
            state.output_error_header=false;
            let clock=crate::weather::day::WeatherGlobalState::default();
            let system=&failed_gains.selected_project_data.as_ref().ok_or("actual gain project")?.system;
            let developer=system.developer_flag;let minimum=system.minimum_report_frequency.clone();
            let mut backend=SourceErrorBackend { state:&mut state,error_stream:Some(&mut error),console:&mut console,
                out_unit1:None,out_unit2:None,f_message:None,message_callback:None,error_callback:None,sqlite:None };
            for (key,name) in requests { prescan.add_record(key,name,&mut backend)?; }
            zones.get_zone_data(&raw,&failed_gains.heat_balance_input_state,&multi_model,&mut ZoneGetterOwners {
                input:&mut input,shortcut:&mut shortcut,globals:&mut globals,output:&mut output,prescan:&prescan,
                report_input:&multi_model.report_inputs,minimum_report_frequency:&minimum,developer:&developer,clock:Some(&clock),
                backend:&mut backend,minutes_in_time_step:15,host:&mut gain_host,eio:&mut eio,
            }, &mut failed_gains.heat_balance_input_errors_found)?;
            assert_eq!((globals.num_of_zones,globals.num_spaces),(2,2));
            let retained=zones.zones.as_ref().ok_or("actual multi gain Zones")?;
            assert_eq!(retained.iter().map(|row|row.name.as_str()).collect::<Vec<_>>(),["ZULU","ALPHA"]);
            assert_eq!(zones.spaces.as_ref().ok_or("actual multi gain Spaces")?.iter().map(|row|row.zone_num).collect::<Vec<_>>(),[1,2]);
            let mut multi=failed_gains.clone();
            multi.selected_zone_input=Some(zones);multi.selected_heat_balance_globals=Some(globals);
            multi.selected_input_processor=Some(input);multi.input_processor_shortcut=shortcut;
            multi.heat_balance_input=Some(raw);multi.output_registration=output;multi.output_prescan=prescan;
            let mut context=CtfInitializationContext { reports:&mut multi,eio:&mut eio,backend };
            context.manage_selected_internal_heat_gains_input_with_dictionary(&multi_model,15,&mut gain_host,Some(&clock),
                Some(SourceGainDictionaryFile { eso:Some(&mut eso) }))?;
            let gains=&context.reports.selected_internal_heat_gains;
            assert!(!gains.get_input_flag && !gains.errors_found);assert_eq!(gains.totals[6],2);
            assert_eq!(gains.allocations.zone_report.as_ref().ok_or("actual two gain reports")?.len(),2);
            assert_eq!(gains.allocations.space_report.as_ref().ok_or("actual two Space reports")?.len(),2);
            let equipment=gains.allocations.other_equipment.as_ref().ok_or("actual two OtherEquipment rows")?;
            assert_eq!(equipment.iter().map(|row|(row.name.as_str(),row.zone_ptr,row.space_index,row.design_level))
                .collect::<Vec<_>>(),[("ALPHA LOAD",2,2,300.0),("ZULU LOAD",1,1,100.0)]);
            assert!(equipment.iter().all(|row|row.fuel as i32==0 && row.fraction_radiant.to_bits()==0.25_f64.to_bits()));
            let devices=gains.allocations.space_devices.as_ref().ok_or("actual separate Space devices")?;
            assert_eq!(devices.len(),2);
            for (space,name) in [(0,"ZULU LOAD"),(1,"ALPHA LOAD")] {
                let row=&devices[space];assert_eq!(row.number_of_devices,1);
                let device=&row.devices.as_ref().ok_or("actual per-Space gain target")?[0];
                assert_eq!(device.component_name,name);assert!(device.targets.iter().all(Option::is_some));
            }
            assert_ne!(devices[0].devices.as_ref().ok_or("actual first target")?[0].targets[0],
                devices[1].devices.as_ref().ok_or("actual second target")?[0].targets[0]);
            let zones=context.reports.selected_zone_input.as_ref().ok_or("retained gain associations")?;
            assert_eq!(zones.zones.as_ref().ok_or("retained gain Zones")?.iter().map(|row|row.internal_heat_gains).collect::<Vec<_>>(),[100.0,300.0]);
            assert!(zones.zones.as_ref().ok_or("actual fuel Zones")?.iter().all(|row|row.other_equipment_fuels==[0]));
            assert!(zones.spaces.as_ref().ok_or("actual fuel Spaces")?.iter().all(|row|row.other_equipment_fuels==[0]));
            let registration=&context.reports.output_registration;
            for (key,name,index,field) in [
                ("ALPHA LOAD","Other Equipment Electricity Energy",1,GainField::OtherConsumption),
                ("ZULU LOAD","Other Equipment Electricity Energy",2,GainField::OtherConsumption),
                ("ALPHA","Zone Other Equipment Total Heating Rate",2,GainField::ZoneOtherThermal { field:9 }),
                ("ZULU","Zone Other Equipment Total Heating Rate",1,GainField::ZoneOtherThermal { field:9 }),
                ("ALPHA","Space Other Equipment Total Heating Rate",2,GainField::SpaceOtherThermal { field:9 }),
                ("ZULU","Space Other Equipment Total Heating Rate",1,GainField::SpaceOtherThermal { field:9 }),
            ] {
                let matches=registration.out_vars.iter().filter(|row|row.key==key && row.name==name).collect::<Vec<_>>();
                assert_eq!(matches.len(),1,"{key}:{name}");assert!(matches[0].report);
                assert_eq!(matches[0].which,Some(SourceRealOutputTarget::InternalGain { index,field }));
            }
            for (name,expected) in [("ELECTRICITY:FACILITY",vec!["ALPHA LOAD","ZULU LOAD"]),
                ("ELECTRICITY:ZONE:ALPHA",vec!["ALPHA LOAD"]),("ELECTRICITY:ZONE:ZULU",vec!["ZULU LOAD"])] {
                let number=*registration.meters.meter_map.get(name).ok_or("genuine electrical meter")?;
                let meter=&registration.meters.meters[number];
                let keys=meter.source_variable_numbers.iter().map(|&number|registration.out_vars[number].key.as_str()).collect::<Vec<_>>();
                assert_eq!(keys,expected,"actual {name} membership");
            }
            assert_eq!(context.backend.state.tracking.total_warning_errors,0);assert_eq!(context.backend.state.tracking.total_severe_errors,0);
            let retained_gains=context.reports.selected_internal_heat_gains.clone();
            let retained_registration=context.reports.output_registration.clone();
            context.manage_selected_internal_heat_gains_input_with_dictionary(&multi_model,15,&mut gain_host,Some(&clock),
                Some(SourceGainDictionaryFile { eso:Some(&mut eso) }))?;
            assert_eq!(context.reports.selected_internal_heat_gains,retained_gains);
            assert_eq!(context.reports.output_registration,retained_registration);
            assert!(error.is_empty());
        }
        // Genuine dictionary capability over the same gain getter: the stream
        // can fail, or actually be closed, without moving SQL/RF owners out.
        for dictionary_case in 0..4 {
            use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainDictionaryFile;
            use crate::heat_balance::selected_input_processor::{
                SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
            };
            let mut requested_model = model.typed.clone();
            requested_model.report_inputs.output_variables =
                vec![ep_model::SourceOutputVariableInput {
                    object_name: "Authored gain request".into(),
                    key: "AUTHORED GAIN 0".into(),
                    variable_name: "Other Equipment Electricity Rate".into(),
                    reporting_frequency: "TIMESTEP".into(),
                    schedule_name: String::new(),
                    alpha_blank: [false, false, false, true],
                    default_applied: [false; 4],
                    num_alphas: 3,
                    is_epjson: true,
                }];
            let mut requested = failed_gains.clone();
            requested.output_registration = Default::default(); // genuine fresh constructor for this authored request
            requested
                .heat_balance_input
                .as_mut()
                .ok_or("requested raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("requested gains")?[0]
                .fields
                .insert(
                    "fuel_type".into(),
                    ProjectInputValue::String("Electricity".into()),
                );
            let mut actual_ip = requested
                .selected_input_processor
                .take()
                .ok_or("requested IP")?;
            let mut definitions = Vec::new();
            for family in [
                "People",
                "Lights",
                "ElectricEquipment",
                "GasEquipment",
                "HotWaterEquipment",
                "SteamEquipment",
                "OtherEquipment",
                "ElectricEquipment:ITE:AirCooled",
                "ZoneBaseboard:OutdoorTemperatureControlled",
                "ZoneContaminantSourceAndSink:CarbonDioxide",
            ] {
                let actual = actual_ip.get_object_def_max_args(family)?;
                definitions.push(SourceObjectDefinition {
                    object_type: family.into(),
                    alpha_fields: actual.alphas,
                    numeric_fields: actual.numerics,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                });
            }
            definitions.push(SourceObjectDefinition {
                object_type: "Output:Variable".into(),
                alpha_fields: 4,
                numeric_fields: 0,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            });
            requested.selected_input_processor = Some(SelectedInputProcessor::from_admitted_input(
                definitions,
                std::collections::BTreeMap::from([
                    (
                        "OtherEquipment".into(),
                        actual_ip
                            .family("OtherEquipment")
                            .ok_or("authored family")?
                            .clone(),
                    ),
                    (
                        "Output:Variable".into(),
                        SourceInputFamily::Objects(vec![SourceObjectShape {
                            object_name: "Authored gain request".into(),
                            field_sizes: Default::default(),
                        }]),
                    ),
                ]),
            )?);
            let (mut state, mut error, mut console, mut eio, mut eso) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            let mut failing = OrdinaryFailingEio;
            let file = match dictionary_case {
                0 => Some(SourceGainDictionaryFile {
                    eso: Some(&mut eso),
                }),
                1 => Some(SourceGainDictionaryFile {
                    eso: Some(&mut failing),
                }),
                2 => Some(SourceGainDictionaryFile { eso: None }),
                _ => None,
            };
            let mut requested_context = CtfInitializationContext {
                reports: &mut requested,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut error),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            };
            requested_context.reports.output_prescan.add_record(
                "AUTHORED GAIN 0", "Other Equipment Electricity Rate",
                &mut requested_context.backend,
            )?;
            requested_context.reports.source_results_output_variables
                .push("actual prior RF entry".into());
            let sqlite_before = requested_context.reports.source_sqlite.clone();
            let outcome = requested_context.manage_selected_internal_heat_gains_input_with_dictionary(
                &requested_model,
                15,
                &mut gain_host,
                None,
                file,
            );
            let succeeded = matches!(dictionary_case, 0 | 2);
            if succeeded {
                outcome?;
            } else if dictionary_case == 1 {
                assert!(
                    matches!(outcome,Err(crate::RuntimeError::OrdinaryRadiation(error))
                    if matches!(*error,crate::heat_balance::radiation::ordinary::OrdinaryRadiationError::Output(crate::error::WarmupOutputError::Io(_))))
                );
            } else {
                assert!(
                    matches!(outcome,Err(crate::RuntimeError::OrdinaryRadiation(error))
                    if matches!(*error,crate::heat_balance::radiation::ordinary::OrdinaryRadiationError::Unavailable(crate::heat_balance::radiation::ordinary::OrdinaryRadiationUnavailable::InputBranch)))
                );
            }
            assert_eq!(requested.source_sqlite, sqlite_before);
            assert_eq!(
                requested.source_results_output_variables[0],
                "actual prior RF entry"
            );
            assert_eq!(
                requested.source_results_output_variables.len(),
                if succeeded { 2 } else { 1 }
            );
            assert_eq!(
                requested.selected_internal_heat_gains.get_input_flag,
                !succeeded
            );
            assert!(!requested.selected_internal_heat_gains.errors_found);
            let rate = requested
                .output_registration
                .out_vars
                .iter()
                .find(|v| v.name == "Other Equipment Electricity Rate")
                .ok_or("actual requested bound rate")?;
            assert!(rate.which.is_some() && rate.report);
            if dictionary_case == 0 {
                assert!(
                    String::from_utf8(eso)?
                        .contains("AUTHORED GAIN 0,Other Equipment Electricity Rate [W] !TimeStep")
                );
                assert!(std::str::from_utf8(&eio)?.contains("OtherEquipment Internal Gains Nominal"));
            } else {
                assert!(eso.is_empty());
            }
            if !succeeded {
                assert!(eio.is_empty(), "failed dictionary precedes nominal writer");
            }
        }
        // Every reached OtherEquipment/aggregate pointer is requested through the real
        // dictionary and then resolved after changes to the same retained report owner.
        {
            use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainDictionaryFile;
            use crate::heat_balance::selected_input_processor::{
                SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
            };
            use crate::heat_balance::source_output_registration::{
                SourceInternalGainOutputField as GainField, SourceRealOutputOwners,
                SourceRealOutputTarget,
            };
            let mut requested = failed_gains.clone();
            requested.output_registration = Default::default();
            requested
                .heat_balance_input
                .as_mut()
                .ok_or("bound gain raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("bound gains")?[0]
                .fields
                .insert(
                    "fuel_type".into(),
                    ProjectInputValue::String("Electricity".into()),
                );
            let zones = requested
                .selected_zone_input
                .as_ref()
                .ok_or("bound actual Zones")?;
            let zone_key = zones.zones.as_ref().ok_or("bound actual Zone row")?[0]
                .name
                .clone();
            let space_key = zones.spaces.as_ref().ok_or("bound implicit Space row")?[0]
                .name
                .clone();
            let mut expected = Vec::<(String, String, f64)>::new();
            let total_names = [
                "Radiant Heating",
                "Visible Radiation Heating",
                "Convective Heating",
                "Latent Gain",
                "Total Heating",
            ];
            let other_names = [
                "Radiant Heating",
                "Convective Heating",
                "Latent Gain",
                "Lost Heat",
                "Total Heating",
            ];
            // Exact Native order: all Zone totals, all actual Spaces (including the
            // implicit Space), object fields, flagged Zone fuels/thermal, then Space.
            for (key, prefix, base) in [(&zone_key, "Zone", 500.0), (&space_key, "Space", 600.0)] {
                for (at, suffix) in total_names.into_iter().enumerate() {
                    expected.push((
                        key.clone(),
                        format!("{prefix} Total Internal {suffix} Energy"),
                        base + (at * 2) as f64,
                    ));
                    expected.push((
                        key.clone(),
                        format!("{prefix} Total Internal {suffix} Rate"),
                        base + (at * 2 + 1) as f64,
                    ));
                }
            }
            expected.push((
                "AUTHORED GAIN 0".into(),
                "Other Equipment Electricity Rate".into(),
                100.0,
            ));
            expected.push((
                "AUTHORED GAIN 0".into(),
                "Other Equipment Electricity Energy".into(),
                200.0,
            ));
            for (at, suffix) in other_names.into_iter().enumerate() {
                expected.push((
                    "AUTHORED GAIN 0".into(),
                    format!("Other Equipment {suffix} Energy"),
                    201.0 + at as f64,
                ));
                expected.push((
                    "AUTHORED GAIN 0".into(),
                    format!("Other Equipment {suffix} Rate"),
                    101.0 + at as f64,
                ));
            }
            for (key, prefix, base, fuel_rate, fuel_energy) in [
                (&zone_key, "Zone", 300.0, 701.0, 702.0),
                (&space_key, "Space", 400.0, 801.0, 802.0),
            ] {
                expected.push((
                    key.clone(),
                    format!("{prefix} Other Equipment Electricity Rate"),
                    fuel_rate,
                ));
                expected.push((
                    key.clone(),
                    format!("{prefix} Other Equipment Electricity Energy"),
                    fuel_energy,
                ));
                for (at, suffix) in other_names.into_iter().enumerate() {
                    expected.push((
                        key.clone(),
                        format!("{prefix} Other Equipment {suffix} Energy"),
                        base + (at * 2) as f64,
                    ));
                    expected.push((
                        key.clone(),
                        format!("{prefix} Other Equipment {suffix} Rate"),
                        base + (at * 2 + 1) as f64,
                    ));
                }
            }
            assert_eq!(expected.len(), 56);
            let mut requested_model = model.typed.clone();
            requested_model.report_inputs.output_variables = expected
                .iter()
                .enumerate()
                .map(|(at, (key, name, _))| ep_model::SourceOutputVariableInput {
                    object_name: format!("Actual gain pointer {at}"),
                    key: key.clone(),
                    variable_name: name.clone(),
                    reporting_frequency: "TIMESTEP".into(),
                    schedule_name: String::new(),
                    alpha_blank: [false, false, false, true],
                    default_applied: [false; 4],
                    num_alphas: 3,
                    is_epjson: true,
                })
                .collect();
            let mut original_ip = requested
                .selected_input_processor
                .take()
                .ok_or("bound actual IP")?;
            let mut definitions = Vec::new();
            for family in [
                "People",
                "Lights",
                "ElectricEquipment",
                "GasEquipment",
                "HotWaterEquipment",
                "SteamEquipment",
                "OtherEquipment",
                "ElectricEquipment:ITE:AirCooled",
                "ZoneBaseboard:OutdoorTemperatureControlled",
                "ZoneContaminantSourceAndSink:CarbonDioxide",
            ] {
                let actual = original_ip.get_object_def_max_args(family)?;
                definitions.push(SourceObjectDefinition {
                    object_type: family.into(),
                    alpha_fields: actual.alphas,
                    numeric_fields: actual.numerics,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                });
            }
            definitions.push(SourceObjectDefinition {
                object_type: "Output:Variable".into(),
                alpha_fields: 4,
                numeric_fields: 0,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            });
            requested.selected_input_processor = Some(SelectedInputProcessor::from_admitted_input(
                definitions,
                std::collections::BTreeMap::from([
                    (
                        "OtherEquipment".into(),
                        original_ip
                            .family("OtherEquipment")
                            .ok_or("same actual gain family")?
                            .clone(),
                    ),
                    (
                        "Output:Variable".into(),
                        SourceInputFamily::Objects(
                            requested_model
                                .report_inputs
                                .output_variables
                                .iter()
                                .map(|row| SourceObjectShape {
                                    object_name: row.object_name.clone(),
                                    field_sizes: Default::default(),
                                })
                                .collect(),
                        ),
                    ),
                ]),
            )?);
            let (mut state, mut error, mut console, mut eio, mut eso) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            let mut requested_context = CtfInitializationContext {
                reports: &mut requested,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut error),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            };
            for (key, name, _) in &expected {
                requested_context.reports.output_prescan.add_record(
                    key, name, &mut requested_context.backend,
                )?;
            }
            requested_context.manage_selected_internal_heat_gains_input_with_dictionary(
                &requested_model,
                15,
                &mut gain_host,
                None,
                Some(SourceGainDictionaryFile {
                    eso: Some(&mut eso),
                }),
            )?;
            assert!(!requested.selected_internal_heat_gains.get_input_flag);
            assert_eq!(requested.output_registration.out_vars.len(), expected.len());
            assert_eq!(requested.output_registration.num_real_variables_setup, 66);
            assert!(error.is_empty());
            assert!(std::str::from_utf8(&eio)?.contains("OtherEquipment Internal Gains Nominal"));
            let before = requested.output_registration.out_vars.iter().map(|row| {
            requested.selected_internal_heat_gains.real_output(row.which.ok_or(crate::heat_balance::radiation::ordinary::OrdinaryRadiationUnavailable::StorageDomain)?)
                .copied()
        }).collect::<Result<Vec<_>, _>>()?;
            // Authored mutations prove live identities only, not timestep arithmetic.
            // They never stand in for a successful parent timestep or physical proof.
            let gains = &mut requested.selected_internal_heat_gains;
            let object = &mut gains
                .allocations
                .other_equipment
                .as_mut()
                .ok_or("actual bound object")?[0];
            object.rates = [100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 0.0];
            object.energies = [200.0, 201.0, 202.0, 203.0, 204.0, 205.0];
            let zone = &mut gains
                .allocations
                .zone_report
                .as_mut()
                .ok_or("actual bound Zone report")?[0];
            zone.other = std::array::from_fn(|i| 300.0 + i as f64);
            zone.total = std::array::from_fn(|i| 500.0 + i as f64);
            zone.other_power[0] = 701.0;
            zone.other_consumption[0] = 702.0;
            let space = &mut gains
                .allocations
                .space_report
                .as_mut()
                .ok_or("actual bound Space report")?[0];
            space.other = std::array::from_fn(|i| 400.0 + i as f64);
            space.total = std::array::from_fn(|i| 600.0 + i as f64);
            space.other_power[0] = 801.0;
            space.other_consumption[0] = 802.0;
            let dictionary = std::str::from_utf8(&eso)?;
            for (at, (actual, (key, name, expected_value))) in requested
                .output_registration
                .out_vars
                .iter()
                .zip(&expected)
                .enumerate()
            {
                assert_eq!(
                    (&actual.key, &actual.name),
                    (key, name),
                    "Native declaration order/key {at}"
                );
                assert_eq!(
                    *gains.real_output(actual.which.ok_or("actual live Which")?)?,
                    *expected_value
                );
                assert_ne!(before[at], *expected_value);
                assert!(actual.report);
                assert!(dictionary.contains(&format!(
                    "{key},{name} [{}] !TimeStep",
                    if name.ends_with("Energy") { "J" } else { "W" }
                )));
            }
            for field in [
                GainField::OtherThermal { field: 10 },
                GainField::ZoneOtherThermal { field: 10 },
                GainField::SpaceOtherThermal { field: 10 },
                GainField::ZoneTotalInternal { field: 10 },
                GainField::SpaceTotalInternal { field: 10 },
                GainField::ZoneOtherFuel {
                    fuel: 13,
                    energy: false,
                },
                GainField::SpaceOtherFuel {
                    fuel: 14,
                    energy: true,
                },
            ] {
                assert!(
                    gains
                        .real_output(SourceRealOutputTarget::InternalGain { field, index: 1 })
                        .is_err()
                );
            }
        }
        // Positive direct People uses the real schedule producer and IHG getter.
        // Later explicit child checks do not turn the genuine Refrig stop into
        // a successful parent. All operands here are authored unit inputs.
        {
            use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::{
                SourceGainDictionaryFile, SourceGainStepOwners, SourceZoneGenericGainAllocation,
            };
            use crate::heat_balance::selected_input_processor::{
                SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
            };
            use crate::heat_balance::selected_schedule_manager::{
                SelectedScheduleInput, SelectedScheduleManager, SourceConstantScheduleInput,
                SourceScheduleTypeInput,
            };
            use crate::heat_balance::source_output_registration::SourceRealOutputOwners;
            struct PeopleAudit(Vec<u8>);
            impl crate::heat_balance::source_initialization::ScheduleAuditFileHost for PeopleAudit {
                fn audit_path(&self) -> &std::path::Path {
                    std::path::Path::new("authored People audit")
                }
                fn ensure_audit_open(&mut self) -> std::io::Result<&mut dyn std::io::Write> {
                    Ok(&mut self.0)
                }
            }
            // The earlier geometry fixture deliberately renamed its Zone for
            // nominal-output checks. This fresh raw getter uses the same typed
            // Zone identity instead of inheriting that authored display name.
            let zone_name = model.typed.zones[0].name.0.clone();
            for (count_token, expected_count) in
                [("1.5", 1.5_f64), ("0", 0.0), ("18446744073709551615", -1.0)]
            {
                let mut people_reports = failed_gains.clone();
                people_reports.output_registration = Default::default();
                let projected = ep_model::classify_source_json_number(count_token)
                    .map_err(|_| "authored valid People JSON number")?
                    .as_f64();
                assert_eq!(projected.to_bits(), expected_count.to_bits());
                let people_objects = [1.5, projected]
                    .into_iter()
                    .enumerate()
                    .map(|(i, count)| ProjectInputObject {
                        object_name: format!("People {i}"),
                        idf_max_fields: None,
                        fields: std::collections::BTreeMap::from([
                            (
                                "zone_or_zonelist_or_space_or_spacelist_name".into(),
                                ProjectInputValue::String(zone_name.clone()),
                            ),
                            (
                                "number_of_people_schedule_name".into(),
                                ProjectInputValue::String("Constant-1.0".into()),
                            ),
                            ("number_of_people".into(), ProjectInputValue::Number(count)),
                            ("fraction_radiant".into(), ProjectInputValue::Number(0.3)),
                            (
                                "sensible_heat_fraction".into(),
                                ProjectInputValue::Number(0.6),
                            ),
                            (
                                "activity_level_schedule_name".into(),
                                ProjectInputValue::String("Activity".into()),
                            ),
                            (
                                "enable_ashrae_55_comfort_warnings".into(),
                                ProjectInputValue::String("Yes".into()),
                            ),
                        ]),
                    })
                    .collect::<Vec<_>>();
                let mut definitions = Vec::new();
                let mut old_ip = people_reports
                    .selected_input_processor
                    .take()
                    .ok_or("People real schema")?;
                for family in [
                    "People",
                    "Lights",
                    "ElectricEquipment",
                    "GasEquipment",
                    "HotWaterEquipment",
                    "SteamEquipment",
                    "OtherEquipment",
                    "ElectricEquipment:ITE:AirCooled",
                    "ZoneBaseboard:OutdoorTemperatureControlled",
                    "ZoneContaminantSourceAndSink:CarbonDioxide",
                ] {
                    let actual = old_ip.get_object_def_max_args(family)?;
                    definitions.push(SourceObjectDefinition {
                        object_type: family.into(),
                        alpha_fields: actual.alphas,
                        numeric_fields: actual.numerics,
                        alpha_extensions: 0,
                        numeric_extensions: 0,
                        extension: None,
                    });
                }
                for (name, a, n) in [
                    ("ScheduleTypeLimits", 3, 2),
                    ("Schedule:Constant", 2, 1),
                    ("Output:Schedules", 1, 0),
                    ("Output:Variable", 4, 0),
                ] {
                    definitions.push(SourceObjectDefinition {
                        object_type: name.into(),
                        alpha_fields: a,
                        numeric_fields: n,
                        alpha_extensions: 0,
                        numeric_extensions: 0,
                        extension: None,
                    });
                }
                // Exact official schema definitions for the genuine Zone and
                // tabular getter prefixes, including absent report families.
                for (name, a, n, ae, ne, extension) in [
                    ("Zone", 4, 9, 0, 0, None),
                    ("ZoneList", 1, 0, 1, 0, Some("zones")),
                    ("ZoneGroup", 2, 1, 0, 0, None),
                    ("ZoneProperty:LocalEnvironment", 3, 0, 0, 0, None),
                    ("ZoneHVAC:EquipmentConnections", 8, 0, 0, 0, None),
                    ("Space", 3, 3, 1, 0, Some("tags")),
                    ("SpaceList", 1, 0, 1, 0, Some("spaces")),
                    ("Output:Table:SummaryReports", 0, 0, 1, 0, Some("reports")),
                    ("Output:Table:Monthly", 1, 1, 2, 0, Some("variable_details")),
                    ("Output:Table:Annual", 3, 0, 2, 1, Some("variable_details")),
                    ("Output:Table:TimeBins", 4, 3, 0, 0, None),
                    ("OutputControl:Table:Style", 3, 0, 0, 0, None),
                ] {
                    definitions.push(SourceObjectDefinition {
                        object_type: name.into(),
                        alpha_fields: a,
                        numeric_fields: n,
                        alpha_extensions: ae,
                        numeric_extensions: ne,
                        extension: extension.map(str::to_owned),
                    });
                }
                let shape = |name: &str| SourceObjectShape {
                    object_name: name.into(),
                    field_sizes: Default::default(),
                };
                let mut people_ip = SelectedInputProcessor::from_admitted_input(
                    definitions,
                    std::collections::BTreeMap::from([
                        (
                            "People".into(),
                            SourceInputFamily::Objects(
                                people_objects
                                    .iter()
                                    .map(|p| shape(&p.object_name))
                                    .collect(),
                            ),
                        ),
                        (
                            "Zone".into(),
                            SourceInputFamily::Objects(vec![shape(&zone_name)]),
                        ),
                        ("Space".into(), SourceInputFamily::Objects(Vec::new())),
                        (
                            "Output:Table:SummaryReports".into(),
                            SourceInputFamily::Objects(vec![SourceObjectShape {
                                object_name: "People comfort summary".into(),
                                field_sizes: std::collections::BTreeMap::from([(
                                    "reports".into(),
                                    1,
                                )]),
                            }]),
                        ),
                        (
                            "ScheduleTypeLimits".into(),
                            SourceInputFamily::Objects(vec![shape("Activity Type")]),
                        ),
                        (
                            "Schedule:Constant".into(),
                            SourceInputFamily::Objects(vec![
                                shape("Activity"),
                                shape("Unused People Auxiliary"),
                                shape("Fanger Method Two"),
                            ]),
                        ),
                        (
                            "Output:Variable".into(),
                            SourceInputFamily::Objects(vec![shape("People rate request")]),
                        ),
                    ]),
                )?;
                let mut people_schedules = SelectedScheduleManager::default();
                people_schedules.init_constant_state(4)?;
                people_schedules.process_schedule_input(
                    &mut people_ip,
                    &SelectedScheduleInput {
                        source_is_epjson: Some(true),
                        types: vec![SourceScheduleTypeInput {
                            object_name: "Activity Type".into(),
                            name: "ACTIVITY TYPE".into(),
                            lower: None,
                            upper: None,
                            numeric_type: None,
                            unit: ep_model::ScheduleUnitType::ActivityLevel,
                        }],
                        constants: vec![
                            SourceConstantScheduleInput {
                                object_name: "Activity".into(),
                                name: "ACTIVITY".into(),
                                type_name: Some("ACTIVITY TYPE".into()),
                                value: 120.0,
                            },
                            SourceConstantScheduleInput {
                                object_name: "Unused People Auxiliary".into(),
                                name: "UNUSED PEOPLE AUXILIARY".into(),
                                type_name: Some("ACTIVITY TYPE".into()),
                                value: 0.125,
                            },
                            SourceConstantScheduleInput {
                                object_name: "Fanger Method Two".into(),
                                name: "FANGER METHOD TWO".into(),
                                type_name: Some("ACTIVITY TYPE".into()),
                                value: 2.0,
                            },
                        ],
                    },
                    4,
                    &mut PeopleAudit(Vec::new()),
                )?;
                people_reports.selected_input_processor = Some(people_ip);
                people_reports.selected_schedule_manager = Some(people_schedules);
                let raw = people_reports
                    .heat_balance_input
                    .as_mut()
                    .ok_or("People raw")?;
                raw.objects.remove("OtherEquipment");
                raw.objects.insert("People".into(), people_objects);
                let mut people_model = model.typed.clone();
                people_model.report_inputs.output_variables =
                    vec![ep_model::SourceOutputVariableInput {
                        object_name: "People rate request".into(),
                        key: "PEOPLE 0".into(),
                        variable_name: "People Total Heating Rate".into(),
                        reporting_frequency: "TIMESTEP".into(),
                        schedule_name: String::new(),
                        alpha_blank: [false, false, false, true],
                        default_applied: [false; 4],
                        num_alphas: 3,
                        is_epjson: true,
                    }];
                let (
                    mut people_state,
                    mut people_errors,
                    mut people_console,
                    mut people_eio,
                    mut people_eso,
                ) = (
                    ctf_ordinary_output_test_state(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                );
                {
                    let mut people_context = CtfInitializationContext {
                        reports: &mut people_reports,
                        eio: &mut people_eio,
                        backend: SourceErrorBackend {
                            state: &mut people_state,
                            error_stream: Some(&mut people_errors),
                            console: &mut people_console,
                            out_unit1: None,
                            out_unit2: None,
                            f_message: None,
                            message_callback: None,
                            error_callback: None,
                            sqlite: None,
                        },
                    };
                    people_context.reports.output_prescan.add_record(
                        "PEOPLE 0", "People Total Heating Rate", &mut people_context.backend,
                    )?;
                }
                let people_clock = crate::weather::day::WeatherGlobalState {
                    time_steps_in_hour: 4,
                    time_step_zone: 0.25,
                    time_step_zone_sec: 900.0,
                    ..Default::default()
                };
                // Invoke the real GetZoneData child on fresh constructor owners.
                // Its resilience/predefined allocations are not hand-set flags;
                // this explicit child fixture does not certify a full geometry run.
                let zone = &people_model.zones[0];
                let auto = |value| match value {
                    ep_model::AutoOrNumber::AutoCalculate => {
                        ProjectInputValue::String("Autocalculate".into())
                    }
                    ep_model::AutoOrNumber::Value(value) => ProjectInputValue::Number(value),
                };
                let outside = zone.outside_convection_algorithm.effective();
                let outside = if outside == ep_model::OutsideSurfaceConvectionAlgorithm::Doe2 {
                    "DOE-2".into()
                } else {
                    format!("{outside:?}")
                };
                people_reports
                    .heat_balance_input
                    .as_mut()
                    .ok_or("People Zone raw owner")?
                    .objects
                    .insert(
                        "Zone".into(),
                        vec![ProjectInputObject {
                            object_name: zone_name.clone(),
                            idf_max_fields: None,
                            fields: std::collections::BTreeMap::from([
                                (
                                    "direction_of_relative_north".into(),
                                    ProjectInputValue::Number(zone.direction_of_relative_north_deg),
                                ),
                                (
                                    "x_origin".into(),
                                    ProjectInputValue::Number(zone.origin.x_m),
                                ),
                                (
                                    "y_origin".into(),
                                    ProjectInputValue::Number(zone.origin.y_m),
                                ),
                                (
                                    "z_origin".into(),
                                    ProjectInputValue::Number(zone.origin.z_m),
                                ),
                                (
                                    "multiplier".into(),
                                    ProjectInputValue::Number(f64::from(zone.multiplier)),
                                ),
                                ("ceiling_height".into(), auto(zone.ceiling_height)),
                                ("volume".into(), auto(zone.volume)),
                                ("floor_area".into(), auto(zone.floor_area)),
                                (
                                    "zone_inside_convection_algorithm".into(),
                                    ProjectInputValue::String(format!(
                                        "{:?}",
                                        zone.inside_convection_algorithm.effective()
                                    )),
                                ),
                                (
                                    "zone_outside_convection_algorithm".into(),
                                    ProjectInputValue::String(outside),
                                ),
                                (
                                    "part_of_total_floor_area".into(),
                                    ProjectInputValue::String(
                                        if zone.is_part_of_total_floor_area {
                                            "Yes"
                                        } else {
                                            "No"
                                        }
                                        .into(),
                                    ),
                                ),
                            ]),
                        }],
                    );
                let mut people_zones = crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners::default();
                let mut people_zone_globals = crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals::default();
                let input = people_reports
                    .selected_input_processor
                    .as_mut()
                    .ok_or("People Zone IP")?;
                people_reports
                    .input_processor_shortcut
                    .allocate_from_input(input)?;
                let mut zone_backend = SourceErrorBackend {
                    state: &mut people_state, error_stream: Some(&mut people_errors), console: &mut people_console,
                    out_unit1: None, out_unit2: None, f_message: None, message_callback: None, error_callback: None, sqlite: None,
                };
                people_zones.get_zone_data(
                    people_reports
                        .heat_balance_input
                        .as_ref()
                        .ok_or("People Zone raw")?,
                    &people_reports.heat_balance_input_state,
                    &people_model,
                    &mut crate::heat_balance::selected_heat_balance_input::zone::ZoneGetterOwners {
                        input,
                        shortcut: &mut people_reports.input_processor_shortcut,
                        globals: &mut people_zone_globals,
                        output: &mut people_reports.output_registration,
                        prescan: &people_reports.output_prescan,
                        report_input: &people_model.report_inputs,
                        minimum_report_frequency: "",
                        developer: &people_reports.selected_project_data.as_ref().ok_or("People Zone project")?.system.developer_flag,
                        clock: Some(&people_clock),
                        backend: &mut zone_backend,
                        minutes_in_time_step: 15,
                        host: &mut gain_host,
                        eio: &mut people_eio,
                    },
                    &mut people_reports.heat_balance_input_errors_found,
                )?;
                assert_eq!(people_zone_globals.num_of_zones, 1);
                assert_eq!(people_zone_globals.num_spaces, 1);
                assert_eq!(
                    people_zones
                        .resilience
                        .as_ref()
                        .ok_or("real GetZoneData resilience allocation")?
                        .len(),
                    1
                );
                assert_eq!(
                    people_zones
                        .zone_predefined
                        .as_ref()
                        .ok_or("real GetZoneData predefined allocation")?
                        .len(),
                    1
                );
                people_reports.selected_zone_input = Some(people_zones);
                people_reports.selected_heat_balance_globals = Some(people_zone_globals);
                // Directly exercise the same shared getter's genuine EOF count:
                // Activity is filled even though its field is outside NumAlphas.
                if expected_count == 1.5 {
                    for (max_fields, expected_alphas) in [(9, 4), (10, 5)] {
                        let mut raw = people_reports
                            .heat_balance_input
                            .clone()
                            .ok_or("People EOF raw")?;
                        raw.source_is_epjson = false;
                        for row in raw.objects.get_mut("People").ok_or("People EOF rows")? {
                            row.idf_max_fields = Some(max_fields);
                            row.fields.remove("enable_ashrae_55_comfort_warnings");
                        }
                        let mut input = people_reports
                            .selected_input_processor
                            .clone()
                            .ok_or("People EOF IP")?;
                        let mut shortcut = SourceInputProcessorShortCut::default();
                        shortcut.allocate_from_input(&input)?;
                        let values =
                            raw.get_object_item::<21, 8>("People", 0, &mut input, &mut shortcut)?;
                        assert_eq!(values.num_alphas, expected_alphas);
                        assert_eq!(values.num_numbers, 5);
                        assert_eq!(values.alphas[4], "ACTIVITY");
                        assert!(!shortcut.alpha_field_blanks[4]);
                    }
                }
                let people_getter_base = people_reports.clone();
                CtfInitializationContext {
                    reports: &mut people_reports,
                    eio: &mut people_eio,
                    backend: SourceErrorBackend {
                        state: &mut people_state,
                        error_stream: Some(&mut people_errors),
                        console: &mut people_console,
                        out_unit1: None,
                        out_unit2: None,
                        f_message: None,
                        message_callback: None,
                        error_callback: None,
                        sqlite: None,
                    },
                }
                .manage_selected_internal_heat_gains_input_with_dictionary(
                    &people_model,
                    15,
                    &mut gain_host,
                    Some(&people_clock),
                    Some(SourceGainDictionaryFile {
                        eso: Some(&mut people_eso),
                    }),
                )?;
                if expected_count == 1.5 {
                    // Fresh reads from the genuine pre-getter checkpoint. These
                    // authored Space operands are independent of the Zone area;
                    // no geometry completion or remainder-space writer is claimed.
                    let mut read_sizing = |mut candidate: CtfInitializationReports|
                     -> Result<_, Box<dyn std::error::Error>> {
                        let (mut state, mut errors, mut console, mut eio, mut eso) = (
                            ctf_ordinary_output_test_state(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        );
                        let mut clock = people_clock.clone();
                        clock.warmup_flag = true;
                        let reached = CtfInitializationContext {
                            reports: &mut candidate,
                            eio: &mut eio,
                            backend: SourceErrorBackend {
                                state: &mut state,
                                error_stream: Some(&mut errors),
                                console: &mut console,
                                out_unit1: None,
                                out_unit2: None,
                                f_message: None,
                                message_callback: None,
                                error_callback: None,
                                sqlite: None,
                            },
                        }
                        .manage_selected_internal_heat_gains_input_with_dictionary(
                            &people_model,
                            15,
                            &mut gain_host,
                            Some(&clock),
                            Some(SourceGainDictionaryFile { eso: Some(&mut eso) }),
                        );
                        Ok((candidate, state, errors, eio, reached))
                    };
                    for (method, selected, area, remainder, expected, warnings) in [
                        ("People", Some(2.0), 100.0, false, 2.0_f64, 0),
                        ("People/Area", Some(0.02), 100.0, false, 2.0, 0),
                        ("Area/Person", Some(50.0), 100.0, false, 2.0, 0),
                        ("People/Area", Some(-0.0), 100.0, false, -0.0, 0),
                        ("People/Area", Some(0.02), 0.0, false, 0.0, 1),
                        ("Area/Person", Some(50.0), 0.0, false, 0.0, 1),
                        ("People/Area", Some(0.02), 0.0, true, 0.0, 0),
                        ("People/Area", None, 0.0, false, 0.0, 1),
                        ("Area/Person", None, 0.0, false, 0.0, 1),
                    ] {
                        let mut candidate = people_getter_base.clone();
                        let zones = candidate
                            .selected_zone_input
                            .as_mut()
                            .ok_or("sizing Zones")?;
                        zones.zones.as_mut().ok_or("sizing Zone area")?[0].floor_area = 9999.0;
                        let space = &mut zones.spaces.as_mut().ok_or("sizing Space")?[0];
                        assert!(!space.is_remainder_space);
                        space.floor_area = area;
                        space.is_remainder_space = remainder;
                        let rows = candidate
                            .heat_balance_input
                            .as_mut()
                            .ok_or("sizing raw")?
                            .objects
                            .get_mut("People")
                            .ok_or("sizing People")?;
                        let field = match method {
                            "People" => "number_of_people",
                            "People/Area" => "people_per_floor_area",
                            "Area/Person" => "floor_area_per_person",
                            _ => return Err("authored sizing method".into()),
                        };
                        rows[0].fields.insert(
                            "number_of_people_calculation_method".into(),
                            ProjectInputValue::String(method.into()),
                        );
                        for name in [
                            "number_of_people",
                            "people_per_floor_area",
                            "floor_area_per_person",
                        ] {
                            rows[0]
                                .fields
                                .insert(name.into(), ProjectInputValue::Number(7.0));
                        }
                        if let Some(value) = selected {
                            rows[0]
                                .fields
                                .insert(field.into(), ProjectInputValue::Number(value));
                        } else {
                            rows[0].fields.remove(field);
                        }
                        rows[1]
                            .fields
                            .insert("number_of_people".into(), ProjectInputValue::Number(0.0));
                        let (candidate, state, errors, _, reached) = read_sizing(candidate)?;
                        assert!(reached.is_ok(), "{method}/{selected:?}: {reached:?}");
                        let gains = &candidate.selected_internal_heat_gains;
                        assert_eq!(
                            gains.allocations.people.rows[0].number_of_people.to_bits(),
                            expected.to_bits()
                        );
                        assert_eq!(gains.allocations.people.rows[1].number_of_people, 0.0);
                        assert_eq!(state.tracking.total_warning_errors, warnings);
                        assert_eq!(state.tracking.total_warning_errors_during_warmup, warnings);
                        assert_eq!(state.tracking.total_severe_errors, 0);
                        assert!(!state.warmup_flag);
                        let text = std::str::from_utf8(&errors)?;
                        if warnings != 0 {
                            assert!(text.contains(if selected.is_none() {
                                "that field is blank.  0 People"
                            } else {
                                "Space Floor Area = 0.  0 People"
                            }));
                        }
                    }
                    // A genuine earlier selected-operand error persists. The
                    // later blank still warns, while a later nonblank operand
                    // returns zero before its zero-area warning. UInt64 is first
                    // projected by the same real JSON-kind child as the frontend.
                    let projected_negative =
                        ep_model::classify_source_json_number("18446744073709551615")
                            .map_err(|_| "sizing UInt64 token")?
                            .as_f64();
                    for (method, selected, later_blank, predicate) in [
                        ("People/Area", projected_negative, true, "[<0.0]=-1.000"),
                        ("Area/Person", 0.0, false, "[<=0.0]=0.000"),
                    ] {
                        let mut candidate = people_getter_base.clone();
                        candidate
                            .selected_zone_input
                            .as_mut()
                            .ok_or("invalid sizing Zone")?
                            .spaces
                            .as_mut()
                            .ok_or("invalid sizing Space")?[0]
                            .floor_area = 0.0;
                        let rows = candidate
                            .heat_balance_input
                            .as_mut()
                            .ok_or("invalid sizing raw")?
                            .objects
                            .get_mut("People")
                            .ok_or("invalid sizing rows")?;
                        rows[0].fields.insert(
                            "number_of_people_calculation_method".into(),
                            ProjectInputValue::String(method.into()),
                        );
                        rows[0].fields.insert(
                            if method == "People/Area" {
                                "people_per_floor_area"
                            } else {
                                "floor_area_per_person"
                            }
                            .into(),
                            ProjectInputValue::Number(selected),
                        );
                        rows[1].fields.insert(
                            "number_of_people_calculation_method".into(),
                            ProjectInputValue::String("People/Area".into()),
                        );
                        if later_blank {
                            rows[1].fields.remove("people_per_floor_area");
                        } else {
                            rows[1].fields.insert(
                                "people_per_floor_area".into(),
                                ProjectInputValue::Number(0.02),
                            );
                        }
                        let (candidate, state, errors, eio, reached) = read_sizing(candidate)?;
                        assert!(reached.is_err(), "invalid selected sizing completed");
                        assert!(candidate.selected_internal_heat_gains.get_input_flag);
                        assert!(candidate.selected_internal_heat_gains.errors_found);
                        assert!(
                            candidate
                                .selected_internal_heat_gains
                                .allocations
                                .people
                                .rows
                                .iter()
                                .all(|p| p.number_of_people == 0.0)
                        );
                        assert_eq!(state.tracking.total_severe_errors, 1);
                        assert_eq!(state.tracking.total_severe_errors_during_warmup, 1);
                        assert_eq!(state.tracking.total_warning_errors, i32::from(later_blank));
                        let text = std::str::from_utf8(&errors)?;
                        assert!(text.contains(predicate), "{text}");
                        assert_eq!(
                            text.matches("that field is blank.  0 People").count(),
                            usize::from(later_blank)
                        );
                        assert!(!text.contains("Space Floor Area = 0"));
                        assert!(eio.is_empty());
                    }
                    // Same actual controller constructor and same allocated
                    // Space MAT owner. The parent still stops at Refrigeration;
                    // no cache/radiation/report completion is manufactured.
                    let run_auto = |candidate: &mut CtfInitializationReports,
                                    space_mat: Option<f64>|
                     -> Result<_, Box<dyn std::error::Error>> {
                        let globals = candidate
                            .selected_heat_balance_globals
                            .as_ref()
                            .ok_or("automatic People globals")?;
                        let mut air = crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance::default();
                        if let Some(mat) = space_mat {
                            air.allocate_source_arrays(globals)?;
                            air.zones.as_mut().ok_or("automatic Zone MAT")?[0].air.mat = 25.0;
                            air.spaces.as_mut().ok_or("automatic Space MAT")?[0].air.mat = mat;
                        }
                        let controller = crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector::default();
                        assert!(!controller.subsystems.room_air.any_non_mixing);
                        let mut modules =
                            crate::heat_balance::selected_gain_modules::SourceGainModules::default(
                            );
                        let mut facility = crate::heat_balance::facility_electric_service::FacilityElectricService::new();
                        let mut provider =
                            crate::heat_balance::selected_gain_modules::SourceModuleGainProvider {
                                modules: &mut modules,
                                input: candidate
                                    .selected_input_processor
                                    .as_mut()
                                    .ok_or("automatic module IP")?,
                                facility: Some(&mut facility),
                            };
                        let (mut state, mut errors, mut console) =
                            (ctf_ordinary_output_test_state(), Vec::new(), Vec::new());
                        let mut backend = SourceErrorBackend {
                            state: &mut state,
                            error_stream: Some(&mut errors),
                            console: &mut console,
                            out_unit1: None,
                            out_unit2: None,
                            f_message: None,
                            message_callback: None,
                            error_callback: None,
                            sqlite: None,
                        };
                        let mut owners = SourceGainStepOwners {
                            clock: &people_clock,
                            globals,
                            control: &candidate.heat_balance_input_state,
                            zones: candidate
                                .selected_zone_input
                                .as_mut()
                                .ok_or("automatic People Zones")?,
                            schedules: candidate
                                .selected_schedule_manager
                                .as_ref()
                                .ok_or("automatic People schedules")?,
                            backend: &mut backend,
                            modules: &mut provider,
                            latent: if space_mat.is_some() {
                                Some(&mut air)
                            } else {
                                None
                            },
                            room_air: space_mat.map(|_| &controller.subsystems.room_air),
                            radiation: None,
                            zone_generic_gain: SourceZoneGenericGainAllocation::Unavailable,
                        };
                        let reached = candidate
                            .selected_internal_heat_gains
                            .manage_timestep(&mut owners);
                        Ok((reached, air))
                    };
                    for (sensible_token, mat, expected_sensible) in [
                        (None, 20.0, 173.20701915999996_f64),
                        (Some(""), 20.0, 173.20701915999996),
                        (Some("Autocalculate"), 20.0, 173.20701915999996),
                        (Some("Autocalculate"), 0.0, 240.0),
                        (Some("Autocalculate"), 50.0, 0.0),
                    ] {
                        let mut candidate = people_getter_base.clone();
                        let rows = candidate
                            .heat_balance_input
                            .as_mut()
                            .ok_or("automatic raw")?
                            .objects
                            .get_mut("People")
                            .ok_or("automatic People")?;
                        rows[0]
                            .fields
                            .insert("number_of_people".into(), ProjectInputValue::Number(2.0));
                        rows[1]
                            .fields
                            .insert("number_of_people".into(), ProjectInputValue::Number(0.0));
                        if let Some(value) = sensible_token {
                            rows[0].fields.insert(
                                "sensible_heat_fraction".into(),
                                ProjectInputValue::String(value.into()),
                            );
                        } else {
                            rows[0].fields.remove("sensible_heat_fraction");
                        }
                        let mut local = SourceInputProcessorShortCut::default();
                        let input = candidate
                            .selected_input_processor
                            .as_mut()
                            .ok_or("automatic IP")?;
                        local.allocate_from_input(input)?;
                        let values = candidate
                            .heat_balance_input
                            .as_ref()
                            .ok_or("automatic getter raw")?
                            .get_object_item::<21, 8>("People", 0, input, &mut local)?;
                        assert_eq!(values.numbers[4], -99999.0);
                        assert_eq!(
                            local.numeric_field_blanks[4],
                            sensible_token != Some("Autocalculate")
                        );
                        let (mut candidate, state, _, _, reached) = read_sizing(candidate)?;
                        assert!(reached.is_ok(), "automatic getter: {reached:?}");
                        assert_eq!(state.tracking.total_warning_errors, 0);
                        assert_eq!(
                            candidate
                                .selected_internal_heat_gains
                                .allocations
                                .people
                                .rows[0]
                                .user_spec_sens_frac,
                            -99999.0
                        );
                        let (reached, air) = run_auto(&mut candidate, Some(mat))?;
                        assert!(matches!(&reached, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("same refrigeration selected official schema definition"))), "automatic parent prefix: {reached:?}");
                        let person = &candidate
                            .selected_internal_heat_gains
                            .allocations
                            .people
                            .rows[0];
                        assert_eq!(person.tot_gain_rate, 240.0);
                        assert!(
                            (person.sen_gain_rate - expected_sensible).abs() < 1.0e-11,
                            "Space MAT {mat}: {}",
                            person.sen_gain_rate
                        );
                        assert_eq!(person.lat_gain_rate, 240.0 - person.sen_gain_rate);
                        assert_eq!(person.rad_gain_rate, person.sen_gain_rate * 0.3);
                        assert_eq!(person.con_gain_rate, person.sen_gain_rate * 0.7);
                        assert_eq!(person.tot_gain_energy, 0.0);
                        assert_eq!(
                            air.zones.as_ref().ok_or("unchanged Zone MAT")?[0].air.mat,
                            25.0
                        );
                        assert_eq!(
                            air.spaces.as_ref().ok_or("actual Space MAT")?[0].air.mat,
                            mat
                        );
                        assert!(
                            candidate
                                .selected_internal_heat_gains
                                .allocations
                                .space_devices
                                .as_ref()
                                .ok_or("automatic cache")?[0]
                                .devices
                                .as_ref()
                                .ok_or("automatic slots")?
                                .iter()
                                .all(|d| d.cached_rates == [0.0; 7])
                        );
                    }
                    for occupants in [0.0_f64, projected_negative] {
                        let mut candidate = people_getter_base.clone();
                        let rows = candidate
                            .heat_balance_input
                            .as_mut()
                            .ok_or("nonpositive automatic raw")?
                            .objects
                            .get_mut("People")
                            .ok_or("nonpositive automatic rows")?;
                        for (i, row) in rows.iter_mut().enumerate() {
                            row.fields.insert(
                                "number_of_people".into(),
                                ProjectInputValue::Number(if i == 0 { occupants } else { 0.0 }),
                            );
                            row.fields.remove("sensible_heat_fraction");
                        }
                        let (mut candidate, _, _, _, reached) = read_sizing(candidate)?;
                        assert!(reached.is_ok(), "nonpositive automatic getter: {reached:?}");
                        let (reached, air) = run_auto(&mut candidate, None)?;
                        assert!(matches!(&reached, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("same refrigeration selected official schema definition"))), "nonpositive automatic prefix: {reached:?}");
                        let person = &candidate
                            .selected_internal_heat_gains
                            .allocations
                            .people
                            .rows[0];
                        assert_eq!(person.num_occ.to_bits(), occupants.to_bits());
                        for rate in [
                            person.tot_gain_rate,
                            person.sen_gain_rate,
                            person.lat_gain_rate,
                        ] {
                            assert_eq!(rate.to_bits(), 0.0_f64.to_bits());
                        }
                        assert!(air.spaces.is_none());
                    }
                    // Real People design totals precede OtherEquipment sizing. Current
                    // occupancy can be zero while the design total remains two.
                    let person_case = |method: &str,
                                       selected: Option<f64>,
                                       counts: Option<[f64; 2]>,
                                       occupancy: &str,
                                       fuel: &str,
                                       prior_error: bool|
                     -> Result<
                        CtfInitializationReports,
                        Box<dyn std::error::Error>,
                    > {
                        let mut candidate = people_getter_base.clone();
                        let raw = candidate
                            .heat_balance_input
                            .as_mut()
                            .ok_or("per-person raw")?;
                        let people = raw.objects.get_mut("People").ok_or("per-person People")?;
                        if let Some(counts) = counts {
                            for (row, count) in people.iter_mut().zip(counts) {
                                row.fields.insert(
                                    "number_of_people".into(),
                                    ProjectInputValue::Number(count),
                                );
                                row.fields.insert(
                                    "number_of_people_schedule_name".into(),
                                    ProjectInputValue::String(occupancy.into()),
                                );
                            }
                            if prior_error {
                                people[0].fields.insert(
                                    "number_of_people_calculation_method".into(),
                                    ProjectInputValue::String("Area/Person".into()),
                                );
                                people[0].fields.insert(
                                    "floor_area_per_person".into(),
                                    ProjectInputValue::Number(0.0),
                                );
                            }
                        } else {
                            people.clear();
                        }
                        let mut equipment = failed_gains
                            .heat_balance_input
                            .as_ref()
                            .ok_or("existing raw equipment")?
                            .objects
                            .get("OtherEquipment")
                            .ok_or("existing equipment rows")?[0]
                            .clone();
                        equipment.object_name = "Per person equipment".into();
                        equipment.fields.insert(
                            "zone_or_zonelist_or_space_or_spacelist_name".into(),
                            ProjectInputValue::String(zone_name.clone()),
                        );
                        equipment.fields.insert(
                            "design_level_calculation_method".into(),
                            ProjectInputValue::String(method.into()),
                        );
                        equipment
                            .fields
                            .insert("fuel_type".into(), ProjectInputValue::String(fuel.into()));
                        equipment
                            .fields
                            .insert("design_level".into(), ProjectInputValue::Number(-7.0));
                        equipment.fields.insert(
                            "power_per_floor_area".into(),
                            ProjectInputValue::Number(-9.0),
                        );
                        if let Some(value) = selected {
                            equipment.fields.insert(
                                "power_per_person".into(),
                                ProjectInputValue::Number(value),
                            );
                        } else {
                            equipment.fields.remove("power_per_person");
                        }
                        raw.objects.insert("OtherEquipment".into(), vec![equipment]);
                        let mut old_ip = candidate
                            .selected_input_processor
                            .take()
                            .ok_or("per-person official definitions")?;
                        let mut definitions = Vec::new();
                        for family in [
                            "People",
                            "Lights",
                            "ElectricEquipment",
                            "GasEquipment",
                            "HotWaterEquipment",
                            "SteamEquipment",
                            "OtherEquipment",
                            "ElectricEquipment:ITE:AirCooled",
                            "ZoneBaseboard:OutdoorTemperatureControlled",
                            "ZoneContaminantSourceAndSink:CarbonDioxide",
                            "Output:Variable",
                        ] {
                            let actual = old_ip.get_object_def_max_args(family)?;
                            definitions.push(SourceObjectDefinition {
                                object_type: family.into(),
                                alpha_fields: actual.alphas,
                                numeric_fields: actual.numerics,
                                alpha_extensions: 0,
                                numeric_extensions: 0,
                                extension: None,
                            });
                        }
                        let mut families = std::collections::BTreeMap::new();
                        for family in ["People", "OtherEquipment"] {
                            let objects =
                                raw.objects.get(family).ok_or("per-person retained rows")?;
                            families.insert(
                                family.into(),
                                SourceInputFamily::Objects(
                                    objects
                                        .iter()
                                        .map(|row| SourceObjectShape {
                                            object_name: row.object_name.clone(),
                                            field_sizes: Default::default(),
                                        })
                                        .collect(),
                                ),
                            );
                        }
                        if let Some(family) = old_ip.family("Output:Variable") {
                            families.insert("Output:Variable".into(), family.clone());
                        }
                        candidate.selected_input_processor = Some(
                            SelectedInputProcessor::from_admitted_input(definitions, families)?,
                        );
                        Ok(candidate)
                    };
                    let projected_negative =
                        ep_model::classify_source_json_number("18446744073709551615")
                            .map_err(|_| "per-person signed design total")?
                            .as_f64();
                    for (
                        method,
                        selected,
                        counts,
                        occupancy,
                        fuel,
                        prior_error,
                        total,
                        level,
                        warnings,
                        severe,
                    ) in [
                        (
                            "Watts/Person",
                            Some(3.5),
                            Some([1.5, 0.5]),
                            "Constant-0.0",
                            "None",
                            false,
                            2.0_f64,
                            7.0_f64,
                            0,
                            0,
                        ),
                        (
                            "Power/Person",
                            Some(3.5),
                            Some([1.5, 0.5]),
                            "Constant-1.0",
                            "None",
                            false,
                            2.0,
                            7.0,
                            0,
                            0,
                        ),
                        (
                            "Watts/Person",
                            Some(3.5),
                            None,
                            "Constant-1.0",
                            "None",
                            false,
                            0.0,
                            0.0,
                            1,
                            0,
                        ),
                        (
                            "Power/Person",
                            Some(3.5),
                            Some([0.0, projected_negative]),
                            "Constant-1.0",
                            "None",
                            false,
                            -1.0,
                            -3.5,
                            1,
                            0,
                        ),
                        (
                            "Watts/Person",
                            None,
                            Some([1.5, 0.5]),
                            "Constant-1.0",
                            "None",
                            false,
                            2.0,
                            0.0,
                            1,
                            0,
                        ),
                        (
                            "Watts/Person",
                            Some(0.0),
                            Some([1.5, 0.5]),
                            "Constant-1.0",
                            "None",
                            false,
                            2.0,
                            0.0,
                            0,
                            1,
                        ),
                        (
                            "Power/Person",
                            Some(-1.0),
                            Some([1.5, 0.5]),
                            "Constant-1.0",
                            "None",
                            false,
                            2.0,
                            0.0,
                            0,
                            1,
                        ),
                        (
                            "Watts/Person",
                            Some(3.5),
                            Some([0.0, projected_negative]),
                            "Constant-1.0",
                            "Electricity",
                            false,
                            -1.0,
                            -3.5,
                            1,
                            1,
                        ),
                        (
                            "Power/Person",
                            Some(3.5),
                            Some([0.0, 0.0]),
                            "Constant-1.0",
                            "None",
                            true,
                            0.0,
                            0.0,
                            0,
                            1,
                        ),
                        (
                            "Power/Person",
                            None,
                            Some([0.0, 0.0]),
                            "Constant-1.0",
                            "None",
                            true,
                            0.0,
                            0.0,
                            1,
                            1,
                        ),
                    ] {
                        let candidate =
                            person_case(method, selected, counts, occupancy, fuel, prior_error)?;
                        let (mut candidate, state, errors, eio, reached) = read_sizing(candidate)?;
                        assert_eq!(
                            reached.is_ok(),
                            severe == 0,
                            "{method}/{selected:?}: {reached:?}"
                        );
                        let gains = &candidate.selected_internal_heat_gains;
                        assert_eq!(gains.get_input_flag, severe != 0);
                        assert_eq!(gains.errors_found, severe != 0);
                        let zones = candidate
                            .selected_zone_input
                            .as_ref()
                            .ok_or("per-person totals")?;
                        assert_eq!(
                            zones.spaces.as_ref().ok_or("per-person Space total")?[0]
                                .total_occupants
                                .to_bits(),
                            total.to_bits()
                        );
                        assert_eq!(
                            zones.zones.as_ref().ok_or("per-person Zone total")?[0]
                                .total_occupants
                                .to_bits(),
                            total.to_bits()
                        );
                        let equipment = &gains
                            .allocations
                            .other_equipment
                            .as_ref()
                            .ok_or("per-person equipment")?[0];
                        assert_eq!(equipment.design_level.to_bits(), level.to_bits());
                        assert_eq!(state.tracking.total_warning_errors, warnings);
                        assert_eq!(state.tracking.total_warning_errors_during_warmup, warnings);
                        assert_eq!(state.tracking.total_severe_errors, severe);
                        assert_eq!(state.tracking.total_severe_errors_during_warmup, severe);
                        assert!(
                            !state.warmup_flag,
                            "actual clock phase stays a borrowed diagnostic operand"
                        );
                        let text = std::str::from_utf8(&errors)?;
                        assert_eq!(
                            text.matches("Total Occupants = 0.  0 OtherEquipment")
                                .count(),
                            usize::from(warnings != 0 && selected.is_some())
                        );
                        if selected.is_none() {
                            assert!(text.contains("specifies power_per_person, but that field is blank.  0 OtherEquipment"));
                        }
                        if selected.is_some_and(|value| value <= 0.0) {
                            assert!(text.contains("invalid power_per_person, value  [<=0.0]="));
                        }
                        if fuel == "Electricity" {
                            assert!(
                                text.find("Total Occupants = 0").ok_or("total warning")?
                                    < text
                                        .find("is not allowed to be negative")
                                        .ok_or("negative fuel severe")?
                            );
                            assert!(
                                text.contains("... when a fuel type of Electricity is specified.")
                            );
                        }
                        if severe != 0 {
                            assert!(
                                text.contains(" **  Fatal  ** GetInternalHeatGains: Errors found")
                            );
                            assert!(eio.is_empty(), "nominal follows the real final Fatal");
                        } else {
                            assert!(
                                std::str::from_utf8(&eio)?
                                    .contains("OtherEquipment Internal Gains Nominal")
                            );
                        }
                        if occupancy == "Constant-0.0" {
                            // An actual timestep prefix distinguishes design total
                            // from current occupants; its later Refrig stop stays real.
                            let (reached, air) = run_auto(&mut candidate, None)?;
                            assert!(matches!(&reached, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("same refrigeration selected official schema definition"))), "per-person timestep prefix: {reached:?}");
                            let gains = &candidate.selected_internal_heat_gains;
                            assert!(
                                gains
                                    .allocations
                                    .people
                                    .rows
                                    .iter()
                                    .all(|row| row.num_occ == 0.0 && row.tot_gain_rate == 0.0)
                            );
                            let equipment = &gains
                                .allocations
                                .other_equipment
                                .as_ref()
                                .ok_or("per-person rates")?[0];
                            assert_eq!(equipment.rates[0], 7.0);
                            assert_eq!(equipment.energies[0], 0.0);
                            assert!(
                                air.spaces.is_none(),
                                "failed module still precedes actual latent sinks"
                            );
                        }
                    }
                    // A real write failure in the new total-occupant warning
                    // precedes equipment assignment, fractions and nominal rows.
                    {
                        let mut candidate = person_case(
                            "Watts/Person",
                            Some(3.5),
                            None,
                            "Constant-1.0",
                            "None",
                            false,
                        )?;
                        let (mut state, mut errors, mut console, mut eio, mut eso) = (
                            ctf_ordinary_output_test_state(),
                            OrdinaryFailingEio,
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        );
                        let mut clock = people_clock.clone();
                        clock.warmup_flag = true;
                        let reached = CtfInitializationContext {
                            reports: &mut candidate,
                            eio: &mut eio,
                            backend: SourceErrorBackend {
                                state: &mut state,
                                error_stream: Some(&mut errors),
                                console: &mut console,
                                out_unit1: None,
                                out_unit2: None,
                                f_message: None,
                                message_callback: None,
                                error_callback: None,
                                sqlite: None,
                            },
                        }
                        .manage_selected_internal_heat_gains_input_with_dictionary(
                            &people_model,
                            15,
                            &mut gain_host,
                            Some(&clock),
                            Some(SourceGainDictionaryFile {
                                eso: Some(&mut eso),
                            }),
                        );
                        assert!(matches!(reached, Err(crate::RuntimeError::WarmupOutput(_))));
                        let gains = &candidate.selected_internal_heat_gains;
                        assert!(gains.get_input_flag && !gains.errors_found);
                        let equipment = &gains
                            .allocations
                            .other_equipment
                            .as_ref()
                            .ok_or("failed per-person row")?[0];
                        assert_eq!(equipment.design_level, 0.0);
                        assert_eq!(equipment.fraction_radiant, 0.0);
                        assert_eq!(state.tracking.total_warning_errors_during_warmup, 1);
                        assert!(eio.is_empty());
                    }
                }
                if expected_count == 1.5 {
                    let mut blank = people_getter_base.clone();
                    blank
                        .heat_balance_input
                        .as_mut()
                        .ok_or("blank People raw")?
                        .objects
                        .get_mut("People")
                        .ok_or("blank People rows")?[0]
                        .fields
                        .remove("number_of_people");
                    let mut blank_clock = people_clock.clone();
                    blank_clock.warmup_flag = true;
                    let (
                        mut blank_state,
                        mut blank_error,
                        mut blank_console,
                        mut blank_eio,
                        mut blank_eso,
                    ) = (
                        ctf_ordinary_output_test_state(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    );
                    assert!(!blank_state.warmup_flag);
                    CtfInitializationContext {
                        reports: &mut blank,
                        eio: &mut blank_eio,
                        backend: SourceErrorBackend {
                            state: &mut blank_state,
                            error_stream: Some(&mut blank_error),
                            console: &mut blank_console,
                            out_unit1: None,
                            out_unit2: None,
                            f_message: None,
                            message_callback: None,
                            error_callback: None,
                            sqlite: None,
                        },
                    }
                    .manage_selected_internal_heat_gains_input_with_dictionary(
                        &people_model,
                        15,
                        &mut gain_host,
                        Some(&blank_clock),
                        Some(SourceGainDictionaryFile {
                            eso: Some(&mut blank_eso),
                        }),
                    )?;
                    assert!(!blank.selected_internal_heat_gains.get_input_flag);
                    assert_eq!(
                        blank.selected_internal_heat_gains.allocations.people.rows[0]
                            .number_of_people,
                        0.0
                    );
                    assert_eq!(blank_state.tracking.total_warning_errors, 1);
                    assert_eq!(blank_state.tracking.total_warning_errors_during_warmup, 1);
                    assert_eq!(blank_state.tracking.total_severe_errors, 0);
                    assert!(std::str::from_utf8(&blank_error)?.contains("specifies number_of_people, but that field is blank.  0 People will result."));
                }
                // Native's unselected auxiliary fields never resolve their
                // names or assign the constructor's MRT/clothing/schedule fields.
                if expected_count == 1.5 {
                    for (auxiliary, mrt, clothing, expected_alphas) in [
                        ("work_efficiency_schedule_name", None, None, 9),
                        ("clothing_insulation_schedule_name", None, None, 12),
                        ("air_velocity_schedule_name", None, None, 13),
                        (
                            "work_efficiency_schedule_name",
                            Some("SurfaceWeighted"),
                            Some("CalculationMethodSchedule"),
                            21,
                        ),
                        (
                            "work_efficiency_schedule_name",
                            Some("AngleFactor"),
                            Some("DynamicClothingModelASHRAE55"),
                            21,
                        ),
                    ] {
                        let mut auxiliary_reports = people_getter_base.clone();
                        let raw = auxiliary_reports
                            .heat_balance_input
                            .as_mut()
                            .ok_or("actual auxiliary People raw")?;
                        let first = &mut raw
                            .objects
                            .get_mut("People")
                            .ok_or("actual auxiliary People rows")?[0];
                        first.fields.insert(
                            auxiliary.into(),
                            ProjectInputValue::String("Unused People Auxiliary".into()),
                        );
                        if let (Some(mrt), Some(clothing)) = (mrt, clothing) {
                            for (field, value) in [
                                ("mean_radiant_temperature_calculation_type", mrt),
                                ("clothing_insulation_calculation_method", clothing),
                                (
                                    "surface_name_angle_factor_list_name",
                                    "missing unused surface or angle list",
                                ),
                                (
                                    "clothing_insulation_calculation_method_schedule_name",
                                    "missing unused method",
                                ),
                                (
                                    "clothing_insulation_schedule_name",
                                    "missing unused clothing",
                                ),
                                ("air_velocity_schedule_name", "missing unused velocity"),
                                (
                                    "ankle_level_air_velocity_schedule_name",
                                    "missing unused ankle",
                                ),
                            ] {
                                first
                                    .fields
                                    .insert(field.into(), ProjectInputValue::String(value.into()));
                            }
                        }
                        let mut input = auxiliary_reports
                            .selected_input_processor
                            .clone()
                            .ok_or("actual auxiliary People IP")?;
                        let mut shortcut = SourceInputProcessorShortCut::default();
                        shortcut.allocate_from_input(&input)?;
                        let values =
                            raw.get_object_item::<21, 8>("People", 0, &mut input, &mut shortcut)?;
                        assert_eq!(values.num_alphas, expected_alphas);
                        let mut auxiliary_clock = people_clock.clone();
                        auxiliary_clock.warmup_flag = true;
                        let (
                            mut auxiliary_state,
                            mut auxiliary_error,
                            mut auxiliary_console,
                            mut auxiliary_eio,
                            mut auxiliary_eso,
                        ) = (
                            ctf_ordinary_output_test_state(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        );
                        assert!(!auxiliary_state.warmup_flag);
                        CtfInitializationContext {
                            reports: &mut auxiliary_reports,
                            eio: &mut auxiliary_eio,
                            backend: SourceErrorBackend {
                                state: &mut auxiliary_state,
                                error_stream: Some(&mut auxiliary_error),
                                console: &mut auxiliary_console,
                                out_unit1: None,
                                out_unit2: None,
                                f_message: None,
                                message_callback: None,
                                error_callback: None,
                                sqlite: None,
                            },
                        }
                        .manage_selected_internal_heat_gains_input_with_dictionary(
                            &people_model,
                            15,
                            &mut gain_host,
                            Some(&auxiliary_clock),
                            Some(SourceGainDictionaryFile {
                                eso: Some(&mut auxiliary_eso),
                            }),
                        )?;
                        assert!(
                            !auxiliary_reports
                                .selected_internal_heat_gains
                                .get_input_flag
                        );
                        assert!(!auxiliary_reports.selected_internal_heat_gains.errors_found);
                        assert_eq!(
                            auxiliary_reports
                                .selected_internal_heat_gains
                                .allocations
                                .people,
                            people_reports
                                .selected_internal_heat_gains
                                .allocations
                                .people,
                            "unconsumed auxiliary fields retain every genuine People field"
                        );
                        let untouched = auxiliary_reports
                            .selected_schedule_manager
                            .as_ref()
                            .ok_or("actual auxiliary schedule owner")?
                            .schedules
                            .iter()
                            .find(|row| row.name == "UNUSED PEOPLE AUXILIARY")
                            .ok_or("actual produced unused auxiliary schedule")?;
                        assert!(!untouched.is_used, "no auxiliary schedule lookup occurred");
                        let expected_warning = if expected_alphas < 14 { 1 } else { 0 };
                        assert_eq!(
                            auxiliary_state.tracking.total_warning_errors,
                            expected_warning
                        );
                        assert_eq!(
                            auxiliary_state.tracking.total_warning_errors_during_warmup,
                            expected_warning
                        );
                        assert_eq!(auxiliary_state.tracking.total_severe_errors, 0);
                        let text = std::str::from_utf8(&auxiliary_error)?;
                        if expected_warning == 1 {
                            let messages = [
                                "GetInternalHeatGains: People=\"PEOPLE 0\" has comfort related schedules but no thermal comfort model selected.",
                                "If schedules are specified for air velocity, clothing insulation, and/or work efficiency but no thermal comfort",
                                "thermal comfort model is selected, the schedules will be listed as unused schedules in the .err file.",
                                "To avoid these errors, select a valid thermal comfort model or eliminate these schedules in the PEOPLE input.",
                            ];
                            let mut start = 0;
                            for message in messages {
                                start += text[start..]
                                    .find(message)
                                    .ok_or("actual ordered People auxiliary diagnostic")?
                                    + message.len();
                            }
                            assert_eq!(text.matches("** Warning **").count(), 1);
                            assert_eq!(text.matches("**   ~~~   **").count(), 3);
                        } else {
                            assert!(
                                auxiliary_error.is_empty(),
                                "A21 suppresses the low-count warning"
                            );
                        }
                    }
                }
                let people_gains = &people_reports.selected_internal_heat_gains;
                assert_eq!(people_gains.totals[0], 2);
                assert!(people_gains.allocations.people.allocated);
                assert_eq!(
                    people_gains.allocations.people.rows[1]
                        .number_of_people
                        .to_bits(),
                    expected_count.to_bits()
                );
                for person in &people_gains.allocations.people.rows {
                    assert_eq!(
                        (
                            person.mrt_calc_type,
                            person.clothing_type,
                            person.surface_ptr
                        ),
                        (-1, -1, -1)
                    );
                    assert!(
                        !person.fanger
                            && !person.pierce
                            && !person.ksu
                            && !person.adaptive_ash55
                            && !person.adaptive_cen15251
                            && !person.cooling_effect_ash55
                            && !person.ankle_draft_ash55
                    );
                    assert!(person.show55_warning);
                    assert_eq!(person.user_spec_sens_frac, 0.6);
                }
                let people_zones = people_reports
                    .selected_zone_input
                    .as_ref()
                    .ok_or("People Zone totals")?;
                for value in [
                    people_zones.zones.as_ref().ok_or("Zone totals")?[0].total_occupants,
                    people_zones.zones.as_ref().ok_or("Zone totals")?[0].min_occupants,
                    people_zones.zones.as_ref().ok_or("Zone totals")?[0].max_occupants,
                    people_zones.spaces.as_ref().ok_or("Space totals")?[0].total_occupants,
                ] {
                    assert_eq!(value, 1.5 + expected_count);
                }
                assert_eq!(
                    people_gains
                        .allocations
                        .space_devices
                        .as_ref()
                        .ok_or("People devices")?[0]
                        .number_of_devices,
                    2
                );
                assert!(
                    std::str::from_utf8(&people_eio)?.contains("People Internal Gains Nominal")
                );
                assert!(
                    std::str::from_utf8(&people_eso)?
                        .contains("PEOPLE 0,People Total Heating Rate [W] !TimeStep")
                );
                assert_eq!(people_state.tracking.total_warning_errors, 0);
                assert!(people_errors.is_empty());
                let people_globals = people_reports
                    .selected_heat_balance_globals
                    .as_ref()
                    .ok_or("People globals")?;
                let mut people_air =
                    crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance::default();
                people_air.allocate_source_arrays(people_globals)?;
                let people_air_before = people_air.clone();
                let mut people_modules =
                    crate::heat_balance::selected_gain_modules::SourceGainModules::default();
                let mut people_facility =
                    crate::heat_balance::facility_electric_service::FacilityElectricService::new();
                let mut people_provider =
                    crate::heat_balance::selected_gain_modules::SourceModuleGainProvider {
                        modules: &mut people_modules,
                        input: people_reports
                            .selected_input_processor
                            .as_mut()
                            .ok_or("People module IP")?,
                        facility: Some(&mut people_facility),
                    };
                let mut people_backend = SourceErrorBackend {
                    state: &mut people_state,
                    error_stream: Some(&mut people_errors),
                    console: &mut people_console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                };
                let mut people_step = SourceGainStepOwners {
                    clock: &people_clock,
                    globals: people_globals,
                    control: &people_reports.heat_balance_input_state,
                    zones: people_reports
                        .selected_zone_input
                        .as_mut()
                        .ok_or("People live Zones")?,
                    schedules: people_reports
                        .selected_schedule_manager
                        .as_ref()
                        .ok_or("People live schedules")?,
                    backend: &mut people_backend,
                    modules: &mut people_provider,
                    latent: Some(&mut people_air),
                    room_air: None,
                    radiation: None,
                    zone_generic_gain: SourceZoneGenericGainAllocation::Unavailable,
                };
                let reached = people_reports
                    .selected_internal_heat_gains
                    .manage_timestep(&mut people_step);
                assert!(matches!(&reached,Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("same refrigeration selected official schema definition"))),"actual People timestep prefix: {reached:?}");
                let rows = &people_reports
                    .selected_internal_heat_gains
                    .allocations
                    .people
                    .rows;
                assert_eq!(rows[0].num_occ, 1.5);
                assert_eq!(rows[0].tot_gain_rate, 180.0);
                assert_eq!(rows[0].sen_gain_rate, 108.0);
                assert_eq!(rows[1].num_occ.to_bits(), expected_count.to_bits());
                assert_eq!(
                    rows[1].tot_gain_rate,
                    if expected_count > 0.0 {
                        expected_count * 120.0
                    } else {
                        0.0
                    }
                );
                assert_eq!(rows[1].tot_gain_energy, 0.0);
                assert_eq!(people_air, people_air_before);
                assert!(
                    people_reports
                        .selected_internal_heat_gains
                        .allocations
                        .space_devices
                        .as_ref()
                        .ok_or("People cache")?[0]
                        .devices
                        .as_ref()
                        .ok_or("People slots")?
                        .iter()
                        .all(|d| d.cached_rates == [0.0; 7])
                );
                let predefined = &people_reports
                    .selected_zone_input
                    .as_ref()
                    .ok_or("People occupied Zone")?
                    .zone_predefined
                    .as_ref()
                    .ok_or("People occupied report")?[0];
                assert_eq!(predefined.is_occupied, expected_count > 0.0);
                assert_eq!(predefined.num_occ, 1.5 + expected_count.max(0.0));
                assert_eq!(
                    predefined.num_occ_accum_time,
                    if expected_count > 0.0 { 0.5 } else { 0.25 }
                );
                // Explicit real children following the failed parent above.
                let mut people_child = SourceGainStepOwners {
                    clock: &people_clock,
                    globals: people_globals,
                    control: &people_reports.heat_balance_input_state,
                    zones: people_reports
                        .selected_zone_input
                        .as_mut()
                        .ok_or("People child Zones")?,
                    schedules: people_reports
                        .selected_schedule_manager
                        .as_ref()
                        .ok_or("People child schedules")?,
                    backend: &mut people_backend,
                    modules: &mut people_provider,
                    latent: Some(&mut people_air),
                    room_air: None,
                    radiation: None,
                    zone_generic_gain: SourceZoneGenericGainAllocation::Unavailable,
                };
                people_reports
                    .selected_internal_heat_gains
                    .update_source_internal_gain_values(&mut people_child, false, true)?;
                people_reports
                    .selected_internal_heat_gains
                    .report_timestep(&people_child)?;
                let total = 180.0
                    + if expected_count > 0.0 {
                        expected_count * 120.0
                    } else {
                        0.0
                    };
                let report = &people_reports
                    .selected_internal_heat_gains
                    .allocations
                    .zone_report
                    .as_ref()
                    .ok_or("People Zone report")?[0];
                assert_eq!(report.people_num_occ, 1.5 + expected_count);
                assert_eq!(report.people_tot_gain_rate, total);
                assert_eq!(report.people_tot_gain, total * 900.0);
                let requested = people_reports
                    .output_registration
                    .out_vars
                    .iter()
                    .find(|v| v.name == "People Total Heating Rate")
                    .ok_or("People bound Which")?;
                assert_eq!(
                    *people_reports
                        .selected_internal_heat_gains
                        .real_output(requested.which.ok_or("People real binding")?)?,
                    180.0
                );
                assert_eq!(
                    people_air.zones.as_ref().ok_or("People latent Zone")?[0]
                        .air
                        .latent_gain,
                    total - total * 0.6
                );
                // Actual shared IP extensible getter: raw arrays, source format,
                // schema-dependent global max args, exact names and usage owner.
                {
                    use crate::heat_balance::ordinary_thermal_comfort::SourceThermalComfort;
                    use crate::heat_balance::selected_input_processor::{
                        SelectedInputProcessor, SourceInputFamily, SourceInputProcessorShortCut,
                        SourceInputStatisticsInput, SourceInputStatisticsValue as V,
                        SourceObjectDefinition, SourceObjectShape,
                    };
                    let mut objects = std::collections::BTreeMap::<
                        String,
                        std::collections::BTreeMap<String, V>,
                    >::new();
                    for (name, e, r) in [
                        ("", 199, 100),
                        ("z list", 200, 100),
                        ("Z LIST", 1, 1),
                        (" ", 0, 0),
                    ] {
                        let rows = (0..r)
                            .map(|at| {
                                let mut row = std::collections::BTreeMap::from([(
                                    "surface_name".into(),
                                    V::String(format!("surface {at}")),
                                )]);
                                if !(name == "Z LIST" || (name.is_empty() && at == 99)) {
                                    row.insert(
                                        "angle_factor".into(),
                                        V::Number(
                                            match at {
                                                0 => "-0",
                                                1 => "-0.0",
                                                2 => "-0e0",
                                                _ => "0.01",
                                            }
                                            .into(),
                                        ),
                                    );
                                }
                                V::Object(row)
                            })
                            .collect();
                        objects.insert(
                            name.into(),
                            std::collections::BTreeMap::from([
                                ("angles".into(), V::Array(rows)),
                                ("idf_max_fields".into(), V::Number((1 + e).to_string())),
                                ("idf_max_extensible_fields".into(), V::Number(e.to_string())),
                            ]),
                        );
                    }
                    objects.insert(
                        "short list".into(),
                        std::collections::BTreeMap::from([
                            (
                                "angles".into(),
                                V::Array(vec![V::Object(Default::default())]),
                            ),
                            ("idf_max_fields".into(), V::Number("2".into())),
                            ("idf_max_extensible_fields".into(), V::Number("1".into())),
                        ]),
                    );
                    // Genuine Native EOF: independent F2/E2, no attached array.
                    objects.insert(
                        "EOF list".into(),
                        std::collections::BTreeMap::from([
                            ("idf_max_fields".into(), V::Number("2".into())),
                            ("idf_max_extensible_fields".into(), V::Number("2".into())),
                        ]),
                    );
                    let order = vec![
                        "z list".into(),
                        "".into(),
                        " ".into(),
                        "Z LIST".into(),
                        "short list".into(),
                        "EOF list".into(),
                    ];
                    let make_input = |source_is_epjson: bool| -> Result<
                        SelectedInputProcessor,
                        Box<dyn std::error::Error>,
                    > {
                        let shapes = objects
                            .iter()
                            .map(|(name, fields)| SourceObjectShape {
                                object_name: name.clone(),
                                field_sizes: fields
                                    .iter()
                                    .map(|(name, value)| {
                                        (
                                            name.clone(),
                                            match value {
                                                V::Array(rows) => rows.len(),
                                                _ => 1,
                                            },
                                        )
                                    })
                                    .collect(),
                            })
                            .collect();
                        let mut input = SelectedInputProcessor::from_admitted_input(
                            vec![SourceObjectDefinition {
                                object_type: "ComfortViewFactorAngles".into(),
                                alpha_fields: 1,
                                numeric_fields: 0,
                                alpha_extensions: 1,
                                numeric_extensions: 1,
                                extension: Some("angles".into()),
                            }],
                            std::collections::BTreeMap::from([(
                                "ComfortViewFactorAngles".into(),
                                SourceInputFamily::Objects(shapes),
                            )]),
                        )?;
                        input.retain_source_input_statistics(SourceInputStatisticsInput {
                            objects: std::collections::BTreeMap::from([(
                                "ComfortViewFactorAngles".into(),
                                objects.clone(),
                            )]),
                            object_order: std::collections::BTreeMap::from([(
                                "ComfortViewFactorAngles".into(),
                                if source_is_epjson {
                                    objects.keys().cloned().collect()
                                } else {
                                    order.clone()
                                },
                            )]),
                        })?;
                        Ok(input)
                    };
                    for source_is_epjson in [false, true] {
                        let mut input = make_input(source_is_epjson)?;
                        let mut shortcut = SourceInputProcessorShortCut::default();
                        shortcut.allocate_from_input(&input)?;
                        assert_eq!(
                            (shortcut.alpha_args.len(), shortcut.numeric_args.len()),
                            (101, 100)
                        );
                        shortcut.current_module_object = "ComfortViewFactorAngles".into();
                        assert_eq!(
                            input.source_raw_object_names("ComfortViewFactorAngles")?[0],
                            if source_is_epjson { "" } else { "z list" }
                        );
                        let mut used = std::collections::BTreeSet::new();
                        // JSON lexical order or retained IDF declaration order.
                        // Every retained row still stores beyond the counted E.
                        for (name, json_counts, idf_counts, r) in [
                            ("z list", (101, 100, 1), (101, 100, 1), 100),
                            ("", (101, 100, 1), (101, 99, 1), 100),
                            (" ", (1, 0, 1), (1, 0, 1), 0),
                            ("Z LIST", (2, 1, 1), (2, 0, 1), 1),
                            ("z list", (101, 100, 1), (101, 100, 1), 100),
                        ] {
                            let ordinal = input
                                .source_raw_object_names("ComfortViewFactorAngles")?
                                .iter()
                                .position(|candidate| candidate == name)
                                .ok_or("actual angle raw key")?;
                            let got = SourceThermalComfort::get_source_angle_factor_object_item(
                                &mut input,
                                &mut shortcut,
                                ordinal,
                                source_is_epjson,
                            )?;
                            assert_eq!(
                                got,
                                if source_is_epjson {
                                    json_counts
                                } else {
                                    idf_counts
                                }
                            );
                            used.insert(name.to_string());
                            for original in
                                input.source_raw_object_names("ComfortViewFactorAngles")?
                            {
                                assert_eq!(
                                    input.is_object_unused("ComfortViewFactorAngles", original),
                                    !used.contains(original)
                                );
                            }
                            assert_eq!(shortcut.alpha_args[0], name.to_ascii_uppercase());
                            assert_eq!(shortcut.alpha_field_blanks[0], name.is_empty());
                            assert_eq!(
                                shortcut.alpha_field_names[0],
                                if source_is_epjson { "name" } else { "Name" }
                            );
                            if r == 100 {
                                assert_eq!(shortcut.alpha_args[100], "SURFACE 99");
                                assert_eq!(shortcut.numeric_args[0].to_bits(), 0); // JSON integer -0 -> i64 zero.
                                assert_eq!(
                                    shortcut.numeric_args[1].to_bits(),
                                    (-0.0_f64).to_bits()
                                );
                                assert_eq!(
                                    shortcut.numeric_args[2].to_bits(),
                                    (-0.0_f64).to_bits()
                                );
                                assert!(
                                    shortcut.numeric_field_blanks[..99]
                                        .iter()
                                        .all(|blank| !*blank)
                                );
                                assert_eq!(shortcut.numeric_field_blanks[99], name.is_empty());
                                assert_eq!(
                                    shortcut.numeric_field_names[99],
                                    if source_is_epjson {
                                        "angle_factor"
                                    } else {
                                        "Angle Factor"
                                    }
                                );
                            } else {
                                assert!(shortcut.alpha_args[r + 1..].iter().all(String::is_empty));
                                assert!(
                                    shortcut
                                        .numeric_args
                                        .iter()
                                        .all(|value| value.to_bits() == 0)
                                );
                                assert!(shortcut.numeric_field_blanks.iter().all(|blank| *blank));
                                assert!(
                                    shortcut.alpha_field_names[r + 1..]
                                        .iter()
                                        .all(String::is_empty)
                                );
                                assert!(
                                    shortcut.numeric_field_names[r..]
                                        .iter()
                                        .all(String::is_empty)
                                );
                            }
                        }
                        // Fill100, reset with partial row, reset again with EOF,
                        // fill100 again, and reset every tail once more.
                        for name in ["short list", "EOF list", "z list", "short list", "EOF list"] {
                            let ordinal = input
                                .source_raw_object_names("ComfortViewFactorAngles")?
                                .iter()
                                .position(|candidate| candidate == name)
                                .ok_or("actual short/EOF raw key")?;
                            let expected = if name == "z list" {
                                (101, 100, 1)
                            } else if name == "short list" {
                                if source_is_epjson {
                                    (2, 1, 1)
                                } else {
                                    (2, 0, 1)
                                }
                            } else {
                                (1, 0, 1)
                            };
                            assert_eq!(
                                SourceThermalComfort::get_source_angle_factor_object_item(
                                    &mut input,
                                    &mut shortcut,
                                    ordinal,
                                    source_is_epjson,
                                )?,
                                expected
                            );
                            used.insert(name.to_string());
                            for original in
                                input.source_raw_object_names("ComfortViewFactorAngles")?
                            {
                                assert_eq!(
                                    input.is_object_unused("ComfortViewFactorAngles", original),
                                    !used.contains(original)
                                );
                            }
                            if name == "z list" {
                                assert_eq!(shortcut.alpha_args[100], "SURFACE 99");
                                assert!(shortcut.numeric_field_blanks.iter().all(|blank| !*blank));
                                continue;
                            }
                            assert_eq!(shortcut.alpha_args[0], name.to_ascii_uppercase());
                            assert!(!shortcut.alpha_field_blanks[0]);
                            assert!(shortcut.alpha_args[1..].iter().all(String::is_empty));
                            assert!(
                                shortcut
                                    .numeric_args
                                    .iter()
                                    .all(|value| value.to_bits() == 0)
                            );
                            assert!(shortcut.alpha_field_blanks[1..].iter().all(|blank| *blank));
                            assert!(shortcut.numeric_field_blanks.iter().all(|blank| *blank));
                            assert!(shortcut.alpha_field_names[2..].iter().all(String::is_empty));
                            assert!(
                                shortcut.numeric_field_names[1..]
                                    .iter()
                                    .all(String::is_empty)
                            );
                            if name == "EOF list" {
                                assert_eq!(shortcut.alpha_field_names[1], "");
                                assert_eq!(shortcut.numeric_field_names[0], "");
                            } else {
                                assert_eq!(
                                    shortcut.alpha_field_names[1],
                                    if source_is_epjson {
                                        "surface_name"
                                    } else {
                                        "Surface Name"
                                    }
                                );
                                assert_eq!(
                                    shortcut.numeric_field_names[0],
                                    if source_is_epjson {
                                        "angle_factor"
                                    } else {
                                        "Angle Factor"
                                    }
                                );
                            }
                        }
                    }
                }
                // People comfort/report child anchor: these are the same genuine
                // getter-populated People/schedules/Air/clock owners.
                // Connected comfort/report children use the genuine People getter checkpoint.
                // Keep the failed parent timestep and its unchanged-cache assertions above.
                if expected_count < 0.0 {
                    use crate::heat_balance::ordinary_thermal_comfort::{
                        SourceComfortPredefinedSummary, SourceThermalComfort,
                        SourceThermalComfortOwners,
                    };
                    use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as ComfortError;
                    use crate::heat_balance::source_output_registration::SourceOutputDictionaryStreams;
                    for (all_unoccupied, fanger_mode, ankle_enabled) in [
                        (false, None, false),
                        (true, None, false),
                        (
                            false,
                            Some(("ClothingInsulationSchedule", "Constant-1.0")),
                            false,
                        ),
                        (
                            false,
                            Some(("DynamicClothingModelASHRAE55", "missing unused method")),
                            false,
                        ),
                        (
                            false,
                            Some(("CalculationMethodSchedule", "Constant-1.0")),
                            false,
                        ),
                        (
                            false,
                            Some(("CalculationMethodSchedule", "Fanger Method Two")),
                            false,
                        ),
                        (false, None, true),
                        (
                            false,
                            Some(("ClothingInsulationSchedule", "Constant-1.0")),
                            true,
                        ),
                    ] {
                        let fanger_enabled = fanger_mode.is_some();
                        let mut comfort_reports =
                            if all_unoccupied || fanger_enabled || ankle_enabled {
                                people_getter_base.clone()
                            } else {
                                people_reports.clone()
                            };
                        let (
                            mut comfort_state,
                            mut comfort_errors,
                            mut comfort_console,
                            mut comfort_eio,
                            mut comfort_eso,
                        ) = (
                            ctf_ordinary_output_test_state(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        );
                        let mut comfort_clock = people_clock.clone();
                        comfort_clock.minutes_in_time_step = 15;
                        comfort_clock.kind_of_sim = 3;
                        comfort_clock.day_of_sim = 1;
                        comfort_clock.hour_of_day = 1;
                        comfort_clock.time_step = 1;
                        comfort_clock.num_of_day_in_envrn = 1;
                        if fanger_enabled || ankle_enabled {
                            // Only raw declared comfort inputs change. The actual getter
                            // resolves all schedules and retains genuine constructor owners.
                            let first = &mut comfort_reports
                                .heat_balance_input
                                .as_mut()
                                .ok_or("Fanger raw input")?
                                .objects
                                .get_mut("People")
                                .ok_or("Fanger raw People")?[0];
                            for (field, value) in [
                                (
                                    "thermal_comfort_model_1_type",
                                    if fanger_enabled {
                                        "Fanger"
                                    } else {
                                        "AnkleDraftASH55"
                                    },
                                ),
                                (
                                    "mean_radiant_temperature_calculation_type",
                                    "EnclosureAveraged",
                                ),
                                ("work_efficiency_schedule_name", "Constant-0.0"),
                                (
                                    "clothing_insulation_calculation_method",
                                    "ClothingInsulationSchedule",
                                ),
                                ("clothing_insulation_schedule_name", "Constant-1.0"),
                                ("air_velocity_schedule_name", "Unused People Auxiliary"),
                            ] {
                                first
                                    .fields
                                    .insert(field.into(), ProjectInputValue::String(value.into()));
                            }
                        }
                        if ankle_enabled {
                            let first = &mut comfort_reports
                                .heat_balance_input
                                .as_mut()
                                .ok_or("ankle raw input")?
                                .objects
                                .get_mut("People")
                                .ok_or("ankle raw People")?[0];
                            if fanger_enabled {
                                first.fields.insert(
                                    "thermal_comfort_model_2_type".into(),
                                    ProjectInputValue::String("AnkleDraftASH55".into()),
                                );
                            }
                            first.fields.insert(
                                "ankle_level_air_velocity_schedule_name".into(),
                                ProjectInputValue::String("Constant-0.0".into()),
                            );
                        }
                        if let Some((method, calculation)) = fanger_mode {
                            let first = &mut comfort_reports
                                .heat_balance_input
                                .as_mut()
                                .ok_or("clothing raw input")?
                                .objects
                                .get_mut("People")
                                .ok_or("clothing raw People")?[0];
                            first.fields.insert(
                                "clothing_insulation_calculation_method".into(),
                                ProjectInputValue::String(method.into()),
                            );
                            first.fields.insert(
                                "clothing_insulation_calculation_method_schedule_name".into(),
                                ProjectInputValue::String(calculation.into()),
                            );
                            if method == "DynamicClothingModelASHRAE55"
                                || calculation == "Fanger Method Two"
                            {
                                first.fields.insert(
                                    "clothing_insulation_schedule_name".into(),
                                    ProjectInputValue::String("missing unused clothing".into()),
                                );
                            }
                        }
                        if all_unoccupied {
                            // An authored raw schedule change and actual getter reread, never
                            // edited People pointers/currentVal/occupancy or report answers.
                            for row in comfort_reports
                                .heat_balance_input
                                .as_mut()
                                .ok_or("comfort raw People")?
                                .objects
                                .get_mut("People")
                                .ok_or("comfort raw People rows")?
                            {
                                row.fields.insert(
                                    "number_of_people_schedule_name".into(),
                                    ProjectInputValue::String("Constant-0.0".into()),
                                );
                            }
                        }
                        if all_unoccupied || fanger_enabled || ankle_enabled {
                            CtfInitializationContext {
                                reports: &mut comfort_reports,
                                eio: &mut comfort_eio,
                                backend: SourceErrorBackend {
                                    state: &mut comfort_state,
                                    error_stream: Some(&mut comfort_errors),
                                    console: &mut comfort_console,
                                    out_unit1: None,
                                    out_unit2: None,
                                    f_message: None,
                                    message_callback: None,
                                    error_callback: None,
                                    sqlite: None,
                                },
                            }
                            .manage_selected_internal_heat_gains_input_with_dictionary(
                                &people_model,
                                15,
                                &mut gain_host,
                                Some(&comfort_clock),
                                Some(SourceGainDictionaryFile {
                                    eso: Some(&mut comfort_eso),
                                }),
                            )?;
                        }
                        let comfort_globals = comfort_reports
                            .selected_heat_balance_globals
                            .as_ref()
                            .ok_or("comfort genuine globals")?;
                        let mut comfort_air = people_air.clone();
                        let comfort_environment = crate::weather::day::WeatherEnvironmentState {
                            month: 1,
                            day_of_month: 1,
                            run_period_environment: true,
                            ..Default::default()
                        };
                        let comfort_hvac =
                            crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals::default();
                        let comfort_controller = crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector::default();
                        let mut comfort_fan = crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays::default();
                        let mut comfort_psychrometrics =
                            crate::psychrometrics::SourcePsychrometricAccess;
                        let ids = comfort_reports
                            .surface_geometry_input
                            .final_surfaces
                            .as_deref()
                            .ok_or("actual comfort history final surfaces")?
                            .iter()
                            .map(|surface| {
                                surface
                                    .typed_id
                                    .ok_or("actual comfort history Surface identity")
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        let mut comfort_history = crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState::new_source_unallocated(ids);
                        let mut comfort_surface_arrays = crate::heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays::default();
                        comfort_surface_arrays.allocate(
                            comfort_history.surface_ids.len(),
                            comfort_reports
                                .heat_balance_input_state
                                .any_internal_heat_source_in_input,
                            cache
                                .public_initialization()
                                .ok_or("actual comfort CTF aggregate")?
                                .aggregate_after_returned_calls
                                .simple_ctf_only,
                            comfort_globals.any_energy_management_system_in_model,
                            &mut comfort_history,
                        )?;
                        let mut comfort = SourceThermalComfort::default();
                        assert_eq!(
                            (
                                comfort.act_met,
                                comfort.ankle_draft_air_vel_warning_ind,
                                comfort.ankle_draft_clo_unit_warning_ind,
                                comfort.ankle_draft_act_met_warning_ind
                            ),
                            (0.0, 0, 0, 0)
                        );
                        let mut comfort_summary = SourceComfortPredefinedSummary::default();
                        {
                            let mut comfort_backend = SourceErrorBackend {
                                state: &mut comfort_state,
                                error_stream: Some(&mut comfort_errors),
                                console: &mut comfort_console,
                                out_unit1: None,
                                out_unit2: None,
                                f_message: None,
                                message_callback: None,
                                error_callback: None,
                                sqlite: None,
                            };
                            let mut run_comfort = |initialize_only, current: &mut SourceThermalComfort,
                                               clock: &crate::weather::day::WeatherGlobalState,
                                               air: &crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance,
                                               environment: &crate::weather::day::WeatherEnvironmentState| {
                            let mut dictionary = SourceOutputDictionaryStreams {
                                eso: Some(&mut comfort_eso), sqlite_present: comfort_reports.source_sqlite.present(),
                                results_framework_output_variables: &mut comfort_reports.source_results_output_variables,
                            };
                            current.manage_source_thermal_comfort(initialize_only, &mut SourceThermalComfortOwners {
                                gains: &mut comfort_reports.selected_internal_heat_gains,
                                schedules: comfort_reports.selected_schedule_manager.as_ref().ok_or(ComfortError::Unavailable("comfort genuine schedules"))?,
                                zones: comfort_reports.selected_zone_input.as_ref().ok_or(ComfortError::Unavailable("comfort genuine Zones"))?,
                                control: &comfort_reports.heat_balance_input_state, globals: comfort_globals,
                                clock, environment, hvac: &comfort_hvac, air,
                                developer: &comfort_reports.selected_project_data.as_ref().ok_or(ComfortError::Unavailable("comfort actual project"))?.system.developer_flag,
                                controller: &comfort_controller, fan: &mut comfort_fan,
                                radiation: comfort_reports.ordinary_solar_radiation.as_ref().ok_or(ComfortError::Unavailable("comfort actual retained radiation"))?,
                                geometry: &mut comfort_reports.surface_geometry_input, history: &comfort_history,
                                psychrometrics: &mut comfort_psychrometrics,
                                recurring_errors: &mut comfort_reports.source_recurring_errors,
                                input: comfort_reports.selected_input_processor.as_mut().ok_or(ComfortError::Unavailable("comfort genuine IP"))?,
                                source_is_epjson: &comfort_reports.heat_balance_input.as_ref().ok_or(ComfortError::Unavailable("comfort actual input format"))?.source_is_epjson,
                                shortcut: &mut comfort_reports.input_processor_shortcut,
                                registration: &mut comfort_reports.output_registration,
                                prescan: &comfort_reports.output_prescan, report_inputs: &people_model.report_inputs,
                                minimum_report_frequency: "", host: &mut gain_host, dictionary_host: &mut dictionary,
                                eio: &mut comfort_eio, backend: &mut comfort_backend,
                                registry: &mut comfort_reports.registry, table: &mut comfort_reports.table,
                                predefined_summary: &mut comfort_summary,
                            })
                        };
                            run_comfort(
                                true,
                                &mut comfort,
                                &comfort_clock,
                                &comfort_air,
                                &comfort_environment,
                            )?;
                            assert!(!comfort.first_time_flag);
                            assert_eq!(
                                comfort
                                    .thermal_comfort_data
                                    .as_ref()
                                    .ok_or("real comfort rows")?
                                    .len(),
                                2
                            );
                            if fanger_enabled || ankle_enabled {
                                // This fixture reran the real Zone getter, producing a new
                                // Space with ctor0 enclosure identity. No fabricated link
                                // is inserted after the genuine selected registrations.
                                let reached = run_comfort(
                                    false,
                                    &mut comfort,
                                    &comfort_clock,
                                    &comfort_air,
                                    &comfort_environment,
                                );
                                assert!(
                                    matches!(
                                        &reached,
                                        Err(ComfortError::Unavailable(
                                            "positive actual ThermalComfort ordinal"
                                        ))
                                    ),
                                    "actual positive comfort prefix: {reached:?}"
                                );
                                use crate::heat_balance::source_output_registration::{
                                    SourceComfortOutputField as F, SourceOutputStoreType as S,
                                    SourceOutputUnits as U, TimeStepKind,
                                };
                                let check_ankle = |reports: &CtfInitializationReports, current: &SourceThermalComfort| -> Result<(), Box<dyn std::error::Error>> {
                                    let ankle = reports.output_registration.dictionary_variables.iter()
                                        .find(|row| row.name == "Zone Thermal Comfort ASHRAE 55 Ankle Draft PPD")
                                        .ok_or("genuine ankle dictionary registration")?;
                                    assert_eq!((ankle.units, ankle.store_type, ankle.time_step_type),
                                        (U::Dimensionless, S::Average, TimeStepKind::Zone));
                                    let row = &current.thermal_comfort_data.as_ref().ok_or("actual ankle row")?[0];
                                    assert!(std::ptr::eq(current.real_output(F::PeopleAnkleDraftPpd,1)?, &row.ankle_draft_ppdash55));
                                    assert_eq!(row.ankle_draft_ppdash55, 0.0, "fresh Space0 stops before any computed ankle output");
                                    let person = &reports.selected_internal_heat_gains.allocations.people.rows[0];
                                    assert!(person.ankle_draft_ash55 && person.ankle_air_velocity_sched.is_some());
                                    assert_eq!(person.fanger, fanger_enabled);
                                    assert!(reports.heat_balance_input_state.any_thermal_comfort_ankle_draft_model);
                                    assert_eq!((current.ankle_draft_air_vel_warning_ind, current.ankle_draft_clo_unit_warning_ind,
                                        current.ankle_draft_act_met_warning_ind), (0,0,0));
                                    Ok(())
                                };
                                if !fanger_enabled {
                                    // End the genuine mutable owner borrow before inspecting
                                    // the ankle-only dictionary and then leaving this case.
                                    check_ankle(&comfort_reports, &comfort)?;
                                    for name in [
                                        "Zone Thermal Comfort Fanger Model PMV",
                                        "Zone Thermal Comfort Fanger Model PPD",
                                        "Zone Thermal Comfort Clothing Surface Temperature",
                                        "Zone Thermal Comfort Mean Radiant Temperature",
                                        "Zone Thermal Comfort Operative Temperature",
                                        "Zone Thermal Comfort Clothing Value",
                                    ] {
                                        assert!(
                                            !comfort_reports
                                                .output_registration
                                                .dictionary_variables
                                                .iter()
                                                .any(|row| row.name == name)
                                        );
                                    }
                                    let person = &comfort_reports
                                        .selected_internal_heat_gains
                                        .allocations
                                        .people
                                        .rows[0];
                                    assert_eq!(
                                        (
                                            person.temperature_in_zone,
                                            person.relative_humidity_in_zone
                                        ),
                                        (0.0, 0.0)
                                    );
                                    continue;
                                }
                                let declarations = [
                                    (
                                        "Zone Thermal Comfort Fanger Model PMV",
                                        F::PeopleFangerPmv,
                                        U::Dimensionless,
                                    ),
                                    (
                                        "Zone Thermal Comfort Fanger Model PPD",
                                        F::PeopleFangerPpd,
                                        U::Percentage,
                                    ),
                                    (
                                        "Zone Thermal Comfort Clothing Surface Temperature",
                                        F::PeopleClothingSurfaceTemperature,
                                        U::C,
                                    ),
                                    (
                                        "Zone Thermal Comfort Mean Radiant Temperature",
                                        F::PeopleMeanRadiantTemperature,
                                        U::C,
                                    ),
                                    (
                                        "Zone Thermal Comfort Operative Temperature",
                                        F::PeopleOperativeTemperature,
                                        U::C,
                                    ),
                                    (
                                        "Zone Thermal Comfort Clothing Value",
                                        F::PeopleClothingValue,
                                        U::Clo,
                                    ),
                                ];
                                let (method, calculation) =
                                    fanger_mode.ok_or("actual selected Fanger clothing test")?;
                                if method == "DynamicClothingModelASHRAE55" {
                                    // Manage(true), not a direct scratch-field assignment,
                                    // writes the actual clock-selected six-AM weather operand.
                                    for (temperature, expected) in [
                                        (-10.0, 1.0),
                                        (-5.0, 1.0),
                                        (0.0, 0.818),
                                        (5.0, 0.6360628048372603),
                                        (25.0, 0.4693534343171951),
                                        (26.0, 0.46),
                                    ] {
                                        let mut weather = comfort_environment.clone();
                                        weather.current_weather.out_dry_bulb_temp = temperature;
                                        let mut clock = comfort_clock.clone();
                                        clock.day_of_sim = 2;
                                        clock.hour_of_day = 7;
                                        clock.time_step = 1;
                                        run_comfort(
                                            true,
                                            &mut comfort,
                                            &clock,
                                            &comfort_air,
                                            &weather,
                                        )?;
                                        assert_eq!(
                                            comfort.temporary_six_am_temperature,
                                            temperature
                                        );
                                        comfort.calc_source_dynamic_clothing()?;
                                        assert!(
                                            (comfort
                                                .thermal_comfort_data
                                                .as_ref()
                                                .ok_or("dynamic row")?[0]
                                                .clothing_value
                                                - expected)
                                                .abs()
                                                < 1.0e-12
                                        );
                                        // Later timesteps retain the real six-AM cell despite
                                        // an unrelated current weather change.
                                        clock.time_step = 2;
                                        weather.current_weather.out_dry_bulb_temp = 40.0;
                                        run_comfort(
                                            true,
                                            &mut comfort,
                                            &clock,
                                            &comfort_air,
                                            &weather,
                                        )?;
                                        assert_eq!(
                                            comfort.temporary_six_am_temperature,
                                            temperature
                                        );
                                    }
                                    let mut first_day = comfort_clock.clone();
                                    first_day.day_of_sim = 1;
                                    first_day.hour_of_day = 6;
                                    run_comfort(
                                        true,
                                        &mut comfort,
                                        &first_day,
                                        &comfort_air,
                                        &comfort_environment,
                                    )?;
                                    assert_eq!(comfort.temporary_six_am_temperature, 1.868132);
                                }
                                // Last Manage(true) use above has completed. Inspection
                                // must not overlap the closure's mutable owner captures.
                                if ankle_enabled {
                                    check_ankle(&comfort_reports, &comfort)?;
                                }
                                let mut prior = None;
                                for (name, field, units) in declarations {
                                    let at = comfort_reports
                                        .output_registration
                                        .dictionary_variables
                                        .iter()
                                        .position(|row| row.name == name)
                                        .ok_or("actual Fanger dictionary declaration")?;
                                    if let Some(previous) = prior {
                                        assert!(at > previous);
                                    }
                                    prior = Some(at);
                                    let row = &comfort_reports
                                        .output_registration
                                        .dictionary_variables[at];
                                    assert_eq!(
                                        (row.units, row.store_type, row.time_step_type),
                                        (units, S::Average, TimeStepKind::Zone)
                                    );
                                    assert_eq!(
                                        *comfort.real_output(field, 1)?,
                                        if field == F::PeopleClothingValue
                                            && method == "DynamicClothingModelASHRAE55"
                                        {
                                            0.46
                                        } else {
                                            0.0
                                        }
                                    );
                                }
                                assert_eq!(U::Clo.symbol(), "clo");
                                let first = &comfort_reports
                                    .selected_internal_heat_gains
                                    .allocations
                                    .people
                                    .rows[0];
                                assert!(
                                    first.fanger
                                        && first.work_eff_sched.is_some()
                                        && first.air_velocity_sched.is_some()
                                );
                                let expected_type = match method {
                                    "ClothingInsulationSchedule" => 0,
                                    "DynamicClothingModelASHRAE55" => 1,
                                    "CalculationMethodSchedule" => 2,
                                    _ => return Err("authored clothing test enum".into()),
                                };
                                assert_eq!(
                                    (first.mrt_calc_type, first.clothing_type),
                                    (0, expected_type)
                                );
                                assert_eq!(
                                    first.clothing_sched.is_some(),
                                    expected_type == 0 || calculation == "Constant-1.0"
                                );
                                assert_eq!(
                                    first.clothing_method_sched.is_some(),
                                    expected_type == 2
                                );
                                assert!(!comfort.first_time_flag);
                                // Original ThermalComfort.unit.cc2079-2116 scalar fixtures,
                                // executed against the real shared Psat state/backend. They
                                // neither construct enclosure readiness nor seed parent data.
                                crate::psychrometrics::with_fresh_psychrometric_state(
                                    || -> Result<(), Box<dyn std::error::Error>> {
                                        crate::psychrometrics::initialize_source_psychrometric_caches();
                                        for (
                                            air,
                                            mrt,
                                            rh,
                                            velocity,
                                            activity,
                                            clothing,
                                            expected,
                                        ) in [
                                            (20.0, 18.0, 0.50, 1.0, 58.2, 1.5, -0.715102),
                                            (25.0, 28.0, 0.75, 1.0, 80.0, 0.8, 0.644378),
                                        ] {
                                            let pmv = comfort.calc_source_fanger_pmv(air, mrt, rh, velocity, activity, clothing, 0.0,
                                            &mut comfort_psychrometrics, &mut crate::psychrometrics::SourcePsychrometricOwners {
                                                clock: &comfort_clock, globals: comfort_globals, environment: &comfort_environment,
                                                hvac: &comfort_hvac, recurring: &mut comfort_reports.source_recurring_errors,
                                                backend: &mut comfort_backend,
                                            })?;
                                            assert!(
                                                (pmv - expected).abs() < 0.00001,
                                                "literal Native Fanger scalar fixture: {pmv}"
                                            );
                                        }
                                        Ok(())
                                    },
                                )?;
                                continue;
                            }
                            // Authored averaged air points cover both polygons independently,
                            // neither polygon, and the Native fBC==0 extended-edge quirk.
                            for (temperature, humidity, not_summer, not_winter, not_either) in [
                                (22.0, 0.006, true, false, false),
                                (27.0, 0.006, false, true, false),
                                (25.0, 0.006, false, false, false),
                                (18.0, 0.006, true, true, true),
                                (100.0, 0.012, false, false, false),
                            ] {
                                let air =
                                    &mut comfort_air.zones.as_mut().ok_or("comfort real AirHB")?[0]
                                        .air;
                                air.ztav_comf = temperature;
                                air.mrt = temperature;
                                air.air_hum_rat_avg_comf = humidity;
                                let reached = run_comfort(
                                    false,
                                    &mut comfort,
                                    &comfort_clock,
                                    &comfort_air,
                                    &comfort_environment,
                                );
                                assert!(matches!(&reached, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("actual same Zone thermostat setpoint"))),
                                "the real SimpleASH child precedes the fixture's absent thermostat owner: {reached:?}");
                                let row = &comfort
                                    .thermal_comfort_in_ash55
                                    .as_ref()
                                    .ok_or("real SimpleASH row")?[0];
                                assert_eq!(row.zone_is_occupied, !all_unoccupied);
                                for (actual, fails) in [
                                    (row.time_not_summer, not_summer),
                                    (row.time_not_winter, not_winter),
                                    (row.time_not_either, not_either),
                                ] {
                                    assert_eq!(
                                        actual,
                                        if !all_unoccupied && fails { 0.25 } else { 0.0 }
                                    );
                                }
                                assert_eq!(
                                    comfort.any_zone_time_not_simple_ash55_either,
                                    row.time_not_either
                                );
                            }
                            let row = &comfort
                                .thermal_comfort_in_ash55
                                .as_ref()
                                .ok_or("real SimpleASH totals")?[0];
                            assert_eq!(
                                row.total_time_not_summer,
                                if all_unoccupied { 0.0 } else { 0.5 }
                            );
                            assert_eq!(
                                row.total_time_not_winter,
                                if all_unoccupied { 0.0 } else { 0.5 }
                            );
                            assert_eq!(
                                row.total_time_not_either,
                                if all_unoccupied { 0.0 } else { 0.25 }
                            );
                            for step in 0..4 {
                                let air = &mut comfort_air
                                    .zones
                                    .as_mut()
                                    .ok_or("comfort end-period AirHB")?[0]
                                    .air;
                                air.ztav_comf = 18.0;
                                air.mrt = 18.0;
                                air.air_hum_rat_avg_comf = 0.006;
                                comfort_clock.end_design_day_envrns_flag = step == 3;
                                let reached = run_comfort(
                                    false,
                                    &mut comfort,
                                    &comfort_clock,
                                    &comfort_air,
                                    &comfort_environment,
                                );
                                assert!(matches!(&reached, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("actual same Zone thermostat setpoint"))), "comfort report prefix: {reached:?}");
                            }
                        }
                        assert_eq!(
                            comfort.zone_occ_hrs.as_ref().ok_or("real occupied hours")?[0],
                            if all_unoccupied { 0.0 } else { 2.25 }
                        );
                        assert_eq!(
                            comfort_summary.total_time_not_simple_ash55_either_for_abups,
                            if all_unoccupied { 0.0 } else { 1.25 }
                        );
                        assert_eq!(comfort.total_any_zone_time_not_simple_ash55_either, 0.0);
                        let row = &comfort
                            .thermal_comfort_in_ash55
                            .as_ref()
                            .ok_or("real period reset")?[0];
                        assert_eq!(
                            (
                                row.total_time_not_summer,
                                row.total_time_not_winter,
                                row.total_time_not_either
                            ),
                            (0.0, 0.0, 0.0)
                        );
                        let column = *comfort_reports
                            .registry
                            .named_indices
                            .get("pdchSCeitherClothes")
                            .ok_or("actual comfort column")?;
                        assert!(
                            comfort_reports
                                .table
                                .entries
                                .as_ref()
                                .ok_or("actual comfort table")?
                                .iter()
                                .take(usize::try_from(comfort_reports.table.num_table_entry)?)
                                .any(|entry| entry.index_column == column
                                    && entry.object_name == "Facility"
                                    && entry.original_entry_is_real
                                    && entry.original_real_entry
                                        == if all_unoccupied { 0.0 } else { 1.25 })
                        );
                        assert_eq!(
                            comfort_state.tracking.total_warning_errors,
                            i32::from(!all_unoccupied)
                        );
                        for person in comfort_reports
                            .selected_internal_heat_gains
                            .allocations
                            .people
                            .iter()
                        {
                            assert_eq!(
                                (person.temperature_in_zone, person.relative_humidity_in_zone),
                                (0.0, 0.0)
                            );
                        }
                        for row in comfort
                            .thermal_comfort_data
                            .as_ref()
                            .ok_or("actual false-Fanger rows")?
                        {
                            assert_eq!((row.fanger_pmv, row.pierce_set), (0.0, 0.0));
                        }

                        // The same actual schedules/Air/comfort rows reach the independent
                        // weather-report child. No report-state answers are seeded here.
                        let mut resilience =
                            crate::heat_balance::ordinary_surface_management::SourceSurfaceManagement::default(
                            );
                        let weather = crate::weather::day::WeatherOwnerState::default();
                        let report = crate::heat_balance::warmup::report::WarmupReportState::new_source_unallocated(
                            people_model.zones.iter().map(|zone| zone.name.0.clone()).collect(),
                            crate::heat_balance::warmup::report::WarmupReportConfiguration {
                                doing_sizing: comfort_globals.doing_sizing,
                                do_pure_load_calc: comfort_globals.do_pure_load_calc,
                                run_period_environment: comfort_environment.run_period_environment,
                                environment_name: comfort_environment.environment_name.clone(),
                                detailed: comfort_reports.selected_project_data.as_ref()
                                    .ok_or("actual comfort report project")?
                                    .system.report_detailed_warmup_convergence,
                            },
                        );
                        let mut heat_balance = crate::heat_balance::ordinary_heat_balance_initialization::SourceHeatBalanceManager::new(report)?;
                        let daylighting =
                            crate::heat_balance::ordinary_daylighting::SourceDaylighting::default();
                        let mut tabular =
                            crate::heat_balance::ordinary_tabular_reports::SourceTabularReports::default();
                        for (temperature, hour, step) in [(15.56, 1, 1), (30.0, 2, 3)] {
                            comfort_air
                                .zones
                                .as_mut()
                                .ok_or("weather-report genuine AirHB")?[0]
                                .air
                                .ztav = temperature;
                            comfort_clock.hour_of_day = hour;
                            comfort_clock.time_step = step;
                            comfort_clock.end_design_day_envrns_flag = false;
                            let mut report_backend = SourceErrorBackend {
                                state: &mut comfort_state,
                                error_stream: Some(&mut comfort_errors),
                                console: &mut comfort_console,
                                out_unit1: None,
                                out_unit2: None,
                                f_message: None,
                                message_callback: None,
                                error_callback: None,
                                sqlite: None,
                            };
                            let mut weather_owners = crate::heat_balance::ordinary_surface_management::SourceResilienceReportOwners {
                            globals: comfort_globals,
                            zones: comfort_reports.selected_zone_input.as_mut().ok_or("real weather-report Zones")?,
                            air: &comfort_air, fan: &comfort_fan, clock: &comfort_clock,
                            weather: &weather, environment: &comfort_environment,
                            control: &comfort_reports.heat_balance_input_state,
                            gains: &comfort_reports.selected_internal_heat_gains,
                            schedules: comfort_reports.selected_schedule_manager.as_ref().ok_or("real weather-report schedules")?,
                            comfort: &comfort, heat_balance: &mut heat_balance, daylighting: &daylighting,
                            tabular: &mut tabular, backend: &mut report_backend,
                        };
                            if hour == 1 {
                                let reached = resilience
                                    .report_source_thermal_resilience(&mut weather_owners);
                                assert!(
                                    matches!(
                                        &reached,
                                        Err(ComfortError::Unavailable(
                                            "actual allocated low_s_e_t_longest_hours_rep_period"
                                        ))
                                    ),
                                    "the genuine outer first-call prefix requires the full InitHeatBalance report matrices: {reached:?}"
                                );
                                assert!(resilience.report_thermal_resilience_first_time);
                                assert!(!resilience.has_pierce_set);
                            }
                            // An explicitly invoked real child after the failed parent above;
                            // no matrix allocation, latch or successful-parent claim is added.
                            let reached =
                                resilience.report_source_thermal_weather(&mut weather_owners);
                            assert!(
                                matches!(
                                    &reached,
                                    Err(ComfortError::Unavailable("actual zoneTstatSetpts6146"))
                                ),
                                "real People safety/PMV bins precede this fixture's absent thermostat: {reached:?}"
                            );
                        }
                        assert!(resilience.report_thermal_resilience_first_time);
                        assert!(!resilience.has_pierce_set);
                        let row = &comfort_reports
                            .selected_zone_input
                            .as_ref()
                            .ok_or("actual weather-report owner")?
                            .resilience
                            .as_ref()
                            .ok_or("genuine GetZoneData resilience allocation")?[0];
                        let last_occupants = if all_unoccupied { -0.0_f64 } else { -1.0 };
                        assert_eq!(
                            row.zone_num_occ.to_bits(),
                            last_occupants.to_bits(),
                            "Native overwrites ZoneNumOcc per People instead of summing or consuming EMS NumOcc"
                        );
                        assert_eq!(
                            (row.pmv, row.zone_pierce_set, row.zone_pierce_set_last_step),
                            (0.0, -1.0, -1.0)
                        );
                        assert!(row.crossed_cold_thresh && row.crossed_heat_thresh);
                        for (bins, first_time) in [
                            (&row.zone_cold_hour_of_safety_bins, 1010100.0),
                            (&row.zone_heat_hour_of_safety_bins, 1010230.0),
                        ] {
                            assert_eq!(bins[1], first_time);
                            assert_eq!(bins[2], 0.5);
                            assert_eq!(bins[3], if all_unoccupied { 0.0 } else { 0.125 });
                            assert_eq!(bins[4], if all_unoccupied { 0.0 } else { 0.25 });
                        }
                        assert_eq!(row.zone_discomfort_wt_exceed_occu_hour_bins, [0.0; 4]);
                        assert_eq!(row.zone_discomfort_wt_exceed_occupied_hour_bins, [0.0; 4]);

                        // Consume an authored real SummaryReports object before the adaptive
                        // writer. No display/input-complete flag is assigned by this test.
                        let sizing = crate::heat_balance::selected_sizing::SelectedSizingInput {
                            summary_report_objects: vec![
                                crate::heat_balance::selected_sizing::SelectedSummaryReportInput {
                                    object_name: "People comfort summary".into(),
                                    reports: vec!["AdaptiveComfortSummary".into()],
                                    idf_max_fields: None,
                                    idf_max_extensible_fields: None,
                                },
                            ],
                        };
                        let raw =
                            crate::heat_balance::ordinary_tabular_reports::SelectedTabularInput {
                                source_is_epjson: true,
                                table_style_objects: Vec::new(),
                            };
                        let inventory = crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::FAMILIES
                        .into_iter().filter_map(|family| match comfort_reports.selected_input_processor.as_ref()?.family(family) {
                            Some(crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(rows)) => Some((family, rows.len())),
                            _ => None,
                        });
                        let equipment_clock = crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::from_source_family_inventory(
                        inventory, usize::try_from(people_model.timestep.number_of_timesteps_per_hour)?,
                    )?;
                        let equipment =
                            crate::heat_balance::ordinary_equipment::OrdinaryEquipment::new_for_source(
                                &people_model,
                                equipment_clock,
                            )?;
                        let solar_input = crate::heat_balance::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory(
                        ["ZoneProperty:UserViewFactors:BySurfaceName", "PerformancePrecisionTradeoffs", "DaylightingDevice:Shelf"]
                            .into_iter().map(|family| (family, comfort_reports.selected_input_processor.as_ref()
                                .map_or(0, |input| input.get_num_objects_found(family)))), false,
                    )?;
                        let radiation =
                            crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(
                                solar_input,
                            );
                        let surface = crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization::default();
                        let surface_averages = crate::heat_balance::ordinary_surface_source_averages::SourceFinalSurfaceHeatBalance::default();
                        let emissions = crate::heat_balance::ordinary_hvac_manager::heat_emission::state::SourceHeatEmissionDeviceModules::default();
                        let modules =
                            crate::heat_balance::selected_gain_modules::SourceGainModules::default(
                            );
                        let mut pollution =
                            crate::heat_balance::ordinary_pollution::SourcePollutionModule::default(
                            );
                        {
                            let mut report_backend = SourceErrorBackend {
                                state: &mut comfort_state,
                                error_stream: Some(&mut comfort_errors),
                                console: &mut comfort_console,
                                out_unit1: None,
                                out_unit2: None,
                                f_message: None,
                                message_callback: None,
                                error_callback: None,
                                sqlite: None,
                            };
                            tabular.update_source_tabular_reports(
                                crate::heat_balance::source_output_registration::TimeStepKind::Zone,
                                &mut crate::heat_balance::ordinary_tabular_reports::SourceTabularInputOwners {
                                    files: &comfort_reports.source_output_control,
                                    results: &comfort_reports.source_results_framework,
                                    input: comfort_reports
                                        .selected_input_processor
                                        .as_mut()
                                        .ok_or("real summary IP")?,
                                    shortcut: &mut comfort_reports.input_processor_shortcut,
                                    raw: &raw,
                                    sizing_raw: &sizing,
                                    globals: comfort_globals,
                                    clock: &comfort_clock,
                                    schedules: comfort_reports
                                        .selected_schedule_manager
                                        .as_ref()
                                        .ok_or("real summary schedules")?,
                                    environment: &comfort_environment,
                                    zones: comfort_reports
                                        .selected_zone_input
                                        .as_mut()
                                        .ok_or("real summary Zones")?,
                                    gains: &comfort_reports.selected_internal_heat_gains,
                                    registry: &mut comfort_reports.registry,
                                    registration: &comfort_reports.output_registration,
                                    backend: &mut report_backend,
                                    eio: &mut comfort_eio,
                                    pollution: &mut pollution,
                                    equipment: &equipment,
                                    space_controls: &comfort_reports.heat_balance_space_controls,
                                    hvac: &comfort_hvac,
                                    control: &mut comfort_reports.heat_balance_input_state,
                                    gain_modules: &modules,
                                    heat_emission: &emissions,
                                    surface_heat_emission: &surface.sum_surface_heat_emission,
                                    controller: &comfort_controller,
                                    radiation: &radiation,
                                    surface_averages: &surface_averages,
                                },
                            )?;
                            assert!(tabular.display_adaptive_comfort);
                            assert!(!tabular.update_tabular_reports_get_input);
                            let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
                                kickoff: &comfort_globals.kickoff_simulation,
                                developer: &comfort_reports
                                    .selected_project_data
                                    .as_ref()
                                    .ok_or("actual summary project")?
                                    .system
                                    .developer_flag,
                                warmup: &comfort_clock.warmup_flag,
                                doing_sizing: &comfort_globals.doing_sizing,
                            };
                            let reached = tabular.write_source_adaptive_comfort_table(
                            &mut crate::heat_balance::ordinary_tabular_reports::SourceAdaptiveComfortReportOwners {
                                gains: &comfort_reports.selected_internal_heat_gains,
                                sqlite: &comfort_reports.source_sqlite, results: &comfort_reports.source_results_framework,
                                writer: crate::heat_balance::ordinary_tabular_reports::SourceTabularWriterOwners {
                                    monetary: &comfort_reports.cost_estimate_monetary,
                                    flags, backend: &mut report_backend,
                                },
                            });
                            assert!(
                                matches!(
                                    &reached,
                                    Err(ComfortError::Unavailable(
                                        "actual unopened TabularOutputFile insertion"
                                    ))
                                ),
                                "the actual zero-row adaptive branch reaches this fixture's genuine unopened report stream: {reached:?}"
                            );
                            assert_eq!(tabular.num_people_adaptive, 0);
                        }
                    }
                }
            }
        }
        // Same actual getter, selected local Numeric2, retained geometry and
        // live diagnostic owners. No Native observations or typed gain values
        // supply this authored boundary table.
        for (method, density, area, remainder, prior_error, warmup, kickoff, sizing, idf) in [
            (
                "Watts/Area",
                Some(3.7),
                25.0,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            (
                "Power/Area",
                Some(7.5),
                25.0,
                false,
                false,
                false,
                false,
                false,
                true,
            ),
            (
                "Power/Area",
                Some(0.0),
                25.0,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            (
                "Watts/Area",
                Some(-0.0),
                25.0,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            (
                "Power/Area",
                None,
                25.0,
                false,
                false,
                true,
                false,
                false,
                false,
            ),
            (
                "Power/Area",
                Some(3.7),
                0.0,
                false,
                false,
                true,
                false,
                false,
                false,
            ),
            (
                "Watts/Area",
                Some(3.7),
                0.0,
                true,
                false,
                false,
                false,
                false,
                false,
            ),
            (
                "Power/Area",
                Some(3.7),
                0.0,
                false,
                true,
                false,
                false,
                false,
                false,
            ),
            (
                "Power/Area",
                None,
                0.0,
                false,
                true,
                true,
                false,
                false,
                false,
            ),
            (
                "Power/Area",
                None,
                25.0,
                false,
                false,
                true,
                true,
                false,
                false,
            ),
            (
                "Power/Area",
                None,
                25.0,
                false,
                false,
                true,
                false,
                true,
                false,
            ),
            (
                "Power/Area",
                Some(-0.125),
                25.0,
                false,
                false,
                true,
                true,
                false,
                false,
            ),
        ] {
            let mut reports = failed_gains.clone();
            reports.selected_internal_heat_gains.errors_found = prior_error;
            let globals = reports
                .selected_heat_balance_globals
                .as_mut()
                .ok_or("area globals")?;
            globals.kickoff_simulation = kickoff;
            globals.doing_sizing = sizing;
            let space = &mut reports
                .selected_zone_input
                .as_mut()
                .and_then(|z| z.spaces.as_mut())
                .ok_or("area Space")?[0];
            assert!(
                !space.is_remainder_space,
                "actual implicit Space constructor"
            );
            space.floor_area = area;
            space.is_remainder_space = remainder;
            // Distinct caller-owned Zone area cannot replace the selected
            // Space operand. This unit override makes a wrong read observable.
            reports
                .selected_zone_input
                .as_mut()
                .and_then(|z| z.zones.as_mut())
                .ok_or("area Zone")?[0]
                .floor_area = 81.0;
            let raw = reports.heat_balance_input.as_mut().ok_or("area raw")?;
            raw.source_is_epjson = !idf;
            let objects = raw
                .objects
                .get_mut("OtherEquipment")
                .ok_or("area objects")?;
            let fields = &mut objects[0].fields;
            fields.insert(
                "design_level_calculation_method".into(),
                ProjectInputValue::String(method.into()),
            );
            fields.insert("design_level".into(), ProjectInputValue::Number(352.0));
            fields.insert("power_per_person".into(), ProjectInputValue::Number(9.0));
            if let Some(value) = density {
                fields.insert(
                    "power_per_floor_area".into(),
                    ProjectInputValue::Number(value),
                );
            } else {
                fields.remove("power_per_floor_area");
            }
            if idf {
                for object in objects {
                    object.idf_max_fields = Some(13);
                }
            }
            let clock = crate::weather::day::WeatherGlobalState {
                warmup_flag: warmup,
                ..Default::default()
            };
            let (mut state, mut errors, mut console, mut eio) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            let result = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut errors),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            );
            let negative = density.is_some_and(|v| v < 0.0);
            let fatal = prior_error || negative;
            if fatal {
                assert!(matches!(
                    result,
                    Err(crate::RuntimeError::HeatBalanceInput(_))
                ));
            } else {
                result?;
            }
            let gains = &reports.selected_internal_heat_gains;
            assert_eq!(gains.get_input_flag, fatal);
            assert_eq!(gains.errors_found, fatal);
            let rows = gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("actual area rows")?;
            let expected = if density.is_none() || fatal {
                0.0
            } else {
                density.ok_or("density")? * area
            };
            assert_eq!(rows[0].design_level.to_bits(), expected.to_bits());
            assert_eq!(rows[1].design_level, if fatal { 0.0 } else { -3.0 });
            assert_eq!(
                rows[0].nominal_min_design_level.to_bits(),
                expected.to_bits()
            );
            assert_eq!(
                rows[0].nominal_max_design_level.to_bits(),
                expected.to_bits()
            );
            assert_eq!(
                rows[0].fraction_radiant, 0.2,
                "fractions follow design-level return"
            );
            let warnings = i32::from(density.is_none() || (!fatal && area <= 0.0 && !remainder));
            assert_eq!(state.tracking.total_warning_errors, warnings);
            assert_eq!(
                state.tracking.total_warning_errors_during_warmup,
                if warmup && !kickoff && !sizing {
                    warnings
                } else {
                    0
                }
            );
            assert_eq!(
                state.tracking.total_warning_errors_during_sizing,
                if sizing { warnings } else { 0 }
            );
            assert_eq!(state.tracking.total_severe_errors, i32::from(negative));
            assert_eq!(
                state.tracking.total_severe_errors_during_warmup,
                if warmup && !kickoff && !sizing {
                    i32::from(negative)
                } else {
                    0
                }
            );
            assert!(
                !state.warmup_flag && !state.kickoff_simulation && !state.doing_sizing,
                "backend constructor flags never synchronized"
            );
            let text = String::from_utf8(errors)?;
            let field = if idf {
                "Power per Floor Area"
            } else {
                "power_per_floor_area"
            };
            if density.is_none() {
                assert!(text.contains(&format!(
                    "specifies {field}, but that field is blank.  0 OtherEquipment will result."
                )));
            }
            if warnings > 0 && density.is_some() {
                assert!(text.contains("Space Floor Area = 0.  0 OtherEquipment will result."));
            }
            if negative {
                assert!(text.contains("invalid power_per_floor_area, value  [<0.0]=-0.125"));
            }
            if fatal {
                assert!(text.contains(" **  Fatal  ** GetInternalHeatGains: Errors found"));
                assert!(!String::from_utf8(eio)?.contains("OtherEquipment Internal Gains Nominal"));
            } else {
                assert!(String::from_utf8(eio)?.contains("OtherEquipment Internal Gains Nominal"));
            }
        }
        {
            // A failed warning happens before the child returns and before the
            // equipment assignment/nominal/fraction/device writes.
            let mut area_failure = failed_gains.clone();
            let fields = &mut area_failure
                .heat_balance_input
                .as_mut()
                .ok_or("warning raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("warning rows")?[0]
                .fields;
            fields.insert(
                "design_level_calculation_method".into(),
                ProjectInputValue::String("Power/Area".into()),
            );
            let clock = crate::weather::day::WeatherGlobalState {
                warmup_flag: true,
                ..Default::default()
            };
            let (mut state, mut console, mut eio, mut errors) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                OrdinaryFailingEio,
            );
            let result = CtfInitializationContext {
                reports: &mut area_failure,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut errors),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            );
            assert!(matches!(result, Err(crate::RuntimeError::WarmupOutput(_))));
            let gains = &area_failure.selected_internal_heat_gains;
            assert!(gains.get_input_flag && !gains.errors_found);
            let rows = gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("failed area rows")?;
            assert_eq!(rows[0].design_level.to_bits(), 0.0_f64.to_bits());
            assert_eq!(rows[0].fraction_radiant, 0.0);
            assert_eq!(state.tracking.total_warning_errors_during_warmup, 1);
            assert!(eio.is_empty(), "nominal EIO remains unreached");
        }
        // Omitted direct Numeric1 reaches the real blank-warning child. The
        // unused finite operands cannot replace it, even after prior errors.
        for (idf, warmup, kickoff, sizing, prior_error) in [
            (false, true, false, false, false),
            (true, true, false, false, false),
            (false, true, true, false, false),
            (false, true, false, true, false),
            (false, true, false, false, true),
        ] {
            let mut reports = failed_gains.clone();
            reports.selected_internal_heat_gains.errors_found = prior_error;
            let globals = reports
                .selected_heat_balance_globals
                .as_mut()
                .ok_or("blank globals")?;
            globals.kickoff_simulation = kickoff;
            globals.doing_sizing = sizing;
            let raw = reports
                .heat_balance_input
                .as_mut()
                .ok_or("blank direct raw")?;
            raw.source_is_epjson = !idf;
            let objects = raw
                .objects
                .get_mut("OtherEquipment")
                .ok_or("blank direct objects")?;
            let fields = &mut objects[0].fields;
            fields.remove("design_level");
            fields.insert(
                "power_per_floor_area".into(),
                ProjectInputValue::Number(-9.0),
            );
            fields.insert("power_per_person".into(), ProjectInputValue::Number(9.0));
            if idf {
                for object in objects {
                    object.idf_max_fields = Some(13);
                }
            }
            let clock = crate::weather::day::WeatherGlobalState {
                warmup_flag: warmup,
                ..Default::default()
            };
            let (mut state, mut errors, mut console, mut eio) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            let result = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut errors),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            );
            if prior_error {
                assert!(matches!(
                    result,
                    Err(crate::RuntimeError::HeatBalanceInput(_))
                ));
            } else {
                result?;
            }
            let gains = &reports.selected_internal_heat_gains;
            assert_eq!(gains.get_input_flag, prior_error);
            assert_eq!(gains.errors_found, prior_error);
            let rows = gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("blank direct rows")?;
            assert_eq!(rows[0].design_level.to_bits(), 0.0_f64.to_bits());
            assert_eq!(
                rows[0].nominal_min_design_level.to_bits(),
                0.0_f64.to_bits()
            );
            assert_eq!(
                rows[0].nominal_max_design_level.to_bits(),
                0.0_f64.to_bits()
            );
            assert_eq!(
                rows[0].fraction_radiant, 0.2,
                "fractions follow successful warning"
            );
            assert_eq!(rows[1].design_level, if prior_error { 0.0 } else { -3.0 });
            assert_eq!(state.tracking.total_warning_errors, 1);
            assert_eq!(
                state.tracking.total_warning_errors_during_warmup,
                i32::from(warmup && !kickoff && !sizing)
            );
            assert_eq!(
                state.tracking.total_warning_errors_during_sizing,
                i32::from(sizing)
            );
            assert_eq!(state.tracking.total_severe_errors, 0);
            assert!(!state.warmup_flag && !state.kickoff_simulation && !state.doing_sizing);
            let text = String::from_utf8(errors)?;
            let title = if idf { "Design Level" } else { "design_level" };
            assert!(text.contains(&format!(
                "specifies {title}, but that field is blank.  0 OtherEquipment will result."
            )));
            assert_eq!(
                String::from_utf8(eio)?.contains("OtherEquipment Internal Gains Nominal"),
                !prior_error
            );
        }
        {
            // Failure in the newly reached direct blank warning preserves the
            // same earlier schedule/fuel writes but no sizing/fraction prefix.
            let mut reports = failed_gains.clone();
            reports
                .heat_balance_input
                .as_mut()
                .ok_or("blank warning raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("blank warning objects")?[0]
                .fields
                .remove("design_level");
            let clock = crate::weather::day::WeatherGlobalState {
                warmup_flag: true,
                ..Default::default()
            };
            let (mut state, mut console, mut eio, mut error_stream) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                OrdinaryFailingEio,
            );
            let result = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut error_stream),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            );
            assert!(matches!(result, Err(crate::RuntimeError::WarmupOutput(_))));
            let gains = &reports.selected_internal_heat_gains;
            assert!(gains.get_input_flag && !gains.errors_found);
            let row = &gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("blank warning row")?[0];
            assert!(row.schedule.is_some());
            assert_eq!(row.design_level.to_bits(), 0.0_f64.to_bits());
            assert_eq!(row.fraction_radiant, 0.0);
            assert_eq!(state.tracking.total_warning_errors_during_warmup, 1);
            assert!(eio.is_empty());
        }
        // The actual getter already owns the exact IHG fraction clamp. Exercise
        // both adjacent binary64 sides and its retained severe/fatal prefix.
        for (lost, valid) in [
            (0.3995_f64, true),
            (0.4005, true),
            (f64::from_bits(0.401_f64.to_bits() - 1), true),
            (0.401, true),
            (f64::from_bits(0.401_f64.to_bits() + 1), false),
            (0.4011, false),
        ] {
            let mut reports = failed_gains.clone();
            let fields = &mut reports
                .heat_balance_input
                .as_mut()
                .ok_or("clamp raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("clamp objects")?[0]
                .fields;
            for (name, value) in [
                ("fraction_latent", 0.0),
                ("fraction_radiant", 0.6),
                ("fraction_lost", lost),
            ] {
                fields.insert(name.into(), ProjectInputValue::Number(value));
            }
            let clock = crate::weather::day::WeatherGlobalState::default();
            let (mut state, mut errors, mut console, mut eio) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            let result = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut errors),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            );
            if valid {
                result?;
            } else {
                assert!(matches!(
                    result,
                    Err(crate::RuntimeError::HeatBalanceInput(_))
                ));
            }
            let gains = &reports.selected_internal_heat_gains;
            let row = &gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("clamp row")?[0];
            let residual = 1.0 - (0.0 + 0.6 + lost);
            let expected = if residual.abs() <= 0.001 {
                0.0
            } else {
                residual
            };
            assert_eq!(row.fraction_convected.to_bits(), expected.to_bits());
            assert_eq!(row.fraction_lost.to_bits(), lost.to_bits());
            assert_eq!(gains.errors_found, !valid);
            assert_eq!(gains.get_input_flag, !valid);
            assert_eq!(state.tracking.total_warning_errors, 0);
            assert_eq!(state.tracking.total_severe_errors, i32::from(!valid));
            let text = String::from_utf8(errors)?;
            assert_eq!(text.contains("Sum of Fractions > 1.0"), !valid);
            assert_eq!(
                String::from_utf8(eio)?.contains("OtherEquipment Internal Gains Nominal"),
                valid
            );
        }
        // Author genuine JSON-kind projected operands at the same getter
        // boundary; the raw/typed/HB tests above its caller retain literal text.
        // No Native observations or report values supply these unit inputs.
        for (token, expected) in [
            ("-0", 0.0_f64),
            ("-0.0", -0.0),
            ("-0e0", -0.0),
            ("1e-9999", 0.0),
            ("-1e-9999", -0.0),
            ("18446744073709551615", -1.0),
        ] {
            let mut reports = failed_gains.clone();
            let projected = ep_model::classify_source_json_number(token)
                .map_err(|_| "authored valid JSON gain operand")?
                .as_f64();
            assert_eq!(projected.to_bits(), expected.to_bits());
            let fields = &mut reports
                .heat_balance_input
                .as_mut()
                .ok_or("kind raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("kind objects")?[0]
                .fields;
            for name in ["design_level", "power_per_floor_area", "power_per_person"] {
                fields.insert(name.into(), ProjectInputValue::Number(projected));
            }
            let zero = if projected < 0.0 { 0.0 } else { projected };
            for name in [
                "fraction_latent",
                "fraction_radiant",
                "fraction_lost",
                "carbon_dioxide_generation_rate",
            ] {
                fields.insert(name.into(), ProjectInputValue::Number(zero));
            }
            let clock = crate::weather::day::WeatherGlobalState::default();
            let (mut state, mut errors, mut console, mut eio) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut errors),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            )?;
            let gains = &reports.selected_internal_heat_gains;
            let rows = gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("kind gain rows")?;
            assert_eq!(
                rows[0].design_level.to_bits(),
                expected.to_bits(),
                "{token}"
            );
            assert_eq!(
                rows[0].nominal_min_design_level.to_bits(),
                expected.to_bits()
            );
            assert_eq!(
                rows[0].nominal_max_design_level.to_bits(),
                expected.to_bits()
            );
            for value in [
                rows[0].fraction_latent,
                rows[0].fraction_radiant,
                rows[0].fraction_lost,
                rows[0].co2_rate_factor,
            ] {
                assert_eq!(value.to_bits(), zero.to_bits());
            }
            assert_eq!(rows[0].fraction_convected, 1.0);
            assert!(!gains.errors_found && !gains.get_input_flag);
            assert_eq!(state.tracking.total_warning_errors, 0);
            assert_eq!(state.tracking.total_severe_errors, 0);
            assert!(errors.is_empty());
            assert!(String::from_utf8(eio)?.contains("OtherEquipment Internal Gains Nominal"));
        }
        // Genuine enum/resource registration and the existing timestep prefix
        // all retain the same real gain rows. The gain-only IP fixture lacks
        // refrigeration schema definitions, an explicit first child boundary.
        for (fuel_index, fuel_name) in [
            "Electricity",
            "NaturalGas",
            "Gasoline",
            "Diesel",
            "Coal",
            "Propane",
            "FuelOilNo1",
            "FuelOilNo2",
            "OtherFuel1",
            "OtherFuel2",
            "DistrictCooling",
            "DistrictHeatingWater",
            "DistrictHeatingSteam",
        ]
        .into_iter()
        .enumerate()
        {
            use crate::heat_balance::source_output_registration::SourceRealOutputOwners;
            let mut reports = failed_gains.clone();
            for (i, object) in reports
                .heat_balance_input
                .as_mut()
                .ok_or("fuel raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("fuel objects")?
                .iter_mut()
                .enumerate()
            {
                object.fields.insert(
                    "fuel_type".into(),
                    ProjectInputValue::String(fuel_name.into()),
                );
                object.fields.insert(
                    "design_level".into(),
                    ProjectInputValue::Number(if i == 0 { 7.0 } else { 3.0 }),
                );
            }
            let clock = crate::weather::day::WeatherGlobalState {
                time_step_zone_sec: 900.0,
                ..Default::default()
            };
            let (mut state, mut console, mut eio, mut error) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut error),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            )?;
            let gains = &reports.selected_internal_heat_gains;
            assert!(!gains.get_input_flag && !gains.errors_found);
            assert!(
                gains
                    .allocations
                    .other_equipment
                    .as_ref()
                    .ok_or("fuel rows")?
                    .iter()
                    .all(|r| r.fuel as usize == fuel_index)
            );
            let zones = reports
                .selected_zone_input
                .as_ref()
                .ok_or("fuel associations")?;
            assert_eq!(
                zones.zones.as_ref().ok_or("fuel Zone")?[0].other_equipment_fuels,
                [fuel_index as i32]
            );
            assert_eq!(
                zones.spaces.as_ref().ok_or("fuel Space")?[0].other_equipment_fuels,
                [fuel_index as i32]
            );
            let meter = reports
                .output_registration
                .meters
                .meters
                .iter()
                .find(|m| m.name == format!("{fuel_name}:Facility"))
                .ok_or("real fuel facility meter")?;
            assert_eq!(meter.resource as usize, fuel_index);
            assert_eq!(
                meter.source_variable_numbers.len(),
                2,
                "both real equipment owners share one meter"
            );
            assert_eq!(
                meter.ip_units.map(|v| v as i32),
                Some(match fuel_index {
                    0 => 1,
                    1 => 2,
                    10 => 3,
                    _ => 0,
                })
            );
            for index in &meter.source_variable_numbers {
                let target = reports.output_registration.out_vars[*index]
                    .which
                    .ok_or("real fuel output binding")?;
                assert_eq!(
                    *gains.real_output(target)?,
                    0.0,
                    "constructor Consumption is retained until reporting"
                );
            }
            for prefix in [
                "Other Equipment",
                "Zone Other Equipment",
                "Space Other Equipment",
            ] {
                for suffix in ["Rate", "Energy"] {
                    assert!(
                        reports
                            .output_registration
                            .dictionary_variables
                            .iter()
                            .any(|v| v.name == format!("{prefix} {fuel_name} {suffix}"))
                    );
                }
            }
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            let globals = reports
                .selected_heat_balance_globals
                .as_ref()
                .ok_or("fuel globals")?;
            // GetZoneData allocates these real report rows before gain resets.
            reports
                .selected_zone_input
                .as_mut()
                .ok_or("fuel predefined Zone reports")?
                .zone_predefined = Some(vec![
                crate::heat_balance::selected_heat_balance_input::zone::ZonePredefined::default();
                globals.num_of_zones as usize
            ]);
            let mut air =
                crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance::default();
            air.allocate_source_arrays(globals)?;
            let retained_air = air.clone();
            let mut modules =
                crate::heat_balance::selected_gain_modules::SourceGainModules::default();
            let mut facility =
                crate::heat_balance::facility_electric_service::FacilityElectricService::new();
            let mut provider =
                crate::heat_balance::selected_gain_modules::SourceModuleGainProvider {
                    modules: &mut modules,
                    input: reports.selected_input_processor.as_mut().ok_or("fuel IP")?,
                    facility: Some(&mut facility),
                };
            let mut step = crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainStepOwners {
                clock: &clock, globals, control: &reports.heat_balance_input_state,
                zones: reports.selected_zone_input.as_mut().ok_or("live fuel zones")?,
                schedules: reports.selected_schedule_manager.as_ref().ok_or("live fuel schedules")?,
                backend: &mut backend, modules: &mut provider, latent: Some(&mut air), room_air: None, radiation: None,
                zone_generic_gain: crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceZoneGenericGainAllocation::Unavailable,
            };
            let reached = reports
                .selected_internal_heat_gains
                .manage_timestep(&mut step);
            assert!(
                matches!(&reached, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("same refrigeration selected official schema definition"))),
                "actual fuel timestep prefix: {reached:?}"
            );
            let gains = &reports.selected_internal_heat_gains;
            let rows = gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("updated fuel rows")?;
            assert_eq!([rows[0].rates[0], rows[1].rates[0]], [7.0, 3.0]);
            let space_report = &gains
                .allocations
                .space_report
                .as_ref()
                .ok_or("fuel rate report")?[0];
            assert_eq!(space_report.other_power[fuel_index], 10.0);
            assert!(
                space_report
                    .other_power
                    .iter()
                    .enumerate()
                    .all(|(i, q)| i == fuel_index || *q == 0.0)
            );
            assert_eq!(
                rows[0].energies[0], 0.0,
                "the failed module child precedes ReportInternalHeatGains"
            );
            assert_eq!(
                air, retained_air,
                "latent sinks follow the failed module child"
            );
            assert!(
                gains
                    .allocations
                    .space_devices
                    .as_ref()
                    .ok_or("fuel device cache")?
                    .iter()
                    .all(|space| space
                        .devices
                        .as_ref()
                        .is_some_and(|rows| rows.iter().all(|row| row.cached_rates == [0.0; 7]))),
                "cache writes also remain unreached"
            );
            assert_eq!(backend.state.tracking.total_warning_errors, 0);
            assert_eq!(backend.state.tracking.total_severe_errors, 0);
            if fuel_index == 0 {
                // The parent stopped at Refrigeration above. These
                // are explicit subsequent child-owner tests, not a successful
                // ManageInternalHeatGains or an invented module continuation.
                let expected_latent = 7.0_f64 * 0.1 + 3.0_f64 * 0.1;
                assert_eq!(rows[0].rates[3].to_bits(), (7.0_f64 * 0.1).to_bits());
                assert_eq!(rows[1].rates[3].to_bits(), (3.0_f64 * 0.1).to_bits());
                let mut child = crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainStepOwners {
                    clock: &clock, globals, control: &reports.heat_balance_input_state,
                    zones: reports.selected_zone_input.as_mut().ok_or("latent Zone owner")?,
                    schedules: reports.selected_schedule_manager.as_ref().ok_or("latent schedule owner")?,
                    backend: &mut backend, modules: &mut provider, latent: None, room_air: None, radiation: None,
                    zone_generic_gain: crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceZoneGenericGainAllocation::Unavailable,
                };
                let missing = reports
                    .selected_internal_heat_gains
                    .update_source_internal_gain_values(&mut child, true, true);
                assert!(matches!(missing, Err(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Unavailable("same actual Space AirHB latentGain sink"))));
                assert_eq!(
                    air, retained_air,
                    "missing sink follows actual cache writes"
                );
                let devices = reports
                    .selected_internal_heat_gains
                    .allocations
                    .space_devices
                    .as_ref()
                    .ok_or("latent caches")?[0]
                    .devices
                    .as_ref()
                    .ok_or("latent device slots")?;
                assert_eq!(
                    devices[0].cached_rates[3].to_bits(),
                    (7.0_f64 * 0.1).to_bits()
                );
                assert_eq!(
                    devices[1].cached_rates[3].to_bits(),
                    (3.0_f64 * 0.1).to_bits()
                );
                assert_eq!(
                    devices[0].cached_rates[2], 0.0,
                    "real HVAC radiation suppression"
                );
                let mut child = crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainStepOwners {
                    clock: &clock, globals, control: &reports.heat_balance_input_state,
                    zones: reports.selected_zone_input.as_mut().ok_or("same latent Zone owner")?,
                    schedules: reports.selected_schedule_manager.as_ref().ok_or("same latent schedule owner")?,
                    backend: &mut backend, modules: &mut provider, latent: Some(&mut air), room_air: None, radiation: None,
                    zone_generic_gain: crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceZoneGenericGainAllocation::Unavailable,
                };
                reports
                    .selected_internal_heat_gains
                    .update_source_internal_gain_values(&mut child, true, true)?;
                assert_eq!(
                    air.zones.as_ref().ok_or("Zone latent sink")?[0]
                        .air
                        .latent_gain
                        .to_bits(),
                    expected_latent.to_bits()
                );
                assert_eq!(
                    air.spaces.as_ref().ok_or("Space latent sink")?[0]
                        .air
                        .latent_gain
                        .to_bits(),
                    expected_latent.to_bits()
                );
                let air_address =
                    &air.zones.as_ref().ok_or("same AirHB allocation")?[0].air as *const _ as usize;
                // Real allocators and RoomAir children supply the ordinary
                // corrector's owners. The surface arrays retain their genuine
                // zero-convection allocation state for this authored child case.
                use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::{
                    SourceZoneAirOwners, SourceZoneAirPredictorCorrector,
                };
                let mut controller = SourceZoneAirPredictorCorrector::default();
                let controls = &reports.heat_balance_space_controls;
                controller
                    .demand
                    .allocate_source_demands(globals, controls)?;
                controller
                    .reports
                    .allocate_source_reports(globals, controls)?;
                let mut fan = crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays::default();
                fan.allocate_source_fan_zone_arrays(globals)?;
                let mut shortcut = crate::heat_balance::selected_input_processor::SourceInputProcessorShortCut::default();
                let mut child_eio = Vec::new();
                controller.subsystems.room_air.get_source_model_parameters(
                    reports
                        .selected_zone_input
                        .as_ref()
                        .ok_or("RoomAir real zones")?,
                    globals,
                    controls,
                    reports
                        .selected_input_processor
                        .as_ref()
                        .ok_or("RoomAir actual IP")?,
                    &mut shortcut,
                    &mut child_eio,
                )?;
                assert!(controller.subsystems.room_air.get_air_model_data);
                let geometry = &reports.surface_geometry_input;
                let ids = geometry
                    .final_surfaces
                    .as_ref()
                    .ok_or("actual opaque surfaces")?
                    .iter()
                    .map(|row| row.typed_id.ok_or("actual opaque Surface identity"))
                    .collect::<Result<Vec<_>, _>>()?;
                let mut history = crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState::new_source_unallocated(ids);
                let mut surfaces = crate::heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays::default();
                surfaces.allocate(
                    history.surface_ids.len(),
                    reports
                        .heat_balance_input_state
                        .any_internal_heat_source_in_input,
                    cache
                        .public_initialization()
                        .ok_or("actual CTF aggregate")?
                        .aggregate_after_returned_calls
                        .simple_ctf_only,
                    globals.any_energy_management_system_in_model,
                    &mut history,
                )?;
                let mut hvac =
                    crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals {
                        time_step_sys: 0.25,
                        time_step_sys_seconds: 900.0,
                        ..Default::default()
                    };
                let weather = crate::weather::day::WeatherEnvironmentState {
                    current_weather: crate::weather::day::CurrentWeatherState {
                        out_baro_press: 101325.0,
                        out_hum_rat: 0.006,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let availability = crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability::default();
                let mut psychrometrics = crate::psychrometrics::SourcePsychrometricAccess;
                // Uncontrolled correctAirTemp/HumRat never reads equipment
                // configuration. Retain its genuine constructor, without
                // pretending that this gain-only IP completed GetZoneEquipment.
                let input = reports
                    .selected_input_processor
                    .as_ref()
                    .ok_or("equipment inventory")?;
                let inventory = crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::FAMILIES
                    .into_iter().filter_map(|family| match input.family(family) {
                        Some(crate::heat_balance::selected_input_processor::SourceInputFamily::Objects(rows)) => Some((family, rows.len())),
                        _ => None,
                    });
                let equipment_clock = crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::from_source_family_inventory(
                    inventory, model.typed.timestep.number_of_timesteps_per_hour as usize,
                )?;
                let mut latent_equipment =
                    crate::heat_balance::ordinary_equipment::OrdinaryEquipment::new_for_source(
                        &model.typed,
                        equipment_clock,
                    )?;
                let mut owners = SourceZoneAirOwners {
                    zones: reports
                        .selected_zone_input
                        .as_mut()
                        .ok_or("corrector same Zones")?,
                    equipment: &mut latent_equipment,
                    geometry,
                    surface_arrays: &surfaces,
                    fan: &mut fan,
                    gains: &reports.selected_internal_heat_gains,
                    control: &mut reports.heat_balance_input_state,
                    space_controls: controls,
                    globals,
                    clock: &clock,
                    reporting_diagnostic_flags: Some(crate::heat_balance::error_backend::SourceDiagnosticFlags {
                        kickoff: &globals.kickoff_simulation,
                        developer: &reports.selected_project_data.as_ref().ok_or("same corrector project")?.system.developer_flag,
                        warmup: &clock.warmup_flag,
                        doing_sizing: &globals.doing_sizing,
                    }),
                    environment: &weather,
                    hvac: &mut hvac,
                    input: reports
                        .selected_input_processor
                        .as_mut()
                        .ok_or("corrector same IP")?,
                    shortcut: &mut shortcut,
                    source_is_epjson: &reports
                        .heat_balance_input
                        .as_ref()
                        .ok_or("actual input format")?
                        .source_is_epjson,
                    faults: &reports.source_faults_manager,
                    schedules: reports
                        .selected_schedule_manager
                        .as_mut()
                        .ok_or("corrector real schedules")?,
                    availability: &availability,
                    psychrometrics: &mut psychrometrics,
                    backend: &mut backend,
                    recurring_errors: &mut reports.source_recurring_errors,
                };
                let zone = &owners.zones.zones.as_ref().ok_or("unconditioned Zone")?[0];
                assert!(!zone.is_controlled && zone.system_zone_node_number == 0);
                let volume = zone.volume;
                let moisture_capacity = zone.moisture_capacity_multiplier;
                assert!(volume > 0.0 && moisture_capacity > 0.0);
                crate::psychrometrics::with_fresh_psychrometric_state(
                    || -> Result<(), Box<dyn std::error::Error>> {
                        crate::psychrometrics::initialize_source_psychrometric_caches();
                        for (previous, branch) in [
                            ([0.008_f64; 4], 0),
                            ([0.001, 0.010, 0.001, 0.007], -1),
                            ([0.040; 4], 1),
                        ] {
                            let current =
                                &mut air.zones.as_mut().ok_or("live humidity input")?[0].air;
                            current.mat = 23.0;
                            current.zt = 23.0;
                            current.air_hum_rat = 0.008;
                            current.xmat = [23.0; 4];
                            current.w_prev_zone_ts = previous;
                            let earlier_system_history = current.dsw_prev_zone_ts;
                            for flow in [
                                current.oamfl,
                                current.vamfl,
                                current.ctmfl,
                                current.eamfl,
                                current.eamf_lx_hum_rat,
                                current.sum_hm_a_ra,
                                current.sum_hm_a_ra_w,
                                current.mixing_mass_flow_zone,
                                current.mixing_mass_flow_x_hum_rat,
                                current.m_dot_oa,
                            ] {
                                assert_eq!(flow, 0.0, "genuine no-flow constructor boundary");
                            }
                            // The same ordinary production corrector invokes its real
                            // temperature, humidity, RH, Space handoff and report children.
                            air.correct_source_zone_air_temperatures(
                                true,
                                &mut controller,
                                &mut owners,
                            )?;
                            let current = &air.zones.as_ref().ok_or("live humidity result")?[0].air;
                            let rho = crate::psychrometrics::energyplus_psy_rho_air_fn_pb_tdb_w(
                                101325.0, current.zt, 0.008,
                            );
                            let hg = crate::psychrometrics::energyplus_psy_hg_air_fn_w_tdb(
                                0.008, current.zt,
                            );
                            let capacity = rho * volume * moisture_capacity / 900.0;
                            // Independent no-flow form of the Native third-order
                            // moisture balance, using the actual latent sink operand.
                            let unbounded = (3.0 * previous[0] - 1.5 * previous[1]
                                + previous[2] / 3.0
                                + expected_latent / hg / capacity)
                                * (6.0 / 11.0);
                            let saturated = owners
                                .psychrometrics
                                .evaluate(
                                    crate::psychrometrics::EnergyPlusPsychrometricFunction::WFromRh,
                                    &[current.zt, 1.0, 101325.0],
                                )
                                .map_err(|_| "actual saturation child")?
                                .result;
                            let expected = unbounded.max(0.0).min(saturated);
                            assert!((current.air_hum_rat - expected).abs() < 1.0e-14);
                            match branch {
                                -1 => assert_eq!(current.air_hum_rat.to_bits(), 0.0_f64.to_bits()),
                                1 => assert_eq!(current.air_hum_rat.to_bits(), saturated.to_bits()),
                                _ => assert!(
                                    current.air_hum_rat > 0.008 && current.air_hum_rat < saturated
                                ),
                            }
                            assert_eq!(current.latent_gain.to_bits(), expected_latent.to_bits());
                            assert_eq!(
                                &air.zones.as_ref().ok_or("retained AirHB")?[0].air as *const _
                                    as usize,
                                air_address
                            );
                            let space =
                                &air.spaces.as_ref().ok_or("same Space humidity handoff")?[0].air;
                            assert_eq!(space.air_hum_rat.to_bits(), current.air_hum_rat.to_bits());
                            assert_eq!(space.air_rel_hum.to_bits(), current.air_rel_hum.to_bits());
                            assert!((0.0..=100.0).contains(&current.air_rel_hum));
                            let actual_w = current.air_hum_rat;
                            air.push_source_system_timestep_histories(&controller, &owners)?;
                            assert_eq!(
                                air.zones.as_ref().ok_or("same humidity history")?[0]
                                    .air
                                    .dsw_prev_zone_ts,
                                [
                                    actual_w,
                                    earlier_system_history[0],
                                    earlier_system_history[1],
                                    earlier_system_history[2]
                                ]
                            );
                        }
                        Ok(())
                    },
                )?;
                assert!(
                    !controller.subsystems.room_air.get_air_model_data,
                    "actual child alone completed its own getter"
                );
                assert_eq!(backend.state.tracking.total_warning_errors, 0);
                assert_eq!(backend.state.tracking.total_severe_errors, 0);
                // The failed parent and the three humidity/clamp cases above
                // remain intact. This explicit child enables diagnostics through
                // the full real project getter, then reuses the same live gains,
                // Zone/Space AirHB, controller and persistent report owners.
                use crate::heat_balance::selected_input_processor::{
                    SelectedInputProcessor, SourceInputFamily, SourceObjectDefinition,
                    SourceObjectShape,
                };
                use crate::heat_balance::selected_project_data::{
                    ProjectDataOwners, ProjectDiagnosticsObject, ProjectInputObject,
                    SelectedProjectData, SelectedProjectDataInput,
                };
                let raw = SelectedProjectDataInput {
                    source_is_epjson: true,
                    objects: [
                        (
                            "Version",
                            "version_identifier",
                            ProjectInputValue::String("26.1".into()),
                        ),
                        (
                            "Timestep",
                            "number_of_timesteps_per_hour",
                            ProjectInputValue::Number(4.0),
                        ),
                    ]
                    .into_iter()
                    .map(|(family, field, value)| {
                        (
                            family.into(),
                            vec![ProjectInputObject {
                                object_name: family.into(),
                                fields: std::collections::BTreeMap::from([(field.into(), value)]),
                                idf_max_fields: None,
                            }],
                        )
                    })
                    .collect(),
                    diagnostics: vec![ProjectDiagnosticsObject {
                        object_name: "Air balance diagnostics".into(),
                        keys: vec![
                            Some("DisplayWeatherMissingDataWarnings".into()),
                            Some("DisplayZoneAirHeatBalanceOffBalance".into()),
                        ],
                    }],
                };
                let definitions = [
                    ("Version", 1, 0, 0),
                    ("Timestep", 0, 1, 0),
                    ("Output:Diagnostics", 0, 0, 1),
                ]
                .into_iter()
                .map(
                    |(family, alphas, numerics, extensions)| SourceObjectDefinition {
                        object_type: family.into(),
                        alpha_fields: alphas,
                        numeric_fields: numerics,
                        alpha_extensions: extensions,
                        numeric_extensions: 0,
                        extension: (extensions != 0).then(|| "diagnostics".into()),
                    },
                )
                .collect();
                let mut families: std::collections::BTreeMap<String, SourceInputFamily> = raw
                    .objects
                    .iter()
                    .map(|(family, objects)| {
                        (
                            family.clone(),
                            SourceInputFamily::Objects(
                                objects
                                    .iter()
                                    .map(|object| SourceObjectShape {
                                        object_name: object.object_name.clone(),
                                        field_sizes: object
                                            .fields
                                            .keys()
                                            .map(|field| (field.clone(), 1))
                                            .collect(),
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect();
                families.insert(
                    "Output:Diagnostics".into(),
                    SourceInputFamily::Objects(vec![SourceObjectShape {
                        object_name: raw.diagnostics[0].object_name.clone(),
                        field_sizes: std::collections::BTreeMap::from([(
                            "diagnostics".into(),
                            raw.diagnostics[0].keys.len(),
                        )]),
                    }]),
                );
                let mut project_input =
                    SelectedInputProcessor::from_admitted_input(definitions, families)?;
                let mut project_shortcut = crate::heat_balance::selected_input_processor::SourceInputProcessorShortCut::default();
                project_shortcut.allocate_from_input(&project_input)?;
                let mut project = SelectedProjectData::default();
                let mut project_weather = crate::weather::day::WeatherDayState {
                    environment: weather,
                    ..Default::default()
                };
                let mut current_module = String::new();
                let globals = reports
                    .selected_heat_balance_globals
                    .as_mut()
                    .ok_or("same diagnostic globals")?;
                assert!(!globals.display_zone_air_heat_balance_off_balance);
                assert!(
                    !project_weather
                        .environment
                        .display_weather_missing_data_warnings
                );
                assert!(
                    project_input.is_object_unused("Output:Diagnostics", "Air balance diagnostics")
                );
                project.get_project_data(
                    &raw,
                    &mut ProjectDataOwners {
                        input: &mut project_input,
                        shortcut: &mut project_shortcut,
                        globals,
                        weather: &mut project_weather,
                        hvac: &mut hvac,
                        heat_balance_current_module_object: &mut current_module,
                    },
                    &mut backend,
                    &mut child_eio,
                )?;
                assert!(globals.display_zone_air_heat_balance_off_balance);
                assert!(
                    project_weather
                        .environment
                        .display_weather_missing_data_warnings
                );
                assert!(
                    !project_input
                        .is_object_unused("Output:Diagnostics", "Air balance diagnostics")
                );
                assert_eq!(project_weather.global.time_step_zone_sec, 900.0);
                assert_eq!(backend.state.tracking.total_warning_errors, 0);
                assert_eq!(reports.source_recurring_errors.number, 0);
                let mut owners = SourceZoneAirOwners {
                    zones: reports
                        .selected_zone_input
                        .as_mut()
                        .ok_or("same diagnostic Zones")?,
                    equipment: &mut latent_equipment,
                    geometry,
                    surface_arrays: &surfaces,
                    fan: &mut fan,
                    gains: &reports.selected_internal_heat_gains,
                    control: &mut reports.heat_balance_input_state,
                    space_controls: controls,
                    globals,
                    clock: &project_weather.global,
                    reporting_diagnostic_flags: Some(crate::heat_balance::error_backend::SourceDiagnosticFlags {
                        kickoff: &globals.kickoff_simulation,
                        developer: &project.system.developer_flag,
                        warmup: &project_weather.global.warmup_flag,
                        doing_sizing: &globals.doing_sizing,
                    }),
                    environment: &project_weather.environment,
                    hvac: &mut hvac,
                    input: reports
                        .selected_input_processor
                        .as_mut()
                        .ok_or("same corrector IP")?,
                    shortcut: &mut shortcut,
                    source_is_epjson: &reports
                        .heat_balance_input
                        .as_ref()
                        .ok_or("same raw format")?
                        .source_is_epjson,
                    faults: &reports.source_faults_manager,
                    schedules: reports
                        .selected_schedule_manager
                        .as_mut()
                        .ok_or("same schedules")?,
                    availability: &availability,
                    psychrometrics: &mut psychrometrics,
                    backend: &mut backend,
                    recurring_errors: &mut reports.source_recurring_errors,
                };
                assert_eq!(
                    owners.zones.zones.as_ref().ok_or("diagnostic Zone")?[0]
                        .air_heat_balance_imbalance_error_index,
                    0
                );
                assert!(!owners.hvac.turn_fans_on);
                let mut excesses = Vec::new();
                let mut same_index = None;
                crate::psychrometrics::with_fresh_psychrometric_state(
                    || -> Result<(), Box<dyn std::error::Error>> {
                        crate::psychrometrics::initialize_source_psychrometric_caches();
                        for occurrence in 1..=2 {
                            // Authored current/history operands go through the real
                            // ThirdOrder solve, humidity and component-report caller.
                            let current =
                                &mut air.zones.as_mut().ok_or("diagnostic AirHB input")?[0].air;
                            current.mat = 23.0;
                            current.zt = 23.0;
                            current.xmat = [23.0; 4];
                            current.air_hum_rat = 0.008;
                            current.w_prev_zone_ts = [0.008; 4];
                            air.correct_source_zone_air_temperatures(
                                true,
                                &mut controller,
                                &mut owners,
                            )?;
                            let report = &controller
                                .reports
                                .zones
                                .as_ref()
                                .ok_or("real component result")?[0];
                            assert!(report.sum_int_gains > 0.0 && report.czd_tdt > 0.0);
                            for value in [
                                report.sum_had_tsurfs,
                                report.sum_m_cp_d_tzones,
                                report.sum_m_cp_dt_infil,
                                report.sum_m_cp_d_tsystem,
                                report.sum_non_air_system,
                            ] {
                                assert_eq!(
                                    value, 0.0,
                                    "real zero-convection/no-flow child operands"
                                );
                            }
                            let threshold = 0.2
                                * (report.sum_int_gains * report.sum_int_gains
                                    + report.czd_tdt * report.czd_tdt)
                                    .sqrt();
                            let excess = report.im_balance.abs() - threshold;
                            assert!(
                                excess > 0.0,
                                "actual solved report reaches Native strict warning predicate"
                            );
                            excesses.push(excess);
                            let index = owners
                                .zones
                                .zones
                                .as_ref()
                                .ok_or("persistent diagnostic Zone")?[0]
                                .air_heat_balance_imbalance_error_index;
                            assert_eq!(index, 1);
                            assert_eq!(*same_index.get_or_insert(index), index);
                            assert_eq!(owners.recurring_errors.number, 1);
                            let row = &owners
                                .recurring_errors
                                .rows
                                .as_ref()
                                .ok_or("actual recurring row")?[0];
                            assert_eq!(row.count, occurrence);
                            assert_eq!((row.warmup_count, row.sizing_count), (0, 0));
                            assert!(row.report_max && row.report_min && !row.report_sum);
                            assert_eq!(
                                (&*row.max_units, &*row.min_units, &*row.sum_units),
                                ("{W}", "{W}", "")
                            );
                            assert_eq!(
                                row.max_value.to_bits(),
                                excesses
                                    .iter()
                                    .copied()
                                    .fold(f64::NEG_INFINITY, f64::max)
                                    .to_bits()
                            );
                            assert_eq!(
                                row.min_value.to_bits(),
                                excesses
                                    .iter()
                                    .copied()
                                    .fold(f64::INFINITY, f64::min)
                                    .to_bits()
                            );
                            assert_eq!(
                                owners.backend.state.tracking.total_warning_errors,
                                occurrence
                            );
                            assert_eq!(owners.backend.state.tracking.total_severe_errors, 0);
                        }
                        Ok(())
                    },
                )?;
                let text = std::str::from_utf8(&error)?;
                assert_eq!(
                    text.matches("Zone Air Heat Balance is out of balance for zone named ")
                        .count(),
                    1
                );
                assert_eq!(
                    text.matches("Zone Air Heat Balance Deviation Rate is more than ")
                        .count(),
                    1
                );
                assert!(text.contains(" {W}\n"));
                assert_eq!(text.matches(" Occurrence info: Environment=").count(), 1);
                assert!(!text.contains("Night cycle fan operation"));
            }
        }
        // Real invalid/Water and negative-fuel diagnostics run before final
        // nominal output; the raw invalid fuel is retained in the fuel lists.
        for (fuel, code, level, expected) in [
            ("Water", 13, 7.0, "invalid fuel_type entered=WATER"),
            (
                "Unrecognized fuel",
                -1,
                7.0,
                "invalid fuel_type entered=UNRECOGNIZED FUEL",
            ),
            (
                "NaturalGas",
                1,
                -7.0,
                "design_level is not allowed to be negative",
            ),
        ] {
            let mut reports = failed_gains.clone();
            let object = &mut reports
                .heat_balance_input
                .as_mut()
                .ok_or("invalid fuel raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("invalid fuel objects")?[0];
            object
                .fields
                .insert("fuel_type".into(), ProjectInputValue::String(fuel.into()));
            object
                .fields
                .insert("design_level".into(), ProjectInputValue::Number(level));
            let clock = crate::weather::day::WeatherGlobalState {
                warmup_flag: true,
                ..Default::default()
            };
            let (mut state, mut console, mut eio, mut error) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            let outcome = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut error),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            );
            assert!(
                matches!(outcome, Err(crate::RuntimeError::HeatBalanceInput(e)) if matches!(*e, crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Fatal(_)))
            );
            assert!(
                reports.selected_internal_heat_gains.get_input_flag
                    && reports.selected_internal_heat_gains.errors_found
            );
            assert_eq!(
                reports
                    .selected_zone_input
                    .as_ref()
                    .ok_or("invalid fuel zones")?
                    .zones
                    .as_ref()
                    .ok_or("invalid Zone")?[0]
                    .other_equipment_fuels,
                [code]
            );
            assert!(
                eio.is_empty(),
                "fuel errors precede the real nominal EIO writer"
            );
            let text = String::from_utf8(error)?;
            assert!(text.contains(expected));
            if level < 0.0 {
                assert!(text.contains("... when a fuel type of NaturalGas is specified."));
            }
            assert_eq!(state.tracking.total_severe_errors_during_warmup, 1);
            assert!(
                !state.warmup_flag,
                "live flags do not rewrite backend constructor phase"
            );
        }
        {
            let mut reports = failed_gains.clone();
            reports
                .heat_balance_input
                .as_mut()
                .ok_or("fuel failure raw")?
                .objects
                .get_mut("OtherEquipment")
                .ok_or("fuel failure objects")?[0]
                .fields
                .insert(
                    "fuel_type".into(),
                    ProjectInputValue::String("Unrecognized fuel".into()),
                );
            let clock = crate::weather::day::WeatherGlobalState {
                warmup_flag: true,
                ..Default::default()
            };
            let (mut state, mut console, mut eio, mut error) = (
                ctf_ordinary_output_test_state(),
                Vec::new(),
                Vec::new(),
                OrdinaryFailingEio,
            );
            let outcome = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut error),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            }
            .manage_selected_internal_heat_gains_input_with_clock(
                &model.typed,
                15,
                &mut gain_host,
                Some(&clock),
            );
            assert!(matches!(outcome, Err(crate::RuntimeError::WarmupOutput(_))));
            let gains = &reports.selected_internal_heat_gains;
            assert!(
                gains.get_input_flag && !gains.errors_found,
                "failed severe precedes ErrorsFound"
            );
            let row = &gains
                .allocations
                .other_equipment
                .as_ref()
                .ok_or("fuel failure allocated rows")?[0];
            assert_eq!(
                row.fuel as i32, -1,
                "fuel lookup precedes the failed diagnostic"
            );
            assert_eq!(row.design_level, 0.0);
            assert!(
                reports
                    .selected_zone_input
                    .as_ref()
                    .ok_or("fuel failure zones")?
                    .zones
                    .as_ref()
                    .ok_or("fuel failure Zone")?[0]
                    .other_equipment_fuels
                    .is_empty(),
                "list insertion follows the returned severe"
            );
            assert_eq!(state.tracking.total_severe_errors_during_warmup, 1);
            assert!(eio.is_empty());
        }
        failed_gains
            .heat_balance_input
            .as_mut()
            .ok_or("failed raw gain owner")?
            .objects
            .get_mut("OtherEquipment")
            .ok_or("failed raw gain rows")?[0]
            .fields
            .insert(
                "zone_or_zonelist_or_space_or_spacelist_name".into(),
                ProjectInputValue::String("MISSING ZONE".into()),
            );
        let (mut gain_state, mut gain_console, mut gain_eio, mut gain_error) = (
            ctf_ordinary_output_test_state(),
            Vec::new(),
            Vec::new(),
            OrdinaryFailingEio,
        );
        let failure = CtfInitializationContext {
            reports: &mut failed_gains,
            eio: &mut gain_eio,
            backend: SourceErrorBackend {
                state: &mut gain_state,
                error_stream: Some(&mut gain_error),
                console: &mut gain_console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        }
        .manage_selected_internal_heat_gains_input(&model.typed, 15, &mut gain_host);
        assert!(
            matches!(failure,Err(crate::RuntimeError::WarmupOutput(error)) if matches!(*error,crate::error::WarmupOutputError::Backend(_)))
        );
        assert!(failed_gains.selected_internal_heat_gains.get_input_flag);
        assert!(
            !failed_gains.selected_internal_heat_gains.errors_found,
            "failed severe precedes ErrorsFound"
        );
        assert!(
            failed_gains
                .selected_internal_heat_gains
                .allocations
                .zone_report
                .is_some()
        );
        assert!(
            failed_gains
                .selected_internal_heat_gains
                .allocations
                .other_equipment
                .is_none()
        );
        // One malformed definition earns the real ErrorsFound write, then
        // reaches the parent fatal only after all preceding selected helpers.
        scheduled_definition.alpha_fields = 3;
        parent.reports.selected_input_processor =
            Some(SelectedInputProcessor::from_admitted_input(
                vec![scheduled_definition],
                Default::default(),
            )?);
        let failure = parent.continue_selected_heat_balance_input_before_solar();
        assert!(
            matches!(failure, Err(crate::RuntimeError::HeatBalanceInput(error))
            if matches!(*error, crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Fatal(
                crate::heat_balance::error_backend::SourceFatalStop::Fatal(_))))
        );
        assert!(parent.reports.heat_balance_input_errors_found);
        assert!(
            !parent.backend.state.tracking.abort_processing,
            "ShowFatalError throws before separate AbortEnergyPlus"
        );
        assert_eq!(
            parent
                .reports
                .input_processor_shortcut
                .current_module_object,
            "ComplexFenestrationProperty:SolarAbsorbedLayers"
        );
        assert!(!parent.reports.heat_balance_input_state.construction_usage[0].is_used);
        nominal.nominal_r_for_u = Some(vec![0.0]);
        nominal.construction_usage[0].type_is_window = true;
        let retained = nominal.clone();
        let (mut zero_state, mut zero_console, mut zero_failure) = (
            ctf_ordinary_output_test_state(),
            Vec::new(),
            OrdinaryFailingEio,
        );
        let failure = nominal.finish_selected_construction_nominal_u(
            ConstructionMaterialOwners {
                model: &layered,
                data: &materials,
                bounds: &construction_bounds,
            },
            &mut SourceErrorBackend {
                state: &mut zero_state,
                error_stream: Some(&mut zero_failure),
                console: &mut zero_console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
            &mut errors,
        );
        assert!(matches!(
            failure,
            Err(
                crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(_)
            )
        ));
        assert_eq!(
            nominal, retained,
            "zero-R failed severe precedes ErrorsFound and Type* writes"
        );
        assert!(!errors);
        assert_eq!(zero_state.tracking.total_severe_errors, 1);
        let reached_eio = String::from_utf8(eio)?;
        assert_eq!(
            reached_eio
                .matches("! <Surface Heat Transfer Algorithm>")
                .count(),
            1
        );
        assert!(reached_eio.contains(
            "Surface Heat Transfer Algorithm, CTF - ConductionTransferFunction,200,0.10,1000.0\n"
        ));
    }
    check_setup_process_native_reference_if_requested()?;
    Ok(())
}

#[test]
fn ordinary_loading_error_retains_layers_and_real_final_fatal_prefix()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{
        CtfInitializationContext, CtfInitializationReports,
    };
    use crate::heat_balance::error_backend::SourceErrorBackend;
    let mut model = ctf_ordinary_output_test_model();
    if let ep_model::MaterialDefinition::NoMass(material) = &mut model.materials[0].definition {
        material.thermal_resistance_m2_k_per_w = 1.0e-9;
    }
    let mut reports = CtfInitializationReports::default();
    let mut state = ctf_ordinary_output_test_state();
    let (mut error, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
    let result = {
        let mut context = CtfInitializationContext {
            reports: &mut reports,
            eio: &mut eio,
            backend: SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        };
        context.prepare_registry(&model)?;
        crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
            &model,&std::collections::BTreeMap::new(),&mut context)
    };
    assert!(matches!(
        result,
        Err(crate::RuntimeError::CtfInitialization(_))
    ));
    assert_eq!(state.tracking.total_severe_errors, 1);
    assert!(!state.tracking.abort_processing);
    assert_eq!(reports.progress.layer_report_declared_indices, [0]);
    assert!(reports.progress.final_fatal_invoked);
    assert!(!reports.progress.postlude_returned);
    let text = String::from_utf8(error)?;
    assert!(text.contains("Material=R1R Value below lowest allowed value"));
    assert!(
        text.contains("Program terminated for reasons listed (InitConductionTransferFunctions)")
    );
    Ok(())
}

#[test]
fn ordinary_missing_reached_name_preserves_earlier_thick_material_severe()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_initialization_context::{
        CtfInitializationContext, CtfInitializationReports,
    };
    use crate::heat_balance::error_backend::SourceErrorBackend;
    let mut model = ctf_ordinary_output_test_model();
    model.ctf_report_names.materials.clear();
    model.materials[0].definition =
        ep_model::MaterialDefinition::Regular(ep_model::RegularMaterial {
            roughness: MaterialSurfaceRoughness::Rough,
            thickness_m: 4.0,
            conductivity_w_per_m_k: 1.0,
            density_kg_per_m3: 1000.0,
            specific_heat_j_per_kg_k: 1000.0,
            thermal_resistance_m2_k_per_w: 4.0,
            surface: ep_model::OpaqueSurfaceProperties::default(),
        });
    let mut reports = CtfInitializationReports::default();
    let mut state = ctf_ordinary_output_test_state();
    let (mut error, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
    let result = {
        let mut context = CtfInitializationContext {
            reports: &mut reports,
            eio: &mut eio,
            backend: SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        };
        context.prepare_registry(&model)?;
        crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
            &model,&std::collections::BTreeMap::new(),&mut context)
    };
    assert!(matches!(
        result,
        Err(crate::RuntimeError::CtfInitialization(_))
    ));
    assert_eq!(state.tracking.total_severe_errors, 1);
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
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn ordinary_eio_failure_remains_discoverable_through_runtime_source_chain()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf_init_postlude::CtfInitPostludeError;
    use crate::heat_balance::ctf_initialization_context::{
        CtfInitializationContext, CtfInitializationError, CtfInitializationReports,
    };
    use crate::heat_balance::ctf_reports::CtfReportError;
    use crate::heat_balance::error_backend::SourceErrorBackend;
    use std::error::Error;
    let mut model = ctf_ordinary_output_test_model();
    model
        .report_inputs
        .objects
        .push(ep_model::ReportInputObject {
            object_name: "authored direct report request".into(),
            object_type: ep_model::ReportInputObjectType::Constructions,
            alphas: ["Constructions".into(), String::new(), String::new()],
            alpha_blank: [false, true, true],
            default_applied: [false; 3],
            num_alphas: 1,
            is_epjson: true,
        });
    let mut reports = CtfInitializationReports::default();
    let mut state = ctf_ordinary_output_test_state();
    let (mut error, mut console) = (Vec::new(), Vec::new());
    let mut eio = OrdinaryFailingEio;
    let mut context = CtfInitializationContext {
        reports: &mut reports,
        eio: &mut eio,
        backend: SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut error),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        },
    };
    context.prepare_registry(&model)?;
    let result = crate::heat_balance::surface_manager::ConstructionThermalDataCache::build_with_initialization_context(
        &model, &std::collections::BTreeMap::new(), &mut context);
    let outer = match result {
        Err(error) => error,
        Ok(_) => return Err("EIO write unexpectedly succeeded".into()),
    };
    let stored = match &outer {
        crate::RuntimeError::CtfInitialization(error) => match error.as_ref() {
            CtfInitializationError::Postlude(CtfInitPostludeError::Report(CtfReportError::Io(
                error,
            ))) => error,
            _ => return Err("wrong initialization failure".into()),
        },
        _ => return Err("wrong runtime failure".into()),
    };
    let discovered = outer
        .source()
        .and_then(Error::source)
        .and_then(Error::source)
        .and_then(Error::source)
        .and_then(|error| error.downcast_ref::<std::io::Error>())
        .ok_or("actual EIO cause missing from RuntimeError source chain")?;
    assert!(std::ptr::eq(stored, discovered));
    assert_eq!(discovered.raw_os_error(), Some(5));
    assert_eq!(reports.progress.construction_scan_result, Some(true));
    assert_eq!(reports.progress.layer_report_declared_indices, [0]);
    assert!(!reports.progress.report_headers_written && !reports.progress.final_fatal_invoked);
    assert!(!reports.progress.postlude_returned && error.is_empty());
    Ok(())
}
