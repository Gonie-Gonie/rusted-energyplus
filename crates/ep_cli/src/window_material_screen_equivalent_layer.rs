use std::collections::{BTreeMap, BTreeSet};

use ep_compare::{
    EioError, EioMaterialDetails, EioWindowMaterialScreenEquivalentLayer, Tolerance,
    WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER, parse_eio_material_details,
    parse_eio_window_material_screen_equivalent_layer,
};
use ep_compiler::compile_raw_model;
use ep_model::{
    ConstructionKind, MaterialDefinition, TypedModel, WindowScreenEquivalentLayerMaterial,
};
use ep_raw_model::{RawModel, RawValue, load_epjson_file};

pub(crate) const CASE_ID: &str = "window_material_screen_equivalent_layer_001";
pub(crate) const USAGE: &str = "usage: eplus-rs compare window-material-screen-equivalent-layer <input.epJSON> <eplusout.eio> [--tolerance exact|near]";

const EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE: &str = "Construction:WindowEquivalentLayer";
const COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE: &str = "Construction:ComplexFenestrationState";
const OUTPUT_CONSTRUCTIONS_OBJECT_TYPE: &str = "Output:Constructions";
const EQUIVALENT_LAYER_CONSTRUCTION_MAX_LAYERS: usize = 11;
const AUTO_CALCULATE_EIO_VALUE: f64 = -99_999.0;
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
                "energyplus-26.1-material-details-zero-exact-screen-equivalent-layer-4R-geometry-5R-auto-sentinel-normalized-exact"
            }
            Self::Near => "absolute-0.00001-relative-0.000001-auto-sentinel-exact",
        }
    }

    fn accepts_expected(self, expected: f64, observed: f64) -> bool {
        if expected == AUTO_CALCULATE_EIO_VALUE {
            return observed == expected;
        }
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
struct ScreenEquivalentLayerDefinition {
    material_name: String,
}

#[derive(Clone, Debug)]
struct ScreenEquivalentLayerOccurrence {
    construction_name: String,
    layer_number: usize,
    material_name: String,
    fields: WindowScreenEquivalentLayerMaterial,
}

#[derive(Debug)]
struct ScreenEquivalentLayerComparison {
    definitions: Vec<ScreenEquivalentLayerDefinition>,
    occurrences: Vec<ScreenEquivalentLayerOccurrence>,
    oracle_material_details: Vec<EioMaterialDetails>,
    oracle_occurrences: Vec<EioWindowMaterialScreenEquivalentLayer>,
    report_requests: ConstructionReportRequests,
    header_rows: usize,
    tolerance_mode: NumericToleranceMode,
    passed: bool,
    first_divergence: Option<String>,
}

pub(crate) fn run_compare_window_material_screen_equivalent_layer(args: &[String]) -> i32 {
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
    if screen_equivalent_layer_definitions(&model).is_empty() {
        eprintln!("no WindowMaterial:Screen:EquivalentLayer objects are available for comparison");
        return 1;
    }

    let eio_contents = match std::fs::read_to_string(eio_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("failed to read {}: {error}", eio_path);
            return 1;
        }
    };
    let comparison = match compare_window_material_screen_equivalent_layer(
        &raw_model,
        &model,
        &eio_contents,
        tolerance_mode,
    ) {
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
                "unsupported WindowMaterial:Screen:EquivalentLayer tolerance mode: {value}"
            )),
        },
        _ => Err("unsupported WindowMaterial:Screen:EquivalentLayer compare options".to_string()),
    }
}

fn compare_window_material_screen_equivalent_layer(
    raw_model: &RawModel,
    model: &TypedModel,
    eio_contents: &str,
    tolerance_mode: NumericToleranceMode,
) -> Result<ScreenEquivalentLayerComparison, String> {
    let definitions = screen_equivalent_layer_definitions(model);
    let report_requests = construction_report_requests(raw_model)?;
    let has_window_construction = model
        .constructions
        .iter()
        .any(|construction| construction.kind == ConstructionKind::Fenestration)
        || !raw_model
            .ordered_instances(EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE)
            .map_err(|error| error.to_string())?
            .is_empty()
        || !raw_model
            .ordered_instances(COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE)
            .map_err(|error| error.to_string())?
            .is_empty();
    let expected_header_rows = usize::from(
        report_requests.constructions && !definitions.is_empty() && has_window_construction,
    );
    let occurrences = if report_requests.constructions {
        screen_equivalent_layer_occurrences(raw_model, model)?
    } else {
        Vec::new()
    };

    let oracle_material_details = match parse_eio_material_details(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingMaterialDetails) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let oracle_occurrences = match parse_eio_window_material_screen_equivalent_layer(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingWindowMaterialScreenEquivalentLayerHeader)
            if expected_header_rows == 0 =>
        {
            Vec::new()
        }
        Err(error) => return Err(error.to_string()),
    };
    let header_rows = screen_equivalent_layer_header_count(eio_contents);
    let mut passed = true;
    let mut first_divergence = None;

    if header_rows != expected_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "WindowMaterial:Screen:EquivalentLayer header expected {expected_header_rows} observed {header_rows}"
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
                "screen equivalent-layer occurrences expected {} observed {}",
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
                "unexpected WindowMaterial:Screen:EquivalentLayer row for material {}",
                unexpected.material_name
            ),
        );
    }

    Ok(ScreenEquivalentLayerComparison {
        definitions,
        occurrences,
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
            let Some(value) = report
                .fields
                .iter()
                .find(|(field, _value)| field.0 == field_name)
                .map(|(_field, value)| value)
            else {
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

fn screen_equivalent_layer_definitions(model: &TypedModel) -> Vec<ScreenEquivalentLayerDefinition> {
    model
        .materials
        .iter()
        .filter_map(|material| {
            let MaterialDefinition::WindowScreenEquivalentLayer(_) = material.definition else {
                return None;
            };
            Some(ScreenEquivalentLayerDefinition {
                material_name: material.name.0.clone(),
            })
        })
        .collect()
}

/// Builds fixture-declared expected occurrences from raw
/// `Construction:WindowEquivalentLayer` metadata.
///
/// The construction object is intentionally not projected into the typed
/// model. This bridge provides bounded static-EIO evidence only and does not
/// claim equivalent-layer construction typing or runtime parity.
fn screen_equivalent_layer_occurrences(
    raw_model: &RawModel,
    model: &TypedModel,
) -> Result<Vec<ScreenEquivalentLayerOccurrence>, String> {
    let constructions = raw_model
        .ordered_instances(EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE)
        .map_err(|error| error.to_string())?;
    let mut rows = Vec::new();
    for (construction_name, construction) in constructions {
        let mut missing_optional_layer = false;
        for layer_number in 1..=EQUIVALENT_LAYER_CONSTRUCTION_MAX_LAYERS {
            let field_name = if layer_number == 1 {
                "outside_layer".to_string()
            } else {
                format!("layer_{layer_number}")
            };
            let Some(value) = construction
                .fields
                .iter()
                .find(|(field, _value)| field.0 == field_name)
                .map(|(_field, value)| value)
            else {
                if layer_number == 1 {
                    return Err(format!(
                        "{EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE}/{} requires field outside_layer",
                        construction_name.0
                    ));
                }
                missing_optional_layer = true;
                continue;
            };
            if missing_optional_layer {
                return Err(format!(
                    "{EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE}/{} has noncontiguous layer field {field_name}",
                    construction_name.0
                ));
            }
            let RawValue::String(material_name) = value else {
                return Err(format!(
                    "{EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE}/{} field {field_name} must be a string",
                    construction_name.0
                ));
            };
            if material_name.trim().is_empty() {
                return Err(format!(
                    "{EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE}/{} field {field_name} must be nonblank",
                    construction_name.0
                ));
            }

            let Some(material_id) = model.material_names.resolve(material_name) else {
                // Other equivalent-layer material objects may remain outside
                // the typed subset and do not emit this specialized row.
                continue;
            };
            let material = model
                .materials
                .iter()
                .find(|material| material.id == material_id)
                .ok_or_else(|| {
                    format!(
                        "{EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE}/{} field {field_name} references a missing typed material",
                        construction_name.0
                    )
                })?;
            let MaterialDefinition::WindowScreenEquivalentLayer(fields) = material.definition
            else {
                continue;
            };
            rows.push(ScreenEquivalentLayerOccurrence {
                construction_name: construction_name.0.trim().to_ascii_uppercase(),
                layer_number,
                material_name: material.name.0.clone(),
                fields,
            });
        }
    }
    Ok(rows)
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

fn screen_equivalent_layer_header_count(eio_contents: &str) -> usize {
    eio_contents
        .lines()
        .filter(|line| *line == WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER)
        .count()
}

fn energyplus_round_sig_digits_nonnegative(value: f64, precision: usize) -> Option<f64> {
    if value == 0.0 {
        Some(0.0)
    } else {
        super::energyplus_round_sig_digits_positive(value, precision)
    }
}

fn energyplus_screen_equivalent_layer_eio_value(value: f64, precision: usize) -> Option<f64> {
    if value == AUTO_CALCULATE_EIO_VALUE {
        Some(value)
    } else {
        energyplus_round_sig_digits_nonnegative(value, precision)
    }
}

fn generic_numeric_fields(oracle: &EioMaterialDetails) -> [(&'static str, f64); 8] {
    [
        (
            "thermal_resistance_m2_k_per_w",
            oracle.thermal_resistance_m2_k_per_w,
        ),
        ("thickness_m", oracle.thickness_m),
        ("conductivity_w_per_m_k", oracle.conductivity_w_per_m_k),
        ("density_kg_per_m3", oracle.density_kg_per_m3),
        ("specific_heat_j_per_kg_k", oracle.specific_heat_j_per_kg_k),
        ("thermal_absorptance", oracle.thermal_absorptance),
        ("solar_absorptance", oracle.solar_absorptance),
        ("visible_absorptance", oracle.visible_absorptance),
    ]
}

fn generic_row_matches(
    definition: &ScreenEquivalentLayerDefinition,
    oracle: &EioMaterialDetails,
) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&definition.material_name)
        && oracle.roughness == "MediumRough"
        && generic_numeric_fields(oracle)
            .into_iter()
            .all(|(_field, observed)| observed == 0.0)
}

fn record_generic_divergence(
    first_divergence: &mut Option<String>,
    definition: &ScreenEquivalentLayerDefinition,
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
    if let Some((field, observed)) = generic_numeric_fields(oracle)
        .into_iter()
        .find(|(_field, observed)| *observed != 0.0)
    {
        record_first_divergence(
            first_divergence,
            format!("{prefix} field {field} expected 0 observed {observed:.9}"),
        );
    }
}

/// Builds the expected source-shaped row after EnergyPlus 26.1 serialization.
fn expected_screen_equivalent_layer_row(
    occurrence: &ScreenEquivalentLayerOccurrence,
) -> EioWindowMaterialScreenEquivalentLayer {
    let beam_beam = super::equivalent_layer_eio_auto_value(
        occurrence.fields.front_solar.beam_beam_transmittance,
    );
    let rounded = |value, precision| {
        energyplus_screen_equivalent_layer_eio_value(value, precision).unwrap_or(f64::NAN)
    };
    EioWindowMaterialScreenEquivalentLayer {
        material_name: occurrence.material_name.clone(),
        beam_beam_solar_transmittance: rounded(beam_beam, 4),
        front_beam_diffuse_solar_transmittance: rounded(
            occurrence.fields.front_solar.beam_diffuse_transmittance,
            4,
        ),
        back_beam_diffuse_solar_transmittance: rounded(
            occurrence.fields.back_solar.beam_diffuse_transmittance,
            4,
        ),
        front_beam_diffuse_solar_reflectance: rounded(
            occurrence.fields.front_solar.beam_diffuse_reflectance,
            4,
        ),
        back_beam_diffuse_solar_reflectance: rounded(
            occurrence.fields.back_solar.beam_diffuse_reflectance,
            4,
        ),
        infrared_transmittance: rounded(occurrence.fields.infrared_transmittance, 4),
        front_infrared_emissivity: rounded(occurrence.fields.front_infrared_emissivity, 4),
        back_infrared_emissivity: rounded(occurrence.fields.back_infrared_emissivity, 4),
        wire_spacing_m: rounded(occurrence.fields.wire_spacing_m, 5),
        wire_diameter_m: rounded(occurrence.fields.wire_diameter_m, 5),
    }
}

fn specialized_numeric_fields(
    expected: &EioWindowMaterialScreenEquivalentLayer,
    observed: &EioWindowMaterialScreenEquivalentLayer,
) -> [(&'static str, f64, f64); 10] {
    [
        (
            "beam_beam_solar_transmittance",
            expected.beam_beam_solar_transmittance,
            observed.beam_beam_solar_transmittance,
        ),
        (
            "front_beam_diffuse_solar_transmittance",
            expected.front_beam_diffuse_solar_transmittance,
            observed.front_beam_diffuse_solar_transmittance,
        ),
        (
            "back_beam_diffuse_solar_transmittance",
            expected.back_beam_diffuse_solar_transmittance,
            observed.back_beam_diffuse_solar_transmittance,
        ),
        (
            "front_beam_diffuse_solar_reflectance",
            expected.front_beam_diffuse_solar_reflectance,
            observed.front_beam_diffuse_solar_reflectance,
        ),
        (
            "back_beam_diffuse_solar_reflectance",
            expected.back_beam_diffuse_solar_reflectance,
            observed.back_beam_diffuse_solar_reflectance,
        ),
        (
            "infrared_transmittance",
            expected.infrared_transmittance,
            observed.infrared_transmittance,
        ),
        (
            "front_infrared_emissivity",
            expected.front_infrared_emissivity,
            observed.front_infrared_emissivity,
        ),
        (
            "back_infrared_emissivity",
            expected.back_infrared_emissivity,
            observed.back_infrared_emissivity,
        ),
        (
            "wire_spacing_m",
            expected.wire_spacing_m,
            observed.wire_spacing_m,
        ),
        (
            "wire_diameter_m",
            expected.wire_diameter_m,
            observed.wire_diameter_m,
        ),
    ]
}

fn specialized_row_matches(
    occurrence: &ScreenEquivalentLayerOccurrence,
    oracle: &EioWindowMaterialScreenEquivalentLayer,
    tolerance_mode: NumericToleranceMode,
) -> bool {
    let expected = expected_screen_equivalent_layer_row(occurrence);
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&expected.material_name)
        && specialized_numeric_fields(&expected, oracle)
            .into_iter()
            .all(|(_field, expected, observed)| tolerance_mode.accepts_expected(expected, observed))
}

fn record_specialized_divergence(
    first_divergence: &mut Option<String>,
    occurrence: &ScreenEquivalentLayerOccurrence,
    oracle: &EioWindowMaterialScreenEquivalentLayer,
    tolerance_mode: NumericToleranceMode,
    occurrence_index: usize,
) {
    let expected = expected_screen_equivalent_layer_row(occurrence);
    let prefix = format!(
        "occurrence {} construction {} layer {} material {}",
        occurrence_index + 1,
        occurrence.construction_name,
        occurrence.layer_number,
        occurrence.material_name
    );
    if normalized_material_name(&oracle.material_name)
        != normalized_material_name(&expected.material_name)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field material_name expected {} observed {}",
                expected.material_name, oracle.material_name
            ),
        );
        return;
    }
    if let Some((field, expected, observed)) = specialized_numeric_fields(&expected, oracle)
        .into_iter()
        .find(|(_field, expected, observed)| !tolerance_mode.accepts_expected(*expected, *observed))
    {
        record_first_divergence(
            first_divergence,
            format!("{prefix} field {field} expected {expected:.9} observed {observed:.9}"),
        );
    }
}

fn record_first_divergence(first_divergence: &mut Option<String>, value: String) {
    if first_divergence.is_none() {
        *first_divergence = Some(value);
    }
}

fn render_comparison(comparison: &ScreenEquivalentLayerComparison) {
    let generic_indices = indices_by_material_name(&comparison.oracle_material_details, |row| {
        row.material_name.as_str()
    });
    let oracle_generic_material_rows = comparison
        .definitions
        .iter()
        .map(|definition| {
            generic_indices
                .get(&normalized_material_name(&definition.material_name))
                .map_or(0, Vec::len)
        })
        .sum::<usize>();

    println!("Window Material Screen EquivalentLayer Comparison");
    println!("  case_id: {CASE_ID}");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  window_runtime_claim: false");
    println!("  window_optics_claim: false");
    println!("  window_thermal_claim: false");
    println!("  daylighting_claim: false");
    println!("  equivalent_layer_construction_claim: false");
    println!("  equivalent_layer_construction_typing_claim: false");
    println!("  complex_fenestration_construction_claim: false");
    println!("  fenestration_surface_claim: false");
    println!("  construction_rating_claim: false");
    println!("  visible_input_claim: false");
    println!("  nominal_resistance_claim: false");
    println!(
        "  occurrence_bridge: fixture-declared-raw-construction-window-equivalent-layer-metadata"
    );
    println!(
        "  occurrence_order_policy: epjson-canonical-construction-name-then-layer-order-exact"
    );
    println!("  tolerance_mode: {}", comparison.tolerance_mode.label());
    println!(
        "  tolerance_policy: {}",
        comparison.tolerance_mode.policy_label()
    );
    println!("  material_objects: {}", comparison.definitions.len());
    println!("  oracle_generic_material_rows: {oracle_generic_material_rows}");
    println!(
        "  oracle_material_detail_rows: {}",
        comparison.oracle_material_details.len()
    );
    println!(
        "  screen_equivalent_layer_occurrences: {}",
        comparison.occurrences.len()
    );
    println!(
        "  oracle_screen_equivalent_layer_occurrence_rows: {}",
        comparison.oracle_occurrences.len()
    );
    println!(
        "  screen_equivalent_layer_header_present: {}",
        comparison.header_rows == 1
    );
    println!(
        "  constructions_report_requested: {}",
        comparison.report_requests.constructions
    );
    println!(
        "  materials_report_requested: {}",
        comparison.report_requests.materials
    );
    println!(
        "  screen_equivalent_layer_header_rows: {}",
        comparison.header_rows
    );

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
            "  definition: {} material: {} oracle_matches: {} generic_fixed_zero_fields: 8 status: {}",
            definition_index + 1,
            definition.material_name,
            matches.len(),
            if status { "pass" } else { "fail" },
        );
    }

    for (occurrence_index, occurrence) in comparison.occurrences.iter().enumerate() {
        let oracle = comparison.oracle_occurrences.get(occurrence_index);
        let status = oracle
            .is_some_and(|row| specialized_row_matches(occurrence, row, comparison.tolerance_mode));
        println!(
            "  occurrence: {} construction: {} layer: {} material: {} status: {}",
            occurrence_index + 1,
            occurrence.construction_name,
            occurrence.layer_number,
            occurrence.material_name,
            if status { "pass" } else { "fail" },
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
#[path = "window_material_screen_equivalent_layer_tests.rs"]
mod tests;
