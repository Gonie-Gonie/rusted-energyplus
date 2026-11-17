use super::super::{CompileResult, ObjectCoverageStatus, compile_raw_model, typed_coverage_status};
use ep_model::{
    MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    WindowShadeEquivalentLayerSideOpticalProperties,
};
use ep_raw_model::parse_epjson_str;
use std::collections::BTreeSet;

const OBJECT_TYPE: &str = "WindowMaterial:Drape:EquivalentLayer";

fn has_diagnostic(
    result: &CompileResult,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

#[test]
fn drape_equivalent_layer_materializes_all_inputs_defaults_and_source_quirks()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Shade Before Drape": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
                }
            },
            "WindowMaterial:Drape:EquivalentLayer": {
                "Z Full EQL Drape": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.15,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.10,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.70,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.12,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.20,
                    "drape_beam_beam_visible_transmittance": 0.21,
                    "drape_beam_diffuse_visible_transmittance": 0.22,
                    "drape_beam_diffuse_visible_reflectance": 0.23,
                    "drape_material_infrared_transmittance": 0.10,
                    "front_side_drape_material_infrared_emissivity": 0.90,
                    "back_side_drape_material_infrared_emissivity": 0.95,
                    "width_of_pleated_fabric": 0.03,
                    "length_of_pleated_fabric": 0.05
                },
                "A Default EQL Drape": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.10,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.20,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.30,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.40
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer drapes"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "SHADE BEFORE DRAPE",
            "A DEFAULT EQL DRAPE",
            "Z FULL EQL DRAPE"
        ],
        "equivalent-layer drapes must follow equivalent-layer shades"
    );

    let material = &model.materials[2];
    assert_eq!(material.kind(), MaterialKind::WindowDrapeEquivalentLayer);
    assert_eq!(material.family(), MaterialFamily::EquivalentLayer);
    assert!(material.as_opaque().is_none());
    assert!(material.as_window_shade_equivalent_layer().is_none());
    assert_eq!(material.thickness_m(), None);
    assert_eq!(material.conductivity_w_per_m_k(), None);
    assert_eq!(material.density_kg_per_m3(), None);
    assert_eq!(material.specific_heat_j_per_kg_k(), None);
    assert_eq!(material.thermal_resistance(), None);
    assert_eq!(material.heat_capacity_per_area(), None);
    assert_eq!(material.thermal_absorptance(), None);
    assert_eq!(material.solar_absorptance(), None);
    assert_eq!(material.visible_absorptance(), None);
    let drape = material
        .as_window_drape_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer drape payload"))?;
    assert_eq!(drape.roughness, MaterialSurfaceRoughness::MediumRough);
    assert!(drape.is_resistance_only());
    assert_eq!(drape.nominal_thermal_resistance_m2_k_per_w(), None);
    assert_eq!(
        drape.front_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.15,
            beam_diffuse_transmittance: 0.10,
            beam_diffuse_reflectance: 0.12,
        }
    );
    assert_eq!(
        drape.back_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.15,
            beam_diffuse_transmittance: 0.70,
            beam_diffuse_reflectance: 0.20,
        },
        "EnergyPlus 26.1 does not validate the back-side solar sum"
    );
    assert_eq!(
        drape.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.21,
            beam_diffuse_transmittance: 0.22,
            beam_diffuse_reflectance: 0.23,
        }
    );
    assert_eq!(
        drape.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default(),
        "EnergyPlus 26.1 leaves all back-side visible TAR slots at zero"
    );
    assert_eq!(drape.infrared_transmittance, 0.10);
    assert_eq!(drape.front_infrared_emissivity, 0.90);
    assert_eq!(drape.back_infrared_emissivity, 0.95);
    assert_eq!(drape.front_thermal_absorptance, 0.90);
    assert_eq!(drape.back_thermal_absorptance, 0.95);
    assert_eq!(drape.thermal_transmittance, 0.10);
    assert_eq!(drape.pleated_width_m, 0.03);
    assert_eq!(drape.pleated_length_m, 0.05);
    assert!(drape.is_pleated());

    let default = model.materials[1]
        .as_window_drape_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing defaulted equivalent-layer drape"))?;
    assert_eq!(default.front_solar.beam_beam_transmittance, 0.0);
    assert_eq!(default.back_solar.beam_beam_transmittance, 0.0);
    assert_eq!(
        default.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        default.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(default.infrared_transmittance, 0.05);
    assert_eq!(default.front_infrared_emissivity, 0.87);
    assert_eq!(default.back_infrared_emissivity, 0.87);
    assert_eq!(default.pleated_width_m, 0.0);
    assert_eq!(default.pleated_length_m, 0.0);
    assert!(!default.is_pleated());
    for (field, value) in [
        (
            "drape_beam_beam_solar_transmittance_at_normal_incidence",
            "0.0",
        ),
        ("drape_beam_beam_visible_transmittance", "0.0"),
        ("drape_beam_diffuse_visible_transmittance", "0.0"),
        ("drape_beam_diffuse_visible_reflectance", "0.0"),
        ("drape_material_infrared_transmittance", "0.05"),
        ("front_side_drape_material_infrared_emissivity", "0.87"),
        ("back_side_drape_material_infrared_emissivity", "0.87"),
        ("width_of_pleated_fabric", "0.0"),
        ("length_of_pleated_fabric", "0.0"),
    ] {
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == OBJECT_TYPE
                && default.object_name == "A Default EQL Drape"
                && default.field == field
                && default.value == value
        }));
    }

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer drape coverage"))?;
    assert_eq!(coverage.object_count, 2);
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

    // SourceB original-schema qualification is separate from the three Native joint sums.
    {
        use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue};
        let source_raw = ep_raw_model::parse_epjson_str_for_source(
            r#"{
    "WindowMaterial:Drape:EquivalentLayer": {
        "\u000bA DEFAULT DRAPE\u00a0": {
            "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
            "back_side_drape_beam_diffuse_solar_transmittance": 0.15,
            "front_side_drape_beam_diffuse_solar_reflectance": 0.2,
            "back_side_drape_beam_diffuse_solar_reflectance": 0.25,
            "name": false,
            "idf_order": [],
            "metadata": null
        },
        " Z FULL DRAPE ": {
            "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.1,
            "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
            "back_side_drape_beam_diffuse_solar_transmittance": 0.15,
            "front_side_drape_beam_diffuse_solar_reflectance": 0.2,
            "back_side_drape_beam_diffuse_solar_reflectance": 0.25,
            "drape_beam_beam_visible_transmittance": 0.1,
            "drape_beam_diffuse_visible_transmittance": 0.2,
            "drape_beam_diffuse_visible_reflectance": 0.3,
            "drape_material_infrared_transmittance": 0.04,
            "front_side_drape_material_infrared_emissivity": 0.8,
            "back_side_drape_material_infrared_emissivity": 0.75,
            "width_of_pleated_fabric": 0.03,
            "length_of_pleated_fabric": 0.05,
            "name": null,
            "idf_order": false,
            "idf_max_fields": {
                "ignored": true
            },
            "idf_max_extensible_fields": [],
            "extra_string": "unconsumed",
            "extra_number": 18446744073709551615,
            "metadata": {
                "retained": [
                    null,
                    true,
                    1.25,
                    "mixed"
                ]
            }
        }
    }
}"#,
        )?;
        let source = super::super::compile_raw_model_for_source(&source_raw);
        assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
        let source_model = source.model.as_ref().expect("two Source EQL drapes");
        let default_id = source_model
            .material_names
            .resolve("\u{b}A DEFAULT DRAPE\u{a0}")
            .expect("untrimmed default name");
        let full_id = source_model
            .material_names
            .resolve(" Z FULL DRAPE ")
            .expect("untrimmed full name");
        assert!(
            default_id == ep_model::MaterialId(0)
                && default_id != full_id
                && source_model.material_names.is_source_exact_namespace()
                && source_model
                    .material_names
                    .resolve("A DEFAULT DRAPE")
                    .is_none()
                && source_model.materials.len() == 2
                && source_model.ctf_report_names.materials
                    == vec![
                        (default_id, "\u{b}A DEFAULT DRAPE\u{a0}".into()),
                        (full_id, " Z FULL DRAPE ".into())
                    ]
                && source_model.materials[0].kind() == MaterialKind::WindowDrapeEquivalentLayer
                && source_model.materials[0].family() == MaterialFamily::EquivalentLayer
        );
        let source_default = source_model.materials[0]
            .as_window_drape_equivalent_layer()
            .expect("default Source payload");
        let source_full = source_model.materials[1]
            .as_window_drape_equivalent_layer()
            .expect("full Source payload");
        assert!(
            source_default.front_solar.beam_beam_transmittance.to_bits() == 0.0_f64.to_bits()
                && source_default.front_visible
                    == WindowShadeEquivalentLayerSideOpticalProperties::default()
                && source_default.back_visible
                    == WindowShadeEquivalentLayerSideOpticalProperties::default()
                && source_default.infrared_transmittance.to_bits() == 0.05_f64.to_bits()
                && source_default.front_infrared_emissivity.to_bits() == 0.87_f64.to_bits()
                && source_default.back_infrared_emissivity.to_bits() == 0.87_f64.to_bits()
                && source_full.front_solar
                    == WindowShadeEquivalentLayerSideOpticalProperties {
                        beam_beam_transmittance: 0.1,
                        beam_diffuse_transmittance: 0.1,
                        beam_diffuse_reflectance: 0.2
                    }
                && source_full.back_solar
                    == WindowShadeEquivalentLayerSideOpticalProperties {
                        beam_beam_transmittance: 0.1,
                        beam_diffuse_transmittance: 0.15,
                        beam_diffuse_reflectance: 0.25
                    }
                && source_full.front_visible
                    == WindowShadeEquivalentLayerSideOpticalProperties {
                        beam_beam_transmittance: 0.1,
                        beam_diffuse_transmittance: 0.2,
                        beam_diffuse_reflectance: 0.3
                    }
                && source_full.back_visible
                    == WindowShadeEquivalentLayerSideOpticalProperties::default()
                && [
                    source_full.infrared_transmittance,
                    source_full.front_infrared_emissivity,
                    source_full.back_infrared_emissivity,
                    source_full.front_thermal_absorptance,
                    source_full.back_thermal_absorptance,
                    source_full.thermal_transmittance
                ]
                .map(f64::to_bits)
                    == [0.04_f64, 0.8, 0.75, 0.8, 0.75, 0.04].map(f64::to_bits)
                && source_default.pleated_width_m.to_bits() == 0.0_f64.to_bits()
                && source_default.pleated_length_m.to_bits() == 0.0_f64.to_bits()
                && !source_default.is_pleated()
                && source_full.pleated_width_m.to_bits() == 0.03_f64.to_bits()
                && source_full.pleated_length_m.to_bits() == 0.05_f64.to_bits()
                && source_full.is_pleated()
                && source_full.roughness == MaterialSurfaceRoughness::MediumRough
                && source_full.is_resistance_only()
                && source_model
                    .materials
                    .iter()
                    .all(|row| row.thickness_m().is_none()
                        && row.conductivity_w_per_m_k().is_none()
                        && row.thermal_resistance().is_none())
        );
        let full_raw = &source_raw.objects[&ObjectType(OBJECT_TYPE.into())]
            [&ObjectName(" Z FULL DRAPE ".into())];
        assert!(
            matches!(
                full_raw.fields.get(&FieldName("name".into())),
                Some(RawValue::Null)
            ) && matches!(
                full_raw.fields.get(&FieldName("idf_order".into())),
                Some(RawValue::Bool(false))
            ) && matches!(
                full_raw.fields.get(&FieldName("idf_max_fields".into())),
                Some(RawValue::Object(_))
            ) && matches!(
                full_raw
                    .fields
                    .get(&FieldName("idf_max_extensible_fields".into())),
                Some(RawValue::Array(_))
            ) && matches!(
                full_raw.fields.get(&FieldName("metadata".into())),
                Some(RawValue::Object(_))
            ) && matches!(full_raw.fields.get(&FieldName("extra_string".into())),Some(RawValue::String(value)) if value == "unconsumed")
                && matches!(full_raw.fields.get(&FieldName("extra_number".into())),Some(RawValue::Number(token)) if token == "18446744073709551615")
        );
        let legacy = compile_raw_model(&source_raw);
        assert!(
            !legacy.has_errors()
                && !legacy
                    .model
                    .as_ref()
                    .expect("held Legacy typed projection")
                    .material_names
                    .is_source_exact_namespace()
                && legacy
                    .model
                    .as_ref()
                    .expect("Legacy full payload")
                    .materials[1]
                    .definition
                    == source_model.materials[1].definition
        );
        let mut referenced = source_raw.clone();
        referenced
            .objects
            .get_mut(&ObjectType(OBJECT_TYPE.into()))
            .expect("EQL family")
            .get_mut(&ObjectName(" Z FULL DRAPE ".into()))
            .expect("EQL row")
            .fields
            .extend([
                (
                    FieldName("drape_beam_beam_solar_transmittance_at_normal_incidence".into()),
                    RawValue::Number("0.2".into()),
                ),
                (
                    FieldName("front_side_drape_beam_diffuse_solar_transmittance".into()),
                    RawValue::Number("0.4".into()),
                ),
                (
                    FieldName("front_side_drape_beam_diffuse_solar_reflectance".into()),
                    RawValue::Number("0.5".into()),
                ),
            ]);
        referenced.objects.insert(
            ObjectType("Construction".into()),
            std::collections::BTreeMap::from([(
                ObjectName("EQL ACTIVE".into()),
                ep_raw_model::RawObject {
                    fields: std::collections::BTreeMap::from([(
                        FieldName("outside_layer".into()),
                        RawValue::String(" Z FULL DRAPE ".into()),
                    )]),
                    source_span: None,
                },
            )]),
        );
        assert!(has_diagnostic(
            &super::super::compile_raw_model_for_source(&referenced),
            "InvalidWindowDrapeEquivalentLayerOpticalSum",
            " Z FULL DRAPE ",
            None
        ));
        for (kind, field, joint) in [
            (
                "kind",
                "drape_beam_beam_solar_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "kind",
                "front_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "kind",
                "back_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "kind",
                "front_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "kind",
                "back_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            ("kind", "drape_beam_beam_visible_transmittance", 0_usize),
            ("kind", "drape_beam_diffuse_visible_transmittance", 0_usize),
            ("kind", "drape_beam_diffuse_visible_reflectance", 0_usize),
            ("kind", "drape_material_infrared_transmittance", 0_usize),
            (
                "kind",
                "front_side_drape_material_infrared_emissivity",
                0_usize,
            ),
            (
                "kind",
                "back_side_drape_material_infrared_emissivity",
                0_usize,
            ),
            ("kind", "width_of_pleated_fabric", 0_usize),
            ("kind", "length_of_pleated_fabric", 0_usize),
            (
                "missing",
                "front_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "missing",
                "back_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "missing",
                "front_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "missing",
                "back_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "lower",
                "drape_beam_beam_solar_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "lower",
                "front_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "lower",
                "back_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "lower",
                "front_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "lower",
                "back_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            ("lower", "drape_beam_beam_visible_transmittance", 0_usize),
            ("lower", "drape_beam_diffuse_visible_transmittance", 0_usize),
            ("lower", "drape_beam_diffuse_visible_reflectance", 0_usize),
            ("lower", "drape_material_infrared_transmittance", 0_usize),
            (
                "lower",
                "front_side_drape_material_infrared_emissivity",
                0_usize,
            ),
            (
                "lower",
                "back_side_drape_material_infrared_emissivity",
                0_usize,
            ),
            ("lower", "width_of_pleated_fabric", 0_usize),
            ("lower", "length_of_pleated_fabric", 0_usize),
            (
                "upper",
                "drape_beam_beam_solar_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "upper",
                "front_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "upper",
                "back_side_drape_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "upper",
                "front_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "upper",
                "back_side_drape_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            ("upper", "drape_beam_beam_visible_transmittance", 0_usize),
            ("upper", "drape_beam_diffuse_visible_transmittance", 0_usize),
            ("upper", "drape_beam_diffuse_visible_reflectance", 0_usize),
            ("upper", "drape_material_infrared_transmittance", 0_usize),
            (
                "upper",
                "front_side_drape_material_infrared_emissivity",
                0_usize,
            ),
            (
                "upper",
                "back_side_drape_material_infrared_emissivity",
                0_usize,
            ),
            ("joint", "", 0_usize),
            ("joint", "", 1_usize),
            ("joint", "", 2_usize),
        ] {
            let mut candidate = source_raw.clone();
            let fields = &mut candidate
                .objects
                .get_mut(&ObjectType(OBJECT_TYPE.into()))
                .expect("EQL family")
                .get_mut(&ObjectName(" Z FULL DRAPE ".into()))
                .expect("full EQL row")
                .fields;
            match kind {
                "kind" => {
                    fields.insert(FieldName(field.into()), RawValue::Bool(false));
                }
                "missing" => {
                    fields.remove(&FieldName(field.into()));
                }
                "lower" => {
                    fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            if field.ends_with("_infrared_emissivity") {
                                "0.0"
                            } else {
                                "-0.01"
                            }
                            .into(),
                        ),
                    );
                }
                "upper" => {
                    fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            if field == "drape_beam_beam_solar_transmittance_at_normal_incidence" {
                                "0.200001"
                            } else {
                                "1.0"
                            }
                            .into(),
                        ),
                    );
                }
                _ => {
                    let values: &[(&str, &str)] = match joint {
                        0 => &[
                            (
                                "drape_beam_beam_solar_transmittance_at_normal_incidence",
                                "0.2",
                            ),
                            ("front_side_drape_beam_diffuse_solar_transmittance", "0.4"),
                            ("front_side_drape_beam_diffuse_solar_reflectance", "0.5"),
                        ],
                        1 => &[
                            ("drape_beam_beam_visible_transmittance", "0.4"),
                            ("drape_beam_diffuse_visible_transmittance", "0.3"),
                            ("drape_beam_diffuse_visible_reflectance", "0.4"),
                        ],
                        _ => &[
                            ("drape_material_infrared_transmittance", "0.3"),
                            ("front_side_drape_material_infrared_emissivity", "0.8"),
                            ("back_side_drape_material_infrared_emissivity", "0.5"),
                        ],
                    };
                    values.iter().for_each(|(key, value)| {
                        fields.insert(FieldName((*key).into()), RawValue::Number((*value).into()));
                    });
                }
            }
            let checked = super::super::compile_raw_model_for_source(&candidate);
            assert_eq!(
                checked.has_errors(),
                kind != "joint",
                "Source original-schema row {kind}/{field}/{joint}: {:?}",
                checked.report.diagnostics
            );
            if kind == "joint" {
                let held_legacy = compile_raw_model(&candidate);
                assert!(
                    held_legacy.has_errors()
                        && has_diagnostic(
                            &held_legacy,
                            "InvalidWindowDrapeEquivalentLayerOpticalSum",
                            " Z FULL DRAPE ",
                            None
                        ),
                    "Legacy joint row {joint}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn drape_equivalent_layer_requires_only_source_required_n2_through_n5()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Missing Required": {}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    let required_fields = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_type == OBJECT_TYPE
                && diagnostic.object_name.as_deref() == Some("Missing Required")
        })
        .filter_map(|diagnostic| diagnostic.field.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        required_fields,
        BTreeSet::from([
            "back_side_drape_beam_diffuse_solar_reflectance".to_string(),
            "back_side_drape_beam_diffuse_solar_transmittance".to_string(),
            "front_side_drape_beam_diffuse_solar_reflectance".to_string(),
            "front_side_drape_beam_diffuse_solar_transmittance".to_string(),
        ])
    );
    Ok(())
}

#[test]
fn drape_equivalent_layer_enforces_all_schema_bounds() -> Result<(), Box<dyn std::error::Error>> {
    let inclusive = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Inclusive Endpoints": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.2,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.0,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.0,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.0,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.0,
                    "drape_beam_beam_visible_transmittance": 0.0,
                    "drape_beam_diffuse_visible_transmittance": 0.0,
                    "drape_beam_diffuse_visible_reflectance": 0.0,
                    "drape_material_infrared_transmittance": 0.0,
                    "front_side_drape_material_infrared_emissivity": 0.5,
                    "back_side_drape_material_infrared_emissivity": 0.5,
                    "width_of_pleated_fabric": 0.0,
                    "length_of_pleated_fabric": 0.0
                }
            }
        }"#,
    )?;
    let inclusive_result = compile_raw_model(&inclusive);
    assert!(
        !inclusive_result.has_errors(),
        "inclusive zero and N1=0.2 endpoints must compile: {:?}",
        inclusive_result.report.diagnostics
    );

    let invalid = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Below Bounds": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": -0.01,
                    "front_side_drape_beam_diffuse_solar_transmittance": -0.01,
                    "back_side_drape_beam_diffuse_solar_transmittance": -0.01,
                    "front_side_drape_beam_diffuse_solar_reflectance": -0.01,
                    "back_side_drape_beam_diffuse_solar_reflectance": -0.01,
                    "drape_beam_beam_visible_transmittance": -0.01,
                    "drape_beam_diffuse_visible_transmittance": -0.01,
                    "drape_beam_diffuse_visible_reflectance": -0.01,
                    "drape_material_infrared_transmittance": -0.01,
                    "front_side_drape_material_infrared_emissivity": 0.0,
                    "back_side_drape_material_infrared_emissivity": 0.0,
                    "width_of_pleated_fabric": -0.01,
                    "length_of_pleated_fabric": -0.01
                },
                "Above Bounds": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.200001,
                    "front_side_drape_beam_diffuse_solar_transmittance": 1.0,
                    "back_side_drape_beam_diffuse_solar_transmittance": 1.0,
                    "front_side_drape_beam_diffuse_solar_reflectance": 1.0,
                    "back_side_drape_beam_diffuse_solar_reflectance": 1.0,
                    "drape_beam_beam_visible_transmittance": 1.0,
                    "drape_beam_diffuse_visible_transmittance": 1.0,
                    "drape_beam_diffuse_visible_reflectance": 1.0,
                    "drape_material_infrared_transmittance": 1.0,
                    "front_side_drape_material_infrared_emissivity": 1.0,
                    "back_side_drape_material_infrared_emissivity": 1.0,
                    "width_of_pleated_fabric": 0.0,
                    "length_of_pleated_fabric": 0.0
                }
            }
        }"#,
    )?;
    let invalid_result = compile_raw_model(&invalid);
    assert!(invalid_result.has_errors());
    let bounded_above_and_below_fields = [
        "drape_beam_beam_solar_transmittance_at_normal_incidence",
        "front_side_drape_beam_diffuse_solar_transmittance",
        "back_side_drape_beam_diffuse_solar_transmittance",
        "front_side_drape_beam_diffuse_solar_reflectance",
        "back_side_drape_beam_diffuse_solar_reflectance",
        "drape_beam_beam_visible_transmittance",
        "drape_beam_diffuse_visible_transmittance",
        "drape_beam_diffuse_visible_reflectance",
        "drape_material_infrared_transmittance",
        "front_side_drape_material_infrared_emissivity",
        "back_side_drape_material_infrared_emissivity",
    ];
    for object_name in ["Below Bounds", "Above Bounds"] {
        for field in bounded_above_and_below_fields {
            assert!(
                has_diagnostic(
                    &invalid_result,
                    "InvalidNumericRange",
                    object_name,
                    Some(field)
                ),
                "missing {object_name}/{field} range diagnostic"
            );
        }
    }
    for field in ["width_of_pleated_fabric", "length_of_pleated_fabric"] {
        assert!(
            has_diagnostic(
                &invalid_result,
                "InvalidNumericRange",
                "Below Bounds",
                Some(field)
            ),
            "missing Below Bounds/{field} range diagnostic"
        );
    }
    Ok(())
}

#[test]
fn drape_equivalent_layer_enforces_only_three_source_optical_sums()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Front Solar Sum": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.2,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.4,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.4,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                },
                "Visible Sum": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "drape_beam_beam_visible_transmittance": 0.3,
                    "drape_beam_diffuse_visible_transmittance": 0.3,
                    "drape_beam_diffuse_visible_reflectance": 0.4
                },
                "Front Infrared Sum": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "drape_material_infrared_transmittance": 0.2,
                    "front_side_drape_material_infrared_emissivity": 0.81,
                    "back_side_drape_material_infrared_emissivity": 0.5
                }
            }
        }"#,
    )?;
    let invalid_result = compile_raw_model(&invalid);
    assert!(invalid_result.has_errors());
    let diagnostics = invalid_result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "InvalidWindowDrapeEquivalentLayerOpticalSum"
                && diagnostic.object_type == OBJECT_TYPE
        })
        .collect::<Vec<_>>();
    assert_eq!(diagnostics.len(), 3, "{diagnostics:?}");
    assert_eq!(
        diagnostics
            .iter()
            .filter_map(|diagnostic| diagnostic.object_name.as_deref())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["Front Infrared Sum", "Front Solar Sum", "Visible Sum"])
    );

    let unchecked_back_and_front_equality = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Unchecked Back And Front Equality": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.2,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.2,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.7,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.7,
                    "drape_material_infrared_transmittance": 0.1,
                    "front_side_drape_material_infrared_emissivity": 0.9,
                    "back_side_drape_material_infrared_emissivity": 0.95
                }
            }
        }"#,
    )?;
    let accepted_result = compile_raw_model(&unchecked_back_and_front_equality);
    assert!(
        !accepted_result.has_errors(),
        "front IR equality and unchecked back-side sums must compile: {:?}",
        accepted_result.report.diagnostics
    );
    Ok(())
}

#[test]
fn drape_equivalent_layer_applies_all_or_nothing_effective_pleat_geometry()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "A Both Positive": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.02,
                    "length_of_pleated_fabric": 0.03
                },
                "B Both Default": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                },
                "C Width Only": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.02
                },
                "D Length Only": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "length_of_pleated_fabric": 0.03
                },
                "E Zero Width": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.0,
                    "length_of_pleated_fabric": 0.03
                },
                "F Zero Length": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.02,
                    "length_of_pleated_fabric": 0.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed pleat cases"))?;

    for (name, width, length, is_pleated) in [
        ("A BOTH POSITIVE", 0.02, 0.03, true),
        ("B BOTH DEFAULT", 0.0, 0.0, false),
        ("C WIDTH ONLY", 0.0, 0.0, false),
        ("D LENGTH ONLY", 0.0, 0.0, false),
        ("E ZERO WIDTH", 0.0, 0.0, false),
        ("F ZERO LENGTH", 0.0, 0.0, false),
    ] {
        let drape = model
            .materials
            .iter()
            .find(|material| material.name.0 == name)
            .and_then(|material| material.as_window_drape_equivalent_layer())
            .ok_or_else(|| std::io::Error::other(format!("missing pleat case {name}")))?;
        assert_eq!(drape.pleated_width_m, width, "{name}");
        assert_eq!(drape.pleated_length_m, length, "{name}");
        assert_eq!(drape.is_pleated(), is_pleated, "{name}");
    }
    Ok(())
}

#[test]
fn drape_equivalent_layer_uses_shared_material_namespace_and_rejects_blank_names()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Shared": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
                }
            },
            "WindowMaterial:Drape:EquivalentLayer": {
                "shared": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                },
                "": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(has_diagnostic(&result, "DuplicateName", "shared", None));
    assert!(has_diagnostic(
        &result,
        "MissingRequiredField",
        "",
        Some("name")
    ));
    Ok(())
}

#[test]
fn drape_equivalent_layer_fails_closed_in_ordinary_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Equivalent Drape": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                }
            },
            "Construction": {
                "Wrong Window": {"outside_layer": "Equivalent Drape"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
    Ok(())
}
