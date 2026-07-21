use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "ComplexFenestrationProperty:SolarAbsorbedLayers";

#[test]
fn complex_fenestration_solar_absorbed_layers_are_all_definition_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage", "thickness":0.003}
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
                    "outside_layer_name":"Glass",
                    "outside_layer_directional_front_absorptance_matrix_name":"Unit Matrix",
                    "outside_layer_directional_back_absorptance_matrix_name":"Unit Matrix"
                }
            },
            "Schedule:Constant": {
                "Absorbed Solar": {"hourly_value":-42.5}
            },
            "ComplexFenestrationProperty:SolarAbsorbedLayers": {
                "Scheduled Layers": {
                    "fenestration_surface":"Unresolved Window",
                    "construction_name":"CFS Single",
                    "layer_1_solar_radiation_absorbed_schedule_name":"Absorbed Solar"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed absorbed-solar request"))?;
    assert_eq!(model.fenestration_solar_absorbed_requests.len(), 1);

    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );
    assert_eq!(assessment.status, SupportStatus::Unsupported);
    assert_eq!(assessment.run_result_state, RunResultState::RunBlocked);
    assert_eq!(assessment.runtime_class, RuntimeClass::None);
    assert!(assessment.typed_objects.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE && entry.count == 1 && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE && entry.count == 1 && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some(OBJECT_TYPE)
            && diagnostic.blocking
    }));
    Ok(())
}
