use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{AutoOrNumber, MaterialDefinition, MaterialFamily, MaterialId, MaterialKind};
use ep_raw_model::parse_epjson_str;

fn equivalent_layer_fields(overrides: &str) -> String {
    let suffix = if overrides.is_empty() {
        String::new()
    } else {
        format!(",{overrides}")
    };
    format!(
        r#""front_side_beam_beam_solar_transmittance":0.61,
           "back_side_beam_beam_solar_transmittance":0.62,
           "front_side_beam_beam_solar_reflectance":0.21,
           "back_side_beam_beam_solar_reflectance":0.22{suffix}"#
    )
}

#[test]
fn equivalent_layer_materializes_all_inputs_and_preserves_source_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Equivalent Glass": {{{}}}
            }},
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {{
                "Alternative Glass": {{
                    "thickness":0.003,
                    "solar_index_of_refraction":1.5,
                    "solar_extinction_coefficient":20.0,
                    "visible_index_of_refraction":1.6,
                    "visible_extinction_coefficient":30.0
                }}
            }},
            "WindowMaterial:Glazing": {{
                "Direct Glass": {{
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.004
                }}
            }},
            "Material": {{
                "Opaque": {{
                    "roughness":"Rough","thickness":0.1,"conductivity":1.0,
                    "density":1000.0,"specific_heat":1000.0
                }}
            }}
        }}"#,
        equivalent_layer_fields(
            r#""optical_data_type":"SpectralAverage",
               "window_glass_spectral_data_set_name":"Ignored For SpectralAverage",
               "front_side_beam_beam_visible_solar_transmittance":0.71,
               "back_side_beam_beam_visible_solar_transmittance":0.72,
               "front_side_beam_beam_visible_solar_reflectance":0.11,
               "back_side_beam_beam_visible_solar_reflectance":0.12,
               "front_side_beam_diffuse_solar_transmittance":0.31,
               "back_side_beam_diffuse_solar_transmittance":0.32,
               "front_side_beam_diffuse_solar_reflectance":0.23,
               "back_side_beam_diffuse_solar_reflectance":0.24,
               "front_side_beam_diffuse_visible_solar_transmittance":0.41,
               "back_side_beam_diffuse_visible_solar_transmittance":0.42,
               "front_side_beam_diffuse_visible_solar_reflectance":0.13,
               "back_side_beam_diffuse_visible_solar_reflectance":0.14,
               "diffuse_diffuse_solar_transmittance":0.51,
               "front_side_diffuse_diffuse_solar_reflectance":0.25,
               "back_side_diffuse_diffuse_solar_reflectance":0.26,
               "diffuse_diffuse_visible_solar_transmittance":0.52,
               "front_side_diffuse_diffuse_visible_solar_reflectance":0.15,
               "back_side_diffuse_diffuse_visible_solar_reflectance":0.16,
               "infrared_transmittance_applies_to_front_and_back_":0.03,
               "front_side_infrared_emissivity":0.81,
               "back_side_infrared_emissivity":0.82,
               "thermal_resistance":0.17"#,
        )
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer glazing"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "OPAQUE",
            "DIRECT GLASS",
            "ALTERNATIVE GLASS",
            "EQUIVALENT GLASS"
        ],
        "material IDs must follow EnergyPlus family source order"
    );
    let glazing = &model.materials[3];
    assert_eq!(glazing.id, MaterialId(3));
    assert_eq!(glazing.kind(), MaterialKind::WindowGlazingEquivalentLayer);
    assert_eq!(glazing.family(), MaterialFamily::EquivalentLayer);
    assert!(glazing.as_opaque().is_none());
    assert_eq!(glazing.thickness_m(), None);
    assert_eq!(glazing.conductivity_w_per_m_k(), None);
    assert_eq!(glazing.thermal_resistance(), None);
    assert_eq!(glazing.heat_capacity_per_area(), None);

    let fields = glazing
        .as_window_glazing_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected equivalent-layer payload"))?;
    assert_eq!(fields.solar.beam_beam.front_transmittance, 0.61);
    assert_eq!(fields.solar.beam_beam.back_transmittance, 0.62);
    assert_eq!(fields.solar.beam_beam.front_reflectance, 0.21);
    assert_eq!(fields.solar.beam_beam.back_reflectance, 0.22);
    assert_eq!(fields.visible.beam_beam.front_transmittance, 0.71);
    assert_eq!(fields.visible.beam_beam.back_transmittance, 0.72);
    assert_eq!(fields.visible.beam_beam.front_reflectance, 0.11);
    assert_eq!(fields.visible.beam_beam.back_reflectance, 0.12);
    assert_eq!(fields.solar.beam_diffuse.front_transmittance, 0.31);
    assert_eq!(fields.solar.beam_diffuse.back_transmittance, 0.32);
    assert_eq!(fields.solar.beam_diffuse.front_reflectance, 0.23);
    assert_eq!(fields.solar.beam_diffuse.back_reflectance, 0.24);
    assert_eq!(fields.visible.beam_diffuse.front_transmittance, 0.41);
    assert_eq!(fields.visible.beam_diffuse.back_transmittance, 0.42);
    assert_eq!(fields.visible.beam_diffuse.front_reflectance, 0.13);
    assert_eq!(fields.visible.beam_diffuse.back_reflectance, 0.14);
    assert_eq!(
        fields.solar.diffuse_diffuse.transmittance,
        AutoOrNumber::Value(0.51)
    );
    assert_eq!(
        fields.solar.diffuse_diffuse.front_reflectance,
        AutoOrNumber::Value(0.25)
    );
    assert_eq!(
        fields.solar.diffuse_diffuse.back_reflectance,
        AutoOrNumber::Value(0.26)
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.transmittance,
        AutoOrNumber::Value(0.52)
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.front_reflectance,
        AutoOrNumber::Value(0.15)
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.back_reflectance,
        AutoOrNumber::Value(0.16)
    );
    assert_eq!(fields.infrared_transmittance, 0.03);
    assert_eq!(fields.front_infrared_emissivity, 0.81);
    assert_eq!(fields.back_infrared_emissivity, 0.82);
    assert_eq!(fields.thermal_resistance_m2_k_per_w, 0.17);

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Glazing:EquivalentLayer")
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer coverage row"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

    // Sole-family Source fixtures independently exercise both SourceExact name hooks.
    {
        use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue};
        const GLASS_EQL: &str = "WindowMaterial:Glazing:EquivalentLayer";
        let source_full_raw = parse_epjson_str(
            r#"{"WindowMaterial:Glazing:EquivalentLayer":{" FULL GLASS EQL ":{"front_side_beam_beam_solar_transmittance":0.61,"back_side_beam_beam_solar_transmittance":0.62,"front_side_beam_beam_solar_reflectance":0.21,"back_side_beam_beam_solar_reflectance":0.22,"front_side_beam_beam_visible_solar_transmittance":0.71,"back_side_beam_beam_visible_solar_transmittance":0.72,"front_side_beam_beam_visible_solar_reflectance":0.11,"back_side_beam_beam_visible_solar_reflectance":0.12,"front_side_beam_diffuse_solar_transmittance":0.31,"back_side_beam_diffuse_solar_transmittance":0.32,"front_side_beam_diffuse_solar_reflectance":0.23,"back_side_beam_diffuse_solar_reflectance":0.24,"front_side_beam_diffuse_visible_solar_transmittance":0.41,"back_side_beam_diffuse_visible_solar_transmittance":0.42,"front_side_beam_diffuse_visible_solar_reflectance":0.13,"back_side_beam_diffuse_visible_solar_reflectance":0.14,"diffuse_diffuse_solar_transmittance":0.51,"front_side_diffuse_diffuse_solar_reflectance":0.25,"back_side_diffuse_diffuse_solar_reflectance":0.26,"diffuse_diffuse_visible_solar_transmittance":0.52,"front_side_diffuse_diffuse_visible_solar_reflectance":0.15,"back_side_diffuse_diffuse_visible_solar_reflectance":0.16,"infrared_transmittance_applies_to_front_and_back_":0.03,"front_side_infrared_emissivity":0.81,"back_side_infrared_emissivity":0.82,"thermal_resistance":0.17,"optical_data_type":"SpectralAverage","window_glass_spectral_data_set_name":"Unused unresolved set","name":false,"idf_order":[],"metadata":{"retained":true}}}}"#,
        )?;
        let source_full = super::super::compile_raw_model_for_source(&source_full_raw);
        let source_model = source_full.model.as_ref().expect("Source full GlazingEQL");
        let full = source_model.materials[0]
            .as_window_glazing_equivalent_layer()
            .expect("actual advisory bands");
        assert!(
            !source_full.has_errors() && source_model.material_names.is_source_exact_namespace()
                && source_model.materials[0].id == MaterialId(0)
                && source_model.ctf_report_names.materials == [(MaterialId(0)," FULL GLASS EQL ".into())]
                && [full.solar.beam_beam.front_transmittance,full.solar.beam_beam.back_transmittance,
                    full.solar.beam_beam.front_reflectance,full.solar.beam_beam.back_reflectance,
                    full.visible.beam_beam.front_transmittance,full.visible.beam_beam.back_transmittance,
                    full.visible.beam_beam.front_reflectance,full.visible.beam_beam.back_reflectance,
                    full.solar.beam_diffuse.front_transmittance,full.solar.beam_diffuse.back_transmittance,
                    full.solar.beam_diffuse.front_reflectance,full.solar.beam_diffuse.back_reflectance,
                    full.visible.beam_diffuse.front_transmittance,full.visible.beam_diffuse.back_transmittance,
                    full.visible.beam_diffuse.front_reflectance,full.visible.beam_diffuse.back_reflectance]
                    .iter().zip([0.61_f64,0.62,0.21,0.22,0.71,0.72,0.11,0.12,0.31,0.32,0.23,0.24,0.41,0.42,0.13,0.14])
                    .all(|(actual,expected)|actual.to_bits() == expected.to_bits())
                && [full.solar.diffuse_diffuse.transmittance,full.solar.diffuse_diffuse.front_reflectance,
                    full.solar.diffuse_diffuse.back_reflectance,full.visible.diffuse_diffuse.transmittance,
                    full.visible.diffuse_diffuse.front_reflectance,full.visible.diffuse_diffuse.back_reflectance]
                    .iter().zip([0.51_f64,0.25,0.26,0.52,0.15,0.16])
                    .all(|(actual,expected)|matches!(actual,AutoOrNumber::Value(value) if value.to_bits() == expected.to_bits()))
                && [full.infrared_transmittance,full.front_infrared_emissivity,full.back_infrared_emissivity,full.thermal_resistance_m2_k_per_w]
                    .iter().zip([0.03_f64,0.81,0.82,0.17]).all(|(actual,expected)|actual.to_bits() == expected.to_bits())
        );
        let minimum = parse_epjson_str(
            r#"{"WindowMaterial:Glazing:EquivalentLayer":{"MIN":{"front_side_beam_beam_solar_transmittance":0.61,"back_side_beam_beam_solar_transmittance":0.62,"front_side_beam_beam_solar_reflectance":0.21,"back_side_beam_beam_solar_reflectance":0.22}}}"#,
        )?;
        let auto_keys = [
            "diffuse_diffuse_solar_transmittance",
            "front_side_diffuse_diffuse_solar_reflectance",
            "back_side_diffuse_diffuse_solar_reflectance",
            "diffuse_diffuse_visible_solar_transmittance",
            "front_side_diffuse_diffuse_visible_solar_reflectance",
            "back_side_diffuse_diffuse_visible_solar_reflectance",
        ];
        let mut empty_auto = minimum.clone();
        empty_auto
            .objects
            .get_mut(&ObjectType(GLASS_EQL.into()))
            .expect("family")
            .get_mut(&ObjectName("MIN".into()))
            .expect("minimum")
            .fields
            .extend(
                auto_keys
                    .iter()
                    .map(|key| (FieldName((*key).into()), RawValue::String(String::new()))),
            );
        let mut exact_auto = minimum.clone();
        exact_auto
            .objects
            .get_mut(&ObjectType(GLASS_EQL.into()))
            .expect("family")
            .get_mut(&ObjectName("MIN".into()))
            .expect("minimum")
            .fields
            .extend(auto_keys.iter().map(|key| {
                (
                    FieldName((*key).into()),
                    RawValue::String("Autocalculate".into()),
                )
            }));
        let mut empty_alphas = minimum.clone();
        empty_alphas
            .objects
            .get_mut(&ObjectType(GLASS_EQL.into()))
            .expect("family")
            .get_mut(&ObjectName("MIN".into()))
            .expect("minimum")
            .fields
            .extend([
                (
                    FieldName("optical_data_type".into()),
                    RawValue::String(String::new()),
                ),
                (
                    FieldName("window_glass_spectral_data_set_name".into()),
                    RawValue::String(String::new()),
                ),
            ]);
        assert!(
            [&minimum, &empty_auto, &exact_auto, &empty_alphas]
                .iter()
                .all(|raw| {
                    let result = super::super::compile_raw_model_for_source(raw);
                    result.model.as_ref().is_some_and(|model| {
                        !result.has_errors()
                            && model.material_names.is_source_exact_namespace()
                            && model.materials.len() == 1
                            && model.material_names.resolve("MIN") == Some(MaterialId(0))
                            && model.ctf_report_names.materials == [(MaterialId(0), "MIN".into())]
                            && model.materials[0]
                                .as_window_glazing_equivalent_layer()
                                .is_some_and(|row| {
                                    [
                                        row.solar.diffuse_diffuse.transmittance,
                                        row.solar.diffuse_diffuse.front_reflectance,
                                        row.solar.diffuse_diffuse.back_reflectance,
                                        row.visible.diffuse_diffuse.transmittance,
                                        row.visible.diffuse_diffuse.front_reflectance,
                                        row.visible.diffuse_diffuse.back_reflectance,
                                    ]
                                    .iter()
                                    .all(|value| *value == AutoOrNumber::AutoCalculate)
                                        && row.infrared_transmittance.to_bits() == 0.0_f64.to_bits()
                                        && row.front_infrared_emissivity.to_bits()
                                            == 0.84_f64.to_bits()
                                        && row.back_infrared_emissivity.to_bits()
                                            == 0.84_f64.to_bits()
                                        && row.thermal_resistance_m2_k_per_w.to_bits()
                                            == 0.158_f64.to_bits()
                                })
                    })
                })
        );
        // These finite Legacy controls follow its actual case/whitespace policy.
        assert!(
            [
                ("optical_data_type", "spectralaverage", true),
                ("optical_data_type", " SpectralAverage ", false),
                ("optical_data_type", "Spectral", false),
                ("diffuse_diffuse_solar_transmittance", "autocalculate", true),
                (
                    "diffuse_diffuse_solar_transmittance",
                    " Autocalculate ",
                    false
                ),
                ("diffuse_diffuse_solar_transmittance", " ", true),
            ]
            .iter()
            .all(|(field, text, legacy_ok)| {
                let mut raw = source_full_raw.clone();
                raw.objects
                    .get_mut(&ObjectType(GLASS_EQL.into()))
                    .expect("family")
                    .get_mut(&ObjectName(" FULL GLASS EQL ".into()))
                    .expect("full")
                    .fields
                    .insert(FieldName((*field).into()), RawValue::String((*text).into()));
                let legacy = compile_raw_model(&raw);
                let source = super::super::compile_raw_model_for_source(&raw);
                legacy.has_errors() != *legacy_ok
                    && source.model.as_ref().is_none_or(|model| {
                        !model.material_names.is_source_exact_namespace()
                            && model.ctf_report_names.materials.is_empty()
                    })
            })
        );
        assert!(["", "\n", " \t\r\u{c}"].iter().all(|name| {
            let mut raw = source_full_raw.clone();
            let rows = raw
                .objects
                .get_mut(&ObjectType(GLASS_EQL.into()))
                .expect("only family");
            let row = rows
                .remove(&ObjectName(" FULL GLASS EQL ".into()))
                .expect("full");
            rows.insert(ObjectName((*name).into()), row);
            let result = super::super::compile_raw_model_for_source(&raw);
            result.has_errors()
                && result.model.as_ref().is_none_or(|model| {
                    !model.material_names.is_source_exact_namespace()
                        && model.ctf_report_names.materials.is_empty()
                })
        }));
        for (kind, field, probe, name) in [
            ("name", "", 0_usize, "\u{b}"),
            ("name", "", 1_usize, "\u{a0}"),
            ("name", "", 2_usize, " odd  spaces "),
            ("name", "", 3_usize, "\tMixed\u{b}\u{a0} "),
            (
                "kind",
                "front_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_beam_beam_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_beam_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_beam_beam_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_beam_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_beam_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_beam_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_beam_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_beam_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_beam_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "diffuse_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "diffuse_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "infrared_transmittance_applies_to_front_and_back_",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "front_side_infrared_emissivity",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "kind",
                "back_side_infrared_emissivity",
                0_usize,
                " FULL GLASS EQL ",
            ),
            ("kind", "thermal_resistance", 0_usize, " FULL GLASS EQL "),
            ("kind", "optical_data_type", 0_usize, " FULL GLASS EQL "),
            (
                "kind",
                "window_glass_spectral_data_set_name",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "missing",
                "front_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "missing",
                "back_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "missing",
                "front_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "missing",
                "back_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_beam_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_beam_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_beam_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_beam_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_beam_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_beam_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "diffuse_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "diffuse_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "infrared_transmittance_applies_to_front_and_back_",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "front_side_infrared_emissivity",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "lower",
                "back_side_infrared_emissivity",
                0_usize,
                " FULL GLASS EQL ",
            ),
            ("lower", "thermal_resistance", 0_usize, " FULL GLASS EQL "),
            (
                "upper",
                "front_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_beam_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_beam_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_beam_beam_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_beam_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_beam_beam_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_beam_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_beam_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_beam_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_beam_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_beam_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_beam_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "diffuse_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "diffuse_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "infrared_transmittance_applies_to_front_and_back_",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "front_side_infrared_emissivity",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "upper",
                "back_side_infrared_emissivity",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "diffuse_diffuse_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "diffuse_diffuse_solar_transmittance",
                1_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "diffuse_diffuse_solar_transmittance",
                2_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "front_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "front_side_diffuse_diffuse_solar_reflectance",
                1_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "front_side_diffuse_diffuse_solar_reflectance",
                2_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "back_side_diffuse_diffuse_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "back_side_diffuse_diffuse_solar_reflectance",
                1_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "back_side_diffuse_diffuse_solar_reflectance",
                2_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "diffuse_diffuse_visible_solar_transmittance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "diffuse_diffuse_visible_solar_transmittance",
                1_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "diffuse_diffuse_visible_solar_transmittance",
                2_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "front_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "front_side_diffuse_diffuse_visible_solar_reflectance",
                1_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "front_side_diffuse_diffuse_visible_solar_reflectance",
                2_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "back_side_diffuse_diffuse_visible_solar_reflectance",
                0_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "back_side_diffuse_diffuse_visible_solar_reflectance",
                1_usize,
                " FULL GLASS EQL ",
            ),
            (
                "auto",
                "back_side_diffuse_diffuse_visible_solar_reflectance",
                2_usize,
                " FULL GLASS EQL ",
            ),
            ("mode", "optical_data_type", 0_usize, " FULL GLASS EQL "),
            ("mode", "optical_data_type", 1_usize, " FULL GLASS EQL "),
            ("mode", "optical_data_type", 2_usize, " FULL GLASS EQL "),
            ("spectral", "optical_data_type", 0_usize, " FULL GLASS EQL "),
            ("wire", "thermal_resistance", 0_usize, " FULL GLASS EQL "),
            ("wire", "thermal_resistance", 1_usize, " FULL GLASS EQL "),
            ("wire", "thermal_resistance", 2_usize, " FULL GLASS EQL "),
            ("wire", "thermal_resistance", 3_usize, " FULL GLASS EQL "),
            ("nosum", "", 0_usize, " FULL GLASS EQL "),
        ] {
            let mut candidate = source_full_raw.clone();
            let rows = candidate
                .objects
                .get_mut(&ObjectType(GLASS_EQL.into()))
                .expect("only GlazingEQL family");
            let mut object = rows
                .remove(&ObjectName(" FULL GLASS EQL ".into()))
                .expect("full row");
            match kind {
                "kind" => {
                    object
                        .fields
                        .insert(FieldName(field.into()), RawValue::Bool(false));
                }
                "missing" => {
                    object.fields.remove(&FieldName(field.into()));
                }
                "lower" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            if matches!(
                                field,
                                "front_side_infrared_emissivity"
                                    | "back_side_infrared_emissivity"
                                    | "thermal_resistance"
                            ) {
                                "0.0"
                            } else {
                                "-0.1"
                            }
                            .into(),
                        ),
                    );
                }
                "upper" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            if matches!(
                                field,
                                "front_side_infrared_emissivity" | "back_side_infrared_emissivity"
                            ) {
                                "1.0"
                            } else {
                                "1.000001"
                            }
                            .into(),
                        ),
                    );
                }
                "auto" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::String(
                            match probe {
                                0 => "autocalculate",
                                1 => " Autocalculate ",
                                _ => " ",
                            }
                            .into(),
                        ),
                    );
                }
                "mode" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::String(
                            match probe {
                                0 => "spectralaverage",
                                1 => " SpectralAverage ",
                                _ => "UnknownMode",
                            }
                            .into(),
                        ),
                    );
                }
                "spectral" => {
                    object
                        .fields
                        .insert(FieldName(field.into()), RawValue::String("Spectral".into()));
                }
                "wire" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            match probe {
                                0 => "18446744073709551615",
                                1 => "9223372036854775808",
                                2 => "18446744073709551616",
                                _ => "1.8446744073709552e19",
                            }
                            .into(),
                        ),
                    );
                }
                "nosum" => {
                    let all_keys = [
                        "front_side_beam_beam_solar_transmittance",
                        "back_side_beam_beam_solar_transmittance",
                        "front_side_beam_beam_solar_reflectance",
                        "back_side_beam_beam_solar_reflectance",
                        "front_side_beam_beam_visible_solar_transmittance",
                        "back_side_beam_beam_visible_solar_transmittance",
                        "front_side_beam_beam_visible_solar_reflectance",
                        "back_side_beam_beam_visible_solar_reflectance",
                        "front_side_beam_diffuse_solar_transmittance",
                        "back_side_beam_diffuse_solar_transmittance",
                        "front_side_beam_diffuse_solar_reflectance",
                        "back_side_beam_diffuse_solar_reflectance",
                        "front_side_beam_diffuse_visible_solar_transmittance",
                        "back_side_beam_diffuse_visible_solar_transmittance",
                        "front_side_beam_diffuse_visible_solar_reflectance",
                        "back_side_beam_diffuse_visible_solar_reflectance",
                        "diffuse_diffuse_solar_transmittance",
                        "front_side_diffuse_diffuse_solar_reflectance",
                        "back_side_diffuse_diffuse_solar_reflectance",
                        "diffuse_diffuse_visible_solar_transmittance",
                        "front_side_diffuse_diffuse_visible_solar_reflectance",
                        "back_side_diffuse_diffuse_visible_solar_reflectance",
                        "infrared_transmittance_applies_to_front_and_back_",
                        "front_side_infrared_emissivity",
                        "back_side_infrared_emissivity",
                        "thermal_resistance",
                    ];
                    object
                        .fields
                        .extend(all_keys.iter().enumerate().map(|(index, key)| {
                            (
                                FieldName((*key).into()),
                                RawValue::Number(if index < 23 { "1.0" } else { "0.5" }.into()),
                            )
                        }));
                }
                _ => {}
            }
            rows.insert(ObjectName(name.into()), object);
            let source = super::super::compile_raw_model_for_source(&candidate);
            if kind == "name" {
                let model = source.model.as_ref().expect("legal standalone Source name");
                let expected = name.to_ascii_uppercase();
                assert!(
                    !source.has_errors()
                        && model.material_names.is_source_exact_namespace()
                        && model.materials.len() == 1
                        && model.materials[0].id == MaterialId(0)
                        && model.materials[0].kind() == MaterialKind::WindowGlazingEquivalentLayer
                        && model.materials[0].name.0 == expected
                        && model.material_names.resolve(name) == Some(MaterialId(0))
                        && model.material_names.len() == 1
                        && model.material_names.names()
                            == [ep_model::NormalizedName(expected.clone())]
                        && model.ctf_report_names.materials == [(MaterialId(0), expected)],
                    "standalone SourceExact {probe}"
                );
            } else if kind == "wire" {
                let token = match probe {
                    0 => "18446744073709551615",
                    1 => "9223372036854775808",
                    2 => "18446744073709551616",
                    _ => "1.8446744073709552e19",
                };
                let model = source.model.as_ref().expect("positive original N26");
                let row = model.materials[0]
                    .as_window_glazing_equivalent_layer()
                    .expect("advisory R");
                assert!(
                    !source.has_errors()
                        && model.material_names.is_source_exact_namespace()
                        && model.ctf_report_names.materials
                            == [(MaterialId(0), name.to_ascii_uppercase())]
                        && row.thermal_resistance_m2_k_per_w.to_bits()
                            == token.parse::<f64>()?.to_bits()
                        && row.thermal_resistance_m2_k_per_w > 0.0
                        && match (probe, ep_model::classify_source_json_number(token)) {
                            (0, Ok(ep_model::SourceJsonNumber::Integer(-1)))
                            | (1, Ok(ep_model::SourceJsonNumber::Integer(i64::MIN))) => true,
                            (2 | 3, Ok(ep_model::SourceJsonNumber::Float(value))) =>
                                value.to_bits() == (18446744073709551616.0_f64).to_bits(),
                            _ => false,
                        },
                    "N26 original/advisory/effective separation {probe}"
                );
            } else if kind == "nosum" {
                let legacy = compile_raw_model(&candidate);
                assert!(
                    !source.has_errors()
                        && !legacy.has_errors()
                        && source.model.as_ref().is_some_and(|model| model
                            .material_names
                            .is_source_exact_namespace()
                            && model.materials[0]
                                .as_window_glazing_equivalent_layer()
                                .is_some_and(|row| row.solar.beam_beam.front_transmittance == 1.0
                                    && row.solar.beam_beam.front_reflectance == 1.0
                                    && row.visible.beam_diffuse.front_transmittance == 1.0
                                    && row.visible.beam_diffuse.front_reflectance == 1.0
                                    && row.solar.diffuse_diffuse.transmittance
                                        == AutoOrNumber::Value(1.0)
                                    && row.front_infrared_emissivity == 0.5
                                    && row.thermal_resistance_m2_k_per_w == 0.5))
                        && legacy
                            .model
                            .as_ref()
                            .is_some_and(|model| !model.material_names.is_source_exact_namespace()),
                    "no invented optical sum gate"
                );
            } else {
                assert!(
                    source
                        .model
                        .as_ref()
                        .is_none_or(|model| !model.material_names.is_source_exact_namespace()
                            && model.ctf_report_names.materials.is_empty())
                        && (matches!(kind, "auto" | "mode") || source.has_errors()),
                    "original Source schema {kind}/{field}/{probe}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn equivalent_layer_applies_energyplus_defaults() -> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Defaults": {{{}}}
            }}
        }}"#,
        equivalent_layer_fields("")
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected defaulted equivalent-layer glazing"))?;
    let MaterialDefinition::WindowGlazingEquivalentLayer(fields) = model.materials[0].definition
    else {
        return Err(std::io::Error::other("expected equivalent-layer glazing variant").into());
    };
    assert_eq!(fields.visible.beam_beam.front_transmittance, 0.0);
    assert_eq!(fields.visible.beam_beam.back_reflectance, 0.0);
    assert_eq!(fields.solar.beam_diffuse.front_transmittance, 0.0);
    assert_eq!(fields.visible.beam_diffuse.back_reflectance, 0.0);
    assert_eq!(
        fields.solar.diffuse_diffuse.transmittance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.back_reflectance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(fields.infrared_transmittance, 0.0);
    assert_eq!(fields.front_infrared_emissivity, 0.84);
    assert_eq!(fields.back_infrared_emissivity, 0.84);
    assert_eq!(fields.thermal_resistance_m2_k_per_w, 0.158);
    for field in [
        "optical_data_type",
        "front_side_beam_diffuse_solar_transmittance",
        "diffuse_diffuse_solar_transmittance",
        "front_side_infrared_emissivity",
        "thermal_resistance",
    ] {
        assert!(
            result.report.defaults_applied.iter().any(|entry| {
                entry.object_type == "WindowMaterial:Glazing:EquivalentLayer"
                    && entry.object_name == "Defaults"
                    && entry.field == field
            }),
            "missing default application for {field}"
        );
    }
    Ok(())
}

#[test]
fn equivalent_layer_preserves_explicit_auto_and_zero_without_inventing_sum_constraints()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Source Actual": {{{}}}
            }}
        }}"#,
        equivalent_layer_fields(
            r#""front_side_beam_beam_visible_solar_transmittance":0.9,
               "front_side_beam_beam_visible_solar_reflectance":0.9,
               "diffuse_diffuse_solar_transmittance":"Autocalculate",
               "front_side_diffuse_diffuse_solar_reflectance":0.0,
               "infrared_transmittance_applies_to_front_and_back_":0.8,
               "front_side_infrared_emissivity":0.8"#,
        )
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(
        !result.has_errors(),
        "EquivalentLayer has individual bounds but no source optical-sum gate: {:?}",
        result.report.diagnostics
    );
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected source-actual equivalent layer"))?;
    let fields = model.materials[0]
        .as_window_glazing_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected equivalent-layer payload"))?;
    assert_eq!(
        fields.solar.diffuse_diffuse.transmittance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(
        fields.solar.diffuse_diffuse.front_reflectance,
        AutoOrNumber::Value(0.0)
    );
    assert_eq!(fields.infrared_transmittance, 0.8);
    assert_eq!(fields.front_infrared_emissivity, 0.8);
    Ok(())
}

#[test]
fn equivalent_layer_enforces_required_and_optional_bounds() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Missing": {},
                "Bad Required": {
                    "front_side_beam_beam_solar_transmittance":-0.1,
                    "back_side_beam_beam_solar_transmittance":1.1,
                    "front_side_beam_beam_solar_reflectance":-0.2,
                    "back_side_beam_beam_solar_reflectance":1.2
                },
                "Bad Optional": {
                    "front_side_beam_beam_solar_transmittance":0.5,
                    "back_side_beam_beam_solar_transmittance":0.5,
                    "front_side_beam_beam_solar_reflectance":0.2,
                    "back_side_beam_beam_solar_reflectance":0.2,
                    "front_side_beam_beam_visible_solar_transmittance":-0.1,
                    "back_side_beam_diffuse_solar_reflectance":1.1,
                    "diffuse_diffuse_solar_transmittance":-0.1,
                    "front_side_diffuse_diffuse_visible_solar_reflectance":1.1,
                    "infrared_transmittance_applies_to_front_and_back_":1.1,
                    "front_side_infrared_emissivity":0.0,
                    "back_side_infrared_emissivity":1.0,
                    "thermal_resistance":0.0
                },
                "Bad Autocalculate": {
                    "front_side_beam_beam_solar_transmittance":0.5,
                    "back_side_beam_beam_solar_transmittance":0.5,
                    "front_side_beam_beam_solar_reflectance":0.2,
                    "back_side_beam_beam_solar_reflectance":0.2,
                    "diffuse_diffuse_solar_transmittance":"Autosize"
                }
            }
        }"#,
    )
    .expect("invalid equivalent-layer inputs should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in [
        "front_side_beam_beam_solar_transmittance",
        "back_side_beam_beam_solar_transmittance",
        "front_side_beam_beam_solar_reflectance",
        "back_side_beam_beam_solar_reflectance",
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_name.as_deref() == Some("Missing")
                && diagnostic.field.as_deref() == Some(field)
        }));
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidNumericRange"
                && diagnostic.object_name.as_deref() == Some("Bad Required")
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    for field in [
        "front_side_beam_beam_visible_solar_transmittance",
        "back_side_beam_diffuse_solar_reflectance",
        "diffuse_diffuse_solar_transmittance",
        "front_side_diffuse_diffuse_visible_solar_reflectance",
        "infrared_transmittance_applies_to_front_and_back_",
        "front_side_infrared_emissivity",
        "back_side_infrared_emissivity",
        "thermal_resistance",
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidNumericRange"
                && diagnostic.object_name.as_deref() == Some("Bad Optional")
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Bad Autocalculate")
            && diagnostic.field.as_deref() == Some("diffuse_diffuse_solar_transmittance")
    }));
}

#[test]
fn equivalent_layer_rejects_spectral_and_invalid_optical_modes() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Spectral": {"optical_data_type":"Spectral"},
                "Bad Enum": {"optical_data_type":"BSDF"},
                "Bad Type": {"optical_data_type":42}
            }
        }"#,
    )
    .expect("invalid optical modes should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedWindowGlazingEquivalentLayerOpticalDataType"
            && diagnostic.object_name.as_deref() == Some("Spectral")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Bad Enum")
            && diagnostic.field.as_deref() == Some("optical_data_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidFieldType"
            && diagnostic.object_name.as_deref() == Some("Bad Type")
            && diagnostic.field.as_deref() == Some("optical_data_type")
    }));
}

#[test]
fn equivalent_layer_shares_names_with_prior_material_families() {
    let epjson = format!(
        r#"{{
            "Material": {{
                "Shared": {{
                    "roughness":"Rough","thickness":0.1,"conductivity":1.0,
                    "density":1000.0,"specific_heat":1000.0
                }}
            }},
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "shared": {{{}}}
            }}
        }}"#,
        equivalent_layer_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&epjson).expect("duplicate equivalent-layer name should parse"),
    );
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "WindowMaterial:Glazing:EquivalentLayer"
            && diagnostic.object_name.as_deref() == Some("shared")
    }));
}

#[test]
fn equivalent_layer_fails_closed_outside_its_deferred_construction_type() {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Equivalent Glass": {{{}}}
            }},
            "Construction": {{
                "Wrong Window": {{"outside_layer":"Equivalent Glass"}}
            }},
            "BuildingSurface:Detailed": {{
                "Wrong Surface": {{
                    "surface_type":"Wall",
                    "construction_name":"Wrong Window"
                }}
            }}
        }}"#,
        equivalent_layer_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&epjson).expect("equivalent-layer boundary should parse"),
    );

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_type == "BuildingSurface:Detailed"
            && diagnostic.object_name.as_deref() == Some("Wrong Surface")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
}
