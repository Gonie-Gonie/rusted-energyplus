use std::path::Path;

use ep_conformance::{ConformanceCase, OutputFrequency, SourceArtifact};

use crate::output_frequency_idf_label;

pub(super) struct OutputInjectionSummary {
    pub(super) outputs: usize,
    pub(super) meters: usize,
    pub(super) surface_details: bool,
}

pub(super) fn stage_idf_with_output_requests(
    source_idf: &Path,
    staged_idf: &Path,
    manifest: &ConformanceCase,
) -> Result<OutputInjectionSummary, String> {
    let mut idf = std::fs::read_to_string(source_idf)
        .map_err(|error| format!("failed to read case IDF for output injection: {error}"))?;
    let injection = render_output_request_injection(manifest, &idf);
    if !injection.text.is_empty() {
        if !idf.ends_with('\n') {
            idf.push('\n');
        }
        idf.push_str(&injection.text);
    }
    std::fs::write(staged_idf, idf)
        .map_err(|error| format!("failed to stage case IDF with output requests: {error}"))?;
    Ok(OutputInjectionSummary {
        outputs: injection.outputs,
        meters: injection.meters,
        surface_details: injection.surface_details,
    })
}

pub(super) struct RenderedOutputInjection {
    pub(super) text: String,
    pub(super) outputs: usize,
    pub(super) meters: usize,
    pub(super) surface_details: bool,
}

pub(super) fn render_output_request_injection(
    manifest: &ConformanceCase,
    existing_idf: &str,
) -> RenderedOutputInjection {
    let mut text = String::new();
    let existing_outputs = existing_output_variables(existing_idf);
    let existing_meters = existing_output_meters(existing_idf);
    let existing_surface_details = has_existing_output_surfaces_details(existing_idf);
    let requested_output_count = manifest
        .outputs
        .iter()
        .filter(|output| output.source == SourceArtifact::Eso)
        .count();
    let requested_meter_count = manifest.meters.len();
    let requested_surface_details = manifest.outputs.iter().any(|output| {
        output.source == SourceArtifact::Eio
            && output
                .variable
                .trim()
                .to_ascii_lowercase()
                .starts_with("heattransfer surface")
    });

    if requested_output_count == 0 && requested_meter_count == 0 && !requested_surface_details {
        return RenderedOutputInjection {
            text,
            outputs: 0,
            meters: 0,
            surface_details: false,
        };
    }

    text.push_str("\n!- eplus-rs output request injection begin\n");
    text.push_str(&format!("!- case_id: {}\n", manifest.id));
    text.push_str("!- source: case manifest outputs/meters\n");
    let mut output_count = 0;
    let mut meter_count = 0;
    let mut surface_details = false;
    if requested_surface_details && !existing_surface_details {
        text.push_str("Output:Surfaces:List,Details;\n\n");
        surface_details = true;
    }
    for output in &manifest.outputs {
        if output.source != SourceArtifact::Eso {
            continue;
        }
        if has_existing_output_variable(
            &existing_outputs,
            &output.key,
            &output.variable,
            output.frequency,
        ) {
            continue;
        }
        if output_count == 0 {
            text.push_str("Output:VariableDictionary,Regular;\n\n");
        }
        text.push_str("Output:Variable,\n");
        text.push_str(&format!("  {},  !- Key Value\n", idf_field(&output.key)));
        text.push_str(&format!(
            "  {},  !- Variable Name\n",
            idf_field(&output.variable)
        ));
        text.push_str(&format!(
            "  {};  !- Reporting Frequency\n\n",
            output_frequency_idf_label(output.frequency)
        ));
        output_count += 1;
    }
    for meter in &manifest.meters {
        if has_existing_output_meter(&existing_meters, &meter.name, meter.frequency) {
            continue;
        }
        text.push_str("Output:Meter,\n");
        text.push_str(&format!("  {},  !- Key Name\n", idf_field(&meter.name)));
        text.push_str(&format!(
            "  {};  !- Reporting Frequency\n\n",
            output_frequency_idf_label(meter.frequency)
        ));
        meter_count += 1;
    }
    if output_count == 0 && meter_count == 0 && !surface_details {
        text.push_str("!- no new output requests; staged IDF already contains manifest requests\n");
    }
    text.push_str("!- eplus-rs output request injection end\n");

    RenderedOutputInjection {
        text,
        outputs: output_count,
        meters: meter_count,
        surface_details,
    }
}

fn existing_output_variables(idf: &str) -> Vec<(String, String, String)> {
    idf_objects(idf, "Output:Variable")
        .into_iter()
        .filter_map(|fields| {
            if fields.len() >= 4 {
                Some((fields[1].clone(), fields[2].clone(), fields[3].clone()))
            } else {
                None
            }
        })
        .collect()
}

fn existing_output_meters(idf: &str) -> Vec<(String, String)> {
    idf_objects(idf, "Output:Meter")
        .into_iter()
        .filter_map(|fields| {
            if fields.len() >= 3 {
                Some((fields[1].clone(), fields[2].clone()))
            } else {
                None
            }
        })
        .collect()
}

fn has_existing_output_surfaces_details(idf: &str) -> bool {
    idf_objects(idf, "Output:Surfaces:List")
        .into_iter()
        .any(|fields| {
            fields
                .get(1)
                .is_some_and(|field| field.eq_ignore_ascii_case("details"))
        })
}

fn idf_objects(idf: &str, object_type: &str) -> Vec<Vec<String>> {
    let object_type = normalize_idf_request_field(object_type);
    let mut objects = Vec::new();
    let mut current = String::new();

    for line in idf.lines() {
        let content = line.split('!').next().unwrap_or("").trim();
        if content.is_empty() {
            continue;
        }
        current.push_str(content);
        current.push(' ');
        if content.contains(';') {
            let fields: Vec<String> = current
                .replace(';', ",")
                .split(',')
                .map(normalize_idf_request_field)
                .filter(|field| !field.is_empty())
                .collect();
            if fields
                .first()
                .is_some_and(|field| field.as_str() == object_type)
            {
                objects.push(fields);
            }
            current.clear();
        }
    }

    objects
}

fn has_existing_output_variable(
    existing: &[(String, String, String)],
    key: &str,
    variable: &str,
    frequency: OutputFrequency,
) -> bool {
    let key = normalize_idf_request_field(key);
    let variable = normalize_idf_request_field(variable);
    let frequency = normalize_idf_request_field(output_frequency_idf_label(frequency));
    existing
        .iter()
        .any(|(existing_key, existing_variable, existing_frequency)| {
            (existing_key == "*" || existing_key == &key)
                && existing_variable == &variable
                && existing_frequency == &frequency
        })
}

fn has_existing_output_meter(
    existing: &[(String, String)],
    name: &str,
    frequency: OutputFrequency,
) -> bool {
    let name = normalize_idf_request_field(name);
    let frequency = normalize_idf_request_field(output_frequency_idf_label(frequency));
    existing.iter().any(|(existing_name, existing_frequency)| {
        existing_name == &name && existing_frequency == &frequency
    })
}

fn normalize_idf_request_field(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn idf_field(value: &str) -> String {
    value
        .replace(['\r', '\n'], " ")
        .replace([';', ','], " ")
        .trim()
        .to_string()
}
