use std::collections::{BTreeMap, BTreeSet};

use ep_compare::{
    EioError, EioMaterialDetails, EioWindowMaterialShadeEquivalentLayer, Tolerance,
    WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER, parse_eio_material_details,
    parse_eio_window_material_shade_equivalent_layer,
};
use ep_compiler::compile_raw_model;
use ep_model::{
    ConstructionKind, MaterialDefinition, TypedModel, WindowShadeEquivalentLayerMaterial,
};
use ep_raw_model::{RawModel, RawValue, load_epjson_file};

pub(crate) const USAGE: &str = "usage: eplus-rs compare window-material-shade-equivalent-layer <input.epJSON> <eplusout.eio> [--tolerance exact|near]";

const EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE: &str = "Construction:WindowEquivalentLayer";
const COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE: &str = "Construction:ComplexFenestrationState";
const OUTPUT_CONSTRUCTIONS_OBJECT_TYPE: &str = "Output:Constructions";
const EQUIVALENT_LAYER_CONSTRUCTION_MAX_LAYERS: usize = 11;
const NEAR_TOLERANCE: Tolerance = Tolerance {
    absolute: 0.00001,
    relative: 0.000001,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
                "energyplus-26.1-material-details-zero-exact-shade-equivalent-layer-4R-normalized-exact"
            }
            Self::Near => "absolute-0.00001-relative-0.000001",
        }
    }

    fn accepts_specialized(self, expected_input: f64, observed: f64) -> bool {
        let Some(expected_eio) = energyplus_round_sig_digits_nonnegative(expected_input, 4) else {
            return false;
        };
        match self {
            Self::Exact => expected_eio == observed,
            Self::Near => NEAR_TOLERANCE.accepts(expected_eio, observed),
        }
    }
}

#[derive(Clone, Debug)]
struct ShadeEquivalentLayerDefinition {
    material_name: String,
}

#[derive(Clone, Debug)]
struct ShadeEquivalentLayerOccurrence {
    construction_name: String,
    layer_number: usize,
    material_name: String,
    fields: WindowShadeEquivalentLayerMaterial,
}

#[derive(Debug)]
struct ShadeEquivalentLayerComparison {
    definitions: Vec<ShadeEquivalentLayerDefinition>,
    occurrences: Vec<ShadeEquivalentLayerOccurrence>,
    oracle_material_details: Vec<EioMaterialDetails>,
    oracle_occurrences: Vec<EioWindowMaterialShadeEquivalentLayer>,
    constructions_report_requested: bool,
    header_rows: usize,
    tolerance_mode: NumericToleranceMode,
    passed: bool,
    first_divergence: Option<String>,
}

pub(crate) fn run_compare_window_material_shade_equivalent_layer(args: &[String]) -> i32 {
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
    if shade_equivalent_layer_definitions(&model).is_empty() {
        eprintln!("no WindowMaterial:Shade:EquivalentLayer objects are available for comparison");
        return 1;
    }

    let eio_contents = match std::fs::read_to_string(eio_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("failed to read {}: {error}", eio_path);
            return 1;
        }
    };
    let comparison = match compare_window_material_shade_equivalent_layer(
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
                "unsupported WindowMaterial:Shade:EquivalentLayer tolerance mode: {value}"
            )),
        },
        _ => Err("unsupported WindowMaterial:Shade:EquivalentLayer compare options".to_string()),
    }
}

fn compare_window_material_shade_equivalent_layer(
    raw_model: &RawModel,
    model: &TypedModel,
    eio_contents: &str,
    tolerance_mode: NumericToleranceMode,
) -> Result<ShadeEquivalentLayerComparison, String> {
    let definitions = shade_equivalent_layer_definitions(model);
    let occurrences = shade_equivalent_layer_occurrences(raw_model, model)?;
    let oracle_material_details = match parse_eio_material_details(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingMaterialDetails) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let oracle_occurrences = match parse_eio_window_material_shade_equivalent_layer(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingWindowMaterialShadeEquivalentLayer) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let header_rows = shade_equivalent_layer_header_count(eio_contents);
    let constructions_report_requested = constructions_report_requested(raw_model)?;
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
        !definitions.is_empty() && constructions_report_requested && has_window_construction,
    );
    let mut passed = true;
    let mut first_divergence = None;

    if header_rows != expected_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "WindowMaterial:Shade:EquivalentLayer header expected {expected_header_rows} observed {header_rows}"
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
        if matching_indices.len() != 1 {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!(
                    "material {} expected exactly one Material Details row observed {}",
                    definition.material_name,
                    matching_indices.len()
                ),
            );
            continue;
        }

        let oracle = &oracle_material_details[matching_indices[0]];
        if !generic_row_matches(definition, oracle) {
            passed = false;
            record_generic_divergence(&mut first_divergence, definition, oracle);
        }
    }

    if occurrences.len() != oracle_occurrences.len() {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "shade equivalent-layer occurrences expected {} observed {}",
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
                "unexpected WindowMaterial:Shade:EquivalentLayer row for material {}",
                unexpected.material_name
            ),
        );
    }

    Ok(ShadeEquivalentLayerComparison {
        definitions,
        occurrences,
        oracle_material_details,
        oracle_occurrences,
        constructions_report_requested,
        header_rows,
        tolerance_mode,
        passed,
        first_divergence,
    })
}

fn constructions_report_requested(raw_model: &RawModel) -> Result<bool, String> {
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
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn shade_equivalent_layer_definitions(model: &TypedModel) -> Vec<ShadeEquivalentLayerDefinition> {
    model
        .materials
        .iter()
        .filter_map(|material| {
            let MaterialDefinition::WindowShadeEquivalentLayer(_) = material.definition else {
                return None;
            };
            Some(ShadeEquivalentLayerDefinition {
                material_name: material.name.0.clone(),
            })
        })
        .collect()
}

fn shade_equivalent_layer_occurrences(
    raw_model: &RawModel,
    model: &TypedModel,
) -> Result<Vec<ShadeEquivalentLayerOccurrence>, String> {
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
                // Other equivalent-layer material objects remain outside the
                // current typed subset and do not emit this specialized row.
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
            let MaterialDefinition::WindowShadeEquivalentLayer(fields) = material.definition else {
                continue;
            };
            rows.push(ShadeEquivalentLayerOccurrence {
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

fn shade_equivalent_layer_header_count(eio_contents: &str) -> usize {
    eio_contents
        .lines()
        .filter(|line| *line == WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER)
        .count()
}

fn energyplus_round_sig_digits_nonnegative(value: f64, precision: usize) -> Option<f64> {
    if value == 0.0 {
        Some(0.0)
    } else {
        super::energyplus_round_sig_digits_positive(value, precision)
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
    definition: &ShadeEquivalentLayerDefinition,
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
    definition: &ShadeEquivalentLayerDefinition,
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

fn specialized_numeric_fields(
    occurrence: &ShadeEquivalentLayerOccurrence,
    oracle: &EioWindowMaterialShadeEquivalentLayer,
) -> [(&'static str, f64, f64); 9] {
    [
        (
            "front_beam_beam_solar_transmittance",
            occurrence.fields.front_solar.beam_beam_transmittance,
            oracle.front_beam_beam_solar_transmittance,
        ),
        (
            "back_beam_beam_solar_transmittance",
            occurrence.fields.back_solar.beam_beam_transmittance,
            oracle.back_beam_beam_solar_transmittance,
        ),
        (
            "front_beam_diffuse_solar_transmittance",
            occurrence.fields.front_solar.beam_diffuse_transmittance,
            oracle.front_beam_diffuse_solar_transmittance,
        ),
        (
            "back_beam_diffuse_solar_transmittance",
            occurrence.fields.back_solar.beam_diffuse_transmittance,
            oracle.back_beam_diffuse_solar_transmittance,
        ),
        (
            "front_beam_diffuse_solar_reflectance",
            occurrence.fields.front_solar.beam_diffuse_reflectance,
            oracle.front_beam_diffuse_solar_reflectance,
        ),
        (
            "back_beam_diffuse_solar_reflectance",
            occurrence.fields.back_solar.beam_diffuse_reflectance,
            oracle.back_beam_diffuse_solar_reflectance,
        ),
        (
            "infrared_transmittance",
            occurrence.fields.infrared_transmittance,
            oracle.infrared_transmittance,
        ),
        (
            "front_infrared_emissivity",
            occurrence.fields.front_infrared_emissivity,
            oracle.front_infrared_emissivity,
        ),
        (
            "back_infrared_emissivity",
            occurrence.fields.back_infrared_emissivity,
            oracle.back_infrared_emissivity,
        ),
    ]
}

fn specialized_row_matches(
    occurrence: &ShadeEquivalentLayerOccurrence,
    oracle: &EioWindowMaterialShadeEquivalentLayer,
    tolerance_mode: NumericToleranceMode,
) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&occurrence.material_name)
        && specialized_numeric_fields(occurrence, oracle)
            .into_iter()
            .all(|(_field, expected, observed)| {
                tolerance_mode.accepts_specialized(expected, observed)
            })
}

fn record_specialized_divergence(
    first_divergence: &mut Option<String>,
    occurrence: &ShadeEquivalentLayerOccurrence,
    oracle: &EioWindowMaterialShadeEquivalentLayer,
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
    if let Some((field, input, observed)) = specialized_numeric_fields(occurrence, oracle)
        .into_iter()
        .find(|(_field, input, observed)| !tolerance_mode.accepts_specialized(*input, *observed))
    {
        let expected = energyplus_round_sig_digits_nonnegative(input, 4).unwrap_or(f64::NAN);
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

fn render_comparison(comparison: &ShadeEquivalentLayerComparison) {
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

    println!("Window Material Shade EquivalentLayer Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  window_runtime_claim: false");
    println!("  window_optics_claim: false");
    println!("  window_thermal_claim: false");
    println!("  daylighting_claim: false");
    println!("  equivalent_layer_construction_claim: false");
    println!("  complex_fenestration_construction_claim: false");
    println!("  fenestration_surface_claim: false");
    println!("  construction_rating_claim: false");
    println!("  visible_input_claim: false");
    println!("  nominal_resistance_claim: false");
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
    println!("  material_objects: {}", comparison.definitions.len());
    println!("  oracle_generic_material_rows: {oracle_generic_material_rows}");
    println!(
        "  oracle_material_detail_rows: {}",
        comparison.oracle_material_details.len()
    );
    println!(
        "  shade_equivalent_layer_occurrences: {}",
        comparison.occurrences.len()
    );
    println!(
        "  oracle_shade_equivalent_layer_occurrence_rows: {}",
        comparison.oracle_occurrences.len()
    );
    println!(
        "  shade_equivalent_layer_header_present: {}",
        comparison.header_rows == 1
    );
    println!(
        "  constructions_report_requested: {}",
        comparison.constructions_report_requested
    );
    println!(
        "  shade_equivalent_layer_header_rows: {}",
        comparison.header_rows
    );

    for (definition_index, definition) in comparison.definitions.iter().enumerate() {
        let matching_indices = generic_indices
            .get(&normalized_material_name(&definition.material_name))
            .map(Vec::as_slice)
            .unwrap_or_default();
        let status = matching_indices
            .first()
            .filter(|_| matching_indices.len() == 1)
            .is_some_and(|index| {
                generic_row_matches(definition, &comparison.oracle_material_details[*index])
            });
        println!(
            "  definition: {} material: {} oracle_matches: {} generic_fixed_zero_fields: 8 status: {}",
            definition_index + 1,
            definition.material_name,
            matching_indices.len(),
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
#[path = "window_material_shade_equivalent_layer_tests.rs"]
mod tests;
