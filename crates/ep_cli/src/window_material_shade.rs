use std::collections::{BTreeMap, BTreeSet};

use ep_compare::{
    EioError, EioMaterialDetails, EioWindowMaterialShade, WINDOW_MATERIAL_SHADE_HEADER,
    parse_eio_material_details, parse_eio_window_material_shade,
};
use ep_compiler::compile_raw_model;
use ep_model::{ConstructionKind, MaterialDefinition, TypedModel, WindowShadeMaterial};
use ep_raw_model::load_epjson_file;

pub(crate) const USAGE: &str =
    "usage: eplus-rs compare window-material-shade <input.epJSON> <eplusout.eio>";

#[derive(Clone, Debug)]
struct WindowShadeDefinition {
    material_name: String,
    fields: WindowShadeMaterial,
}

#[derive(Clone, Debug)]
struct WindowShadeOccurrence {
    construction_name: String,
    layer_number: usize,
    material_name: String,
    fields: WindowShadeMaterial,
}

#[derive(Debug)]
struct WindowShadeComparison {
    definitions: Vec<WindowShadeDefinition>,
    occurrences: Vec<WindowShadeOccurrence>,
    oracle_material_details: Vec<EioMaterialDetails>,
    oracle_shade_rows: Vec<EioWindowMaterialShade>,
    shade_header_rows: usize,
    passed: bool,
    first_divergence: Option<String>,
}

pub(crate) fn run_compare_window_material_shade(args: &[String]) -> i32 {
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
    if window_shade_definitions(&model).is_empty() {
        eprintln!("no WindowMaterial:Shade objects are available for comparison");
        return 1;
    }

    let eio_contents = match std::fs::read_to_string(eio_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("failed to read {}: {error}", eio_path);
            return 1;
        }
    };
    let comparison = match compare_window_material_shade(&model, &eio_contents) {
        Ok(comparison) => comparison,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    render_window_material_shade_comparison(&comparison);
    if comparison.passed { 0 } else { 1 }
}

fn compare_window_material_shade(
    model: &TypedModel,
    eio_contents: &str,
) -> Result<WindowShadeComparison, String> {
    let definitions = window_shade_definitions(model);
    let occurrences = window_shade_occurrences(model)?;
    let oracle_material_details = match parse_eio_material_details(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingMaterialDetails) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let oracle_shade_rows = match parse_eio_window_material_shade(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingWindowMaterialShade) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let shade_header_rows = window_material_shade_header_count(eio_contents);
    let expected_shade_header_rows = usize::from(
        model
            .constructions
            .iter()
            .any(|construction| construction.kind == ConstructionKind::Fenestration),
    );
    let mut passed = true;
    let mut first_divergence = None;

    if shade_header_rows != expected_shade_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "WindowMaterial:Shade header expected {expected_shade_header_rows} observed {shade_header_rows}"
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

        let oracle_row = &oracle_material_details[matching_indices[0]];
        if !window_shade_generic_row_matches(definition, oracle_row) {
            passed = false;
            record_window_shade_generic_divergence(&mut first_divergence, definition, oracle_row);
        }
    }

    let expected_occurrence_indices =
        indices_by_material_name(&occurrences, |row| row.material_name.as_str());
    let oracle_occurrence_indices =
        indices_by_material_name(&oracle_shade_rows, |row| row.material_name.as_str());
    for definition in &definitions {
        let normalized_name = normalized_material_name(&definition.material_name);
        let expected_indices = expected_occurrence_indices
            .get(&normalized_name)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let oracle_indices = oracle_occurrence_indices
            .get(&normalized_name)
            .map(Vec::as_slice)
            .unwrap_or_default();
        if expected_indices.len() != oracle_indices.len() {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!(
                    "material {} shade occurrences expected {} observed {}",
                    definition.material_name,
                    expected_indices.len(),
                    oracle_indices.len()
                ),
            );
        }
        for (expected_index, oracle_index) in expected_indices.iter().zip(oracle_indices) {
            let expected = &occurrences[*expected_index];
            let oracle = &oracle_shade_rows[*oracle_index];
            if !window_shade_occurrence_row_matches(expected, oracle) {
                passed = false;
                record_window_shade_occurrence_divergence(&mut first_divergence, expected, oracle);
            }
        }
    }

    let definition_names = definitions
        .iter()
        .map(|definition| normalized_material_name(&definition.material_name))
        .collect::<BTreeSet<_>>();
    if let Some(unexpected) = oracle_shade_rows
        .iter()
        .find(|row| !definition_names.contains(&normalized_material_name(&row.material_name)))
    {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "unexpected WindowMaterial:Shade row for material {}",
                unexpected.material_name
            ),
        );
    }

    Ok(WindowShadeComparison {
        definitions,
        occurrences,
        oracle_material_details,
        oracle_shade_rows,
        shade_header_rows,
        passed,
        first_divergence,
    })
}

fn window_shade_definitions(model: &TypedModel) -> Vec<WindowShadeDefinition> {
    model
        .materials
        .iter()
        .filter_map(|material| {
            let MaterialDefinition::WindowShade(fields) = material.definition else {
                return None;
            };
            Some(WindowShadeDefinition {
                material_name: material.name.0.clone(),
                fields,
            })
        })
        .collect()
}

fn window_shade_occurrences(model: &TypedModel) -> Result<Vec<WindowShadeOccurrence>, String> {
    let mut rows = Vec::new();
    for construction in &model.constructions {
        let layer_ids = if construction.layers.is_empty() {
            std::slice::from_ref(&construction.outside_layer)
        } else {
            construction.layers.as_slice()
        };
        for (layer_index, layer_id) in layer_ids.iter().enumerate() {
            let material = model
                .materials
                .iter()
                .find(|material| material.id == *layer_id)
                .ok_or_else(|| {
                    format!(
                        "construction {} references missing material layer {}",
                        construction.name.0,
                        layer_index + 1
                    )
                })?;
            let MaterialDefinition::WindowShade(fields) = material.definition else {
                continue;
            };
            rows.push(WindowShadeOccurrence {
                construction_name: construction.name.0.clone(),
                layer_number: layer_index + 1,
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

fn window_material_shade_header_count(eio_contents: &str) -> usize {
    eio_contents
        .lines()
        .filter(|line| *line == WINDOW_MATERIAL_SHADE_HEADER)
        .count()
}

fn energyplus_round_sig_digits_nonnegative(value: f64, precision: usize) -> Option<f64> {
    if value == 0.0 {
        Some(0.0)
    } else {
        super::energyplus_round_sig_digits_positive(value, precision)
    }
}

fn window_shade_generic_numeric_fields(
    definition: &WindowShadeDefinition,
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
            energyplus_round_sig_digits_nonnegative(definition.fields.thickness_m, 4)
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
            energyplus_round_sig_digits_nonnegative(
                definition.fields.infrared_hemispherical_emissivity,
                4,
            )
            .unwrap_or(f64::NAN),
            oracle.thermal_absorptance,
        ),
        (
            "solar_absorptance",
            energyplus_round_sig_digits_nonnegative(definition.fields.solar_absorptance, 4)
                .unwrap_or(f64::NAN),
            oracle.solar_absorptance,
        ),
        ("visible_absorptance", 0.0, oracle.visible_absorptance),
    ]
}

fn window_shade_generic_row_matches(
    definition: &WindowShadeDefinition,
    oracle: &EioMaterialDetails,
) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&definition.material_name)
        && oracle.roughness == "MediumRough"
        && window_shade_generic_numeric_fields(definition, oracle)
            .into_iter()
            .all(|(_field, expected, observed)| expected == observed)
}

fn record_window_shade_generic_divergence(
    first_divergence: &mut Option<String>,
    definition: &WindowShadeDefinition,
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
    if let Some((field, expected, observed)) =
        window_shade_generic_numeric_fields(definition, oracle)
            .into_iter()
            .find(|(_field, expected, observed)| expected != observed)
    {
        record_first_divergence(
            first_divergence,
            format!("{prefix} field {field} expected {expected:.9} observed {observed:.9}"),
        );
    }
}

fn window_shade_occurrence_numeric_fields(
    occurrence: &WindowShadeOccurrence,
    oracle: &EioWindowMaterialShade,
) -> [(&'static str, f64, f64); 6] {
    [
        (
            "thickness_m",
            energyplus_round_sig_digits_nonnegative(occurrence.fields.thickness_m, 3)
                .unwrap_or(f64::NAN),
            oracle.thickness_m,
        ),
        (
            "conductivity_w_per_m_k",
            energyplus_round_sig_digits_nonnegative(occurrence.fields.conductivity_w_per_m_k, 3)
                .unwrap_or(f64::NAN),
            oracle.conductivity_w_per_m_k,
        ),
        (
            "thermal_absorptance",
            energyplus_round_sig_digits_nonnegative(
                occurrence.fields.infrared_hemispherical_emissivity,
                3,
            )
            .unwrap_or(f64::NAN),
            oracle.thermal_absorptance,
        ),
        (
            "solar_transmittance",
            energyplus_round_sig_digits_nonnegative(occurrence.fields.solar_transmittance, 3)
                .unwrap_or(f64::NAN),
            oracle.solar_transmittance,
        ),
        (
            "visible_transmittance",
            energyplus_round_sig_digits_nonnegative(occurrence.fields.visible_transmittance, 3)
                .unwrap_or(f64::NAN),
            oracle.visible_transmittance,
        ),
        (
            "solar_reflectance",
            energyplus_round_sig_digits_nonnegative(occurrence.fields.solar_reflectance, 3)
                .unwrap_or(f64::NAN),
            oracle.solar_reflectance,
        ),
    ]
}

fn window_shade_occurrence_row_matches(
    occurrence: &WindowShadeOccurrence,
    oracle: &EioWindowMaterialShade,
) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&occurrence.material_name)
        && window_shade_occurrence_numeric_fields(occurrence, oracle)
            .into_iter()
            .all(|(_field, expected, observed)| expected == observed)
}

fn record_window_shade_occurrence_divergence(
    first_divergence: &mut Option<String>,
    occurrence: &WindowShadeOccurrence,
    oracle: &EioWindowMaterialShade,
) {
    let prefix = format!(
        "construction {} layer {} material {}",
        occurrence.construction_name, occurrence.layer_number, occurrence.material_name
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
    if let Some((field, expected, observed)) =
        window_shade_occurrence_numeric_fields(occurrence, oracle)
            .into_iter()
            .find(|(_field, expected, observed)| expected != observed)
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

fn render_window_material_shade_comparison(comparison: &WindowShadeComparison) {
    let generic_indices = indices_by_material_name(&comparison.oracle_material_details, |row| {
        row.material_name.as_str()
    });
    let occurrence_indices = indices_by_material_name(&comparison.oracle_shade_rows, |row| {
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

    println!("Window Material Shade Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  window_runtime_claim: false");
    println!("  window_optics_claim: false");
    println!("  window_thermal_claim: false");
    println!("  daylighting_claim: false");
    println!("  shading_control_claim: false");
    println!("  fenestration_surface_claim: false");
    println!("  construction_rating_claim: false");
    println!("  shade_to_glass_distance_claim: false");
    println!("  opening_multiplier_claim: false");
    println!("  airflow_permeability_claim: false");
    println!("  visible_reflectance_claim: false");
    println!("  infrared_transmittance_claim: false");
    println!("  nominal_resistance_claim: false");
    println!("  broad_idf_declaration_order_claim: false");
    println!("  arbitrary_idf_declaration_order_claim: false");
    println!(
        "  tolerance_policy: energyplus-26.1-material-details-4R-shade-occurrence-3R-normalized-exact"
    );
    println!("  material_objects: {}", comparison.definitions.len());
    println!("  oracle_generic_material_rows: {oracle_generic_material_rows}");
    println!(
        "  oracle_material_detail_rows: {}",
        comparison.oracle_material_details.len()
    );
    println!("  shade_occurrences: {}", comparison.occurrences.len());
    println!(
        "  oracle_shade_occurrence_rows: {}",
        comparison.oracle_shade_rows.len()
    );
    println!(
        "  shade_header_present: {}",
        comparison.shade_header_rows == 1
    );
    println!("  shade_header_rows: {}", comparison.shade_header_rows);

    for (definition_index, definition) in comparison.definitions.iter().enumerate() {
        let matching_indices = generic_indices
            .get(&normalized_material_name(&definition.material_name))
            .map(Vec::as_slice)
            .unwrap_or_default();
        if matching_indices.len() != 1 {
            println!(
                "  definition: {} material: {} oracle_matches: {} status: fail",
                definition_index + 1,
                definition.material_name,
                matching_indices.len()
            );
            continue;
        }
        let oracle = &comparison.oracle_material_details[matching_indices[0]];
        let status = if window_shade_generic_row_matches(definition, oracle) {
            "pass"
        } else {
            "fail"
        };
        println!(
            "  definition: {} material: {}/{} roughness: {}/MediumRough thickness_m: {:.9}/{:.9} conductivity: {:.9}/{:.9} thermal_resistance: {:.9}/0 density: {:.9}/0 specific_heat: {:.9}/0 absorptances_thermal_solar_visible: {:.9},{:.9},{:.9}/{:.9},{:.9},0 status: {}",
            definition_index + 1,
            oracle.material_name,
            definition.material_name,
            oracle.roughness,
            oracle.thickness_m,
            energyplus_round_sig_digits_nonnegative(definition.fields.thickness_m, 4)
                .unwrap_or(f64::NAN),
            oracle.conductivity_w_per_m_k,
            energyplus_round_sig_digits_nonnegative(definition.fields.conductivity_w_per_m_k, 3,)
                .unwrap_or(f64::NAN),
            oracle.thermal_resistance_m2_k_per_w,
            oracle.density_kg_per_m3,
            oracle.specific_heat_j_per_kg_k,
            oracle.thermal_absorptance,
            oracle.solar_absorptance,
            oracle.visible_absorptance,
            energyplus_round_sig_digits_nonnegative(
                definition.fields.infrared_hemispherical_emissivity,
                4,
            )
            .unwrap_or(f64::NAN),
            energyplus_round_sig_digits_nonnegative(definition.fields.solar_absorptance, 4)
                .unwrap_or(f64::NAN),
            status,
        );
    }

    let mut emitted_by_name = BTreeMap::<String, usize>::new();
    for (occurrence_index, occurrence) in comparison.occurrences.iter().enumerate() {
        let normalized_name = normalized_material_name(&occurrence.material_name);
        let emitted = emitted_by_name.entry(normalized_name.clone()).or_default();
        let oracle = occurrence_indices
            .get(&normalized_name)
            .and_then(|indices| indices.get(*emitted))
            .map(|index| &comparison.oracle_shade_rows[*index]);
        *emitted += 1;
        let Some(oracle) = oracle else {
            println!(
                "  occurrence: {} construction: {} layer: {} material: {} status: fail",
                occurrence_index + 1,
                occurrence.construction_name,
                occurrence.layer_number,
                occurrence.material_name,
            );
            continue;
        };
        let status = if window_shade_occurrence_row_matches(occurrence, oracle) {
            "pass"
        } else {
            "fail"
        };
        println!(
            "  occurrence: {} construction: {} layer: {} material: {}/{} thickness_m: {:.9}/{:.9} conductivity: {:.9}/{:.9} thermal_absorptance: {:.9}/{:.9} solar_transmittance: {:.9}/{:.9} visible_transmittance: {:.9}/{:.9} solar_reflectance: {:.9}/{:.9} status: {}",
            occurrence_index + 1,
            occurrence.construction_name,
            occurrence.layer_number,
            oracle.material_name,
            occurrence.material_name,
            oracle.thickness_m,
            energyplus_round_sig_digits_nonnegative(occurrence.fields.thickness_m, 3)
                .unwrap_or(f64::NAN),
            oracle.conductivity_w_per_m_k,
            energyplus_round_sig_digits_nonnegative(occurrence.fields.conductivity_w_per_m_k, 3,)
                .unwrap_or(f64::NAN),
            oracle.thermal_absorptance,
            energyplus_round_sig_digits_nonnegative(
                occurrence.fields.infrared_hemispherical_emissivity,
                3,
            )
            .unwrap_or(f64::NAN),
            oracle.solar_transmittance,
            energyplus_round_sig_digits_nonnegative(occurrence.fields.solar_transmittance, 3,)
                .unwrap_or(f64::NAN),
            oracle.visible_transmittance,
            energyplus_round_sig_digits_nonnegative(occurrence.fields.visible_transmittance, 3,)
                .unwrap_or(f64::NAN),
            oracle.solar_reflectance,
            energyplus_round_sig_digits_nonnegative(occurrence.fields.solar_reflectance, 3,)
                .unwrap_or(f64::NAN),
            status,
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
#[path = "window_material_shade_tests.rs"]
mod tests;
