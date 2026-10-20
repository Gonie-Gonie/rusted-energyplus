use super::*;
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
use crate::heat_balance::source_initialization::{
    SourceInitializationFilePaths, SourceInitializationFiles,
};
use crate::heat_balance::source_output_registration::{
    SourceOutVarReal, SourceOutputUnits, SourceRequestedVariable,
};

#[test]
fn scheduled_filter_catalog_reads_actual_registered_name_and_retains_report_ids()
-> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::PathBuf::from("unopened-catalog-metadata-test");
    let files = SourceInitializationFiles::new(SourceInitializationFilePaths {
        stat_input: path.clone(),
        eso: path.clone(),
        eio: path.clone(),
        mtr: path.clone(),
        bnd: path.clone(),
        dxf: path.clone(),
        rdd: path.clone(),
        mdd: path.clone(),
        audit: path.clone(),
        debug: path.clone(),
        end: path.clone(),
        error: path.clone(),
        dfs: path.clone(),
        tabular_csv: path.clone(),
        tabular_tab: path.clone(),
        tabular_fixed: path.clone(),
        tabular_html: path.clone(),
        tabular_xml: path,
    });
    let mut reports = CtfInitializationReports {
        selected_project_data: Some(SelectedProjectData::default()),
        ..Default::default()
    };
    reports
        .output_registration
        .requested_variables
        .push(SourceRequestedVariable {
            used: true,
            reporting_schedule: Some(SourceScheduleIndex(1)),
            reporting_schedule_name: "Constant-1.0".into(),
            ..Default::default()
        });
    reports.output_registration.out_vars.push(SourceOutVarReal {
        report: true,
        report_id: 7,
        key: "Environment".into(),
        name: "Site Outdoor Air Drybulb Temperature".into(),
        units: SourceOutputUnits::C,
        schedule: Some(SourceScheduleIndex(1)),
        ..Default::default()
    });
    let catalog = reports.source_reported_catalog(&files)?;
    assert_eq!(catalog.declarations.len(), 1);
    assert_eq!(
        catalog.declarations[0].reporting_schedule.as_deref(),
        Some("Constant-1.0")
    );
    assert_eq!(
        catalog.declarations[0].dictionary_line()?,
        "7,1,Environment,Site Outdoor Air Drybulb Temperature [C] !Hourly,Constant-1.0"
    );
    assert!(catalog.eso_file_state.closed);
    // Unopened is not evidence of the real empty-file deletion action.
    assert!(!catalog.eso_file_state.deleted_empty);
    reports.output_registration.requested_variables[0]
        .reporting_schedule_name
        .clear();
    assert!(reports.source_reported_catalog(&files).is_err());
    Ok(())
}
