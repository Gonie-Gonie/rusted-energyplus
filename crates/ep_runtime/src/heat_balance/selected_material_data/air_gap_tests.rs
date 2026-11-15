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
        simple_glazing: Vec::new(),
        window_gas: Vec::new(),
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
        ("WindowMaterial:Gas", &raw.window_gas, 2, 12),
        (
            "WindowMaterial:SimpleGlazingSystem",
            &raw.simple_glazing,
            1,
            3,
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
    // Same material fixture/IP/getter/report owner, now with one UNUSED real SGS row.
    let (mut model, mut raw) = fixture(true, 0.15);
    let id = MaterialId(3);
    model.materials.push(Material {
        id,
        name: NormalizedName::new("Simple"),
        definition: MaterialDefinition::WindowSimpleGlazing(
            ep_model::WindowSimpleGlazingMaterial::from_performance_indices(4.2, 0.57, None),
        ),
    });
    model.ctf_report_names.materials.push((id, "SIMPLE".into()));
    raw.simple_glazing.push(ProjectInputObject {
        object_name: "SiMpLe".into(),
        idf_max_fields: None,
        fields: BTreeMap::from([
            ("u_factor".into(), Value::Number(4.2)),
            ("solar_heat_gain_coefficient".into(), Value::Number(0.57)),
        ]),
    });
    let (data, input, shortcut) = get_materials(&model, &raw).expect("actual SGS getter");
    assert_eq!(data.num_simple_windows, 1);
    assert_eq!(data.material_map["SIMPLE"], 4);
    assert_eq!(data.material_groups[&id], SourceMaterialGroup::GlassSimple);
    assert!(!data.resistance_only[&id]);
    assert!(!input.is_object_unused("WindowMaterial:SimpleGlazingSystem", "SiMpLe"));
    assert_eq!(shortcut.alpha_args, ["SIMPLE", ""]);
    assert_eq!(&shortcut.numeric_args[..3], &[4.2, 0.57, 0.0]);
    assert_eq!(&shortcut.numeric_field_blanks[..3], &[false, false, true]);
    assert_eq!(
        &shortcut.numeric_field_names[..3],
        &[
            "u_factor",
            "solar_heat_gain_coefficient",
            "visible_transmittance"
        ]
    );
    let material = &data.simple_glazing[&id];
    assert!(!material.visible_input_by_user);
    assert_eq!(material.visible_input.to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        material.visible_transmittance.to_bits(),
        material.transmittance.to_bits()
    );
    assert_eq!(
        material.visible_reflectance_back.to_bits(),
        material.solar_reflectance_back.to_bits()
    );
    assert_eq!(
        material.visible_reflectance_front.to_bits(),
        material.solar_reflectance_front.to_bits()
    );
    assert_eq!(
        material.roughness,
        Some(MaterialSurfaceRoughness::VerySmooth)
    );
    assert_eq!(
        [
            material.density,
            material.specific_heat,
            material.solar_absorptance,
            material.visible_absorptance,
            material.thermal_transmittance
        ],
        [0.0; 5]
    );
    assert_eq!(
        [
            material.thermal_absorptance,
            material.thermal_back,
            material.thermal_front
        ],
        [0.84; 3]
    );
    assert_eq!(material.spectral_data_pointer, 0);
    assert!(!material.solar_diffusing);
    assert_eq!(material.dirt_factor, 1.0);
    assert_eq!(data.nominal_r[3].to_bits(), material.resistance.to_bits());
    // This same checked path is consumed by post-runtime geometry. The
    // typed SGS intentionally has no opaque properties; read the real row.
    assert!(model.materials[3].surface_properties().is_none());
    let (roughness, surface) = data
        .source_simple_glazing_surface_properties(4, id)
        .expect("actual retained SGS geometry properties");
    assert_eq!(Some(roughness), material.roughness);
    assert_eq!(
        [
            surface.thermal_absorptance,
            surface.solar_absorptance,
            surface.visible_absorptance,
        ],
        [
            material.thermal_absorptance,
            material.solar_absorptance,
            material.visible_absorptance,
        ]
    );
    assert!(
        data.source_simple_glazing_surface_properties(3, id)
            .is_err()
    );
    let mut eio = Vec::new();
    data.write_material_reports(&model, &mut eio)
        .expect("real SGS Details row");
    let text = std::str::from_utf8(&eio).expect("actual EIO UTF8");
    let details = text
        .lines()
        .find(|line| line.starts_with(" Material Details,SIMPLE,"))
        .expect("actual material order includes the unused SGS row");
    assert!(details.contains(",VerySmooth,"));
    assert!(details.ends_with(",0.000,0.000,0.8400,0.0000,0.0000"));
    raw.simple_glazing[0]
        .fields
        .insert("visible_transmittance".into(), Value::Number(0.61));
    model.materials[3].definition = MaterialDefinition::WindowSimpleGlazing(
        ep_model::WindowSimpleGlazingMaterial::from_performance_indices(4.2, 0.57, Some(0.61)),
    );
    let (provided, _, args) = get_materials(&model, &raw).expect("same getter explicit VT");
    let material = &provided.simple_glazing[&id];
    assert!(material.visible_input_by_user);
    assert_eq!(material.visible_input.to_bits(), 0.61_f64.to_bits());
    assert_eq!(material.visible_transmittance.to_bits(), 0.61_f64.to_bits());
    assert!(!args.numeric_field_blanks[2]);
    // Reuse the original SGS model/raw fixture only AFTER every old assertion.
    // Two additional real material/IP owner calls cover normal and numeric-edge
    // rows; no consumed owner or input flag is reset.
    let named_gases = [
        (
            "A air",
            ep_model::WindowGasType::Air,
            0.012,
            2.873e-3,
            28.97,
        ),
        (
            "B argon",
            ep_model::WindowGasType::Argon,
            0.013,
            2.285e-3,
            39.948,
        ),
        (
            "C krypton",
            ep_model::WindowGasType::Krypton,
            0.014,
            9.443e-4,
            83.8,
        ),
        (
            "D xenon",
            ep_model::WindowGasType::Xenon,
            0.015,
            4.538e-4,
            131.3,
        ),
    ];
    for (name, gas_type, thickness, _, _) in named_gases {
        let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
        let mut advisory = gas_type.standard_properties().unwrap();
        advisory.conductivity.coefficient_a = 99.0; // Deliberately not runtime state.
        model.materials.push(Material {
            id,
            name: NormalizedName::new(name),
            definition: MaterialDefinition::WindowGas(ep_model::WindowGasMaterial {
                gas_type,
                thickness_m: thickness,
                properties: advisory,
            }),
        });
        model
            .ctf_report_names
            .materials
            .push((id, name.to_ascii_uppercase()));
        raw.window_gas.push(ProjectInputObject {
            object_name: name.into(),
            idf_max_fields: None,
            fields: BTreeMap::from([
                (
                    "gas_type".into(),
                    Value::String(gas_type.energyplus_name().into()),
                ),
                ("thickness".into(), Value::Number(thickness)),
            ]),
        });
    }
    let (gases, gas_input, _) = get_materials(&model, &raw).expect("actual four named Gas owners");
    assert_eq!(gases.num_w5_gases, 4);
    assert_eq!(gases.materials.len(), 8);
    assert_eq!(
        gases.materials[7],
        MaterialId(3),
        "Gas phase precedes existing SGS"
    );
    for (offset, (name, gas_type, thickness, k0, weight)) in named_gases.into_iter().enumerate() {
        let id = MaterialId(u32::try_from(4 + offset).unwrap());
        let source_number = i32::try_from(4 + offset).unwrap();
        let row = &gases.window_gas[&id];
        assert_eq!(
            gases.material_map[&name.to_ascii_uppercase()],
            source_number
        );
        assert_eq!(gases.material_groups[&id], SourceMaterialGroup::Gas);
        assert!(gases.resistance_only[&id]);
        assert_eq!(row.num_gases, 1);
        assert_eq!(row.gas_fractions, [1.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(row.gap_vent_type, ep_model::WindowGapVentType::Sealed);
        assert_eq!(row.gases[0].gas_type, gas_type);
        assert_eq!(row.gases[0].properties.conductivity.coefficient_a, k0);
        assert_eq!(row.gases[0].properties.molecular_weight_g_per_mol, weight);
        assert!(
            row.gases[1..]
                .iter()
                .all(|gas| gas.gas_type == ep_model::WindowGasType::Custom
                    && gas.properties.conductivity.coefficient_a == 0.0
                    && gas.properties.molecular_weight_g_per_mol == 0.0)
        );
        assert_eq!(row.thickness.to_bits(), thickness.to_bits());
        assert_eq!(row.roughness, Some(MaterialSurfaceRoughness::MediumRough));
        assert_eq!(
            [
                row.resistance,
                row.conductivity,
                row.density,
                row.specific_heat,
                row.thermal_absorptance,
                row.solar_absorptance,
                row.visible_absorptance
            ],
            [0.0; 7]
        );
        assert!(gases.nominal_r[usize::try_from(source_number - 1).unwrap()] > 0.0);
        assert!(!gas_input.is_object_unused("WindowMaterial:Gas", name));
        assert!(model.materials[4 + offset].surface_properties().is_none());
        let (roughness, surface) = gases
            .source_window_gas_surface_properties(source_number, id)
            .expect("same actual Gas property path as whole geometry");
        assert_eq!(Some(roughness), row.roughness);
        assert_eq!(
            [
                surface.thermal_absorptance,
                surface.solar_absorptance,
                surface.visible_absorptance
            ],
            [0.0; 3]
        );
        assert!(
            gases
                .source_window_gas_surface_properties(source_number - 1, id)
                .is_err()
        );
    }
    assert!(
        gases
            .source_simple_glazing_surface_properties(8, MaterialId(3))
            .is_ok()
    );
    let mut gas_eio = Vec::new();
    gases
        .write_material_reports(&model, &mut gas_eio)
        .expect("actual common Material Details");
    let text = std::str::from_utf8(&gas_eio).unwrap();
    let mut previous = 0;
    for name in ["A AIR", "B ARGON", "C KRYPTON", "D XENON"] {
        let prefix = format!(" Material Details,{name},0.0000,MediumRough,");
        let at = text
            .find(&prefix)
            .expect("actual base Resistance0, not NominalR");
        assert!(at > previous);
        previous = at;
        let row = text[at..].lines().next().unwrap();
        assert!(row.ends_with(",0.000,0.000,0.000,0.0000,0.0000,0.0000"));
    }
    assert!(previous < text.find(" Material Details,SIMPLE,").unwrap());
    let wrapped = ep_model::classify_source_json_number("18446744073709551615")
        .expect("actual unsigned-stored/signed-i64 IP conversion")
        .as_f64();
    assert_eq!(wrapped.to_bits(), (-1.0_f64).to_bits());
    raw.window_gas[0]
        .fields
        .insert("thickness".into(), Value::Number(wrapped));
    raw.window_gas[1]
        .fields
        .insert("thickness".into(), Value::Number(f64::MAX));
    if let MaterialDefinition::WindowGas(value) = &mut model.materials[4].definition {
        value.thickness_m = "18446744073709551615".parse::<f64>().unwrap();
    }
    if let MaterialDefinition::WindowGas(value) = &mut model.materials[5].definition {
        value.thickness_m = f64::MAX;
    }
    let (edges, _, _) = get_materials(&model, &raw)
        .expect("Native has no effective-positive or derived-finite filter");
    assert_eq!(
        edges.window_gas[&MaterialId(4)].thickness.to_bits(),
        wrapped.to_bits()
    );
    assert!(edges.nominal_r[3] < 0.0);
    assert!(edges.nominal_r[4].is_infinite() && edges.nominal_r[4].is_sign_positive());
    assert_eq!(edges.window_gas[&MaterialId(4)].resistance, 0.0);
    assert_eq!(edges.window_gas[&MaterialId(5)].resistance, 0.0);
    assert!(
        edges
            .source_window_gas_surface_properties(4, MaterialId(4))
            .is_ok()
    );
    assert!(
        edges
            .source_window_gas_surface_properties(5, MaterialId(5))
            .is_ok()
    );
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
    // One focused real material/IP/backend setup exercises the SGS diagnostic
    // prefixes; it grants no successful run credit to the fatal input.
    let (mut model, mut raw) = fixture(true, 0.15);
    // The same actual EPJSON family order is lexical A/B/C, including the duplicate.
    model.materials[1].name = NormalizedName::new("A DUPLICATE");
    model.ctf_report_names.materials[1].1 = "A DUPLICATE".into();
    raw.no_mass[0].object_name = "A DUPLICATE".into();
    let film_id = MaterialId(3);
    let fatal_id = MaterialId(4);
    model.materials.push(Material {
        id: film_id,
        name: NormalizedName::new("B FILM"),
        definition: MaterialDefinition::WindowSimpleGlazing(
            ep_model::WindowSimpleGlazingMaterial::from_performance_indices(99.0, 0.5, None),
        ),
    });
    model.materials.push(Material {
        id: fatal_id,
        name: NormalizedName::new("C FATAL"),
        definition: MaterialDefinition::WindowSimpleGlazing(
            ep_model::WindowSimpleGlazingMaterial::from_performance_indices(1.0e-320, 0.5, None),
        ),
    });
    model
        .ctf_report_names
        .materials
        .extend([(film_id, "B FILM".into()), (fatal_id, "C FATAL".into())]);
    raw.simple_glazing = [
        ("A DUPLICATE", 2.7),
        ("B FILM", 99.0),
        ("C FATAL", 1.0e-320),
    ]
    .into_iter()
    .map(|(name, u)| ProjectInputObject {
        object_name: name.into(),
        idf_max_fields: None,
        fields: BTreeMap::from([
            ("u_factor".into(), Value::Number(u)),
            ("solar_heat_gain_coefficient".into(), Value::Number(0.5)),
        ]),
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
        ("WindowMaterial:Gas", &raw.window_gas, 2, 12),
        (
            "WindowMaterial:SimpleGlazingSystem",
            &raw.simple_glazing,
            1,
            3,
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
                            field_sizes: row.fields.keys().map(|name| (name.clone(), 1)).collect(),
                        })
                        .collect(),
                ),
            )
        })
        .collect();
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes)
        .expect("actual SGS raw/IP families");
    let mut shortcut = SourceInputProcessorShortCut::default();
    shortcut
        .allocate_from_input(&input)
        .expect("same complete shared arrays");
    let globals = SelectedHeatBalanceGlobals {
        doing_sizing: true,
        ..Default::default()
    };
    let clock = crate::weather::day::WeatherGlobalState {
        warmup_flag: true,
        ..Default::default()
    };
    let developer = false;
    let flags = SourceDiagnosticFlags {
        kickoff: &globals.kickoff_simulation,
        developer: &developer,
        warmup: &clock.warmup_flag,
        doing_sizing: &globals.doing_sizing,
    };
    let mut state = error_state(); // Copied legacy phase flags remain false.
    let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
    let mut scan = ReportScanState::default();
    let mut errors_found = false;
    let mut data = SelectedMaterialData::default();
    let outcome = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut text),
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
    assert!(
        matches!(&outcome, Err(RuntimeError::HeatBalanceInput(error))
        if matches!(error.as_ref(), crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Fatal(_)))
    );
    assert!(
        errors_found,
        "duplicate sets the actual outer input error owner"
    );
    assert_eq!(data.num_simple_windows, 3);
    assert_eq!(
        data.material_map["A DUPLICATE"], 2,
        "duplicate never replaces the earlier owner"
    );
    assert_eq!(data.materials.len(), 5, "duplicate does not allocate a row");
    let film = &data.simple_glazing[&film_id];
    assert_eq!(
        [film.thickness, film.conductivity, film.resistance],
        [0.002, 2.0, 0.001]
    );
    assert_eq!(data.nominal_r[3].to_bits(), 0.001_f64.to_bits());
    let failed = &data.simple_glazing[&fatal_id];
    assert_eq!(failed.u_factor.to_bits(), 1.0e-320_f64.to_bits());
    assert_eq!(failed.conductivity.to_bits(), 0.0_f64.to_bits());
    assert_eq!(failed.resistance.to_bits(), 0.0_f64.to_bits());
    assert_eq!(data.nominal_r[4].to_bits(), 0.0_f64.to_bits());
    assert_eq!(failed.thermal_absorptance.to_bits(), 0.84_f64.to_bits());
    assert!(
        failed.transmittance > 0.0,
        "solar writes follow the conductivity Severe"
    );
    assert_eq!(
        failed.visible_transmittance.to_bits(),
        failed.transmittance.to_bits()
    );
    assert_eq!(state.tracking.total_warning_errors, 1);
    assert_eq!(state.tracking.total_warning_errors_during_sizing, 1);
    assert_eq!(state.tracking.total_severe_errors, 2);
    assert_eq!(state.tracking.total_severe_errors_during_sizing, 2);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 0);
    assert!(!input.is_object_unused("WindowMaterial:SimpleGlazingSystem", "A DUPLICATE"));
    assert!(!input.is_object_unused("WindowMaterial:SimpleGlazingSystem", "B FILM"));
    assert!(!input.is_object_unused("WindowMaterial:SimpleGlazingSystem", "C FATAL"));
    assert!(
        eio.is_empty(),
        "Setup Fatal precedes the Materials Scan/report suffix"
    );
    let text = std::str::from_utf8(&text).expect("actual diagnostic bytes");
    let duplicate =
        "GetMaterialData: WindowMaterial:SimpleGlazingSystem = A DUPLICATE, duplicate name.";
    let warning = "WindowMaterial:SimpleGlazingSystem: B FILM has U-factor higher than that provided by surface film resistances, Check value of U-factor";
    let severe = "WindowMaterial:SimpleGlazingSystem: C FATAL has Conductivity <= 0.0, must be >0.0, Check value of U-factor";
    let fatal = "Program halted because of input problem(s) in WindowMaterial:SimpleGlazingSystem";
    assert_eq!(text.matches(duplicate).count(), 1);
    assert_eq!(text.matches(warning).count(), 1);
    assert_eq!(text.matches(&format!(" ** Severe  ** {severe}")).count(), 1);
    assert_eq!(text.matches(fatal).count(), 1);
    assert!(text.find(duplicate) < text.find(warning));
    assert!(text.find(warning) < text.find(severe));
    assert!(text.find(severe) < text.find(fatal));
    // Two genuinely fresh owner contexts. The original IRT/SGS diagnostics,
    // phase flags, failed prefixes and all old assertions above remain intact.
    for failure in [false, true] {
        let (mut model, mut raw) = fixture(true, 0.15);
        for (name, gas_type) in [
            ("X BASIC", ep_model::WindowGasType::Air),
            ("Z OPTIONAL", ep_model::WindowGasType::Argon),
        ] {
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material {
                id,
                name: NormalizedName::new(name),
                definition: MaterialDefinition::WindowGas(ep_model::WindowGasMaterial {
                    gas_type,
                    thickness_m: 0.012,
                    properties: gas_type.standard_properties().unwrap(),
                }),
            });
            model.ctf_report_names.materials.push((id, name.into()));
        }
        raw.window_gas = [
            ("No mass", "Air"),
            ("X BASIC", "Air"),
            ("Z OPTIONAL", "Argon"),
        ]
        .into_iter()
        .map(|(name, gas_type)| ProjectInputObject {
            object_name: name.into(),
            idf_max_fields: None,
            fields: BTreeMap::from([
                ("gas_type".into(), Value::String(gas_type.into())),
                ("thickness".into(), Value::Number(0.012)),
            ]),
        })
        .collect();
        raw.window_gas[2]
            .fields
            .insert("conductivity_coefficient_c".into(), Value::Number(0.0));
        raw.window_gas[2]
            .fields
            .insert("specific_heat_ratio".into(), Value::Number(1.5));
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
            ("WindowMaterial:Gas", &raw.window_gas, 2, 12),
            (
                "WindowMaterial:SimpleGlazingSystem",
                &raw.simple_glazing,
                1,
                3,
            ),
        ];
        let definitions = families
            .iter()
            .map(|(name, _, a, n)| SourceObjectDefinition {
                object_type: (*name).into(),
                alpha_fields: *a,
                numeric_fields: *n,
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let mut scan = ReportScanState::default();
        let mut state = error_state();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();
        let mut broken = BrokenDiagnostic;
        assert!(input.is_object_unused("WindowMaterial:Gas", "X BASIC"));
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
            let mut owners = MaterialDataOwners {
                model: &model,
                input: &mut input,
                shortcut: &mut shortcut,
                scan: &mut scan,
                globals: &globals,
                backend: &mut backend,
                eio: &mut eio,
                errors_found: &mut errors_found,
                diagnostic_flags: Some(SourceDiagnosticFlags {
                    kickoff: &globals.kickoff_simulation,
                    developer: &developer,
                    warmup: &clock.warmup_flag,
                    doing_sizing: &globals.doing_sizing,
                }),
            };
            let result = data.get_material_data(&raw, &mut owners);
            if !failure {
                // Observe the SAME real input primitive's local return/counts;
                // do not invent a stored count or reset consumed input flags.
                assert_eq!(
                    raw.get_window_gas_item(&raw.window_gas[1], &mut owners)
                        .unwrap(),
                    (2, 1)
                );
                assert_eq!(owners.shortcut.alpha_args, ["X BASIC", "AIR"]);
                assert!(!owners.shortcut.numeric_field_blanks[0]);
                assert!(
                    owners.shortcut.numeric_field_blanks[1..]
                        .iter()
                        .all(|value| *value)
                );
                assert!(
                    owners.shortcut.numeric_args[1..]
                        .iter()
                        .all(|value| value.to_bits() == 0)
                );
                assert_eq!(owners.shortcut.numeric_field_names.len(), 12);
                assert_eq!(
                    owners.shortcut.numeric_field_names[11],
                    "specific_heat_ratio"
                );
                assert_eq!(
                    raw.get_window_gas_item(&raw.window_gas[2], &mut owners)
                        .unwrap(),
                    (2, 12)
                );
                assert_eq!(owners.shortcut.numeric_args[3].to_bits(), 0.0_f64.to_bits());
                assert!(!owners.shortcut.numeric_field_blanks[3]);
                assert_eq!(owners.shortcut.numeric_args[11], 1.5);
                assert!(!owners.shortcut.numeric_field_blanks[11]);
            }
            result
        };
        assert_eq!(data.num_w5_gases, 3);
        assert!(!input.is_object_unused("WindowMaterial:Gas", "No mass"));
        assert_eq!(state.tracking.total_severe_errors, 1);
        if failure {
            assert!(outcome.is_err());
            assert!(
                !errors_found,
                "diagnostic failure precedes setting outer ErrorsFound"
            );
            assert_eq!(data.materials.len(), 3);
            assert!(data.window_gas.is_empty());
            assert!(input.is_object_unused("WindowMaterial:Gas", "X BASIC"));
            assert!(input.is_object_unused("WindowMaterial:Gas", "Z OPTIONAL"));
            assert_eq!(shortcut.current_module_object, "WindowMaterial:Gas");
        } else {
            outcome.expect("duplicate continues through real later Gas rows");
            assert!(errors_found);
            assert_eq!(data.materials.len(), 5);
            assert_eq!(data.window_gas.len(), 2);
            assert_eq!(
                data.nominal_r[3..],
                [0.0, 0.0],
                "shared ErrorsFound suppresses later nominal writes"
            );
            assert_eq!(data.window_gas[&MaterialId(3)].resistance, 0.0);
            assert_eq!(
                data.window_gas[&MaterialId(4)].gases[0]
                    .properties
                    .specific_heat_ratio,
                1.67
            );
            assert!(!input.is_object_unused("WindowMaterial:Gas", "X BASIC"));
            assert!(!input.is_object_unused("WindowMaterial:Gas", "Z OPTIONAL"));
            let message = "GetMaterialData: WindowMaterial:Gas = NO MASS, duplicate name.";
            let text = std::str::from_utf8(&text).unwrap();
            assert_eq!(text.matches(message).count(), 1);
            assert!(!text.contains(" **   ~~~   **"));
        }
    }
}
