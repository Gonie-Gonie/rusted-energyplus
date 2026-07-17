use std::collections::BTreeMap;

use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    NormalizedName, TypedModel, WindowDividerType, WindowFrameAndDividerId, WindowNfrcProductType,
};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawModel, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "WindowProperty:FrameAndDivider";
const WIDTH_RESET_WARNING: &str = "WindowFrameAndDividerWidthResetWithoutDividers";
const SILL_RESET_WARNING: &str = "WindowFrameAndDividerInsideSillDepthReset";

const NUMERIC_FIELDS: &[&str] = &[
    "frame_width",
    "frame_outside_projection",
    "frame_inside_projection",
    "frame_conductance",
    "ratio_of_frame_edge_glass_conductance_to_center_of_glass_conductance",
    "frame_solar_absorptance",
    "frame_visible_absorptance",
    "frame_thermal_hemispherical_emissivity",
    "divider_width",
    "number_of_horizontal_dividers",
    "number_of_vertical_dividers",
    "divider_outside_projection",
    "divider_inside_projection",
    "divider_conductance",
    "ratio_of_divider_edge_glass_conductance_to_center_of_glass_conductance",
    "divider_solar_absorptance",
    "divider_visible_absorptance",
    "divider_thermal_hemispherical_emissivity",
    "outside_reveal_solar_absorptance",
    "inside_sill_depth",
    "inside_sill_solar_absorptance",
    "inside_reveal_depth",
    "inside_reveal_solar_absorptance",
];

fn fields_json(overrides: &[(&str, &str)]) -> String {
    let mut fields = BTreeMap::new();
    for &(field, value) in overrides {
        fields.insert(field, value);
    }
    fields
        .into_iter()
        .map(|(field, value)| format!(r#""{field}":{value}"#))
        .collect::<Vec<_>>()
        .join(",")
}

fn compile_frame(
    object_name: &str,
    overrides: &[(&str, &str)],
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&format!(
        r#"{{"{OBJECT_TYPE}":{{"{object_name}":{{{}}}}}}}"#,
        fields_json(overrides)
    ))?;
    Ok(compile_raw_model(&raw))
}

fn frame_object_mut<'a>(
    raw: &'a mut RawModel,
    object_name: &str,
) -> Result<&'a mut ep_raw_model::RawObject, Box<dyn std::error::Error>> {
    raw.objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|instances| instances.get_mut(&ObjectName(object_name.to_string())))
        .ok_or_else(|| std::io::Error::other("missing raw frame-and-divider object").into())
}

fn has_diagnostic(
    result: &CompileResult,
    severity: DiagnosticSeverity,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == severity
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

#[test]
fn window_frame_and_divider_materializes_defaults_and_source_recoveries()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowProperty:FrameAndDivider": {
                "A Defaults": {},
                "B DividedLite Recovery": {
                    "frame_width": 0,
                    "frame_outside_projection": 0.2,
                    "frame_inside_projection": 0.3,
                    "divider_type": "dividedLITE",
                    "divider_width": 0.1,
                    "number_of_horizontal_dividers": 0.9,
                    "number_of_vertical_dividers": 0.2,
                    "divider_outside_projection": 0.21,
                    "divider_inside_projection": 0.31,
                    "inside_sill_depth": 0.1,
                    "inside_reveal_depth": 0.4
                },
                "C Suspended Full": {
                    "frame_width": 0.2,
                    "frame_outside_projection": 0.1,
                    "frame_inside_projection": 0.15,
                    "frame_conductance": 3.4,
                    "ratio_of_frame_edge_glass_conductance_to_center_of_glass_conductance": 2,
                    "frame_solar_absorptance": 0.4,
                    "frame_visible_absorptance": 0.5,
                    "frame_thermal_hemispherical_emissivity": 1.2,
                    "divider_type": "sUsPeNdEd",
                    "divider_width": 0.15,
                    "number_of_horizontal_dividers": 1.9,
                    "number_of_vertical_dividers": 2.9,
                    "divider_outside_projection": 0.11,
                    "divider_inside_projection": 0.12,
                    "divider_conductance": 4.5,
                    "ratio_of_divider_edge_glass_conductance_to_center_of_glass_conductance": 3,
                    "divider_solar_absorptance": 0.2,
                    "divider_visible_absorptance": 0.3,
                    "divider_thermal_hemispherical_emissivity": 0.8,
                    "outside_reveal_solar_absorptance": 0.4,
                    "inside_sill_depth": 0.6,
                    "inside_sill_solar_absorptance": 0.5,
                    "inside_reveal_depth": 0.4,
                    "inside_reveal_solar_absorptance": 0.6,
                    "nfrc_product_type_for_assembly_calculations": "fIxEd"
                },
                "D Zero Width Divider": {
                    "divider_width": 0,
                    "number_of_horizontal_dividers": 2,
                    "divider_outside_projection": 0.2,
                    "divider_inside_projection": 0.3
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed frame-and-divider model"))?;

    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing frame-and-divider coverage"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 4);
    assert_eq!(model.window_frame_and_dividers.len(), 4);
    assert_eq!(model.window_frame_and_divider_names.len(), 4);
    assert_eq!(model.object_count(), 5, "four definitions plus Timestep");
    assert_eq!(result.report.typed_object_count, model.object_count());

    let defaults = &model.window_frame_and_dividers[0];
    assert_eq!(defaults.id, WindowFrameAndDividerId(0));
    assert_eq!(defaults.name, NormalizedName::new("A Defaults"));
    assert_eq!(
        model.window_frame_and_divider_names.resolve(" a defaults "),
        Some(WindowFrameAndDividerId(0))
    );
    assert_eq!(defaults.frame.width_m, 0.0);
    assert_eq!(defaults.frame.outside_projection_m, 0.0);
    assert_eq!(defaults.frame.inside_projection_m, 0.0);
    assert_eq!(defaults.frame.conductance_w_per_m2_k, 0.0);
    assert_eq!(defaults.frame.edge_to_center_glass_conductance_ratio, 1.0);
    assert_eq!(defaults.frame.solar_absorptance, 0.7);
    assert_eq!(defaults.frame.visible_absorptance, 0.7);
    assert_eq!(defaults.frame.thermal_hemispherical_emissivity, 0.9);
    assert_eq!(defaults.frame.edge_width_m, 0.06355);
    assert_eq!(
        defaults.divider.divider_type,
        WindowDividerType::DividedLite
    );
    assert_eq!(defaults.divider.width_m, 0.0);
    assert_eq!(defaults.divider.horizontal_count, 0);
    assert_eq!(defaults.divider.vertical_count, 0);
    assert_eq!(defaults.divider.conductance_w_per_m2_k, 0.0);
    assert_eq!(defaults.divider.edge_to_center_glass_conductance_ratio, 1.0);
    assert_eq!(defaults.divider.solar_absorptance, 0.0);
    assert_eq!(defaults.divider.visible_absorptance, 0.0);
    assert_eq!(defaults.divider.thermal_hemispherical_emissivity, 0.9);
    assert_eq!(defaults.divider.edge_width_m, 0.06355);
    assert_eq!(defaults.reveal.outside_solar_absorptance, 0.0);
    assert_eq!(defaults.reveal.inside_sill_depth_m, 0.0);
    assert_eq!(defaults.reveal.inside_sill_solar_absorptance, 0.0);
    assert_eq!(defaults.reveal.inside_reveal_depth_m, 0.0);
    assert_eq!(defaults.reveal.inside_reveal_solar_absorptance, 0.0);
    assert_eq!(
        defaults.nfrc_product_type,
        WindowNfrcProductType::CurtainWall
    );
    assert_eq!(
        result
            .report
            .defaults_applied
            .iter()
            .filter(|entry| {
                entry.object_type == OBJECT_TYPE && entry.object_name == "A Defaults"
            })
            .count(),
        25,
        "all 23 numeric fields and both enums default"
    );

    let recovered = &model.window_frame_and_dividers[1];
    assert_eq!(recovered.frame.outside_projection_m, 0.0);
    assert_eq!(recovered.frame.inside_projection_m, 0.0);
    assert_eq!(recovered.divider.horizontal_count, 0);
    assert_eq!(recovered.divider.vertical_count, 0);
    assert_eq!(recovered.divider.width_m, 0.0);
    assert_eq!(
        recovered.divider.outside_projection_m, 0.21,
        "positive DividedLite projections survive the later no-divider width reset"
    );
    assert_eq!(recovered.divider.inside_projection_m, 0.31);
    assert_eq!(recovered.reveal.inside_sill_depth_m, 0.4);

    let full = &model.window_frame_and_dividers[2];
    assert_eq!(full.frame.width_m, 0.2);
    assert_eq!(full.frame.outside_projection_m, 0.1);
    assert_eq!(full.frame.inside_projection_m, 0.15);
    assert_eq!(full.frame.conductance_w_per_m2_k, 3.4);
    assert_eq!(full.frame.edge_to_center_glass_conductance_ratio, 2.0);
    assert_eq!(full.frame.solar_absorptance, 0.4);
    assert_eq!(full.frame.visible_absorptance, 0.5);
    assert_eq!(full.frame.thermal_hemispherical_emissivity, 1.2);
    assert_eq!(full.divider.divider_type, WindowDividerType::Suspended);
    assert_eq!(full.divider.width_m, 0.15);
    assert_eq!(full.divider.horizontal_count, 1);
    assert_eq!(full.divider.vertical_count, 2);
    assert_eq!(full.divider.outside_projection_m, 0.0);
    assert_eq!(full.divider.inside_projection_m, 0.0);
    assert_eq!(full.divider.conductance_w_per_m2_k, 4.5);
    assert_eq!(full.divider.edge_to_center_glass_conductance_ratio, 3.0);
    assert_eq!(full.divider.solar_absorptance, 0.2);
    assert_eq!(full.divider.visible_absorptance, 0.3);
    assert_eq!(full.divider.thermal_hemispherical_emissivity, 0.8);
    assert_eq!(full.reveal.outside_solar_absorptance, 0.4);
    assert_eq!(full.reveal.inside_sill_depth_m, 0.6);
    assert_eq!(full.reveal.inside_sill_solar_absorptance, 0.5);
    assert_eq!(full.reveal.inside_reveal_depth_m, 0.4);
    assert_eq!(full.reveal.inside_reveal_solar_absorptance, 0.6);
    assert_eq!(full.nfrc_product_type, WindowNfrcProductType::Fixed);

    let zero_width = &model.window_frame_and_dividers[3];
    assert_eq!(zero_width.divider.width_m, 0.0);
    assert_eq!(zero_width.divider.horizontal_count, 2);
    assert_eq!(zero_width.divider.outside_projection_m, 0.0);
    assert_eq!(zero_width.divider.inside_projection_m, 0.0);

    let width_warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == WIDTH_RESET_WARNING)
        .collect::<Vec<_>>();
    assert_eq!(width_warnings.len(), 1);
    assert_eq!(
        width_warnings[0].object_name.as_deref(),
        Some("B DividedLite Recovery")
    );
    let sill_warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == SILL_RESET_WARNING)
        .collect::<Vec<_>>();
    assert_eq!(sill_warnings.len(), 1);
    assert_eq!(
        sill_warnings[0].object_name.as_deref(),
        Some("B DividedLite Recovery")
    );
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Warning
                    && diagnostic.object_name.as_deref() == Some("B DividedLite Recovery")
            })
            .map(|diagnostic| diagnostic.code.as_str())
            .collect::<Vec<_>>(),
        vec![WIDTH_RESET_WARNING, SILL_RESET_WARNING],
        "source corrections must emit warnings in source order"
    );
    Ok(())
}

#[test]
fn window_frame_and_divider_parses_all_enums_case_insensitively_and_rejects_invalid_values()
-> Result<(), Box<dyn std::error::Error>> {
    let product_types = [
        ("casementdouble", WindowNfrcProductType::CasementDouble),
        ("CASEMENTSINGLE", WindowNfrcProductType::CasementSingle),
        ("dualAction", WindowNfrcProductType::DualAction),
        ("FIXED", WindowNfrcProductType::Fixed),
        ("garage", WindowNfrcProductType::Garage),
        ("GREENHOUSE", WindowNfrcProductType::Greenhouse),
        ("hingedEscape", WindowNfrcProductType::HingedEscape),
        ("HORIZONTALSLIDER", WindowNfrcProductType::HorizontalSlider),
        ("jal", WindowNfrcProductType::Jal),
        ("PIVOTED", WindowNfrcProductType::Pivoted),
        ("projectingSingle", WindowNfrcProductType::ProjectingSingle),
        ("PROJECTINGDUAL", WindowNfrcProductType::ProjectingDual),
        ("doorSidelite", WindowNfrcProductType::DoorSidelite),
        ("SKYLIGHT", WindowNfrcProductType::Skylight),
        ("slidingPatioDoor", WindowNfrcProductType::SlidingPatioDoor),
        ("CURTAINWALL", WindowNfrcProductType::CurtainWall),
        ("spandrelPanel", WindowNfrcProductType::SpandrelPanel),
        ("SIDEHINGEDDOOR", WindowNfrcProductType::SideHingedDoor),
        ("doorTransom", WindowNfrcProductType::DoorTransom),
        ("TROPICALAWNING", WindowNfrcProductType::TropicalAwning),
        (
            "tubularDaylightingDevice",
            WindowNfrcProductType::TubularDaylightingDevice,
        ),
        ("VERTICALSLIDER", WindowNfrcProductType::VerticalSlider),
    ];
    for (token, expected) in product_types {
        let value = format!(r#""{token}""#);
        let result = compile_frame(
            "Product",
            &[("nfrc_product_type_for_assembly_calculations", &value)],
        )?;
        assert!(
            !result.has_errors(),
            "{token}: {:?}",
            result.report.diagnostics
        );
        assert_eq!(
            result
                .model
                .as_ref()
                .ok_or_else(|| std::io::Error::other("expected enum model"))?
                .window_frame_and_dividers[0]
                .nfrc_product_type,
            expected,
            "token={token}"
        );
    }

    for (token, expected) in [
        ("dividedlite", WindowDividerType::DividedLite),
        ("SuSpEnDeD", WindowDividerType::Suspended),
    ] {
        let value = format!(r#""{token}""#);
        let result = compile_frame("Divider", &[("divider_type", &value)])?;
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert_eq!(
            result
                .model
                .as_ref()
                .ok_or_else(|| std::io::Error::other("expected divider enum model"))?
                .window_frame_and_dividers[0]
                .divider
                .divider_type,
            expected
        );
    }

    for field in [
        "divider_type",
        "nfrc_product_type_for_assembly_calculations",
    ] {
        let result = compile_frame("Invalid Enum", &[(field, r#""Unknown""#)])?;
        assert!(has_diagnostic(
            &result,
            DiagnosticSeverity::Error,
            "InvalidEnumValue",
            "Invalid Enum",
            Some(field)
        ));

        let result = compile_frame("Invalid Enum Type", &[(field, "1")])?;
        assert!(has_diagnostic(
            &result,
            DiagnosticSeverity::Error,
            "InvalidFieldType",
            "Invalid Enum Type",
            Some(field)
        ));
    }
    Ok(())
}

#[test]
fn window_frame_and_divider_accepts_numeric_endpoints_unbounded_values_and_truncates_counts()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_frame(
        "Endpoints",
        &[
            ("frame_width", "1"),
            ("frame_outside_projection", "0.5"),
            ("frame_inside_projection", "0.5"),
            ("frame_conductance", "1e308"),
            (
                "ratio_of_frame_edge_glass_conductance_to_center_of_glass_conductance",
                "4",
            ),
            ("frame_solar_absorptance", "1"),
            ("frame_visible_absorptance", "0"),
            ("frame_thermal_hemispherical_emissivity", "2.5"),
            ("divider_width", "0.5"),
            ("number_of_horizontal_dividers", "2147483647.5"),
            ("number_of_vertical_dividers", "1.999"),
            ("divider_outside_projection", "0.5"),
            ("divider_inside_projection", "0"),
            ("divider_conductance", "1e308"),
            (
                "ratio_of_divider_edge_glass_conductance_to_center_of_glass_conductance",
                "4",
            ),
            ("divider_solar_absorptance", "1"),
            ("divider_visible_absorptance", "0"),
            ("divider_thermal_hemispherical_emissivity", "0.999999"),
            ("outside_reveal_solar_absorptance", "1"),
            ("inside_sill_depth", "2"),
            ("inside_sill_solar_absorptance", "1"),
            ("inside_reveal_depth", "2"),
            ("inside_reveal_solar_absorptance", "1"),
        ],
    )?;
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let definition = &result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected endpoint model"))?
        .window_frame_and_dividers[0];
    assert_eq!(definition.divider.horizontal_count, 2_147_483_647);
    assert_eq!(definition.divider.vertical_count, 1);
    assert_eq!(definition.frame.conductance_w_per_m2_k, 1e308);
    assert_eq!(definition.divider.conductance_w_per_m2_k, 1e308);
    assert_eq!(definition.frame.thermal_hemispherical_emissivity, 2.5);

    let lower = compile_frame(
        "Lower Endpoints",
        &[
            ("frame_width", "0"),
            ("frame_conductance", "0"),
            (
                "ratio_of_frame_edge_glass_conductance_to_center_of_glass_conductance",
                "0.000001",
            ),
            ("frame_thermal_hemispherical_emissivity", "0.000001"),
            ("divider_width", "0"),
            ("number_of_horizontal_dividers", "0.999"),
            ("number_of_vertical_dividers", "0"),
            ("divider_conductance", "0"),
            (
                "ratio_of_divider_edge_glass_conductance_to_center_of_glass_conductance",
                "0.000001",
            ),
            ("divider_thermal_hemispherical_emissivity", "0.000001"),
            ("inside_sill_depth", "0"),
            ("inside_reveal_depth", "0"),
        ],
    )?;
    assert!(!lower.has_errors(), "{:?}", lower.report.diagnostics);
    assert_eq!(
        lower
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected lower-bound model"))?
            .window_frame_and_dividers[0]
            .divider
            .horizontal_count,
        0
    );
    Ok(())
}

#[test]
fn window_frame_and_divider_blank_fields_apply_their_source_defaults()
-> Result<(), Box<dyn std::error::Error>> {
    for field in NUMERIC_FIELDS {
        let result = compile_frame("Blank Numeric", &[(field, r#""""#)])?;
        assert!(
            !result.has_errors(),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == OBJECT_TYPE
                && default.object_name == "Blank Numeric"
                && default.field == *field
        }));
    }

    for field in [
        "divider_type",
        "nfrc_product_type_for_assembly_calculations",
    ] {
        let result = compile_frame("Blank Enum", &[(field, r#""""#)])?;
        assert!(
            !result.has_errors(),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == OBJECT_TYPE
                && default.object_name == "Blank Enum"
                && default.field == field
        }));
    }
    Ok(())
}

#[test]
fn window_frame_and_divider_rejects_every_numeric_range_and_nonfinite_input()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid_ranges = [
        ("frame_width", "1.1"),
        ("frame_outside_projection", "-0.1"),
        ("frame_inside_projection", "0.6"),
        ("frame_conductance", "-0.1"),
        (
            "ratio_of_frame_edge_glass_conductance_to_center_of_glass_conductance",
            "0",
        ),
        ("frame_solar_absorptance", "-0.1"),
        ("frame_visible_absorptance", "1.1"),
        ("frame_thermal_hemispherical_emissivity", "0"),
        ("divider_width", "0.6"),
        ("number_of_horizontal_dividers", "-0.1"),
        ("number_of_vertical_dividers", "2147483648"),
        ("divider_outside_projection", "-0.1"),
        ("divider_inside_projection", "0.6"),
        ("divider_conductance", "-0.1"),
        (
            "ratio_of_divider_edge_glass_conductance_to_center_of_glass_conductance",
            "4.1",
        ),
        ("divider_solar_absorptance", "-0.1"),
        ("divider_visible_absorptance", "1.1"),
        ("divider_thermal_hemispherical_emissivity", "0"),
        ("divider_thermal_hemispherical_emissivity", "1"),
        ("outside_reveal_solar_absorptance", "1.1"),
        ("inside_sill_depth", "2.1"),
        ("inside_sill_solar_absorptance", "-0.1"),
        ("inside_reveal_depth", "2.1"),
        ("inside_reveal_solar_absorptance", "1.1"),
    ];
    for (field, value) in invalid_ranges {
        let result = compile_frame("Invalid Range", &[(field, value)])?;
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                "InvalidNumericRange",
                "Invalid Range",
                Some(field)
            ),
            "field={field}, value={value}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }

    for field in NUMERIC_FIELDS {
        let mut raw = parse_epjson_str(&format!(r#"{{"{OBJECT_TYPE}":{{"Nonfinite":{{}}}}}}"#))?;
        frame_object_mut(&mut raw, "Nonfinite")?.fields.insert(
            FieldName((*field).to_string()),
            RawValue::Number("NaN".to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                "InvalidNumber",
                "Nonfinite",
                Some(field)
            ),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );

        let mut raw = parse_epjson_str(&format!(r#"{{"{OBJECT_TYPE}":{{"Wrong Type":{{}}}}}}"#))?;
        frame_object_mut(&mut raw, "Wrong Type")?.fields.insert(
            FieldName((*field).to_string()),
            RawValue::String("1".to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                "InvalidFieldType",
                "Wrong Type",
                Some(field)
            ),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for nonfinite in ["inf", "-inf"] {
        let mut raw = parse_epjson_str(&format!(r#"{{"{OBJECT_TYPE}":{{"Infinite":{{}}}}}}"#))?;
        frame_object_mut(&mut raw, "Infinite")?.fields.insert(
            FieldName("frame_conductance".to_string()),
            RawValue::Number(nonfinite.to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(has_diagnostic(
            &result,
            DiagnosticSeverity::Error,
            "InvalidNumber",
            "Infinite",
            Some("frame_conductance")
        ));
    }
    Ok(())
}

#[test]
fn window_frame_and_divider_uses_an_independent_casefolded_namespace_and_validates_before_reservation()
-> Result<(), Box<dyn std::error::Error>> {
    let same_other_type = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Shared Name": {"roughness":"Rough","thermal_resistance":1}
            },
            "Construction": {
                "Shared Name": {"outside_layer":"Shared Name"}
            },
            "WindowProperty:FrameAndDivider": {
                "Shared Name": {}
            }
        }"#,
    )?;
    let result = compile_raw_model(&same_other_type);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected separate namespaces"))?;
    assert!(model.material_names.resolve("shared name").is_some());
    assert!(model.construction_names.resolve("shared name").is_some());
    assert_eq!(
        model.window_frame_and_divider_names.resolve("SHARED NAME"),
        Some(WindowFrameAndDividerId(0))
    );

    let duplicate = parse_epjson_str(
        r#"{
            "WindowProperty:FrameAndDivider": {
                "Case Name": {},
                "case name": {"frame_width":0.1}
            }
        }"#,
    )?;
    let result = compile_raw_model(&duplicate);
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "DuplicateName",
        "case name",
        None
    ));
    assert!(result.model.is_none());

    let blank = compile_frame("   ", &[])?;
    assert!(has_diagnostic(
        &blank,
        DiagnosticSeverity::Error,
        "MissingRequiredField",
        "   ",
        Some("name")
    ));

    let raw = parse_epjson_str(
        r#"{
            "WindowProperty:FrameAndDivider": {
                "Reserve Me": {"frame_width":-1},
                "reserve me": {"frame_width":0.2}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_window_frame_and_dividers(&mut model);
    assert_eq!(model.window_frame_and_dividers.len(), 1);
    assert_eq!(
        model.window_frame_and_dividers[0].id,
        WindowFrameAndDividerId(0)
    );
    assert_eq!(
        model.window_frame_and_dividers[0].name,
        NormalizedName::new("reserve me")
    );
    assert_eq!(
        model.window_frame_and_divider_names.resolve("RESERVE ME"),
        Some(WindowFrameAndDividerId(0))
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_name.as_deref() == Some("Reserve Me")
            && diagnostic.field.as_deref() == Some("frame_width")
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "DuplicateName")
    );
    Ok(())
}
