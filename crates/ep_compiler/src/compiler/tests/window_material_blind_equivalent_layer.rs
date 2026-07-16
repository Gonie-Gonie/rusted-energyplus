use std::collections::{BTreeMap, BTreeSet};

use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    WindowBlindDirectionalOpticalProperties, WindowBlindEquivalentLayerSlatAngleControl,
    WindowBlindSlatOrientation, WindowShadeEquivalentLayerSideOpticalProperties,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:Blind:EquivalentLayer";

fn blind_equivalent_layer_fields(overrides: &[(&str, &str)]) -> String {
    let mut fields = BTreeMap::from([
        ("slat_orientation", "\"Horizontal\""),
        ("slat_width", "0.02"),
        ("slat_separation", "0.02"),
        ("slat_crown", "0.0"),
        ("slat_angle", "45.0"),
        ("front_side_slat_beam_diffuse_solar_transmittance", "0.1"),
        ("back_side_slat_beam_diffuse_solar_transmittance", "0.1"),
        ("front_side_slat_beam_diffuse_solar_reflectance", "0.2"),
        ("back_side_slat_beam_diffuse_solar_reflectance", "0.2"),
        ("front_side_slat_beam_diffuse_visible_transmittance", "0.1"),
        ("back_side_slat_beam_diffuse_visible_transmittance", "0.1"),
        ("front_side_slat_beam_diffuse_visible_reflectance", "0.2"),
        ("back_side_slat_beam_diffuse_visible_reflectance", "0.2"),
        ("slat_diffuse_diffuse_solar_transmittance", "0.1"),
        ("front_side_slat_diffuse_diffuse_solar_reflectance", "0.2"),
        ("back_side_slat_diffuse_diffuse_solar_reflectance", "0.2"),
        ("slat_diffuse_diffuse_visible_transmittance", "0.1"),
        ("front_side_slat_diffuse_diffuse_visible_reflectance", "0.2"),
        ("back_side_slat_diffuse_diffuse_visible_reflectance", "0.2"),
        ("slat_infrared_transmittance", "0.1"),
        ("front_side_slat_infrared_emissivity", "0.2"),
        ("back_side_slat_infrared_emissivity", "0.2"),
        ("slat_angle_control", "\"FixedSlatAngle\""),
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

fn compile_blind_equivalent_layer(
    object_name: &str,
    overrides: &[(&str, &str)],
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{"WindowMaterial:Blind:EquivalentLayer":{{"{object_name}":{{{}}}}}}}"#,
        blind_equivalent_layer_fields(overrides)
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
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

#[test]
fn blind_equivalent_layer_materializes_all_inputs_defaults_and_source_quirks()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Blind": {
                "Ordinary Before Equivalent Blind": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_solar_reflectance": 0.2,
                    "back_side_slat_beam_solar_reflectance": 0.3,
                    "front_side_slat_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_solar_reflectance": 0.3,
                    "slat_beam_visible_transmittance": 0.0
                }
            },
            "WindowMaterial:Blind:EquivalentLayer": {
                "Z Full Equivalent Blind": {
                    "slat_orientation": "Vertical",
                    "slat_width": 0.024,
                    "slat_separation": 0.018,
                    "slat_crown": 0.0006,
                    "slat_angle": 63.0,
                    "front_side_slat_beam_diffuse_solar_transmittance": 0.12,
                    "back_side_slat_beam_diffuse_solar_transmittance": 0.23,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.34,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.45,
                    "front_side_slat_beam_diffuse_visible_transmittance": 0.11,
                    "back_side_slat_beam_diffuse_visible_transmittance": 0.22,
                    "front_side_slat_beam_diffuse_visible_reflectance": 0.33,
                    "back_side_slat_beam_diffuse_visible_reflectance": 0.44,
                    "slat_diffuse_diffuse_solar_transmittance": 0.02,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.76,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.65,
                    "slat_diffuse_diffuse_visible_transmittance": 0.03,
                    "front_side_slat_diffuse_diffuse_visible_reflectance": 0.12,
                    "back_side_slat_diffuse_diffuse_visible_reflectance": 0.22,
                    "slat_infrared_transmittance": 0.02,
                    "front_side_slat_infrared_emissivity": 0.76,
                    "back_side_slat_infrared_emissivity": 0.65,
                    "slat_angle_control": "MaximizeSolar"
                },
                "A Default Equivalent Blind": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.25,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.35
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
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer blinds"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ORDINARY BEFORE EQUIVALENT BLIND",
            "A DEFAULT EQUIVALENT BLIND",
            "Z FULL EQUIVALENT BLIND"
        ]
    );

    let material = &model.materials[2];
    assert_eq!(material.kind(), MaterialKind::WindowBlindEquivalentLayer);
    assert_eq!(material.family(), MaterialFamily::EquivalentLayer);
    assert!(material.as_opaque().is_none());
    assert!(material.as_window_blind().is_none());
    assert_eq!(material.roughness(), None);
    assert_eq!(material.thickness_m(), None);
    assert_eq!(material.conductivity_w_per_m_k(), None);
    assert_eq!(material.density_kg_per_m3(), None);
    assert_eq!(material.specific_heat_j_per_kg_k(), None);
    assert_eq!(material.thermal_resistance(), None);
    assert_eq!(material.heat_capacity_per_area(), None);
    assert_eq!(material.thermal_absorptance(), None);
    assert_eq!(material.solar_absorptance(), None);
    assert_eq!(material.visible_absorptance(), None);

    let blind = material
        .as_window_blind_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer blind payload"))?;
    assert_eq!(blind.roughness, MaterialSurfaceRoughness::Rough);
    assert!(blind.is_resistance_only());
    assert_eq!(blind.nominal_thermal_resistance_m2_k_per_w(), None);
    assert_eq!(blind.slat_orientation, WindowBlindSlatOrientation::Vertical);
    assert_eq!(blind.slat_width_m, 0.024);
    assert_eq!(blind.slat_separation_m, 0.018);
    assert_eq!(blind.slat_crown_m, 0.0006);
    assert_eq!(blind.slat_angle_deg, 63.0);
    assert_eq!(
        blind.front_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.0,
            beam_diffuse_transmittance: 0.12,
            beam_diffuse_reflectance: 0.34,
        }
    );
    assert_eq!(
        blind.back_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.0,
            beam_diffuse_transmittance: 0.23,
            beam_diffuse_reflectance: 0.45,
        }
    );
    assert_eq!(
        blind.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.0,
            beam_diffuse_transmittance: 0.11,
            beam_diffuse_reflectance: 0.33,
        }
    );
    assert_eq!(
        blind.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.0,
            beam_diffuse_transmittance: 0.22,
            beam_diffuse_reflectance: 0.44,
        }
    );
    let expected_diffuse = WindowBlindDirectionalOpticalProperties {
        transmittance: 0.02,
        front_reflectance: 0.76,
        back_reflectance: 0.65,
    };
    assert_eq!(blind.solar_diffuse_diffuse, expected_diffuse);
    assert_eq!(
        blind.visible_diffuse_diffuse, expected_diffuse,
        "EnergyPlus 26.1 copies N13-N15 when the N16-N18 guard is satisfied"
    );
    assert_eq!(blind.infrared_transmittance, 0.02);
    assert_eq!(blind.front_infrared_emissivity, 0.76);
    assert_eq!(blind.back_infrared_emissivity, 0.65);
    assert_eq!(blind.front_thermal_absorptance, 0.76);
    assert_eq!(blind.back_thermal_absorptance, 0.65);
    assert_eq!(blind.thermal_transmittance, 0.02);
    assert_eq!(
        blind.slat_angle_control,
        WindowBlindEquivalentLayerSlatAngleControl::MaximizeSolar
    );

    let default = model.materials[1]
        .as_window_blind_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing default equivalent-layer blind"))?;
    assert_eq!(
        default.slat_orientation,
        WindowBlindSlatOrientation::Horizontal
    );
    assert_eq!(default.slat_crown_m, 0.0015);
    assert_eq!(default.slat_angle_deg, 45.0);
    assert_eq!(default.front_solar.beam_diffuse_transmittance, 0.0);
    assert_eq!(default.back_solar.beam_diffuse_transmittance, 0.0);
    assert_eq!(
        default.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        default.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        default.solar_diffuse_diffuse,
        WindowBlindDirectionalOpticalProperties::default(),
        "blank N13 suppresses the otherwise-required N14-N15 values"
    );
    assert_eq!(
        default.visible_diffuse_diffuse,
        WindowBlindDirectionalOpticalProperties::default()
    );
    assert_eq!(default.infrared_transmittance, 0.0);
    assert_eq!(default.front_infrared_emissivity, 0.0);
    assert_eq!(default.back_infrared_emissivity, 0.0);
    assert_eq!(default.front_thermal_absorptance, 0.0);
    assert_eq!(default.back_thermal_absorptance, 0.0);
    assert_eq!(default.thermal_transmittance, 0.0);
    assert_eq!(
        default.slat_angle_control,
        WindowBlindEquivalentLayerSlatAngleControl::FixedSlatAngle
    );
    for (field, value) in [
        ("slat_orientation", "Horizontal"),
        ("slat_crown", "0.0015"),
        ("slat_angle", "45.0"),
        ("front_side_slat_beam_diffuse_solar_transmittance", "0.0"),
        ("back_side_slat_beam_diffuse_solar_transmittance", "0.0"),
        ("front_side_slat_beam_diffuse_visible_transmittance", "0.0"),
        ("back_side_slat_beam_diffuse_visible_transmittance", "0.0"),
        ("slat_diffuse_diffuse_solar_transmittance", "0.0"),
        ("slat_infrared_transmittance", "0.0"),
        ("front_side_slat_infrared_emissivity", "0.0"),
        ("back_side_slat_infrared_emissivity", "0.0"),
        ("slat_angle_control", "FixedSlatAngle"),
    ] {
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == OBJECT_TYPE
                && default.object_name == "A Default Equivalent Blind"
                && default.field == field
                && default.value == value
        }));
    }

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer blind coverage"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 2);
    Ok(())
}

#[test]
fn blind_equivalent_layer_enforces_required_fields_schema_bounds_and_enums()
-> Result<(), Box<dyn std::error::Error>> {
    let missing = compile_raw_model(&parse_epjson_str(
        r#"{"WindowMaterial:Blind:EquivalentLayer":{"Missing Required":{}}}"#,
    )?);
    assert!(missing.has_errors());
    let required_fields = missing
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
            "slat_width".to_string(),
            "slat_separation".to_string(),
            "front_side_slat_beam_diffuse_solar_reflectance".to_string(),
            "back_side_slat_beam_diffuse_solar_reflectance".to_string(),
            "front_side_slat_diffuse_diffuse_solar_reflectance".to_string(),
            "back_side_slat_diffuse_diffuse_solar_reflectance".to_string(),
        ])
    );

    let invalid = compile_blind_equivalent_layer(
        "Bad Bounds",
        &[
            ("slat_orientation", "\"Diagonal\""),
            ("slat_width", "0.0"),
            ("slat_separation", "0.0"),
            ("slat_crown", "-0.01"),
            ("slat_angle", "90.0001"),
            ("front_side_slat_beam_diffuse_solar_transmittance", "1.0"),
            ("back_side_slat_beam_diffuse_solar_transmittance", "1.0"),
            ("front_side_slat_beam_diffuse_solar_reflectance", "1.0"),
            ("back_side_slat_beam_diffuse_solar_reflectance", "1.0"),
            ("front_side_slat_beam_diffuse_visible_transmittance", "1.0"),
            ("back_side_slat_beam_diffuse_visible_transmittance", "1.0"),
            ("front_side_slat_beam_diffuse_visible_reflectance", "1.0"),
            ("back_side_slat_beam_diffuse_visible_reflectance", "1.0"),
            ("slat_diffuse_diffuse_solar_transmittance", "1.0"),
            ("front_side_slat_diffuse_diffuse_solar_reflectance", "1.0"),
            ("back_side_slat_diffuse_diffuse_solar_reflectance", "1.0"),
            ("slat_diffuse_diffuse_visible_transmittance", "1.0"),
            ("front_side_slat_diffuse_diffuse_visible_reflectance", "1.0"),
            ("back_side_slat_diffuse_diffuse_visible_reflectance", "1.0"),
            ("slat_infrared_transmittance", "1.0"),
            ("front_side_slat_infrared_emissivity", "1.0"),
            ("back_side_slat_infrared_emissivity", "1.0"),
            ("slat_angle_control", "\"TrackSun\""),
        ],
    )?;
    assert!(invalid.has_errors());
    assert!(has_diagnostic(
        &invalid,
        "InvalidEnumValue",
        "Bad Bounds",
        Some("slat_orientation")
    ));
    assert!(has_diagnostic(
        &invalid,
        "InvalidEnumValue",
        "Bad Bounds",
        Some("slat_angle_control")
    ));
    for field in [
        "slat_width",
        "slat_separation",
        "slat_crown",
        "slat_angle",
        "front_side_slat_beam_diffuse_solar_transmittance",
        "back_side_slat_beam_diffuse_solar_transmittance",
        "front_side_slat_beam_diffuse_solar_reflectance",
        "back_side_slat_beam_diffuse_solar_reflectance",
        "front_side_slat_beam_diffuse_visible_transmittance",
        "back_side_slat_beam_diffuse_visible_transmittance",
        "front_side_slat_beam_diffuse_visible_reflectance",
        "back_side_slat_beam_diffuse_visible_reflectance",
        "slat_diffuse_diffuse_solar_transmittance",
        "front_side_slat_diffuse_diffuse_solar_reflectance",
        "back_side_slat_diffuse_diffuse_solar_reflectance",
        "slat_diffuse_diffuse_visible_transmittance",
        "front_side_slat_diffuse_diffuse_visible_reflectance",
        "back_side_slat_diffuse_diffuse_visible_reflectance",
        "slat_infrared_transmittance",
        "front_side_slat_infrared_emissivity",
        "back_side_slat_infrared_emissivity",
    ] {
        assert!(
            has_diagnostic(&invalid, "InvalidNumericRange", "Bad Bounds", Some(field)),
            "missing range diagnostic for {field}"
        );
    }

    let optical_fields = [
        "front_side_slat_beam_diffuse_solar_transmittance",
        "back_side_slat_beam_diffuse_solar_transmittance",
        "front_side_slat_beam_diffuse_solar_reflectance",
        "back_side_slat_beam_diffuse_solar_reflectance",
        "front_side_slat_beam_diffuse_visible_transmittance",
        "back_side_slat_beam_diffuse_visible_transmittance",
        "front_side_slat_beam_diffuse_visible_reflectance",
        "back_side_slat_beam_diffuse_visible_reflectance",
        "slat_diffuse_diffuse_solar_transmittance",
        "front_side_slat_diffuse_diffuse_solar_reflectance",
        "back_side_slat_diffuse_diffuse_solar_reflectance",
        "slat_diffuse_diffuse_visible_transmittance",
        "front_side_slat_diffuse_diffuse_visible_reflectance",
        "back_side_slat_diffuse_diffuse_visible_reflectance",
        "slat_infrared_transmittance",
        "front_side_slat_infrared_emissivity",
        "back_side_slat_infrared_emissivity",
    ];
    let mut opposite_overrides = vec![
        ("slat_width", "0.0250001"),
        ("slat_separation", "0.0250001"),
        ("slat_crown", "0.0015601"),
        ("slat_angle", "-90.0001"),
    ];
    opposite_overrides.extend(optical_fields.iter().map(|field| (*field, "-0.01")));
    let opposite = compile_blind_equivalent_layer("Opposite Bounds", &opposite_overrides)?;
    assert!(opposite.has_errors());
    for field in ["slat_width", "slat_separation", "slat_crown", "slat_angle"]
        .into_iter()
        .chain(optical_fields)
    {
        assert!(
            has_diagnostic(
                &opposite,
                "InvalidNumericRange",
                "Opposite Bounds",
                Some(field)
            ),
            "missing opposite range diagnostic for {field}"
        );
    }

    for (name, angle) in [("Lower Endpoints", "-90.0"), ("Upper Endpoints", "90.0")] {
        let accepted = compile_blind_equivalent_layer(
            name,
            &[
                ("slat_orientation", "\"  vErTiCaL  \""),
                ("slat_width", "0.025"),
                ("slat_separation", "0.025"),
                ("slat_crown", "0.00156"),
                ("slat_angle", angle),
                ("front_side_slat_beam_diffuse_solar_transmittance", "0.0"),
                ("back_side_slat_beam_diffuse_solar_transmittance", "0.0"),
                ("front_side_slat_beam_diffuse_solar_reflectance", "0.0"),
                ("back_side_slat_beam_diffuse_solar_reflectance", "0.0"),
                ("slat_angle_control", "\"  bLoCkBeAmSoLaR  \""),
            ],
        )?;
        assert!(
            !accepted.has_errors(),
            "{name} must compile: {:?}",
            accepted.report.diagnostics
        );
        let blind = accepted
            .model
            .as_ref()
            .and_then(|model| model.materials[0].as_window_blind_equivalent_layer())
            .ok_or_else(|| std::io::Error::other(format!("missing {name}")))?;
        assert_eq!(blind.slat_orientation, WindowBlindSlatOrientation::Vertical);
        assert_eq!(
            blind.slat_angle_control,
            WindowBlindEquivalentLayerSlatAngleControl::BlockBeamSolar
        );
    }
    Ok(())
}

#[test]
fn blind_equivalent_layer_enforces_only_four_raw_beam_optical_sums()
-> Result<(), Box<dyn std::error::Error>> {
    let cases: [(&str, [(&str, &str); 2]); 4] = [
        (
            "Front Solar Sum",
            [
                ("front_side_slat_beam_diffuse_solar_transmittance", "0.6"),
                ("front_side_slat_beam_diffuse_solar_reflectance", "0.4"),
            ],
        ),
        (
            "Back Solar Sum",
            [
                ("back_side_slat_beam_diffuse_solar_transmittance", "0.6"),
                ("back_side_slat_beam_diffuse_solar_reflectance", "0.4"),
            ],
        ),
        (
            "Front Visible Sum",
            [
                ("front_side_slat_beam_diffuse_visible_transmittance", "0.6"),
                ("front_side_slat_beam_diffuse_visible_reflectance", "0.4"),
            ],
        ),
        (
            "Back Visible Sum",
            [
                ("back_side_slat_beam_diffuse_visible_transmittance", "0.6"),
                ("back_side_slat_beam_diffuse_visible_reflectance", "0.4"),
            ],
        ),
    ];
    for (name, overrides) in cases {
        let result = compile_blind_equivalent_layer(name, &overrides)?;
        assert!(result.has_errors(), "{name} equality must fail");
        assert!(has_diagnostic(
            &result,
            "InvalidWindowBlindEquivalentLayerOpticalSum",
            name,
            None
        ));
    }

    let partial_visible = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Blind:EquivalentLayer": {
                "Partial Raw Visible Sum": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "front_side_slat_beam_diffuse_visible_transmittance": 0.6,
                    "front_side_slat_beam_diffuse_visible_reflectance": 0.4,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.2
                }
            }
        }"#,
    )?);
    assert!(has_diagnostic(
        &partial_visible,
        "InvalidWindowBlindEquivalentLayerOpticalSum",
        "Partial Raw Visible Sum",
        None
    ));

    let unchecked = compile_blind_equivalent_layer(
        "Unchecked Diffuse And Infrared Sums",
        &[
            ("slat_diffuse_diffuse_solar_transmittance", "0.7"),
            ("front_side_slat_diffuse_diffuse_solar_reflectance", "0.7"),
            ("back_side_slat_diffuse_diffuse_solar_reflectance", "0.7"),
            ("slat_diffuse_diffuse_visible_transmittance", "0.7"),
            ("front_side_slat_diffuse_diffuse_visible_reflectance", "0.7"),
            ("back_side_slat_diffuse_diffuse_visible_reflectance", "0.7"),
            ("slat_infrared_transmittance", "0.7"),
            ("front_side_slat_infrared_emissivity", "0.7"),
            ("back_side_slat_infrared_emissivity", "0.7"),
        ],
    )?;
    assert!(
        !unchecked.has_errors(),
        "diffuse and IR sums are not checked in the source reader: {:?}",
        unchecked.report.diagnostics
    );
    Ok(())
}

#[test]
fn blind_equivalent_layer_preserves_blank_group_guards_and_visible_copy_bug()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Blind:EquivalentLayer": {
                "A Explicit Blank And Cross Guard": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_beam_diffuse_visible_transmittance": "",
                    "back_side_slat_beam_diffuse_visible_transmittance": "",
                    "front_side_slat_beam_diffuse_visible_reflectance": "",
                    "back_side_slat_beam_diffuse_visible_reflectance": "",
                    "slat_diffuse_diffuse_solar_transmittance": "",
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.25,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.35,
                    "slat_diffuse_diffuse_visible_transmittance": 0.05,
                    "front_side_slat_diffuse_diffuse_visible_reflectance": 0.15,
                    "back_side_slat_diffuse_diffuse_visible_reflectance": 0.25,
                    "slat_infrared_transmittance": "",
                    "front_side_slat_infrared_emissivity": "",
                    "back_side_slat_infrared_emissivity": ""
                },
                "B Partial Groups": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_beam_diffuse_visible_transmittance": 0.11,
                    "front_side_slat_beam_diffuse_visible_reflectance": 0.22,
                    "back_side_slat_beam_diffuse_visible_reflectance": 0.33,
                    "slat_diffuse_diffuse_solar_transmittance": 0.04,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.24,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.34,
                    "slat_diffuse_diffuse_visible_transmittance": 0.05,
                    "back_side_slat_diffuse_diffuse_visible_reflectance": 0.25,
                    "front_side_slat_infrared_emissivity": 0.75,
                    "back_side_slat_infrared_emissivity": 0.2
                },
                "C Complete Groups": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_beam_diffuse_visible_transmittance": 0.11,
                    "back_side_slat_beam_diffuse_visible_transmittance": 0.12,
                    "front_side_slat_beam_diffuse_visible_reflectance": 0.21,
                    "back_side_slat_beam_diffuse_visible_reflectance": 0.22,
                    "slat_diffuse_diffuse_solar_transmittance": 0.04,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.24,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.34,
                    "slat_diffuse_diffuse_visible_transmittance": 0.05,
                    "front_side_slat_diffuse_diffuse_visible_reflectance": 0.15,
                    "back_side_slat_diffuse_diffuse_visible_reflectance": 0.25
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected blank-group cases"))?;

    let explicit_blank = model.materials[0]
        .as_window_blind_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing explicit-blank blind"))?;
    assert_eq!(
        explicit_blank.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        explicit_blank.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        explicit_blank.solar_diffuse_diffuse,
        WindowBlindDirectionalOpticalProperties::default()
    );
    assert_eq!(
        explicit_blank.visible_diffuse_diffuse,
        WindowBlindDirectionalOpticalProperties {
            transmittance: 0.0,
            front_reflectance: 0.25,
            back_reflectance: 0.35,
        },
        "the complete N16-N18 guard copies raw N13-N15 independently of the solar assignment guard"
    );
    assert_eq!(explicit_blank.infrared_transmittance, 0.0);
    assert_eq!(explicit_blank.front_infrared_emissivity, 0.0);
    assert_eq!(explicit_blank.back_infrared_emissivity, 0.0);
    for field in [
        "front_side_slat_beam_diffuse_visible_transmittance",
        "back_side_slat_beam_diffuse_visible_transmittance",
        "slat_diffuse_diffuse_solar_transmittance",
        "slat_infrared_transmittance",
        "front_side_slat_infrared_emissivity",
        "back_side_slat_infrared_emissivity",
    ] {
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == OBJECT_TYPE
                && default.object_name == "A Explicit Blank And Cross Guard"
                && default.field == field
                && default.value == "0.0"
        }));
    }

    let partial = model.materials[1]
        .as_window_blind_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing partial-groups blind"))?;
    assert_eq!(
        partial.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        partial.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        partial.solar_diffuse_diffuse,
        WindowBlindDirectionalOpticalProperties {
            transmittance: 0.04,
            front_reflectance: 0.24,
            back_reflectance: 0.34,
        }
    );
    assert_eq!(
        partial.visible_diffuse_diffuse,
        WindowBlindDirectionalOpticalProperties::default()
    );
    assert_eq!(partial.infrared_transmittance, 0.0);
    assert_eq!(partial.front_infrared_emissivity, 0.75);
    assert_eq!(partial.back_infrared_emissivity, 0.2);
    assert_eq!(partial.front_thermal_absorptance, 0.75);
    assert_eq!(partial.back_thermal_absorptance, 0.2);
    assert_eq!(partial.thermal_transmittance, 0.0);

    let complete = model.materials[2]
        .as_window_blind_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing complete-groups blind"))?;
    assert_eq!(complete.front_visible.beam_diffuse_transmittance, 0.11);
    assert_eq!(complete.back_visible.beam_diffuse_transmittance, 0.12);
    assert_eq!(complete.front_visible.beam_diffuse_reflectance, 0.21);
    assert_eq!(complete.back_visible.beam_diffuse_reflectance, 0.22);
    assert_eq!(
        complete.visible_diffuse_diffuse,
        complete.solar_diffuse_diffuse
    );
    assert_eq!(complete.visible_diffuse_diffuse.transmittance, 0.04);
    assert_eq!(complete.visible_diffuse_diffuse.front_reflectance, 0.24);
    assert_eq!(complete.visible_diffuse_diffuse.back_reflectance, 0.34);
    Ok(())
}

#[test]
fn blind_equivalent_layer_recovers_geometry_with_source_warning_order()
-> Result<(), Box<dyn std::error::Error>> {
    let warning_only = compile_blind_equivalent_layer(
        "Width Warning Only",
        &[("slat_width", "0.01"), ("slat_separation", "0.02")],
    )?;
    assert!(
        !warning_only.has_errors(),
        "{:?}",
        warning_only.report.diagnostics
    );
    assert!(has_diagnostic(
        &warning_only,
        "WindowBlindEquivalentLayerSlatWidthLessThanSeparation",
        "Width Warning Only",
        Some("slat_width")
    ));
    assert!(!has_diagnostic(
        &warning_only,
        "WindowBlindEquivalentLayerSlatWidthReset",
        "Width Warning Only",
        None
    ));
    let warning_only_blind = warning_only
        .model
        .as_ref()
        .and_then(|model| model.materials[0].as_window_blind_equivalent_layer())
        .ok_or_else(|| std::io::Error::other("missing warning-only blind"))?;
    assert_eq!(warning_only_blind.slat_width_m, 0.01);
    assert_eq!(warning_only_blind.slat_separation_m, 0.02);

    let cascade = compile_blind_equivalent_layer(
        "Cascade Recovery",
        &[("slat_width", "0.0004"), ("slat_separation", "0.0005")],
    )?;
    assert!(!cascade.has_errors(), "{:?}", cascade.report.diagnostics);
    let warning_codes = cascade
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Warning
                && diagnostic.object_type == OBJECT_TYPE
                && diagnostic.object_name.as_deref() == Some("Cascade Recovery")
        })
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        warning_codes,
        vec![
            "WindowBlindEquivalentLayerSlatWidthLessThanSeparation",
            "WindowBlindEquivalentLayerSlatSeparationReset",
            "WindowBlindEquivalentLayerSlatWidthReset",
        ]
    );
    let cascade_blind = cascade
        .model
        .as_ref()
        .and_then(|model| model.materials[0].as_window_blind_equivalent_layer())
        .ok_or_else(|| std::io::Error::other("missing recovered cascade blind"))?;
    assert_eq!(cascade_blind.slat_separation_m, 0.025);
    assert_eq!(cascade_blind.slat_width_m, 0.025);

    let ratio = compile_blind_equivalent_layer(
        "Width Then Crown Recovery",
        &[
            ("slat_width", "0.002"),
            ("slat_separation", "0.001"),
            ("slat_crown", "0.00075"),
        ],
    )?;
    assert!(!ratio.has_errors(), "{:?}", ratio.report.diagnostics);
    let ratio_warning_codes = ratio
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Warning
                && diagnostic.object_type == OBJECT_TYPE
                && diagnostic.object_name.as_deref() == Some("Width Then Crown Recovery")
        })
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        ratio_warning_codes,
        vec![
            "WindowBlindEquivalentLayerSlatWidthReset",
            "WindowBlindEquivalentLayerSlatCrownReset",
        ],
        "crown must be checked against the source-recovered width"
    );
    let ratio_blind = ratio
        .model
        .as_ref()
        .and_then(|model| model.materials[0].as_window_blind_equivalent_layer())
        .ok_or_else(|| std::io::Error::other("missing ratio-recovered blind"))?;
    assert_eq!(ratio_blind.slat_width_m, 0.001);
    assert_eq!(ratio_blind.slat_crown_m, 0.0);

    let crown = compile_blind_equivalent_layer(
        "Crown Recovery",
        &[
            ("slat_width", "0.002"),
            ("slat_separation", "0.002"),
            ("slat_crown", "0.001"),
        ],
    )?;
    assert!(!crown.has_errors(), "{:?}", crown.report.diagnostics);
    assert!(has_diagnostic(
        &crown,
        "WindowBlindEquivalentLayerSlatCrownReset",
        "Crown Recovery",
        Some("slat_crown")
    ));
    let crown_blind = crown
        .model
        .as_ref()
        .and_then(|model| model.materials[0].as_window_blind_equivalent_layer())
        .ok_or_else(|| std::io::Error::other("missing crown-recovered blind"))?;
    assert_eq!(crown_blind.slat_crown_m, 0.0);
    Ok(())
}

#[test]
fn blind_equivalent_layer_uses_shared_namespace_and_fails_closed_in_ordinary_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let namespace = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Blind": {
                "Shared": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_solar_reflectance": 0.2,
                    "back_side_slat_beam_solar_reflectance": 0.3,
                    "front_side_slat_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_solar_reflectance": 0.3,
                    "slat_beam_visible_transmittance": 0.0
                }
            },
            "WindowMaterial:Blind:EquivalentLayer": {
                "shared": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.3
                },
                "": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.3
                }
            }
        }"#,
    )?);
    assert!(namespace.has_errors());
    assert!(has_diagnostic(&namespace, "DuplicateName", "shared", None));
    assert!(has_diagnostic(
        &namespace,
        "MissingRequiredField",
        "",
        Some("name")
    ));

    let construction = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Blind:EquivalentLayer": {
                "Equivalent Blind": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.3
                }
            },
            "Construction": {
                "Wrong Window": {"outside_layer": "Equivalent Blind"}
            }
        }"#,
    )?);
    assert!(construction.has_errors());
    assert!(construction.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
    Ok(())
}
