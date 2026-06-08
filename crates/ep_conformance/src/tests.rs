use crate::{
    CASE_MANIFEST_V2_SCHEMA, CaseTier, ComparisonClass, ManifestError, OutputFrequency,
    OutputLevel, OutputRegistry, SourceArtifact, ValidationError, VariableClass, load_case_file,
    load_suite_file, parse_case_str, parse_case_v2_str,
};
use std::path::PathBuf;

#[test]
fn loads_foundation_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest =
        load_case_file(repo_root().join("data/conformance_cases/schedule_constant_001/case.toml"))?;

    assert_eq!(manifest.id, "schedule_constant_001");
    assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
    assert!(manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 1);
    assert_eq!(manifest.outputs[0].key, "ALWAYSON");
    assert_eq!(manifest.outputs[0].variable, "Schedule Value");
    assert_eq!(manifest.outputs[0].source, SourceArtifact::Eso);
    assert_eq!(manifest.outputs[0].level, Some(OutputLevel::Conformance));
    assert_eq!(manifest.tolerances.len(), 1);
    assert_eq!(
        manifest.tolerances[0].variable_class,
        VariableClass::Schedule
    );
    let gate = manifest
        .gate
        .as_ref()
        .ok_or_else(|| std::io::Error::other("schedule conformance should declare a gate"))?;
    assert_eq!(gate.script, "scripts/dev.cmd compare-schedule-conformance");
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_zone_temperature_diagnostic_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/zone_temperature_diagnostic_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "zone_temperature_diagnostic_001");
    assert_eq!(manifest.comparison_class, ComparisonClass::DiagnosticOnly);
    assert!(!manifest.conformance_claim);
    assert!(manifest.tolerances.is_empty());
    assert_eq!(manifest.outputs.len(), 1);
    assert_eq!(manifest.outputs[0].key, "ZONE ONE");
    assert_eq!(manifest.outputs[0].variable, "Zone Mean Air Temperature");
    assert_eq!(manifest.outputs[0].frequency, OutputFrequency::Hourly);
    assert_eq!(manifest.outputs[0].class, VariableClass::ZoneState);
    let report = manifest.report.as_ref().ok_or_else(|| {
        std::io::Error::other("zone diagnostic case should declare diagnostic report path")
    })?;
    assert!(report.path.ends_with("compare-report.md"));
    let gate = manifest.gate.as_ref().ok_or_else(|| {
        std::io::Error::other("zone diagnostic case should declare diagnostic gate")
    })?;
    assert!(!gate.blocking);

    Ok(())
}

#[test]
fn loads_heat_balance_nomass_conformance_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/heat_balance_nomass_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "heat_balance_nomass_001");
    assert_eq!(manifest.milestone, "v0.8-heat-balance");
    assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
    assert!(manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 1);
    assert_eq!(manifest.outputs[0].key, "ZONE ONE");
    assert_eq!(manifest.outputs[0].variable, "Zone Mean Air Temperature");
    assert_eq!(manifest.outputs[0].frequency, OutputFrequency::Hourly);
    assert_eq!(manifest.outputs[0].class, VariableClass::ZoneState);
    assert_eq!(manifest.outputs[0].source, SourceArtifact::Eso);
    assert_eq!(manifest.tolerances.len(), 1);
    assert_eq!(
        manifest.tolerances[0].variable_class,
        VariableClass::ZoneState
    );
    assert_eq!(manifest.tolerances[0].max_abs, Some(0.000001));
    assert_eq!(manifest.tolerances[0].max_rmse, Some(0.000001));
    let gate = manifest
        .gate
        .as_ref()
        .ok_or_else(|| std::io::Error::other("heat-balance conformance should declare a gate"))?;
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_surface_temperature_nomass_conformance_case_fixture()
-> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/surface_temperature_nomass_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "surface_temperature_nomass_001");
    assert_eq!(manifest.milestone, "v0.9-surface-temperature");
    assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
    assert!(manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 8);
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "FLOOR"
            && output.variable == "Surface Inside Face Temperature"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "FLOOR"
            && output.variable == "Surface Outside Face Temperature"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "FLOOR"
            && output.variable == "Surface Inside Face Conduction Heat Transfer Rate"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE"
            && output.variable == "Zone Opaque Surface Inside Faces Conduction Rate"
            && output.class == VariableClass::SurfaceState
    }));
    assert_eq!(manifest.tolerances.len(), 2);
    assert!(manifest.tolerances.iter().any(|tolerance| {
        tolerance.variable_class == VariableClass::ZoneState
            && tolerance.max_abs == Some(0.000001)
            && tolerance.max_rmse == Some(0.000001)
    }));
    assert!(manifest.tolerances.iter().any(|tolerance| {
        tolerance.variable_class == VariableClass::SurfaceState
            && tolerance.max_abs == Some(0.000001)
            && tolerance.max_rmse == Some(0.000001)
    }));
    let gate = manifest.gate.as_ref().ok_or_else(|| {
        std::io::Error::other("surface-temperature conformance should declare a gate")
    })?;
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_ideal_loads_thermostat_smoke_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/ideal_loads_thermostat_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "ideal_loads_thermostat_001");
    assert_eq!(manifest.milestone, "v0.10-ideal-loads-thermostat");
    assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
    assert!(!manifest.conformance_claim);
    assert!(manifest.tolerances.is_empty());
    assert_eq!(manifest.outputs.len(), 4);
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE"
            && output.variable == "Zone Thermostat Heating Setpoint Temperature"
            && output.class == VariableClass::ZoneState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE IDEAL LOADS"
            && output.variable == "Zone Ideal Loads Zone Total Heating Rate"
            && output.class == VariableClass::HvacState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE IDEAL LOADS"
            && output.variable == "Zone Ideal Loads Zone Total Cooling Rate"
            && output.class == VariableClass::HvacState
    }));
    let report = manifest.report.as_ref().ok_or_else(|| {
        std::io::Error::other("IdealLoads thermostat smoke should declare a report path")
    })?;
    assert!(report.path.ends_with("compare-report.md"));
    let gate = manifest.gate.as_ref().ok_or_else(|| {
        std::io::Error::other("IdealLoads thermostat smoke should declare a gate")
    })?;
    assert_eq!(gate.script, "scripts/dev.cmd ideal-loads-thermostat-smoke");
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_air_side_node_diagnostic_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/air_side_node_diagnostic_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "air_side_node_diagnostic_001");
    assert_eq!(manifest.milestone, "v0.11-air-side-node-diagnostic");
    assert_eq!(manifest.comparison_class, ComparisonClass::DiagnosticOnly);
    assert!(!manifest.conformance_claim);
    assert!(manifest.tolerances.is_empty());
    assert_eq!(manifest.outputs.len(), 13);
    assert_eq!(
        manifest
            .outputs
            .iter()
            .filter(|output| output.class == VariableClass::NodeState)
            .count(),
        9
    );
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE INLET"
            && output.variable == "System Node Temperature"
            && output.class == VariableClass::NodeState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE AIR NODE"
            && output.variable == "System Node Mass Flow Rate"
            && output.class == VariableClass::NodeState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE RETURN"
            && output.variable == "System Node Humidity Ratio"
            && output.class == VariableClass::NodeState
    }));
    let gate = manifest
        .gate
        .as_ref()
        .ok_or_else(|| std::io::Error::other("air-side node diagnostic should declare a gate"))?;
    assert_eq!(
        gate.script,
        "scripts/dev.cmd air-side-node-diagnostic-smoke"
    );
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_plant_loop_diagnostic_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/plant_loop_diagnostic_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "plant_loop_diagnostic_001");
    assert_eq!(manifest.milestone, "v0.15-plant-loop-diagnostic");
    assert_eq!(manifest.comparison_class, ComparisonClass::DiagnosticOnly);
    assert!(!manifest.conformance_claim);
    assert!(manifest.tolerances.is_empty());
    assert_eq!(manifest.outputs.len(), 8);
    assert_eq!(
        manifest
            .outputs
            .iter()
            .filter(|output| output.class == VariableClass::PlantState)
            .count(),
        5
    );
    assert_eq!(
        manifest
            .outputs
            .iter()
            .filter(|output| output.class == VariableClass::PlantEquipment)
            .count(),
        3
    );
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "MAIN LOOP"
            && output.variable == "Plant Supply Side Heating Demand Rate"
            && output.class == VariableClass::PlantState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "PUMP"
            && output.variable == "Pump Electricity Rate"
            && output.class == VariableClass::PlantEquipment
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "LOAD PROFILE 1"
            && output.variable == "Plant Load Profile Heat Transfer Rate"
            && output.class == VariableClass::PlantEquipment
    }));
    let gate = manifest
        .gate
        .as_ref()
        .ok_or_else(|| std::io::Error::other("plant loop diagnostic should declare a gate"))?;
    assert_eq!(gate.script, "scripts/dev.cmd plant-loop-diagnostic-smoke");
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_weather_fields_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest =
        load_case_file(repo_root().join("data/conformance_cases/weather_fields_001/case.toml"))?;

    assert_eq!(manifest.id, "weather_fields_001");
    assert_eq!(
        manifest.milestone,
        "v0.22-time-weather-schedule-conformance"
    );
    assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
    assert!(manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 6);
    assert!(manifest.outputs.iter().all(|output| {
        output.key == "Environment"
            && output.frequency == OutputFrequency::Hourly
            && output.class == VariableClass::Weather
            && output.source == SourceArtifact::Eso
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.variable == "Site Outdoor Air Drybulb Temperature"
            && output.level == Some(OutputLevel::Conformance)
    }));
    assert!(
        manifest
            .outputs
            .iter()
            .any(|output| output.variable == "Site Wind Direction"
                && output.level == Some(OutputLevel::Diagnostic))
    );
    assert_eq!(manifest.tolerances.len(), 1);
    assert_eq!(
        manifest.tolerances[0].variable_class,
        VariableClass::Weather
    );
    let gate = manifest
        .gate
        .as_ref()
        .ok_or_else(|| std::io::Error::other("weather conformance should declare a gate"))?;
    assert_eq!(gate.script, "scripts/dev.cmd compare-weather-conformance");
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_surface_geometry_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest =
        load_case_file(repo_root().join("data/conformance_cases/surface_geometry_001/case.toml"))?;

    assert_eq!(manifest.id, "surface_geometry_001");
    assert_eq!(manifest.milestone, "v0.5-geometry-internal-variables");
    assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
    assert!(!manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 5);
    assert!(manifest.outputs.iter().all(|output| {
        output.key == "*"
            && output.frequency == OutputFrequency::Static
            && output.class == VariableClass::SurfaceState
            && output.source == SourceArtifact::Eio
    }));
    assert!(
        manifest
            .outputs
            .iter()
            .any(|output| { output.variable == "HeatTransfer Surface Azimuth" })
    );

    Ok(())
}

#[test]
fn loads_construction_materials_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/construction_materials_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "construction_materials_001");
    assert_eq!(manifest.milestone, "v0.5-geometry-internal-variables");
    assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
    assert!(!manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 7);
    assert!(manifest.outputs.iter().all(|output| {
        output.key == "*"
            && output.frequency == OutputFrequency::Static
            && output.class == VariableClass::ConstructionMaterial
            && output.source == SourceArtifact::Eio
    }));
    assert!(
        manifest
            .outputs
            .iter()
            .any(|output| { output.variable == "Material CTF Summary Thermal Resistance" })
    );

    Ok(())
}

#[test]
fn loads_internal_gains_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest =
        load_case_file(repo_root().join("data/conformance_cases/internal_gains_001/case.toml"))?;

    assert_eq!(manifest.id, "internal_gains_001");
    assert_eq!(manifest.milestone, "v0.26-internal-convective-gains");
    assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
    assert!(manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 8);
    assert!(manifest.outputs.iter().all(|output| {
        output.class == VariableClass::InternalGain
            && (output.source == SourceArtifact::Eio || output.source == SourceArtifact::Eso)
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.variable == "Zone Total Internal Convective Heating Rate"
            && output.frequency == OutputFrequency::Hourly
            && output.source == SourceArtifact::Eso
            && output.level == Some(OutputLevel::Conformance)
    }));
    assert_eq!(manifest.tolerances.len(), 1);
    assert_eq!(
        manifest.tolerances[0].variable_class,
        VariableClass::InternalGain
    );
    let gate = manifest
        .gate
        .as_ref()
        .ok_or_else(|| std::io::Error::other("internal-gains conformance should declare a gate"))?;
    assert_eq!(
        gate.script,
        "scripts/dev.cmd compare-internal-convective-gain-conformance"
    );
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_official_dynamic_heat_balance_diagnostic_case_fixture()
-> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_case_file(repo_root().join(
        "data/conformance_cases/official_1zone_uncontrolled_dynamic_diagnostic_001/case.toml",
    ))?;

    assert_eq!(
        manifest.id,
        "official_1zone_uncontrolled_dynamic_diagnostic_001"
    );
    assert_eq!(manifest.milestone, "v0.33-official-dynamic-diagnostic");
    assert_eq!(manifest.comparison_class, ComparisonClass::DiagnosticOnly);
    assert!(!manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 24);
    assert!(manifest.outputs.iter().all(|output| {
        output.frequency == OutputFrequency::Hourly
            && output.source == SourceArtifact::Eso
            && output.level == Some(OutputLevel::Diagnostic)
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE"
            && output.variable == "Zone Mean Air Temperature"
            && output.class == VariableClass::ZoneState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZN001:ROOF001"
            && output.variable == "Surface Inside Face Temperature"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZN001:WALL001"
            && output.variable == "Surface Inside Face Conduction Heat Transfer Rate"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZN001:FLR001"
            && output.variable == "Surface Inside Face Conduction Heat Transfer Rate"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZN001:ROOF001"
            && output.variable == "Surface Outside Face Temperature"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZN001:ROOF001"
            && output.variable == "Surface Outside Face Incident Solar Radiation Rate per Area"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZN001:ROOF001"
            && output.variable == "Surface Inside Face Conduction Heat Transfer Rate"
            && output.class == VariableClass::SurfaceState
    }));
    assert!(manifest.outputs.iter().any(|output| {
        output.key == "ZONE ONE"
            && output.variable == "Zone Opaque Surface Inside Faces Conduction Rate"
            && output.class == VariableClass::SurfaceState
    }));
    assert_eq!(manifest.tolerances.len(), 2);
    let gate = manifest.gate.as_ref().ok_or_else(|| {
        std::io::Error::other("official dynamic diagnostic should declare a gate")
    })?;
    assert_eq!(
        gate.script,
        "scripts/dev.cmd official-dynamic-heat-balance-diagnostic"
    );
    assert!(!gate.blocking);

    Ok(())
}

#[test]
fn loads_official_static_model_conformance_case_fixture() -> Result<(), Box<dyn std::error::Error>>
{
    let manifest = load_case_file(
        repo_root().join("data/conformance_cases/official_1zone_static_model_001/case.toml"),
    )?;

    assert_eq!(manifest.id, "official_1zone_static_model_001");
    assert_eq!(manifest.milestone, "v0.23-static-model-evidence");
    assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
    assert!(manifest.conformance_claim);
    assert_eq!(manifest.outputs.len(), 19);
    assert!(manifest.outputs.iter().all(|output| {
        output.key == "*"
            && output.frequency == OutputFrequency::Static
            && output.source == SourceArtifact::Eio
            && output.level == Some(OutputLevel::Conformance)
    }));
    assert_eq!(
        manifest
            .outputs
            .iter()
            .filter(|output| output.class == VariableClass::SurfaceState)
            .count(),
        5
    );
    assert_eq!(
        manifest
            .outputs
            .iter()
            .filter(|output| output.class == VariableClass::ConstructionMaterial)
            .count(),
        7
    );
    assert_eq!(
        manifest
            .outputs
            .iter()
            .filter(|output| output.class == VariableClass::InternalGain)
            .count(),
        7
    );
    assert_eq!(manifest.tolerances.len(), 3);
    assert!(manifest.tolerances.iter().any(|tolerance| {
        tolerance.variable_class == VariableClass::SurfaceState && tolerance.max_abs == Some(0.01)
    }));
    assert!(manifest.tolerances.iter().any(|tolerance| {
        tolerance.variable_class == VariableClass::ConstructionMaterial
            && tolerance.max_abs == Some(0.001)
    }));
    assert!(manifest.tolerances.iter().any(|tolerance| {
        tolerance.variable_class == VariableClass::InternalGain && tolerance.max_abs == Some(0.02)
    }));
    let gate = manifest
        .gate
        .as_ref()
        .ok_or_else(|| std::io::Error::other("static model conformance should declare a gate"))?;
    assert_eq!(
        gate.script,
        "scripts/dev.cmd compare-static-model-conformance"
    );
    assert!(gate.blocking);

    Ok(())
}

#[test]
fn loads_foundation_suite_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_suite_file(repo_root().join("data/conformance_suites/foundation.toml"))?;

    assert_eq!(manifest.id, "foundation");
    assert_eq!(manifest.oracle_version, "26.1.0");
    assert_eq!(manifest.cases.len(), 7);
    assert!(
        manifest
            .cases
            .iter()
            .any(|case| case.ends_with("data/conformance_cases/weather_fields_001/case.toml"))
    );
    assert!(
        manifest.cases.iter().any(|case| {
            case.ends_with("data/conformance_cases/surface_geometry_001/case.toml")
        })
    );
    assert!(manifest.cases.iter().any(|case| {
        case.ends_with("data/conformance_cases/construction_materials_001/case.toml")
    }));
    assert!(manifest.cases.iter().any(|case| {
        case.ends_with("data/conformance_cases/official_1zone_static_model_001/case.toml")
    }));
    assert!(
        manifest
            .cases
            .iter()
            .any(|case| { case.ends_with("data/conformance_cases/internal_gains_001/case.toml") })
    );
    assert!(manifest.cases.iter().any(|case| {
        case.ends_with("data/conformance_cases/zone_temperature_diagnostic_001/case.toml")
    }));

    Ok(())
}

#[test]
fn rejects_diagnostic_case_with_true_conformance_claim() {
    let result = parse_case_str(
        r#"
id = "bad_diagnostic_claim"
title = "Bad diagnostic claim"
milestone = "P1"
purpose = "Prove validation blocks false claims."
comparison_class = "diagnostic-only"
conformance_claim = true
oracle_version = "26.1.0"

[input]
idf = "bad.idf"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"
"#,
    );

    assert!(matches!(
        result,
        Err(ManifestError::Validation(
            ValidationError::InvalidConformanceClaim { .. }
        ))
    ));
}

#[test]
fn accepts_conformance_claim_with_full_evidence_contract() -> Result<(), Box<dyn std::error::Error>>
{
    let manifest = parse_case_str(
        r#"
id = "schedule_constant_claim"
title = "Schedule constant claim"
milestone = "P1"
purpose = "Exercise the full conformance evidence contract."
comparison_class = "conformance"
conformance_claim = true
oracle_version = "26.1.0"

[input]
idf = "schedule_constant.idf"
weather = "weather.epw"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"

[[tolerances]]
variable_class = "schedule"
max_abs = 0.0

[report]
format = "markdown"
path = ".runtime/conformance/schedule_constant_claim/compare-report.md"

[gate]
script = "scripts/dev.cmd conformance-schema-smoke"
blocking = true
"#,
    )?;

    assert!(manifest.conformance_claim);
    assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);

    Ok(())
}

#[test]
fn accepts_case_manifest_v2_contract() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = parse_case_v2_str(
        r#"
id = "schedule_constant_claim"
title = "Schedule constant claim"
milestone = "v0.17-manifest-v2"
purpose = "Exercise the full v2 conformance evidence contract."
comparison_class = "conformance"
conformance_claim = true
oracle_version = "26.1.0"

[manifest_v2]
schema = "rusted-energyplus.case-manifest.v2"
source_kind = "local-fixture"
source_file = "schedule_constant.idf"
tier = "A"

[scope]
domains = ["schedule"]
has_zone = false
has_surface = false
has_fenestration = false
has_air_loop = false
has_plant_loop = false
has_ems = false
has_python_plugin = false
has_daylighting = false

[input]
idf = "schedule_constant.idf"
weather = "weather.epw"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"
domain = "schedule"
level = "conformance"
abs_tol = 0.0

[[tolerances]]
variable_class = "schedule"
max_abs = 0.0

[report]
format = "markdown"
path = ".runtime/conformance/schedule_constant_claim/compare-report.md"

[gate]
script = "scripts/dev.cmd conformance-schema-smoke"
blocking = true
"#,
    )?;

    assert_eq!(
        manifest
            .manifest_v2
            .as_ref()
            .map(|metadata| metadata.schema.as_str()),
        Some(CASE_MANIFEST_V2_SCHEMA)
    );
    assert_eq!(
        manifest.manifest_v2.as_ref().map(|metadata| metadata.tier),
        Some(CaseTier::A)
    );
    assert_eq!(manifest.outputs[0].level, Some(OutputLevel::Conformance));

    Ok(())
}

#[test]
fn rejects_missing_manifest_v2_when_v2_validation_is_requested() {
    let result = parse_case_v2_str(
        r#"
id = "missing_v2"
title = "Missing v2"
milestone = "v0.17"
purpose = "Prove v2 validation requires v2 metadata."
comparison_class = "smoke"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "fixture.idf"
"#,
    );

    assert!(matches!(
        result,
        Err(ManifestError::Validation(
            ValidationError::MissingManifestV2
        ))
    ));
}

#[test]
fn rejects_conformance_level_output_without_claim() {
    let result = parse_case_v2_str(
        r#"
id = "bad_level"
title = "Bad level"
milestone = "v0.17"
purpose = "Prove v2 validation blocks conformance-level output false claims."
comparison_class = "diagnostic-only"
conformance_claim = false
oracle_version = "26.1.0"

[manifest_v2]
schema = "rusted-energyplus.case-manifest.v2"
source_kind = "local-fixture"
source_file = "fixture.idf"
tier = "B"

[scope]
domains = ["schedule"]
has_zone = false
has_surface = false
has_fenestration = false
has_air_loop = false
has_plant_loop = false
has_ems = false
has_python_plugin = false
has_daylighting = false

[input]
idf = "fixture.idf"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"
domain = "schedule"
level = "conformance"
"#,
    );

    assert!(matches!(
        result,
        Err(ManifestError::Validation(
            ValidationError::ConformanceOutputWithoutClaim { .. }
        ))
    ));
}

#[test]
fn builds_output_registry_from_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let manifest =
        load_case_file(repo_root().join("data/conformance_cases/schedule_constant_001/case.toml"))?;
    let registry = OutputRegistry::from_case(&manifest)?;

    assert_eq!(registry.len(), 1);
    assert_eq!(registry.series()[0].identity.key, "ALWAYSON");
    assert_eq!(registry.series()[0].identity.variable, "SCHEDULE VALUE");

    Ok(())
}

#[test]
fn rejects_duplicate_output_requests() {
    let result = parse_case_str(
        r#"
id = "duplicate_outputs"
title = "Duplicate outputs"
milestone = "P1"
purpose = "Prove output registry rejects duplicates."
comparison_class = "smoke"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "duplicate.idf"

[[outputs]]
key = "AlwaysOn"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"

[[outputs]]
key = " ALWAYSON "
variable = "schedule value"
frequency = "hourly"
class = "schedule"
source = "eso"
"#,
    );

    assert!(matches!(
        result,
        Err(ManifestError::Validation(
            ValidationError::DuplicateOutputRequest { .. }
        ))
    ));
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}
