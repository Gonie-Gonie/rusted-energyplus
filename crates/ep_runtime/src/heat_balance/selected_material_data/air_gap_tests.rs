use super::*;
use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
};
use crate::heat_balance::selected_project_data::ProjectInputValue as Value;
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::{
    CtfLayerContext, CtfLayerIssue, material_layer_inputs, preprocess_ctf_layers,
};
use ep_model::{
    AirGapMaterial, Material, NoMassMaterial, NormalizedName, OpaqueSurfaceProperties,
    RegularMaterial,
};

fn fixture(epjson: bool, resistance: f64) -> (TypedModel, SelectedMaterialInput) {
    let regular = Material {
        id: MaterialId(0),
        name: NormalizedName::new("Mass"),
        definition: MaterialDefinition::Regular(RegularMaterial {
            roughness: MaterialSurfaceRoughness::MediumRough,
            thickness_m: 0.1,
            conductivity_w_per_m_k: 2.0,
            density_kg_per_m3: 2000.0,
            specific_heat_j_per_kg_k: 800.0,
            thermal_resistance_m2_k_per_w: 0.05,
            surface: OpaqueSurfaceProperties::default(),
        }),
    };
    let no_mass = Material {
        id: MaterialId(1),
        name: NormalizedName::new("No mass"),
        definition: MaterialDefinition::NoMass(NoMassMaterial {
            roughness: MaterialSurfaceRoughness::MediumRough,
            thermal_resistance_m2_k_per_w: 0.2,
            surface: OpaqueSurfaceProperties::default(),
        }),
    };
    let gap = Material {
        id: MaterialId(2),
        name: NormalizedName::new("MiXeD gap"),
        definition: MaterialDefinition::AirGap(AirGapMaterial {
            thermal_resistance_m2_k_per_w: resistance,
        }),
    };
    let mut model = TypedModel {
        materials: vec![regular, no_mass, gap],
        ..TypedModel::default()
    };
    model.ctf_report_names.materials = vec![
        (MaterialId(0), "Mass".into()),
        (MaterialId(1), "NO MASS".into()),
        (MaterialId(2), "MIXED GAP".into()),
    ];
    let raw = SelectedMaterialInput {
        source_is_epjson: epjson,
        regular: vec![ProjectInputObject {
            object_name: "Mass".into(),
            idf_max_fields: (!epjson).then_some(6),
            fields: BTreeMap::from([
                ("roughness".into(), Value::String("MediumRough".into())),
                ("thickness".into(), Value::Number(0.1)),
                ("conductivity".into(), Value::Number(2.0)),
                ("density".into(), Value::Number(2000.0)),
                ("specific_heat".into(), Value::Number(800.0)),
            ]),
        }],
        no_mass: vec![ProjectInputObject {
            object_name: "No mass".into(),
            idf_max_fields: (!epjson).then_some(3),
            fields: BTreeMap::from([
                ("roughness".into(), Value::String("MediumRough".into())),
                ("thermal_resistance".into(), Value::Number(0.2)),
            ]),
        }],
        infrared_transparent: Vec::new(),
        air_gaps: vec![ProjectInputObject {
            object_name: "MiXeD gap".into(),
            idf_max_fields: (!epjson).then_some(2),
            fields: BTreeMap::from([("thermal_resistance".into(), Value::Number(resistance))]),
        }],
    };
    (model, raw)
}

fn error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "declared test version".into(),
        idd_version_string: "declared test IDD".into(),
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
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

fn get_materials(
    model: &TypedModel,
    raw: &SelectedMaterialInput,
) -> Result<
    (
        SelectedMaterialData,
        SelectedInputProcessor,
        SourceInputProcessorShortCut,
    ),
    RuntimeError,
> {
    let families = [
        ("Material", &raw.regular, 2, 7),
        ("Material:NoMass", &raw.no_mass, 2, 4),
        ("Material:AirGap", &raw.air_gaps, 1, 1),
        (
            "Material:InfraredTransparent",
            &raw.infrared_transparent,
            1,
            0,
        ),
    ];
    let definitions = families
        .iter()
        .map(|(family, _, alphas, numerics)| SourceObjectDefinition {
            object_type: (*family).into(),
            alpha_fields: *alphas,
            numeric_fields: *numerics,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        })
        .collect();
    let mut input = SelectedInputProcessor::from_admitted_input(
        definitions,
        families
            .into_iter()
            .map(|(family, objects, _, _)| {
                (
                    family.into(),
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
            .collect(),
    )
    .expect("declared schema and actual raw identities");
    let mut shortcut = SourceInputProcessorShortCut::default();
    shortcut
        .allocate_from_input(&input)
        .expect("actual shared allocation");
    shortcut.alpha_args.fill("stale".into());
    shortcut.numeric_args.fill(-123.0);
    shortcut.alpha_field_names.fill("stale".into());
    shortcut.numeric_field_names.fill("stale".into());
    let mut scan = ReportScanState::default();
    let globals = SelectedHeatBalanceGlobals::default();
    let mut state = error_state();
    let mut errors = Vec::new();
    let mut console = Vec::new();
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
    let mut eio = Vec::new();
    let mut errors_found = false;
    let mut data = SelectedMaterialData::default();
    data.get_material_data(
        raw,
        &mut MaterialDataOwners {
            model,
            input: &mut input,
            shortcut: &mut shortcut,
            scan: &mut scan,
            globals: &globals,
            backend: &mut backend,
            eio: &mut eio,
            errors_found: &mut errors_found,
            diagnostic_flags: None,
        },
    )?;
    assert!(!errors_found);
    assert!(errors.is_empty());
    Ok((data, input, shortcut))
}

#[test]
fn air_gap_getter_retains_real_group_and_ronly_through_ctf_and_material_report() {
    for epjson in [false, true] {
        for resistance in [0.15, 0.001, 0.0005] {
            let (model, raw) = fixture(epjson, resistance);
            let (data, input, shortcut) =
                get_materials(&model, &raw).expect("actual material getter");
            assert_eq!(
                data.materials,
                [MaterialId(0), MaterialId(1), MaterialId(2)]
            );
            assert_eq!(data.material_map["MIXED GAP"], 3);
            assert_eq!(
                data.material_groups[&MaterialId(2)],
                SourceMaterialGroup::AirGap
            );
            assert_eq!(
                data.resistance_only.values().copied().collect::<Vec<_>>(),
                [false, true, true]
            );
            assert_eq!(data.nominal_r, [0.05, 0.2, resistance]);
            for (family, name) in [
                ("Material", "Mass"),
                ("Material:NoMass", "No mass"),
                ("Material:AirGap", "MiXeD gap"),
            ] {
                assert!(!input.is_object_unused(family, name));
            }
            assert_eq!(shortcut.alpha_args, ["MIXED GAP", ""]);
            assert_eq!(shortcut.numeric_args[0].to_bits(), resistance.to_bits());
            assert!(
                shortcut.numeric_args[1..]
                    .iter()
                    .all(|value| value.to_bits() == 0)
            );
            assert_eq!(shortcut.alpha_field_blanks, [false, true]);
            assert!(!shortcut.numeric_field_blanks[0]);
            assert!(
                shortcut.numeric_field_blanks[1..]
                    .iter()
                    .all(|value| *value)
            );
            assert_eq!(
                shortcut.alpha_field_names,
                [if epjson { "name" } else { "Name" }, ""]
            );
            assert_eq!(
                shortcut.numeric_field_names[0],
                if epjson {
                    "thermal_resistance"
                } else {
                    "Thermal Resistance"
                }
            );
            assert!(
                shortcut.numeric_field_names[1..]
                    .iter()
                    .all(String::is_empty)
            );

            let layers = material_layer_inputs(&[
                &model.materials[0],
                &model.materials[2],
                &model.materials[0],
            ])
            .expect("genuine opaque material adapter");
            assert_eq!(
                layers[1].resistance_only,
                data.resistance_only[&MaterialId(2)]
            );
            assert_eq!(
                [
                    layers[1].thickness,
                    layers[1].conductivity,
                    layers[1].density,
                    layers[1].specific_heat
                ],
                [0.0; 4]
            );
            assert_eq!(layers[1].resistance.to_bits(), resistance.to_bits());
            let result = preprocess_ctf_layers(
                &layers,
                CtfLayerContext {
                    is_used_ctf: true,
                    errors_found: false,
                    source_sink_present: false,
                    solution_dimensions: 1,
                },
            )
            .expect("existing genuine CTF loading");
            assert_eq!(result.errors_found, resistance < 0.001);
            if resistance < 0.001 {
                assert_eq!(result.issues, [CtfLayerIssue::ResistanceBelowMinimum(2)]);
                assert!(result.after_conversion.is_none());
            } else {
                let loaded = result.after_load.expect("actual complete loading loop");
                assert_eq!(loaded.num_res_layers, 1);
                assert_eq!(
                    [
                        loaded.layers[1].cp,
                        loaded.layers[1].rho,
                        loaded.layers[1].rk,
                        loaded.layers[1].dl
                    ],
                    [0.0, 0.0, 1.0, resistance]
                );
                assert!(result.after_conversion.is_some());
            }
            let mut eio = Vec::new();
            data.write_material_reports(&model, &mut eio)
                .expect("actual Material reports");
            let text = String::from_utf8(eio).expect("EIO text");
            assert_eq!(
                text.lines()
                    .filter(|line| line.starts_with(" Material Details,"))
                    .count(),
                2
            );
            assert_eq!(
                text.lines()
                    .filter(|line| line.starts_with(" Material:Air,"))
                    .count(),
                1
            );
            if resistance == 0.15 {
                assert!(text.ends_with(" Material:Air,MIXED GAP,0.1500\n"));
            }
        }
    }
    // Extend the same real ordered getter/CTF-owner test with the name-only
    // fourth opaque family, keeping its genuine group and Invalid roughness.
    for epjson in [false, true] {
        let (mut model, mut raw) = fixture(epjson, 0.15);
        let id = MaterialId(3);
        model.materials.push(Material {
            id,
            name: NormalizedName::new("Mixed IRT"),
            definition: MaterialDefinition::InfraredTransparent(
                ep_model::InfraredTransparentMaterial,
            ),
        });
        model
            .ctf_report_names
            .materials
            .push((id, "MIXED IRT".into()));
        raw.infrared_transparent.push(ProjectInputObject {
            object_name: "MiXeD IRT".into(),
            idf_max_fields: (!epjson).then_some(1),
            fields: BTreeMap::new(),
        });
        let (data, input, shortcut) =
            get_materials(&model, &raw).expect("actual name-only IRT getter");
        assert_eq!(data.num_irts, 1);
        assert_eq!(
            data.materials,
            [MaterialId(0), MaterialId(1), MaterialId(2), id]
        );
        assert_eq!(data.material_map["MIXED IRT"], 4);
        assert_eq!(
            data.material_groups[&id],
            SourceMaterialGroup::IrTransparent
        );
        assert!(data.resistance_only[&id]);
        assert_eq!(data.nominal_r[3].to_bits(), 0.01_f64.to_bits());
        assert!(!input.is_object_unused("Material:InfraredTransparent", "MiXeD IRT"));
        assert_eq!(shortcut.alpha_args, ["MIXED IRT", ""]);
        assert_eq!(
            shortcut.alpha_field_names,
            [if epjson { "name" } else { "Name" }, ""]
        );
        assert_eq!(shortcut.alpha_field_blanks, [false, true]);
        assert!(
            shortcut
                .numeric_args
                .iter()
                .all(|value| value.to_bits() == 0)
        );
        assert!(shortcut.numeric_field_blanks.iter().all(|value| *value));
        assert!(shortcut.numeric_field_names.iter().all(String::is_empty));
        let irt = &model.materials[3];
        assert_eq!(irt.roughness(), None);
        let properties = irt.surface_properties().expect("fixed IRT properties");
        assert_eq!(
            properties.thermal_absorptance.to_bits(),
            0.9999_f64.to_bits()
        );
        assert_eq!(properties.solar_absorptance.to_bits(), 1.0_f64.to_bits());
        assert_eq!(properties.visible_absorptance.to_bits(), 1.0_f64.to_bits());
        let layers = material_layer_inputs(&[irt]).expect("same genuine IRT base defaults for CTF");
        assert_eq!(layers.len(), 1);
        assert_eq!(
            [
                layers[0].thickness,
                layers[0].conductivity,
                layers[0].density,
                layers[0].specific_heat
            ],
            [0.0; 4]
        );
        assert_eq!(layers[0].resistance.to_bits(), data.nominal_r[3].to_bits());
        assert_eq!(layers[0].resistance_only, data.resistance_only[&id]);
    }
}

#[test]
fn air_gap_getter_rejects_missing_resistance_and_wrong_scalar_or_typed_custody() {
    for epjson in [false, true] {
        for value in [
            None,
            Some(Value::String(String::new())),
            Some(Value::String("0.15".into())),
            Some(Value::Number(f64::NAN)),
            Some(Value::Number(0.16)),
        ] {
            let (model, mut raw) = fixture(epjson, 0.15);
            raw.air_gaps[0].fields.remove("thermal_resistance");
            if let Some(value) = value {
                raw.air_gaps[0]
                    .fields
                    .insert("thermal_resistance".into(), value);
            }
            assert!(get_materials(&model, &raw).is_err());
        }
        let (mut model, raw) = fixture(epjson, 0.15);
        model.materials[2].definition = model.materials[1].definition;
        assert!(
            get_materials(&model, &raw).is_err(),
            "AirGap must never borrow NoMass identity"
        );
    }
    // A cross-family duplicate has no IRT typed owner. The native map check
    // must emit its one Severe before typed binding, then continue on success.
    struct BrokenDiagnostic;
    impl std::io::Write for BrokenDiagnostic {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("authored IRT diagnostic failure"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    for failure in [false, true] {
        let (mut model, mut raw) = fixture(true, 0.15);
        let id = MaterialId(3);
        model.materials.push(Material {
            id,
            name: NormalizedName::new("IRT AFTER"),
            definition: MaterialDefinition::InfraredTransparent(
                ep_model::InfraredTransparentMaterial,
            ),
        });
        model
            .ctf_report_names
            .materials
            .push((id, "IRT AFTER".into()));
        raw.infrared_transparent = ["nO mAsS", "IRT AFTER"]
            .into_iter()
            .map(|name| ProjectInputObject {
                object_name: name.into(),
                fields: BTreeMap::new(),
                idf_max_fields: None,
            })
            .collect();
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            (
                "Material:InfraredTransparent",
                &raw.infrared_transparent,
                1,
                0,
            ),
        ];
        let definitions = families
            .iter()
            .map(|(name, _, alphas, numerics)| SourceObjectDefinition {
                object_type: (*name).into(),
                alpha_fields: *alphas,
                numeric_fields: *numerics,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(name, rows, _, _)| {
                (
                    name.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
                                    .fields
                                    .keys()
                                    .map(|name| (name.clone(), 1))
                                    .collect(),
                            })
                            .collect(),
                    ),
                )
            })
            .collect();
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes)
            .expect("actual duplicate raw families");
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut
            .allocate_from_input(&input)
            .expect("same real shared arrays");
        let globals = SelectedHeatBalanceGlobals {
            doing_sizing: true,
            ..Default::default()
        };
        let clock = crate::weather::day::WeatherGlobalState {
            warmup_flag: true,
            ..Default::default()
        };
        let developer = true;
        let flags = SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &developer,
            warmup: &clock.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        };
        let mut state = error_state(); // Its legacy sizing/warmup fields stay false.
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut broken = BrokenDiagnostic;
        let mut scan = ReportScanState::default();
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();
        let outcome = {
            let stream: &mut dyn std::io::Write = if failure { &mut broken } else { &mut text };
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(stream),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            data.get_material_data(
                &raw,
                &mut MaterialDataOwners {
                    model: &model,
                    input: &mut input,
                    shortcut: &mut shortcut,
                    scan: &mut scan,
                    globals: &globals,
                    backend: &mut backend,
                    eio: &mut eio,
                    errors_found: &mut errors_found,
                    diagnostic_flags: Some(flags),
                },
            )
        };
        assert_eq!(state.tracking.total_severe_errors, 1);
        assert_eq!(state.tracking.total_severe_errors_during_sizing, 1);
        assert_eq!(state.tracking.total_severe_errors_during_warmup, 0);
        assert_eq!(data.material_map["NO MASS"], 2);
        assert_eq!(
            data.material_groups[&MaterialId(1)],
            SourceMaterialGroup::Regular
        );
        assert!(!input.is_object_unused("Material:InfraredTransparent", "nO mAsS"));
        assert_eq!(
            input.is_object_unused("Material:InfraredTransparent", "IRT AFTER"),
            failure
        );
        assert_eq!(
            errors_found, !failure,
            "native ErrorsFound assignment follows successful Severe"
        );
        if failure {
            assert!(outcome.is_err());
            assert_eq!(data.materials.len(), 3);
            assert_eq!(shortcut.alpha_args[0], "NO MASS");
            assert!(
                shortcut
                    .numeric_args
                    .iter()
                    .all(|value| value.to_bits() == 0)
            );
            assert!(
                state.tracking.last_severe_error.is_empty(),
                "last-text write follows successful diagnostic output"
            );
        } else {
            outcome.expect("one Severe then native continue");
            assert_eq!(data.materials.len(), 4);
            assert_eq!(data.material_map["IRT AFTER"], 4);
            let message =
                "GetMaterialData: Material:InfraredTransparent = NO MASS, duplicate name.";
            assert_eq!(state.tracking.last_severe_error, message);
            let text = std::str::from_utf8(&text).expect("actual error bytes");
            assert_eq!(text.matches(message).count(), 1);
            assert!(
                !text.contains(" **   ~~~   **"),
                "duplicate-name helper emits no Continue"
            );
        }
    }
}
