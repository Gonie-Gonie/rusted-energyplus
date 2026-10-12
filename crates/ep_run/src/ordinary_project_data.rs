//! Purpose-specific retained GetProjectData values; no default/count authority.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use ep_runtime::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_runtime::heat_balance::selected_project_data::{
    ProjectDiagnosticsObject, ProjectInputObject, ProjectInputValue, SelectedProjectDataInput,
};
use std::collections::BTreeMap;

pub(crate) fn resolve(
    raw: &RawModel,
    source_is_epjson: bool,
    input: &SelectedInputProcessor,
) -> Result<SelectedProjectDataInput, String> {
    // The caller already passed this exact raw model through the sole ordinary
    // admission and created this same live IP owner. Do not run another grammar.
    let mut result = SelectedProjectDataInput {
        source_is_epjson,
        ..SelectedProjectDataInput::default()
    };
    for family in [
        "Version",
        "HeatBalanceAlgorithm",
        "Timestep",
        "SimulationControl",
    ] {
        let Some(objects) = raw.objects.get(&ObjectType(family.into())) else {
            continue;
        };
        if input.get_num_objects_found(family) != objects.len() {
            return Err(format!("GetProjectData raw/IP association: {family}"));
        }
        let mut projected = Vec::new();
        // Positive getObjectItem families are unique under the existing ordinary
        // admission; no independently sorted multi-object IDF order is invented.
        if objects.len() > 1 {
            return Err(format!("GetProjectData admitted unique family: {family}"));
        }
        for (name, object) in objects {
            if !input.has_object(family, &name.0) {
                return Err(format!(
                    "GetProjectData object identity: {family}/{}",
                    name.0
                ));
            }
            let mut fields = BTreeMap::new();
            let mut idf_max_fields = None;
            for (field, value) in &object.fields {
                match field.0.as_str() {
                    "idf_max_fields" => {
                        let RawValue::Number(text) = value else {
                            return Err("actual idf_max_fields is not numeric".into());
                        };
                        idf_max_fields = Some(
                            text.parse::<usize>()
                                .map_err(|_| "actual idf_max_fields is not usize")?,
                        );
                        continue;
                    }
                    "idf_order" | "idf_max_extensible_fields" => continue,
                    _ => {}
                }
                let value = match value {
                    RawValue::String(value) => ProjectInputValue::String(value.clone()),
                    RawValue::Number(value) => {
                        let value = value
                            .parse::<f64>()
                            .map_err(|_| "GetProjectData raw numeric parse")?;
                        if !value.is_finite() {
                            return Err("GetProjectData nonfinite JSON number".into());
                        }
                        ProjectInputValue::Number(value)
                    }
                    _ => {
                        return Err(format!(
                            "GetProjectData actual JSON type unavailable: {family}.{}",
                            field.0
                        ));
                    }
                };
                fields.insert(field.0.clone(), value);
            }
            projected.push(ProjectInputObject {
                object_name: name.0.clone(),
                fields,
                idf_max_fields,
            });
        }
        result.objects.insert(family.into(), projected);
    }
    if let Some(objects) = raw.objects.get(&ObjectType("Output:Diagnostics".into())) {
        if input.get_num_objects_found("Output:Diagnostics") != objects.len() {
            return Err("GetProjectData diagnostics/IP count association".into());
        }
        for (name, object) in objects {
            if !input.has_object("Output:Diagnostics", &name.0) {
                return Err("GetProjectData diagnostics identity".into());
            }
            let mut keys = Vec::new();
            if let Some(value) = object.fields.get(&FieldName("diagnostics".into())) {
                let RawValue::Array(rows) = value else {
                    return Err("GetProjectData diagnostics JSON array".into());
                };
                for row in rows {
                    let RawValue::Object(row) = row else {
                        return Err("GetProjectData diagnostics JSON object".into());
                    };
                    keys.push(match row.get(&FieldName("key".into())) {
                        None => None,
                        Some(RawValue::String(value)) => Some(value.clone()),
                        _ => return Err("GetProjectData diagnostic key JSON string".into()),
                    });
                }
            }
            result.diagnostics.push(ProjectDiagnosticsObject {
                object_name: name.0.clone(),
                keys,
            });
        }
    }
    Ok(result)
}
