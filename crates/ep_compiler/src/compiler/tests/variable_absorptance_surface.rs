use super::super::{Compiler, DiagnosticSeverity, compile_raw_model};
use ep_model::{
    MaterialVariableAbsorptanceId, SurfaceId, TypedModel, VariableAbsorptanceSurfaceBinding,
};
use ep_raw_model::{RawModel, parse_epjson_str};

fn surface(name: &str, construction: &str, boundary: &str, x: f64) -> String {
    format!(
        r#""{name}": {{
            "surface_type":"Wall",
            "construction_name":"{construction}",
            "zone_name":"Zone One",
            "outside_boundary_condition":"{boundary}",
            "vertices":[
                {{"vertex_x_coordinate":{x},"vertex_y_coordinate":0,"vertex_z_coordinate":0}},
                {{"vertex_x_coordinate":{x},"vertex_y_coordinate":1,"vertex_z_coordinate":0}},
                {{"vertex_x_coordinate":{x},"vertex_y_coordinate":1,"vertex_z_coordinate":1}},
                {{"vertex_x_coordinate":{x},"vertex_y_coordinate":0,"vertex_z_coordinate":1}}
            ]
        }}"#
    )
}

fn raw_fixture(
    materials: &str,
    constructions: &str,
    overlays: &str,
    surfaces: &str,
) -> Result<RawModel, Box<dyn std::error::Error>> {
    let surfaces = if surfaces.is_empty() {
        String::new()
    } else {
        format!(r#", "Zone": {{"Zone One":{{}}}}, "BuildingSurface:Detailed": {{{surfaces}}}"#)
    };
    Ok(parse_epjson_str(&format!(
        r#"{{
            "Material:NoMass": {{{materials}}},
            "Construction": {{{constructions}}},
            "MaterialProperty:VariableAbsorptance": {{{overlays}}}
            {surfaces}
        }}"#
    ))?)
}

fn overlay(name: &str, material: &str) -> String {
    format!(
        r#""{name}": {{
            "reference_material_name":"{material}",
            "control_signal":"Scheduled",
            "thermal_absorptance_schedule_name":"Constant-0.0"
        }}"#
    )
}

#[test]
fn outdoors_selection_preserves_surface_order_multiplicity_and_direct_overlay_ids()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = raw_fixture(
        r#"
            "Coating A":{"roughness":"Rough","thermal_resistance":1.0},
            "Coating B":{"roughness":"Rough","thermal_resistance":1.0},
            "Plain":{"roughness":"Rough","thermal_resistance":1.0}
        "#,
        r#"
            "A Coating A":{"outside_layer":"Coating A"},
            "B Coating B":{"outside_layer":"Coating B"},
            "C Plain":{"outside_layer":"Plain"}
        "#,
        &[
            overlay("A Overlay B", "Coating B"),
            overlay("Z Overlay A", "Coating A"),
        ]
        .join(","),
        &[
            surface("A Outdoor B", "B Coating B", "Outdoors", 0.0),
            surface("B Outdoor A", "A Coating A", "Outdoors", 1.0),
            surface("C Outdoor A Again", "A Coating A", "Outdoors", 2.0),
            surface("D Ground A", "A Coating A", "Ground", 3.0),
            surface("E Plain", "C Plain", "Outdoors", 4.0),
        ]
        .join(","),
    )?;

    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;

    assert_eq!(
        model.variable_absorptance_surface_bindings,
        vec![
            VariableAbsorptanceSurfaceBinding {
                surface: SurfaceId(0),
                variable_absorptance: MaterialVariableAbsorptanceId(0),
            },
            VariableAbsorptanceSurfaceBinding {
                surface: SurfaceId(1),
                variable_absorptance: MaterialVariableAbsorptanceId(1),
            },
            VariableAbsorptanceSurfaceBinding {
                surface: SurfaceId(2),
                variable_absorptance: MaterialVariableAbsorptanceId(1),
            },
        ]
    );
    assert_eq!(
        model.surface_names.resolve("A Outdoor B"),
        Some(SurfaceId(0))
    );
    assert_eq!(
        model.surface_names.resolve("B Outdoor A"),
        Some(SurfaceId(1))
    );
    assert_eq!(
        model.surface_names.resolve("C Outdoor A Again"),
        Some(SurfaceId(2))
    );
    assert_eq!(
        model
            .material_variable_absorptance_names
            .resolve("A Overlay B"),
        Some(MaterialVariableAbsorptanceId(0))
    );
    assert_eq!(
        model
            .material_variable_absorptance_names
            .resolve("Z Overlay A"),
        Some(MaterialVariableAbsorptanceId(1))
    );
    assert_eq!(result.report.typed_object_count, model.object_count());

    let warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "VariableAbsorptanceIgnoredOnNonOutdoorSurface")
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].severity, DiagnosticSeverity::Warning);
    assert_eq!(warnings[0].object_type, "BuildingSurface:Detailed");
    assert_eq!(warnings[0].object_name.as_deref(), Some("D GROUND A"));
    assert_eq!(
        warnings[0].field.as_deref(),
        Some("outside_boundary_condition")
    );
    Ok(())
}

#[test]
fn inside_layer_overlays_are_excluded_and_warn_once_per_construction_layer_in_order()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = raw_fixture(
        r#"
            "Coating":{"roughness":"Rough","thermal_resistance":1.0},
            "Plain":{"roughness":"Rough","thermal_resistance":1.0}
        "#,
        r#"
            "A Twice":{"outside_layer":"Plain","layer_2":"Coating","layer_3":"Coating"},
            "B Once":{"outside_layer":"Plain","layer_2":"Coating"}
        "#,
        &overlay("Coating Overlay", "Coating"),
        &surface("A Outdoor", "A Twice", "Outdoors", 0.0),
    )?;

    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert!(model.variable_absorptance_surface_bindings.is_empty());

    let warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "VariableAbsorptanceIgnoredOnInsideConstructionLayer"
        })
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 3);
    assert_eq!(
        warnings
            .iter()
            .map(|warning| (
                warning.object_name.as_deref(),
                warning.field.as_deref(),
                warning.severity,
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                Some("A TWICE"),
                Some("layer_2"),
                DiagnosticSeverity::Warning
            ),
            (
                Some("A TWICE"),
                Some("layer_3"),
                DiagnosticSeverity::Warning
            ),
            (Some("B ONCE"), Some("layer_2"), DiagnosticSeverity::Warning),
        ]
    );
    Ok(())
}

#[test]
fn surface_parse_errors_prevent_partial_binding_publication_and_warnings()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = raw_fixture(
        r#""Coating":{"roughness":"Rough","thermal_resistance":1.0}"#,
        r#""Coated":{"outside_layer":"Coating"}"#,
        &overlay("Coating Overlay", "Coating"),
        &[
            surface("A Retained Ground", "Coated", "Ground", 0.0),
            surface("B Invalid", "Missing Construction", "Outdoors", 1.0),
        ]
        .join(","),
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    compiler.parse_material_variable_absorptances(&mut model);
    compiler.parse_zones(&mut model);
    compiler.parse_space_data(&mut model);
    let diagnostics_before_surfaces = compiler.diagnostics.len();
    compiler.parse_surfaces(&mut model);
    compiler.build_variable_absorptance_surface_list(&mut model, diagnostics_before_surfaces);

    assert_eq!(model.surfaces.len(), 1);
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == "MissingReference"
            && diagnostic.object_name.as_deref() == Some("B Invalid")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
    assert!(model.variable_absorptance_surface_bindings.is_empty());
    assert!(compiler.diagnostics.iter().all(|diagnostic| {
        diagnostic.code != "VariableAbsorptanceIgnoredOnNonOutdoorSurface"
            && diagnostic.code != "VariableAbsorptanceIgnoredOnInsideConstructionLayer"
    }));
    Ok(())
}
