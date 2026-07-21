use super::super::{CompileResult, DiagnosticSeverity, compile_raw_model};
use ep_model::SpaceOrigin;
use ep_raw_model::parse_epjson_str;

const WARNING_CODE: &str = "IncompleteScheduledSurfaceGainsTypedSubset";

fn fixture(
    zones: &str,
    spaces: &str,
    surfaces: &str,
    incidents: &str,
    extra_objects: &str,
) -> String {
    format!(
        r#"{{
            "Material:NoMass": {{"Opaque Layer": {{"roughness":"Rough","thermal_resistance":1}}}},
            "Construction": {{
                "Current Construction": {{"outside_layer":"Opaque Layer"}},
                "Alternate Construction": {{"outside_layer":"Opaque Layer"}}
            }},
            "Schedule:Constant": {{"Solar Schedule": {{"hourly_value":100}}}},
            "Zone": {{{zones}}},
            "Space": {{{spaces}}},
            "BuildingSurface:Detailed": {{{surfaces}}},
            "SurfaceProperty:SolarIncidentInside": {{{incidents}}}
            {extra_objects}
        }}"#
    )
}

fn surface(name: &str, zone: &str, space: Option<&str>, construction: &str, x: f64) -> String {
    let space = space.map_or_else(String::new, |space| format!(r#", "space_name":"{space}""#));
    format!(
        r#""{name}": {{
            "surface_type":"Wall",
            "construction_name":"{construction}",
            "zone_name":"{zone}"{space},
            "outside_boundary_condition":"Outdoors",
            "vertices":[
                {{"vertex_x_coordinate":{x},"vertex_y_coordinate":0,"vertex_z_coordinate":0}},
                {{"vertex_x_coordinate":{x},"vertex_y_coordinate":1,"vertex_z_coordinate":0}},
                {{"vertex_x_coordinate":{x},"vertex_y_coordinate":1,"vertex_z_coordinate":1}}
            ]
        }}"#
    )
}

fn incident(name: &str, surface: &str, construction: &str) -> String {
    format!(
        r#""{name}": {{
            "surface_name":"{surface}",
            "construction_name":"{construction}",
            "inside_surface_incident_sun_solar_radiation_schedule_name":"Solar Schedule"
        }}"#
    )
}

fn compile(epjson: &str) -> Result<CompileResult, Box<dyn std::error::Error>> {
    Ok(compile_raw_model(&parse_epjson_str(epjson)?))
}

fn scheduled_gain_warnings(result: &CompileResult) -> Vec<&super::super::ModelDiagnostic> {
    result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == WARNING_CODE)
        .collect()
}

fn single_layer_complex_request(include_schedule: bool) -> String {
    let schedule = include_schedule
        .then_some(r#", "layer_1_solar_radiation_absorbed_schedule_name":"Solar Schedule""#);
    format!(
        r#",
        "WindowMaterial:Glazing": {{
            "Complex Glass": {{"optical_data_type":"SpectralAverage","thickness":0.003}}
        }},
        "WindowThermalModel:Params": {{"Thermal": {{}}}},
        "Matrix:TwoDimension": {{
            "Unit Matrix": {{"number_of_rows":1,"number_of_columns":1,"values":[{{"value":0}}]}}
        }},
        "Construction:ComplexFenestrationState": {{
            "CFS Single": {{
                "window_thermal_model":"Thermal",
                "basis_matrix_name":"Unit Matrix",
                "solar_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "solar_optical_complex_back_reflectance_matrix_name":"Unit Matrix",
                "visible_optical_complex_front_transmittance_matrix_name":"Unit Matrix",
                "visible_optical_complex_back_transmittance_matrix_name":"Unit Matrix",
                "outside_layer_name":"Complex Glass",
                "outside_layer_directional_front_absorptance_matrix_name":"Unit Matrix",
                "outside_layer_directional_back_absorptance_matrix_name":"Unit Matrix"
            }}
        }},
        "ComplexFenestrationProperty:SolarAbsorbedLayers": {{
            "Window Gain": {{
                "fenestration_surface":"Raw Scheduled Window",
                "construction_name":"CFS Single"{}
            }}
        }}"#,
        schedule.unwrap_or_default()
    )
}

#[test]
fn no_scheduled_surface_objects_do_not_warn() -> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface("Wall One", "Zone One", None, "Current Construction", 0.0),
        surface("Wall Two", "Zone One", None, "Current Construction", 1.0),
    ]
    .join(",");
    let result = compile(&fixture(r#""Zone One":{}"#, "", &surfaces, "", ""))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    assert!(scheduled_gain_warnings(&result).is_empty());
    Ok(())
}

#[test]
fn one_exact_pair_and_an_all_unmatched_subset_do_not_warn() -> Result<(), Box<dyn std::error::Error>>
{
    let one_surface = surface("Only Wall", "Zone One", None, "Current Construction", 0.0);
    let one_exact = compile(&fixture(
        r#""Zone One":{}"#,
        "",
        &one_surface,
        &incident("Only Gain", "Only Wall", "Current Construction"),
        "",
    ))?;
    assert!(
        !one_exact.has_errors(),
        "{:?}",
        one_exact.report.diagnostics
    );
    assert!(scheduled_gain_warnings(&one_exact).is_empty());

    let two_surfaces = [
        surface("Wall One", "Zone One", None, "Current Construction", 0.0),
        surface("Wall Two", "Zone One", None, "Current Construction", 1.0),
    ]
    .join(",");
    let all_unmatched = compile(&fixture(
        r#""Zone One":{}"#,
        "",
        &two_surfaces,
        &incident("Mismatched Gain", "Wall One", "Alternate Construction"),
        "",
    ))?;
    assert!(
        !all_unmatched.has_errors(),
        "{:?}",
        all_unmatched.report.diagnostics
    );
    assert!(scheduled_gain_warnings(&all_unmatched).is_empty());
    Ok(())
}

#[test]
fn same_space_mixed_subset_warns_and_preserves_model_with_arena_ordered_names()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface(
            "Zulu Unscheduled",
            "Zone One",
            None,
            "Current Construction",
            2.0,
        ),
        surface(
            "Alpha Scheduled",
            "Zone One",
            None,
            "Current Construction",
            0.0,
        ),
        surface(
            "Beta Unscheduled",
            "Zone One",
            None,
            "Current Construction",
            1.0,
        ),
    ]
    .join(",");
    let result = compile(&fixture(
        r#""Zone One":{}"#,
        "",
        &surfaces,
        &incident("Alpha Gain", "Alpha Scheduled", "Current Construction"),
        "",
    ))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .expect("warnings retain a typed model");
    assert_eq!(
        model
            .surfaces
            .iter()
            .map(|surface| surface.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ALPHA SCHEDULED", "BETA UNSCHEDULED", "ZULU UNSCHEDULED"]
    );
    let warnings = scheduled_gain_warnings(&result);
    assert_eq!(warnings.len(), 1);
    let warning = warnings[0];
    assert_eq!(warning.severity, DiagnosticSeverity::Warning);
    assert_eq!(warning.object_type, "Zone");
    assert_eq!(warning.object_name.as_deref(), Some("ZONE ONE"));
    assert_eq!(warning.field, None);
    assert!(warning.message.contains("mixed retained typed"));
    let beta = warning.message.find("BETA UNSCHEDULED").expect("beta name");
    let zulu = warning.message.find("ZULU UNSCHEDULED").expect("zulu name");
    assert!(beta < zulu, "{}", warning.message);
    Ok(())
}

#[test]
fn authored_default_and_remainder_spaces_each_preserve_mixed_zone_proofs()
-> Result<(), Box<dyn std::error::Error>> {
    let zones = r#""Authored Zone":{},"Default Zone":{},"Remainder Zone":{}"#;
    let spaces = r#"
        "Authored A":{"zone_name":"Authored Zone"},
        "Authored B":{"zone_name":"Authored Zone"},
        "Remainder Authored":{"zone_name":"Remainder Zone"}
    "#;
    let surfaces = [
        surface(
            "Authored Scheduled",
            "Authored Zone",
            Some("Authored A"),
            "Current Construction",
            0.0,
        ),
        surface(
            "Authored Unscheduled",
            "Authored Zone",
            Some("Authored B"),
            "Current Construction",
            1.0,
        ),
        surface(
            "Default Scheduled",
            "Default Zone",
            None,
            "Current Construction",
            2.0,
        ),
        surface(
            "Default Unscheduled",
            "Default Zone",
            None,
            "Current Construction",
            3.0,
        ),
        surface(
            "Remainder Scheduled",
            "Remainder Zone",
            Some("Remainder Authored"),
            "Current Construction",
            4.0,
        ),
        surface(
            "Remainder Unscheduled",
            "Remainder Zone",
            None,
            "Current Construction",
            5.0,
        ),
    ]
    .join(",");
    let incidents = [
        incident(
            "Authored Gain",
            "Authored Scheduled",
            "Current Construction",
        ),
        incident("Default Gain", "Default Scheduled", "Current Construction"),
        incident(
            "Remainder Gain",
            "Remainder Scheduled",
            "Current Construction",
        ),
    ]
    .join(",");
    let result = compile(&fixture(zones, spaces, &surfaces, &incidents, ""))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result.model.as_ref().expect("mixed warnings retain model");
    assert!(
        model
            .spaces
            .iter()
            .any(|space| space.origin == SpaceOrigin::AutoZoneDefault)
    );
    assert!(
        model
            .spaces
            .iter()
            .any(|space| space.origin == SpaceOrigin::AutoZoneRemainder)
    );
    assert_eq!(
        scheduled_gain_warnings(&result)
            .iter()
            .map(|warning| warning.object_name.as_deref().expect("zone name"))
            .collect::<Vec<_>>(),
        vec!["AUTHORED ZONE", "DEFAULT ZONE", "REMAINDER ZONE"]
    );
    Ok(())
}

#[test]
fn current_construction_exactness_ignores_mismatches_and_allows_other_pairs_for_same_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface("Wall One", "Zone One", None, "Current Construction", 0.0),
        surface("Wall Two", "Zone One", None, "Current Construction", 1.0),
    ]
    .join(",");
    let mismatch = incident("Alternate Gain", "Wall One", "Alternate Construction");
    let mismatch_only = compile(&fixture(r#""Zone One":{}"#, "", &surfaces, &mismatch, ""))?;
    assert!(scheduled_gain_warnings(&mismatch_only).is_empty());

    let same_surface_pairs = [
        mismatch,
        incident("Current Gain", "Wall One", "Current Construction"),
    ]
    .join(",");
    let exact_and_other = compile(&fixture(
        r#""Zone One":{}"#,
        "",
        &surfaces,
        &same_surface_pairs,
        "",
    ))?;
    assert!(!exact_and_other.has_errors());
    let warnings = scheduled_gain_warnings(&exact_and_other);
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("WALL TWO"));
    assert!(!warnings[0].message.contains("WALL ONE,"));
    Ok(())
}

#[test]
fn only_the_mixed_zone_warns_in_a_multi_zone_model() -> Result<(), Box<dyn std::error::Error>> {
    let zones = r#""Mixed Zone":{},"Matched Zone":{},"Unmatched Zone":{}"#;
    let surfaces = [
        surface(
            "Mixed Scheduled",
            "Mixed Zone",
            None,
            "Current Construction",
            0.0,
        ),
        surface(
            "Mixed Unscheduled",
            "Mixed Zone",
            None,
            "Current Construction",
            1.0,
        ),
        surface(
            "Matched Wall",
            "Matched Zone",
            None,
            "Current Construction",
            2.0,
        ),
        surface(
            "Unmatched Wall",
            "Unmatched Zone",
            None,
            "Current Construction",
            3.0,
        ),
    ]
    .join(",");
    let incidents = [
        incident("Mixed Gain", "Mixed Scheduled", "Current Construction"),
        incident("Matched Gain", "Matched Wall", "Current Construction"),
    ]
    .join(",");
    let result = compile(&fixture(zones, "", &surfaces, &incidents, ""))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let warnings = scheduled_gain_warnings(&result);
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].object_name.as_deref(), Some("MIXED ZONE"));
    Ok(())
}

#[test]
fn cp101_only_input_has_no_opaque_subset_warning() -> Result<(), Box<dyn std::error::Error>> {
    let result = compile(&fixture(
        "",
        "",
        "",
        "",
        &single_layer_complex_request(true),
    ))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        result
            .model
            .as_ref()
            .expect("typed model")
            .fenestration_solar_absorbed_requests
            .len(),
        1
    );
    assert!(scheduled_gain_warnings(&result).is_empty());
    Ok(())
}

#[test]
fn raw_fenestration_and_legacy_surfaces_cannot_erase_an_existing_mixed_proof()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface(
            "Typed Scheduled",
            "Zone One",
            None,
            "Current Construction",
            0.0,
        ),
        surface(
            "Typed Unscheduled",
            "Zone One",
            None,
            "Current Construction",
            1.0,
        ),
    ]
    .join(",");
    let extras = r#",
        "FenestrationSurface:Detailed": {"Raw Window": {}},
        "Wall:Detailed": {"Legacy Wall": {}}
    "#;
    let result = compile(&fixture(
        r#""Zone One":{}"#,
        "",
        &surfaces,
        &incident("Typed Gain", "Typed Scheduled", "Current Construction"),
        extras,
    ))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(scheduled_gain_warnings(&result).len(), 1);
    Ok(())
}

#[test]
fn possible_fenestration_only_mixture_does_not_create_an_unproved_warning()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface("Opaque One", "Zone One", None, "Current Construction", 0.0),
        surface("Opaque Two", "Zone One", None, "Current Construction", 1.0),
    ]
    .join(",");
    let extras = format!(
        "{}{}",
        single_layer_complex_request(true),
        r#", "FenestrationSurface:Detailed": {
            "Raw Scheduled Window": {}, "Raw Unscheduled Window": {}
        }"#
    );
    let result = compile(&fixture(r#""Zone One":{}"#, "", &surfaces, "", &extras))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        result
            .model
            .as_ref()
            .expect("typed model")
            .fenestration_solar_absorbed_requests
            .len(),
        1
    );
    assert!(scheduled_gain_warnings(&result).is_empty());
    Ok(())
}

#[test]
fn empty_zone_does_not_reproduce_the_source_first_surface_quirk()
-> Result<(), Box<dyn std::error::Error>> {
    let occupied_surface = surface(
        "Only Wall",
        "Occupied Zone",
        None,
        "Current Construction",
        0.0,
    );
    let result = compile(&fixture(
        r#""Empty Zone":{},"Occupied Zone":{}"#,
        "",
        &occupied_surface,
        &incident("Only Gain", "Only Wall", "Current Construction"),
        "",
    ))?;

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(scheduled_gain_warnings(&result).is_empty());
    Ok(())
}

#[test]
fn related_surface_or_scheduled_input_error_suppresses_all_subset_warnings()
-> Result<(), Box<dyn std::error::Error>> {
    let valid_surfaces = [
        surface(
            "Alpha Scheduled",
            "Zone One",
            None,
            "Current Construction",
            0.0,
        ),
        surface(
            "Beta Unscheduled",
            "Zone One",
            None,
            "Current Construction",
            1.0,
        ),
    ]
    .join(",");
    let scheduled = incident("Alpha Gain", "Alpha Scheduled", "Current Construction");

    let invalid_surface = r#""Invalid Surface": {
        "surface_type":"Wall", "zone_name":"Zone One",
        "outside_boundary_condition":"Outdoors", "vertices":[]
    }"#;
    let surface_error_result = compile(&fixture(
        r#""Zone One":{}"#,
        "",
        &[valid_surfaces.clone(), invalid_surface.to_string()].join(","),
        &scheduled,
        "",
    ))?;
    assert!(surface_error_result.has_errors());
    assert!(scheduled_gain_warnings(&surface_error_result).is_empty());

    let scheduled_error_result = compile(&fixture(
        r#""Zone One":{}"#,
        "",
        &valid_surfaces,
        &scheduled,
        &single_layer_complex_request(false),
    ))?;
    assert!(scheduled_error_result.has_errors());
    assert!(scheduled_gain_warnings(&scheduled_error_result).is_empty());
    Ok(())
}
