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
            .source_ordered_instances(&family.0)?
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

// Source A's connected People and schedule getters need full operands and genuine original
// order. Other A statistics retain their existing projection; no schedule values
// or getter-owned state are calculated by this custody transfer.
pub(super) fn resolve_source_a(raw: &RawModel) -> Result<SourceInputStatisticsInput, String> {
    let mut result = resolve(raw);
    // The explicit Source order map covers every retained family, including
    // statistics-only operands. Numeric/value custody is unchanged below.
    for family in raw.objects.keys() {
        result.object_order.insert(
            family.0.clone(),
            raw.source_ordered_instances(&family.0)?
                .into_iter()
                .map(|(name, _)| name.0.clone())
                .collect(),
        );
    }
    for family in [
        "People",
        "ZoneList",
        "Space",
        "SpaceList",
        "ComfortViewFactorAngles",
        "Output:Schedules",
        "Schedule:Compact",
        "Schedule:File",
        "Schedule:Day:Hourly",
        "Schedule:Day:Interval",
        "Schedule:Day:List",
        "Schedule:Week:Compact",
        "Schedule:Week:Daily",
        "Schedule:Year",
    ] {
        let Some(objects) = raw.objects.get(&ep_raw_model::ObjectType(family.into())) else {
            continue;
        };
        result.objects.insert(
            family.into(),
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
    }
    Ok(result)
}
