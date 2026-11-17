use std::collections::BTreeMap;

use super::super::{CompileResult, ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    ConstructionKind, MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    WindowBlindSlatAngleType, WindowBlindSlatOrientation,
};
use ep_raw_model::parse_epjson_str;

const GLASS_FIELDS: &str = r#""optical_data_type":"SpectralAverage","thickness":0.003"#;
const SHADE_FIELDS: &str = r#"
    "solar_transmittance":0.15,
    "solar_reflectance":0.35,
    "visible_transmittance":0.10,
    "visible_reflectance":0.40,
    "infrared_hemispherical_emissivity":0.80,
    "infrared_transmittance":0.10,
    "thickness":0.002,
    "conductivity":0.20
"#;
const SCREEN_FIELDS: &str = r#"
    "diffuse_solar_reflectance":0.2,
    "diffuse_visible_reflectance":0.3,
    "screen_material_spacing":0.002,
    "screen_material_diameter":0.0005
"#;

fn blind_fields(overrides: &[(&str, &str)]) -> String {
    let mut fields = BTreeMap::from([
        ("slat_orientation", "\"Horizontal\""),
        ("slat_width", "0.02"),
        ("slat_separation", "0.02"),
        ("slat_thickness", "0.00025"),
        ("slat_angle", "45.0"),
        ("slat_conductivity", "221.0"),
        ("slat_beam_solar_transmittance", "0.1"),
        ("front_side_slat_beam_solar_reflectance", "0.2"),
        ("back_side_slat_beam_solar_reflectance", "0.3"),
        ("slat_diffuse_solar_transmittance", "0.1"),
        ("front_side_slat_diffuse_solar_reflectance", "0.2"),
        ("back_side_slat_diffuse_solar_reflectance", "0.3"),
        ("slat_beam_visible_transmittance", "0.1"),
        ("front_side_slat_beam_visible_reflectance", "0.2"),
        ("back_side_slat_beam_visible_reflectance", "0.3"),
        ("slat_diffuse_visible_transmittance", "0.1"),
        ("front_side_slat_diffuse_visible_reflectance", "0.2"),
        ("back_side_slat_diffuse_visible_reflectance", "0.3"),
        ("slat_infrared_hemispherical_transmittance", "0.1"),
        ("front_side_slat_infrared_hemispherical_emissivity", "0.7"),
        ("back_side_slat_infrared_hemispherical_emissivity", "0.6"),
        ("blind_to_glass_distance", "0.05"),
        ("blind_top_opening_multiplier", "0.5"),
        ("blind_bottom_opening_multiplier", "0.0"),
        ("blind_left_side_opening_multiplier", "0.5"),
        ("blind_right_side_opening_multiplier", "0.5"),
        ("minimum_slat_angle", "0.0"),
        ("maximum_slat_angle", "180.0"),
    ]);
    for &(field, value) in overrides {
        fields.insert(field, value);
    }
    fields
        .into_iter()
        .map(|(field, value)| format!(r#""{field}":{value}"#))
        .collect::<Vec<_>>()
        .join(",")
}

fn compile_blind(
    object_name: &str,
    overrides: &[(&str, &str)],
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{"WindowMaterial:Blind":{{"{object_name}":{{{}}}}}}}"#,
        blind_fields(overrides)
    );
    Ok(compile_raw_model(&parse_epjson_str(&epjson)?))
}

fn has_diagnostic(
    result: &CompileResult,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == code
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

#[test]
fn blind_projects_all_source_fields_defaults_and_base_state()
-> Result<(), Box<dyn std::error::Error>> {
    let explicit_fields = blind_fields(&[
        ("slat_orientation", "\"Vertical\""),
        ("slat_width", "0.03"),
        ("slat_separation", "0.02"),
        ("slat_thickness", "0.001"),
        ("slat_angle", "60.0"),
        ("slat_conductivity", "15.0"),
        ("slat_beam_solar_transmittance", "0.11"),
        ("front_side_slat_beam_solar_reflectance", "0.22"),
        ("back_side_slat_beam_solar_reflectance", "0.33"),
        ("slat_diffuse_solar_transmittance", "0.11"),
        ("front_side_slat_diffuse_solar_reflectance", "0.22"),
        ("back_side_slat_diffuse_solar_reflectance", "0.33"),
        ("slat_beam_visible_transmittance", "0.14"),
        ("front_side_slat_beam_visible_reflectance", "0.25"),
        ("back_side_slat_beam_visible_reflectance", "0.35"),
        ("slat_diffuse_visible_transmittance", "0.14"),
        ("front_side_slat_diffuse_visible_reflectance", "0.25"),
        ("back_side_slat_diffuse_visible_reflectance", "0.35"),
        ("slat_infrared_hemispherical_transmittance", "0.12"),
        ("front_side_slat_infrared_hemispherical_emissivity", "0.70"),
        ("back_side_slat_infrared_hemispherical_emissivity", "0.60"),
        ("blind_to_glass_distance", "0.02"),
        ("blind_top_opening_multiplier", "0.1"),
        ("blind_bottom_opening_multiplier", "0.2"),
        ("blind_left_side_opening_multiplier", "0.3"),
        ("blind_right_side_opening_multiplier", "0.4"),
        ("minimum_slat_angle", "11.0"),
        ("maximum_slat_angle", "169.0"),
    ]);
    let epjson = format!(
        r#"{{
            "WindowMaterial:Blind": {{
                "Default Blind": {{
                    "slat_width":0.02,
                    "slat_separation":0.02,
                    "front_side_slat_beam_solar_reflectance":0.2,
                    "back_side_slat_beam_solar_reflectance":0.3,
                    "front_side_slat_diffuse_solar_reflectance":0.2,
                    "back_side_slat_diffuse_solar_reflectance":0.3,
                    "slat_beam_visible_transmittance":0.0
                }},
                "Explicit Blind": {{{explicit_fields}}}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed blind model"))?;
    assert_eq!(model.materials.len(), 2);

    let material = model
        .materials
        .iter()
        .find(|material| material.name.0 == "EXPLICIT BLIND")
        .ok_or_else(|| std::io::Error::other("missing explicit blind"))?;
    assert_eq!(material.kind(), MaterialKind::WindowBlind);
    assert_eq!(material.family(), MaterialFamily::Fenestration);
    assert!(material.as_opaque().is_none());
    assert_eq!(material.roughness(), None);
    assert_eq!(material.thickness_m(), None);
    assert_eq!(material.thermal_resistance(), None);
    let blind = material
        .as_window_blind()
        .ok_or_else(|| std::io::Error::other("expected blind payload"))?;
    assert_eq!(blind.roughness, MaterialSurfaceRoughness::Rough);
    assert!(blind.is_resistance_only());
    assert_eq!(blind.nominal_thermal_resistance_m2_k_per_w(), None);
    assert_eq!(blind.slat_orientation, WindowBlindSlatOrientation::Vertical);
    assert_eq!(blind.slat_width_m, 0.03);
    assert_eq!(blind.slat_separation_m, 0.02);
    assert_eq!(blind.slat_thickness_m, 0.001);
    assert_eq!(blind.slat_angle_deg, 60.0);
    assert_eq!(blind.slat_conductivity_w_per_m_k, 15.0);
    assert_eq!(blind.solar_beam_diffuse.transmittance, 0.11);
    assert_eq!(blind.solar_beam_diffuse.front_reflectance, 0.22);
    assert_eq!(blind.solar_beam_diffuse.back_reflectance, 0.33);
    assert_eq!(blind.solar_diffuse_diffuse.transmittance, 0.11);
    assert_eq!(blind.solar_diffuse_diffuse.front_reflectance, 0.22);
    assert_eq!(blind.solar_diffuse_diffuse.back_reflectance, 0.33);
    assert_eq!(blind.visible_beam_diffuse.transmittance, 0.14);
    assert_eq!(blind.visible_beam_diffuse.front_reflectance, 0.25);
    assert_eq!(blind.visible_beam_diffuse.back_reflectance, 0.35);
    assert_eq!(blind.visible_diffuse_diffuse.transmittance, 0.14);
    assert_eq!(blind.visible_diffuse_diffuse.front_reflectance, 0.25);
    assert_eq!(blind.visible_diffuse_diffuse.back_reflectance, 0.35);
    assert_eq!(blind.front_infrared_transmittance, 0.12);
    assert_eq!(blind.back_infrared_transmittance, 0.12);
    assert_eq!(blind.front_infrared_emissivity, 0.70);
    assert_eq!(blind.back_infrared_emissivity, 0.60);
    assert_eq!(blind.blind_to_glass_distance_m, 0.02);
    assert_eq!(blind.top_opening_multiplier, 0.1);
    assert_eq!(blind.bottom_opening_multiplier, 0.2);
    assert_eq!(blind.left_side_opening_multiplier, 0.3);
    assert_eq!(blind.right_side_opening_multiplier, 0.4);
    assert_eq!(blind.minimum_slat_angle_deg, 11.0);
    assert_eq!(blind.maximum_slat_angle_deg, 169.0);
    assert_eq!(blind.slat_angle_type, WindowBlindSlatAngleType::Fixed);
    assert_eq!(blind.slat_crown_m, 0.0);
    assert_eq!(blind.base_thickness_m, 0.0);
    assert_eq!(blind.base_conductivity_w_per_m_k, 0.0);
    assert_eq!(blind.base_thermal_resistance_m2_k_per_w, 0.0);
    assert_eq!(blind.base_solar_absorptance, 0.0);
    assert_eq!(blind.base_visible_absorptance, 0.0);
    assert_eq!(blind.base_thermal_absorptance, 0.0);

    let default = model
        .materials
        .iter()
        .find(|material| material.name.0 == "DEFAULT BLIND")
        .and_then(|material| material.as_window_blind())
        .ok_or_else(|| std::io::Error::other("missing default blind"))?;
    assert_eq!(
        default.slat_orientation,
        WindowBlindSlatOrientation::Horizontal
    );
    assert_eq!(default.slat_thickness_m, 0.00025);
    assert_eq!(default.slat_angle_deg, 45.0);
    assert_eq!(default.slat_conductivity_w_per_m_k, 221.0);
    assert_eq!(default.solar_beam_diffuse.transmittance, 0.0);
    assert_eq!(default.solar_diffuse_diffuse.transmittance, 0.0);
    assert_eq!(default.visible_beam_diffuse.front_reflectance, 0.0);
    assert_eq!(default.visible_beam_diffuse.back_reflectance, 0.0);
    assert_eq!(default.visible_diffuse_diffuse.transmittance, 0.0);
    assert_eq!(default.visible_diffuse_diffuse.front_reflectance, 0.0);
    assert_eq!(default.visible_diffuse_diffuse.back_reflectance, 0.0);
    assert_eq!(default.front_infrared_transmittance, 0.0);
    assert_eq!(default.back_infrared_transmittance, 0.0);
    assert_eq!(default.front_infrared_emissivity, 0.9);
    assert_eq!(default.back_infrared_emissivity, 0.9);
    assert_eq!(default.blind_to_glass_distance_m, 0.05);
    assert_eq!(default.top_opening_multiplier, 0.5);
    assert_eq!(default.bottom_opening_multiplier, 0.0);
    assert_eq!(default.left_side_opening_multiplier, 0.5);
    assert_eq!(default.right_side_opening_multiplier, 0.5);
    assert_eq!(default.minimum_slat_angle_deg, 0.0);
    assert_eq!(default.maximum_slat_angle_deg, 180.0);

    for (field, value) in [
        ("slat_orientation", "Horizontal"),
        ("slat_thickness", "0.00025"),
        ("slat_angle", "45.0"),
        ("slat_conductivity", "221.0"),
        ("slat_beam_solar_transmittance", "0.0"),
        ("slat_diffuse_solar_transmittance", "0.0"),
        ("slat_diffuse_visible_transmittance", "0.0"),
        ("slat_infrared_hemispherical_transmittance", "0.0"),
        ("front_side_slat_infrared_hemispherical_emissivity", "0.9"),
        ("back_side_slat_infrared_hemispherical_emissivity", "0.9"),
        ("blind_to_glass_distance", "0.05"),
        ("blind_top_opening_multiplier", "0.5"),
        ("blind_bottom_opening_multiplier", "0.0"),
        ("blind_left_side_opening_multiplier", "0.5"),
        ("blind_right_side_opening_multiplier", "0.5"),
        ("minimum_slat_angle", "0.0"),
        ("maximum_slat_angle", "180.0"),
    ] {
        assert!(
            result.report.defaults_applied.iter().any(|default| {
                default.object_type == "WindowMaterial:Blind"
                    && default.object_name == "Default Blind"
                    && default.field == field
                    && default.value == value
            }),
            "missing default record for {field}={value}"
        );
    }
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Blind")
        .ok_or_else(|| std::io::Error::other("missing blind coverage"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 2);

    // Source-only unused Blind: all preceding Legacy assertions and helper bodies stay literal.
    {
        use super::super::compile_raw_model_for_source;
        use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue};
        let minimal = r#""slat_width":0.02,"slat_separation":0.02,
            "front_side_slat_beam_solar_reflectance":0.2,
            "back_side_slat_beam_solar_reflectance":0.3,
            "front_side_slat_diffuse_solar_reflectance":0.2,
            "back_side_slat_diffuse_solar_reflectance":0.3,
            "slat_beam_visible_transmittance":0.0"#;
        let explicit = blind_fields(&[
            ("slat_orientation", "\"Vertical\""),
            ("slat_conductivity", "18446744073709551615"),
        ]);
        let positive = format!(
            r#"{{"WindowMaterial:Blind":{{
            " BLIND EXACT ":{{{explicit},"name":false,"idf_order":[],
                "metadata":[null,true,1.25,"opaque",{{"nested":18446744073709551615}}]}},
            "BLIND DEFAULT":{{{minimal}}},"\u000b":{{{minimal}}},"\u00a0":{{{minimal}}}
        }}}}"#
        );
        let positive_raw = parse_epjson_str(&positive)?;
        let source = compile_raw_model_for_source(&positive_raw);
        assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
        let source_model = source.model.as_ref().expect("genuine Source Blind model");
        let source_id = source_model
            .material_names
            .resolve(" BLIND EXACT ")
            .unwrap();
        assert!(
            source_model.material_names.is_source_exact_namespace()
                && source_model.material_names.resolve("BLIND EXACT").is_none()
                && source_model.material_names.resolve("\u{000b}").is_some()
                && source_model.material_names.resolve("\u{00a0}").is_some()
                && source_model.materials.len() == 4
                && source_model
                    .materials
                    .iter()
                    .all(|row| row.kind() == MaterialKind::WindowBlind)
                && source_model
                    .ctf_report_names
                    .materials
                    .contains(&(source_id, " BLIND EXACT ".into()))
        );
        let source_blind = source_model
            .materials
            .iter()
            .find(|row| row.id == source_id)
            .and_then(ep_model::Material::as_window_blind)
            .unwrap();
        let source_default = source_model
            .materials
            .iter()
            .find(|row| row.name.0 == "BLIND DEFAULT")
            .and_then(ep_model::Material::as_window_blind)
            .unwrap();
        assert!(
            source_blind.slat_orientation == WindowBlindSlatOrientation::Vertical
                && source_blind.slat_conductivity_w_per_m_k.to_bits()
                    == f64::from_bits(0x43f0_0000_0000_0000).to_bits()
                && source_blind.slat_angle_type == WindowBlindSlatAngleType::Fixed
                && source_blind.base_thickness_m == 0.0
                && source_blind.base_conductivity_w_per_m_k == 0.0
                && source_blind
                    .nominal_thermal_resistance_m2_k_per_w()
                    .is_none()
                && source_default.slat_orientation == WindowBlindSlatOrientation::Horizontal
                && [
                    source_default.slat_thickness_m,
                    source_default.slat_angle_deg,
                    source_default.slat_conductivity_w_per_m_k,
                    source_default.front_infrared_emissivity,
                    source_default.back_infrared_emissivity,
                    source_default.blind_to_glass_distance_m,
                    source_default.top_opening_multiplier,
                    source_default.bottom_opening_multiplier,
                    source_default.left_side_opening_multiplier,
                    source_default.right_side_opening_multiplier,
                    source_default.minimum_slat_angle_deg,
                    source_default.maximum_slat_angle_deg
                ]
                .map(f64::to_bits)
                    == [
                        0.00025_f64,
                        45.0,
                        221.0,
                        0.9,
                        0.9,
                        0.05,
                        0.5,
                        0.0,
                        0.5,
                        0.5,
                        0.0,
                        180.0
                    ]
                    .map(f64::to_bits)
                && source_default.visible_beam_diffuse.front_reflectance == 0.0
                && source_default.visible_beam_diffuse.back_reflectance == 0.0
                && source_default.visible_diffuse_diffuse.front_reflectance == 0.0
                && source_default.visible_diffuse_diffuse.back_reflectance == 0.0
        );
        let original = &positive_raw.objects[&ObjectType("WindowMaterial:Blind".into())]
            [&ObjectName(" BLIND EXACT ".into())]
            .fields;
        assert!(
            original.len() == 31
                && matches!(original.get(&FieldName("slat_conductivity".into())),Some(RawValue::Number(token))
                if token == "18446744073709551615" && matches!(ep_model::classify_source_json_number(token),Ok(ep_model::SourceJsonNumber::Integer(-1))))
                && original.get(&FieldName("name".into())) == Some(&RawValue::Bool(false))
                && matches!(original.get(&FieldName("idf_order".into())),Some(RawValue::Array(rows)) if rows.is_empty())
                && matches!(original.get(&FieldName("metadata".into())),Some(RawValue::Array(rows)) if rows.len()==5)
        );
        let joint_fields = blind_fields(&[
            ("slat_width", "0.04"),
            ("slat_angle", "0.0"),
            ("blind_to_glass_distance", "0.01"),
            ("slat_beam_solar_transmittance", "0.6"),
            ("front_side_slat_beam_solar_reflectance", "0.6"),
            ("back_side_slat_beam_solar_reflectance", "0.7"),
            ("slat_diffuse_solar_transmittance", "0.5"),
            ("front_side_slat_diffuse_solar_reflectance", "0.7"),
            ("back_side_slat_diffuse_solar_reflectance", "0.8"),
        ]);
        let joint_raw = parse_epjson_str(&format!(
            r#"{{"WindowMaterial:Blind":{{"JOINT":{{{joint_fields}}}}}}}"#
        ))?;
        let source_joint = compile_raw_model_for_source(&joint_raw);
        let legacy_joint = compile_raw_model(&joint_raw);
        assert!(
            !source_joint.has_errors()
                && legacy_joint.has_errors()
                && has_diagnostic(&legacy_joint, "InvalidWindowBlindOpticalSum", "JOINT", None)
                && has_diagnostic(
                    &legacy_joint,
                    "InvalidWindowBlindBeamDiffuseMismatch",
                    "JOINT",
                    None
                )
                && has_diagnostic(
                    &legacy_joint,
                    "InvalidWindowBlindToGlassDistance",
                    "JOINT",
                    None
                )
                && has_diagnostic(
                    &legacy_joint,
                    "InvalidWindowBlindSlatGeometry",
                    "JOINT",
                    None
                )
        );
        let warning_fields = blind_fields(&[("slat_width", "0.01")]);
        let warning_raw = parse_epjson_str(&format!(
            r#"{{"WindowMaterial:Blind":{{"WARNING":{{{warning_fields}}}}}}}"#
        ))?;
        let source_warning = compile_raw_model_for_source(&warning_raw);
        assert!(
            !source_warning.has_errors()
                && !has_diagnostic(
                    &source_warning,
                    "WindowBlindSlatWidthLessThanSeparation",
                    "WARNING",
                    None
                )
                && has_diagnostic(
                    &compile_raw_model(&warning_raw),
                    "WindowBlindSlatWidthLessThanSeparation",
                    "WARNING",
                    None
                )
        );
        // One finite table:28 wrong kinds,7 required omissions,all original numeric
        // lower/upper bounds,one invalid orientation andone original-RE2 blank name.
        let mut invalid_cases: Vec<(&str, Option<RawValue>)> = [
            "slat_orientation",
            "slat_width",
            "slat_separation",
            "slat_thickness",
            "slat_angle",
            "slat_conductivity",
            "slat_beam_solar_transmittance",
            "front_side_slat_beam_solar_reflectance",
            "back_side_slat_beam_solar_reflectance",
            "slat_diffuse_solar_transmittance",
            "front_side_slat_diffuse_solar_reflectance",
            "back_side_slat_diffuse_solar_reflectance",
            "slat_beam_visible_transmittance",
            "front_side_slat_beam_visible_reflectance",
            "back_side_slat_beam_visible_reflectance",
            "slat_diffuse_visible_transmittance",
            "front_side_slat_diffuse_visible_reflectance",
            "back_side_slat_diffuse_visible_reflectance",
            "slat_infrared_hemispherical_transmittance",
            "front_side_slat_infrared_hemispherical_emissivity",
            "back_side_slat_infrared_hemispherical_emissivity",
            "blind_to_glass_distance",
            "blind_top_opening_multiplier",
            "blind_bottom_opening_multiplier",
            "blind_left_side_opening_multiplier",
            "blind_right_side_opening_multiplier",
            "minimum_slat_angle",
            "maximum_slat_angle",
        ]
        .into_iter()
        .map(|field| (field, Some(RawValue::Bool(false))))
        .collect();
        invalid_cases.extend([
            ("slat_width", None),
            ("slat_separation", None),
            ("front_side_slat_beam_solar_reflectance", None),
            ("back_side_slat_beam_solar_reflectance", None),
            ("front_side_slat_diffuse_solar_reflectance", None),
            ("back_side_slat_diffuse_solar_reflectance", None),
            ("slat_beam_visible_transmittance", None),
        ]);
        invalid_cases.extend([
            ("slat_width", Some(RawValue::Number("0.0".into()))),
            ("slat_width", Some(RawValue::Number("2.0".into()))),
            ("slat_separation", Some(RawValue::Number("0.0".into()))),
            ("slat_separation", Some(RawValue::Number("2.0".into()))),
            ("slat_thickness", Some(RawValue::Number("0.0".into()))),
            ("slat_thickness", Some(RawValue::Number("1.1".into()))),
            ("slat_angle", Some(RawValue::Number("-1.0".into()))),
            ("slat_angle", Some(RawValue::Number("181.0".into()))),
            ("slat_conductivity", Some(RawValue::Number("0.0".into()))),
            (
                "slat_beam_solar_transmittance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "slat_beam_solar_transmittance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "front_side_slat_beam_solar_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "front_side_slat_beam_solar_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "back_side_slat_beam_solar_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "back_side_slat_beam_solar_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "slat_diffuse_solar_transmittance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "slat_diffuse_solar_transmittance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "front_side_slat_diffuse_solar_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "front_side_slat_diffuse_solar_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "back_side_slat_diffuse_solar_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "back_side_slat_diffuse_solar_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "slat_beam_visible_transmittance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "slat_beam_visible_transmittance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "front_side_slat_beam_visible_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "front_side_slat_beam_visible_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "back_side_slat_beam_visible_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "back_side_slat_beam_visible_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "slat_diffuse_visible_transmittance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "slat_diffuse_visible_transmittance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "front_side_slat_diffuse_visible_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "front_side_slat_diffuse_visible_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "back_side_slat_diffuse_visible_reflectance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "back_side_slat_diffuse_visible_reflectance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "slat_infrared_hemispherical_transmittance",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "slat_infrared_hemispherical_transmittance",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "front_side_slat_infrared_hemispherical_emissivity",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "front_side_slat_infrared_hemispherical_emissivity",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "back_side_slat_infrared_hemispherical_emissivity",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "back_side_slat_infrared_hemispherical_emissivity",
                Some(RawValue::Number("1.0".into())),
            ),
            (
                "blind_to_glass_distance",
                Some(RawValue::Number("-0.99".into())),
            ),
            (
                "blind_to_glass_distance",
                Some(RawValue::Number("2.0".into())),
            ),
            (
                "blind_top_opening_multiplier",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "blind_top_opening_multiplier",
                Some(RawValue::Number("2.0".into())),
            ),
            (
                "blind_bottom_opening_multiplier",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "blind_bottom_opening_multiplier",
                Some(RawValue::Number("2.0".into())),
            ),
            (
                "blind_left_side_opening_multiplier",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "blind_left_side_opening_multiplier",
                Some(RawValue::Number("2.0".into())),
            ),
            (
                "blind_right_side_opening_multiplier",
                Some(RawValue::Number("-1.0".into())),
            ),
            (
                "blind_right_side_opening_multiplier",
                Some(RawValue::Number("2.0".into())),
            ),
            ("minimum_slat_angle", Some(RawValue::Number("-1.0".into()))),
            ("minimum_slat_angle", Some(RawValue::Number("181.0".into()))),
            ("maximum_slat_angle", Some(RawValue::Number("-1.0".into()))),
            ("maximum_slat_angle", Some(RawValue::Number("181.0".into()))),
            (
                "slat_orientation",
                Some(RawValue::String("Diagonal".into())),
            ),
            ("__blank_name", None),
        ]);
        for (field, replacement) in invalid_cases {
            let mut invalid = positive_raw.clone();
            let rows = invalid
                .objects
                .get_mut(&ObjectType("WindowMaterial:Blind".into()))
                .unwrap();
            if field == "__blank_name" {
                let row = rows.remove(&ObjectName(" BLIND EXACT ".into())).unwrap();
                rows.insert(ObjectName(" \t".into()), row);
            } else {
                let object = rows.get_mut(&ObjectName(" BLIND EXACT ".into())).unwrap();
                if let Some(value) = replacement {
                    object.fields.insert(FieldName(field.into()), value);
                } else {
                    object.fields.remove(&FieldName(field.into()));
                }
            }
            assert!(
                compile_raw_model_for_source(&invalid).has_errors(),
                "original schema/name: {field}"
            );
        }
        let mut used_raw = positive_raw.clone();
        used_raw.objects.extend(
            parse_epjson_str(
                r#"{"Construction":{"BLIND USED":{"outside_layer":" BLIND EXACT "}}}"#,
            )?
            .objects,
        );
        assert!(compile_raw_model_for_source(&used_raw).has_errors());
    }
    Ok(())
}

#[test]
fn blind_enforces_required_fields_enum_and_exact_schema_bounds()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid_fields = blind_fields(&[
        ("slat_orientation", "\"Diagonal\""),
        ("slat_width", "0.0"),
        ("slat_separation", "1.01"),
        ("slat_thickness", "0.1001"),
        ("slat_angle", "180.1"),
        ("slat_conductivity", "0.0"),
        ("slat_beam_solar_transmittance", "1.0"),
        ("front_side_slat_beam_solar_reflectance", "-0.01"),
        ("back_side_slat_beam_solar_reflectance", "1.0"),
        ("slat_diffuse_solar_transmittance", "1.0"),
        ("front_side_slat_diffuse_solar_reflectance", "-0.01"),
        ("back_side_slat_diffuse_solar_reflectance", "1.0"),
        ("slat_beam_visible_transmittance", "1.0"),
        ("front_side_slat_beam_visible_reflectance", "-0.01"),
        ("back_side_slat_beam_visible_reflectance", "1.0"),
        ("slat_diffuse_visible_transmittance", "1.0"),
        ("front_side_slat_diffuse_visible_reflectance", "-0.01"),
        ("back_side_slat_diffuse_visible_reflectance", "1.0"),
        ("slat_infrared_hemispherical_transmittance", "1.0"),
        ("front_side_slat_infrared_hemispherical_emissivity", "-0.01"),
        ("back_side_slat_infrared_hemispherical_emissivity", "1.0"),
        ("blind_to_glass_distance", "1.01"),
        ("blind_top_opening_multiplier", "-0.01"),
        ("blind_bottom_opening_multiplier", "1.01"),
        ("blind_left_side_opening_multiplier", "-0.01"),
        ("blind_right_side_opening_multiplier", "1.01"),
        ("minimum_slat_angle", "-0.1"),
        ("maximum_slat_angle", "180.1"),
    ]);
    let epjson = format!(
        r#"{{
            "WindowMaterial:Blind": {{
                "Missing Required": {{}},
                "Bad Bounds": {{{invalid_fields}}}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);
    assert!(result.has_errors());

    for field in [
        "slat_width",
        "slat_separation",
        "front_side_slat_beam_solar_reflectance",
        "back_side_slat_beam_solar_reflectance",
        "front_side_slat_diffuse_solar_reflectance",
        "back_side_slat_diffuse_solar_reflectance",
        "slat_beam_visible_transmittance",
    ] {
        assert!(
            has_diagnostic(
                &result,
                "MissingRequiredField",
                "Missing Required",
                Some(field)
            ),
            "missing required-field diagnostic for {field}"
        );
    }
    assert!(has_diagnostic(
        &result,
        "InvalidEnumValue",
        "Bad Bounds",
        Some("slat_orientation")
    ));
    for field in [
        "slat_width",
        "slat_separation",
        "slat_thickness",
        "slat_angle",
        "slat_conductivity",
        "slat_beam_solar_transmittance",
        "front_side_slat_beam_solar_reflectance",
        "back_side_slat_beam_solar_reflectance",
        "slat_diffuse_solar_transmittance",
        "front_side_slat_diffuse_solar_reflectance",
        "back_side_slat_diffuse_solar_reflectance",
        "slat_beam_visible_transmittance",
        "front_side_slat_beam_visible_reflectance",
        "back_side_slat_beam_visible_reflectance",
        "slat_diffuse_visible_transmittance",
        "front_side_slat_diffuse_visible_reflectance",
        "back_side_slat_diffuse_visible_reflectance",
        "slat_infrared_hemispherical_transmittance",
        "front_side_slat_infrared_hemispherical_emissivity",
        "back_side_slat_infrared_hemispherical_emissivity",
        "blind_to_glass_distance",
        "blind_top_opening_multiplier",
        "blind_bottom_opening_multiplier",
        "blind_left_side_opening_multiplier",
        "blind_right_side_opening_multiplier",
        "minimum_slat_angle",
        "maximum_slat_angle",
    ] {
        assert!(
            has_diagnostic(&result, "InvalidNumericRange", "Bad Bounds", Some(field)),
            "missing range diagnostic for {field}"
        );
    }

    let endpoint_result = compile_blind(
        "Valid Endpoints",
        &[
            ("slat_orientation", "\"vertical\""),
            ("slat_width", "1.0"),
            ("slat_separation", "1.0"),
            ("slat_thickness", "0.1"),
            ("slat_angle", "180.0"),
            ("slat_conductivity", "0.000001"),
            ("slat_beam_solar_transmittance", "0.0"),
            ("front_side_slat_beam_solar_reflectance", "0.999999"),
            ("back_side_slat_beam_solar_reflectance", "0.999999"),
            ("slat_diffuse_solar_transmittance", "0.0"),
            ("front_side_slat_diffuse_solar_reflectance", "0.999999"),
            ("back_side_slat_diffuse_solar_reflectance", "0.999999"),
            ("slat_beam_visible_transmittance", "0.0"),
            ("front_side_slat_beam_visible_reflectance", "0.999999"),
            ("back_side_slat_beam_visible_reflectance", "0.999999"),
            ("slat_diffuse_visible_transmittance", "0.0"),
            ("front_side_slat_diffuse_visible_reflectance", "0.999999"),
            ("back_side_slat_diffuse_visible_reflectance", "0.999999"),
            ("slat_infrared_hemispherical_transmittance", "0.0"),
            (
                "front_side_slat_infrared_hemispherical_emissivity",
                "0.999999",
            ),
            (
                "back_side_slat_infrared_hemispherical_emissivity",
                "0.999999",
            ),
            ("blind_to_glass_distance", "1.0"),
            ("blind_top_opening_multiplier", "0.0"),
            ("blind_bottom_opening_multiplier", "1.0"),
            ("blind_left_side_opening_multiplier", "0.0"),
            ("blind_right_side_opening_multiplier", "1.0"),
            ("minimum_slat_angle", "0.0"),
            ("maximum_slat_angle", "180.0"),
        ],
    )?;
    assert!(
        !endpoint_result.has_errors(),
        "{:?}",
        endpoint_result.report.diagnostics
    );
    Ok(())
}

#[test]
fn blind_enforces_all_ten_strict_optical_sums() -> Result<(), Box<dyn std::error::Error>> {
    for (name, transmittance, reflectance) in [
        (
            "Solar Beam Front",
            "slat_beam_solar_transmittance",
            "front_side_slat_beam_solar_reflectance",
        ),
        (
            "Solar Beam Back",
            "slat_beam_solar_transmittance",
            "back_side_slat_beam_solar_reflectance",
        ),
        (
            "Solar Diffuse Front",
            "slat_diffuse_solar_transmittance",
            "front_side_slat_diffuse_solar_reflectance",
        ),
        (
            "Solar Diffuse Back",
            "slat_diffuse_solar_transmittance",
            "back_side_slat_diffuse_solar_reflectance",
        ),
        (
            "Visible Beam Front",
            "slat_beam_visible_transmittance",
            "front_side_slat_beam_visible_reflectance",
        ),
        (
            "Visible Beam Back",
            "slat_beam_visible_transmittance",
            "back_side_slat_beam_visible_reflectance",
        ),
        (
            "Visible Diffuse Front",
            "slat_diffuse_visible_transmittance",
            "front_side_slat_diffuse_visible_reflectance",
        ),
        (
            "Visible Diffuse Back",
            "slat_diffuse_visible_transmittance",
            "back_side_slat_diffuse_visible_reflectance",
        ),
        (
            "Infrared Front",
            "slat_infrared_hemispherical_transmittance",
            "front_side_slat_infrared_hemispherical_emissivity",
        ),
        (
            "Infrared Back",
            "slat_infrared_hemispherical_transmittance",
            "back_side_slat_infrared_hemispherical_emissivity",
        ),
    ] {
        let result = compile_blind(name, &[(transmittance, "0.6"), (reflectance, "0.4")])?;
        assert!(result.has_errors(), "{name} sum at one must be rejected");
        assert!(
            has_diagnostic(&result, "InvalidWindowBlindOpticalSum", name, None),
            "missing strict-sum diagnostic for {name}"
        );
    }
    Ok(())
}

#[test]
fn blind_uses_exact_source_equality_tolerance_for_six_beam_diffuse_pairs()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, beam_field, diffuse_field) in [
        (
            "Solar Transmittance",
            "slat_beam_solar_transmittance",
            "slat_diffuse_solar_transmittance",
        ),
        (
            "Solar Front Reflectance",
            "front_side_slat_beam_solar_reflectance",
            "front_side_slat_diffuse_solar_reflectance",
        ),
        (
            "Solar Back Reflectance",
            "back_side_slat_beam_solar_reflectance",
            "back_side_slat_diffuse_solar_reflectance",
        ),
        (
            "Visible Transmittance",
            "slat_beam_visible_transmittance",
            "slat_diffuse_visible_transmittance",
        ),
        (
            "Visible Front Reflectance",
            "front_side_slat_beam_visible_reflectance",
            "front_side_slat_diffuse_visible_reflectance",
        ),
        (
            "Visible Back Reflectance",
            "back_side_slat_beam_visible_reflectance",
            "back_side_slat_diffuse_visible_reflectance",
        ),
    ] {
        let exact = compile_blind(
            &format!("Exact {name}"),
            &[(beam_field, "0.0"), (diffuse_field, "0.00001")],
        )?;
        assert!(
            !exact.has_errors(),
            "exact 1e-5 difference for {name} was rejected: {:?}",
            exact.report.diagnostics
        );

        let over_name = format!("Over {name}");
        let over = compile_blind(
            &over_name,
            &[(beam_field, "0.0"), (diffuse_field, "0.0000100001")],
        )?;
        assert!(
            over.has_errors(),
            "difference over 1e-5 for {name} was accepted"
        );
        assert!(has_diagnostic(
            &over,
            "InvalidWindowBlindBeamDiffuseMismatch",
            &over_name,
            None
        ));
    }
    Ok(())
}

#[test]
fn blind_width_warning_is_nonfatal_and_equality_is_quiet() -> Result<(), Box<dyn std::error::Error>>
{
    let narrow = compile_blind("Narrow Slats", &[("slat_width", "0.019")])?;
    assert!(!narrow.has_errors(), "{:?}", narrow.report.diagnostics);
    assert!(narrow.model.is_some());
    assert!(has_diagnostic(
        &narrow,
        "WindowBlindSlatWidthLessThanSeparation",
        "Narrow Slats",
        Some("slat_width")
    ));

    let equal = compile_blind("Equal Slats", &[])?;
    assert!(!equal.has_errors(), "{:?}", equal.report.diagnostics);
    assert!(!has_diagnostic(
        &equal,
        "WindowBlindSlatWidthLessThanSeparation",
        "Equal Slats",
        None
    ));
    Ok(())
}

#[test]
fn blind_enforces_half_width_glass_clearance() -> Result<(), Box<dyn std::error::Error>> {
    let exact = compile_blind(
        "Exact Half Width",
        &[
            ("slat_width", "0.1"),
            ("slat_separation", "0.1"),
            ("blind_to_glass_distance", "0.05"),
        ],
    )?;
    assert!(!exact.has_errors(), "{:?}", exact.report.diagnostics);

    let below = compile_blind(
        "Below Half Width",
        &[
            ("slat_width", "0.1"),
            ("slat_separation", "0.1"),
            ("blind_to_glass_distance", "0.049999"),
        ],
    )?;
    assert!(has_diagnostic(
        &below,
        "InvalidWindowBlindToGlassDistance",
        "Below Half Width",
        Some("blind_to_glass_distance")
    ));
    Ok(())
}

#[test]
fn blind_geometry_uses_strict_overlap_gate_and_derived_angle_boundaries()
-> Result<(), Box<dyn std::error::Error>> {
    for angle in ["0.0", "180.0"] {
        let result = compile_blind(
            &format!("Touching {angle}"),
            &[
                ("slat_width", "0.03"),
                ("slat_separation", "0.02"),
                ("slat_thickness", "0.01"),
                ("slat_angle", angle),
            ],
        )?;
        assert!(
            !result.has_errors(),
            "S+T=W at {angle} degrees must be accepted: {:?}",
            result.report.diagnostics
        );
    }

    let minimum = (0.01_f64 / 0.03).asin().to_degrees();
    let maximum = 180.0 - minimum;
    for (name, angle) in [("Derived Minimum", minimum), ("Derived Maximum", maximum)] {
        let angle = angle.to_string();
        let result = compile_blind(
            name,
            &[
                ("slat_width", "0.04"),
                ("slat_separation", "0.02"),
                ("slat_thickness", "0.01"),
                ("slat_angle", angle.as_str()),
            ],
        )?;
        assert!(
            !result.has_errors(),
            "{name} must be accepted: {:?}",
            result.report.diagnostics
        );
    }
    for (name, angle) in [
        ("Below Derived Minimum", minimum - 1.0e-8),
        ("Above Derived Maximum", maximum + 1.0e-8),
    ] {
        let angle = angle.to_string();
        let result = compile_blind(
            name,
            &[
                ("slat_width", "0.04"),
                ("slat_separation", "0.02"),
                ("slat_thickness", "0.01"),
                ("slat_angle", angle.as_str()),
            ],
        )?;
        assert!(has_diagnostic(
            &result,
            "InvalidWindowBlindSlatGeometry",
            name,
            Some("slat_angle")
        ));
    }

    let unconstrained_input_interval = compile_blind(
        "Source Fixed Slat Interval",
        &[
            ("slat_angle", "45.0"),
            ("minimum_slat_angle", "180.0"),
            ("maximum_slat_angle", "0.0"),
        ],
    )?;
    assert!(
        !unconstrained_input_interval.has_errors(),
        "N26/N27 must not invent fixed-slat constraints: {:?}",
        unconstrained_input_interval.report.diagnostics
    );
    Ok(())
}

#[test]
fn blind_shares_the_material_name_registry() -> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "Material:NoMass": {{
                "Shared": {{"roughness":"Rough","thermal_resistance":1.0}}
            }},
            "WindowMaterial:Blind": {{"Shared": {{{}}}}}
        }}"#,
        blind_fields(&[])
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);
    assert!(has_diagnostic(&result, "DuplicateName", "Shared", None));
    Ok(())
}

#[test]
fn blind_constructions_accept_end_and_between_glass_source_topologies()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{"Glass": {{{GLASS_FIELDS}}}}},
            "WindowMaterial:Gas": {{
                "Air": {{"gas_type":"Air","thickness":0.01}},
                "Thin Air A": {{"gas_type":"Air","thickness":0.0005}},
                "Thin Air B": {{"gas_type":"Air","thickness":0.001}}
            }},
            "WindowMaterial:GasMixture": {{
                "Mixture A": {{
                    "thickness":0.01,"number_of_gases_in_mixture":2,
                    "gas_1_type":"Air","gas_1_fraction":0.6,
                    "gas_2_type":"Argon","gas_2_fraction":0.4
                }},
                "Mixture B": {{
                    "thickness":0.01,"number_of_gases_in_mixture":2,
                    "gas_1_type":"Air","gas_1_fraction":0.6,
                    "gas_2_type":"Argon","gas_2_fraction":0.4,
                    "gas_3_type":"Krypton","gas_3_fraction":0.9
                }}
            }},
            "WindowMaterial:Blind": {{
                "Blind": {{{}}},
                "Narrow Blind": {{{}}}
            }},
            "Construction": {{
                "Exterior Blind": {{"outside_layer":"Blind","layer_2":"Glass"}},
                "Interior Blind": {{"outside_layer":"Glass","layer_2":"Blind"}},
                "Between Double Exact Width": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Blind",
                    "layer_4":"Air","layer_5":"Glass"
                }},
                "Between Triple": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Glass",
                    "layer_4":"Air","layer_5":"Blind","layer_6":"Air",
                    "layer_7":"Glass"
                }},
                "Matching Mixtures": {{
                    "outside_layer":"Glass","layer_2":"Mixture A","layer_3":"Blind",
                    "layer_4":"Mixture B","layer_5":"Glass"
                }},
                "Thickness Tolerance": {{
                    "outside_layer":"Glass","layer_2":"Thin Air A",
                    "layer_3":"Narrow Blind","layer_4":"Thin Air B","layer_5":"Glass"
                }}
            }}
        }}"#,
        blind_fields(&[]),
        blind_fields(&[("slat_width", "0.0015"), ("slat_separation", "0.0015")])
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected blind constructions"))?;
    assert_eq!(model.constructions.len(), 6);
    assert!(
        model
            .constructions
            .iter()
            .all(|construction| construction.kind == ConstructionKind::Fenestration)
    );
    for (name, layers) in [
        ("EXTERIOR BLIND", 2),
        ("INTERIOR BLIND", 2),
        ("BETWEEN DOUBLE EXACT WIDTH", 5),
        ("BETWEEN TRIPLE", 7),
        ("MATCHING MIXTURES", 5),
        ("THICKNESS TOLERANCE", 5),
    ] {
        assert_eq!(
            model
                .constructions
                .iter()
                .find(|construction| construction.name.0 == name)
                .ok_or_else(|| std::io::Error::other(format!("missing {name}")))?
                .layers
                .len(),
            layers
        );
    }
    Ok(())
}

#[test]
fn blind_constructions_reject_invalid_devices_layering_and_gap_rules()
-> Result<(), Box<dyn std::error::Error>> {
    let diffusing_glass = format!(r#"{GLASS_FIELDS},"solar_diffusing":"Yes""#);
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{
                "Glass": {{{GLASS_FIELDS}}},
                "Diffusing Glass": {{{diffusing_glass}}}
            }},
            "WindowMaterial:Gas": {{
                "Air": {{"gas_type":"Air","thickness":0.01}},
                "Argon": {{"gas_type":"Argon","thickness":0.01}},
                "Air Wide": {{"gas_type":"Air","thickness":0.0106}},
                "Air Thin": {{"gas_type":"Air","thickness":0.009}}
            }},
            "WindowMaterial:Shade": {{"Shade": {{{SHADE_FIELDS}}}}},
            "WindowMaterial:Screen": {{"Screen": {{{SCREEN_FIELDS}}}}},
            "WindowMaterial:GasMixture": {{
                "Mixture 60 40": {{
                    "thickness":0.01,"number_of_gases_in_mixture":2,
                    "gas_1_type":"Air","gas_1_fraction":0.6,
                    "gas_2_type":"Argon","gas_2_fraction":0.4
                }},
                "Mixture 50 50": {{
                    "thickness":0.01,"number_of_gases_in_mixture":2,
                    "gas_1_type":"Air","gas_1_fraction":0.5,
                    "gas_2_type":"Argon","gas_2_fraction":0.5
                }}
            }},
            "WindowMaterial:Blind": {{
                "Blind One": {{{}}},
                "Blind Two": {{{}}},
                "Wide Blind": {{{}}}
            }},
            "Construction": {{
                "Two Blinds": {{
                    "outside_layer":"Blind One","layer_2":"Glass","layer_3":"Blind Two"
                }},
                "Blind And Shade": {{
                    "outside_layer":"Blind One","layer_2":"Glass","layer_3":"Shade"
                }},
                "Blind And Screen": {{
                    "outside_layer":"Screen","layer_2":"Glass","layer_3":"Blind One"
                }},
                "Diffusing With Blind": {{
                    "outside_layer":"Blind One","layer_2":"Diffusing Glass"
                }},
                "Exterior End Gap": {{
                    "outside_layer":"Blind One","layer_2":"Air","layer_3":"Glass"
                }},
                "Interior End Gap": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Blind One"
                }},
                "Misplaced Blind": {{
                    "outside_layer":"Glass","layer_2":"Blind One",
                    "layer_3":"Air","layer_4":"Glass"
                }},
                "Blind Alone": {{"outside_layer":"Blind One"}},
                "Wrong Triple Position": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Blind One",
                    "layer_4":"Air","layer_5":"Glass","layer_6":"Air","layer_7":"Glass"
                }},
                "Exterior Blank Hole": {{
                    "outside_layer":"Blind One","layer_3":"Glass"
                }},
                "Interior Blank Hole": {{
                    "outside_layer":"Glass","layer_3":"Blind One"
                }},
                "Composition Mismatch": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Blind One",
                    "layer_4":"Argon","layer_5":"Glass"
                }},
                "Fraction Mismatch": {{
                    "outside_layer":"Glass","layer_2":"Mixture 60 40",
                    "layer_3":"Blind One","layer_4":"Mixture 50 50","layer_5":"Glass"
                }},
                "Thickness Mismatch": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Blind One",
                    "layer_4":"Air Wide","layer_5":"Glass"
                }},
                "Gap Width Too Small": {{
                    "outside_layer":"Glass","layer_2":"Air Thin","layer_3":"Wide Blind",
                    "layer_4":"Air Thin","layer_5":"Glass"
                }}
            }}
        }}"#,
        blind_fields(&[]),
        blind_fields(&[]),
        blind_fields(&[("slat_width", "0.03"), ("slat_separation", "0.03")])
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);
    assert!(result.has_errors());

    for name in ["Two Blinds", "Blind And Shade"] {
        assert!(has_diagnostic(
            &result,
            "InvalidWindowBlindCount",
            name,
            None
        ));
    }
    assert!(has_diagnostic(
        &result,
        "InvalidWindowScreenCount",
        "Blind And Screen",
        None
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidSolarDiffusingGlazingWithBlind",
        "Diffusing With Blind",
        None
    ));
    for name in ["Exterior End Gap", "Interior End Gap"] {
        assert!(has_diagnostic(
            &result,
            "UnsafeWindowBlindEndLayering",
            name,
            None
        ));
    }
    for name in ["Misplaced Blind", "Blind Alone", "Wrong Triple Position"] {
        assert!(has_diagnostic(
            &result,
            "InvalidWindowBlindConstructionLayering",
            name,
            None
        ));
    }
    for name in ["Exterior Blank Hole", "Interior Blank Hole"] {
        assert!(has_diagnostic(
            &result,
            "NonContiguousConstructionLayers",
            name,
            Some("layer_2")
        ));
    }
    assert!(has_diagnostic(
        &result,
        "InvalidBetweenGlassBlindGasComposition",
        "Composition Mismatch",
        None
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidBetweenGlassBlindGasComposition",
        "Fraction Mismatch",
        None
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidBetweenGlassBlindGapThickness",
        "Thickness Mismatch",
        None
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidBetweenGlassBlindGapWidth",
        "Gap Width Too Small",
        None
    ));
    Ok(())
}
