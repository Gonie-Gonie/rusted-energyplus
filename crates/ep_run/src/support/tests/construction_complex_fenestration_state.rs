use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_complex_fenestration_states_are_all_definition_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier":"26.1"}},
            "Zone": {"Zone One": {"volume":100.0}},
            "WindowMaterial:Glazing": {
                "CFS Glass": {
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.003
                }
            },
            "WindowThermalModel:Params": {
                "CFS Thermal Defaults": {}
            },
            "Matrix:TwoDimension": {
                "Basis": {
                    "number_of_rows":1,
                    "number_of_columns":1,
                    "values":[{"value":0.0}]
                },
                "Solar Front": {
                    "number_of_rows":1,
                    "number_of_columns":1,
                    "values":[{"value":0.4}]
                },
                "Solar Back": {
                    "number_of_rows":1,
                    "number_of_columns":1,
                    "values":[{"value":0.2}]
                },
                "Visible Front": {
                    "number_of_rows":1,
                    "number_of_columns":1,
                    "values":[{"value":0.4}]
                },
                "Visible Back": {
                    "number_of_rows":1,
                    "number_of_columns":1,
                    "values":[{"value":0.2}]
                },
                "Abs Front": {
                    "number_of_rows":1,
                    "number_of_columns":1,
                    "values":[{"value":0.1}]
                },
                "Abs Back": {
                    "number_of_rows":1,
                    "number_of_columns":1,
                    "values":[{"value":0.1}]
                }
            },
            "Construction:ComplexFenestrationState": {
                "Unused CFS One": {
                    "basis_type":"LBNLWINDOW",
                    "basis_symmetry_type":"None",
                    "window_thermal_model":"CFS Thermal Defaults",
                    "basis_matrix_name":"Basis",
                    "solar_optical_complex_front_transmittance_matrix_name":"Solar Front",
                    "solar_optical_complex_back_reflectance_matrix_name":"Solar Back",
                    "visible_optical_complex_front_transmittance_matrix_name":"Visible Front",
                    "visible_optical_complex_back_transmittance_matrix_name":"Visible Back",
                    "outside_layer_name":"CFS Glass",
                    "outside_layer_directional_front_absorptance_matrix_name":"Abs Front",
                    "outside_layer_directional_back_absorptance_matrix_name":"Abs Back"
                },
                "Unused CFS Two": {
                    "basis_type":"LBNLWINDOW",
                    "basis_symmetry_type":"None",
                    "window_thermal_model":"CFS Thermal Defaults",
                    "basis_matrix_name":"Basis",
                    "solar_optical_complex_front_transmittance_matrix_name":"Solar Front",
                    "solar_optical_complex_back_reflectance_matrix_name":"Solar Back",
                    "visible_optical_complex_front_transmittance_matrix_name":"Visible Front",
                    "visible_optical_complex_back_transmittance_matrix_name":"Visible Back",
                    "outside_layer_name":"CFS Glass",
                    "outside_layer_directional_front_absorptance_matrix_name":"Abs Front",
                    "outside_layer_directional_back_absorptance_matrix_name":"Abs Back"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed complex fenestration states"))?;
    assert_eq!(
        model
            .constructions
            .iter()
            .filter(|construction| construction.is_complex_fenestration())
            .count(),
        2
    );

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
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "Construction:ComplexFenestrationState"
            && entry.count == 2
            && entry.status == "unsupported"
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.stage == "support"
            && diagnostic.object_type.as_deref() == Some("Construction:ComplexFenestrationState")
            && diagnostic.blocking
    }));
    Ok(())
}
