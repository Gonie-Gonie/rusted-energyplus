use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialPhaseChangeHysteresisId;
use ep_raw_model::parse_epjson_str;

const PHASE_CHANGE_BODY: &str = r#"
    "latent_heat_during_the_entire_phase_change_process":10000,
    "liquid_state_thermal_conductivity":1.5,
    "liquid_state_density":2200,
    "liquid_state_specific_heat":2000,
    "high_temperature_difference_of_melting_curve":1,
    "peak_melting_temperature":20,
    "low_temperature_difference_of_melting_curve":1,
    "solid_state_thermal_conductivity":1.8,
    "solid_state_density":2300,
    "solid_state_specific_heat":1800,
    "high_temperature_difference_of_freezing_curve":1,
    "peak_freezing_temperature":23,
    "low_temperature_difference_of_freezing_curve":1
"#;

#[test]
fn phase_change_hysteresis_blocks_used_and_unused_targets_before_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&format!(
        r#"{{
            "Version": {{"Version 1": {{"version_identifier": "26.1"}}}},
            "Zone": {{"Zone One": {{"volume": 100}}}},
            "Material": {{
                "Used": {{
                    "roughness":"MediumRough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }}
            }},
            "Material:NoMass": {{
                "Unused": {{"roughness":"MediumRough","thermal_resistance":1.5}}
            }},
            "MaterialProperty:PhaseChangeHysteresis": {{
                "Used": {{{PHASE_CHANGE_BODY}}},
                "Unused": {{{PHASE_CHANGE_BODY}}}
            }},
            "Construction": {{"Wall": {{"outside_layer":"Used"}}}},
            "BuildingSurface:Detailed": {{
                "Wall One": {{
                    "surface_type":"Wall",
                    "construction_name":"Wall",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0}},
                        {{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0}},
                        {{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1}},
                        {{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1}}
                    ]
                }}
            }}
        }}"#
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.material_phase_change_hystereses.len(), 2);
    assert_eq!(
        model.material_phase_change_hystereses[0].id,
        MaterialPhaseChangeHysteresisId(0)
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
    assert!(assessment.typed_objects.iter().any(|entry| {
        entry.object_type == "MaterialProperty:PhaseChangeHysteresis"
            && entry.count == 2
            && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "MaterialProperty:PhaseChangeHysteresis"
            && entry.count == 2
            && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("MaterialProperty:PhaseChangeHysteresis")
    }));
    Ok(())
}
