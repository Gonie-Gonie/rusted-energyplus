use std::collections::{BTreeMap, BTreeSet};

use ep_compare::{
    EioError, EioMaterialDetails, EioWindowMaterialScreen, Tolerance,
    WINDOW_MATERIAL_SCREEN_HEADER, parse_eio_material_details, parse_eio_window_material_screen,
};
use ep_compiler::compile_raw_model;
use ep_model::{ConstructionKind, MaterialDefinition, TypedModel, WindowScreenMaterial};
use ep_raw_model::{RawModel, RawObject, RawValue, load_epjson_file};

#[path = "window_material_screen_optics.rs"]
mod optics;
use optics::{ScreenSourceOptics, calculate_screen_source_optics};

pub(crate) const USAGE: &str = "usage: eplus-rs compare window-material-screen <input.epJSON> <eplusout.eio> [--tolerance exact|near]";

const OUTPUT_CONSTRUCTIONS_OBJECT_TYPE: &str = "Output:Constructions";
const WINDOW_SHADING_CONTROL_OBJECT_TYPE: &str = "WindowShadingControl";
const FENESTRATION_SURFACE_OBJECT_TYPE: &str = "FenestrationSurface:Detailed";
const NEAR_TOLERANCE: Tolerance = Tolerance {
    absolute: 0.00001,
    relative: 0.000001,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NumericToleranceMode {
    Exact,
    Near,
}

impl NumericToleranceMode {
    const fn label(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Near => "near",
        }
    }

    const fn policy_label(self) -> &'static str {
        match self {
            Self::Exact => {
                "energyplus-26.1-material-details-4R-screen-thickness-5R-other-specialized-3R-normalized-exact"
            }
            Self::Near => "absolute-0.00001-relative-0.000001",
        }
    }

    fn accepts_specialized(self, input: f64, observed: f64, precision: usize) -> bool {
        let Some(expected) = energyplus_round_sig_digits_nonnegative(input, precision) else {
            return false;
        };
        match self {
            Self::Exact => expected == observed,
            Self::Near => NEAR_TOLERANCE.accepts(expected, observed),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ConstructionReportRequests {
    constructions: bool,
    materials: bool,
}

#[derive(Clone, Debug)]
struct WindowScreenDefinition {
    material_name: String,
    fields: WindowScreenMaterial,
}

#[derive(Clone, Debug)]
struct WindowScreenOccurrence {
    construction_name: String,
    layer_number: usize,
    material_name: String,
    fields: WindowScreenMaterial,
    optics: ScreenSourceOptics,
}

#[derive(Debug)]
struct WindowScreenComparison {
    definitions: Vec<WindowScreenDefinition>,
    occurrences: Vec<WindowScreenOccurrence>,
    activated_material_names: BTreeSet<String>,
    oracle_material_details: Vec<EioMaterialDetails>,
    oracle_occurrences: Vec<EioWindowMaterialScreen>,
    report_requests: ConstructionReportRequests,
    header_rows: usize,
    tolerance_mode: NumericToleranceMode,
    passed: bool,
    first_divergence: Option<String>,
}

pub(crate) fn run_compare_window_material_screen(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("{USAGE}");
        return 2;
    };
    let Some(eio_path) = args.get(1) else {
        eprintln!("missing eplusout.eio path");
        eprintln!("{USAGE}");
        return 2;
    };
    let tolerance_mode = match parse_tolerance_mode(&args[2..]) {
        Ok(mode) => mode,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("{USAGE}");
            return 2;
        }
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        super::print_compile_diagnostics(&result.report);
        return 1;
    };
    if window_screen_definitions(&model).is_empty() {
        eprintln!("no WindowMaterial:Screen objects are available for comparison");
        return 1;
    }

    let eio_contents = match std::fs::read_to_string(eio_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("failed to read {}: {error}", eio_path);
            return 1;
        }
    };
    let comparison =
        match compare_window_material_screen(&raw_model, &model, &eio_contents, tolerance_mode) {
            Ok(comparison) => comparison,
            Err(error) => {
                eprintln!("{error}");
                return 1;
            }
        };

    render_comparison(&comparison);
    if comparison.passed { 0 } else { 1 }
}

fn parse_tolerance_mode(args: &[String]) -> Result<NumericToleranceMode, String> {
    match args {
        [] => Ok(NumericToleranceMode::Exact),
        [flag, value] if flag == "--tolerance" => match value.as_str() {
            "exact" => Ok(NumericToleranceMode::Exact),
            "near" => Ok(NumericToleranceMode::Near),
            _ => Err(format!(
                "unsupported WindowMaterial:Screen tolerance mode: {value}"
            )),
        },
        _ => Err("unsupported WindowMaterial:Screen compare options".to_string()),
    }
}

fn compare_window_material_screen(
    raw_model: &RawModel,
    model: &TypedModel,
    eio_contents: &str,
    tolerance_mode: NumericToleranceMode,
) -> Result<WindowScreenComparison, String> {
    let definitions = window_screen_definitions(model);
    let report_requests = construction_report_requests(raw_model)?;
    let has_window_construction = model
        .constructions
        .iter()
        .any(|construction| construction.kind == ConstructionKind::Fenestration);
    let expected_header_rows = usize::from(
        report_requests.constructions && has_window_construction && !definitions.is_empty(),
    );
    let has_screen_construction_occurrence = has_window_screen_construction_occurrence(model);

    let activated_material_names =
        if expected_header_rows == 1 && has_screen_construction_occurrence {
            activated_window_screen_material_names(raw_model, model)?
        } else {
            BTreeSet::new()
        };
    let occurrences = if report_requests.constructions && has_screen_construction_occurrence {
        window_screen_occurrences(model, &activated_material_names)?
    } else {
        Vec::new()
    };

    let oracle_material_details = match parse_eio_material_details(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingMaterialDetails) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let oracle_occurrences = match parse_eio_window_material_screen(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingWindowMaterialScreenHeader) if expected_header_rows == 0 => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let header_rows = window_material_screen_header_count(eio_contents);
    let mut passed = true;
    let mut first_divergence = None;

    if header_rows != expected_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "WindowMaterial:Screen header expected {expected_header_rows} observed {header_rows}"
            ),
        );
    }

    let generic_indices =
        indices_by_material_name(&oracle_material_details, |row| row.material_name.as_str());
    for definition in &definitions {
        let matching_indices = generic_indices
            .get(&normalized_material_name(&definition.material_name))
            .map(Vec::as_slice)
            .unwrap_or_default();
        let expected_rows = usize::from(report_requests.materials);
        if matching_indices.len() != expected_rows {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!(
                    "material {} expected {expected_rows} Material Details row(s) observed {}",
                    definition.material_name,
                    matching_indices.len()
                ),
            );
            continue;
        }
        if let Some(index) = matching_indices.first() {
            let oracle = &oracle_material_details[*index];
            if !generic_row_matches(definition, oracle) {
                passed = false;
                record_generic_divergence(&mut first_divergence, definition, oracle);
            }
        }
    }

    if occurrences.len() != oracle_occurrences.len() {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "screen construction occurrences expected {} observed {}",
                occurrences.len(),
                oracle_occurrences.len()
            ),
        );
    }
    for (occurrence_index, (expected, observed)) in
        occurrences.iter().zip(&oracle_occurrences).enumerate()
    {
        if !specialized_row_matches(expected, observed, tolerance_mode) {
            passed = false;
            record_specialized_divergence(
                &mut first_divergence,
                expected,
                observed,
                tolerance_mode,
                occurrence_index,
            );
        }
    }

    let definition_names = definitions
        .iter()
        .map(|definition| normalized_material_name(&definition.material_name))
        .collect::<BTreeSet<_>>();
    if let Some(unexpected) = oracle_occurrences
        .iter()
        .find(|row| !definition_names.contains(&normalized_material_name(&row.material_name)))
    {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "unexpected WindowMaterial:Screen row for material {}",
                unexpected.material_name
            ),
        );
    }

    Ok(WindowScreenComparison {
        definitions,
        occurrences,
        activated_material_names,
        oracle_material_details,
        oracle_occurrences,
        report_requests,
        header_rows,
        tolerance_mode,
        passed,
        first_divergence,
    })
}

fn construction_report_requests(
    raw_model: &RawModel,
) -> Result<ConstructionReportRequests, String> {
    let mut requests = ConstructionReportRequests::default();
    for (report_name, report) in raw_model
        .ordered_instances(OUTPUT_CONSTRUCTIONS_OBJECT_TYPE)
        .map_err(|error| error.to_string())?
    {
        for field_name in ["details_type_1", "details_type_2"] {
            let Some(value) = raw_field(report, field_name) else {
                continue;
            };
            let RawValue::String(value) = value else {
                return Err(format!(
                    "{OUTPUT_CONSTRUCTIONS_OBJECT_TYPE}/{} field {field_name} must be a string",
                    report_name.0
                ));
            };
            if value.trim().eq_ignore_ascii_case("Constructions") {
                requests.constructions = true;
            }
            if value.trim().eq_ignore_ascii_case("Materials") {
                requests.materials = true;
            }
        }
    }
    Ok(requests)
}

fn window_screen_definitions(model: &TypedModel) -> Vec<WindowScreenDefinition> {
    model
        .materials
        .iter()
        .filter_map(|material| {
            let MaterialDefinition::WindowScreen(fields) = material.definition else {
                return None;
            };
            Some(WindowScreenDefinition {
                material_name: material.name.0.clone(),
                fields,
            })
        })
        .collect()
}

fn has_window_screen_construction_occurrence(model: &TypedModel) -> bool {
    model.constructions.iter().any(|construction| {
        if construction.kind != ConstructionKind::Fenestration {
            return false;
        }
        let layers = if construction.layers.is_empty() {
            std::slice::from_ref(&construction.outside_layer)
        } else {
            construction.layers.as_slice()
        };
        layers.iter().any(|material_id| {
            model.materials.iter().any(|material| {
                material.id == *material_id
                    && matches!(material.definition, MaterialDefinition::WindowScreen(_))
            })
        })
    })
}

fn activated_window_screen_material_names(
    raw_model: &RawModel,
    model: &TypedModel,
) -> Result<BTreeSet<String>, String> {
    let fenestration_surfaces = raw_model
        .ordered_instances(FENESTRATION_SURFACE_OBJECT_TYPE)
        .map_err(|error| error.to_string())?;
    let mut activated = BTreeSet::new();
    let mut surface_controls = BTreeMap::<String, String>::new();

    for (control_name, control) in raw_model
        .ordered_instances(WINDOW_SHADING_CONTROL_OBJECT_TYPE)
        .map_err(|error| error.to_string())?
    {
        let shading_type = required_raw_string(
            WINDOW_SHADING_CONTROL_OBJECT_TYPE,
            &control_name.0,
            control,
            "shading_type",
        )?;

        let surface_names = match raw_field(control, "fenestration_surfaces") {
            None => Vec::new(),
            Some(RawValue::Array(surface_values)) => {
                let mut names = Vec::with_capacity(surface_values.len());
                for (surface_index, value) in surface_values.iter().enumerate() {
                    let RawValue::Object(fields) = value else {
                        return Err(format!(
                            "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} fenestration_surfaces entry {} must be an object",
                            control_name.0,
                            surface_index + 1
                        ));
                    };
                    let entry = RawObject {
                        fields: fields.clone(),
                        source_span: None,
                    };
                    names.push(required_raw_string(
                        WINDOW_SHADING_CONTROL_OBJECT_TYPE,
                        &control_name.0,
                        &entry,
                        "fenestration_surface_name",
                    )?);
                }
                names
            }
            Some(_) => {
                return Err(format!(
                    "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} field fenestration_surfaces must be an array",
                    control_name.0
                ));
            }
        };
        for surface_name in &surface_names {
            let normalized_surface_name = normalized_material_name(surface_name);
            if let Some(previous_control) =
                surface_controls.insert(normalized_surface_name, control_name.0.clone())
            {
                return Err(format!(
                    "{FENESTRATION_SURFACE_OBJECT_TYPE}/{surface_name} is referenced by multiple WindowShadingControl entries ({previous_control} and {}); bounded Screen EIO comparison rejects source active-control selection ambiguity",
                    control_name.0
                ));
            }
        }

        if !shading_type.eq_ignore_ascii_case("ExteriorScreen") {
            continue;
        }

        let shaded_construction_name = required_raw_string(
            WINDOW_SHADING_CONTROL_OBJECT_TYPE,
            &control_name.0,
            control,
            "construction_with_shading_name",
        )?;
        let shaded_construction = model
            .constructions
            .iter()
            .find(|construction| {
                normalized_material_name(&construction.name.0)
                    == normalized_material_name(&shaded_construction_name)
            })
            .ok_or_else(|| {
                format!(
                    "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} references missing typed shaded construction {shaded_construction_name}",
                    control_name.0
                )
            })?;
        if shaded_construction.kind != ConstructionKind::Fenestration {
            return Err(format!(
                "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} shaded construction {shaded_construction_name} is not fenestration",
                control_name.0
            ));
        }
        let screen_material = model
            .materials
            .iter()
            .find(|material| material.id == shaded_construction.outside_layer)
            .ok_or_else(|| {
                format!(
                    "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} shaded construction {shaded_construction_name} has a missing outside material",
                    control_name.0
                )
            })?;
        if !matches!(
            screen_material.definition,
            MaterialDefinition::WindowScreen(_)
        ) {
            return Err(format!(
                "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} shaded construction {shaded_construction_name} does not start with WindowMaterial:Screen",
                control_name.0
            ));
        }

        if surface_names.is_empty() {
            return Err(format!(
                "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} requires a nonempty explicit fenestration_surfaces list for the bounded Screen EIO comparison",
                control_name.0
            ));
        }

        for surface_name in surface_names {
            let Some((_raw_name, surface)) =
                fenestration_surfaces.iter().find(|(name, _surface)| {
                    normalized_material_name(&name.0) == normalized_material_name(&surface_name)
                })
            else {
                return Err(format!(
                    "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} references missing {FENESTRATION_SURFACE_OBJECT_TYPE}/{surface_name}",
                    control_name.0
                ));
            };
            let bare_construction_name = required_raw_string(
                FENESTRATION_SURFACE_OBJECT_TYPE,
                &surface_name,
                surface,
                "construction_name",
            )?;
            let bare_construction = model
                .constructions
                .iter()
                .find(|construction| {
                    normalized_material_name(&construction.name.0)
                        == normalized_material_name(&bare_construction_name)
                })
                .ok_or_else(|| {
                    format!(
                        "{FENESTRATION_SURFACE_OBJECT_TYPE}/{surface_name} references missing typed construction {bare_construction_name}"
                    )
                })?;
            if bare_construction.kind != ConstructionKind::Fenestration {
                return Err(format!(
                    "{FENESTRATION_SURFACE_OBJECT_TYPE}/{surface_name} construction {bare_construction_name} is not fenestration"
                ));
            }
            if shaded_construction.layers.get(1..) != Some(bare_construction.layers.as_slice()) {
                return Err(format!(
                    "{WINDOW_SHADING_CONTROL_OBJECT_TYPE}/{} shaded construction {shaded_construction_name} must equal one exterior Screen followed by the bare layers of {bare_construction_name}",
                    control_name.0
                ));
            }
        }

        activated.insert(normalized_material_name(&screen_material.name.0));
    }

    Ok(activated)
}

fn window_screen_occurrences(
    model: &TypedModel,
    activated_material_names: &BTreeSet<String>,
) -> Result<Vec<WindowScreenOccurrence>, String> {
    let mut rows = Vec::new();
    for construction in &model.constructions {
        if construction.kind != ConstructionKind::Fenestration {
            continue;
        }
        let layers = if construction.layers.is_empty() {
            std::slice::from_ref(&construction.outside_layer)
        } else {
            construction.layers.as_slice()
        };
        for (layer_index, material_id) in layers.iter().enumerate() {
            let material = model
                .materials
                .iter()
                .find(|material| material.id == *material_id)
                .ok_or_else(|| {
                    format!(
                        "construction {} references missing material layer {}",
                        construction.name.0,
                        layer_index + 1
                    )
                })?;
            let MaterialDefinition::WindowScreen(fields) = material.definition else {
                continue;
            };
            let normalized_name = normalized_material_name(&material.name.0);
            if !activated_material_names.contains(&normalized_name) {
                return Err(format!(
                    "construction {} layer {} uses WindowMaterial:Screen {} without an explicit ExteriorScreen WindowShadingControl activation; source static optics are unavailable",
                    construction.name.0,
                    layer_index + 1,
                    material.name.0
                ));
            }
            if layer_index != 0 {
                return Err(format!(
                    "construction {} layer {} uses WindowMaterial:Screen {} away from the exterior layer; EnergyPlus Screen EIO reporting requires an exterior Screen with a matching bare fenestration construction",
                    construction.name.0,
                    layer_index + 1,
                    material.name.0
                ));
            }
            let bare_layers = &layers[1..];
            let has_bare_companion = !bare_layers.is_empty()
                && model.constructions.iter().any(|candidate| {
                    if candidate.kind != ConstructionKind::Fenestration
                        || candidate.id == construction.id
                    {
                        return false;
                    }
                    let candidate_layers = if candidate.layers.is_empty() {
                        std::slice::from_ref(&candidate.outside_layer)
                    } else {
                        candidate.layers.as_slice()
                    };
                    candidate_layers == bare_layers
                });
            if !has_bare_companion {
                return Err(format!(
                    "construction {} uses WindowMaterial:Screen {} without an exact bare fenestration construction matching its post-Screen layer tail; EnergyPlus omits this specialized row",
                    construction.name.0, material.name.0
                ));
            }
            rows.push(WindowScreenOccurrence {
                construction_name: construction.name.0.clone(),
                layer_number: layer_index + 1,
                material_name: material.name.0.clone(),
                fields,
                optics: calculate_screen_source_optics(fields)?,
            });
        }
    }
    Ok(rows)
}

fn generic_numeric_fields(
    definition: &WindowScreenDefinition,
    oracle: &EioMaterialDetails,
) -> [(&'static str, f64, f64); 8] {
    [
        (
            "thermal_resistance_m2_k_per_w",
            0.0,
            oracle.thermal_resistance_m2_k_per_w,
        ),
        (
            "thickness_m",
            energyplus_round_sig_digits_nonnegative(definition.fields.thickness_m(), 4)
                .unwrap_or(f64::NAN),
            oracle.thickness_m,
        ),
        (
            "conductivity_w_per_m_k",
            energyplus_round_sig_digits_nonnegative(definition.fields.conductivity_w_per_m_k, 3)
                .unwrap_or(f64::NAN),
            oracle.conductivity_w_per_m_k,
        ),
        ("density_kg_per_m3", 0.0, oracle.density_kg_per_m3),
        (
            "specific_heat_j_per_kg_k",
            0.0,
            oracle.specific_heat_j_per_kg_k,
        ),
        (
            "thermal_absorptance",
            energyplus_round_sig_digits_nonnegative(definition.fields.thermal_absorptance, 4)
                .unwrap_or(f64::NAN),
            oracle.thermal_absorptance,
        ),
        (
            "solar_absorptance",
            energyplus_round_sig_digits_nonnegative(definition.fields.solar_absorptance, 4)
                .unwrap_or(f64::NAN),
            oracle.solar_absorptance,
        ),
        (
            "visible_absorptance",
            energyplus_round_sig_digits_nonnegative(definition.fields.visible_absorptance, 4)
                .unwrap_or(f64::NAN),
            oracle.visible_absorptance,
        ),
    ]
}

fn generic_row_matches(definition: &WindowScreenDefinition, oracle: &EioMaterialDetails) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&definition.material_name)
        && oracle.roughness == "MediumRough"
        && generic_numeric_fields(definition, oracle)
            .into_iter()
            .all(|(_field, expected, observed)| expected == observed)
}

fn record_generic_divergence(
    first_divergence: &mut Option<String>,
    definition: &WindowScreenDefinition,
    oracle: &EioMaterialDetails,
) {
    let prefix = format!("material {}", definition.material_name);
    if normalized_material_name(&oracle.material_name)
        != normalized_material_name(&definition.material_name)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field material_name expected {} observed {}",
                definition.material_name, oracle.material_name
            ),
        );
        return;
    }
    if oracle.roughness != "MediumRough" {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field roughness expected MediumRough observed {}",
                oracle.roughness
            ),
        );
        return;
    }
    if let Some((field, expected, observed)) = generic_numeric_fields(definition, oracle)
        .into_iter()
        .find(|(_field, expected, observed)| expected != observed)
    {
        record_first_divergence(
            first_divergence,
            format!("{prefix} field {field} expected {expected:.9} observed {observed:.9}"),
        );
    }
}

fn specialized_numeric_fields(
    occurrence: &WindowScreenOccurrence,
    oracle: &EioWindowMaterialScreen,
) -> [(&'static str, f64, f64, usize); 10] {
    [
        (
            "thickness_m",
            occurrence.fields.thickness_m(),
            oracle.thickness_m,
            5,
        ),
        (
            "conductivity_w_per_m_k",
            occurrence.fields.conductivity_w_per_m_k,
            oracle.conductivity_w_per_m_k,
            3,
        ),
        (
            "thermal_absorptance",
            occurrence.fields.thermal_absorptance,
            oracle.thermal_absorptance,
            3,
        ),
        (
            "solar_transmittance",
            occurrence.optics.normal_solar_transmittance,
            oracle.solar_transmittance,
            3,
        ),
        (
            "solar_reflectance",
            occurrence.optics.normal_solar_reflectance,
            oracle.solar_reflectance,
            3,
        ),
        (
            "visible_reflectance",
            occurrence.optics.normal_visible_reflectance,
            oracle.visible_reflectance,
            3,
        ),
        (
            "diffuse_solar_reflectance",
            occurrence.optics.diffuse_solar_reflectance,
            oracle.diffuse_solar_reflectance,
            3,
        ),
        (
            "diffuse_visible_reflectance",
            occurrence.optics.diffuse_visible_reflectance,
            oracle.diffuse_visible_reflectance,
            3,
        ),
        (
            "diameter_to_spacing_ratio",
            occurrence.optics.diameter_to_spacing_ratio,
            oracle.diameter_to_spacing_ratio,
            3,
        ),
        (
            "screen_to_glass_distance_m",
            occurrence.fields.screen_to_glass_distance_m,
            oracle.screen_to_glass_distance_m,
            3,
        ),
    ]
}

fn specialized_row_matches(
    occurrence: &WindowScreenOccurrence,
    oracle: &EioWindowMaterialScreen,
    tolerance_mode: NumericToleranceMode,
) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&occurrence.material_name)
        && specialized_numeric_fields(occurrence, oracle)
            .into_iter()
            .all(|(_field, input, observed, precision)| {
                tolerance_mode.accepts_specialized(input, observed, precision)
            })
}

fn record_specialized_divergence(
    first_divergence: &mut Option<String>,
    occurrence: &WindowScreenOccurrence,
    oracle: &EioWindowMaterialScreen,
    tolerance_mode: NumericToleranceMode,
    occurrence_index: usize,
) {
    let prefix = format!(
        "occurrence {} construction {} layer {} material {}",
        occurrence_index + 1,
        occurrence.construction_name,
        occurrence.layer_number,
        occurrence.material_name
    );
    if normalized_material_name(&oracle.material_name)
        != normalized_material_name(&occurrence.material_name)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field material_name expected {} observed {}",
                occurrence.material_name, oracle.material_name
            ),
        );
        return;
    }
    if let Some((field, input, observed, precision)) =
        specialized_numeric_fields(occurrence, oracle)
            .into_iter()
            .find(|(_field, input, observed, precision)| {
                !tolerance_mode.accepts_specialized(*input, *observed, *precision)
            })
    {
        let expected =
            energyplus_round_sig_digits_nonnegative(input, precision).unwrap_or(f64::NAN);
        record_first_divergence(
            first_divergence,
            format!("{prefix} field {field} expected {expected:.9} observed {observed:.9}"),
        );
    }
}

fn indices_by_material_name<T>(
    rows: &[T],
    material_name: impl Fn(&T) -> &str,
) -> BTreeMap<String, Vec<usize>> {
    let mut indices = BTreeMap::<String, Vec<usize>>::new();
    for (index, row) in rows.iter().enumerate() {
        indices
            .entry(normalized_material_name(material_name(row)))
            .or_default()
            .push(index);
    }
    indices
}

fn normalized_material_name(name: &str) -> String {
    name.trim().to_ascii_uppercase()
}

fn raw_field<'a>(object: &'a RawObject, field_name: &str) -> Option<&'a RawValue> {
    object
        .fields
        .iter()
        .find(|(field, _value)| field.0 == field_name)
        .map(|(_field, value)| value)
}

fn required_raw_string(
    object_type: &str,
    object_name: &str,
    object: &RawObject,
    field_name: &str,
) -> Result<String, String> {
    let Some(value) = raw_field(object, field_name) else {
        return Err(format!(
            "{object_type}/{object_name} requires field {field_name}"
        ));
    };
    let RawValue::String(value) = value else {
        return Err(format!(
            "{object_type}/{object_name} field {field_name} must be a string"
        ));
    };
    if value.trim().is_empty() {
        return Err(format!(
            "{object_type}/{object_name} field {field_name} must be nonblank"
        ));
    }
    Ok(value.trim().to_string())
}

fn window_material_screen_header_count(eio_contents: &str) -> usize {
    eio_contents
        .lines()
        .filter(|line| *line == WINDOW_MATERIAL_SCREEN_HEADER)
        .count()
}

fn energyplus_round_sig_digits_nonnegative(value: f64, precision: usize) -> Option<f64> {
    if value == 0.0 {
        Some(0.0)
    } else {
        super::energyplus_round_sig_digits_positive(value, precision)
    }
}

fn record_first_divergence(first_divergence: &mut Option<String>, value: String) {
    if first_divergence.is_none() {
        *first_divergence = Some(value);
    }
}

fn render_comparison(comparison: &WindowScreenComparison) {
    let generic_indices = indices_by_material_name(&comparison.oracle_material_details, |row| {
        row.material_name.as_str()
    });
    let oracle_generic_screen_rows = comparison
        .definitions
        .iter()
        .map(|definition| {
            generic_indices
                .get(&normalized_material_name(&definition.material_name))
                .map_or(0, Vec::len)
        })
        .sum::<usize>();

    println!("Window Material Screen Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!(
        "  source_optics_scope: energyplus-26.1-calc-screen-transmittance-normal-incidence-and-static-reverse-18x18-quarter-hemisphere"
    );
    println!(
        "  wsc_activation_scope: explicit-ExteriorScreen-shaded-construction-and-fenestration-surface-links-single-control-per-surface"
    );
    let mut occurrence_counts = BTreeMap::<String, usize>::new();
    for occurrence in &comparison.occurrences {
        *occurrence_counts
            .entry(normalized_material_name(&occurrence.material_name))
            .or_default() += 1;
    }
    println!(
        "  shared_material_initialization_evidence: {}",
        comparison.passed && occurrence_counts.values().any(|count| *count > 1)
    );
    println!("  zero_reflectance_source_optics_claim: false");
    println!("  dynamic_screen_optics_claim: false");
    println!("  transmittance_map_claim: false");
    println!("  window_optics_claim: false");
    println!("  window_runtime_claim: false");
    println!("  window_thermal_claim: false");
    println!("  daylighting_claim: false");
    println!("  shading_schedule_behavior_claim: false");
    println!("  surface_runtime_behavior_claim: false");
    println!("  construction_rating_claim: false");
    println!("  broad_idf_declaration_order_claim: false");
    println!("  arbitrary_idf_declaration_order_claim: false");
    println!(
        "  occurrence_order_policy: epjson-canonical-construction-name-then-layer-order-exact"
    );
    println!("  tolerance_mode: {}", comparison.tolerance_mode.label());
    println!(
        "  tolerance_policy: {}",
        comparison.tolerance_mode.policy_label()
    );
    println!(
        "  materials_report_requested: {}",
        comparison.report_requests.materials
    );
    println!(
        "  constructions_report_requested: {}",
        comparison.report_requests.constructions
    );
    println!("  material_objects: {}", comparison.definitions.len());
    println!(
        "  activated_screen_materials: {}",
        comparison.activated_material_names.len()
    );
    println!("  oracle_generic_screen_rows: {oracle_generic_screen_rows}");
    println!(
        "  oracle_material_detail_rows: {}",
        comparison.oracle_material_details.len()
    );
    println!("  screen_occurrences: {}", comparison.occurrences.len());
    println!(
        "  oracle_screen_occurrence_rows: {}",
        comparison.oracle_occurrences.len()
    );
    println!("  screen_header_present: {}", comparison.header_rows == 1);
    println!("  screen_header_rows: {}", comparison.header_rows);

    for (definition_index, definition) in comparison.definitions.iter().enumerate() {
        let matches = generic_indices
            .get(&normalized_material_name(&definition.material_name))
            .map(Vec::as_slice)
            .unwrap_or_default();
        let expected_rows = usize::from(comparison.report_requests.materials);
        let status = matches.len() == expected_rows
            && matches.first().is_none_or(|index| {
                generic_row_matches(definition, &comparison.oracle_material_details[*index])
            });
        println!(
            "  definition: {} material: {} oracle_matches: {} status: {}",
            definition_index + 1,
            definition.material_name,
            matches.len(),
            if status { "pass" } else { "fail" }
        );
    }

    for (occurrence_index, occurrence) in comparison.occurrences.iter().enumerate() {
        let status = comparison
            .oracle_occurrences
            .get(occurrence_index)
            .is_some_and(|row| specialized_row_matches(occurrence, row, comparison.tolerance_mode));
        println!(
            "  occurrence: {} construction: {} layer: {} material: {} status: {}",
            occurrence_index + 1,
            occurrence.construction_name,
            occurrence.layer_number,
            occurrence.material_name,
            if status { "pass" } else { "fail" }
        );
    }

    println!(
        "  first_divergence: {}",
        comparison.first_divergence.as_deref().unwrap_or("none")
    );
    println!(
        "  status: {}",
        if comparison.passed { "pass" } else { "fail" }
    );
}

#[cfg(test)]
#[path = "window_material_screen_tests.rs"]
mod tests;
