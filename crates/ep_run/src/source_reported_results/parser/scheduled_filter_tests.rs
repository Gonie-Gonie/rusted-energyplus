use super::*;
use ep_runtime::OutputStoreType;
use ep_runtime::heat_balance::source_reported_catalog::SourceReportedFileState;
use std::sync::atomic::{AtomicU64, Ordering};

type TestResult = Result<(), Box<dyn std::error::Error>>;
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Directory(std::path::PathBuf);
impl Directory {
    fn new() -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "ep-scheduled-filter-parser-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn declaration(id: i32, schedule: Option<&str>) -> SourceReportedDeclaration {
    SourceReportedDeclaration {
        stream: Stream::Eso,
        report_id: id,
        key: Some("Environment".into()),
        name: "Site Outdoor Air Drybulb Temperature".into(),
        units: "C".into(),
        frequency: Frequency::Hourly,
        store_type: OutputStoreType::Average,
        kind: SourceDeclarationKind::Variable,
        reporting_schedule: schedule.map(str::to_owned),
        unreached_annual_reporting_boundary: false,
    }
}
fn catalog(declarations: Vec<SourceReportedDeclaration>, count: i64) -> SourceReportedCatalog {
    SourceReportedCatalog {
        declarations,
        stamp_report_numbers: [2, 2, 2, 3, 4, 5, 6],
        eso_record_count: count,
        mtr_record_count: 0,
        eso_file_state: SourceReportedFileState {
            closed: true,
            deleted_empty: count == 0,
        },
        mtr_file_state: SourceReportedFileState {
            closed: true,
            deleted_empty: true,
        },
        unresolved_requests: Vec::new(),
        report_during_warmup: false,
    }
}
fn physical(c: &SourceReportedCatalog) -> Result<String, Box<dyn std::error::Error>> {
    let mut lines = vec![
        "Program Version,authored parser input".to_owned(),
        "1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]".to_owned(),
    ];
    lines.extend(
        clock_rows(c, Stream::Eso)?
            .into_values()
            .map(|(_, line)| line),
    );
    lines.extend(
        c.declarations
            .iter()
            .filter(|d| d.stream == Stream::Eso)
            .map(SourceReportedDeclaration::dictionary_line)
            .collect::<Result<Vec<_>, _>>()?,
    );
    lines.extend([
        "End of Data Dictionary".to_owned(),
        "1,AUTHORED,39.74,-105.18,-7.00,1829.00".to_owned(),
        "2,1,1,1,0,1,0.00,60.00,Tuesday".to_owned(),
        "7,-4.5".to_owned(),
        "End of Data".to_owned(),
        " Number of Records Written=           1".to_owned(),
    ]);
    Ok(lines.join("\n") + "\n")
}

#[test]
fn scheduled_filter_completed_parser_retains_dictionary_and_zero_sample_series() -> TestResult {
    let directory = Directory::new()?;
    let eso = directory.0.join("own.eso");
    let mtr = directory.0.join("own.mtr");
    let mut c = catalog(
        vec![declaration(7, Some("ON")), declaration(8, Some("OFF"))],
        1,
    );
    let bytes = physical(&c)?;
    assert!(bytes.contains(
        "!Hourly,ON\n8,1,Environment,Site Outdoor Air Drybulb Temperature [C] !Hourly,OFF\n"
    ));
    std::fs::write(&eso, &bytes)?;
    let result = read_completed_source_reports(&eso, &mtr, &c)?;
    assert_eq!(result.eso.series.len(), 2);
    assert_eq!(result.eso.series[0].samples[0].reported_fields, ["-4.5"]);
    assert!(result.eso.series[1].samples.is_empty());
    assert_eq!(
        result.eso.series[1]
            .declaration
            .reporting_schedule
            .as_deref(),
        Some("OFF")
    );
    write_source_reported_artifacts(&directory.0.join("export"), &result)?;
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(
        directory.0.join("export/source-reported-results.json"),
    )?)?;
    assert_eq!(json["eso"]["series"][1]["reporting_schedule"], "OFF");
    assert_eq!(json["eso"]["series"][1]["sample_count"], 0);
    assert_eq!(json["eso"]["series"][1]["samples"], serde_json::json!([]));
    std::fs::write(&eso, bytes.replace("!Hourly,OFF", "!Hourly,OTHER"))?;
    assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    c.declarations[1].reporting_schedule = None;
    std::fs::write(&eso, physical(&c)?)?;
    // A missing unscheduled report remains invalid even beside an emitted report.
    assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    Ok(())
}

#[test]
fn scheduled_filter_deleted_stream_requires_actual_io_deletion_and_zero_counter() -> TestResult {
    let directory = Directory::new()?;
    let eso = directory.0.join("absent.eso");
    let mtr = directory.0.join("absent.mtr");
    let mut c = catalog(
        vec![declaration(8, Some("ZERO")), declaration(7, Some("OFF"))],
        0,
    );
    let result = read_completed_source_reports(&eso, &mtr, &c)?;
    assert!(result.eso.deleted_empty);
    assert_eq!(result.eso.record_count, 0);
    assert!(result.eso.series.is_empty());
    assert_eq!(result.eso.deleted_empty_declarations, c.declarations);
    write_source_reported_artifacts(&directory.0.join("export"), &result)?;
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(
        directory.0.join("export/source-reported-results.json"),
    )?)?;
    assert_eq!(
        json["eso"]["deleted_empty_declarations"][0]["reporting_schedule"],
        "ZERO"
    );
    assert!(
        json["eso"]["deleted_empty_declarations"][0]
            .get("samples")
            .is_none()
    );
    for (closed, deleted, count) in [(true, false, 0), (false, true, 0), (true, true, 1)] {
        c.eso_file_state = SourceReportedFileState {
            closed,
            deleted_empty: deleted,
        };
        c.eso_record_count = count;
        assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    }
    c.eso_file_state = SourceReportedFileState {
        closed: true,
        deleted_empty: true,
    };
    c.eso_record_count = 0;
    c.declarations[0].reporting_schedule = None;
    assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    Ok(())
}

#[test]
fn source_annual_empty_series_requires_unreached_actual_boundary() -> TestResult {
    let directory = Directory::new()?;
    let eso = directory.0.join("annual.eso");
    let mtr = directory.0.join("annual.mtr");
    let mut annual = declaration(8, None);
    annual.key = None;
    annual.name = "Electricity:Facility".into();
    annual.units = "J".into();
    annual.frequency = Frequency::Annual;
    annual.store_type = OutputStoreType::Sum;
    annual.kind = SourceDeclarationKind::Meter;
    annual.unreached_annual_reporting_boundary = true;
    let mut meter_partner = annual.clone();
    meter_partner.stream = Stream::Mtr;
    let mut c = catalog(
        vec![declaration(7, None), annual.clone(), meter_partner.clone()],
        1,
    );
    std::fs::write(&eso, physical(&c)?)?;
    let result = read_completed_source_reports(&eso, &mtr, &c)?;
    assert!(result.eso.series[1].samples.is_empty());
    assert!(
        result.eso.series[1]
            .declaration
            .unreached_annual_reporting_boundary
    );
    c.declarations[1].unreached_annual_reporting_boundary = false;
    assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    c.declarations[1].unreached_annual_reporting_boundary = true;
    c.declarations[1].frequency = Frequency::Monthly;
    std::fs::write(&eso, physical(&c)?)?;
    assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    c = catalog(vec![annual.clone(), meter_partner.clone()], 0);
    std::fs::remove_file(&eso)?;
    assert!(
        read_completed_source_reports(&eso, &mtr, &c)?
            .eso
            .deleted_empty
    );
    // Both physically deleted streams still require both exact declarations.
    for invalid in [
        {
            let mut v = annual.clone();
            v.unreached_annual_reporting_boundary = false;
            vec![v, meter_partner.clone()]
        },
        vec![annual.clone()],
        {
            let mut v = meter_partner.clone();
            v.name = "wrong meter".into();
            vec![annual.clone(), v]
        },
        {
            let mut v = meter_partner.clone();
            v.frequency = Frequency::Monthly;
            vec![annual.clone(), v]
        },
        {
            let mut v = meter_partner.clone();
            v.unreached_annual_reporting_boundary = false;
            vec![annual.clone(), v]
        },
    ] {
        let invalid_catalog = catalog(invalid, 0);
        assert!(read_completed_source_reports(&eso, &mtr, &invalid_catalog).is_err());
    }
    c.declarations[0].unreached_annual_reporting_boundary = false;
    assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    // The same Annual child follows variable reports even without any meter.
    let mut variable = declaration(8, None);
    variable.frequency = Frequency::Annual;
    variable.unreached_annual_reporting_boundary = true;
    c = catalog(vec![variable], 0);
    assert!(
        read_completed_source_reports(&eso, &mtr, &c)?
            .eso
            .deleted_empty
    );
    c.declarations[0].unreached_annual_reporting_boundary = false;
    assert!(read_completed_source_reports(&eso, &mtr, &c).is_err());
    Ok(())
}
