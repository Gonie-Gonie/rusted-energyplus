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
        glazing: Vec::new(),
        refraction: Vec::new(),
        spectral_data: Vec::new(),
        window_gas: Vec::new(),
        window_gas_mixture: Vec::new(),
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
            curves: None,
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
        assert_eq!(row.gases[0].gas_type, Some(gas_type));
        assert_eq!(row.gases[0].properties.conductivity.coefficient_a, k0);
        assert_eq!(row.gases[0].properties.molecular_weight_g_per_mol, weight);
        assert!(
            row.gases[1..]
                .iter()
                .all(|gas| gas.gas_type == Some(ep_model::WindowGasType::Custom)
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
    // One additional genuine getter context, AFTER every Named/SGS assertion.
    // Two Custom rows cover omitted ratio0 and a legal raw-unsigned effective
    // negative ratio/Thickness; no actual runtime material row is seeded.
    let wrapped = ep_model::classify_source_json_number("18446744073709551615")
        .unwrap()
        .as_f64();
    let mut custom_ids = Vec::new();
    for (name, thickness, ratio) in [
        ("E CUSTOM", 0.012, None),
        ("F CUSTOM", wrapped, Some(wrapped)),
    ] {
        let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
        let mut advisory = ep_model::WindowGasType::Air.standard_properties().unwrap();
        advisory.conductivity.coefficient_a = 99.0;
        model.materials.push(Material {
            id,
            name: NormalizedName::new(name),
            definition: MaterialDefinition::WindowGas(ep_model::WindowGasMaterial {
                gas_type: ep_model::WindowGasType::Custom,
                thickness_m: if thickness < 0.0 {
                    "18446744073709551615".parse().unwrap()
                } else {
                    thickness
                },
                properties: advisory,
            }),
        });
        model.ctf_report_names.materials.push((id, name.into()));
        let mut fields = BTreeMap::from([
            ("gas_type".into(), Value::String("Custom".into())),
            ("thickness".into(), Value::Number(thickness)),
            ("conductivity_coefficient_a".into(), Value::Number(0.02)),
            ("viscosity_coefficient_a".into(), Value::Number(0.00001)),
            ("specific_heat_coefficient_a".into(), Value::Number(1000.0)),
            ("molecular_weight".into(), Value::Number(28.0)),
        ]);
        if name == "E CUSTOM" {
            fields.extend([
                ("conductivity_coefficient_b".into(), Value::Number(0.00001)),
                (
                    "conductivity_coefficient_c".into(),
                    Value::Number(0.00000001),
                ),
                (
                    "viscosity_coefficient_b".into(),
                    Value::Number(-0.000000002),
                ),
                (
                    "viscosity_coefficient_c".into(),
                    Value::Number(0.00000000001),
                ),
                ("specific_heat_coefficient_b".into(), Value::Number(0.1)),
                ("specific_heat_coefficient_c".into(), Value::Number(-0.0001)),
            ]);
        }
        if let Some(value) = ratio {
            fields.insert("specific_heat_ratio".into(), Value::Number(value));
        }
        raw.window_gas.push(ProjectInputObject {
            object_name: name.into(),
            idf_max_fields: None,
            fields,
        });
        custom_ids.push(id);
    }
    let (customs, custom_input, _) =
        get_materials(&model, &raw).expect("actual Custom getter, no property/denom error");
    assert_eq!(customs.num_w5_gases, 6);
    assert_eq!(customs.materials.len(), 10);
    for (offset, name) in ["E CUSTOM", "F CUSTOM"].into_iter().enumerate() {
        let id = custom_ids[offset];
        let row = &customs.window_gas[&id];
        assert_eq!(row.gases[0].gas_type, Some(ep_model::WindowGasType::Custom));
        assert_eq!(row.gases[0].properties.conductivity.coefficient_a, 0.02);
        assert_eq!(row.gases[0].properties.viscosity.coefficient_a, 0.00001);
        assert_eq!(row.gases[0].properties.specific_heat.coefficient_a, 1000.0);
        assert_eq!(row.gases[0].properties.molecular_weight_g_per_mol, 28.0);
        assert_eq!(
            [
                row.gases[0].properties.conductivity.coefficient_b,
                row.gases[0].properties.conductivity.coefficient_c,
                row.gases[0].properties.viscosity.coefficient_b,
                row.gases[0].properties.viscosity.coefficient_c,
                row.gases[0].properties.specific_heat.coefficient_b,
                row.gases[0].properties.specific_heat.coefficient_c
            ],
            if offset == 0 {
                [
                    0.00001,
                    0.00000001,
                    -0.000000002,
                    0.00000000001,
                    0.1,
                    -0.0001,
                ]
            } else {
                [0.0; 6]
            }
        );
        assert_eq!(
            row.gases[0].properties.specific_heat_ratio.to_bits(),
            if offset == 0 {
                0.0_f64.to_bits()
            } else {
                wrapped.to_bits()
            }
        );
        assert_eq!(
            row.thickness.to_bits(),
            if offset == 0 {
                0.012_f64.to_bits()
            } else {
                wrapped.to_bits()
            }
        );
        let number = customs.material_map[name];
        assert_eq!(number, 8 + i32::try_from(offset).unwrap());
        let nominal = customs.nominal_r[usize::try_from(number - 1).unwrap()];
        assert_eq!(nominal.is_sign_negative(), offset == 1);
        assert_ne!(nominal, 0.0);
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
        assert_eq!(row.num_gases, 1);
        assert_eq!(row.gas_fractions, [1.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(row.gap_vent_type, ep_model::WindowGapVentType::Sealed);
        assert!(
            model
                .materials
                .iter()
                .find(|row| row.id == id)
                .unwrap()
                .surface_properties()
                .is_none()
        );
        let (roughness, surface) = customs
            .source_window_gas_surface_properties(number, id)
            .unwrap();
        assert_eq!(Some(roughness), row.roughness);
        assert_eq!(
            [
                surface.thermal_absorptance,
                surface.solar_absorptance,
                surface.visible_absorptance
            ],
            [0.0; 3]
        );
        assert!(!custom_input.is_object_unused("WindowMaterial:Gas", name));
    }
    assert_eq!(customs.material_map["SIMPLE"], 10);
    let mut custom_eio = Vec::new();
    customs
        .write_material_reports(&model, &mut custom_eio)
        .unwrap();
    let text = std::str::from_utf8(&custom_eio).unwrap();
    for name in ["E CUSTOM", "F CUSTOM"] {
        let prefix = format!(" Material Details,{name},0.0000,MediumRough,");
        let at = text
            .find(&prefix)
            .expect("actual Custom baseR0 common report");
        assert!(at < text.find(" Material Details,SIMPLE,").unwrap());
        assert!(
            text[at..]
                .lines()
                .next()
                .unwrap()
                .ends_with(",0.000,0.000,0.000,0.0000,0.0000,0.0000")
        );
    }
    // One genuinely fresh Material/IP/backend context; old Named/Custom/SGS
    // assertions above have already run. Four real Mixture counts share it.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        for count in [1_usize, 2, 3, 4] {
            let name = format!("MIX {count}");
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            let types = [
                ep_model::WindowStandardGasType::Air,
                if count == 2 {
                    ep_model::WindowStandardGasType::Air
                } else {
                    ep_model::WindowStandardGasType::Argon
                },
                ep_model::WindowStandardGasType::Krypton,
                ep_model::WindowStandardGasType::Xenon,
            ];
            let mut components = types.map(|gas_type| ep_model::WindowGasMixtureComponent {
                gas_type,
                fraction: 0.0,
            });
            components[0].fraction = 0.4;
            components[1].fraction = 0.8;
            let active = match count {
                1 => ep_model::WindowGasMixture::One([components[0]]),
                2 => ep_model::WindowGasMixture::Two([components[0], components[1]]),
                3 => {
                    ep_model::WindowGasMixture::Three([components[0], components[1], components[2]])
                }
                _ => ep_model::WindowGasMixture::Four(components),
            };
            model.materials.push(Material {
                id,
                name: NormalizedName::new(&name),
                definition: MaterialDefinition::WindowGasMixture(
                    ep_model::WindowGasMixtureMaterial {
                        thickness_m: 99.0,
                        gases: active,
                    },
                ),
            });
            model.ctf_report_names.materials.push((id, name.clone()));
            let mut fields = BTreeMap::from([
                ("thickness".into(), Value::Number(0.012)),
                (
                    "number_of_gases_in_mixture".into(),
                    Value::Number(f64::from(u32::try_from(count).unwrap())),
                ),
                ("gas_1_type".into(), Value::String("Air".into())),
                ("gas_1_fraction".into(), Value::Number(0.4)),
                (
                    "gas_2_type".into(),
                    Value::String(types[1].energyplus_name().into()),
                ),
                ("gas_2_fraction".into(), Value::Number(0.8)),
            ]);
            if count >= 3 {
                fields.insert("gas_3_type".into(), Value::String("Krypton".into()));
            }
            if count == 4 {
                fields.insert("gas_4_type".into(), Value::String("Xenon".into()));
            }
            if count == 1 {
                fields.insert("gas_4_fraction".into(), Value::Number(0.6));
            }
            raw.window_gas_mixture.push(ProjectInputObject {
                object_name: name,
                fields,
                idf_max_fields: None,
            });
        }
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:GasMixture", &raw.window_gas_mixture, 5, 6),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        {
            let stream: &mut dyn std::io::Write = &mut text;
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
                curves: None,
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

            data.get_material_data(&raw, &mut owners).unwrap();
            // Same real IP/scratch/owners, no new material context or latch reset.
            owners.shortcut.alpha_args.fill("stale".into());
            owners.shortcut.numeric_args.fill(-123.0);
            assert_eq!(
                raw.get_window_gas_mixture_item(&raw.window_gas_mixture[0], &mut owners)
                    .unwrap(),
                (5, 6)
            );
            assert!(owners.shortcut.alpha_field_blanks[3] && owners.shortcut.alpha_field_blanks[4]);
            assert_eq!(owners.shortcut.numeric_args[5], 0.6);
            assert!(owners.shortcut.numeric_field_blanks[4]);
            assert_eq!(
                raw.get_window_gas_mixture_item(&raw.window_gas_mixture[1], &mut owners)
                    .unwrap(),
                (3, 4)
            );
            assert!(owners.shortcut.alpha_args[3..].iter().all(String::is_empty));
            assert!(
                owners.shortcut.numeric_args[4..]
                    .iter()
                    .all(|value| *value == 0.0)
            );
            assert_eq!(
                &owners.shortcut.numeric_field_names[..6],
                [
                    "thickness",
                    "number_of_gases_in_mixture",
                    "gas_1_fraction",
                    "gas_2_fraction",
                    "gas_3_fraction",
                    "gas_4_fraction"
                ]
            );
        }
        assert!(!errors_found && text.is_empty());
        assert_eq!((data.num_w5_gas_mixtures, data.materials.len()), (4, 7));
        for (offset, count) in [1_usize, 2, 3, 4].into_iter().enumerate() {
            let id = MaterialId(u32::try_from(3 + offset).unwrap());
            let number = i32::try_from(4 + offset).unwrap();
            let name = format!("MIX {count}");
            let row = &data.window_gas[&id];
            assert_eq!(data.material_map[&name], number);
            assert_eq!(data.material_groups[&id], SourceMaterialGroup::GasMixture);
            assert!(data.resistance_only[&id]);
            assert_eq!(row.num_gases, i32::try_from(count).unwrap());
            assert_eq!(row.thickness, 0.012);
            assert_eq!(
                row.gas_fractions,
                if count == 1 {
                    [0.4, 0.0, 0.0, 0.0, 0.0]
                } else {
                    [0.4, 0.8, 0.0, 0.0, 0.0]
                }
            );
            assert_eq!(row.gap_vent_type, ep_model::WindowGapVentType::Sealed);
            for (index, gas) in row.gases.iter().enumerate() {
                if index < count {
                    let expected = if index == 0 || count == 2 {
                        ep_model::WindowGasType::Air
                    } else if index == 1 {
                        ep_model::WindowGasType::Argon
                    } else if index == 2 {
                        ep_model::WindowGasType::Krypton
                    } else {
                        ep_model::WindowGasType::Xenon
                    };
                    assert_eq!(gas.gas_type, Some(expected));
                    let actual_named = gases
                        .window_gas
                        .values()
                        .find(|row| row.gases[0].gas_type == Some(expected))
                        .unwrap();
                    assert_eq!(gas.properties, actual_named.gases[0].properties);
                } else {
                    assert_eq!(gas.gas_type, None);
                    let p = gas.properties;
                    assert_eq!(
                        [
                            p.conductivity.coefficient_a,
                            p.conductivity.coefficient_b,
                            p.conductivity.coefficient_c,
                            p.viscosity.coefficient_a,
                            p.viscosity.coefficient_b,
                            p.viscosity.coefficient_c,
                            p.specific_heat.coefficient_a,
                            p.specific_heat.coefficient_b,
                            p.specific_heat.coefficient_c,
                            p.molecular_weight_g_per_mol,
                            p.specific_heat_ratio
                        ],
                        [0.0; 11]
                    );
                }
            }
            assert_eq!(
                data.nominal_r[3 + offset].to_bits(),
                gases.nominal_r[3].to_bits(),
                "same real first-Air owner, fractions do not enter NominalR"
            );
            assert_eq!(
                [
                    row.resistance,
                    row.conductivity,
                    row.density,
                    row.specific_heat
                ],
                [0.0; 4]
            );
            let (roughness, surface) = data
                .source_window_gas_surface_properties(number, id)
                .unwrap();
            assert_eq!(roughness, MaterialSurfaceRoughness::MediumRough);
            assert_eq!(
                [
                    surface.thermal_absorptance,
                    surface.solar_absorptance,
                    surface.visible_absorptance
                ],
                [0.0; 3]
            );
            assert!(!input.is_object_unused("WindowMaterial:GasMixture", &name));
        }
        let mut report = Vec::new();
        data.write_material_reports(&model, &mut report).unwrap();
        assert_eq!(
            std::str::from_utf8(&report)
                .unwrap()
                .matches(" Material Details,MIX ")
                .count(),
            4
        );
    }

    // Exactly one fresh ordinary MaterialGlass context; old SGS/Gas/Mixture
    // assertions above remain on their original owners and have already run.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        let glass = MaterialDefinition::WindowGlazingSpectralAverage(
            ep_model::WindowGlazingSpectralAverageMaterial {
                thickness_m: 0.003,
                solar_transmittance_at_normal_incidence: 0.0,
                front_side_solar_reflectance_at_normal_incidence: 0.0,
                back_side_solar_reflectance_at_normal_incidence: 0.0,
                visible_transmittance_at_normal_incidence: 0.0,
                front_side_visible_reflectance_at_normal_incidence: 0.0,
                back_side_visible_reflectance_at_normal_incidence: 0.0,
                infrared_transmittance_at_normal_incidence: 0.0,
                front_side_infrared_hemispherical_emissivity: 0.84,
                back_side_infrared_hemispherical_emissivity: 0.84,
                conductivity_w_per_m_k: 0.9,
                dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
                solar_diffusing: false,
                youngs_modulus_pa: 72_000_000_000.0,
                poissons_ratio: 0.22,
            },
        );
        for (name, fields) in [
            ("A DEFAULT", BTreeMap::from([
                ("optical_data_type".into(), Value::String("SpectralAverage".into())),
                ("thickness".into(), Value::Number(0.003)),
            ])),
            ("B EXPLICIT", BTreeMap::from([
                ("optical_data_type".into(), Value::String("SpectralAverage".into())),
                ("thickness".into(), Value::Number(0.003)),
                ("solar_transmittance_at_normal_incidence".into(), Value::Number(0.11)),
                ("front_side_solar_reflectance_at_normal_incidence".into(), Value::Number(0.12)),
                ("back_side_solar_reflectance_at_normal_incidence".into(), Value::Number(0.13)),
                ("visible_transmittance_at_normal_incidence".into(), Value::Number(0.21)),
                ("front_side_visible_reflectance_at_normal_incidence".into(), Value::Number(0.22)),
                ("back_side_visible_reflectance_at_normal_incidence".into(), Value::Number(0.23)),
                ("infrared_transmittance_at_normal_incidence".into(), Value::Number(0.01)),
                ("front_side_infrared_hemispherical_emissivity".into(), Value::Number(0.8)),
                ("back_side_infrared_hemispherical_emissivity".into(), Value::Number(0.7)),
                ("conductivity".into(), Value::Number(0.9)),
                ("dirt_correction_factor_for_solar_and_visible_transmittance".into(), Value::Number(0.95)),
                ("solar_diffusing".into(), Value::String("Yes".into())),
                ("young_s_modulus".into(), Value::Number(12_000_000_000.0)),
                ("poisson_s_ratio".into(), Value::Number(0.2)),
                ("window_glass_spectral_data_set_name".into(), Value::String("not resolved for Average".into())),
                ("window_glass_spectral_and_incident_angle_transmittance_data_set_table_name".into(), Value::String("".into())),
                ("window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name".into(), Value::String(" ".into())),
                ("window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name".into(), Value::String("inactive table".into())),
            ])),
        ] {
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material { id, name: NormalizedName::new(name), definition: glass });
            model.ctf_report_names.materials.push((id, name.into()));
            raw.glazing.push(ProjectInputObject { object_name: name.into(), fields, idf_max_fields: None });
        }
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        let args = input
            .get_object_def_max_args("WindowMaterial:Glazing")
            .unwrap();
        assert_eq!(
            (args.alphas, args.numerics, args.total().unwrap()),
            (7, 14, 21)
        );
        {
            let stream: &mut dyn std::io::Write = &mut text;
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
                curves: None,
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

            data.get_material_data(&raw, &mut owners).unwrap();
            // Observe the same real IP/scratch after actual GetMaterialData.
            owners.shortcut.alpha_args.fill("stale".into());
            owners.shortcut.numeric_args.fill(-123.0);
            owners.shortcut.alpha_field_names.fill("stale".into());
            owners.shortcut.numeric_field_names.fill("stale".into());
            assert_eq!(
                raw.get_glazing_item(&raw.glazing[1], &mut owners).unwrap(),
                (7, 14)
            );
            assert_eq!(
                &owners.shortcut.alpha_field_names[..7],
                [
                    "name",
                    "optical_data_type",
                    "window_glass_spectral_data_set_name",
                    "solar_diffusing",
                    "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name",
                    "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name",
                    "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name"
                ]
            );
            assert_eq!(
                &owners.shortcut.numeric_field_names[..14],
                [
                    "thickness",
                    "solar_transmittance_at_normal_incidence",
                    "front_side_solar_reflectance_at_normal_incidence",
                    "back_side_solar_reflectance_at_normal_incidence",
                    "visible_transmittance_at_normal_incidence",
                    "front_side_visible_reflectance_at_normal_incidence",
                    "back_side_visible_reflectance_at_normal_incidence",
                    "infrared_transmittance_at_normal_incidence",
                    "front_side_infrared_hemispherical_emissivity",
                    "back_side_infrared_hemispherical_emissivity",
                    "conductivity",
                    "dirt_correction_factor_for_solar_and_visible_transmittance",
                    "young_s_modulus",
                    "poisson_s_ratio"
                ]
            );
            assert_eq!(
                raw.get_glazing_item(&raw.glazing[0], &mut owners).unwrap(),
                (3, 11)
            );
            assert_eq!(
                &owners.shortcut.alpha_args[..7],
                ["A DEFAULT", "SPECTRALAVERAGE", "", "NO", "", "", ""]
            );
            assert_eq!(
                &owners.shortcut.alpha_field_blanks[..7],
                [false, false, true, true, true, true, true]
            );
            assert_eq!(
                &owners.shortcut.numeric_args[..14],
                [
                    0.003,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.84,
                    0.84,
                    0.9,
                    1.0,
                    72_000_000_000.0,
                    0.22
                ]
            );
            assert!(
                !owners.shortcut.numeric_field_blanks[0]
                    && owners.shortcut.numeric_field_blanks[1..14]
                        .iter()
                        .all(|blank| *blank)
            );
        }
        let (default_id, explicit_id) = (MaterialId(3), MaterialId(4));
        let (a, b) = (
            &data.simple_glazing[&default_id],
            &data.simple_glazing[&explicit_id],
        );
        assert_eq!(
            (
                data.num_w5_glazings,
                data.materials.len(),
                data.material_map["A DEFAULT"],
                data.material_map["B EXPLICIT"]
            ),
            (2, 5, 4, 5)
        );
        assert!(
            data.material_groups[&default_id] == SourceMaterialGroup::Glass
                && data.material_groups[&explicit_id] == SourceMaterialGroup::Glass
                && data.resistance_only[&default_id]
                && data.resistance_only[&explicit_id]
        );
        assert_eq!(
            [
                b.thickness,
                b.transmittance,
                b.solar_reflectance_front,
                b.solar_reflectance_back,
                b.visible_transmittance,
                b.visible_reflectance_front,
                b.visible_reflectance_back,
                b.thermal_transmittance,
                b.thermal_front,
                b.thermal_back,
                b.conductivity,
                b.dirt_factor,
                b.young_modulus,
                b.poissons_ratio
            ],
            [
                0.003,
                0.11,
                0.12,
                0.13,
                0.21,
                0.22,
                0.23,
                0.01,
                0.8,
                0.7,
                0.9,
                0.95,
                12_000_000_000.0,
                0.2
            ]
        );
        assert_eq!(
            [
                a.transmittance,
                a.visible_transmittance,
                a.thermal_back,
                a.conductivity,
                a.dirt_factor,
                a.young_modulus,
                a.poissons_ratio
            ],
            [0.0, 0.0, 0.84, 0.9, 1.0, 72_000_000_000.0, 0.22]
        );
        assert!(
            b.solar_diffusing
                && !a.solar_diffusing
                && b.thermal_absorptance == b.thermal_back
                && b.spectral_data_pointer == 0
        );
        assert_eq!(
            [
                b.density,
                b.specific_heat,
                b.solar_absorptance,
                b.visible_absorptance,
                b.u_factor,
                b.shgc,
                b.visible_input
            ],
            [0.0; 7]
        );
        assert!(
            !b.visible_input_by_user
                && a.resistance > 0.0
                && a.resistance.to_bits() == b.resistance.to_bits()
                && data.nominal_r[3].to_bits() == a.resistance.to_bits()
                && data.nominal_r[4].to_bits() == b.resistance.to_bits()
        );
        assert!(
            model.materials[4].surface_properties().is_none()
                && data
                    .source_simple_glazing_surface_properties(5, explicit_id)
                    .is_err()
        );
        assert_eq!(
            data.source_glazing_surface_properties(5, explicit_id)
                .unwrap(),
            (
                MaterialSurfaceRoughness::VerySmooth,
                OpaqueSurfaceProperties {
                    thermal_absorptance: 0.7,
                    solar_absorptance: 0.0,
                    visible_absorptance: 0.0
                }
            )
        );
        assert!(
            data.source_glazing_surface_properties(4, explicit_id)
                .is_err()
        );
        data.write_material_reports(&model, &mut eio).unwrap();
        let report = std::str::from_utf8(&eio).unwrap();
        let fields: Vec<_> = report
            .lines()
            .find(|line| line.starts_with(" Material Details,B EXPLICIT,"))
            .unwrap()
            .split(',')
            .collect();
        assert!(
            fields[3] == "VerySmooth"
                && fields[4].parse::<f64>().unwrap() == b.thickness
                && fields[5].parse::<f64>().unwrap() == b.conductivity
                && fields[8].parse::<f64>().unwrap() == b.thermal_back
                && fields[9] == "0.0000"
                && fields[10] == "0.0000"
        );
        assert!(
            !errors_found
                && text.is_empty()
                && !input.is_object_unused("WindowMaterial:Glazing", "A DEFAULT")
                && !input.is_object_unused("WindowMaterial:Glazing", "B EXPLICIT")
        );
    }

    // One fresh actual alternative-Glass owner; all old owner assertions above
    // remain on their original contexts. No SGS Setup/typed optical calculation.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        let inputs = ep_model::WindowGlazingRefractionExtinctionMaterial {
            thickness_m: 0.003,
            solar_index_of_refraction: 1.5,
            solar_extinction_coefficient_per_m: 20.0,
            visible_index_of_refraction: 1.7,
            visible_extinction_coefficient_per_m: 100.0,
            infrared_transmittance_at_normal_incidence: 0.0,
            infrared_hemispherical_emissivity: 0.84,
            conductivity_w_per_m_k: 0.9,
            dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
            solar_diffusing: false,
        };
        let original = "18446744073709551615".parse::<f64>().unwrap();
        let wrapped = ep_model::classify_source_json_number("18446744073709551615")
            .unwrap()
            .as_f64();
        for (name, fields, typed) in [
            (
                "A DEFAULT",
                BTreeMap::from([
                    ("thickness".into(), Value::Number(0.003)),
                    ("solar_index_of_refraction".into(), Value::Number(1.5)),
                    ("solar_extinction_coefficient".into(), Value::Number(20.0)),
                    ("visible_index_of_refraction".into(), Value::Number(1.7)),
                    (
                        "visible_extinction_coefficient".into(),
                        Value::Number(100.0),
                    ),
                ]),
                inputs,
            ),
            (
                "B EXPLICIT",
                BTreeMap::from([
                    ("thickness".into(), Value::Number(0.003)),
                    ("solar_index_of_refraction".into(), Value::Number(1.5)),
                    ("solar_extinction_coefficient".into(), Value::Number(20.0)),
                    ("visible_index_of_refraction".into(), Value::Number(1.7)),
                    (
                        "visible_extinction_coefficient".into(),
                        Value::Number(100.0),
                    ),
                    (
                        "infrared_transmittance_at_normal_incidence".into(),
                        Value::Number(0.01),
                    ),
                    (
                        "infrared_hemispherical_emissivity".into(),
                        Value::Number(0.7),
                    ),
                    ("conductivity".into(), Value::Number(0.9)),
                    (
                        "dirt_correction_factor_for_solar_and_visible_transmittance".into(),
                        Value::Number(0.95),
                    ),
                    ("solar_diffusing".into(), Value::String("Yes".into())),
                ]),
                ep_model::WindowGlazingRefractionExtinctionMaterial {
                    infrared_transmittance_at_normal_incidence: 0.01,
                    infrared_hemispherical_emissivity: 0.7,
                    dirt_correction_factor_for_solar_and_visible_transmittance: 0.95,
                    solar_diffusing: true,
                    ..inputs
                },
            ),
            (
                "C WRAPPED",
                BTreeMap::from([
                    ("thickness".into(), Value::Number(wrapped)),
                    ("solar_index_of_refraction".into(), Value::Number(wrapped)),
                    (
                        "solar_extinction_coefficient".into(),
                        Value::Number(wrapped),
                    ),
                    ("visible_index_of_refraction".into(), Value::Number(wrapped)),
                    (
                        "visible_extinction_coefficient".into(),
                        Value::Number(wrapped),
                    ),
                    ("conductivity".into(), Value::Number(wrapped)),
                ]),
                ep_model::WindowGlazingRefractionExtinctionMaterial {
                    thickness_m: original,
                    solar_index_of_refraction: original,
                    solar_extinction_coefficient_per_m: original,
                    visible_index_of_refraction: original,
                    visible_extinction_coefficient_per_m: original,
                    conductivity_w_per_m_k: original,
                    ..inputs
                },
            ),
        ] {
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material {
                id,
                name: NormalizedName::new(name),
                definition: MaterialDefinition::WindowGlazingRefractionExtinction(typed),
            });
            model.ctf_report_names.materials.push((id, name.into()));
            raw.refraction.push(ProjectInputObject {
                object_name: name.into(),
                fields,
                idf_max_fields: None,
            });
        }
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            (
                "WindowMaterial:Glazing:RefractionExtinctionMethod",
                &raw.refraction,
                2,
                9,
            ),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        let args = input
            .get_object_def_max_args("WindowMaterial:Glazing:RefractionExtinctionMethod")
            .unwrap();
        assert_eq!(
            (args.alphas, args.numerics, args.total().unwrap()),
            (2, 9, 11)
        );
        {
            let stream: &mut dyn std::io::Write = &mut text;
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
                curves: None,
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

            data.get_material_data(&raw, &mut owners).unwrap();
            // The only direct primitive probe uses these same actual owners.
            owners.shortcut.alpha_args.fill("stale".into());
            owners.shortcut.numeric_args.fill(-123.0);
            owners.shortcut.alpha_field_names.fill("stale".into());
            owners.shortcut.numeric_field_names.fill("stale".into());
            assert_eq!(
                raw.get_refraction_item(&raw.refraction[1], &mut owners)
                    .unwrap(),
                (2, 9)
            );
            assert_eq!(
                &owners.shortcut.alpha_field_names[..2],
                ["name", "solar_diffusing"]
            );
            assert_eq!(
                &owners.shortcut.numeric_field_names[..9],
                [
                    "thickness",
                    "solar_index_of_refraction",
                    "solar_extinction_coefficient",
                    "visible_index_of_refraction",
                    "visible_extinction_coefficient",
                    "infrared_transmittance_at_normal_incidence",
                    "infrared_hemispherical_emissivity",
                    "conductivity",
                    "dirt_correction_factor_for_solar_and_visible_transmittance"
                ]
            );
            assert_eq!(
                raw.get_refraction_item(&raw.refraction[0], &mut owners)
                    .unwrap(),
                (1, 5)
            );
            assert!(
                owners.shortcut.alpha_args[..2]
                    .iter()
                    .map(String::as_str)
                    .eq(["A DEFAULT", "NO"])
                    && owners.shortcut.alpha_field_blanks[..2] == [false, true]
            );
            assert_eq!(
                &owners.shortcut.numeric_args[..9],
                [0.003, 1.5, 20.0, 1.7, 100.0, 0.0, 0.84, 0.9, 1.0]
            );
            assert!(
                owners.shortcut.numeric_field_blanks[..5]
                    .iter()
                    .all(|blank| !*blank)
                    && owners.shortcut.numeric_field_blanks[5..9]
                        .iter()
                        .all(|blank| *blank)
            );
        }
        let (a, b, c) = (
            &data.simple_glazing[&MaterialId(3)],
            &data.simple_glazing[&MaterialId(4)],
            &data.simple_glazing[&MaterialId(5)],
        );
        assert_eq!(
            (
                data.num_w5_alt_glazings,
                data.materials.len(),
                data.material_map["A DEFAULT"],
                data.material_map["C WRAPPED"]
            ),
            (3, 6, 4, 6)
        );
        assert!(
            data.material_groups[&MaterialId(3)] == SourceMaterialGroup::Glass
                && data.material_groups[&MaterialId(4)] == SourceMaterialGroup::Glass
                && data.resistance_only[&MaterialId(3)]
                && data.resistance_only[&MaterialId(5)]
        );
        assert_eq!(
            [
                a.thermal_transmittance,
                a.thermal_front,
                a.thermal_back,
                a.conductivity,
                a.dirt_factor
            ],
            [0.0, 0.84, 0.84, 0.9, 1.0]
        );
        assert_eq!(
            [
                b.thickness,
                b.thermal_transmittance,
                b.thermal_front,
                b.thermal_back,
                b.conductivity,
                b.dirt_factor
            ],
            [0.003, 0.01, 0.7, 0.7, 0.9, 0.95]
        );
        assert!(
            a.transmittance > 0.0
                && a.transmittance < 1.0
                && a.transmittance.to_bits() == b.transmittance.to_bits()
                && a.visible_transmittance.to_bits() == b.visible_transmittance.to_bits()
        );
        assert!(
            b.solar_reflectance_back.to_bits() == b.solar_reflectance_front.to_bits()
                && b.visible_reflectance_back.to_bits() == b.solar_reflectance_front.to_bits()
                && b.visible_reflectance_front.to_bits() != b.solar_reflectance_front.to_bits()
        );
        assert_eq!(
            [
                b.density,
                b.specific_heat,
                b.solar_absorptance,
                b.visible_absorptance,
                b.young_modulus,
                b.poissons_ratio,
                b.u_factor,
                b.shgc,
                b.visible_input
            ],
            [0.0; 9]
        );
        assert!(
            b.solar_diffusing
                && !a.solar_diffusing
                && !b.visible_input_by_user
                && b.thermal_absorptance == b.thermal_back
                && b.spectral_data_pointer == 0
                && b.optical_model
                    == super::simple_glazing::SourceSimpleGlazingOpticalModel::SpectralAverage
        );
        assert!(
            a.resistance > 0.0
                && a.resistance.to_bits() == b.resistance.to_bits()
                && a.resistance.to_bits() == data.nominal_r[3].to_bits()
                && b.resistance.to_bits() == data.nominal_r[4].to_bits()
        );
        assert_eq!(
            [c.thickness, c.conductivity, c.resistance, data.nominal_r[5]],
            [-1.0, -1.0, 0.0, 0.0]
        );
        assert!(c.transmittance.is_nan() && c.visible_transmittance.is_nan());
        assert!(
            model.materials[4].surface_properties().is_none()
                && data
                    .source_simple_glazing_surface_properties(5, MaterialId(4))
                    .is_err()
                && data
                    .source_glazing_surface_properties(4, MaterialId(4))
                    .is_err()
        );
        assert_eq!(
            data.source_glazing_surface_properties(5, MaterialId(4))
                .unwrap(),
            (
                MaterialSurfaceRoughness::VerySmooth,
                OpaqueSurfaceProperties {
                    thermal_absorptance: 0.7,
                    solar_absorptance: 0.0,
                    visible_absorptance: 0.0
                }
            )
        );
        data.write_material_reports(&model, &mut eio).unwrap();
        let fields: Vec<_> = std::str::from_utf8(&eio)
            .unwrap()
            .lines()
            .find(|line| line.starts_with(" Material Details,B EXPLICIT,"))
            .unwrap()
            .split(',')
            .collect();
        assert!(
            fields[3] == "VerySmooth"
                && fields[4].parse::<f64>().unwrap() == b.thickness
                && fields[5].parse::<f64>().unwrap() == b.conductivity
                && fields[8].parse::<f64>().unwrap() == b.thermal_back
                && fields[9] == "0.0000"
                && fields[10] == "0.0000"
        );
        assert!(
            !errors_found
                && state.tracking.total_severe_errors == 0
                && text.is_empty()
                && ["A DEFAULT", "B EXPLICIT", "C WRAPPED"]
                    .iter()
                    .all(|name| !input.is_object_unused(
                        "WindowMaterial:Glazing:RefractionExtinctionMethod",
                        name
                    ))
        );
    }
    // One fresh Spectral context: actual HBM266 arrays precede Material268.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        let fixed_names = [
            "wavelength_1",
            "transmittance_1",
            "front_reflectance_1",
            "back_reflectance_1",
            "wavelength_2",
            "transmittance_2",
            "front_reflectance_2",
            "back_reflectance_2",
            "wavelength_3",
            "transmittance_3",
            "front_reflectance_3",
            "back_reflectance_3",
            "wavelength_4",
            "transmittance_4",
            "front_reflectance_4",
            "back_reflectance_4",
        ];
        let wrapped = ep_model::classify_source_json_number("18446744073709551615")
            .unwrap()
            .as_f64();
        let fixed: BTreeMap<String, Value> = fixed_names
            .iter()
            .zip([
                0.1, 0.2, 0.3, 0.4, 0.2, 0.2, 0.3, 0.4, 0.3, 0.2, 0.3, 0.4, 0.4, 0.2, 0.3, 0.4,
            ])
            .map(|(name, value)| ((*name).into(), Value::Number(value)))
            .collect();
        let extensions: Vec<BTreeMap<String, Value>> = (4..800)
            .map(|index| {
                BTreeMap::from([
                    (
                        "wavelength".into(),
                        Value::Number(0.1 + f64::from(index) * 0.004),
                    ),
                    ("transmittance".into(), Value::Number(0.2)),
                    ("front_reflectance".into(), Value::Number(0.3)),
                    ("back_reflectance".into(), Value::Number(0.4)),
                ])
            })
            .collect();
        let mut bulk_fixed = fixed.clone();
        bulk_fixed.insert("wavelength_2".into(), Value::Number(0.104));
        bulk_fixed.insert("wavelength_3".into(), Value::Number(0.108));
        bulk_fixed.insert("wavelength_4".into(), Value::Number(0.112));
        raw.spectral_data = [
            ("A ZERO", BTreeMap::new(), Vec::new()),
            (
                "B FIXED EMPTY EXT",
                fixed_names[..4]
                    .iter()
                    .zip([0.5, 0.4, 0.2, 0.3])
                    .map(|(name, value)| ((*name).into(), Value::Number(value)))
                    .collect(),
                Vec::new(),
            ),
            (
                "C Shared",
                fixed_names[..4]
                    .iter()
                    .zip([0.5, wrapped, 0.2, 0.3])
                    .map(|(name, value)| ((*name).into(), Value::Number(value)))
                    .collect(),
                Vec::new(),
            ),
            (
                "c shared",
                fixed_names[..4]
                    .iter()
                    .zip([0.6, 0.7, 0.1, 0.1])
                    .map(|(name, value)| ((*name).into(), Value::Number(value)))
                    .collect(),
                Vec::new(),
            ),
            ("Z EIGHT HUNDRED", bulk_fixed, extensions),
        ]
        .map(|(name, fields, extensions)| SourceGlazingSpectralInput {
            object: ProjectInputObject {
                object_name: name.into(),
                fields,
                idf_max_fields: None,
            },
            extensions,
        })
        .into();

        for (name, reference) in [("D LINK FIRST", "c ShArEd"), ("E LINK ZERO", "a zero")] {
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material {
                id,
                name: NormalizedName::new(name),
                definition: MaterialDefinition::WindowGlazingSpectral(
                    ep_model::WindowGlazingSpectralMaterial {
                        thickness_m: 0.003,
                        solar_transmittance_at_normal_incidence: 0.0,
                        front_side_solar_reflectance_at_normal_incidence: 0.0,
                        back_side_solar_reflectance_at_normal_incidence: 0.0,
                        visible_transmittance_at_normal_incidence: 0.0,
                        front_side_visible_reflectance_at_normal_incidence: 0.0,
                        back_side_visible_reflectance_at_normal_incidence: 0.0,
                        infrared_transmittance_at_normal_incidence: 0.0,
                        front_side_infrared_hemispherical_emissivity: 0.84,
                        back_side_infrared_hemispherical_emissivity: 0.84,
                        conductivity_w_per_m_k: 0.9,
                        dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
                        solar_diffusing: false,
                        youngs_modulus_pa: 72_000_000_000.0,
                        poissons_ratio: 0.22,
                        spectral_data: None,
                    },
                ),
            });
            model.ctf_report_names.materials.push((id, name.into()));
            raw.glazing.push(ProjectInputObject {
                object_name: name.into(),
                idf_max_fields: None,
                fields: BTreeMap::from([
                    ("optical_data_type".into(), Value::String("Spectral".into())),
                    (
                        "window_glass_spectral_data_set_name".into(),
                        Value::String(reference.into()),
                    ),
                    ("thickness".into(), Value::Number(0.003)),
                ]),
            });
        }

        // Every context owns real family definitions, every extension position,
        // input unused accounting and the same shared scratch/backend/Material owner.
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let mut definitions: Vec<_> = families
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
        definitions.push(SourceObjectDefinition {
            object_type: "MaterialProperty:GlazingSpectralData".into(),
            alpha_fields: 1,
            numeric_fields: 16,
            alpha_extensions: 0,
            numeric_extensions: 4,
            extension: Some("extensions".into()),
        });
        let mut shapes: BTreeMap<_, _> = families
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
                                    .map(|key| (key.clone(), 1))
                                    .collect(),
                            })
                            .collect(),
                    ),
                )
            })
            .collect();
        shapes.insert(
            "MaterialProperty:GlazingSpectralData".into(),
            SourceInputFamily::Objects(
                raw.spectral_data
                    .iter()
                    .map(|row| {
                        let mut sizes: BTreeMap<_, _> = row
                            .object
                            .fields
                            .keys()
                            .map(|key| (key.clone(), 1))
                            .collect();
                        if !row.extensions.is_empty()
                            || row.object.object_name == "B FIXED EMPTY EXT"
                        {
                            sizes.insert("extensions".into(), row.extensions.len());
                        }
                        SourceObjectShape {
                            object_name: row.object.object_name.clone(),
                            field_sizes: sizes,
                        }
                    })
                    .collect(),
            ),
        );
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        assert!(data.spectral_data.is_none());
        let args = input
            .get_object_def_max_args("MaterialProperty:GlazingSpectralData")
            .unwrap();
        assert_eq!(
            (args.alphas, args.numerics, args.total().unwrap()),
            (1, 3200, 3201)
        );
        {
            let stream: &mut dyn std::io::Write = &mut text;

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
                curves: None,
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

            for (index, expected) in [(0, 0), (1, 4), (4, 3200)] {
                owners.shortcut.alpha_args.fill("stale".into());
                owners.shortcut.numeric_args.fill(-123.0);
                owners.shortcut.alpha_field_names.fill("stale".into());
                owners.shortcut.numeric_field_names.fill("stale".into());
                owners.shortcut.alpha_field_blanks.fill(false);
                owners.shortcut.numeric_field_blanks.fill(false);
                assert_eq!(
                    raw.get_spectral_data_item(&raw.spectral_data[index], &mut owners)
                        .unwrap(),
                    (1, expected)
                );
                assert!(
                    owners.shortcut.alpha_args[0]
                        == raw.spectral_data[index]
                            .object
                            .object_name
                            .to_ascii_uppercase()
                        && owners.shortcut.alpha_field_names[0] == "name"
                        && !owners.shortcut.alpha_field_blanks[0]
                        && owners.shortcut.alpha_args[1..].iter().all(String::is_empty)
                        && owners.shortcut.alpha_field_names[1..]
                            .iter()
                            .all(String::is_empty)
                        && owners.shortcut.alpha_field_blanks[1..]
                            .iter()
                            .all(|value| *value)
                );
                assert_eq!(&owners.shortcut.numeric_field_names[..16], &fixed_names);
                assert!(
                    owners.shortcut.numeric_args[expected..]
                        .iter()
                        .all(|value| *value == 0.0)
                        && owners.shortcut.numeric_field_blanks[expected..]
                            .iter()
                            .all(|value| *value)
                        && owners.shortcut.numeric_field_names
                            [(16 + raw.spectral_data[index].extensions.len() * 4)..]
                            .iter()
                            .all(String::is_empty)
                );
            }
            data.get_glass_spectral_data(&raw, &mut owners).unwrap();
            data.get_material_data(&raw, &mut owners).unwrap();
        }
        let rows = data.spectral_data.as_ref().unwrap();
        assert_eq!((data.num_spectral_data, rows.len()), (5, 5));
        assert!(
            rows[0].num_of_wavelengths == 0
                && [
                    &rows[0].wavelengths,
                    &rows[0].transmittance,
                    &rows[0].front_reflectance,
                    &rows[0].back_reflectance
                ]
                .iter()
                .all(|array| array.as_ref().is_some_and(|array| array.is_empty()))
        );
        assert_eq!(
            (
                rows[1].num_of_wavelengths,
                rows[2].name.as_str(),
                rows[3].name.as_str()
            ),
            (1, "C SHARED", "C SHARED")
        );
        assert_eq!(
            (
                wrapped,
                rows[2].transmittance.as_ref().unwrap()[0],
                rows[3].transmittance.as_ref().unwrap()[0]
            ),
            (-1.0, 0.001, 0.7)
        );
        assert!(
            rows[4].num_of_wavelengths == 800
                && [
                    &rows[4].wavelengths,
                    &rows[4].transmittance,
                    &rows[4].front_reflectance,
                    &rows[4].back_reflectance
                ]
                .iter()
                .all(|array| array.as_ref().unwrap().len() == 800)
        );
        assert_eq!(
            [
                rows[4].transmittance.as_ref().unwrap()[799],
                rows[4].front_reflectance.as_ref().unwrap()[799],
                rows[4].back_reflectance.as_ref().unwrap()[799]
            ],
            [0.2, 0.3, 0.4]
        );
        let first = &data.simple_glazing[&MaterialId(3)];
        let zero = &data.simple_glazing[&MaterialId(4)];
        assert_eq!(
            (first.spectral_data_pointer, zero.spectral_data_pointer),
            (3, 1)
        );
        assert!(
            first.optical_model == super::simple_glazing::SourceSimpleGlazingOpticalModel::Spectral
                && first.thickness == 0.003
                && first.transmittance == 0.0
                && first.resistance > 0.0
                && first.resistance.to_bits() == data.nominal_r[3].to_bits()
        );
        assert_eq!(
            data.source_glazing_surface_properties(4, MaterialId(3))
                .unwrap(),
            (
                MaterialSurfaceRoughness::VerySmooth,
                OpaqueSurfaceProperties {
                    thermal_absorptance: first.thermal_back,
                    solar_absorptance: 0.0,
                    visible_absorptance: 0.0
                }
            )
        );
        assert!(
            data.source_glazing_surface_properties(5, MaterialId(3))
                .is_err()
        );
        data.write_material_reports(&model, &mut eio).unwrap();
        assert!(
            std::str::from_utf8(&eio)
                .unwrap()
                .contains(" Material Details,D LINK FIRST,")
                && data.material_groups[&MaterialId(3)] == SourceMaterialGroup::Glass
                && !errors_found
                && text.is_empty()
                && raw.spectral_data.iter().all(|row| !input.is_object_unused(
                    "MaterialProperty:GlazingSpectralData",
                    &row.object.object_name
                ))
        );
    }
    // One fresh genuine Curve/IP/backend owner; no Material helper or latch is reset.
    // Local14A/10000N and returned NumA/NumN/IOStatus remain static fidelity only.
    {
        use crate::heat_balance::selected_input_processor::{
            SourceInputProcessorShortCut, SourceInputStatisticsInput,
            SourceInputStatisticsValue as S,
        };
        use crate::heat_balance::setup_postludes::{
            SelectedCurves, SourceCurveOutputField as Field, SourceCurveReportContext,
            SourceCurveType as Kind,
        };
        use crate::heat_balance::source_output_registration::{
            SourceMtdFileHost, SourceOutputDictionaryStreams, SourceOutputRegistration,
            SourceRealOutputOwners, SourceRealOutputTarget,
        };
        let rows = vec![
            (
                "Curve:Biquadratic",
                "B SECOND",
                BTreeMap::from([
                    (
                        "coefficient1_constant".into(),
                        S::Number("18446744073709551615".into()),
                    ),
                    ("coefficient2_x".into(), S::Number("0.2".into())),
                    ("coefficient3_x_2".into(), S::Number("0.3".into())),
                    ("coefficient4_y".into(), S::Number("0.4".into())),
                    ("coefficient5_y_2".into(), S::Number("0.5".into())),
                    ("coefficient6_x_y".into(), S::Number("0.6".into())),
                    ("minimum_value_of_x".into(), S::Number("-10.0".into())),
                    ("maximum_value_of_x".into(), S::Number("50.0".into())),
                    ("minimum_value_of_y".into(), S::Number("-5.0".into())),
                    ("maximum_value_of_y".into(), S::Number("25.0".into())),
                    ("minimum_curve_output".into(), S::Number("0.0".into())),
                    ("input_unit_type_for_x".into(), S::String("".into())),
                    (
                        "input_unit_type_for_y".into(),
                        S::String("Temperature".into()),
                    ),
                    ("output_unit_type".into(), S::String("Power".into())),
                ]),
                vec![-1.0, 0.2, 0.3, 0.4, 0.5, 0.6],
                Kind::BiQuadratic,
            ),
            (
                "Curve:Biquadratic",
                "a first",
                BTreeMap::from([
                    ("coefficient1_constant".into(), S::Number("0.1".into())),
                    ("coefficient2_x".into(), S::Number("0.2".into())),
                    ("coefficient3_x_2".into(), S::Number("0.3".into())),
                    ("coefficient4_y".into(), S::Number("0.4".into())),
                    ("coefficient5_y_2".into(), S::Number("0.5".into())),
                    ("coefficient6_x_y".into(), S::Number("0.6".into())),
                    ("minimum_value_of_x".into(), S::Number("-10.0".into())),
                    ("maximum_value_of_x".into(), S::Number("50.0".into())),
                    ("minimum_value_of_y".into(), S::Number("-5.0".into())),
                    ("maximum_value_of_y".into(), S::Number("25.0".into())),
                ]),
                vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6],
                Kind::BiQuadratic,
            ),
            (
                "Curve:QuadraticLinear",
                "QUADRATICLINEAR",
                BTreeMap::from([
                    ("coefficient1_constant".into(), S::Number("0.1".into())),
                    ("coefficient2_x".into(), S::Number("0.2".into())),
                    ("coefficient3_x_2".into(), S::Number("0.3".into())),
                    ("coefficient4_y".into(), S::Number("0.4".into())),
                    ("coefficient5_x_y".into(), S::Number("0.5".into())),
                    ("coefficient6_x_2_y".into(), S::Number("0.6".into())),
                    ("minimum_value_of_x".into(), S::Number("-10.0".into())),
                    ("maximum_value_of_x".into(), S::Number("50.0".into())),
                    ("minimum_value_of_y".into(), S::Number("-5.0".into())),
                    ("maximum_value_of_y".into(), S::Number("25.0".into())),
                ]),
                vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6],
                Kind::QuadraticLinear,
            ),
            (
                "Curve:CubicLinear",
                "CUBICLINEAR",
                BTreeMap::from([
                    ("coefficient1_constant".into(), S::Number("0.1".into())),
                    ("coefficient2_x".into(), S::Number("0.2".into())),
                    ("coefficient3_x_2".into(), S::Number("0.3".into())),
                    ("coefficient4_x_3".into(), S::Number("0.4".into())),
                    ("coefficient5_y".into(), S::Number("0.5".into())),
                    ("coefficient6_x_y".into(), S::Number("0.6".into())),
                    ("minimum_value_of_x".into(), S::Number("-10.0".into())),
                    ("maximum_value_of_x".into(), S::Number("50.0".into())),
                    ("minimum_value_of_y".into(), S::Number("-5.0".into())),
                    ("maximum_value_of_y".into(), S::Number("25.0".into())),
                ]),
                vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6],
                Kind::CubicLinear,
            ),
            (
                "Curve:Bicubic",
                "BICUBIC",
                BTreeMap::from([
                    ("coefficient1_constant".into(), S::Number("0.1".into())),
                    ("coefficient2_x".into(), S::Number("0.2".into())),
                    ("coefficient3_x_2".into(), S::Number("0.3".into())),
                    ("coefficient4_y".into(), S::Number("0.4".into())),
                    ("coefficient5_y_2".into(), S::Number("0.5".into())),
                    ("coefficient6_x_y".into(), S::Number("0.6".into())),
                    ("coefficient7_x_3".into(), S::Number("0.7".into())),
                    ("coefficient8_y_3".into(), S::Number("0.8".into())),
                    ("coefficient9_x_2_y".into(), S::Number("0.9".into())),
                    ("coefficient10_x_y_2".into(), S::Number("1.0".into())),
                    ("minimum_value_of_x".into(), S::Number("-10.0".into())),
                    ("maximum_value_of_x".into(), S::Number("50.0".into())),
                    ("minimum_value_of_y".into(), S::Number("-5.0".into())),
                    ("maximum_value_of_y".into(), S::Number("25.0".into())),
                ]),
                vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0],
                Kind::BiCubic,
            ),
            (
                "Curve:FanPressureRise",
                "FANPRESSURERISE",
                BTreeMap::from([
                    ("coefficient1_c1".into(), S::Number("0.1".into())),
                    ("coefficient2_c2".into(), S::Number("0.2".into())),
                    ("coefficient3_c3".into(), S::Number("0.3".into())),
                    ("coefficient4_c4".into(), S::Number("0.4".into())),
                    ("minimum_value_of_qfan".into(), S::Number("0.1".into())),
                    ("maximum_value_of_qfan".into(), S::Number("2.0".into())),
                    ("minimum_value_of_psm".into(), S::Number("50.0".into())),
                    ("maximum_value_of_psm".into(), S::Number("500.0".into())),
                ]),
                vec![0.1, 0.2, 0.3, 0.4],
                Kind::FanPressureRise,
            ),
        ];
        let definitions = [
            ("Curve:Biquadratic", 4, 12),
            ("Curve:QuadraticLinear", 4, 12),
            ("Curve:CubicLinear", 4, 12),
            ("Curve:Bicubic", 4, 16),
            ("Curve:FanPressureRise", 1, 10),
        ]
        .into_iter()
        .map(|(name, a, n)| SourceObjectDefinition {
            object_type: name.into(),
            alpha_fields: a,
            numeric_fields: n,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        })
        .collect();
        let mut objects = BTreeMap::<String, BTreeMap<String, BTreeMap<String, S>>>::new();
        let mut object_order = BTreeMap::<String, Vec<String>>::new();
        let mut shapes = BTreeMap::<String, Vec<SourceObjectShape>>::new();
        for (family, name, fields, _, _) in &rows {
            objects
                .entry((*family).into())
                .or_default()
                .insert((*name).into(), fields.clone());
            object_order
                .entry((*family).into())
                .or_default()
                .push((*name).into());
            shapes
                .entry((*family).into())
                .or_default()
                .push(SourceObjectShape {
                    object_name: (*name).into(),
                    field_sizes: fields.keys().map(|field| (field.clone(), 1)).collect(),
                });
        }
        let mut input = SelectedInputProcessor::from_admitted_input(
            definitions,
            shapes
                .into_iter()
                .map(|(k, v)| (k, SourceInputFamily::Objects(v)))
                .collect(),
        )
        .unwrap();
        input
            .retain_source_input_statistics(SourceInputStatisticsInput {
                objects,
                object_order,
            })
            .unwrap();
        let maximum = input.get_max_schema_args().unwrap();
        let bicubic = input.get_object_def_max_args("Curve:Bicubic").unwrap();
        assert_eq!(
            (
                maximum.alphas,
                maximum.numerics,
                bicubic.alphas,
                bicubic.numerics
            ),
            (4, 16, 4, 16)
        );
        let mut statistics =
            crate::heat_balance::source_output_prescan::SourceOutputPreScan::default();
        input
            .report_source_idf_records_stats(&mut statistics)
            .unwrap();
        assert_eq!(
            (
                statistics.number_input_records,
                statistics.total_fields_with_defaults,
                statistics.number_defaulted_fields
            ),
            (6, 15, 12)
        );
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let unpassed = (
            shortcut.alpha_args.clone(),
            shortcut.numeric_args.clone(),
            shortcut.alpha_field_blanks.clone(),
            shortcut.current_module_object.clone(),
        );
        let globals =
            crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals::default(
            );
        let project = crate::heat_balance::selected_project_data::SelectedProjectData::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &project.system.developer_flag,
            warmup: &clock.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        };
        let mut curves = SelectedCurves::default();
        let (mut state, mut text, mut console, mut eio) =
            (error_state(), Vec::new(), Vec::new(), Vec::new());
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
        curves
            .initialize_state(&mut input, &mut shortcut, &flags, &mut backend)
            .unwrap();
        for (at, (_, name, _, expected, kind)) in rows.iter().enumerate() {
            let upper = name.to_ascii_uppercase();
            let actual = curves.get_curve(&upper).unwrap();
            assert_eq!(
                (
                    actual.name.as_str(),
                    actual.num,
                    actual.curve_type,
                    actual.num_dims
                ),
                (upper.as_str(), i32::try_from(at + 1).unwrap(), *kind, 2)
            );
            assert_eq!(&actual.coeff[..expected.len()], expected.as_slice());
            assert!(
                actual.coeff[expected.len()..]
                    .iter()
                    .all(|v| v.to_bits() == 0)
                    && actual.inputs.iter().all(|v| v.to_bits() == 0)
                    && actual.output.to_bits() == 0
            );
            assert!(
                actual
                    .input_limits
                    .iter()
                    .all(|v| !v.min_present && !v.max_present)
                    && actual.table_index == 0
                    && actual.grid_value_index == 0
                    && actual.context_string.is_empty()
                    && !actual.ems_override_on
                    && actual.ems_override_curve_value.to_bits() == 0
            );
            assert!(
                std::ptr::eq(
                    curves
                        .real_output(SourceRealOutputTarget::Curve {
                            field: Field::Input(1),
                            index: actual.num
                        })
                        .unwrap(),
                    &actual.inputs[0]
                ) && std::ptr::eq(
                    curves.read_curve_output(Field::Output, actual.num).unwrap(),
                    &actual.output
                )
            );
        }
        assert_eq!(
            (
                curves.get_curve_index("B SECOND"),
                curves.get_curve_index("A FIRST"),
                curves.get_curve_index("a first")
            ),
            (1, 2, 0)
        );
        assert!(
            curves.get_curve("MISSING").is_none()
                && curves.read_curve_output(Field::Input(0), 1).is_none()
                && curves.read_curve_output(Field::Input(3), 1).is_none()
                && curves.read_curve_output(Field::Output, 0).is_none()
        );
        assert!(
            curves
                .get_curve("B SECOND")
                .unwrap()
                .output_limits
                .min_present
                && !curves
                    .get_curve("B SECOND")
                    .unwrap()
                    .output_limits
                    .max_present
                && curves
                    .get_curve("B SECOND")
                    .unwrap()
                    .output_limits
                    .min
                    .to_bits()
                    == 0
                && !curves
                    .get_curve("A FIRST")
                    .unwrap()
                    .output_limits
                    .min_present
        );
        assert_eq!(
            (
                shortcut.alpha_args.clone(),
                shortcut.numeric_args.clone(),
                shortcut.alpha_field_blanks.clone(),
                shortcut.current_module_object.clone()
            ),
            unpassed
        );
        assert_eq!(
            &shortcut.numeric_field_names[..10],
            [
                "coefficient1_c1",
                "coefficient2_c2",
                "coefficient3_c3",
                "coefficient4_c4",
                "minimum_value_of_qfan",
                "maximum_value_of_qfan",
                "minimum_value_of_psm",
                "maximum_value_of_psm",
                "minimum_curve_output",
                "maximum_curve_output"
            ]
        );
        assert!(
            shortcut.numeric_field_blanks[..8]
                .iter()
                .all(|blank| !blank)
                && shortcut.numeric_field_blanks[8..]
                    .iter()
                    .all(|blank| *blank)
                && shortcut.alpha_field_names[0] == "name"
                && shortcut.alpha_field_names[1..].iter().all(String::is_empty)
        );
        assert!(
            rows.iter()
                .all(|(family, name, _, _, _)| !input.is_object_unused(family, name))
        );
        let report_inputs = ep_model::ReportInputs::default();
        let mut schedules =
            crate::heat_balance::selected_schedule_manager::SelectedScheduleManager::default();
        let mut registration = SourceOutputRegistration::default();
        let mut result_variables = Vec::new();
        let mut eso = Vec::new();
        let path = std::env::temp_dir().join(format!(
            "rusted-curve-unrequested-{}-{}.mtd",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut host = SourceMtdFileHost::new(&path);
        let mut dictionary = SourceOutputDictionaryStreams {
            eso: Some(&mut eso),
            sqlite_present: false,
            results_framework_output_variables: &mut result_variables,
        };
        curves
            .initialize_source_curve_reporting(&mut SourceCurveReportContext {
                globals: &globals,
                input: &mut input,
                schedules: Some(&mut schedules),
                reports: &report_inputs,
                registration: &mut registration,
                prescan: &statistics,
                minimum_report_frequency: Some(&project.system.minimum_report_frequency),
                minutes_in_time_step: 15,
                diagnostic_flags: Some(flags),
                backend: &mut backend,
                host: &mut host,
                dictionary_host: &mut dictionary,
                eio: &mut eio,
            })
            .unwrap();
        assert_eq!(
            (
                registration.num_real_variables_setup,
                registration.dictionary_variables.len(),
                registration.out_vars.len()
            ),
            (18, 3, 0)
        );
        assert_eq!(
            registration
                .dictionary_variables
                .iter()
                .map(|v| v.name.as_str())
                .collect::<Vec<_>>(),
            [
                "Performance Curve Input Variable 1 Value",
                "Performance Curve Input Variable 2 Value",
                "Performance Curve Output Value"
            ]
        );
        drop(host);
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        assert!(
            state.tracking.total_severe_errors == 0
                && state.tracking.total_warning_errors == 0
                && text.is_empty()
                && eio.is_empty()
                && eso.is_empty()
        );
    }
    // One new genuine Angle context; same real IP initializes Curves before Material.
    {
        use crate::heat_balance::selected_input_processor::{
            SourceInputStatisticsInput, SourceInputStatisticsValue as S,
        };
        use crate::heat_balance::setup_postludes::SelectedCurves;
        let (mut model, mut raw) = fixture(true, 0.15);
        let curve_names = [
            "ANGLE BIQUADRATIC",
            "ANGLE QUADRATIC LINEAR",
            "ANGLE CUBIC LINEAR",
            "ANGLE BICUBIC",
            "ANGLE FAN",
        ];
        for (at, curve_name) in curve_names.iter().enumerate() {
            let name = format!("ANGLE GLASS {at}");
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material {
                id,
                name: NormalizedName::new(&name),
                definition: MaterialDefinition::WindowGlazingSpectralAndAngle(
                    ep_model::WindowGlazingSpectralAndAngleMaterial {
                        thickness_m: 99.0,
                        solar_transmittance_at_normal_incidence: 0.2,
                        front_side_solar_reflectance_at_normal_incidence: 0.2,
                        back_side_solar_reflectance_at_normal_incidence: 0.2,
                        visible_transmittance_at_normal_incidence: 0.2,
                        front_side_visible_reflectance_at_normal_incidence: 0.2,
                        back_side_visible_reflectance_at_normal_incidence: 0.2,
                        infrared_transmittance_at_normal_incidence: 0.2,
                        front_side_infrared_hemispherical_emissivity: 0.7,
                        back_side_infrared_hemispherical_emissivity: 0.7,
                        conductivity_w_per_m_k: 99.0,
                        dirt_correction_factor_for_solar_and_visible_transmittance: 0.5,
                        solar_diffusing: false,
                        youngs_modulus_pa: 1.0,
                        poissons_ratio: 0.2,
                    },
                ),
            });
            model.ctf_report_names.materials.push((id, name.clone()));
            raw.glazing.push(ProjectInputObject { object_name: name, idf_max_fields: None, fields: BTreeMap::from([
                ("optical_data_type".into(), Value::String("SpectralAndAngle".into())), ("thickness".into(), Value::Number(0.003)),
                ("solar_transmittance_at_normal_incidence".into(), Value::Number(0.75)),
                ("front_side_solar_reflectance_at_normal_incidence".into(), Value::Number(0.75)),
                ("back_side_solar_reflectance_at_normal_incidence".into(), Value::Number(0.75)),
                ("visible_transmittance_at_normal_incidence".into(), Value::Number(0.75)),
                ("front_side_visible_reflectance_at_normal_incidence".into(), Value::Number(0.75)),
                ("back_side_visible_reflectance_at_normal_incidence".into(), Value::Number(0.75)),
                ("infrared_transmittance_at_normal_incidence".into(), Value::Number(0.75)),
                ("window_glass_spectral_and_incident_angle_transmittance_data_set_table_name".into(), Value::String(curve_name.to_ascii_lowercase())),
                ("window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name".into(), Value::String((*curve_name).into())),
                ("window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name".into(), Value::String(if at == 0 { "ANGLE BACK BAD" } else { curve_name }.to_string())),
            ]) });
        }
        let mut objects: BTreeMap<String, BTreeMap<String, BTreeMap<String, S>>> =
            BTreeMap::from([
                (
                    "Curve:Biquadratic".into(),
                    BTreeMap::from([
                        (
                            "ANGLE BACK BAD".into(),
                            BTreeMap::from([
                                ("coefficient1_constant".into(), S::Number("0.1".into())),
                                ("coefficient2_x".into(), S::Number("0.1".into())),
                                ("coefficient3_x_2".into(), S::Number("0.1".into())),
                                ("coefficient4_y".into(), S::Number("0.1".into())),
                                ("coefficient5_y_2".into(), S::Number("0.1".into())),
                                ("coefficient6_x_y".into(), S::Number("0.1".into())),
                                ("minimum_value_of_x".into(), S::Number("0.0".into())),
                                ("maximum_value_of_x".into(), S::Number("60.0".into())),
                                ("minimum_value_of_y".into(), S::Number("0.1".into())),
                                ("maximum_value_of_y".into(), S::Number("4.0".into())),
                                ("input_unit_type_for_x".into(), S::String("".into())),
                            ]),
                        ),
                        (
                            "ANGLE BIQUADRATIC".into(),
                            BTreeMap::from([
                                ("coefficient1_constant".into(), S::Number("0.1".into())),
                                ("coefficient2_x".into(), S::Number("0.1".into())),
                                ("coefficient3_x_2".into(), S::Number("0.1".into())),
                                ("coefficient4_y".into(), S::Number("0.1".into())),
                                ("coefficient5_y_2".into(), S::Number("0.1".into())),
                                ("coefficient6_x_y".into(), S::Number("0.1".into())),
                                ("minimum_value_of_x".into(), S::Number("0.000001".into())),
                                ("maximum_value_of_x".into(), S::Number("90.0".into())),
                                ("minimum_value_of_y".into(), S::Number("0.1".into())),
                                ("maximum_value_of_y".into(), S::Number("4.0".into())),
                                ("input_unit_type_for_x".into(), S::String("".into())),
                            ]),
                        ),
                    ]),
                ),
                (
                    "Curve:QuadraticLinear".into(),
                    BTreeMap::from([(
                        "ANGLE QUADRATIC LINEAR".into(),
                        BTreeMap::from([
                            ("coefficient1_constant".into(), S::Number("0.1".into())),
                            ("coefficient2_x".into(), S::Number("0.1".into())),
                            ("coefficient3_x_2".into(), S::Number("0.1".into())),
                            ("coefficient4_y".into(), S::Number("0.1".into())),
                            ("coefficient5_x_y".into(), S::Number("0.1".into())),
                            ("coefficient6_x_2_y".into(), S::Number("0.1".into())),
                            ("minimum_value_of_x".into(), S::Number("-1.0".into())),
                            ("maximum_value_of_x".into(), S::Number("90.0000005".into())),
                            ("minimum_value_of_y".into(), S::Number("0.1".into())),
                            ("maximum_value_of_y".into(), S::Number("4.0".into())),
                            ("minimum_curve_output".into(), S::Number("-0.0".into())),
                        ]),
                    )]),
                ),
                (
                    "Curve:CubicLinear".into(),
                    BTreeMap::from([(
                        "ANGLE CUBIC LINEAR".into(),
                        BTreeMap::from([
                            ("coefficient1_constant".into(), S::Number("0.1".into())),
                            ("coefficient2_x".into(), S::Number("0.1".into())),
                            ("coefficient3_x_2".into(), S::Number("0.1".into())),
                            ("coefficient4_x_3".into(), S::Number("0.1".into())),
                            ("coefficient5_y".into(), S::Number("0.1".into())),
                            ("coefficient6_x_y".into(), S::Number("0.1".into())),
                            ("minimum_value_of_x".into(), S::Number("0.0".into())),
                            ("maximum_value_of_x".into(), S::Number("90.0".into())),
                            ("minimum_value_of_y".into(), S::Number("0.1".into())),
                            ("maximum_value_of_y".into(), S::Number("4.0".into())),
                            ("maximum_curve_output".into(), S::Number("0.0".into())),
                        ]),
                    )]),
                ),
                (
                    "Curve:Bicubic".into(),
                    BTreeMap::from([(
                        "ANGLE BICUBIC".into(),
                        BTreeMap::from([
                            ("coefficient1_constant".into(), S::Number("0.1".into())),
                            ("coefficient2_x".into(), S::Number("0.1".into())),
                            ("coefficient3_x_2".into(), S::Number("0.1".into())),
                            ("coefficient4_y".into(), S::Number("0.1".into())),
                            ("coefficient5_y_2".into(), S::Number("0.1".into())),
                            ("coefficient6_x_y".into(), S::Number("0.1".into())),
                            ("coefficient7_x_3".into(), S::Number("0.1".into())),
                            ("coefficient8_y_3".into(), S::Number("0.1".into())),
                            ("coefficient9_x_2_y".into(), S::Number("0.1".into())),
                            ("coefficient10_x_y_2".into(), S::Number("0.1".into())),
                            ("minimum_value_of_x".into(), S::Number("0.0".into())),
                            ("maximum_value_of_x".into(), S::Number("90.0".into())),
                            ("minimum_value_of_y".into(), S::Number("0.1".into())),
                            ("maximum_value_of_y".into(), S::Number("4.0".into())),
                            ("minimum_curve_output".into(), S::Number("2.0".into())),
                            ("maximum_curve_output".into(), S::Number("-1.0".into())),
                        ]),
                    )]),
                ),
                (
                    "Curve:FanPressureRise".into(),
                    BTreeMap::from([(
                        "ANGLE FAN".into(),
                        BTreeMap::from([
                            ("coefficient1_c1".into(), S::Number("0.1".into())),
                            ("coefficient2_c2".into(), S::Number("0.1".into())),
                            ("coefficient3_c3".into(), S::Number("0.1".into())),
                            ("coefficient4_c4".into(), S::Number("0.1".into())),
                            ("minimum_value_of_qfan".into(), S::Number("0.0".into())),
                            ("maximum_value_of_qfan".into(), S::Number("90.0".into())),
                            ("minimum_value_of_psm".into(), S::Number("0.1".into())),
                            ("maximum_value_of_psm".into(), S::Number("4.0".into())),
                            ("input_unit_type_for_x".into(), S::Bool(true)),
                        ]),
                    )]),
                ),
            ]);
        let mut object_order: BTreeMap<String, Vec<String>> = BTreeMap::from([
            (
                "Curve:Biquadratic".into(),
                vec!["ANGLE BACK BAD".into(), "ANGLE BIQUADRATIC".into()],
            ),
            (
                "Curve:QuadraticLinear".into(),
                vec!["ANGLE QUADRATIC LINEAR".into()],
            ),
            (
                "Curve:CubicLinear".into(),
                vec!["ANGLE CUBIC LINEAR".into()],
            ),
            ("Curve:Bicubic".into(), vec!["ANGLE BICUBIC".into()]),
            ("Curve:FanPressureRise".into(), vec!["ANGLE FAN".into()]),
        ]);
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let definitions = [
            ("Curve:Biquadratic", 4, 12),
            ("Curve:QuadraticLinear", 4, 12),
            ("Curve:CubicLinear", 4, 12),
            ("Curve:Bicubic", 4, 16),
            ("Curve:FanPressureRise", 1, 10),
        ]
        .into_iter()
        .map(|(f, a, n)| SourceObjectDefinition {
            object_type: f.into(),
            alpha_fields: a,
            numeric_fields: n,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        })
        .chain(families.iter().map(|(f, _, a, n)| SourceObjectDefinition {
            object_type: (*f).into(),
            alpha_fields: *a,
            numeric_fields: *n,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        }))
        .collect();
        for (family, rows, _, _) in families {
            object_order.insert(
                family.into(),
                rows.iter().map(|r| r.object_name.clone()).collect(),
            );
            objects.insert(
                family.into(),
                rows.iter()
                    .map(|r| {
                        (
                            r.object_name.clone(),
                            r.fields
                                .iter()
                                .map(|(k, v)| {
                                    (
                                        k.clone(),
                                        match v {
                                            Value::String(v) => S::String(v.clone()),
                                            Value::Number(v) => S::Number(v.to_string()),
                                        },
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            );
        }
        let shapes = objects
            .iter()
            .map(|(family, rows)| {
                (
                    family.clone(),
                    SourceInputFamily::Objects(
                        object_order[family]
                            .iter()
                            .map(|name| SourceObjectShape {
                                object_name: name.clone(),
                                field_sizes: rows[name].keys().map(|k| (k.clone(), 1)).collect(),
                            })
                            .collect(),
                    ),
                )
            })
            .collect();
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        input
            .retain_source_input_statistics(SourceInputStatisticsInput {
                objects,
                object_order,
            })
            .unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let flags = SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &developer,
            warmup: &clock.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        };
        let mut curves = SelectedCurves::default();
        let mut state = error_state();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut scan = ReportScanState::default();
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();
        {
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
            curves
                .initialize_state(&mut input, &mut shortcut, &flags, &mut backend)
                .unwrap();
            data.get_material_data(
                &raw,
                &mut MaterialDataOwners {
                    curves: Some(&curves),
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
            .unwrap();
        }
        for (at, name) in curve_names.iter().enumerate() {
            let id = MaterialId(u32::try_from(at + 3).unwrap());
            let material = &data.simple_glazing[&id];
            let curve = curves.get_curve(name).unwrap();
            let back = curves
                .get_curve(if at == 0 { "ANGLE BACK BAD" } else { name })
                .unwrap();
            assert_eq!(
                (
                    material.spec_ang_trans_curve,
                    material.spec_ang_front_curve,
                    material.spec_ang_back_curve
                ),
                (curve.num, curve.num, back.num)
            );
            assert!(
                material.optical_model
                    == super::simple_glazing::SourceSimpleGlazingOpticalModel::SpectralAndAngle
                    && [
                        material.transmittance,
                        material.solar_reflectance_front,
                        material.solar_reflectance_back,
                        material.visible_transmittance,
                        material.visible_reflectance_front,
                        material.visible_reflectance_back,
                        material.thermal_transmittance
                    ]
                    .iter()
                    .all(|v| v.to_bits() == 0)
            );
            assert!(
                material.thickness == 0.003
                    && material.conductivity == 0.9
                    && material.thermal_absorptance == 0.84
                    && material.resistance > 0.0
                    && material.resistance.to_bits()
                        == data.nominal_r[usize::try_from(
                            data.material_map[&format!("ANGLE GLASS {at}")] - 1
                        )
                        .unwrap()]
                        .to_bits()
                    && data.material_groups[&id] == SourceMaterialGroup::Glass
            );
            assert_eq!(
                data.source_glazing_surface_properties(
                    data.material_map[&format!("ANGLE GLASS {at}")],
                    id
                )
                .unwrap(),
                (
                    MaterialSurfaceRoughness::VerySmooth,
                    OpaqueSurfaceProperties {
                        thermal_absorptance: 0.84,
                        solar_absorptance: 0.0,
                        visible_absorptance: 0.0
                    }
                )
            );
            let expected = [
                (false, false, 0.0_f64.to_bits(), 0.0_f64.to_bits()),
                (true, false, (-0.0_f64).to_bits(), 0.0_f64.to_bits()),
                (false, true, 0.0_f64.to_bits(), 0.0_f64.to_bits()),
                (true, true, 2.0_f64.to_bits(), (-1.0_f64).to_bits()),
                (false, false, 0.0_f64.to_bits(), 0.0_f64.to_bits()),
            ][at];
            assert_eq!(
                (
                    curve.output_limits.min_present,
                    curve.output_limits.max_present,
                    curve.output_limits.min.to_bits(),
                    curve.output_limits.max.to_bits()
                ),
                expected
            );
            assert!(curve.inputs.iter().all(|v| v.to_bits() == 0) && curve.output.to_bits() == 0);
        }
        assert!(
            curves.get_curve("ANGLE BACK BAD").unwrap().input_limits[0].max == 60.0
                && curves.get_curve("ANGLE BIQUADRATIC").unwrap().input_limits[0].max == 90.0
                && curves.get_curve("ANGLE BIQUADRATIC").unwrap().input_limits[0].min == 1.0e-6
                && curves
                    .get_curve("ANGLE QUADRATIC LINEAR")
                    .unwrap()
                    .input_limits[0]
                    .min
                    == -1.0
                && curves
                    .get_curve("ANGLE QUADRATIC LINEAR")
                    .unwrap()
                    .input_limits[0]
                    .max
                    > 90.0
                && !errors_found
                && text.is_empty()
        );
        assert!(input.source_unused_objects().next().is_none());
        data.write_material_reports(&model, &mut eio).unwrap();
        assert_eq!(
            std::str::from_utf8(&eio)
                .unwrap()
                .lines()
                .filter(|line| line.starts_with(" Material Details,ANGLE GLASS "))
                .count(),
            5
        );
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
                    curves: None,
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
                curves: None,
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
                curves: None,
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
    // The only new local type: two mandatory Write trait methods, no test or
    // fixture function. Real Severe bytes succeed; real Continue bytes fail.
    struct FailContinue {
        bytes: Vec<u8>,
    }
    impl std::io::Write for FailContinue {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if bytes.starts_with(b" **   ~~~   ** ") {
                return Err(std::io::Error::other("actual Continue sink failure"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    // Exactly four fresh actual IP/Data/backend contexts. The original named
    // duplicate contexts and all their assertions execute above unchanged.
    for (missing_properties, failure) in
        [(true, false), (true, true), (false, false), (false, true)]
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        for (name, gas_type) in [
            ("C CUSTOM", ep_model::WindowGasType::Custom),
            ("Z AFTER", ep_model::WindowGasType::Air),
        ] {
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material {
                id,
                name: NormalizedName::new(name),
                definition: MaterialDefinition::WindowGas(ep_model::WindowGasMaterial {
                    gas_type,
                    thickness_m: 0.012,
                    properties: ep_model::WindowGasType::Air.standard_properties().unwrap(),
                }),
            });
            model.ctf_report_names.materials.push((id, name.into()));
        }
        let mut fields = BTreeMap::from([
            ("gas_type".into(), Value::String("Custom".into())),
            ("thickness".into(), Value::Number(0.012)),
            (
                "conductivity_coefficient_a".into(),
                Value::Number(if missing_properties { 0.02 } else { 0.0 }),
            ),
        ]);
        let effective_viscosity = if missing_properties && !failure {
            Some(
                ep_model::classify_source_json_number("18446744073709551615")
                    .unwrap()
                    .as_f64(),
            )
        } else {
            None
        };
        if let Some(value) = effective_viscosity {
            fields.insert("viscosity_coefficient_a".into(), Value::Number(value));
        }
        if !missing_properties {
            fields.insert("viscosity_coefficient_a".into(), Value::Number(0.00001));
            fields.insert("specific_heat_coefficient_a".into(), Value::Number(1000.0));
            fields.insert("molecular_weight".into(), Value::Number(28.0));
        }
        raw.window_gas = vec![
            ProjectInputObject {
                object_name: "C CUSTOM".into(),
                idf_max_fields: None,
                fields,
            },
            ProjectInputObject {
                object_name: "Z AFTER".into(),
                idf_max_fields: None,
                fields: BTreeMap::from([
                    ("gas_type".into(), Value::String("Air".into())),
                    ("thickness".into(), Value::Number(0.012)),
                ]),
            },
        ];
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
        let mut broken = BrokenDiagnostic;
        let mut continue_sink = FailContinue { bytes: Vec::new() };
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();
        let outcome = {
            let stream: &mut dyn std::io::Write = if !failure {
                &mut text
            } else if missing_properties {
                &mut broken
            } else {
                &mut continue_sink
            };
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
                curves: None,
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
            if effective_viscosity.is_some() {
                assert_eq!(
                    raw.get_window_gas_item(&raw.window_gas[0], &mut owners)
                        .unwrap(),
                    (2, 5)
                );
                assert_eq!(
                    owners.shortcut.numeric_args[4].to_bits(),
                    (-1.0_f64).to_bits()
                );
                assert!(!owners.shortcut.numeric_field_blanks[4]);
                assert!(
                    owners.shortcut.numeric_field_blanks[7]
                        && owners.shortcut.numeric_field_blanks[10]
                );
            }
            result
        };
        let row = &data.window_gas[&MaterialId(3)];
        assert_eq!(row.gases[0].gas_type, Some(ep_model::WindowGasType::Custom));
        assert_eq!(row.thickness, 0.012);
        assert_eq!(
            row.gases[0].properties.conductivity.coefficient_a,
            if missing_properties { 0.02 } else { 0.0 }
        );
        assert_eq!(
            row.gases[0].properties.viscosity.coefficient_a,
            if missing_properties {
                effective_viscosity.unwrap_or(0.0)
            } else {
                0.00001
            }
        );
        assert_eq!(
            row.gases[0].properties.specific_heat.coefficient_a,
            if missing_properties { 0.0 } else { 1000.0 }
        );
        assert_eq!(
            row.gases[0].properties.molecular_weight_g_per_mol,
            if missing_properties { 0.0 } else { 28.0 }
        );
        assert_eq!(row.gases[0].properties.specific_heat_ratio, 0.0);
        assert_eq!(row.resistance, 0.0);
        assert_eq!(data.nominal_r[3], 0.0);
        assert_eq!(data.material_map["C CUSTOM"], 4);
        assert!(!input.is_object_unused("WindowMaterial:Gas", "C CUSTOM"));
        if failure {
            assert!(outcome.is_err());
            assert_eq!(
                data.materials.len(),
                4,
                "allocation and all Custom writes precede diagnostic stop"
            );
            assert!(input.is_object_unused("WindowMaterial:Gas", "Z AFTER"));
            assert_eq!(state.tracking.total_severe_errors, 1);
            assert_eq!(
                errors_found, missing_properties,
                "property flag BEFORE IO; denominator flag only AFTER Continue"
            );
            if missing_properties {
                assert!(state.tracking.last_severe_error.is_empty());
            } else {
                let bytes = std::str::from_utf8(&continue_sink.bytes).unwrap();
                assert!(bytes.contains("WindowMaterial:Gas=\"C CUSTOM\", Illegal value."));
                assert!(!bytes.contains(" **   ~~~   ** "));
                assert_eq!(
                    state.tracking.last_severe_error,
                    "WindowMaterial:Gas=\"C CUSTOM\", Illegal value."
                );
            }
        } else {
            outcome.expect("real Custom diagnostic continues, no invented Gas-local Fatal");
            assert!(errors_found);
            assert_eq!(data.materials.len(), 5);
            assert_eq!(
                data.nominal_r[4], 0.0,
                "shared actual error suppresses later Named NominalR"
            );
            assert!(!input.is_object_unused("WindowMaterial:Gas", "Z AFTER"));
            let text = std::str::from_utf8(&text).unwrap();
            if missing_properties {
                assert_eq!(state.tracking.total_severe_errors, 3);
                assert_eq!(
                    text.matches("GetMaterialData: WindowMaterial:Gas = C CUSTOM")
                        .count(),
                    2
                );
                let vis = text.find("viscosity_coefficient_a not > 0.0").unwrap();
                let cp = text.find("specific_heat_coefficient_a not > 0.0").unwrap();
                let mw = text.find("molecular_weight not > 0.0").unwrap();
                assert!(vis < cp && cp < mw);
                assert!(!text.contains("Nominal resistance of gap"));
            } else {
                assert_eq!(state.tracking.total_severe_errors, 1);
                assert_eq!(
                    text.matches("WindowMaterial:Gas=\"C CUSTOM\", Illegal value.")
                        .count(),
                    1
                );
                assert!(text.contains("Nominal resistance of gap at room temperature calculated at a negative Conductivity=[0.000]."));
            }
        }
    }
    // Three fresh real contexts for the same wrapped Thickness: successful IO,
    // first diagnostic IO failure, then Continue failure. Existing sinks reused.
    for failure in [0, 1, 2] {
        let (mut model, mut raw) = fixture(true, 0.15);
        let id = MaterialId(3);
        model.materials.push(Material {
            id,
            name: NormalizedName::new("M NEG"),
            definition: MaterialDefinition::WindowGasMixture(ep_model::WindowGasMixtureMaterial {
                thickness_m: 0.012,
                gases: ep_model::WindowGasMixture::One([ep_model::WindowGasMixtureComponent {
                    gas_type: ep_model::WindowStandardGasType::Air,
                    fraction: 1.0,
                }]),
            }),
        });
        model.ctf_report_names.materials.push((id, "M NEG".into()));
        let wrapped = ep_model::classify_source_json_number("18446744073709551615")
            .unwrap()
            .as_f64();
        raw.window_gas_mixture.push(ProjectInputObject {
            object_name: "M NEG".into(),
            idf_max_fields: None,
            fields: BTreeMap::from([
                ("thickness".into(), Value::Number(wrapped)),
                ("number_of_gases_in_mixture".into(), Value::Number(1.0)),
                ("gas_1_type".into(), Value::String("Air".into())),
                ("gas_1_fraction".into(), Value::Number(1.0)),
                ("gas_2_type".into(), Value::String("Argon".into())),
                ("gas_2_fraction".into(), Value::Number(1.0)),
            ]),
        });
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:GasMixture", &raw.window_gas_mixture, 5, 6),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        let mut broken = BrokenDiagnostic;
        let mut continue_sink = FailContinue { bytes: Vec::new() };
        let outcome = {
            let stream: &mut dyn std::io::Write = match failure {
                1 => &mut broken,
                2 => &mut continue_sink,
                _ => &mut text,
            };
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
                curves: None,
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

            data.get_material_data(&raw, &mut owners)
        };
        let row = &data.window_gas[&id];
        assert_eq!((data.materials.len(), data.num_w5_gas_mixtures), (4, 1));
        assert_eq!(row.thickness.to_bits(), (-1.0_f64).to_bits());
        assert_eq!(row.gases[0].gas_type, Some(ep_model::WindowGasType::Air));
        assert!(
            !errors_found,
            "Thickness branch does not set the shared error flag, even after IO returns"
        );
        assert_eq!(state.tracking.total_severe_errors, 1);
        assert!(!input.is_object_unused("WindowMaterial:GasMixture", "M NEG"));
        if failure == 0 {
            outcome.unwrap();
            assert!(data.resistance_only[&id]);
            assert_eq!(row.gas_fractions, [1.0, 0.0, 0.0, 0.0, 0.0]);
            assert!(data.nominal_r[3] < 0.0);
            let text = std::str::from_utf8(&text).unwrap();
            assert!(
                text.find("WindowMaterial:GasMixture=\"M NEG\", Illegal value.")
                    .zip(text.find("thickness must be greater than 0."))
                    .is_some_and(|(severe, continuation)| severe < continuation)
            );
        } else {
            assert!(outcome.is_err());
            assert!(!data.resistance_only[&id]);
            assert_eq!(row.gas_fractions, [0.0; 5]);
            assert_eq!(
                (
                    row.gases[0].properties.conductivity.coefficient_a,
                    data.nominal_r[3]
                ),
                (0.0, 0.0)
            );
            if failure == 1 {
                assert!(state.tracking.last_severe_error.is_empty());
            } else {
                let text = std::str::from_utf8(&continue_sink.bytes).unwrap();
                assert!(
                    text.contains("WindowMaterial:GasMixture=\"M NEG\", Illegal value.")
                        && !text.contains(" **   ~~~   ** ")
                );
            }
        }
    }
    // Fourth diagnostic context: a real earlier duplicate sets the shared flag;
    // the subsequent valid Mixture still writes its unconditional NominalR.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        let id = MaterialId(3);
        model.materials.push(Material {
            id,
            name: NormalizedName::new("Z MIXTURE"),
            definition: MaterialDefinition::WindowGasMixture(ep_model::WindowGasMixtureMaterial {
                thickness_m: 0.012,
                gases: ep_model::WindowGasMixture::One([ep_model::WindowGasMixtureComponent {
                    gas_type: ep_model::WindowStandardGasType::Air,
                    fraction: 1.0,
                }]),
            }),
        });
        model
            .ctf_report_names
            .materials
            .push((id, "Z MIXTURE".into()));
        raw.window_gas_mixture = ["No mass", "Z MIXTURE"]
            .map(|name| ProjectInputObject {
                object_name: name.into(),
                idf_max_fields: None,
                fields: BTreeMap::from([
                    ("thickness".into(), Value::Number(0.012)),
                    ("number_of_gases_in_mixture".into(), Value::Number(1.0)),
                    ("gas_1_type".into(), Value::String("Air".into())),
                    ("gas_1_fraction".into(), Value::Number(1.0)),
                    ("gas_2_type".into(), Value::String("Argon".into())),
                    ("gas_2_fraction".into(), Value::Number(1.0)),
                ]),
            })
            .into();
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:GasMixture", &raw.window_gas_mixture, 5, 6),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        {
            let stream: &mut dyn std::io::Write = &mut text;
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
                curves: None,
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

            data.get_material_data(&raw, &mut owners).unwrap();
        }
        assert!(errors_found);
        assert_eq!((data.material_map["NO MASS"], data.materials.len()), (2, 4));
        assert!(data.nominal_r[3] > 0.0 && data.resistance_only[&id]);
        assert_eq!(state.tracking.total_severe_errors, 1);
        assert!(
            !input.is_object_unused("WindowMaterial:GasMixture", "No mass")
                && !input.is_object_unused("WindowMaterial:GasMixture", "Z MIXTURE")
        );
        assert_eq!(
            std::str::from_utf8(&text)
                .unwrap()
                .matches("GetMaterialData: WindowMaterial:GasMixture = NO MASS, duplicate name.")
                .count(),
            1
        );
    }

    // Two genuinely fresh sum contexts: all six original-schema-legal joint
    // failures, once with real IO and once failing the first Severe write.
    let glass = MaterialDefinition::WindowGlazingSpectralAverage(
        ep_model::WindowGlazingSpectralAverageMaterial {
            thickness_m: 0.003,
            solar_transmittance_at_normal_incidence: 0.0,
            front_side_solar_reflectance_at_normal_incidence: 0.0,
            back_side_solar_reflectance_at_normal_incidence: 0.0,
            visible_transmittance_at_normal_incidence: 0.0,
            front_side_visible_reflectance_at_normal_incidence: 0.0,
            back_side_visible_reflectance_at_normal_incidence: 0.0,
            infrared_transmittance_at_normal_incidence: 0.0,
            front_side_infrared_hemispherical_emissivity: 0.84,
            back_side_infrared_hemispherical_emissivity: 0.84,
            conductivity_w_per_m_k: 0.9,
            dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
            solar_diffusing: false,
            youngs_modulus_pa: 72_000_000_000.0,
            poissons_ratio: 0.22,
        },
    );
    for fail_first in [false, true] {
        let (mut model, mut raw) = fixture(true, 0.15);
        let id = MaterialId(3);
        model.materials.push(Material {
            id,
            name: NormalizedName::new("SUMS"),
            definition: glass,
        });
        model.ctf_report_names.materials.push((id, "SUMS".into()));
        raw.glazing.push(ProjectInputObject {
            object_name: "SUMS".into(),
            idf_max_fields: None,
            fields: BTreeMap::from([
                (
                    "optical_data_type".into(),
                    Value::String("SpectralAverage".into()),
                ),
                ("thickness".into(), Value::Number(0.003)),
                (
                    "solar_transmittance_at_normal_incidence".into(),
                    Value::Number(0.75),
                ),
                (
                    "front_side_solar_reflectance_at_normal_incidence".into(),
                    Value::Number(0.75),
                ),
                (
                    "back_side_solar_reflectance_at_normal_incidence".into(),
                    Value::Number(0.75),
                ),
                (
                    "visible_transmittance_at_normal_incidence".into(),
                    Value::Number(0.75),
                ),
                (
                    "front_side_visible_reflectance_at_normal_incidence".into(),
                    Value::Number(0.75),
                ),
                (
                    "back_side_visible_reflectance_at_normal_incidence".into(),
                    Value::Number(0.75),
                ),
                (
                    "infrared_transmittance_at_normal_incidence".into(),
                    Value::Number(0.75),
                ),
                ("solar_diffusing".into(), Value::String("Yes".into())),
            ]),
        });
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        let mut broken = BrokenDiagnostic;
        let result;
        {
            let stream: &mut dyn std::io::Write = if fail_first { &mut broken } else { &mut text };
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
                curves: None,
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

            result = data.get_material_data(&raw, &mut owners);
        }
        let row = &data.simple_glazing[&id];
        assert_eq!(
            (
                result.is_err(),
                errors_found,
                state.tracking.total_severe_errors,
                state.tracking.total_warning_errors
            ),
            (fail_first, true, if fail_first { 1 } else { 6 }, 0)
        );
        assert!(
            row.resistance > 0.0
                && row.resistance.to_bits() == data.nominal_r[3].to_bits()
                && data.resistance_only[&id]
                && row.thermal_transmittance == 0.75
                && row.thermal_back == 0.84
        );
        assert_eq!(row.solar_diffusing, !fail_first);
        let text = std::str::from_utf8(&text).unwrap();
        let positions: Option<Vec<_>> = [
            "solar_transmittance_at_normal_incidence + front_side_solar_reflectance_at_normal_incidence not <= 1.0",
            "solar_transmittance_at_normal_incidence + back_side_solar_reflectance_at_normal_incidence not <= 1.0",
            "visible_transmittance_at_normal_incidence + front_side_visible_reflectance_at_normal_incidence not <= 1.0",
            "visible_transmittance_at_normal_incidence + back_side_visible_reflectance_at_normal_incidence not <= 1.0",
            "infrared_transmittance_at_normal_incidence + front_side_infrared_hemispherical_emissivity not <= 1.0",
            "infrared_transmittance_at_normal_incidence + back_side_infrared_hemispherical_emissivity not <= 1.0",
        ].map(|needle| text.find(needle)).into_iter().collect();
        assert!(if fail_first {
            text.is_empty()
        } else {
            text.matches("GetMaterialData: WindowMaterial:Glazing = SUMS")
                .count()
                == 6
                && positions.is_some_and(|p| p.windows(2).all(|pair| pair[0] < pair[1]))
        });
        assert!(!input.is_object_unused("WindowMaterial:Glazing", "SUMS"));
    }
    // Fourth and last fresh context: real earlier duplicate, then wrapped
    // conductivity/Young, then normal. Never seed/reset the shared error latch.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        model.materials.extend(
            [("WRAPPED", MaterialId(3)), ("Z NORMAL", MaterialId(4))].map(|(name, id)| Material {
                id,
                name: NormalizedName::new(name),
                definition: glass,
            }),
        );
        model.ctf_report_names.materials.extend([
            (MaterialId(3), "WRAPPED".into()),
            (MaterialId(4), "Z NORMAL".into()),
        ]);
        let wrapped = ep_model::classify_source_json_number("18446744073709551615")
            .unwrap()
            .as_f64();
        raw.glazing = ["No mass", "WRAPPED", "Z NORMAL"]
            .map(|name| ProjectInputObject {
                object_name: name.into(),
                idf_max_fields: None,
                fields: BTreeMap::from([
                    (
                        "optical_data_type".into(),
                        Value::String("SpectralAverage".into()),
                    ),
                    ("thickness".into(), Value::Number(0.003)),
                ]),
            })
            .into();
        raw.glazing[1]
            .fields
            .insert("conductivity".into(), Value::Number(wrapped));
        raw.glazing[1]
            .fields
            .insert("young_s_modulus".into(), Value::Number(wrapped));
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        {
            let stream: &mut dyn std::io::Write = &mut text;
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
                curves: None,
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

            data.get_material_data(&raw, &mut owners).unwrap();
        }
        assert_eq!(
            (
                errors_found,
                state.tracking.total_severe_errors,
                state.tracking.total_warning_errors
            ),
            (true, 4, 0)
        );
        assert_eq!(
            (
                data.material_map["NO MASS"],
                data.material_map["WRAPPED"],
                data.material_map["Z NORMAL"],
                data.materials.len()
            ),
            (2, 4, 5, 5)
        );
        let bad = &data.simple_glazing[&MaterialId(3)];
        let normal = &data.simple_glazing[&MaterialId(4)];
        assert_eq!(
            [
                bad.conductivity,
                bad.young_modulus,
                bad.resistance,
                data.nominal_r[3]
            ],
            [-1.0, -1.0, 0.0, 0.0]
        );
        assert!(
            normal.resistance > 0.0
                && normal.resistance.to_bits() == data.nominal_r[4].to_bits()
                && data.resistance_only[&MaterialId(4)]
        );
        let text = std::str::from_utf8(&text).unwrap();
        let positions: Option<Vec<_>> = [
            "GetMaterialData: WindowMaterial:Glazing = NO MASS, duplicate name.",
            "Window glass material WRAPPED has Conductivity = 0.0",
            "conductivity not > 0.0",
            "young_s_modulus not > 0.0",
        ]
        .map(|needle| text.find(needle))
        .into_iter()
        .collect();
        assert!(
            positions.is_some_and(|p| p.windows(2).all(|pair| pair[0] < pair[1]))
                && ["No mass", "WRAPPED", "Z NORMAL"]
                    .iter()
                    .all(|name| !input.is_object_unused("WindowMaterial:Glazing", name))
        );
    }

    // Two fresh, fully bound Refraction contexts: exact IR equality succeeds
    // through diagnostic IO, or stops at its first real Severe write.
    let inputs = ep_model::WindowGlazingRefractionExtinctionMaterial {
        thickness_m: 0.003,
        solar_index_of_refraction: 1.5,
        solar_extinction_coefficient_per_m: 20.0,
        visible_index_of_refraction: 1.7,
        visible_extinction_coefficient_per_m: 100.0,
        infrared_transmittance_at_normal_incidence: 0.0,
        infrared_hemispherical_emissivity: 0.84,
        conductivity_w_per_m_k: 0.9,
        dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
        solar_diffusing: false,
    };
    for fail_first in [false, true] {
        let (mut model, mut raw) = fixture(true, 0.15);
        let id = MaterialId(3);
        model.materials.push(Material {
            id,
            name: NormalizedName::new("IR EQUALITY"),
            definition: MaterialDefinition::WindowGlazingRefractionExtinction(
                ep_model::WindowGlazingRefractionExtinctionMaterial {
                    infrared_transmittance_at_normal_incidence: 0.5,
                    infrared_hemispherical_emissivity: 0.5,
                    solar_diffusing: true,
                    ..inputs
                },
            ),
        });
        model
            .ctf_report_names
            .materials
            .push((id, "IR EQUALITY".into()));
        raw.refraction.push(ProjectInputObject {
            object_name: "IR EQUALITY".into(),
            idf_max_fields: None,
            fields: BTreeMap::from([
                ("thickness".into(), Value::Number(0.003)),
                ("solar_index_of_refraction".into(), Value::Number(1.5)),
                ("solar_extinction_coefficient".into(), Value::Number(20.0)),
                ("visible_index_of_refraction".into(), Value::Number(1.7)),
                (
                    "visible_extinction_coefficient".into(),
                    Value::Number(100.0),
                ),
                (
                    "infrared_transmittance_at_normal_incidence".into(),
                    Value::Number(0.5),
                ),
                (
                    "infrared_hemispherical_emissivity".into(),
                    Value::Number(0.5),
                ),
                ("solar_diffusing".into(), Value::String("Yes".into())),
            ]),
        });
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            (
                "WindowMaterial:Glazing:RefractionExtinctionMethod",
                &raw.refraction,
                2,
                9,
            ),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        let mut broken = BrokenDiagnostic;
        let result;
        {
            let stream: &mut dyn std::io::Write = if fail_first { &mut broken } else { &mut text };
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
                curves: None,
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

            result = data.get_material_data(&raw, &mut owners);
        }
        let row = &data.simple_glazing[&id];
        assert_eq!(
            (
                result.is_err(),
                errors_found,
                state.tracking.total_severe_errors,
                state.tracking.total_warning_errors
            ),
            (fail_first, true, 1, 0)
        );
        assert!(
            row.resistance > 0.0
                && row.resistance.to_bits() == data.nominal_r[3].to_bits()
                && row.transmittance > 0.0
                && row.thermal_transmittance == 0.5
                && row.thermal_back == 0.5
                && data.resistance_only[&id]
        );
        assert_eq!(row.solar_diffusing, !fail_first);
        let text = std::str::from_utf8(&text).unwrap();
        assert!(if fail_first {
            text.is_empty()
        } else {
            text.find("GetMaterialData: WindowMaterial:Glazing:RefractionExtinctionMethod = IR EQUALITY").zip(text.find("infrared_transmittance_at_normal_incidence + infrared_hemispherical_emissivity not < 1.0")).is_some_and(|(a,b)|a<b)
        });
        assert!(!input.is_object_unused(
            "WindowMaterial:Glazing:RefractionExtinctionMethod",
            "IR EQUALITY"
        ));
    }
    // Fourth fresh, fully bound context. The duplicate itself creates the
    // shared error; nonpositive K adds none, and later normal R still writes.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        let original = "18446744073709551615".parse::<f64>().unwrap();
        let wrapped = ep_model::classify_source_json_number("18446744073709551615")
            .unwrap()
            .as_f64();
        model.materials.extend(
            [
                (
                    "WRAPPED K",
                    MaterialId(3),
                    ep_model::WindowGlazingRefractionExtinctionMaterial {
                        thickness_m: original,
                        conductivity_w_per_m_k: original,
                        ..inputs
                    },
                ),
                ("Z NORMAL", MaterialId(4), inputs),
            ]
            .map(|(name, id, value)| Material {
                id,
                name: NormalizedName::new(name),
                definition: MaterialDefinition::WindowGlazingRefractionExtinction(value),
            }),
        );
        model.ctf_report_names.materials.extend([
            (MaterialId(3), "WRAPPED K".into()),
            (MaterialId(4), "Z NORMAL".into()),
        ]);
        raw.refraction = ["No mass", "WRAPPED K", "Z NORMAL"]
            .map(|name| ProjectInputObject {
                object_name: name.into(),
                idf_max_fields: None,
                fields: BTreeMap::from([
                    ("thickness".into(), Value::Number(0.003)),
                    ("solar_index_of_refraction".into(), Value::Number(1.5)),
                    ("solar_extinction_coefficient".into(), Value::Number(20.0)),
                    ("visible_index_of_refraction".into(), Value::Number(1.7)),
                    (
                        "visible_extinction_coefficient".into(),
                        Value::Number(100.0),
                    ),
                ]),
            })
            .into();
        raw.refraction[1]
            .fields
            .insert("thickness".into(), Value::Number(wrapped));
        raw.refraction[1]
            .fields
            .insert("conductivity".into(), Value::Number(wrapped));
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            (
                "WindowMaterial:Glazing:RefractionExtinctionMethod",
                &raw.refraction,
                2,
                9,
            ),
        ];
        let definitions = families
            .iter()
            .map(|(family, _, a, n)| SourceObjectDefinition {
                object_type: (*family).into(),
                alpha_fields: *a,
                numeric_fields: *n,
                alpha_extensions: 0,
                numeric_extensions: 0,
                extension: None,
            })
            .collect();
        let shapes = families
            .into_iter()
            .map(|(family, rows, _, _)| {
                (
                    family.into(),
                    SourceInputFamily::Objects(
                        rows.iter()
                            .map(|row| SourceObjectShape {
                                object_name: row.object_name.clone(),
                                field_sizes: row
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
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        {
            let stream: &mut dyn std::io::Write = &mut text;
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
                curves: None,
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

            data.get_material_data(&raw, &mut owners).unwrap();
        }
        assert_eq!(
            (
                errors_found,
                state.tracking.total_severe_errors,
                state.tracking.total_warning_errors
            ),
            (true, 1, 0)
        );
        assert_eq!(
            (
                data.material_map["NO MASS"],
                data.material_map["WRAPPED K"],
                data.material_map["Z NORMAL"],
                data.materials.len()
            ),
            (2, 4, 5, 5)
        );
        let wrapped = &data.simple_glazing[&MaterialId(3)];
        let normal = &data.simple_glazing[&MaterialId(4)];
        assert_eq!(
            [
                wrapped.thickness,
                wrapped.conductivity,
                wrapped.resistance,
                data.nominal_r[3]
            ],
            [-1.0, -1.0, 0.0, 0.0]
        );
        assert!(
            normal.resistance > 0.0
                && normal.resistance.to_bits() == data.nominal_r[4].to_bits()
                && data.resistance_only[&MaterialId(4)]
        );
        let text = std::str::from_utf8(&text).unwrap();
        assert!(text.matches("GetMaterialData: WindowMaterial:Glazing:RefractionExtinctionMethod = NO MASS, duplicate name.").count()==1 && !text.contains("Conductivity") && ["No mass","WRAPPED K","Z NORMAL"].iter().all(|name|!input.is_object_unused("WindowMaterial:Glazing:RefractionExtinctionMethod",name)));
    }

    // One successful-IO context reaches both count errors and every integrity
    // predicate, then continues into the SAME Material owners with prior errors.
    {
        let (mut model, mut raw) = fixture(true, 0.15);
        let fixed_names = [
            "wavelength_1",
            "transmittance_1",
            "front_reflectance_1",
            "back_reflectance_1",
            "wavelength_2",
            "transmittance_2",
            "front_reflectance_2",
            "back_reflectance_2",
            "wavelength_3",
            "transmittance_3",
            "front_reflectance_3",
            "back_reflectance_3",
            "wavelength_4",
            "transmittance_4",
            "front_reflectance_4",
            "back_reflectance_4",
        ];
        let integrity: BTreeMap<_, _> = fixed_names
            .iter()
            .zip([
                0.5, 1.1, -0.1, 1.1, 0.5, 0.5, 0.6, 0.6, 4.1, 0.2, 0.2, 0.2, 0.05, 0.2, 0.2, 0.2,
            ])
            .map(|(name, value)| ((*name).into(), Value::Number(value)))
            .collect();
        let bulk_fixed: BTreeMap<_, _> = fixed_names
            .iter()
            .zip([
                0.1, 0.2, 0.3, 0.4, 0.2, 0.2, 0.3, 0.4, 0.3, 0.2, 0.3, 0.4, 0.4, 0.2, 0.3, 0.4,
            ])
            .map(|(name, value)| ((*name).into(), Value::Number(value)))
            .collect();
        let extensions = (4..801)
            .map(|index| {
                BTreeMap::from([
                    (
                        "wavelength".into(),
                        Value::Number(0.1 + f64::from(index) * 0.004),
                    ),
                    ("transmittance".into(), Value::Number(0.2)),
                    ("front_reflectance".into(), Value::Number(0.3)),
                    ("back_reflectance".into(), Value::Number(0.4)),
                ])
            })
            .collect();
        raw.spectral_data = [
            (
                "A PARTIAL",
                BTreeMap::from([("wavelength_1".into(), Value::Number(0.5))]),
                Vec::new(),
            ),
            ("B OVER CAP", bulk_fixed, extensions),
            ("C INTEGRITY", integrity, vec![BTreeMap::new()]),
        ]
        .map(|(name, fields, extensions)| SourceGlazingSpectralInput {
            object: ProjectInputObject {
                object_name: name.into(),
                fields,
                idf_max_fields: None,
            },
            extensions,
        })
        .into();

        for (name, reference) in [
            ("D BLANK", ""),
            ("E MISSING", "not found"),
            ("F NORMAL", "c integrity"),
        ] {
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material {
                id,
                name: NormalizedName::new(name),
                definition: MaterialDefinition::WindowGlazingSpectral(
                    ep_model::WindowGlazingSpectralMaterial {
                        thickness_m: 0.003,
                        solar_transmittance_at_normal_incidence: 0.0,
                        front_side_solar_reflectance_at_normal_incidence: 0.0,
                        back_side_solar_reflectance_at_normal_incidence: 0.0,
                        visible_transmittance_at_normal_incidence: 0.0,
                        front_side_visible_reflectance_at_normal_incidence: 0.0,
                        back_side_visible_reflectance_at_normal_incidence: 0.0,
                        infrared_transmittance_at_normal_incidence: 0.0,
                        front_side_infrared_hemispherical_emissivity: 0.84,
                        back_side_infrared_hemispherical_emissivity: 0.84,
                        conductivity_w_per_m_k: 0.9,
                        dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
                        solar_diffusing: false,
                        youngs_modulus_pa: 72_000_000_000.0,
                        poissons_ratio: 0.22,
                        spectral_data: None,
                    },
                ),
            });
            model.ctf_report_names.materials.push((id, name.into()));
            raw.glazing.push(ProjectInputObject {
                object_name: name.into(),
                idf_max_fields: None,
                fields: BTreeMap::from([
                    ("optical_data_type".into(), Value::String("Spectral".into())),
                    (
                        "window_glass_spectral_data_set_name".into(),
                        Value::String(reference.into()),
                    ),
                    ("thickness".into(), Value::Number(0.003)),
                ]),
            });
        }

        // Every context owns real family definitions, every extension position,
        // input unused accounting and the same shared scratch/backend/Material owner.
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let mut definitions: Vec<_> = families
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
        definitions.push(SourceObjectDefinition {
            object_type: "MaterialProperty:GlazingSpectralData".into(),
            alpha_fields: 1,
            numeric_fields: 16,
            alpha_extensions: 0,
            numeric_extensions: 4,
            extension: Some("extensions".into()),
        });
        let mut shapes: BTreeMap<_, _> = families
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
                                    .map(|key| (key.clone(), 1))
                                    .collect(),
                            })
                            .collect(),
                    ),
                )
            })
            .collect();
        shapes.insert(
            "MaterialProperty:GlazingSpectralData".into(),
            SourceInputFamily::Objects(
                raw.spectral_data
                    .iter()
                    .map(|row| {
                        let mut sizes: BTreeMap<_, _> = row
                            .object
                            .fields
                            .keys()
                            .map(|key| (key.clone(), 1))
                            .collect();
                        if !row.extensions.is_empty()
                            || row.object.object_name == "B FIXED EMPTY EXT"
                        {
                            sizes.insert("extensions".into(), row.extensions.len());
                        }
                        SourceObjectShape {
                            object_name: row.object.object_name.clone(),
                            field_sizes: sizes,
                        }
                    })
                    .collect(),
            ),
        );
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        {
            let stream: &mut dyn std::io::Write = &mut text;

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
                curves: None,
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

            assert_eq!(
                raw.get_spectral_data_item(&raw.spectral_data[2], &mut owners)
                    .unwrap(),
                (1, 20)
            );
            assert!(
                owners.shortcut.numeric_args[16..20] == [0.0; 4]
                    && owners.shortcut.numeric_field_blanks[16..20] == [true; 4]
                    && owners.shortcut.numeric_field_names[16..20]
                        == [
                            "wavelength",
                            "transmittance",
                            "front_reflectance",
                            "back_reflectance"
                        ]
            );
            data.get_glass_spectral_data(&raw, &mut owners).unwrap();
            data.get_material_data(&raw, &mut owners).unwrap();
        }
        let rows = data.spectral_data.as_ref().unwrap();
        assert!(rows[..2].iter().all(|row| row.num_of_wavelengths == 0
            && row.wavelengths.is_none()
            && row.transmittance.is_none()
            && row.front_reflectance.is_none()
            && row.back_reflectance.is_none()));
        assert_eq!(
            (
                rows[0].name.as_str(),
                rows[1].name.as_str(),
                rows[2].num_of_wavelengths
            ),
            ("A PARTIAL", "B OVER CAP", 5)
        );
        assert_eq!(
            [
                rows[2].wavelengths.as_ref().unwrap()[4],
                rows[2].transmittance.as_ref().unwrap()[4],
                rows[2].front_reflectance.as_ref().unwrap()[4],
                rows[2].back_reflectance.as_ref().unwrap()[4]
            ],
            [0.0, 0.001, 0.0, 0.0]
        );
        let text = std::str::from_utf8(&text).unwrap();
        assert!(
            text.find("1 of items in data set is not a multiple of 4")
                .zip(text.find("More than 800 entries in set (3204)"))
                .is_some_and(|(a, b)| a < b)
        );
        assert!(
            [
                "Wavelengths not in increasing order",
                "not in the range 0.1 to 4.0",
                "A transmittance is > 1.0",
                "RhoF value",
                "RhoB value",
                "Transmittance + reflectance) > 1.0"
            ]
            .iter()
            .all(|message| text.contains(message))
        );
        assert!(
            text.find("invalid set.")
                .zip(text.find("Wavelengths not in increasing order"))
                .is_some_and(|(a, b)| a < b)
                && text.contains("window_glass_spectral_data_set_name is blank")
                && text.contains("NOT FOUND, item not found")
        );
        assert_eq!(
            [
                data.simple_glazing[&MaterialId(3)].spectral_data_pointer,
                data.simple_glazing[&MaterialId(4)].spectral_data_pointer,
                data.simple_glazing[&MaterialId(5)].spectral_data_pointer
            ],
            [0, 0, 3]
        );
        assert!(
            errors_found
                && state.tracking.total_warning_errors == 1
                && state.tracking.total_severe_errors > 1
                && data.simple_glazing[&MaterialId(5)].resistance > 0.0
                && data.nominal_r[5].to_bits()
                    == data.simple_glazing[&MaterialId(5)].resistance.to_bits()
                && raw.spectral_data.iter().all(|row| !input.is_object_unused(
                    "MaterialProperty:GlazingSpectralData",
                    &row.object.object_name
                ))
        );
    }
    // Two bounded fresh contexts: count-custom IO stops before its flag;
    // integrity IO stops after full array fill and its genuine flag write.
    for integrity_failure in [false, true] {
        let (model, mut raw) = fixture(true, 0.15);
        let fixed_names = [
            "wavelength_1",
            "transmittance_1",
            "front_reflectance_1",
            "back_reflectance_1",
            "wavelength_2",
            "transmittance_2",
            "front_reflectance_2",
            "back_reflectance_2",
            "wavelength_3",
            "transmittance_3",
            "front_reflectance_3",
            "back_reflectance_3",
            "wavelength_4",
            "transmittance_4",
            "front_reflectance_4",
            "back_reflectance_4",
        ];
        let fields = if integrity_failure {
            fixed_names
                .iter()
                .zip([
                    0.5, 0.2, 0.2, 0.2, 0.5, 0.3, 0.2, 0.2, 0.7, 0.4, 0.2, 0.2, 0.8, 0.5, 0.2, 0.2,
                ])
                .map(|(name, value)| ((*name).into(), Value::Number(value)))
                .collect()
        } else {
            BTreeMap::from([("wavelength_1".into(), Value::Number(0.5))])
        };
        raw.spectral_data = vec![SourceGlazingSpectralInput {
            object: ProjectInputObject {
                object_name: "IO PREFIX".into(),
                fields,
                idf_max_fields: None,
            },
            extensions: if integrity_failure {
                vec![BTreeMap::new()]
            } else {
                Vec::new()
            },
        }];

        // Every context owns real family definitions, every extension position,
        // input unused accounting and the same shared scratch/backend/Material owner.
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let mut definitions: Vec<_> = families
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
        definitions.push(SourceObjectDefinition {
            object_type: "MaterialProperty:GlazingSpectralData".into(),
            alpha_fields: 1,
            numeric_fields: 16,
            alpha_extensions: 0,
            numeric_extensions: 4,
            extension: Some("extensions".into()),
        });
        let mut shapes: BTreeMap<_, _> = families
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
                                    .map(|key| (key.clone(), 1))
                                    .collect(),
                            })
                            .collect(),
                    ),
                )
            })
            .collect();
        shapes.insert(
            "MaterialProperty:GlazingSpectralData".into(),
            SourceInputFamily::Objects(
                raw.spectral_data
                    .iter()
                    .map(|row| {
                        let mut sizes: BTreeMap<_, _> = row
                            .object
                            .fields
                            .keys()
                            .map(|key| (key.clone(), 1))
                            .collect();
                        if !row.extensions.is_empty()
                            || row.object.object_name == "B FIXED EMPTY EXT"
                        {
                            sizes.insert("extensions".into(), row.extensions.len());
                        }
                        SourceObjectShape {
                            object_name: row.object.object_name.clone(),
                            field_sizes: sizes,
                        }
                    })
                    .collect(),
            ),
        );
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let mut state = error_state();
        let mut scan = ReportScanState::default();
        let (text, mut console, mut eio) = (Vec::<u8>::new(), Vec::new(), Vec::new());
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();

        let mut broken = BrokenDiagnostic;
        let result;
        {
            let stream: &mut dyn std::io::Write = &mut broken;

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
                curves: None,
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

            result = data.get_glass_spectral_data(&raw, &mut owners);
        }
        let row = &data.spectral_data.as_ref().unwrap()[0];
        assert!(
            result.is_err()
                && row.name == "IO PREFIX"
                && text.is_empty()
                && !input.is_object_unused("MaterialProperty:GlazingSpectralData", "IO PREFIX")
        );
        assert_eq!(
            (errors_found, row.num_of_wavelengths),
            (integrity_failure, if integrity_failure { 5 } else { 0 })
        );
        assert!(if integrity_failure {
            [
                &row.wavelengths,
                &row.transmittance,
                &row.front_reflectance,
                &row.back_reflectance,
            ]
            .iter()
            .all(|array| array.as_ref().is_some_and(|array| array.len() == 5))
                && row.wavelengths.as_ref().unwrap()[4] == 0.0
                && row.transmittance.as_ref().unwrap()[4] == 0.001
                && row.transmittance.as_ref().unwrap()[3] == 0.5
        } else {
            [
                &row.wavelengths,
                &row.transmittance,
                &row.front_reflectance,
                &row.back_reflectance,
            ]
            .iter()
            .all(|array| array.is_none())
        });
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_severe_errors
            ),
            if integrity_failure { (0, 1) } else { (1, 0) }
        );
    }
    // Exactly three new Curve contexts: successful diagnostics/fatal, first Severe
    // I/O failure, and late-flag Continue failure. Existing sinks/types are reused.
    for mode in 0..3 {
        use crate::heat_balance::selected_input_processor::{
            SourceInputProcessorShortCut, SourceInputStatisticsInput,
            SourceInputStatisticsValue as S,
        };
        use crate::heat_balance::setup_postludes::{
            SelectedCurves, SelectedSetupServiceError, SourceCurveType,
        };
        let mut first = BTreeMap::from([
            ("coefficient1_constant".into(), S::Number("0.1".into())),
            ("coefficient2_x".into(), S::Number("0.2".into())),
            ("coefficient3_x_2".into(), S::Number("0.3".into())),
            ("coefficient4_y".into(), S::Number("0.4".into())),
            ("coefficient5_y_2".into(), S::Number("0.5".into())),
            ("coefficient6_x_y".into(), S::Number("0.6".into())),
            ("minimum_value_of_x".into(), S::Number("-10.0".into())),
            ("maximum_value_of_x".into(), S::Number("50.0".into())),
            ("minimum_value_of_y".into(), S::Number("-5.0".into())),
            ("maximum_value_of_y".into(), S::Number("25.0".into())),
        ]);
        if mode != 1 {
            first.insert("minimum_value_of_x".into(), S::Number("2.0".into()));
            first.insert("maximum_value_of_x".into(), S::Number("1.0".into()));
            first.insert("minimum_curve_output".into(), S::Number("0.0".into()));
        }
        if mode == 0 {
            first.insert("minimum_value_of_y".into(), S::Number("4.0".into()));
            first.insert("maximum_value_of_y".into(), S::Number("3.0".into()));
        }
        let mut second = first.clone();
        second.remove("coefficient5_y_2");
        second.remove("coefficient6_x_y");
        second.insert("coefficient5_x_y".into(), S::Number("0.5".into()));
        second.insert("coefficient6_x_2_y".into(), S::Number("0.6".into()));
        second.insert("minimum_value_of_x".into(), S::Number("-10.0".into()));
        second.insert("maximum_value_of_x".into(), S::Number("50.0".into()));
        second.insert("minimum_value_of_y".into(), S::Number("-5.0".into()));
        second.insert("maximum_value_of_y".into(), S::Number("25.0".into()));
        let raw_objects: BTreeMap<String, BTreeMap<String, BTreeMap<String, S>>> =
            BTreeMap::from([
                (
                    "Curve:Biquadratic".into(),
                    BTreeMap::from([("A CURVE".into(), first)]),
                ),
                (
                    "Curve:QuadraticLinear".into(),
                    BTreeMap::from([("a curve".into(), second)]),
                ),
            ]);
        let mut input = SelectedInputProcessor::from_admitted_input(
            ["Curve:Biquadratic", "Curve:QuadraticLinear"]
                .into_iter()
                .map(|name| SourceObjectDefinition {
                    object_type: name.into(),
                    alpha_fields: 4,
                    numeric_fields: 12,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                })
                .collect(),
            raw_objects
                .iter()
                .map(|(family, objects)| {
                    (
                        family.clone(),
                        SourceInputFamily::Objects(
                            objects
                                .iter()
                                .map(|(name, fields)| SourceObjectShape {
                                    object_name: name.clone(),
                                    field_sizes: fields
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
        .unwrap();
        input
            .retain_source_input_statistics(SourceInputStatisticsInput {
                object_order: BTreeMap::from([
                    ("Curve:Biquadratic".into(), vec!["A CURVE".into()]),
                    ("Curve:QuadraticLinear".into(), vec!["a curve".into()]),
                ]),
                objects: raw_objects,
            })
            .unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals =
            crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals::default(
            );
        let project = crate::heat_balance::selected_project_data::SelectedProjectData::default();
        let clock = crate::weather::day::WeatherGlobalState::default();
        let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &project.system.developer_flag,
            warmup: &clock.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        };
        let mut curves = SelectedCurves::default();
        let (mut state, mut text, mut console) = (error_state(), Vec::new(), Vec::new());
        let mut broken = BrokenDiagnostic;
        let mut continue_sink = FailContinue { bytes: Vec::new() };
        let error_stream: &mut dyn std::io::Write = match mode {
            0 => &mut text,
            1 => &mut broken,
            _ => &mut continue_sink,
        };
        let result = curves.initialize_state(
            &mut input,
            &mut shortcut,
            &flags,
            &mut SourceErrorBackend {
                state: &mut state,
                error_stream: Some(error_stream),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        );
        assert!(match mode {0=>matches!(&result,Err(SelectedSetupServiceError::Diagnostic(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Fatal(_)))),1=>matches!(&result,Err(SelectedSetupServiceError::Diagnostic(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(_)))),_=>matches!(&result,Err(SelectedSetupServiceError::Backend(_)))});
        assert_eq!(
            (
                curves.get_curve_index("A CURVE"),
                curves.get_curve_index("a curve")
            ),
            (if mode == 0 { 2 } else { 1 }, 0)
        );
        assert_eq!(
            state.tracking.total_severe_errors,
            if mode == 0 { 3 } else { 1 }
        );
        assert_eq!(
            input.is_object_unused("Curve:QuadraticLinear", "a curve"),
            mode == 2
        );
        let retained = curves.get_curve("A CURVE").unwrap();
        assert!(
            retained.num_dims == 2
                && retained.coeff[..6] == [0.1, 0.2, 0.3, 0.4, 0.5, 0.6]
                && retained
                    .input_limits
                    .iter()
                    .all(|l| !l.min_present && !l.max_present)
        );
        assert_eq!(
            (
                retained.curve_type,
                retained.input_limits[0].min,
                retained.input_limits[0].max
            ),
            (
                if mode == 0 {
                    SourceCurveType::QuadraticLinear
                } else {
                    SourceCurveType::BiQuadratic
                },
                if mode == 2 { 2.0 } else { -10.0 },
                if mode == 2 { 1.0 } else { 50.0 }
            )
        );
        if mode == 0 {
            let reached = std::str::from_utf8(&text).unwrap();
            assert!(
                reached
                    .find("minimum_value_of_x")
                    .zip(reached.find("minimum_value_of_y"))
                    .is_some_and(|(a, b)| a < b)
                    && reached
                        .find("duplicate name.")
                        .zip(reached.find("GetCurveInput: Errors found in getting Curve Objects."))
                        .is_some_and(|(a, b)| a < b)
            );
        } else if mode == 1 {
            assert!(text.is_empty() && console.is_empty() && !retained.output_limits.min_present);
        } else {
            assert!(
                std::str::from_utf8(&continue_sink.bytes)
                    .unwrap()
                    .contains("GetCurveInput: For Curve:Biquadratic:")
                    && !std::str::from_utf8(&continue_sink.bytes)
                        .unwrap()
                        .contains(" **   ~~~   ** ")
                    && retained.output_limits.min_present
                    && retained.output_limits.min.to_bits() == 0
            );
        }
    }
    // Exactly three more fresh Angle contexts: successful diagnostics,firstblank IO,firstbounds IO.
    for case in [0, 1, 2] {
        use crate::heat_balance::selected_input_processor::{
            SourceInputStatisticsInput, SourceInputStatisticsValue as S,
        };
        use crate::heat_balance::setup_postludes::SelectedCurves;
        let (mut model, mut raw) = fixture(true, 0.15);
        let glass_rows = match case {
            0 => vec![
                ("A BLANK", ["", "", ""]),
                ("B MISSING", ["MISSING", "MISSING", "MISSING"]),
                ("C TRANS BAD", ["BAD", "GOOD", "GOOD"]),
                ("D FRONT BAD", ["GOOD", "BAD", "GOOD"]),
                ("E BACK BAD", ["GOOD", "GOOD", "BAD"]),
                ("F GOOD", ["GOOD", "GOOD", "GOOD"]),
            ],
            1 => vec![
                ("A BLANK", ["", "GOOD", "GOOD"]),
                ("Z LATER", ["GOOD", "GOOD", "GOOD"]),
            ],
            _ => vec![
                ("A BOUNDS", ["BAD", "GOOD", "GOOD"]),
                ("Z LATER", ["GOOD", "GOOD", "GOOD"]),
            ],
        };
        for (name, refs) in &glass_rows {
            let id = MaterialId(u32::try_from(model.materials.len()).unwrap());
            model.materials.push(Material {
                id,
                name: NormalizedName::new(name),
                definition: MaterialDefinition::WindowGlazingSpectralAndAngle(
                    ep_model::WindowGlazingSpectralAndAngleMaterial {
                        thickness_m: 99.0,
                        solar_transmittance_at_normal_incidence: 0.2,
                        front_side_solar_reflectance_at_normal_incidence: 0.2,
                        back_side_solar_reflectance_at_normal_incidence: 0.2,
                        visible_transmittance_at_normal_incidence: 0.2,
                        front_side_visible_reflectance_at_normal_incidence: 0.2,
                        back_side_visible_reflectance_at_normal_incidence: 0.2,
                        infrared_transmittance_at_normal_incidence: 0.2,
                        front_side_infrared_hemispherical_emissivity: 0.7,
                        back_side_infrared_hemispherical_emissivity: 0.7,
                        conductivity_w_per_m_k: 99.0,
                        dirt_correction_factor_for_solar_and_visible_transmittance: 0.5,
                        solar_diffusing: false,
                        youngs_modulus_pa: 1.0,
                        poissons_ratio: 0.2,
                    },
                ),
            });
            model.ctf_report_names.materials.push((id, (*name).into()));
            let mut fields=BTreeMap::from([
                ("optical_data_type".into(),Value::String("SpectralAndAngle".into())),("thickness".into(),Value::Number(0.003)),
                ("window_glass_spectral_and_incident_angle_transmittance_data_set_table_name".into(),Value::String(refs[0].into())),
                ("window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name".into(),Value::String(refs[1].into())),
                ("window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name".into(),Value::String(refs[2].into())),
            ]);
            if case == 1 && *name == "A BLANK" {
                fields.remove(
                    "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name",
                );
            }
            raw.glazing.push(ProjectInputObject {
                object_name: (*name).into(),
                fields,
                idf_max_fields: None,
            });
        }
        let mut objects: BTreeMap<String, BTreeMap<String, BTreeMap<String, S>>> =
            BTreeMap::from([(
                "Curve:Biquadratic".into(),
                BTreeMap::from([
                    (
                        "BAD".into(),
                        BTreeMap::from([
                            ("coefficient1_constant".into(), S::Number("0.1".into())),
                            ("coefficient2_x".into(), S::Number("0.1".into())),
                            ("coefficient3_x_2".into(), S::Number("0.1".into())),
                            ("coefficient4_y".into(), S::Number("0.1".into())),
                            ("coefficient5_y_2".into(), S::Number("0.1".into())),
                            ("coefficient6_x_y".into(), S::Number("0.1".into())),
                            ("minimum_value_of_x".into(), S::Number("0.000002".into())),
                            ("maximum_value_of_x".into(), S::Number("100.0".into())),
                            ("minimum_value_of_y".into(), S::Number("0.05".into())),
                            ("maximum_value_of_y".into(), S::Number("5.0".into())),
                        ]),
                    ),
                    (
                        "GOOD".into(),
                        BTreeMap::from([
                            ("coefficient1_constant".into(), S::Number("0.1".into())),
                            ("coefficient2_x".into(), S::Number("0.1".into())),
                            ("coefficient3_x_2".into(), S::Number("0.1".into())),
                            ("coefficient4_y".into(), S::Number("0.1".into())),
                            ("coefficient5_y_2".into(), S::Number("0.1".into())),
                            ("coefficient6_x_y".into(), S::Number("0.1".into())),
                            ("minimum_value_of_x".into(), S::Number("0.0".into())),
                            ("maximum_value_of_x".into(), S::Number("90.0".into())),
                            ("minimum_value_of_y".into(), S::Number("0.1".into())),
                            ("maximum_value_of_y".into(), S::Number("4.0".into())),
                        ]),
                    ),
                ]),
            )]);
        let mut object_order: BTreeMap<String, Vec<String>> = BTreeMap::from([(
            "Curve:Biquadratic".into(),
            vec!["BAD".into(), "GOOD".into()],
        )]);
        let families = [
            ("Material", &raw.regular, 2, 7),
            ("Material:NoMass", &raw.no_mass, 2, 4),
            ("Material:AirGap", &raw.air_gaps, 1, 1),
            ("WindowMaterial:Glazing", &raw.glazing, 7, 14),
        ];
        let definitions = [
            ("Curve:Biquadratic", 4, 12),
            ("Curve:QuadraticLinear", 4, 12),
            ("Curve:CubicLinear", 4, 12),
            ("Curve:Bicubic", 4, 16),
            ("Curve:FanPressureRise", 1, 10),
        ]
        .into_iter()
        .map(|(f, a, n)| SourceObjectDefinition {
            object_type: f.into(),
            alpha_fields: a,
            numeric_fields: n,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        })
        .chain(families.iter().map(|(f, _, a, n)| SourceObjectDefinition {
            object_type: (*f).into(),
            alpha_fields: *a,
            numeric_fields: *n,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        }))
        .collect();
        for (family, rows, _, _) in families {
            object_order.insert(
                family.into(),
                rows.iter().map(|r| r.object_name.clone()).collect(),
            );
            objects.insert(
                family.into(),
                rows.iter()
                    .map(|r| {
                        (
                            r.object_name.clone(),
                            r.fields
                                .iter()
                                .map(|(k, v)| {
                                    (
                                        k.clone(),
                                        match v {
                                            Value::String(v) => S::String(v.clone()),
                                            Value::Number(v) => S::Number(v.to_string()),
                                        },
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            );
        }
        let shapes = objects
            .iter()
            .map(|(family, rows)| {
                (
                    family.clone(),
                    SourceInputFamily::Objects(
                        object_order[family]
                            .iter()
                            .map(|name| SourceObjectShape {
                                object_name: name.clone(),
                                field_sizes: rows[name].keys().map(|k| (k.clone(), 1)).collect(),
                            })
                            .collect(),
                    ),
                )
            })
            .collect();
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, shapes).unwrap();
        input
            .retain_source_input_statistics(SourceInputStatisticsInput {
                objects,
                object_order,
            })
            .unwrap();
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input).unwrap();
        let globals = SelectedHeatBalanceGlobals {
            doing_sizing: true,
            ..Default::default()
        };
        let clock = crate::weather::day::WeatherGlobalState::default();
        let developer = false;
        let flags = SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &developer,
            warmup: &clock.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        };
        let mut state = error_state();
        let (mut text, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut broken = BrokenDiagnostic;
        let mut scan = ReportScanState::default();
        let mut errors_found = false;
        let mut data = SelectedMaterialData::default();
        let mut curves = SelectedCurves::default();
        let outcome = {
            let stream: &mut dyn std::io::Write = if case == 0 { &mut text } else { &mut broken };
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
            curves
                .initialize_state(&mut input, &mut shortcut, &flags, &mut backend)
                .unwrap();
            data.get_material_data(
                &raw,
                &mut MaterialDataOwners {
                    curves: Some(&curves),
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
        assert_eq!(outcome.is_err(), case != 0);
        assert!(
            errors_found
                && data.simple_glazing[&MaterialId(3)].transmittance.to_bits() == 0
                && data.simple_glazing[&MaterialId(3)]
                    .thermal_transmittance
                    .to_bits()
                    == 0
        );
        if case == 0 {
            let log = std::str::from_utf8(&text).unwrap();
            let first = &data.simple_glazing[&MaterialId(3)];
            let missing = &data.simple_glazing[&MaterialId(4)];
            assert_eq!(
                (
                    first.spec_ang_trans_curve,
                    first.spec_ang_front_curve,
                    first.spec_ang_back_curve,
                    missing.spec_ang_trans_curve,
                    missing.spec_ang_front_curve,
                    missing.spec_ang_back_curve
                ),
                (0, 0, 0, 0, 0, 0)
            );
            let last = &data.simple_glazing[&MaterialId(8)];
            let good = curves.get_curve("GOOD").unwrap().num;
            let bad = curves.get_curve("BAD").unwrap().num;
            assert_eq!(
                (
                    data.simple_glazing[&MaterialId(7)].spec_ang_back_curve,
                    last.spec_ang_trans_curve,
                    last.spec_ang_front_curve,
                    last.spec_ang_back_curve
                ),
                (bad, good, good, good)
            );
            assert_eq!(
                (
                    state.tracking.total_severe_errors,
                    state.tracking.total_severe_errors_during_sizing
                ),
                (18, 18)
            );
            let minimum = log.find("requires the minimum value = 0.0").unwrap();
            let maximum = log.find("requires the maximum value = 90.0").unwrap();
            let wave_min = log.find("requires the minimum value = 0.1 micron").unwrap();
            let wave_max = log
                .find("requires the maximum value = 4.0 microns")
                .unwrap();
            assert!(
                minimum < maximum
                    && maximum < wave_min
                    && wave_min < wave_max
                    && log.find("cannot be empty").unwrap() < log.find("item not found").unwrap()
                    && log.find("item not found").unwrap() < minimum
            );
            assert_eq!(log.matches("requires the minimum value = 0.0").count(), 3);
            assert!(log.contains("window_glass_spectral_and_incident_angle_transmittance_data_set_table_name requires the minimum value = 0.0 in the entered table name=GOOD") && !log.contains("front_reflectance_data_set_table_name requires") && last.resistance>0.0 && !input.is_object_unused("WindowMaterial:Glazing","F GOOD"));
        } else {
            let material = &data.simple_glazing[&MaterialId(3)];
            assert_eq!(
                (
                    state.tracking.total_severe_errors,
                    state.tracking.total_severe_errors_during_sizing,
                    data.materials.len()
                ),
                (1, 1, 4)
            );
            assert_eq!(
                (
                    material.spec_ang_trans_curve,
                    material.spec_ang_front_curve,
                    material.spec_ang_back_curve
                ),
                (
                    if case == 1 {
                        0
                    } else {
                        curves.get_curve("BAD").unwrap().num
                    },
                    0,
                    0
                )
            );
            assert!(
                !input.is_object_unused("WindowMaterial:Glazing", glass_rows[0].0)
                    && input.is_object_unused("WindowMaterial:Glazing", "Z LATER")
            );
        }
        assert!(
            curves
                .get_curve("GOOD")
                .unwrap()
                .inputs
                .iter()
                .all(|v| v.to_bits() == 0)
                && curves.get_curve("BAD").unwrap().output.to_bits() == 0
        );
    }
}
