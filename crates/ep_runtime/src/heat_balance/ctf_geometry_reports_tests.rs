use super::*;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationReports,
};
use crate::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking,
};
use ep_model::{
    AutoOrNumber, Construction, ConstructionId, ConstructionKind, Material, MaterialId,
    MaterialSurfaceRoughness, NoMassMaterial, NormalizedName, OpaqueSurfaceProperties, Point3,
    Space, SpaceId, SpaceOrigin, SpaceTypeId, SunExposure, SurfaceId, WindExposure, Zone, ZoneId,
};

fn model() -> TypedModel {
    let mut model = TypedModel::default();
    model.materials.push(Material {
        id: MaterialId(5),
        name: NormalizedName::new("M5"),
        definition: MaterialDefinition::NoMass(NoMassMaterial {
            roughness: MaterialSurfaceRoughness::MediumRough,
            thermal_resistance_m2_k_per_w: 2.0,
            surface: OpaqueSurfaceProperties::default(),
        }),
    });
    model.constructions.push(Construction {
        id: ConstructionId(30),
        name: NormalizedName::new("C30"),
        kind: ConstructionKind::Opaque,
        outside_layer: Some(MaterialId(5)),
        layers: vec![MaterialId(5)],
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    });
    model.zones.push(Zone {
        id: ZoneId(9),
        name: NormalizedName::new("Z9"),
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
        ceiling_height: AutoOrNumber::Value(1.0),
        volume: AutoOrNumber::Value(1.0),
        floor_area: AutoOrNumber::Value(1.0),
        inside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
            ep_model::InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
            ep_model::OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: false,
        linked_outdoor_air_node: None,
        spaces: vec![SpaceId(3)],
    });
    model.spaces.push(Space {
        id: SpaceId(3),
        name: NormalizedName::new("S3"),
        zone: ZoneId(9),
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
        floor_area: AutoOrNumber::AutoCalculate,
        space_type: NormalizedName::new("GENERAL"),
        space_type_id: SpaceTypeId(0),
        tags: Vec::new(),
        origin: SpaceOrigin::Authored,
    });
    model.surfaces.push(Surface {
        id: SurfaceId(500),
        name: NormalizedName::new("W500"),
        surface_type: SurfaceType::Wall,
        construction: ConstructionId(30),
        zone: ZoneId(9),
        space: SpaceId(3),
        outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::NoSun,
        wind_exposure: WindExposure::NoWind,
        view_factor_to_ground: AutoOrNumber::Value(0.0),
        vertices: Vec::new(),
        computed_geometry: None,
    });
    model
}

fn registry() -> Result<PredefinedRegistry, CtfReportUnavailable> {
    let mut owner = PredefinedRegistry::default();
    owner.set_predefined_tables(false, false)?;
    Ok(owner)
}

#[test]
fn actual_space_class_order_and_existing_full_table_are_retained()
-> Result<(), CtfReportUnavailable> {
    let mut model = model();
    let mut second_space = model.spaces[0].clone();
    second_space.id = SpaceId(4);
    model.spaces.push(second_space);
    model.zones[0].spaces.push(SpaceId(4));
    let mut floor = model.surfaces[0].clone();
    floor.id = SurfaceId(501);
    floor.name = NormalizedName::new("F501");
    floor.surface_type = SurfaceType::Floor;
    let mut second_wall = model.surfaces[0].clone();
    second_wall.id = SurfaceId(502);
    second_wall.name = NormalizedName::new("W502");
    second_wall.space = SpaceId(4);
    // Input Vec order is deliberately different from native registration/report order.
    model.surfaces.insert(0, floor);
    model.surfaces.insert(0, second_wall);
    let mut table = PredefinedStringTable::default();
    table.string_entry(91, "retained", "prefix")?;
    let rows = table
        .entries
        .as_mut()
        .ok_or(CtfReportUnavailable::TableOwner)?;
    rows[1].sub_table_index = 55;
    rows[1].unique_object_name = 56;
    rows[99].sub_table_index = 123;
    rows[99].original_real_entry = -0.0;
    let tail = rows[99].clone();
    let prefix = rows[0].clone();
    report_ordinary_surface_nominal_u(&model, &registry()?, &mut table)?;
    let rows = table
        .entries
        .as_ref()
        .ok_or(CtfReportUnavailable::TableOwner)?;
    assert_eq!(
        (table.num_table_entry, table.size_table_entry, rows.len()),
        (4, 100, 100)
    );
    assert_eq!(rows[0], prefix);
    assert_eq!(
        (rows[1].sub_table_index, rows[1].unique_object_name),
        (55, 56)
    );
    assert_eq!(
        (
            &*rows[1].object_name,
            &*rows[2].object_name,
            &*rows[3].object_name
        ),
        ("W500", "F501", "W502")
    );
    assert_eq!(rows[99], tail);
    assert_eq!(rows[99].original_real_entry.to_bits(), (-0.0_f64).to_bits());
    Ok(())
}

#[test]
fn source_class_and_adiabatic_films_use_real_columns() -> Result<(), CtfReportUnavailable> {
    let mut model = model();
    let mut floor = model.surfaces[0].clone();
    floor.surface_type = SurfaceType::Floor;
    let mut roof = model.surfaces[0].clone();
    roof.surface_type = SurfaceType::Roof;
    let mut ceiling = roof.clone();
    ceiling.surface_type = SurfaceType::Ceiling;
    let wall_u = film_u(&model, &model.surfaces[0])?.1;
    assert!(film_u(&model, &roof)?.1 > wall_u);
    assert!(wall_u > film_u(&model, &floor)?.1);
    assert_eq!(
        film_u(&model, &ceiling)?.1.to_bits(),
        film_u(&model, &roof)?.1.to_bits()
    );
    model.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
    let interior_u = film_u(&model, &model.surfaces[0])?.1;
    assert!(interior_u < wall_u);
    let registry = registry()?;
    let mut table = PredefinedStringTable::default();
    report_ordinary_surface_nominal_u(&model, &registry, &mut table)?;
    let rows = table
        .entries
        .as_ref()
        .ok_or(CtfReportUnavailable::TableOwner)?;
    assert_eq!(
        rows[0].index_column,
        registry.named_indices["pdchIntOpUfactFilm"]
    );
    assert_eq!(rows[0].original_real_entry.to_bits(), interior_u.to_bits());
    assert_eq!(
        (rows[0].significant_digits, rows[0].original_entry_is_real),
        (3, true)
    );
    assert_eq!(
        (rows[0].sub_table_index, rows[0].unique_object_name),
        (0, 0)
    );
    Ok(())
}

#[test]
fn genuine_loader_resistance_and_zero_nominal_u_are_not_replaced()
-> Result<(), CtfReportUnavailable> {
    let mut model = model();
    let MaterialDefinition::NoMass(mut value) = model.materials[0].definition else {
        return Err(CtfReportUnavailable::Association);
    };
    model.materials[0].definition = MaterialDefinition::Regular(ep_model::RegularMaterial {
        roughness: value.roughness,
        thickness_m: 0.25,
        conductivity_w_per_m_k: 1.0,
        density_kg_per_m3: 1000.0,
        specific_heat_j_per_kg_k: 1000.0,
        thermal_resistance_m2_k_per_w: value.thermal_resistance_m2_k_per_w,
        surface: value.surface,
    });
    // Stored loader NominalR is an independent owner; it is not recomputed here
    // from deliberately different direct-test thickness/conductivity operands.
    assert_eq!(nominal_u(&model, &model.surfaces[0])?, 0.5);
    value.thermal_resistance_m2_k_per_w = 0.0;
    model.materials[0].definition = MaterialDefinition::NoMass(value);
    let mut table = PredefinedStringTable::default();
    report_ordinary_surface_nominal_u(&model, &registry()?, &mut table)?;
    let rows = table
        .entries
        .as_ref()
        .ok_or(CtfReportUnavailable::TableOwner)?;
    assert_eq!(rows[0].original_real_entry.to_bits(), 0.0_f64.to_bits());
    assert_eq!(rows[0].char_entry.as_slice(), b"      0.000");
    Ok(())
}

#[test]
fn unused_unsupported_definition_is_unread_but_used_one_stops_at_actual_prefix()
-> Result<(), CtfReportUnavailable> {
    let mut model = model();
    model.materials.push(Material {
        id: MaterialId(8),
        name: NormalizedName::new("UNREAD"),
        definition: MaterialDefinition::InfraredTransparent(ep_model::InfraredTransparentMaterial),
    });
    let mut table = PredefinedStringTable::default();
    report_ordinary_surface_nominal_u(&model, &registry()?, &mut table)?;
    assert_eq!(table.num_table_entry, 1);
    let mut second = model.surfaces[0].clone();
    second.id = SurfaceId(501);
    second.name = NormalizedName::new("LATER");
    second.construction = ConstructionId(31);
    model.surfaces.push(second);
    let mut construction = model.constructions[0].clone();
    construction.id = ConstructionId(31);
    construction.layers = vec![MaterialId(8)];
    model.constructions.push(construction);
    let mut table = PredefinedStringTable::default();
    assert_eq!(
        report_ordinary_surface_nominal_u(&model, &registry()?, &mut table),
        Err(CtfReportUnavailable::Material(MaterialId(8)))
    );
    assert_eq!(table.num_table_entry, 1);
    Ok(())
}

#[test]
fn unsupported_boundary_or_unregistered_surface_is_not_silently_dropped()
-> Result<(), CtfReportUnavailable> {
    let mut model = model();
    model.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Ground;
    assert_eq!(
        report_ordinary_surface_nominal_u(
            &model,
            &registry()?,
            &mut PredefinedStringTable::default()
        ),
        Err(CtfReportUnavailable::SurfaceBoundary(SurfaceId(500)))
    );
    model.surfaces[0].space = SpaceId(99);
    assert_eq!(
        report_ordinary_surface_nominal_u(
            &model,
            &registry()?,
            &mut PredefinedStringTable::default()
        ),
        Err(CtfReportUnavailable::Association)
    );
    Ok(())
}

#[test]
fn early_report_scan_is_shared_and_prescanned_owner_is_not_replayed() -> Result<(), String> {
    use ep_model::{ReportInputObject, ReportInputObjectType, ReportInputs};
    let input = ReportInputs {
        objects: vec![ReportInputObject {
            object_name: "authored report input".into(),
            object_type: ReportInputObjectType::Constructions,
            alphas: ["CONSTRUCTIONS".into(), String::new(), String::new()],
            alpha_blank: [false, true, true],
            default_applied: [false; 3],
            num_alphas: 1,
            is_epjson: true,
        }],
        legacy_variable_dictionary_sections: -1,
        output_variables: Vec::new(),
    };
    let mut reports = CtfInitializationReports::default();
    let mut state = SourceErrorState {
        output_error_header: true,
        version_string: "authored test version".into(),
        idd_version_string: "authored test IDD".into(),
        doing_input_processing: false,
        print_console_output: false,
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
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    };
    let mut eio = Vec::new();
    let mut console = Vec::new();
    let mut error = Vec::new();
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
    context
        .prepare_report_input_cache(&input)
        .map_err(|error| error.to_string())?;
    assert!(!context.reports.scan.get_report_input);
    assert!(context.reports.scan.constructions);
    assert!(context.reports.registry.reports.is_none());
    let reached = context.reports.scan.clone();
    // A separately supplied prescanned owner remains authoritative. Repeating
    // ordinary preparation cannot reread changed input or invent a new query.
    context
        .prepare_report_input_cache(&ReportInputs::default())
        .map_err(|error| error.to_string())?;
    assert_eq!(context.reports.scan, reached);
    assert_eq!(context.backend.state.tracking.total_warning_errors, 0);

    // The actual input owner takes the same production context branch; the
    // legacy checks above keep exercising the independent public entry point.
    use crate::heat_balance::selected_input_processor::{
        SelectedInputProcessor, SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
    };
    let source = SelectedInputProcessor::from_admitted_input(
        vec![SourceObjectDefinition {
            object_type: "Output:Constructions".into(),
            alpha_fields: 2,
            numeric_fields: 0,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        }],
        std::collections::BTreeMap::from([(
            "Output:Constructions".into(),
            SourceInputFamily::Objects(vec![SourceObjectShape {
                object_name: "authored report input".into(),
                field_sizes: std::collections::BTreeMap::new(),
            }]),
        )]),
    )
    .map_err(|error| error.to_string())?;
    context
        .reports
        .initialize_input_processor(source)
        .map_err(|error| error.to_string())?;
    context.reports.scan = crate::heat_balance::report_scan::ReportScanState::default();
    context
        .prepare_report_input_cache(&input)
        .map_err(|error| error.to_string())?;
    assert!(context.reports.scan.constructions);
    assert!(!context.reports.scan.get_report_input);
    assert_eq!(
        context
            .reports
            .input_processor_shortcut
            .current_module_object,
        "Output:EnergyManagementSystem"
    );
    assert!(
        context
            .reports
            .selected_input_processor
            .as_ref()
            .is_some_and(
                |source| !source.is_object_unused("Output:Constructions", "authored report input")
            )
    );
    let retained_input = context.reports.selected_input_processor.clone();
    let retained_shortcut = context.reports.input_processor_shortcut.clone();
    context
        .prepare_report_input_cache(&ReportInputs::default())
        .map_err(|error| error.to_string())?;
    assert_eq!(context.reports.selected_input_processor, retained_input);
    assert_eq!(context.reports.input_processor_shortcut, retained_shortcut);
    assert_eq!(context.backend.state.tracking.total_warning_errors, 0);

    use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
    use crate::heat_balance::source_output_registration::{
        SourceOutputFileHost, SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits,
        SourceRealOutputDeclaration, SourceReportVariableInput,
    };
    struct AuthoredMtd {
        opens: usize,
    }
    impl SourceOutputFileHost for AuthoredMtd {
        fn ensure_mtd_open(&mut self) -> std::io::Result<()> {
            self.opens += 1;
            Ok(())
        }
    }
    for (minimum, regex, expected_requests, used) in [
        ("", false, 2, true),
        ("", true, 1, true),
        ("Daily", false, 0, false),
    ] {
        let input = ReportInputs {
            output_variables: [
                (
                    "first",
                    if regex { "bad.regex" } else { "*" },
                    "BRACKET \t [W]",
                ),
                ("second", "MiXeD", "NO BRACKET \t "),
            ]
            .into_iter()
            .map(
                |(identity, key, name)| ep_model::SourceOutputVariableInput {
                    object_name: identity.into(),
                    key: key.into(),
                    variable_name: name.into(),
                    reporting_frequency: "Hourly".into(),
                    schedule_name: String::new(),
                    alpha_blank: [false, false, true, true],
                    default_applied: [false, false, true, false],
                    num_alphas: 2,
                    is_epjson: true,
                },
            )
            .collect(),
            ..ReportInputs::default()
        };
        let mut source = SelectedInputProcessor::from_admitted_input(
            vec![SourceObjectDefinition {
                object_type: "Output:Variable".into(),
                alpha_fields: 4,
                numeric_fields: 0,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            }],
            std::collections::BTreeMap::from([(
                "Output:Variable".into(),
                SourceInputFamily::Objects(
                    input
                        .output_variables
                        .iter()
                        .map(|row| SourceObjectShape {
                            object_name: row.object_name.clone(),
                            field_sizes: std::collections::BTreeMap::new(),
                        })
                        .collect(),
                ),
            )]),
        )
        .map_err(|error| error.to_string())?;
        let scratch = context.reports.input_processor_shortcut.clone();
        let mut output = SourceOutputRegistration::default();
        let mut host = AuthoredMtd { opens: 0 };
        let mut eio = Vec::new();
        let mut operands = SourceReportVariableInput::new(&input, &mut source, minimum);
        let result = output.setup_unmetered_zone_real(
            &mut operands,
            &SourceOutputPreScan::default(),
            SourceRealOutputDeclaration {
                name: "UNMATCHED AUTHORED VARIABLE",
                key: "ZONE",
                units: SourceOutputUnits::C,
                store_type: SourceOutputStoreType::Average,
            },
            15,
            &mut host,
            &mut eio,
        );
        assert_eq!(result.is_ok(), minimum.is_empty() && !regex);
        assert_eq!(host.opens, 1);
        assert!(output.output_initialized);
        assert!(!output.get_output_input);
        assert_eq!(output.time_step_zone_sec.to_bits(), 900.0_f64.to_bits());
        assert_eq!(output.requested_variables.len(), expected_requests);
        assert_eq!(source.is_object_unused("Output:Variable", "first"), !used);
        assert_eq!(
            source.is_object_unused("Output:Variable", "second"),
            regex || !used
        );
        assert_eq!(
            context.reports.input_processor_shortcut, scratch,
            "the real Output getter uses only local 4A/1N argument arrays"
        );
        assert!(eio.is_empty());
        if minimum.is_empty() && !regex {
            assert_eq!(output.requested_variables[0].name, "BRACKET");
            assert_eq!(output.requested_variables[1].name, "NO BRACKET \t ");
            assert!(output.requested_variables[0].key.is_empty());
            assert_eq!(output.requested_variables[1].key, "MiXeD");
            assert_eq!(output.num_real_variables_setup, 1);
            assert_eq!(output.num_total_real_variables, 1);
        } else {
            assert!(output.dictionary_variables.is_empty());
            assert_eq!(output.num_real_variables_setup, 0);
            if regex {
                assert_eq!(output.requested_variables[0].key, "bad.regex");
                assert!(output.requested_variables[0].name.is_empty());
            }
        }
    }
    Ok(())
}
