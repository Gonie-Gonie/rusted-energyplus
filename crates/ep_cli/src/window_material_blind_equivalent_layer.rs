use std::collections::{BTreeMap, BTreeSet};

use ep_compare::{
    EioError, EioMaterialDetails, EioWindowMaterialBlindEquivalentLayer, Tolerance,
    WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER, parse_eio_material_details,
    parse_eio_window_material_blind_equivalent_layer,
};
use ep_compiler::compile_raw_model;
use ep_model::{
    ConstructionKind, MaterialDefinition, TypedModel, WindowBlindEquivalentLayerMaterial,
    WindowBlindSlatOrientation,
};
use ep_raw_model::{RawModel, RawValue, load_epjson_file};

pub(crate) const CASE_ID: &str = "window_material_blind_equivalent_layer_001";
pub(crate) const USAGE: &str = "usage: eplus-rs compare window-material-blind-equivalent-layer <input.epJSON> <eplusout.eio> [--tolerance exact|near]";

const EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE: &str = "Construction:WindowEquivalentLayer";
const COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE: &str = "Construction:ComplexFenestrationState";
const OUTPUT_CONSTRUCTIONS_OBJECT_TYPE: &str = "Output:Constructions";
const MATERIAL_DETAILS_HEADER_MARKER: &str = "! <Material Details>";
const MATERIAL_DETAILS_HEADER: &str = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible";
const EQUIVALENT_LAYER_CONSTRUCTION_MAX_LAYERS: usize = 11;
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
                "energyplus-26.1-material-details-zero-exact-blind-equivalent-layer-signed-5R-normalized-exact"
            }
            Self::Near => "absolute-0.00001-relative-0.000001",
        }
    }

    fn accepts_expected(self, expected: f64, observed: f64) -> bool {
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
struct BlindEquivalentLayerDefinition {
    material_name: String,
}

#[derive(Clone, Debug)]
struct BlindEquivalentLayerOccurrence {
    construction_name: String,
    layer_number: usize,
    material_name: String,
    fields: WindowBlindEquivalentLayerMaterial,
}

#[derive(Debug)]
struct BlindEquivalentLayerComparison {
    definitions: Vec<BlindEquivalentLayerDefinition>,
    occurrences: Vec<BlindEquivalentLayerOccurrence>,
    oracle_material_details: Vec<EioMaterialDetails>,
    oracle_occurrences: Vec<EioWindowMaterialBlindEquivalentLayer>,
    report_requests: ConstructionReportRequests,
    material_details_header_rows: usize,
    header_rows: usize,
    tolerance_mode: NumericToleranceMode,
    passed: bool,
    first_divergence: Option<String>,
}

pub(crate) fn run_compare_window_material_blind_equivalent_layer(args: &[String]) -> i32 {
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
    if blind_equivalent_layer_definitions(&model).is_empty() {
        eprintln!("no WindowMaterial:Blind:EquivalentLayer objects are available for comparison");
        return 1;
    }

    let eio_contents = match std::fs::read_to_string(eio_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("failed to read {}: {error}", eio_path);
            return 1;
        }
    };
    let comparison = match compare_window_material_blind_equivalent_layer(
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
                "unsupported WindowMaterial:Blind:EquivalentLayer tolerance mode: {value}"
            )),
        },
        _ => Err("unsupported WindowMaterial:Blind:EquivalentLayer compare options".to_string()),
    }
}

fn compare_window_material_blind_equivalent_layer(
    raw_model: &RawModel,
    model: &TypedModel,
    eio_contents: &str,
    tolerance_mode: NumericToleranceMode,
) -> Result<BlindEquivalentLayerComparison, String> {
    let definitions = blind_equivalent_layer_definitions(model);
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
    let expected_material_details_header_rows = usize::from(report_requests.materials);
    let occurrences = if report_requests.constructions {
        blind_equivalent_layer_occurrences(raw_model, model)?
    } else {
        Vec::new()
    };

    let oracle_material_details = match parse_eio_material_details(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingMaterialDetails) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let oracle_occurrences = match parse_eio_window_material_blind_equivalent_layer(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingWindowMaterialBlindEquivalentLayerHeader)
            if expected_header_rows == 0 =>
        {
            Vec::new()
        }
        Err(error) => return Err(error.to_string()),
    };
    let material_details_header_rows = material_details_header_count(eio_contents)?;
    let header_rows = blind_equivalent_layer_header_count(eio_contents);
    let mut passed = true;
    let mut first_divergence = None;

    if header_rows != expected_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "WindowMaterial:Blind:EquivalentLayer header expected {expected_header_rows} observed {header_rows}"
            ),
        );
    }

    if material_details_header_rows != expected_material_details_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "Material Details header expected {expected_material_details_header_rows} observed {material_details_header_rows}"
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
                "blind equivalent-layer occurrences expected {} observed {}",
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
                "unexpected WindowMaterial:Blind:EquivalentLayer row for material {}",
                unexpected.material_name
            ),
        );
    }

    Ok(BlindEquivalentLayerComparison {
        definitions,
        occurrences,
        oracle_material_details,
        oracle_occurrences,
        report_requests,
        material_details_header_rows,
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

fn blind_equivalent_layer_definitions(model: &TypedModel) -> Vec<BlindEquivalentLayerDefinition> {
    model
        .materials
        .iter()
        .filter_map(|material| {
            let MaterialDefinition::WindowBlindEquivalentLayer(_) = material.definition else {
                return None;
            };
            Some(BlindEquivalentLayerDefinition {
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
fn blind_equivalent_layer_occurrences(
    raw_model: &RawModel,
    model: &TypedModel,
) -> Result<Vec<BlindEquivalentLayerOccurrence>, String> {
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
            let MaterialDefinition::WindowBlindEquivalentLayer(fields) = material.definition else {
                continue;
            };
            rows.push(BlindEquivalentLayerOccurrence {
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

fn blind_equivalent_layer_header_count(eio_contents: &str) -> usize {
    eio_contents
        .lines()
        .filter(|line| *line == WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER)
        .count()
}

fn material_details_header_count(eio_contents: &str) -> Result<usize, String> {
    let mut count = 0;
    let mut first_header_line = None;
    for (line_index, line) in eio_contents.lines().enumerate() {
        let line_number = line_index + 1;
        if line
            .trim_start()
            .starts_with(MATERIAL_DETAILS_HEADER_MARKER)
        {
            if line != MATERIAL_DETAILS_HEADER {
                return Err(format!(
                    "invalid EIO Material Details header at line {line_number}: header must exactly match the EnergyPlus 26.1 source literal: {line}"
                ));
            }
            count += 1;
            first_header_line.get_or_insert(line_number);
        }
        if line.trim().starts_with("Material Details,")
            && first_header_line.is_none_or(|header_line| header_line >= line_number)
        {
            return Err(format!(
                "invalid EIO Material Details row at line {line_number}: row appears before the exact Material Details header: {line}"
            ));
        }
    }
    Ok(count)
}

fn energyplus_round_sig_digits_signed(value: f64, precision: usize) -> Option<f64> {
    if value == 0.0 {
        Some(0.0)
    } else if value > 0.0 {
        super::energyplus_round_sig_digits_positive(value, precision)
    } else {
        super::energyplus_round_sig_digits_positive(-value, precision).map(|rounded| -rounded)
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
    definition: &BlindEquivalentLayerDefinition,
    oracle: &EioMaterialDetails,
) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&definition.material_name)
        && oracle.roughness == "Rough"
        && generic_numeric_fields(oracle)
            .into_iter()
            .all(|(_field, observed)| observed == 0.0)
}

fn record_generic_divergence(
    first_divergence: &mut Option<String>,
    definition: &BlindEquivalentLayerDefinition,
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
    if oracle.roughness != "Rough" {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field roughness expected Rough observed {}",
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

fn canonical_orientation(orientation: WindowBlindSlatOrientation) -> &'static str {
    match orientation {
        WindowBlindSlatOrientation::Horizontal => "Horizontal",
        WindowBlindSlatOrientation::Vertical => "Vertical",
    }
}

/// Builds the expected source-shaped row after EnergyPlus 26.1 serialization.
fn expected_blind_equivalent_layer_row(
    occurrence: &BlindEquivalentLayerOccurrence,
) -> EioWindowMaterialBlindEquivalentLayer {
    let rounded = |value| energyplus_round_sig_digits_signed(value, 5).unwrap_or(f64::NAN);
    EioWindowMaterialBlindEquivalentLayer {
        material_name: occurrence.material_name.clone(),
        slat_orientation: canonical_orientation(occurrence.fields.slat_orientation).to_string(),
        slat_width_m: rounded(occurrence.fields.slat_width_m),
        slat_separation_m: rounded(occurrence.fields.slat_separation_m),
        slat_crown_m: rounded(occurrence.fields.slat_crown_m),
        slat_angle_deg: rounded(occurrence.fields.slat_angle_deg),
        front_beam_diffuse_solar_transmittance: rounded(
            occurrence.fields.front_solar.beam_diffuse_transmittance,
        ),
        back_beam_diffuse_solar_transmittance: rounded(
            occurrence.fields.back_solar.beam_diffuse_transmittance,
        ),
        front_beam_diffuse_solar_reflectance: rounded(
            occurrence.fields.front_solar.beam_diffuse_reflectance,
        ),
        back_beam_diffuse_solar_reflectance: rounded(
            occurrence.fields.back_solar.beam_diffuse_reflectance,
        ),
        diffuse_diffuse_solar_transmittance: rounded(
            occurrence.fields.solar_diffuse_diffuse.transmittance,
        ),
        front_diffuse_diffuse_solar_reflectance: rounded(
            occurrence.fields.solar_diffuse_diffuse.front_reflectance,
        ),
        back_diffuse_diffuse_solar_reflectance: rounded(
            occurrence.fields.solar_diffuse_diffuse.back_reflectance,
        ),
        infrared_transmittance: rounded(occurrence.fields.infrared_transmittance),
        front_infrared_emissivity: rounded(occurrence.fields.front_infrared_emissivity),
        back_infrared_emissivity: rounded(occurrence.fields.back_infrared_emissivity),
    }
}

fn specialized_numeric_fields(
    expected: &EioWindowMaterialBlindEquivalentLayer,
    observed: &EioWindowMaterialBlindEquivalentLayer,
) -> [(&'static str, f64, f64); 14] {
    [
        ("slat_width_m", expected.slat_width_m, observed.slat_width_m),
        (
            "slat_separation_m",
            expected.slat_separation_m,
            observed.slat_separation_m,
        ),
        ("slat_crown_m", expected.slat_crown_m, observed.slat_crown_m),
        (
            "slat_angle_deg",
            expected.slat_angle_deg,
            observed.slat_angle_deg,
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
            "diffuse_diffuse_solar_transmittance",
            expected.diffuse_diffuse_solar_transmittance,
            observed.diffuse_diffuse_solar_transmittance,
        ),
        (
            "front_diffuse_diffuse_solar_reflectance",
            expected.front_diffuse_diffuse_solar_reflectance,
            observed.front_diffuse_diffuse_solar_reflectance,
        ),
        (
            "back_diffuse_diffuse_solar_reflectance",
            expected.back_diffuse_diffuse_solar_reflectance,
            observed.back_diffuse_diffuse_solar_reflectance,
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
    ]
}

fn specialized_row_matches(
    occurrence: &BlindEquivalentLayerOccurrence,
    oracle: &EioWindowMaterialBlindEquivalentLayer,
    tolerance_mode: NumericToleranceMode,
) -> bool {
    let expected = expected_blind_equivalent_layer_row(occurrence);
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&expected.material_name)
        && oracle.slat_orientation == expected.slat_orientation
        && specialized_numeric_fields(&expected, oracle)
            .into_iter()
            .all(|(_field, expected, observed)| tolerance_mode.accepts_expected(expected, observed))
}

fn record_specialized_divergence(
    first_divergence: &mut Option<String>,
    occurrence: &BlindEquivalentLayerOccurrence,
    oracle: &EioWindowMaterialBlindEquivalentLayer,
    tolerance_mode: NumericToleranceMode,
    occurrence_index: usize,
) {
    let expected = expected_blind_equivalent_layer_row(occurrence);
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
    if oracle.slat_orientation != expected.slat_orientation {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field slat_orientation expected {} observed {}",
                expected.slat_orientation, oracle.slat_orientation
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

fn render_comparison(comparison: &BlindEquivalentLayerComparison) {
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

    println!("Window Material Blind EquivalentLayer Comparison");
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
    println!("  slat_angle_control_claim: false");
    println!("  nominal_resistance_claim: false");
    println!("  source_row_trailing_newline: false");
    println!("  source_row_concatenation_contract: energyplus-26.1-next-eio-record-direct");
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
        "  material_details_header_rows: {}",
        comparison.material_details_header_rows
    );
    println!(
        "  blind_equivalent_layer_occurrences: {}",
        comparison.occurrences.len()
    );
    println!(
        "  oracle_blind_equivalent_layer_occurrence_rows: {}",
        comparison.oracle_occurrences.len()
    );
    println!(
        "  blind_equivalent_layer_header_present: {}",
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
        "  blind_equivalent_layer_header_rows: {}",
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
#[path = "window_material_blind_equivalent_layer_tests.rs"]
mod tests;
