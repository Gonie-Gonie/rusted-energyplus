//! Same admitted raw Surface operands; no coordinate transformation or new query owner.
use ep_raw_model::{FieldName, RawModel, RawObject, RawValue};
use ep_runtime::heat_balance::selected_heat_balance_input::surface::{
    OpaqueSurfaceInput, SelectedSurfaceInput,
};
use ep_runtime::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_runtime::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use std::collections::BTreeMap;

fn metadata(object: &RawObject, field: &str) -> Result<Option<usize>, String> {
    object
        .fields
        .get(&FieldName(field.into()))
        .map(|value| {
            let RawValue::Number(text) = value else {
                return Err(format!("surface {field} actual integer"));
            };
            text.parse::<usize>()
                .map_err(|_| format!("surface {field} integer domain"))
        })
        .transpose()
}
fn scalar_object(name: &str, object: &RawObject) -> Result<ProjectInputObject, String> {
    let mut fields = BTreeMap::new();
    for (key, value) in &object.fields {
        if matches!(
            key.0.as_str(),
            "idf_order" | "idf_max_fields" | "idf_max_extensible_fields" | "vertices"
        ) {
            continue;
        }
        let scalar = match value {
            RawValue::String(value) => ProjectInputValue::String(value.clone()),
            RawValue::Number(text) => {
                let value = text
                    .parse::<f64>()
                    .map_err(|_| format!("surface {} numeric parse", key.0))?;
                if !value.is_finite() {
                    return Err(format!("surface {} nonfinite JSON", key.0));
                }
                ProjectInputValue::Number(value)
            }
            _ => return Err(format!("surface {} scalar representation", key.0)),
        };
        fields.insert(key.0.clone(), scalar);
    }
    Ok(ProjectInputObject {
        object_name: name.into(),
        fields,
        idf_max_fields: metadata(object, "idf_max_fields")?,
    })
}
pub(super) fn resolve(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedSurfaceInput, String> {
    let mut result = SelectedSurfaceInput::default();
    for family in ["GlobalGeometryRules", "BuildingSurface:Detailed"] {
        let ordered = raw
            .ordered_instances(family)
            .map_err(|error| error.to_string())?;
        if input.get_num_objects_found(family) != ordered.len() {
            return Err(format!("surface raw/IP count: {family}"));
        }
        for (name, object) in ordered {
            if !input.has_object(family, &name.0) {
                return Err(format!("surface raw/IP identity: {family}/{}", name.0));
            }
            let projected = scalar_object(&name.0, object)?;
            if !raw.is_epjson() && projected.idf_max_fields.is_none() {
                return Err(format!("surface missing actual IDF max_fields: {family}"));
            }
            if family == "GlobalGeometryRules" {
                result.geometry_rules.push(projected);
                continue;
            }
            let idf_max_extensible_fields = metadata(object, "idf_max_extensible_fields")?;
            if !raw.is_epjson() && idf_max_extensible_fields.is_none() {
                return Err("surface missing actual IDF extensible count".into());
            }
            let rows = match object.fields.get(&FieldName("vertices".into())) {
                Some(RawValue::Array(rows)) => rows,
                None => {
                    result.building_surfaces.push(OpaqueSurfaceInput {
                        object: projected,
                        idf_max_extensible_fields,
                        vertices: Vec::new(),
                    });
                    continue;
                }
                _ => return Err("surface actual vertices array representation".into()),
            };
            let mut vertices = Vec::with_capacity(rows.len());
            for row in rows {
                let RawValue::Object(row) = row else {
                    return Err("surface actual extensible row".into());
                };
                let mut point = [0.0; 3];
                for (index, key) in [
                    "vertex_x_coordinate",
                    "vertex_y_coordinate",
                    "vertex_z_coordinate",
                ]
                .iter()
                .enumerate()
                {
                    let Some(RawValue::Number(text)) = row.get(&FieldName((*key).into())) else {
                        return Err(format!("surface admitted coordinate {key}"));
                    };
                    point[index] = text
                        .parse::<f64>()
                        .map_err(|_| format!("surface coordinate {key} parse"))?;
                    if !point[index].is_finite() {
                        return Err(format!("surface coordinate {key} nonfinite"));
                    }
                }
                vertices.push(point);
            }
            result.building_surfaces.push(OpaqueSurfaceInput {
                object: projected,
                idf_max_extensible_fields,
                vertices,
            });
        }
    }
    Ok(result)
}
