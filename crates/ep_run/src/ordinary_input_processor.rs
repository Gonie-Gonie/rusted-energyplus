//! Actual selected raw presence/shapes and pinned legacy_idd query metadata.
//! The existing ordinary equipment resolver remains the sole positive admission.
use ep_raw_model::{RawModel, RawValue};
use ep_runtime::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceObjectShape,
};
use std::collections::BTreeMap;

mod input_statistics;
mod reporting_prescan;
mod schema;
pub(crate) use reporting_prescan::resolve as resolve_reporting_prescan;

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
    project(raw, input_statistics::resolve(raw))
}
/// Explicit SourceB keeps full raw operands on the same IP; no getter is bypassed.
pub(crate) fn resolve_source_b(raw: &RawModel) -> Result<SelectedInputProcessor, String> {
    super::ordinary_equipment_input::resolve_source_b(raw)?;
    project(raw, input_statistics::resolve_source_b(raw)?)
}
fn project(
    raw: &RawModel,
    statistics: ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsInput,
) -> Result<SelectedInputProcessor, String> {
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
    let mut input = SelectedInputProcessor::from_admitted_input(schema::definitions(), families)
        .map_err(|error| error.to_string())?;
    input
        .retain_source_input_statistics(statistics)
        .map_err(|error| error.to_string())?;
    Ok(input)
}

#[cfg(test)]
#[path = "ordinary_input_processor_tests.rs"]
pub(crate) mod tests;
