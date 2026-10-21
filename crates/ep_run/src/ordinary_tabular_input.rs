//! Exact Table:Style raw fields, format and staged metadata after sole admission.
use ep_raw_model::{RawModel, RawValue};
use ep_runtime::heat_balance::ordinary_tabular_reports::SelectedTabularInput;
use ep_runtime::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_runtime::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use std::collections::BTreeMap;
pub(crate) fn resolve(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedTabularInput, String> {
    resolve_ordered(raw, input, false)
}

pub(crate) fn resolve_source(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedTabularInput, String> {
    resolve_ordered(raw, input, true)
}

fn resolve_ordered(
    raw: &RawModel,
    input: &SelectedInputProcessor,
    source_order: bool,
) -> Result<SelectedTabularInput, String> {
    let mut result = SelectedTabularInput {
        source_is_epjson: raw.is_epjson(),
        table_style_objects: Vec::new(),
    };
    for (name, object) in if source_order {
        raw.source_ordered_instances("OutputControl:Table:Style")?
    } else {
        raw.ordered_instances("OutputControl:Table:Style")
            .map_err(|e| e.to_string())?
    } {
        if !input.has_object("OutputControl:Table:Style", &name.0) {
            return Err("Table:Style same raw/IP object identity".into());
        }
        let mut fields = BTreeMap::new();
        let mut idf_max_fields = None;
        for (field, value) in &object.fields {
            match field.0.as_str() {
                "idf_max_fields" => {
                    let RawValue::Number(v) = value else {
                        return Err("actual Table:Style IDF metadata shape".into());
                    };
                    idf_max_fields = Some(
                        v.parse::<usize>()
                            .map_err(|_| "actual Table:Style IDF metadata integer")?,
                    );
                }
                "idf_order" | "idf_max_extensible_fields" => {}
                _ => {
                    let RawValue::String(v) = value else {
                        return Err("actual Table:Style alpha JSON shape".into());
                    };
                    fields.insert(field.0.clone(), ProjectInputValue::String(v.clone()));
                }
            }
        }
        result.table_style_objects.push(ProjectInputObject {
            object_name: name.0.clone(),
            fields,
            idf_max_fields,
        });
    }
    Ok(result)
}
