//! Raw strings for the actual direct-JSON output pre-scan, before getter casing.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use ep_runtime::heat_balance::source_output_prescan::{
    SourcePreScanSummary, SourcePreScanVariable, SourceReportingPreScanInput,
};

pub(crate) fn resolve(raw: &RawModel) -> Result<SourceReportingPreScanInput, String> {
    resolve_ordered(raw, false)
}

pub(crate) fn resolve_source(raw: &RawModel) -> Result<SourceReportingPreScanInput, String> {
    resolve_ordered(raw, true)
}

fn resolve_ordered(
    raw: &RawModel,
    source_order: bool,
) -> Result<SourceReportingPreScanInput, String> {
    let mut input = SourceReportingPreScanInput::default();
    let variables = if source_order {
        raw.source_ordered_instances("Output:Variable")?
    } else {
        raw.objects
            .get(&ObjectType("Output:Variable".into()))
            .map(|objects| objects.iter().collect())
            .unwrap_or_default()
    };
    for (name, object) in variables {
        let key = match object.fields.get(&FieldName("key_value".into())) {
            None | Some(RawValue::Null) => "*",
            Some(RawValue::String(text)) if text.is_empty() => "*",
            Some(RawValue::String(text)) => text,
            _ => return Err("output pre-scan key must be a raw string".into()),
        };
        let Some(RawValue::String(variable_name)) =
            object.fields.get(&FieldName("variable_name".into()))
        else {
            return Err("output pre-scan variable_name must be a raw string".into());
        };
        input.variables.push(SourcePreScanVariable {
            object_name: name.0.clone(),
            key: key.into(),
            variable_name: variable_name.clone(),
        });
    }
    let summaries = if source_order {
        raw.source_ordered_instances("Output:Table:SummaryReports")?
    } else {
        raw.objects
            .get(&ObjectType("Output:Table:SummaryReports".into()))
            .map(|objects| objects.iter().collect())
            .unwrap_or_default()
    };
    for (name, object) in summaries {
        let mut report_names = Vec::new();
        if let Some(RawValue::Array(reports)) = object.fields.get(&FieldName("reports".into())) {
            for report in reports {
                let RawValue::Object(report) = report else {
                    continue;
                };
                if let Some(RawValue::String(report_name)) =
                    report.get(&FieldName("report_name".into()))
                {
                    report_names.push(report_name.clone());
                }
            }
        }
        input.summaries.push(SourcePreScanSummary {
            object_name: name.0.clone(),
            report_names,
        });
    }
    Ok(input)
}
