use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{FenestrationSolarAbsorbedRequestId, NormalizedName, TypedModel};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "ComplexFenestrationProperty:SolarAbsorbedLayers";
const SURFACE_FIELD: &str = "fenestration_surface";
const CONSTRUCTION_FIELD: &str = "construction_name";

fn schedule_field(layer: usize) -> String {
    format!("layer_{layer}_solar_radiation_absorbed_schedule_name")
}

fn model_with_requests(requests: &str) -> String {
    r#"{
        "Material:NoMass": {
            "Opaque Layer": {"roughness":"Rough", "thermal_resistance":1.0}
        },
        "WindowMaterial:Glazing": {
            "Glass One": {"optical_data_type":"SpectralAverage", "thickness":0.003},
            "Glass Two": {"optical_data_type":"SpectralAverage", "thickness":0.004},
            "Glass Three": {"optical_data_type":"SpectralAverage", "thickness":0.005}
        },
        "WindowMaterial:Gas": {
            "Gap Gas": {"gas_type":"Air", "thickness":0.012}
        },
        "WindowMaterial:Gap": {
            "Complex Gap": {"thickness":0.012, "gas_or_gas_mixture_":"Gap Gas"}
        },
        "Construction": {
            "Ordinary Construction": {"outside_layer":"Opaque Layer"}
        },
        "WindowThermalModel:Params": {"Thermal": {}},
        "Matrix:TwoDimension": {
            "Unit Matrix": {
                "number_of_rows":1,
                "number_of_columns":1,
                "values":[{"value":0.0}]
            }
        },
        "Construction:ComplexFenestrationState": {
            "CFS Single": {
                "window_thermal_model":"Thermal",
                "basis_matrix_name":"Unit Matrix",
                "solar_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "solar_optical_complex_back_reflectance_matrix_name":"Unit Matrix",
                "visible_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "visible_optical_complex_back_transmittance_matrix_name":"Unit Matrix",
                "outside_layer_name":"Glass One",
                "outside_layer_directional_front_absorptance_matrix_name":"Unit Matrix",
                "outside_layer_directional_back_absorptance_matrix_name":"Unit Matrix"
            },
            "CFS Double": {
                "window_thermal_model":"Thermal",
                "basis_matrix_name":"Unit Matrix",
                "solar_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "solar_optical_complex_back_reflectance_matrix_name":"Unit Matrix",
                "visible_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "visible_optical_complex_back_transmittance_matrix_name":"Unit Matrix",
                "outside_layer_name":"Glass One",
                "outside_layer_directional_front_absorptance_matrix_name":"Unit Matrix",
                "outside_layer_directional_back_absorptance_matrix_name":"Unit Matrix",
                "gap_1_name":"Complex Gap",
                "layer_2_name":"Glass Two",
                "layer_2_directional_front_absorptance_matrix_name":"Unit Matrix",
                "layer_2_directional_back_absorptance_matrix_name":"Unit Matrix"
            },
            "CFS Triple": {
                "window_thermal_model":"Thermal",
                "basis_matrix_name":"Unit Matrix",
                "solar_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "solar_optical_complex_back_reflectance_matrix_name":"Unit Matrix",
                "visible_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "visible_optical_complex_back_transmittance_matrix_name":"Unit Matrix",
                "outside_layer_name":"Glass One",
                "outside_layer_directional_front_absorptance_matrix_name":"Unit Matrix",
                "outside_layer_directional_back_absorptance_matrix_name":"Unit Matrix",
                "gap_1_name":"Complex Gap",
                "layer_2_name":"Glass Two",
                "layer_2_directional_front_absorptance_matrix_name":"Unit Matrix",
                "layer_2_directional_back_absorptance_matrix_name":"Unit Matrix",
                "gap_2_name":"Complex Gap",
                "layer_3_name":"Glass Three",
                "layer_3_directional_front_absorptance_matrix_name":"Unit Matrix",
                "layer_3_directional_back_absorptance_matrix_name":"Unit Matrix"
            }
        },
        "Schedule:Constant": {
            "Layer Negative": {"hourly_value":-321.5},
            "Layer Huge": {"hourly_value":1000000000.0},
            "Layer Three": {"hourly_value":17.25}
        },
        "ComplexFenestrationProperty:SolarAbsorbedLayers": {
            __REQUESTS__
        }
    }"#
    .replace("__REQUESTS__", requests)
}

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: &str) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == Some(field)
    })
}

fn parse_request_prerequisites(compiler: &mut Compiler<'_>, model: &mut TypedModel) {
    compiler.parse_materials(model);
    compiler.parse_constructions(model);
    compiler.parse_schedule_type_limits(model);
    compiler.parse_schedules(model);
    compiler.parse_complex_fenestration_states(model);
}

#[test]
fn solar_absorbed_layers_resolves_complex_construction_and_ordered_layer_schedules()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&model_with_requests(
        r#"
            "SHARED NAME": {
                "fenestration_surface":" Unresolved Window A ",
                "construction_name":"cfs single",
                "layer_1_solar_radiation_absorbed_schedule_name":"layer negative"
            },
            "shared name": {
                "fenestration_surface":"UNRESOLVED WINDOW A",
                "construction_name":"CFS DOUBLE",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Huge",
                "layer_2_solar_radiation_absorbed_schedule_name":"Layer Negative"
            },
            "Third Request": {
                "fenestration_surface":"Unresolved Window B",
                "construction_name":"CFS Single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Three"
            }
        "#,
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed absorbed-solar requests"))?;

    assert_eq!(model.fenestration_solar_absorbed_requests.len(), 3);
    assert_eq!(
        model
            .fenestration_solar_absorbed_requests
            .iter()
            .map(|request| request.id)
            .collect::<Vec<_>>(),
        vec![
            FenestrationSolarAbsorbedRequestId(0),
            FenestrationSolarAbsorbedRequestId(1),
            FenestrationSolarAbsorbedRequestId(2),
        ]
    );
    let shared = model
        .fenestration_solar_absorbed_requests
        .iter()
        .filter(|request| request.name == NormalizedName::new("shared name"))
        .collect::<Vec<_>>();
    assert_eq!(shared.len(), 2, "case-colliding semantic names are data");
    assert!(shared.iter().all(|request| {
        request.fenestration_surface_name == NormalizedName::new("Unresolved Window A")
    }));
    assert_ne!(shared[0].construction, shared[1].construction);

    let double = shared
        .iter()
        .find(|request| request.layer_schedules.len() == 2)
        .expect("two-solid request");
    assert_eq!(
        double.layer_schedules,
        vec![
            model
                .schedule_names
                .resolve("Layer Huge")
                .expect("schedule"),
            model
                .schedule_names
                .resolve("Layer Negative")
                .expect("schedule"),
        ]
    );
    let single_construction = model
        .construction_names
        .resolve("CFS Single")
        .expect("construction");
    assert_eq!(
        model
            .fenestration_solar_absorbed_requests
            .iter()
            .filter(|request| request.construction == single_construction)
            .count(),
        2,
        "one construction may serve distinct unresolved surfaces"
    );

    let mut without_requests = model.clone();
    without_requests
        .fenestration_solar_absorbed_requests
        .clear();
    assert_eq!(model.object_count(), without_requests.object_count() + 3);
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE
            && entry.object_count == 3
            && entry.status == ObjectCoverageStatus::Typed
    }));
    Ok(())
}

#[test]
fn semantic_name_and_required_fields_fail_closed() -> Result<(), Box<dyn std::error::Error>> {
    let layer_one = schedule_field(1);
    let cases = [
        (
            r#""": {"fenestration_surface":"Window", "construction_name":"CFS Single", "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative"}"#,
            "MissingRequiredField",
            "",
            "name",
        ),
        (
            r#""Missing Surface": {"construction_name":"CFS Single", "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative"}"#,
            "MissingRequiredField",
            "Missing Surface",
            SURFACE_FIELD,
        ),
        (
            r#""Blank Surface": {"fenestration_surface":" ", "construction_name":"CFS Single", "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative"}"#,
            "MissingRequiredField",
            "Blank Surface",
            SURFACE_FIELD,
        ),
        (
            r#""Blank Construction": {"fenestration_surface":"Window", "construction_name":"", "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative"}"#,
            "MissingRequiredField",
            "Blank Construction",
            CONSTRUCTION_FIELD,
        ),
        (
            r#""Missing Layer": {"fenestration_surface":"Window", "construction_name":"CFS Single"}"#,
            "MissingRequiredField",
            "Missing Layer",
            layer_one.as_str(),
        ),
        (
            r#""Blank Layer": {"fenestration_surface":"Window", "construction_name":"CFS Single", "layer_1_solar_radiation_absorbed_schedule_name":" "}"#,
            "MissingRequiredField",
            "Blank Layer",
            layer_one.as_str(),
        ),
        (
            r#""Wrong Type Layer": {"fenestration_surface":"Window", "construction_name":"CFS Single", "layer_1_solar_radiation_absorbed_schedule_name":1}"#,
            "InvalidFieldType",
            "Wrong Type Layer",
            layer_one.as_str(),
        ),
    ];

    for (request, code, object_name, field) in cases {
        let result = compile_raw_model(&parse_epjson_str(&model_with_requests(request))?);
        assert!(result.model.is_none());
        assert!(
            has_error(&result, code, object_name, field),
            "case={object_name}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn schedules_must_be_contiguous_and_exactly_match_solid_layers()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            r#""Missing Middle": {
                "fenestration_surface":"Window", "construction_name":"CFS Triple",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative",
                "layer_3_solar_radiation_absorbed_schedule_name":"Layer Three"
            }"#,
            "MissingRequiredField",
            schedule_field(2),
        ),
        (
            r#""Nonblank Extra": {
                "fenestration_surface":"Window", "construction_name":"CFS Single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative",
                "layer_2_solar_radiation_absorbed_schedule_name":"Layer Huge"
            }"#,
            "UnexpectedFenestrationSolarAbsorbedLayerSchedule",
            schedule_field(2),
        ),
        (
            r#""Blank Extra": {
                "fenestration_surface":"Window", "construction_name":"CFS Single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative",
                "layer_2_solar_radiation_absorbed_schedule_name":""
            }"#,
            "UnexpectedFenestrationSolarAbsorbedLayerSchedule",
            schedule_field(2),
        ),
        (
            r#""Wrong Type Extra": {
                "fenestration_surface":"Window", "construction_name":"CFS Single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative",
                "layer_2_solar_radiation_absorbed_schedule_name":2
            }"#,
            "UnexpectedFenestrationSolarAbsorbedLayerSchedule",
            schedule_field(2),
        ),
    ];

    for (request, code, field) in cases {
        let raw = parse_epjson_str(&model_with_requests(request))?;
        let result = compile_raw_model(&raw);
        let object_name = if request.contains("Missing Middle") {
            "Missing Middle"
        } else if request.contains("Nonblank Extra") {
            "Nonblank Extra"
        } else if request.contains("Blank Extra") {
            "Blank Extra"
        } else {
            "Wrong Type Extra"
        };
        assert!(result.model.is_none());
        assert!(
            has_error(&result, code, object_name, &field),
            "case={object_name}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn unknown_schedule_reports_the_actual_failing_layer_field()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(&model_with_requests(
        r#""Unknown Second Schedule": {
            "fenestration_surface":"Window", "construction_name":"CFS Double",
            "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative",
            "layer_2_solar_radiation_absorbed_schedule_name":"Missing Schedule"
        }"#,
    ))?);
    assert!(result.model.is_none());
    assert!(has_error(
        &result,
        "MissingReference",
        "Unknown Second Schedule",
        &schedule_field(2)
    ));
    Ok(())
}

#[test]
fn missing_and_non_complex_constructions_fail_closed() -> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            "Ordinary Construction",
            "InvalidFenestrationSolarAbsorbedConstruction",
        ),
        ("Missing Construction", "MissingReference"),
    ];
    for (construction, code) in cases {
        let request = format!(
            r#""{construction}": {{
                "fenestration_surface":"Window",
                "construction_name":"{construction}",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative"
            }}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&model_with_requests(&request))?);
        assert!(result.model.is_none());
        assert!(has_error(&result, code, construction, CONSTRUCTION_FIELD));
    }
    Ok(())
}

#[test]
fn duplicate_normalized_surface_construction_pair_fails_close_the_whole_pass()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&model_with_requests(
        r#"
            "First": {
                "fenestration_surface":"Window A", "construction_name":"CFS Single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative"
            },
            "Second": {
                "fenestration_surface":" window a ", "construction_name":"cfs single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Huge"
            }
        "#,
    ))?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    parse_request_prerequisites(&mut compiler, &mut model);
    compiler.parse_fenestration_solar_absorbed_requests(&mut model);

    assert!(model.fenestration_solar_absorbed_requests.is_empty());
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateFenestrationSolarAbsorbedPair"
            && diagnostic.object_name.as_deref() == Some("Second")
            && diagnostic.field.as_deref() == Some(CONSTRUCTION_FIELD)
    }));
    Ok(())
}

#[test]
fn invalid_request_does_not_reserve_its_normalized_pair() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(&model_with_requests(
        r#"
            "A Invalid": {
                "fenestration_surface":"Window A", "construction_name":"CFS Single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Missing Schedule"
            },
            "B Valid": {
                "fenestration_surface":"window a", "construction_name":"cfs single",
                "layer_1_solar_radiation_absorbed_schedule_name":"Layer Negative"
            }
        "#,
    ))?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    parse_request_prerequisites(&mut compiler, &mut model);
    compiler.parse_fenestration_solar_absorbed_requests(&mut model);

    assert!(model.fenestration_solar_absorbed_requests.is_empty());
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_name.as_deref() == Some("A Invalid")
            && diagnostic.field.as_deref() == Some(schedule_field(1).as_str())
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "DuplicateFenestrationSolarAbsorbedPair")
    );
    Ok(())
}
