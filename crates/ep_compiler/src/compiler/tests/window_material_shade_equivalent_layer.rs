use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    WindowShadeEquivalentLayerSideOpticalProperties,
};
use ep_raw_model::parse_epjson_str;
use std::collections::BTreeSet;

const OBJECT_TYPE: &str = "WindowMaterial:Shade:EquivalentLayer";

fn has_diagnostic(
    result: &super::super::CompileResult,
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
fn shade_equivalent_layer_materializes_all_inputs_and_source_quirks()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Z Full EQL Shade": {
                    "shade_beam_beam_solar_transmittance": 0.20,
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.10,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.11,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.12,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.13,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence": 0.21,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence": 0.22,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence": 0.23,
                    "shade_material_infrared_transmittance": 0.04,
                    "front_side_shade_material_infrared_emissivity": 0.71,
                    "back_side_shade_material_infrared_emissivity": 0.72
                },
                "A Default EQL Shade": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.10,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.20,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.30,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.40
                }
            },
            "WindowMaterial:Shade": {
                "Ordinary Shade": {
                    "solar_transmittance": 0.10,
                    "solar_reflectance": 0.40,
                    "visible_transmittance": 0.10,
                    "visible_reflectance": 0.40,
                    "infrared_hemispherical_emissivity": 0.80,
                    "infrared_transmittance": 0.10,
                    "thickness": 0.002,
                    "conductivity": 0.20
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer shades"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ORDINARY SHADE", "A DEFAULT EQL SHADE", "Z FULL EQL SHADE"],
        "equivalent-layer shades must follow ordinary WindowMaterial:Shade"
    );

    let material = &model.materials[2];
    assert_eq!(material.kind(), MaterialKind::WindowShadeEquivalentLayer);
    assert_eq!(material.family(), MaterialFamily::EquivalentLayer);
    assert!(material.as_opaque().is_none());
    assert!(material.as_window_shade().is_none());
    assert_eq!(material.thickness_m(), None);
    assert_eq!(material.conductivity_w_per_m_k(), None);
    assert_eq!(material.density_kg_per_m3(), None);
    assert_eq!(material.specific_heat_j_per_kg_k(), None);
    assert_eq!(material.thermal_resistance(), None);
    assert_eq!(material.heat_capacity_per_area(), None);
    assert_eq!(material.thermal_absorptance(), None);
    assert_eq!(material.solar_absorptance(), None);
    assert_eq!(material.visible_absorptance(), None);
    let shade = material
        .as_window_shade_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer shade payload"))?;
    assert_eq!(shade.roughness, MaterialSurfaceRoughness::MediumRough);
    assert!(shade.is_resistance_only());
    assert_eq!(shade.nominal_thermal_resistance_m2_k_per_w(), None);
    assert_eq!(
        shade.front_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.20,
            beam_diffuse_transmittance: 0.10,
            beam_diffuse_reflectance: 0.12,
        }
    );
    assert_eq!(
        shade.back_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.20,
            beam_diffuse_transmittance: 0.11,
            beam_diffuse_reflectance: 0.13,
        }
    );
    assert_eq!(
        shade.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.21,
            beam_diffuse_transmittance: 0.22,
            beam_diffuse_reflectance: 0.23,
        }
    );
    assert_eq!(
        shade.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default(),
        "EnergyPlus 26.1 leaves all back-side visible TAR slots at zero"
    );
    assert_eq!(shade.infrared_transmittance, 0.04);
    assert_eq!(shade.front_infrared_emissivity, 0.71);
    assert_eq!(shade.back_infrared_emissivity, 0.72);
    assert_eq!(shade.front_thermal_absorptance, 0.71);
    assert_eq!(shade.back_thermal_absorptance, 0.72);
    assert_eq!(shade.thermal_transmittance, 0.04);

    let default = model.materials[1]
        .as_window_shade_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing defaulted equivalent-layer shade"))?;
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
    assert_eq!(default.front_infrared_emissivity, 0.91);
    assert_eq!(default.back_infrared_emissivity, 0.91);
    for (field, value) in [
        ("shade_beam_beam_solar_transmittance", "0.0"),
        (
            "shade_beam_beam_visible_transmittance_at_normal_incidence",
            "0.0",
        ),
        (
            "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
            "0.0",
        ),
        (
            "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
            "0.0",
        ),
        ("shade_material_infrared_transmittance", "0.05"),
        ("front_side_shade_material_infrared_emissivity", "0.91"),
        ("back_side_shade_material_infrared_emissivity", "0.91"),
    ] {
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == OBJECT_TYPE
                && default.object_name == "A Default EQL Shade"
                && default.field == field
                && default.value == value
        }));
    }

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer shade coverage"))?;
    assert_eq!(coverage.object_count, 2);
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

    // SourceB original-schema qualification is separate from the five Native joint sums.
    {
        use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue};
        let source_raw = ep_raw_model::parse_epjson_str_for_source(
            r#"{"WindowMaterial:Shade:EquivalentLayer":{
                " A DEFAULT EQL ":{
                    "front_side_shade_beam_diffuse_solar_transmittance":0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.15,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.2,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.25,
                    "name":false,"idf_order":[],"metadata":null
                },
                " Z FULL EQL ":{
                    "shade_beam_beam_solar_transmittance":0.1,
                    "front_side_shade_beam_diffuse_solar_transmittance":0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.15,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.2,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.25,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence":0.1,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence":0.2,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence":0.3,
                    "shade_material_infrared_transmittance":0.04,
                    "front_side_shade_material_infrared_emissivity":0.8,
                    "back_side_shade_material_infrared_emissivity":0.75,
                    "name":null,"idf_order":false,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":[],"metadata":{"retained":[null,true,1.25,"mixed"]}
                }
            }}"#,
        )?;
        let source = super::super::compile_raw_model_for_source(&source_raw);
        assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
        let source_model = source.model.as_ref().expect("two Source EQL shades");
        let default_id = source_model
            .material_names
            .resolve(" A DEFAULT EQL ")
            .expect("untrimmed default name");
        let full_id = source_model
            .material_names
            .resolve(" Z FULL EQL ")
            .expect("untrimmed full name");
        assert!(
            default_id == ep_model::MaterialId(0)
                && default_id != full_id
                && source_model.material_names.is_source_exact_namespace()
                && source_model
                    .material_names
                    .resolve("A DEFAULT EQL")
                    .is_none()
                && source_model.materials.len() == 2
                && source_model.ctf_report_names.materials
                    == vec![
                        (default_id, " A DEFAULT EQL ".into()),
                        (full_id, " Z FULL EQL ".into())
                    ]
                && source_model.materials[0].kind() == MaterialKind::WindowShadeEquivalentLayer
                && source_model.materials[0].family() == MaterialFamily::EquivalentLayer
        );
        let source_default = source_model.materials[0]
            .as_window_shade_equivalent_layer()
            .expect("default Source payload");
        let source_full = source_model.materials[1]
            .as_window_shade_equivalent_layer()
            .expect("full Source payload");
        assert!(
            source_default.front_solar.beam_beam_transmittance.to_bits() == 0.0_f64.to_bits()
                && source_default.front_visible
                    == WindowShadeEquivalentLayerSideOpticalProperties::default()
                && source_default.back_visible
                    == WindowShadeEquivalentLayerSideOpticalProperties::default()
                && source_default.infrared_transmittance.to_bits() == 0.05_f64.to_bits()
                && source_default.front_infrared_emissivity.to_bits() == 0.91_f64.to_bits()
                && source_default.back_infrared_emissivity.to_bits() == 0.91_f64.to_bits()
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
            [&ObjectName(" Z FULL EQL ".into())];
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
            )
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
            .get_mut(&ObjectName(" Z FULL EQL ".into()))
            .expect("EQL row")
            .fields
            .extend([
                (
                    FieldName("shade_beam_beam_solar_transmittance".into()),
                    RawValue::Number("0.4".into()),
                ),
                (
                    FieldName("front_side_shade_beam_diffuse_solar_transmittance".into()),
                    RawValue::Number("0.4".into()),
                ),
                (
                    FieldName("front_side_shade_beam_diffuse_solar_reflectance".into()),
                    RawValue::Number("0.4".into()),
                ),
            ]);
        referenced.objects.insert(
            ObjectType("Construction".into()),
            std::collections::BTreeMap::from([(
                ObjectName("EQL ACTIVE".into()),
                ep_raw_model::RawObject {
                    fields: std::collections::BTreeMap::from([(
                        FieldName("outside_layer".into()),
                        RawValue::String(" Z FULL EQL ".into()),
                    )]),
                    source_span: None,
                },
            )]),
        );
        assert!(has_diagnostic(
            &super::super::compile_raw_model_for_source(&referenced),
            "InvalidWindowShadeEquivalentLayerOpticalSum",
            " Z FULL EQL ",
            None
        ));
        for (kind, field, joint) in [
            ("kind", "shade_beam_beam_solar_transmittance", 0_usize),
            (
                "kind",
                "front_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "kind",
                "back_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "kind",
                "front_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "kind",
                "back_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "kind",
                "shade_beam_beam_visible_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "kind",
                "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "kind",
                "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
                0_usize,
            ),
            ("kind", "shade_material_infrared_transmittance", 0_usize),
            (
                "kind",
                "front_side_shade_material_infrared_emissivity",
                0_usize,
            ),
            (
                "kind",
                "back_side_shade_material_infrared_emissivity",
                0_usize,
            ),
            (
                "missing",
                "front_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "missing",
                "back_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "missing",
                "front_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "missing",
                "back_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            ("lower", "shade_beam_beam_solar_transmittance", 0_usize),
            (
                "lower",
                "front_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "lower",
                "back_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "lower",
                "front_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "lower",
                "back_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "lower",
                "shade_beam_beam_visible_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "lower",
                "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "lower",
                "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
                0_usize,
            ),
            ("lower", "shade_material_infrared_transmittance", 0_usize),
            (
                "lower",
                "front_side_shade_material_infrared_emissivity",
                0_usize,
            ),
            (
                "lower",
                "back_side_shade_material_infrared_emissivity",
                0_usize,
            ),
            ("upper", "shade_beam_beam_solar_transmittance", 0_usize),
            (
                "upper",
                "front_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "upper",
                "back_side_shade_beam_diffuse_solar_transmittance",
                0_usize,
            ),
            (
                "upper",
                "front_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "upper",
                "back_side_shade_beam_diffuse_solar_reflectance",
                0_usize,
            ),
            (
                "upper",
                "shade_beam_beam_visible_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "upper",
                "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
                0_usize,
            ),
            (
                "upper",
                "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
                0_usize,
            ),
            ("upper", "shade_material_infrared_transmittance", 0_usize),
            (
                "upper",
                "front_side_shade_material_infrared_emissivity",
                0_usize,
            ),
            (
                "upper",
                "back_side_shade_material_infrared_emissivity",
                0_usize,
            ),
            ("joint", "", 0_usize),
            ("joint", "", 1_usize),
            ("joint", "", 2_usize),
            ("joint", "", 3_usize),
            ("joint", "", 4_usize),
        ] {
            let mut candidate = source_raw.clone();
            let fields = &mut candidate
                .objects
                .get_mut(&ObjectType(OBJECT_TYPE.into()))
                .expect("EQL family")
                .get_mut(&ObjectName(" Z FULL EQL ".into()))
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
                    fields.insert(FieldName(field.into()), RawValue::Number("-0.01".into()));
                }
                "upper" => {
                    fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            if field == "shade_beam_beam_solar_transmittance" {
                                "0.800001"
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
                            ("shade_beam_beam_solar_transmittance", "0.4"),
                            ("front_side_shade_beam_diffuse_solar_transmittance", "0.4"),
                            ("front_side_shade_beam_diffuse_solar_reflectance", "0.4"),
                        ],
                        1 => &[
                            ("shade_beam_beam_solar_transmittance", "0.4"),
                            ("back_side_shade_beam_diffuse_solar_transmittance", "0.4"),
                            ("back_side_shade_beam_diffuse_solar_reflectance", "0.4"),
                        ],
                        2 => &[
                            (
                                "shade_beam_beam_visible_transmittance_at_normal_incidence",
                                "0.4",
                            ),
                            (
                                "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
                                "0.3",
                            ),
                            (
                                "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
                                "0.4",
                            ),
                        ],
                        3 => &[
                            ("shade_material_infrared_transmittance", "0.3"),
                            ("front_side_shade_material_infrared_emissivity", "0.8"),
                            ("back_side_shade_material_infrared_emissivity", "0.5"),
                        ],
                        _ => &[
                            ("shade_material_infrared_transmittance", "0.3"),
                            ("front_side_shade_material_infrared_emissivity", "0.5"),
                            ("back_side_shade_material_infrared_emissivity", "0.8"),
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
                            "InvalidWindowShadeEquivalentLayerOpticalSum",
                            " Z FULL EQL ",
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
fn shade_equivalent_layer_requires_only_source_required_n2_through_n5()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
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
            "back_side_shade_beam_diffuse_solar_reflectance".to_string(),
            "back_side_shade_beam_diffuse_solar_transmittance".to_string(),
            "front_side_shade_beam_diffuse_solar_reflectance".to_string(),
            "front_side_shade_beam_diffuse_solar_transmittance".to_string(),
        ])
    );
    Ok(())
}

#[test]
fn shade_equivalent_layer_enforces_inclusive_and_exclusive_schema_bounds()
-> Result<(), Box<dyn std::error::Error>> {
    let inclusive = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Inclusive Endpoints": {
                    "shade_beam_beam_solar_transmittance": 0.8,
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.0,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.0,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.0,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.0,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence": 0.0,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence": 0.0,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence": 0.0,
                    "shade_material_infrared_transmittance": 0.0,
                    "front_side_shade_material_infrared_emissivity": 0.5,
                    "back_side_shade_material_infrared_emissivity": 0.5
                }
            }
        }"#,
    )?;
    let inclusive_result = compile_raw_model(&inclusive);
    assert!(
        !inclusive_result.has_errors(),
        "N1=0.8 and the inclusive zero endpoints must compile: {:?}",
        inclusive_result.report.diagnostics
    );

    let invalid = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Below Bounds": {
                    "shade_beam_beam_solar_transmittance": -0.01,
                    "front_side_shade_beam_diffuse_solar_transmittance": -0.01,
                    "back_side_shade_beam_diffuse_solar_transmittance": -0.01,
                    "front_side_shade_beam_diffuse_solar_reflectance": -0.01,
                    "back_side_shade_beam_diffuse_solar_reflectance": -0.01,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence": -0.01,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence": -0.01,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence": -0.01,
                    "shade_material_infrared_transmittance": -0.01,
                    "front_side_shade_material_infrared_emissivity": 0.0,
                    "back_side_shade_material_infrared_emissivity": 0.0
                },
                "Above Bounds": {
                    "shade_beam_beam_solar_transmittance": 0.800001,
                    "front_side_shade_beam_diffuse_solar_transmittance": 1.0,
                    "back_side_shade_beam_diffuse_solar_transmittance": 1.0,
                    "front_side_shade_beam_diffuse_solar_reflectance": 1.0,
                    "back_side_shade_beam_diffuse_solar_reflectance": 1.0,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence": 1.0,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence": 1.0,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence": 1.0,
                    "shade_material_infrared_transmittance": 1.0,
                    "front_side_shade_material_infrared_emissivity": 1.0,
                    "back_side_shade_material_infrared_emissivity": 1.0
                }
            }
        }"#,
    )?;
    let invalid_result = compile_raw_model(&invalid);
    assert!(invalid_result.has_errors());
    let fields = [
        "shade_beam_beam_solar_transmittance",
        "front_side_shade_beam_diffuse_solar_transmittance",
        "back_side_shade_beam_diffuse_solar_transmittance",
        "front_side_shade_beam_diffuse_solar_reflectance",
        "back_side_shade_beam_diffuse_solar_reflectance",
        "shade_beam_beam_visible_transmittance_at_normal_incidence",
        "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
        "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
        "shade_material_infrared_transmittance",
        "front_side_shade_material_infrared_emissivity",
        "back_side_shade_material_infrared_emissivity",
    ];
    for object_name in ["Below Bounds", "Above Bounds"] {
        for field in fields {
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
    Ok(())
}

#[test]
fn shade_equivalent_layer_enforces_all_five_strict_optical_sums()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Front Solar Sum": {
                    "shade_beam_beam_solar_transmittance": 0.2,
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.4,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.4,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
                },
                "Back Solar Sum": {
                    "shade_beam_beam_solar_transmittance": 0.2,
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.4,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.4
                },
                "Visible Sum": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence": 0.3,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence": 0.3,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence": 0.4
                },
                "Front Infrared Sum": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "shade_material_infrared_transmittance": 0.1,
                    "front_side_shade_material_infrared_emissivity": 0.9,
                    "back_side_shade_material_infrared_emissivity": 0.5
                },
                "Back Infrared Sum": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "shade_material_infrared_transmittance": 0.1,
                    "front_side_shade_material_infrared_emissivity": 0.5,
                    "back_side_shade_material_infrared_emissivity": 0.9
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    let diagnostics = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "InvalidWindowShadeEquivalentLayerOpticalSum"
                && diagnostic.object_type == OBJECT_TYPE
        })
        .collect::<Vec<_>>();
    assert_eq!(diagnostics.len(), 5, "{diagnostics:?}");
    assert_eq!(
        diagnostics
            .iter()
            .filter_map(|diagnostic| diagnostic.object_name.as_deref())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "Back Infrared Sum",
            "Back Solar Sum",
            "Front Infrared Sum",
            "Front Solar Sum",
            "Visible Sum",
        ])
    );
    Ok(())
}

#[test]
fn shade_equivalent_layer_checks_each_solar_side_independently()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Missing Back Reflectance": {
                    "shade_beam_beam_solar_transmittance": 0.2,
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.4,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.4
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        "MissingRequiredField",
        "Missing Back Reflectance",
        Some("back_side_shade_beam_diffuse_solar_reflectance")
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidWindowShadeEquivalentLayerOpticalSum",
        "Missing Back Reflectance",
        Some("shade_beam_beam_solar_transmittance")
    ));
    Ok(())
}

#[test]
fn shade_equivalent_layer_uses_shared_material_namespace_and_rejects_blank_names()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade": {
                "Shared": {
                    "solar_transmittance": 0.10,
                    "solar_reflectance": 0.40,
                    "visible_transmittance": 0.10,
                    "visible_reflectance": 0.40,
                    "infrared_hemispherical_emissivity": 0.80,
                    "infrared_transmittance": 0.10,
                    "thickness": 0.002,
                    "conductivity": 0.20
                }
            },
            "WindowMaterial:Shade:EquivalentLayer": {
                "shared": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
                },
                "": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
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
fn shade_equivalent_layer_fails_closed_in_ordinary_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Equivalent Shade": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
                }
            },
            "Construction": {
                "Wrong Window": {"outside_layer": "Equivalent Shade"}
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
