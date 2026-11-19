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
pub(crate) use reporting_prescan::{
    resolve as resolve_reporting_prescan, resolve_source as resolve_reporting_prescan_source,
};

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
    project(raw, input_statistics::resolve(raw), false)
}
/// Explicit Source A retains actual calendar and Compact getter operands.
pub(crate) fn resolve_source_a(raw: &RawModel) -> Result<SelectedInputProcessor, String> {
    super::ordinary_equipment_input::resolve_source_a(raw)?;
    project(raw, input_statistics::resolve_source_a(raw)?, true)
}
/// Explicit SourceB keeps full raw operands on the same IP; no getter is bypassed.
pub(crate) fn resolve_source_b(raw: &RawModel) -> Result<SelectedInputProcessor, String> {
    super::ordinary_equipment_input::resolve_source_b(raw)?;
    project(raw, input_statistics::resolve_source_b(raw)?, true)
}
fn project(
    raw: &RawModel,
    statistics: ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsInput,
    source_json_order: bool,
) -> Result<SelectedInputProcessor, String> {
    let mut families = BTreeMap::new();
    for (family, objects) in &raw.objects {
        // initializeMaps stores DOM iterators in JSON map order, even when
        // getObjectItem subsequently uses a separate IDF declaration overlay.
        let ordered = if source_json_order {
            raw.source_json_instances(&family.0)?
        } else {
            objects.iter().collect()
        };
        let shapes = ordered
            .into_iter()
            .map(|(name, object)| SourceObjectShape {
                object_name: name.0.clone(),
                field_sizes: object
                    .fields
                    .iter()
                    .map(|(field, value)| (field.0.clone(), json_size(value)))
                    .collect(),
            })
            .collect();
        families.insert(family.0.clone(), SourceInputFamily::Objects(shapes));
    }
    let mut input = if source_json_order && raw.is_epjson() {
        SelectedInputProcessor::from_admitted_source_direct_json_input(
            schema::definitions(),
            families,
        )
    } else {
        SelectedInputProcessor::from_admitted_input(schema::definitions(), families)
    }
    .map_err(|error| error.to_string())?;
    if source_json_order && !raw.is_epjson() {
        for &family in ep_raw_model::SOURCE_IDF_OBJECT_FAMILIES {
            if raw
                .objects
                .get(&ep_raw_model::ObjectType(family.into()))
                .is_some_and(|rows| !rows.is_empty())
            {
                raw.validate_source_idf_object_custody(family)?;
                let (_, fields, extensions) = ep_raw_model::source_idf_object_layout(family)
                    .ok_or_else(|| format!("Source fixed layout absent: {family}"))?;
                input
                    .retain_source_original_idf_fields(
                        family,
                        fields
                            .iter()
                            .map(|(name, alpha, label)| ((*name).into(), *alpha, (*label).into()))
                            .collect(),
                        extensions
                            .iter()
                            .map(|(name, alpha, label)| ((*name).into(), *alpha, (*label).into()))
                            .collect(),
                    )
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    input
        .retain_source_input_statistics(statistics)
        .map_err(|error| error.to_string())?;
    if source_json_order
        && raw.is_epjson()
        && raw
            .objects
            .get(&ep_raw_model::ObjectType("Material:NoMass".into()))
            .is_some_and(|rows| !rows.is_empty())
    {
        input
            .retain_source_direct_json_no_mass_fields()
            .map_err(|error| error.to_string())?;
    }
    Ok(input)
}

#[cfg(test)]
#[path = "ordinary_input_processor_tests.rs"]
pub(crate) mod tests;
