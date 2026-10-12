//! Actual selected raw presence/shapes and pinned legacy_idd query metadata.
//! The existing ordinary equipment resolver remains the sole positive admission.
use ep_raw_model::{RawModel, RawValue};
use ep_runtime::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceObjectShape,
};
use std::collections::BTreeMap;

mod schema;

fn json_size(value: &RawValue) -> usize {
    match value {
        RawValue::Null => 0,
        RawValue::Array(values) => values.len(),
        RawValue::Object(values) => values.len(),
        RawValue::Bool(_) | RawValue::Number(_) | RawValue::String(_) => 1,
    }
}

pub(crate) fn resolve(raw: &RawModel) -> Result<SelectedInputProcessor, String> {
    super::ordinary_equipment_input::resolve(raw)?;
    let families: BTreeMap<_, _> = raw
        .objects
        .iter()
        .map(|(family, objects)| {
            let shapes = objects
                .iter()
                .map(|(name, object)| SourceObjectShape {
                    object_name: name.0.clone(),
                    field_sizes: object
                        .fields
                        .iter()
                        .map(|(field, value)| (field.0.clone(), json_size(value)))
                        .collect(),
                })
                .collect();
            (family.0.clone(), SourceInputFamily::Objects(shapes))
        })
        .collect();
    SelectedInputProcessor::from_admitted_input(schema::definitions(), families)
        .map_err(|error| error.to_string())
}

#[cfg(test)]
#[path = "ordinary_input_processor_tests.rs"]
pub(crate) mod tests;
