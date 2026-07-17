use std::collections::BTreeMap;

use ep_compare::{EioError, EioMaterialDetails, Tolerance, parse_eio_material_details};
use ep_compiler::compile_raw_model;
use ep_model::{
    MaterialDefinition, MaterialSurfaceRoughness, TypedModel, WindowSimpleGlazingMaterial,
};
use ep_raw_model::{RawModel, RawObject, RawValue, load_epjson_file};

pub(crate) const CASE_ID: &str = "window_material_simple_glazing_system_001";
pub(crate) const USAGE: &str = "usage: eplus-rs compare window-material-simple-glazing-system <input.epJSON> <eplusout.eio> [--tolerance exact|near]";

const OUTPUT_CONSTRUCTIONS_OBJECT_TYPE: &str = "Output:Constructions";
const MATERIAL_DETAILS_HEADER_MARKER: &str = "! <Material Details>";
const SIMPLE_GLAZING_ROUGHNESS: &str = "VerySmooth";
const MATERIAL_DETAILS_HEADER: &str = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible";
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
            Self::Exact => "energyplus-26.1-simple-glazing-material-details-4R-3R-normalized-exact",
            Self::Near => "absolute-0.00001-relative-0.000001-against-source-rounded-expected",
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
struct SimpleGlazingDefinition {
    material_name: String,
    fields: WindowSimpleGlazingMaterial,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct MaterialDetailsTableShape {
    candidate_header_rows: usize,
    exact_header_rows: usize,
    data_rows: usize,
    first_row_without_preceding_exact_header: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ForbiddenWindowTableShape {
    glazing_header_rows: usize,
    glazing_data_rows: usize,
    construction_header_rows: usize,
    construction_data_rows: usize,
}

impl ForbiddenWindowTableShape {
    const fn total_rows(self) -> usize {
        self.glazing_header_rows
            + self.glazing_data_rows
            + self.construction_header_rows
            + self.construction_data_rows
    }
}

#[derive(Debug)]
struct SimpleGlazingComparison {
    definitions: Vec<SimpleGlazingDefinition>,
    oracle_material_details: Vec<EioMaterialDetails>,
    report_requests: ConstructionReportRequests,
    material_details_shape: MaterialDetailsTableShape,
    forbidden_window_shape: ForbiddenWindowTableShape,
    tolerance_mode: NumericToleranceMode,
    passed: bool,
    first_divergence: Option<String>,
}

pub(crate) fn run_compare_window_material_simple_glazing_system(args: &[String]) -> i32 {
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
    if simple_glazing_definitions(&model).is_empty() {
        eprintln!("no WindowMaterial:SimpleGlazingSystem objects are available for comparison");
        return 1;
    }

    let eio_contents = match std::fs::read_to_string(eio_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("failed to read {}: {error}", eio_path);
            return 1;
        }
    };
    let comparison = match compare_window_material_simple_glazing_system(
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
                "unsupported WindowMaterial:SimpleGlazingSystem tolerance mode: {value}"
            )),
        },
        _ => Err("unsupported WindowMaterial:SimpleGlazingSystem compare options".to_string()),
    }
}

fn compare_window_material_simple_glazing_system(
    raw_model: &RawModel,
    model: &TypedModel,
    eio_contents: &str,
    tolerance_mode: NumericToleranceMode,
) -> Result<SimpleGlazingComparison, String> {
    let definitions = simple_glazing_definitions(model);
    let report_requests = construction_report_requests(raw_model)?;
    let oracle_material_details = match parse_eio_material_details(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingMaterialDetails) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let material_details_shape = material_details_table_shape(eio_contents);
    let forbidden_window_shape = forbidden_window_table_shape(eio_contents);
    let expected_header_rows = usize::from(report_requests.materials);
    let mut passed = true;
    let mut first_divergence = None;

    if material_details_shape.exact_header_rows != expected_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "Material Details header expected {expected_header_rows} observed {}",
                material_details_shape.exact_header_rows
            ),
        );
    }
    if material_details_shape.candidate_header_rows != material_details_shape.exact_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "Material Details header candidates expected every candidate to match the EnergyPlus 26.1 source literal; candidates {} exact {}",
                material_details_shape.candidate_header_rows,
                material_details_shape.exact_header_rows
            ),
        );
    }
    if !report_requests.materials && material_details_shape.data_rows != 0 {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "Material Details rows expected 0 when Materials reporting is not requested observed {}",
                material_details_shape.data_rows
            ),
        );
    }
    if let Some(line) = material_details_shape.first_row_without_preceding_exact_header {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "Material Details row at line {line} appears without a preceding exact EnergyPlus 26.1 header"
            ),
        );
    }
    if forbidden_window_shape.total_rows() != 0 {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "forbidden specialized window report evidence observed: glazing headers {}, glazing rows {}, WindowConstruction headers {}, WindowConstruction rows {}",
                forbidden_window_shape.glazing_header_rows,
                forbidden_window_shape.glazing_data_rows,
                forbidden_window_shape.construction_header_rows,
                forbidden_window_shape.construction_data_rows
            ),
        );
    }

    let indices = indices_by_material_name(&oracle_material_details);
    for definition in &definitions {
        let matching_indices = indices
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
            let observed = &oracle_material_details[*index];
            if !generic_row_matches(definition, observed, tolerance_mode) {
                passed = false;
                record_generic_divergence(
                    &mut first_divergence,
                    definition,
                    observed,
                    tolerance_mode,
                );
            }
        }
    }

    Ok(SimpleGlazingComparison {
        definitions,
        oracle_material_details,
        report_requests,
        material_details_shape,
        forbidden_window_shape,
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
            let selector = value.trim();
            if selector.is_empty() {
                continue;
            }
            if selector.eq_ignore_ascii_case("Constructions") {
                requests.constructions = true;
            } else if selector.eq_ignore_ascii_case("Materials") {
                requests.materials = true;
            } else {
                return Err(format!(
                    "{OUTPUT_CONSTRUCTIONS_OBJECT_TYPE}/{} field {field_name} must be blank, Constructions, or Materials; found {value:?}",
                    report_name.0
                ));
            }
        }
    }
    Ok(requests)
}

fn simple_glazing_definitions(model: &TypedModel) -> Vec<SimpleGlazingDefinition> {
    model
        .materials
        .iter()
        .filter_map(|material| {
            let MaterialDefinition::WindowSimpleGlazing(fields) = material.definition else {
                return None;
            };
            Some(SimpleGlazingDefinition {
                material_name: material.name.0.clone(),
                fields,
            })
        })
        .collect()
}

fn indices_by_material_name(rows: &[EioMaterialDetails]) -> BTreeMap<String, Vec<usize>> {
    let mut indices = BTreeMap::<String, Vec<usize>>::new();
    for (index, row) in rows.iter().enumerate() {
        indices
            .entry(normalized_material_name(&row.material_name))
            .or_default()
            .push(index);
    }
    indices
}

fn normalized_material_name(name: &str) -> String {
    name.trim().to_ascii_uppercase()
}

fn source_rounded_nonnegative(value: f64, precision: usize) -> Option<f64> {
    if value == 0.0 {
        Some(0.0)
    } else {
        super::energyplus_round_sig_digits_positive(value, precision)
    }
}

fn expected_numeric_fields(
    definition: &SimpleGlazingDefinition,
    observed: &EioMaterialDetails,
) -> Option<[(&'static str, f64, f64); 8]> {
    let fields = definition.fields;
    Some([
        (
            "thermal_resistance_m2_k_per_w",
            super::energyplus_round_sig_digits_positive(fields.thermal_resistance_m2_k_per_w, 4)?,
            observed.thermal_resistance_m2_k_per_w,
        ),
        (
            "thickness_m",
            super::energyplus_round_sig_digits_positive(fields.thickness_m, 4)?,
            observed.thickness_m,
        ),
        (
            "conductivity_w_per_m_k",
            super::energyplus_round_sig_digits_positive(fields.conductivity_w_per_m_k, 3)?,
            observed.conductivity_w_per_m_k,
        ),
        ("density_kg_per_m3", 0.0, observed.density_kg_per_m3),
        (
            "specific_heat_j_per_kg_k",
            0.0,
            observed.specific_heat_j_per_kg_k,
        ),
        (
            "thermal_absorptance",
            source_rounded_nonnegative(fields.thermal_absorptance, 4)?,
            observed.thermal_absorptance,
        ),
        (
            "solar_absorptance",
            source_rounded_nonnegative(fields.solar_absorptance, 4)?,
            observed.solar_absorptance,
        ),
        (
            "visible_absorptance",
            source_rounded_nonnegative(fields.visible_absorptance, 4)?,
            observed.visible_absorptance,
        ),
    ])
}

fn generic_row_matches(
    definition: &SimpleGlazingDefinition,
    observed: &EioMaterialDetails,
    tolerance_mode: NumericToleranceMode,
) -> bool {
    definition.fields.roughness == MaterialSurfaceRoughness::VerySmooth
        && normalized_material_name(&observed.material_name)
            == normalized_material_name(&definition.material_name)
        && observed.roughness == SIMPLE_GLAZING_ROUGHNESS
        && expected_numeric_fields(definition, observed).is_some_and(|fields| {
            fields.into_iter().all(|(_field, expected, observed)| {
                tolerance_mode.accepts_expected(expected, observed)
            })
        })
}

fn record_generic_divergence(
    first_divergence: &mut Option<String>,
    definition: &SimpleGlazingDefinition,
    observed: &EioMaterialDetails,
    tolerance_mode: NumericToleranceMode,
) {
    let prefix = format!("material {}", definition.material_name);
    if definition.fields.roughness != MaterialSurfaceRoughness::VerySmooth {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} typed roughness expected {SIMPLE_GLAZING_ROUGHNESS} observed {:?}",
                definition.fields.roughness
            ),
        );
        return;
    }
    if normalized_material_name(&observed.material_name)
        != normalized_material_name(&definition.material_name)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field material_name expected {} observed {}",
                definition.material_name, observed.material_name
            ),
        );
        return;
    }
    if observed.roughness != SIMPLE_GLAZING_ROUGHNESS {
        record_first_divergence(
            first_divergence,
            format!(
                "{prefix} field roughness expected {SIMPLE_GLAZING_ROUGHNESS} observed {}",
                observed.roughness
            ),
        );
        return;
    }
    let Some(numeric_fields) = expected_numeric_fields(definition, observed) else {
        record_first_divergence(
            first_divergence,
            format!("{prefix} contains a value that cannot be source-formatted"),
        );
        return;
    };
    if let Some((field, expected, observed)) = numeric_fields
        .into_iter()
        .find(|(_field, expected, observed)| !tolerance_mode.accepts_expected(*expected, *observed))
    {
        record_first_divergence(
            first_divergence,
            format!("{prefix} field {field} expected {expected:.9} observed {observed:.9}"),
        );
    }
}

fn raw_field<'a>(object: &'a RawObject, field_name: &str) -> Option<&'a RawValue> {
    object
        .fields
        .iter()
        .find(|(field, _value)| field.0 == field_name)
        .map(|(_field, value)| value)
}

fn material_details_table_shape(eio_contents: &str) -> MaterialDetailsTableShape {
    const ROW_LABEL: &str = "Material Details,";

    let mut shape = MaterialDetailsTableShape::default();
    let mut first_exact_header_line = None;
    for (line_index, line) in eio_contents.lines().enumerate() {
        let line_number = line_index + 1;
        if line
            .trim_start()
            .starts_with(MATERIAL_DETAILS_HEADER_MARKER)
        {
            shape.candidate_header_rows += 1;
            if line == MATERIAL_DETAILS_HEADER {
                shape.exact_header_rows += 1;
                first_exact_header_line.get_or_insert(line_number);
            }
        }
        if line.trim().starts_with(ROW_LABEL) {
            shape.data_rows += 1;
            if first_exact_header_line.is_none()
                && shape.first_row_without_preceding_exact_header.is_none()
            {
                shape.first_row_without_preceding_exact_header = Some(line_number);
            }
        }
    }
    shape
}

fn forbidden_window_table_shape(eio_contents: &str) -> ForbiddenWindowTableShape {
    let mut shape = ForbiddenWindowTableShape::default();
    for line in eio_contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("! <WindowMaterial:Glazing>,") {
            shape.glazing_header_rows += 1;
        }
        if trimmed.starts_with("WindowMaterial:Glazing,") {
            shape.glazing_data_rows += 1;
        }
        if trimmed.starts_with("! <WindowConstruction>,") {
            shape.construction_header_rows += 1;
        }
        if trimmed.starts_with("WindowConstruction,") {
            shape.construction_data_rows += 1;
        }
    }
    shape
}

fn record_first_divergence(first_divergence: &mut Option<String>, value: String) {
    if first_divergence.is_none() {
        *first_divergence = Some(value);
    }
}

fn render_comparison(comparison: &SimpleGlazingComparison) {
    let indices = indices_by_material_name(&comparison.oracle_material_details);
    let oracle_simple_glazing_rows = comparison
        .definitions
        .iter()
        .map(|definition| {
            indices
                .get(&normalized_material_name(&definition.material_name))
                .map_or(0, Vec::len)
        })
        .sum::<usize>();

    println!("Window Material Simple Glazing System Comparison");
    println!("  case_id: {CASE_ID}");
    println!("  comparison_class: smoke");
    println!("  evidence: diagnostic-only");
    println!("  blocking: false");
    println!("  conformance_claim: false");
    println!("  runtime_claim: false");
    println!("  input_u_factor_reporting_claim: false");
    println!("  input_shgc_reporting_claim: false");
    println!("  input_visible_transmittance_reporting_claim: false");
    println!("  specialized_glazing_claim: false");
    println!("  window_construction_claim: false");
    println!("  construction_use_claim: false");
    println!("  window_optics_claim: false");
    println!("  incident_angle_optics_claim: false");
    println!("  hemispherical_optics_claim: false");
    println!("  window_thermal_claim: false");
    println!("  ratings_claim: false");
    println!("  surface_behavior_claim: false");
    println!("  daylighting_claim: false");
    println!("  rust_eio_serialization_claim: false");
    println!("  broad_idf_declaration_order_claim: false");
    println!("  tolerance_mode: {}", comparison.tolerance_mode.label());
    println!(
        "  tolerance_policy: {}",
        comparison.tolerance_mode.policy_label()
    );
    println!("  material_objects: {}", comparison.definitions.len());
    println!("  oracle_simple_glazing_rows: {oracle_simple_glazing_rows}");
    println!(
        "  oracle_material_detail_rows: {}",
        comparison.oracle_material_details.len()
    );
    println!(
        "  material_details_header_rows: {}",
        comparison.material_details_shape.exact_header_rows
    );
    println!(
        "  specialized_glazing_header_rows: {}",
        comparison.forbidden_window_shape.glazing_header_rows
    );
    println!(
        "  specialized_glazing_rows: {}",
        comparison.forbidden_window_shape.glazing_data_rows
    );
    println!(
        "  window_construction_header_rows: {}",
        comparison.forbidden_window_shape.construction_header_rows
    );
    println!(
        "  window_construction_rows: {}",
        comparison.forbidden_window_shape.construction_data_rows
    );
    println!(
        "  materials_report_requested: {}",
        comparison.report_requests.materials
    );
    println!(
        "  constructions_report_requested: {}",
        comparison.report_requests.constructions
    );

    for (definition_index, definition) in comparison.definitions.iter().enumerate() {
        let matches = indices
            .get(&normalized_material_name(&definition.material_name))
            .map(Vec::as_slice)
            .unwrap_or_default();
        let expected_rows = usize::from(comparison.report_requests.materials);
        let status = matches.len() == expected_rows
            && matches.first().is_none_or(|index| {
                generic_row_matches(
                    definition,
                    &comparison.oracle_material_details[*index],
                    comparison.tolerance_mode,
                )
            });
        println!(
            "  definition: {} material: {} oracle_matches: {} status: {}",
            definition_index + 1,
            definition.material_name,
            matches.len(),
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
#[path = "window_material_simple_glazing_system_tests.rs"]
mod tests;
