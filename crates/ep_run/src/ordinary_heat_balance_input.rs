//! Raw operands for the consumed selected HB getters; no second query service.
use ep_raw_model::{FieldName, RawModel, RawValue};
use ep_runtime::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInput;
use ep_runtime::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_runtime::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use std::collections::BTreeMap;
mod surface;

pub(crate) fn resolve(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedHeatBalanceInput, String> {
    // Actual SourceInputFormat custody comes from the loader, not field keys.
    // Existing ordinary admission has already checked this same raw model.
    let mut result = SelectedHeatBalanceInput {
        source_is_epjson: raw.is_epjson(),
        surface: surface::resolve(raw, input)?,
        objects: BTreeMap::new(),
    };
    for family in [
        "Building",
        "SurfaceConvectionAlgorithm:Inside",
        "SurfaceConvectionAlgorithm:Outside",
        "HeatBalanceAlgorithm",
        "ZoneAirHeatBalanceAlgorithm",
        "Construction",
        "Zone",
    ] {
        let ordered = raw.ordered_instances(family).map_err(|e| e.to_string())?;
        if input.get_num_objects_found(family) != ordered.len() {
            return Err(format!("HB raw/IP count: {family}"));
        }
        let mut projected = Vec::new();
        for (name, object) in ordered {
            if !input.has_object(family, &name.0) {
                return Err(format!("HB raw/IP identity: {family}/{}", name.0));
            }
            let idf_max_fields = object
                .fields
                .get(&FieldName("idf_max_fields".into()))
                .map(|value| {
                    let RawValue::Number(text) = value else {
                        return Err("HB idf_max_fields is not an actual integer".to_string());
                    };
                    text.parse::<usize>()
                        .map_err(|_| "HB idf_max_fields integer domain".to_string())
                })
                .transpose()?;
            if !result.source_is_epjson && idf_max_fields.is_none() {
                return Err(format!(
                    "HB requires real staged IDF max_fields for {family}"
                ));
            }
            let mut fields = BTreeMap::new();
            for (field, value) in &object.fields {
                if matches!(
                    field.0.as_str(),
                    "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
                ) {
                    continue;
                }
                let scalar = match value {
                    RawValue::String(value) => ProjectInputValue::String(value.clone()),
                    RawValue::Number(value) => {
                        let value = value
                            .parse::<f64>()
                            .map_err(|_| format!("HB numeric parse: {family}.{}", field.0))?;
                        if !value.is_finite() {
                            return Err(format!("HB nonfinite JSON: {family}.{}", field.0));
                        }
                        ProjectInputValue::Number(value)
                    }
                    _ => {
                        return Err(format!(
                            "HB actual JSON representation: {family}.{}",
                            field.0
                        ));
                    }
                };
                fields.insert(field.0.clone(), scalar);
            }
            projected.push(ProjectInputObject {
                object_name: name.0.clone(),
                fields,
                idf_max_fields,
            });
        }
        result.objects.insert(family.into(), projected);
    }
    Ok(result)
}
