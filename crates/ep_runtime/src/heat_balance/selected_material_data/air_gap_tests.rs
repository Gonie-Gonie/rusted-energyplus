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
}
