//! Pure raw JSON operand transfer for the genuine post-allocation IP statistics.
use ep_raw_model::{RawModel, RawValue};
use ep_runtime::heat_balance::selected_input_processor::{
    SourceInputStatisticsInput, SourceInputStatisticsValue,
};
fn value(raw: &RawValue) -> SourceInputStatisticsValue {
    match raw {
        RawValue::Null => SourceInputStatisticsValue::Null,
        RawValue::Bool(_) | RawValue::Number(_) => SourceInputStatisticsValue::OtherScalar,
        RawValue::String(text) => SourceInputStatisticsValue::String(text.clone()),
        RawValue::Array(items) => {
            SourceInputStatisticsValue::Array(items.iter().map(value).collect())
        }
        RawValue::Object(fields) => SourceInputStatisticsValue::Object(
            fields
                .iter()
                .map(|(name, raw)| (name.0.clone(), value(raw)))
                .collect(),
        ),
    }
}
pub(super) fn resolve(raw: &RawModel) -> SourceInputStatisticsInput {
    SourceInputStatisticsInput {
        object_order: Default::default(),
        objects: raw
            .objects
            .iter()
            .map(|(family, objects)| {
                (
                    family.0.clone(),
                    objects
                        .iter()
                        .map(|(name, object)| {
                            (
                                name.0.clone(),
                                object
                                    .fields
                                    .iter()
                                    .map(|(field, raw)| (field.0.clone(), value(raw)))
                                    .collect(),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
    }
}

// Exact SourceB raw values for subsequent genuine getters. The existing
// statistics-only A/None projection above stays unchanged.
fn full_value(raw: &RawValue) -> SourceInputStatisticsValue {
    match raw {
        RawValue::Null => SourceInputStatisticsValue::Null,
        RawValue::Bool(value) => SourceInputStatisticsValue::Bool(*value),
        RawValue::Number(text) => SourceInputStatisticsValue::Number(text.clone()),
        RawValue::String(text) => SourceInputStatisticsValue::String(text.clone()),
        RawValue::Array(items) => {
            SourceInputStatisticsValue::Array(items.iter().map(full_value).collect())
        }
        RawValue::Object(fields) => SourceInputStatisticsValue::Object(
            fields
                .iter()
                .map(|(name, value)| (name.0.clone(), full_value(value)))
                .collect(),
        ),
    }
}
pub(super) fn resolve_source_b(raw: &RawModel) -> Result<SourceInputStatisticsInput, String> {
    let mut object_order = std::collections::BTreeMap::new();
    for family in raw.objects.keys() {
        let names = raw
            .ordered_instances(&family.0)
            .map_err(|error| error.to_string())?
            .into_iter()
            .map(|(name, _)| name.0.clone())
            .collect();
        object_order.insert(family.0.clone(), names);
    }
    Ok(SourceInputStatisticsInput {
        object_order,
        objects: raw
            .objects
            .iter()
            .map(|(family, objects)| {
                (
                    family.0.clone(),
                    objects
                        .iter()
                        .map(|(name, object)| {
                            (
                                name.0.clone(),
                                object
                                    .fields
                                    .iter()
                                    .map(|(field, raw)| (field.0.clone(), full_value(raw)))
                                    .collect(),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
    })
}

// Source A's Compact producer needs full numeric operands and genuine original
// order. Other A statistics retain their existing projection; no schedule values
// or getter-owned state are calculated by this custody transfer.
pub(super) fn resolve_source_a(raw: &RawModel) -> Result<SourceInputStatisticsInput, String> {
    let mut result = resolve(raw);
    if let Some(objects) = raw
        .objects
        .get(&ep_raw_model::ObjectType("Schedule:Compact".into()))
    {
        // A retained order map must cover every family. Other A families keep
        // their previous lexical fallback order; Compact alone transfers its
        // actual declaration order together with complete numeric operands.
        result.object_order = result
            .objects
            .iter()
            .map(|(family, objects)| (family.clone(), objects.keys().cloned().collect()))
            .collect();
        result.objects.insert(
            "Schedule:Compact".into(),
            objects
                .iter()
                .map(|(name, object)| {
                    (
                        name.0.clone(),
                        object
                            .fields
                            .iter()
                            .map(|(field, value)| (field.0.clone(), full_value(value)))
                            .collect(),
                    )
                })
                .collect(),
        );
        result.object_order.insert(
            "Schedule:Compact".into(),
            raw.ordered_instances("Schedule:Compact")
                .map_err(|error| error.to_string())?
                .into_iter()
                .map(|(name, _)| name.0.clone())
                .collect(),
        );
    }
    Ok(result)
}
