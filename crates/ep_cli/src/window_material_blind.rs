use std::collections::{BTreeMap, BTreeSet};

use ep_compare::{
    EioError, EioMaterialDetails, EioWindowMaterialBlind, Tolerance, WINDOW_MATERIAL_BLIND_HEADER,
    parse_eio_material_details, parse_eio_window_material_blind,
};
use ep_compiler::compile_raw_model;
use ep_model::{
    Construction, ConstructionKind, MaterialDefinition, TypedModel, WindowBlindMaterial,
};
use ep_raw_model::{RawModel, RawObject, RawValue, load_epjson_file};

pub(crate) const USAGE: &str = "usage: eplus-rs compare window-material-blind <input.epJSON> <eplusout.eio> [--tolerance exact|near]";

const OUTPUT_CONSTRUCTIONS_OBJECT_TYPE: &str = "Output:Constructions";
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
            Self::Exact => {
                "energyplus-26.1-material-details-zero-exact-blind-width-separation-thickness-4R-other-specialized-3R-normalized-exact"
            }
            Self::Near => "absolute-0.00001-relative-0.000001-against-source-rounded-expected",
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
struct WindowBlindDefinition {
    material_name: String,
}

#[derive(Clone, Debug)]
struct WindowBlindOccurrence {
    construction_name: String,
    layer_number: usize,
    material_name: String,
    fields: WindowBlindMaterial,
}

#[derive(Debug)]
struct WindowBlindComparison {
    definitions: Vec<WindowBlindDefinition>,
    occurrences: Vec<WindowBlindOccurrence>,
    activated_material_names: BTreeSet<String>,
    oracle_material_details: Vec<EioMaterialDetails>,
    oracle_occurrences: Vec<EioWindowMaterialBlind>,
    report_requests: ConstructionReportRequests,
    material_details_header_rows: usize,
    header_rows: usize,
    tolerance_mode: NumericToleranceMode,
    passed: bool,
    first_divergence: Option<String>,
}

pub(crate) fn run_compare_window_material_blind(args: &[String]) -> i32 {
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
    if window_blind_definitions(&model).is_empty() {
        eprintln!("no WindowMaterial:Blind objects are available for comparison");
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
        match compare_window_material_blind(&raw_model, &model, &eio_contents, tolerance_mode) {
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
                "unsupported WindowMaterial:Blind tolerance mode: {value}"
            )),
        },
        _ => Err("unsupported WindowMaterial:Blind compare options".to_string()),
    }
}

fn compare_window_material_blind(
    raw_model: &RawModel,
    model: &TypedModel,
    eio_contents: &str,
    tolerance_mode: NumericToleranceMode,
) -> Result<WindowBlindComparison, String> {
    let definitions = window_blind_definitions(model);
    let report_requests = construction_report_requests(raw_model)?;
    let has_fenestration_construction = model
        .constructions
        .iter()
        .any(|construction| construction.kind == ConstructionKind::Fenestration);
    let expected_header_rows = usize::from(
        report_requests.constructions && has_fenestration_construction && !definitions.is_empty(),
    );
    let occurrences = if report_requests.constructions {
        window_blind_occurrences(model)?
    } else {
        Vec::new()
    };
    let activated_material_names = occurrences
        .iter()
        .map(|occurrence| normalized_material_name(&occurrence.material_name))
        .collect::<BTreeSet<_>>();

    let oracle_material_details = match parse_eio_material_details(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingMaterialDetails) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let oracle_occurrences = match parse_eio_window_material_blind(eio_contents) {
        Ok(rows) => rows,
        Err(EioError::MissingWindowMaterialBlindHeader) => Vec::new(),
        Err(error) => return Err(error.to_string()),
    };
    let material_details_shape = material_details_table_shape(eio_contents);
    let header_rows = window_material_blind_header_count(eio_contents);
    let mut passed = true;
    let mut first_divergence = None;

    let expected_material_details_header_rows = usize::from(report_requests.materials);
    if material_details_shape.exact_header_rows != expected_material_details_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "Material Details header expected {expected_material_details_header_rows} observed {}",
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

    if header_rows != expected_header_rows {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "WindowMaterial:Blind header expected {expected_header_rows} observed {header_rows}"
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
            let observed = &oracle_material_details[*index];
            if !generic_row_matches(definition, observed) {
                passed = false;
                record_generic_divergence(&mut first_divergence, definition, observed);
            }
        }
    }

    if occurrences.len() != oracle_occurrences.len() {
        passed = false;
        record_first_divergence(
            &mut first_divergence,
            format!(
                "blind construction occurrences expected {} observed {}",
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
                "unexpected WindowMaterial:Blind row for material {}",
                unexpected.material_name
            ),
        );
    }

    Ok(WindowBlindComparison {
        definitions,
        occurrences,
        activated_material_names,
        oracle_material_details,
        oracle_occurrences,
        report_requests,
        material_details_header_rows: material_details_shape.exact_header_rows,
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

fn window_blind_definitions(model: &TypedModel) -> Vec<WindowBlindDefinition> {
    model
        .materials
        .iter()
        .filter_map(|material| {
            let MaterialDefinition::WindowBlind(_) = material.definition else {
                return None;
            };
            Some(WindowBlindDefinition {
                material_name: material.name.0.clone(),
            })
        })
        .collect()
}

fn construction_layers(construction: &Construction) -> &[ep_model::MaterialId] {
    if construction.layers.is_empty() {
        std::slice::from_ref(&construction.outside_layer)
    } else {
        construction.layers.as_slice()
    }
}

fn window_blind_occurrences(model: &TypedModel) -> Result<Vec<WindowBlindOccurrence>, String> {
    let mut rows = Vec::new();
    for construction in &model.constructions {
        let layers = construction_layers(construction);
        let mut blind_layers = Vec::new();
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
            if let MaterialDefinition::WindowBlind(fields) = material.definition {
                blind_layers.push((layer_index, material, fields));
            }
        }
        if blind_layers.is_empty() {
            continue;
        }
        if construction.kind != ConstructionKind::Fenestration {
            return Err(format!(
                "construction {} contains ordinary Blind material but is not fenestration; bounded Blind EIO comparison rejects this topology",
                construction.name.0
            ));
        }
        if blind_layers.len() != 1 {
            return Err(format!(
                "construction {} contains {} ordinary Blind layers; bounded Blind EIO comparison requires exactly one",
                construction.name.0,
                blind_layers.len()
            ));
        }

        let (layer_index, material, fields) = blind_layers[0];
        if layer_index != 0 && layer_index + 1 != layers.len() {
            return Err(format!(
                "construction {} layer {} material {} is a between-glass ordinary Blind; bounded Blind EIO comparison does not claim between-glass reporting",
                construction.name.0,
                layer_index + 1,
                material.name.0
            ));
        }
        let bare_layers = if layer_index == 0 {
            &layers[1..]
        } else {
            &layers[..layers.len() - 1]
        };
        let has_exact_bare_companion = !bare_layers.is_empty()
            && model.constructions.iter().any(|candidate| {
                candidate.kind == ConstructionKind::Fenestration
                    && candidate.id != construction.id
                    && construction_layers(candidate) == bare_layers
            });
        if !has_exact_bare_companion {
            return Err(format!(
                "construction {} layer {} material {} is missing an exact bare companion fenestration construction for its non-Blind layer stack; EnergyPlus omits this specialized row",
                construction.name.0,
                layer_index + 1,
                material.name.0
            ));
        }
        rows.push(WindowBlindOccurrence {
            construction_name: construction.name.0.clone(),
            layer_number: layer_index + 1,
            material_name: material.name.0.clone(),
            fields,
        });
    }
    Ok(rows)
}

fn generic_numeric_fields(oracle: &EioMaterialDetails) -> [(&'static str, f64, f64); 8] {
    [
        (
            "thermal_resistance_m2_k_per_w",
            0.0,
            oracle.thermal_resistance_m2_k_per_w,
        ),
        ("thickness_m", 0.0, oracle.thickness_m),
        ("conductivity_w_per_m_k", 0.0, oracle.conductivity_w_per_m_k),
        ("density_kg_per_m3", 0.0, oracle.density_kg_per_m3),
        (
            "specific_heat_j_per_kg_k",
            0.0,
            oracle.specific_heat_j_per_kg_k,
        ),
        ("thermal_absorptance", 0.0, oracle.thermal_absorptance),
        ("solar_absorptance", 0.0, oracle.solar_absorptance),
        ("visible_absorptance", 0.0, oracle.visible_absorptance),
    ]
}

fn generic_row_matches(definition: &WindowBlindDefinition, oracle: &EioMaterialDetails) -> bool {
    normalized_material_name(&oracle.material_name)
        == normalized_material_name(&definition.material_name)
        && oracle.roughness == "Rough"
        && generic_numeric_fields(oracle)
            .into_iter()
            .all(|(_field, expected, observed)| expected == observed)
}

fn record_generic_divergence(
    first_divergence: &mut Option<String>,
    definition: &WindowBlindDefinition,
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
    if let Some((field, expected, observed)) = generic_numeric_fields(oracle)
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
    occurrence: &WindowBlindOccurrence,
    oracle: &EioWindowMaterialBlind,
) -> [(&'static str, f64, f64, usize); 7] {
    [
        (
            "slat_width_m",
            occurrence.fields.slat_width_m,
            oracle.slat_width_m,
            4,
        ),
        (
            "slat_separation_m",
            occurrence.fields.slat_separation_m,
            oracle.slat_separation_m,
            4,
        ),
        (
            "slat_thickness_m",
            occurrence.fields.slat_thickness_m,
            oracle.slat_thickness_m,
            4,
        ),
        (
            "slat_angle_deg",
            occurrence.fields.slat_angle_deg,
            oracle.slat_angle_deg,
            3,
        ),
        (
            "slat_beam_solar_transmittance",
            occurrence.fields.solar_beam_diffuse.transmittance,
            oracle.slat_beam_solar_transmittance,
            3,
        ),
        (
            "slat_beam_solar_front_reflectance",
            occurrence.fields.solar_beam_diffuse.front_reflectance,
            oracle.slat_beam_solar_front_reflectance,
            3,
        ),
        (
            "blind_to_glass_distance_m",
            occurrence.fields.blind_to_glass_distance_m,
            oracle.blind_to_glass_distance_m,
            3,
        ),
    ]
}

fn specialized_row_matches(
    occurrence: &WindowBlindOccurrence,
    oracle: &EioWindowMaterialBlind,
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
    occurrence: &WindowBlindOccurrence,
    oracle: &EioWindowMaterialBlind,
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct MaterialDetailsTableShape {
    candidate_header_rows: usize,
    exact_header_rows: usize,
    data_rows: usize,
    first_row_without_preceding_exact_header: Option<usize>,
}

fn material_details_table_shape(eio_contents: &str) -> MaterialDetailsTableShape {
    const HEADER_MARKER: &str = "! <Material Details>";
    const ROW_LABEL: &str = "Material Details,";

    let mut shape = MaterialDetailsTableShape::default();
    let mut first_exact_header_line = None;
    for (line_index, line) in eio_contents.lines().enumerate() {
        let line_number = line_index + 1;
        if line.trim_start().starts_with(HEADER_MARKER) {
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

fn window_material_blind_header_count(eio_contents: &str) -> usize {
    eio_contents
        .lines()
        .filter(|line| *line == WINDOW_MATERIAL_BLIND_HEADER)
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

fn render_comparison(comparison: &WindowBlindComparison) {
    let generic_indices = indices_by_material_name(&comparison.oracle_material_details, |row| {
        row.material_name.as_str()
    });
    let oracle_generic_blind_rows = comparison
        .definitions
        .iter()
        .map(|definition| {
            generic_indices
                .get(&normalized_material_name(&definition.material_name))
                .map_or(0, Vec::len)
        })
        .sum::<usize>();

    println!("Window Material Blind Comparison");
    println!("  evidence: diagnostic-only");
    println!("  blocking: false");
    println!("  conformance_claim: false");
    println!("  calc_blind_properties_claim: false");
    println!("  blind_optics_claim: false");
    println!("  window_runtime_claim: false");
    println!("  fenestration_surface_claim: false");
    println!("  window_shading_control_claim: false");
    println!("  construction_rating_claim: false");
    println!("  broad_idf_declaration_order_claim: false");
    println!("  arbitrary_idf_declaration_order_claim: false");
    println!("  between_glass_blind_claim: false");
    println!("  missing_bare_companion_claim: false");
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
    println!("  definitions: {}", comparison.definitions.len());
    println!(
        "  material_details_header_rows: {}",
        comparison.material_details_header_rows
    );
    println!("  oracle_generic_blind_rows: {oracle_generic_blind_rows}");
    println!(
        "  activated_blind_materials: {}",
        comparison.activated_material_names.len()
    );
    println!("  blind_occurrences: {}", comparison.occurrences.len());
    println!(
        "  oracle_blind_rows: {}",
        comparison.oracle_occurrences.len()
    );
    println!("  blind_header_rows: {}", comparison.header_rows);
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
#[path = "window_material_blind_tests.rs"]
mod tests;
