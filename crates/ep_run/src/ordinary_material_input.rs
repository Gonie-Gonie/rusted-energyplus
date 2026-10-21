//! Purpose-specific Material raw custody, using the sole admitted IP and loader.
use ep_raw_model::{RawModel, RawValue};
use ep_runtime::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_runtime::heat_balance::selected_material_data::SelectedMaterialInput;
use ep_runtime::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use std::collections::BTreeMap;

pub(crate) fn resolve(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedMaterialInput, String> {
    resolve_ordered(raw, input, false)
}

pub(crate) fn resolve_source(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedMaterialInput, String> {
    resolve_ordered(raw, input, true)
}

fn resolve_ordered(
    raw: &RawModel,
    input: &SelectedInputProcessor,
    source_order: bool,
) -> Result<SelectedMaterialInput, String> {
    let mut result = SelectedMaterialInput {
        source_is_epjson: raw.is_epjson(),
        ..SelectedMaterialInput::default()
    };
    for family in ["Material", "Material:NoMass", "Material:AirGap"] {
        let rows = if source_order {
            raw.source_ordered_instances(family)?
        } else {
            raw.ordered_instances(family)
                .map_err(|error| error.to_string())?
        };
        if rows.len() != input.get_num_objects_found(family) {
            return Err(format!("Material raw/IP count: {family}"));
        }
        if !raw.is_epjson() && !rows.is_empty() && !raw.has_idf_declaration_order(family) {
            return Err(format!("actual Material IDF order unavailable: {family}"));
        }
        let mut values = Vec::new();
        for (name, row) in rows {
            if !input.has_object(family, &name.0) {
                return Err(format!("Material raw/IP identity: {family}/{}", name.0));
            }
            let mut fields = BTreeMap::new();
            let mut idf_max_fields = None;
            for (field, value) in &row.fields {
                if field.0 == "idf_max_fields" {
                    let RawValue::Number(text) = value else {
                        return Err("Material idf_max_fields numeric custody".into());
                    };
                    idf_max_fields = Some(
                        text.parse::<usize>()
                            .map_err(|_| "Material idf_max_fields usize custody")?,
                    );
                    continue;
                }
                if matches!(field.0.as_str(), "idf_order" | "idf_max_extensible_fields") {
                    continue;
                }
                let value = match value {
                    RawValue::String(value) => ProjectInputValue::String(value.clone()),
                    RawValue::Number(text) => {
                        let value = text
                            .parse::<f64>()
                            .map_err(|_| "Material raw binary64 parse")?;
                        if !value.is_finite() {
                            return Err("Material nonfinite JSON number".into());
                        }
                        ProjectInputValue::Number(value)
                    }
                    _ => {
                        return Err(format!(
                            "Material scalar representation: {family}.{}",
                            field.0
                        ));
                    }
                };
                fields.insert(field.0.clone(), value);
            }
            if !raw.is_epjson()
                && matches!(family, "Material:NoMass" | "Material:AirGap")
                && idf_max_fields.is_none()
            {
                return Err(format!("actual {family} IDF physical field count absent"));
            }
            values.push(ProjectInputObject {
                object_name: name.0.clone(),
                fields,
                idf_max_fields,
            });
        }
        match family {
            "Material" => result.regular = values,
            "Material:NoMass" => result.no_mass = values,
            "Material:AirGap" => result.air_gaps = values,
            _ => unreachable!("literal selected material family"),
        }
    }
    Ok(result)
}
