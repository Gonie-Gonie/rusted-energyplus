//! Project actual summary-report getter inputs after the sole ordinary admission.
use ep_raw_model::{FieldName, RawModel, RawValue};
use ep_runtime::heat_balance::selected_sizing::{SelectedSizingInput, SelectedSummaryReportInput};

pub(crate) fn resolve(raw: &RawModel) -> Result<SelectedSizingInput, String> {
    // The sole ordinary admission has already returned before this pure projection.
    let mut result = SelectedSizingInput::default();
    for (name, object) in raw
        .ordered_instances("Output:Table:SummaryReports")
        .map_err(|e| e.to_string())?
    {
        let values = match object.fields.get(&FieldName("reports".into())) {
            None | Some(RawValue::Null) => Vec::new(),
            Some(RawValue::Array(rows)) => rows
                .iter()
                .map(|row| {
                    let RawValue::Object(fields) = row else {
                        return Err("selected summary report item shape".into());
                    };
                    match fields.get(&FieldName("report_name".into())) {
                        None | Some(RawValue::Null) => Ok(String::new()),
                        Some(RawValue::String(value)) => Ok(value.clone()),
                        _ => Err("selected summary report name shape".into()),
                    }
                })
                .collect::<Result<Vec<_>, String>>()?,
            _ => return Err("selected summary reports array shape".into()),
        };
        result
            .summary_report_objects
            .push(SelectedSummaryReportInput {
                object_name: name.0.clone(),
                reports: values,
            });
    }
    Ok(result)
}
