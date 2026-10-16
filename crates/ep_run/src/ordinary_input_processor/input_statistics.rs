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
