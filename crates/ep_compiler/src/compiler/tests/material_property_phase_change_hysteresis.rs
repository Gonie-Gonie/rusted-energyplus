use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{MaterialDefinition, MaterialPhaseChangeHysteresisId, NormalizedName, TypedModel};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "MaterialProperty:PhaseChangeHysteresis";
const NUMERIC_FIELDS: [(&str, &str); 13] = [
    (
        "latent_heat_during_the_entire_phase_change_process",
        "10000",
    ),
    ("liquid_state_thermal_conductivity", "1.5"),
    ("liquid_state_density", "2200"),
    ("liquid_state_specific_heat", "2000"),
    ("high_temperature_difference_of_melting_curve", "1"),
    ("peak_melting_temperature", "20"),
    ("low_temperature_difference_of_melting_curve", "2"),
    ("solid_state_thermal_conductivity", "1.8"),
    ("solid_state_density", "2300"),
    ("solid_state_specific_heat", "1800"),
    ("high_temperature_difference_of_freezing_curve", "3"),
    ("peak_freezing_temperature", "23"),
    ("low_temperature_difference_of_freezing_curve", "4"),
];

fn phase_change_body(overrides: &[(&str, &str)], omitted: Option<&str>) -> String {
    NUMERIC_FIELDS
        .iter()
        .filter(|(field, _value)| Some(*field) != omitted)
        .map(|(field, default)| {
            let value = overrides
                .iter()
                .find_map(|(candidate, value)| (*candidate == *field).then_some(*value))
                .unwrap_or(default);
            format!(r#""{field}":{value}"#)
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn one_material_model(name: &str, body: &str) -> String {
    format!(
        r#"{{
            "Material": {{
                "{name}": {{
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }}
            }},
            "{OBJECT_TYPE}": {{"{name}": {{{body}}}}}
        }}"#
    )
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
fn phase_change_hysteresis_materializes_regular_nomass_and_variable_absorptance_coexistence()
-> Result<(), Box<dyn std::error::Error>> {
    let mass_body = phase_change_body(&[], None);
    let no_mass_body = phase_change_body(
        &[
            (
                "latent_heat_during_the_entire_phase_change_process",
                "60000",
            ),
            ("liquid_state_specific_heat", "2400"),
            ("solid_state_specific_heat", "2100"),
            ("peak_melting_temperature", "55"),
            ("peak_freezing_temperature", "43"),
        ],
        None,
    );
    let raw = parse_epjson_str(&format!(
        r#"{{
            "Material": {{
                "PCM Mass": {{
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }}
            }},
            "Material:NoMass": {{
                "PCM NoMass": {{
                    "roughness":"MediumRough",
                    "thermal_resistance":2.0
                }}
            }},
            "MaterialProperty:VariableAbsorptance": {{
                "Absorptance Overlay": {{
                    "reference_material_name":"PCM Mass",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }}
            }},
            "{OBJECT_TYPE}": {{
                "pcm mass": {{{mass_body}}},
                "pcm nomass": {{{no_mass_body}}}
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

    let mass_id = model
        .material_names
        .resolve("PCM MASS")
        .ok_or_else(|| std::io::Error::other("expected mass material"))?;
    let no_mass_id = model
        .material_names
        .resolve("PCM NOMASS")
        .ok_or_else(|| std::io::Error::other("expected no-mass material"))?;
    let mass = model
        .material_phase_change_hystereses
        .iter()
        .find(|attachment| attachment.reference_material == mass_id)
        .ok_or_else(|| std::io::Error::other("expected mass attachment"))?;
    let no_mass = model
        .material_phase_change_hystereses
        .iter()
        .find(|attachment| attachment.reference_material == no_mass_id)
        .ok_or_else(|| std::io::Error::other("expected no-mass attachment"))?;

    assert_eq!(mass.id, MaterialPhaseChangeHysteresisId(0));
    assert_eq!(mass.name, NormalizedName::new("pcm mass"));
    assert_eq!(mass.total_latent_heat_j_per_kg, 10000.0);
    assert_eq!(mass.liquid_state.conductivity_w_per_m_k, 1.5);
    assert_eq!(mass.liquid_state.density_kg_per_m3, 2200.0);
    assert_eq!(mass.liquid_state.specific_heat_j_per_kg_k, 2000.0);
    assert_eq!(mass.melting_curve.high_temperature_difference_c, 1.0);
    assert_eq!(mass.melting_curve.peak_temperature_c, 20.0);
    assert_eq!(mass.melting_curve.low_temperature_difference_c, 2.0);
    assert_eq!(mass.solid_state.conductivity_w_per_m_k, 1.8);
    assert_eq!(mass.solid_state.density_kg_per_m3, 2300.0);
    assert_eq!(mass.solid_state.specific_heat_j_per_kg_k, 1800.0);
    assert_eq!(mass.freezing_curve.high_temperature_difference_c, 3.0);
    assert_eq!(mass.freezing_curve.peak_temperature_c, 23.0);
    assert_eq!(mass.freezing_curve.low_temperature_difference_c, 4.0);
    assert_eq!(mass.transition_specific_heat_j_per_kg_k, 1900.0);
    assert_eq!(mass.initial_specific_heat_j_per_kg_k, 1800.0);

    assert_eq!(no_mass.total_latent_heat_j_per_kg, 60000.0);
    assert_eq!(no_mass.transition_specific_heat_j_per_kg_k, 2250.0);
    assert_eq!(no_mass.initial_specific_heat_j_per_kg_k, 2100.0);
    assert!(matches!(
        model.materials[mass_id.0 as usize].definition,
        MaterialDefinition::Regular(_)
    ));
    assert!(matches!(
        model.materials[no_mass_id.0 as usize].definition,
        MaterialDefinition::NoMass(_)
    ));
    assert_eq!(model.material_variable_absorptances.len(), 1);
    assert_eq!(
        model.material_variable_absorptances[0].reference_material,
        mass_id
    );
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE
            && entry.object_count == 2
            && entry.status == ObjectCoverageStatus::Typed
    }));
    Ok(())
}

#[test]
fn phase_change_hysteresis_requires_every_finite_strictly_positive_number()
-> Result<(), Box<dyn std::error::Error>> {
    for (field, _default) in NUMERIC_FIELDS {
        for invalid in ["0", "-1"] {
            let body = phase_change_body(&[(field, invalid)], None);
            let raw = parse_epjson_str(&one_material_model("PCM", &body))?;
            let result = compile_raw_model(&raw);
            assert!(
                has_error(&result, "InvalidNumericRange", "PCM", Some(field)),
                "field={field}, invalid={invalid}, diagnostics={:?}",
                result.report.diagnostics
            );
        }

        let body = phase_change_body(&[], Some(field));
        let raw = parse_epjson_str(&one_material_model("PCM", &body))?;
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "MissingRequiredField", "PCM", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );

        let body = phase_change_body(&[(field, r#""not numeric""#)], None);
        let raw = parse_epjson_str(&one_material_model("PCM", &body))?;
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "InvalidFieldType", "PCM", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn phase_change_hysteresis_accepts_positive_inputs_without_cross_field_constraints()
-> Result<(), Box<dyn std::error::Error>> {
    let body = phase_change_body(
        &[
            ("liquid_state_thermal_conductivity", "8"),
            ("solid_state_thermal_conductivity", "1"),
            ("liquid_state_density", "3000"),
            ("solid_state_density", "100"),
            ("peak_melting_temperature", "5"),
            ("peak_freezing_temperature", "90"),
        ],
        None,
    );
    let raw = parse_epjson_str(&one_material_model("PCM", &body))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        result
            .model
            .as_ref()
            .map(|model| model.material_phase_change_hystereses.len()),
        Some(1)
    );
    Ok(())
}

#[test]
fn phase_change_hysteresis_accepts_only_public_regular_group_targets()
-> Result<(), Box<dyn std::error::Error>> {
    let body = phase_change_body(&[], None);
    let raw = parse_epjson_str(&format!(
        r#"{{
            "Material:AirGap": {{"Gap": {{"thermal_resistance":0.2}}}},
            "Material:InfraredTransparent": {{"IRT": {{}}}},
            "WindowMaterial:Gas": {{"Gas": {{"gas_type":"Air","thickness":0.012}}}},
            "{OBJECT_TYPE}": {{
                "Gap": {{{body}}},
                "IRT": {{{body}}},
                "Gas": {{{body}}},
                "Missing": {{{body}}}
            }}
        }}"#
    ))?;
    let result = compile_raw_model(&raw);
    for target in ["Gap", "IRT", "Gas"] {
        assert!(has_error(
            &result,
            "InvalidPhaseChangeHysteresisMaterialType",
            target,
            Some("name")
        ));
    }
    assert!(has_error(
        &result,
        "MissingReference",
        "Missing",
        Some("name")
    ));
    Ok(())
}

#[test]
fn phase_change_hysteresis_duplicate_target_fails_and_invalid_first_does_not_reserve()
-> Result<(), Box<dyn std::error::Error>> {
    let valid_body = phase_change_body(&[], None);
    let duplicate = parse_epjson_str(&format!(
        r#"{{
            "Material": {{"M": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}}}},
            "{OBJECT_TYPE}": {{"M": {{{valid_body}}}, "m": {{{valid_body}}}}}
        }}"#
    ))?;
    let result = compile_raw_model(&duplicate);
    assert!(has_error(
        &result,
        "DuplicatePhaseChangeHysteresisMaterial",
        "m",
        Some("name")
    ));

    let invalid_body = phase_change_body(
        &[("latent_heat_during_the_entire_phase_change_process", "0")],
        None,
    );
    let raw = parse_epjson_str(&format!(
        r#"{{
            "Material": {{"M": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}}}},
            "{OBJECT_TYPE}": {{"M": {{{invalid_body}}}, "m": {{{valid_body}}}}}
        }}"#
    ))?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_phase_change_hystereses(&mut model);
    assert_eq!(model.material_phase_change_hystereses.len(), 1);
    assert_eq!(
        model.material_phase_change_hystereses[0].id,
        MaterialPhaseChangeHysteresisId(0)
    );
    assert_eq!(
        model.material_phase_change_hystereses[0].name,
        NormalizedName::new("m")
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange" && diagnostic.object_name.as_deref() == Some("M")
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| { diagnostic.code != "DuplicatePhaseChangeHysteresisMaterial" })
    );
    Ok(())
}

#[test]
fn phase_change_hysteresis_rejects_blank_material_key() -> Result<(), Box<dyn std::error::Error>> {
    let body = phase_change_body(&[], None);
    let raw = parse_epjson_str(&format!(
        r#"{{
            "Material": {{"M": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}}}},
            "{OBJECT_TYPE}": {{"   ": {{{body}}}}}
        }}"#
    ))?;
    let result = compile_raw_model(&raw);
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "   ",
        Some("name")
    ));
    Ok(())
}
