use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{NormalizedName, SurfaceVaporCoefficientsId, TypedModel};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawModel, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "SurfaceProperties:VaporCoefficients";

fn one_surface_model(instance_name: &str, surface_name: &str, extra_fields: &str) -> String {
    format!(
        r#"{{
            "Material:NoMass": {{
                "R13": {{"roughness":"Rough","thermal_resistance":1}}
            }},
            "Construction": {{
                "Wall Construction": {{"outside_layer":"R13"}}
            }},
            "Zone": {{"Zone One": {{}}}},
            "BuildingSurface:Detailed": {{
                "Wall One": {{
                    "surface_type":"Wall",
                    "construction_name":"Wall Construction",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0}},
                        {{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0}},
                        {{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":0}}
                    ]
                }}
            }},
            "{OBJECT_TYPE}": {{
                "{instance_name}": {{"surface_name":"{surface_name}"{extra_fields}}}
            }}
        }}"#
    )
}

fn vapor_object_mut<'a>(
    raw: &'a mut RawModel,
    instance_name: &str,
) -> Result<&'a mut ep_raw_model::RawObject, Box<dyn std::error::Error>> {
    raw.objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|instances| instances.get_mut(&ObjectName(instance_name.to_string())))
        .ok_or_else(|| {
            std::io::Error::other("missing raw surface vapor-coefficients object").into()
        })
}

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: Option<&str>) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

#[test]
fn surface_vapor_coefficients_materialize_defaults_and_surface_targets()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&one_surface_model("Default Coefficients", "wAlL oNe", ""))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;

    assert_eq!(model.surface_vapor_coefficients.len(), 1);
    let coefficients = &model.surface_vapor_coefficients[0];
    assert_eq!(coefficients.id, SurfaceVaporCoefficientsId(0));
    assert_eq!(
        coefficients.name,
        NormalizedName::new("Default Coefficients")
    );
    assert_eq!(
        coefficients.reference_surface,
        model
            .surface_names
            .resolve("WALL ONE")
            .ok_or_else(|| std::io::Error::other("expected surface target"))?
    );
    assert!(!coefficients.external.is_constant);
    assert_eq!(coefficients.external.value_kg_per_pa_s_m2, 0.0);
    assert!(!coefficients.internal.is_constant);
    assert_eq!(coefficients.internal.value_kg_per_pa_s_m2, 0.0);

    assert!(
        model
            .material_heat_and_moisture_transfer_settings
            .is_empty()
    );
    assert!(
        model
            .material_heat_and_moisture_transfer_sorption_isotherms
            .is_empty()
    );
    assert!(
        model
            .material_heat_and_moisture_transfer_suctions
            .is_empty()
    );
    assert!(
        model
            .material_heat_and_moisture_transfer_redistributions
            .is_empty()
    );
    assert!(
        model
            .material_heat_and_moisture_transfer_diffusions
            .is_empty()
    );
    assert!(
        model
            .material_heat_and_moisture_transfer_thermal_conductivities
            .is_empty()
    );
    assert_eq!(model.object_count(), 6);
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE
            && entry.object_count == 1
            && entry.status == ObjectCoverageStatus::Typed
    }));
    Ok(())
}

#[test]
fn vapor_coefficients_preserve_four_side_combinations_blanks_and_nonsemantic_keys()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "R13": {"roughness":"Rough","thermal_resistance":1}
            },
            "Construction": {
                "Wall Construction": {"outside_layer":"R13"}
            },
            "Zone": {"Zone One": {}},
            "BuildingSurface:Detailed": {
                "Surface A": {
                    "surface_type":"Wall","construction_name":"Wall Construction","zone_name":"Zone One","outside_boundary_condition":"Outdoors",
                    "vertices":[{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":0}]
                },
                "Surface B": {
                    "surface_type":"Wall","construction_name":"Wall Construction","zone_name":"Zone One","outside_boundary_condition":"Outdoors",
                    "vertices":[{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1},{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1},{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":1}]
                },
                "Surface C": {
                    "surface_type":"Wall","construction_name":"Wall Construction","zone_name":"Zone One","outside_boundary_condition":"Outdoors",
                    "vertices":[{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":2},{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":2},{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":2}]
                },
                "Surface D": {
                    "surface_type":"Wall","construction_name":"Wall Construction","zone_name":"Zone One","outside_boundary_condition":"Outdoors",
                    "vertices":[{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":3},{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":3},{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":3}]
                }
            },
            "SurfaceProperties:VaporCoefficients": {
                "   ": {
                    "surface_name":"Surface A",
                    "constant_external_vapor_transfer_coefficient":"",
                    "external_vapor_coefficient_value":"",
                    "constant_internal_vapor_transfer_coefficient":"",
                    "internal_vapor_coefficient_value":""
                },
                "Curve": {
                    "surface_name":"Surface B",
                    "constant_external_vapor_transfer_coefficient":"yEs",
                    "external_vapor_coefficient_value":0,
                    "constant_internal_vapor_transfer_coefficient":"nO",
                    "internal_vapor_coefficient_value":9
                },
                "curve": {
                    "surface_name":"Surface C",
                    "constant_external_vapor_transfer_coefficient":"NO",
                    "external_vapor_coefficient_value":11,
                    "constant_internal_vapor_transfer_coefficient":"yes",
                    "internal_vapor_coefficient_value":0
                },
                "Mixed": {
                    "surface_name":"Surface D",
                    "constant_external_vapor_transfer_coefficient":"Yes",
                    "external_vapor_coefficient_value":1.25,
                    "constant_internal_vapor_transfer_coefficient":"YES",
                    "internal_vapor_coefficient_value":2.5
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.surface_vapor_coefficients.len(), 4);

    let for_surface = |surface_name: &str| {
        let surface = model
            .surface_names
            .resolve(surface_name)
            .expect("expected surface");
        model
            .surface_vapor_coefficients
            .iter()
            .find(|coefficients| coefficients.reference_surface == surface)
            .expect("expected vapor coefficients")
    };
    let neither = for_surface("Surface A");
    assert_eq!(neither.name, NormalizedName::new("   "));
    assert_eq!(
        (
            neither.external.is_constant,
            neither.external.value_kg_per_pa_s_m2,
            neither.internal.is_constant,
            neither.internal.value_kg_per_pa_s_m2,
        ),
        (false, 0.0, false, 0.0)
    );

    let external_only = for_surface("Surface B");
    assert_eq!(
        (
            external_only.external.is_constant,
            external_only.external.value_kg_per_pa_s_m2,
            external_only.internal.is_constant,
            external_only.internal.value_kg_per_pa_s_m2,
        ),
        (true, 0.0, false, 9.0)
    );
    let internal_only = for_surface("Surface C");
    assert_eq!(
        (
            internal_only.external.is_constant,
            internal_only.external.value_kg_per_pa_s_m2,
            internal_only.internal.is_constant,
            internal_only.internal.value_kg_per_pa_s_m2,
        ),
        (false, 11.0, true, 0.0)
    );
    assert_eq!(external_only.name, internal_only.name);

    let both = for_surface("Surface D");
    assert_eq!(
        (
            both.external.is_constant,
            both.external.value_kg_per_pa_s_m2,
            both.internal.is_constant,
            both.internal.value_kg_per_pa_s_m2,
        ),
        (true, 1.25, true, 2.5)
    );
    Ok(())
}

#[test]
fn vapor_coefficients_accept_large_finite_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut raw = parse_epjson_str(&one_surface_model(
        "Large",
        "Wall One",
        r#", "constant_external_vapor_transfer_coefficient":"Yes", "constant_internal_vapor_transfer_coefficient":"Yes""#,
    ))?;
    let object = vapor_object_mut(&mut raw, "Large")?;
    for field in [
        "external_vapor_coefficient_value",
        "internal_vapor_coefficient_value",
    ] {
        object.fields.insert(
            FieldName(field.to_string()),
            RawValue::Number(f64::MAX.to_string()),
        );
    }

    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let coefficients = &result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?
        .surface_vapor_coefficients[0];
    assert_eq!(
        coefficients.external.value_kg_per_pa_s_m2.to_bits(),
        f64::MAX.to_bits()
    );
    assert_eq!(
        coefficients.internal.value_kg_per_pa_s_m2.to_bits(),
        f64::MAX.to_bits()
    );
    Ok(())
}

#[test]
fn vapor_coefficients_validate_inactive_numeric_fields_and_field_types()
-> Result<(), Box<dyn std::error::Error>> {
    for field in [
        "external_vapor_coefficient_value",
        "internal_vapor_coefficient_value",
    ] {
        for (raw_value, code) in [
            (RawValue::Number("-0.1".to_string()), "InvalidNumericRange"),
            (RawValue::Number("NaN".to_string()), "InvalidNumber"),
            (RawValue::Number("inf".to_string()), "InvalidNumber"),
            (RawValue::Number("-inf".to_string()), "InvalidNumber"),
            (RawValue::String("1".to_string()), "InvalidFieldType"),
        ] {
            let mut raw = parse_epjson_str(&one_surface_model(
                "Invalid Numeric",
                "Wall One",
                r#", "constant_external_vapor_transfer_coefficient":"No", "constant_internal_vapor_transfer_coefficient":"No""#,
            ))?;
            vapor_object_mut(&mut raw, "Invalid Numeric")?
                .fields
                .insert(FieldName(field.to_string()), raw_value);
            let result = compile_raw_model(&raw);
            assert!(
                has_error(&result, code, "Invalid Numeric", Some(field)),
                "field={field}, code={code}, diagnostics={:?}",
                result.report.diagnostics
            );
        }
    }

    for field in [
        "constant_external_vapor_transfer_coefficient",
        "constant_internal_vapor_transfer_coefficient",
    ] {
        for (raw_value, code) in [
            (RawValue::String("Maybe".to_string()), "InvalidEnumValue"),
            (RawValue::Number("1".to_string()), "InvalidFieldType"),
        ] {
            let mut raw = parse_epjson_str(&one_surface_model("Invalid Enum", "Wall One", ""))?;
            vapor_object_mut(&mut raw, "Invalid Enum")?
                .fields
                .insert(FieldName(field.to_string()), raw_value);
            let result = compile_raw_model(&raw);
            assert!(
                has_error(&result, code, "Invalid Enum", Some(field)),
                "field={field}, code={code}, diagnostics={:?}",
                result.report.diagnostics
            );
        }
    }

    let mut raw = parse_epjson_str(&one_surface_model("Invalid Surface Type", "Wall One", ""))?;
    vapor_object_mut(&mut raw, "Invalid Surface Type")?
        .fields
        .insert(
            FieldName("surface_name".to_string()),
            RawValue::Number("1".to_string()),
        );
    let result = compile_raw_model(&raw);
    assert!(has_error(
        &result,
        "InvalidFieldType",
        "Invalid Surface Type",
        Some("surface_name")
    ));
    Ok(())
}

#[test]
fn vapor_coefficients_require_a_building_surface_target() -> Result<(), Box<dyn std::error::Error>>
{
    let mut missing = parse_epjson_str(&one_surface_model("Missing", "Wall One", ""))?;
    vapor_object_mut(&mut missing, "Missing")?
        .fields
        .remove(&FieldName("surface_name".to_string()));
    let result = compile_raw_model(&missing);
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "Missing",
        Some("surface_name")
    ));

    let blank = parse_epjson_str(&one_surface_model("Blank", "   ", ""))?;
    let result = compile_raw_model(&blank);
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "Blank",
        Some("surface_name")
    ));

    let nonexistent = parse_epjson_str(&one_surface_model("Unknown", "Not A Surface", ""))?;
    let result = compile_raw_model(&nonexistent);
    assert!(has_error(
        &result,
        "MissingReference",
        "Unknown",
        Some("surface_name")
    ));

    let wrong_object_type = parse_epjson_str(&one_surface_model("Material Target", "R13", ""))?;
    let result = compile_raw_model(&wrong_object_type);
    assert!(has_error(
        &result,
        "MissingReference",
        "Material Target",
        Some("surface_name")
    ));
    Ok(())
}

#[test]
fn vapor_coefficients_duplicate_target_fails_and_invalid_first_does_not_reserve()
-> Result<(), Box<dyn std::error::Error>> {
    let duplicate = parse_epjson_str(&one_surface_model("A", "Wall One", "").replace(
        r#""A": {"surface_name":"Wall One"}"#,
        r#""A": {"surface_name":"Wall One"}, "B": {"surface_name":"wall one"}"#,
    ))?;
    let result = compile_raw_model(&duplicate);
    assert!(has_error(
        &result,
        "DuplicateSurfaceVaporCoefficientsSurface",
        "B",
        Some("surface_name")
    ));
    assert!(result.model.is_none(), "duplicate target must fail closed");

    let raw = parse_epjson_str(
        &one_surface_model("A", "Wall One", "").replace(
            r#""A": {"surface_name":"Wall One"}"#,
            r#""A": {"surface_name":"Wall One", "external_vapor_coefficient_value":-1}, "B": {"surface_name":"WALL ONE", "external_vapor_coefficient_value":2}"#,
        ),
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    compiler.parse_zones(&mut model);
    compiler.parse_surfaces(&mut model);
    compiler.parse_surface_vapor_coefficients(&mut model);

    assert_eq!(model.surface_vapor_coefficients.len(), 1);
    assert_eq!(
        model.surface_vapor_coefficients[0].id,
        SurfaceVaporCoefficientsId(0)
    );
    assert_eq!(
        model.surface_vapor_coefficients[0].name,
        NormalizedName::new("B")
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_name.as_deref() == Some("A")
            && diagnostic.field.as_deref() == Some("external_vapor_coefficient_value")
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| { diagnostic.code != "DuplicateSurfaceVaporCoefficientsSurface" })
    );
    Ok(())
}
