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

    // This authored closed-stream child checks parser custody, not successful
    // ManageSimulation execution. Every warmup/regular row remains in order.
    let mut warmup = catalog(vec![declaration(7, None), declaration(8, Some("OFF"))], 4);
    warmup.report_during_warmup = true;
    let original_body =
        "1,AUTHORED,39.74,-105.18,-7.00,1829.00\n2,1,1,1,0,1,0.00,60.00,Tuesday\n7,-4.5\n";
    let phase_body = concat!(
        "1,Warmup {1} ,39.74,-105.18,-7.00,1829.00\n",
        "2,0,1,1,0,1,0.00,60.00,Tuesday\n7,-4.50\n",
        "1,Warmup {2} AUTHORED,39.74,-105.18,-7.00,1829.00\n",
        "2,0,1,1,0,1,0.00,60.00,Tuesday\n7,-3.250\n",
        // An ordinary name can look like the warmup prefix, even with the flag.
        "1,Warmup {7} AUTHORED,39.74,-105.18,-7.00,1829.00\n",
        "2,1,1,1,0,1,0.00,60.00,Tuesday\n7,-2.0\n",
        "2,2,1,2,0,1,0.00,60.00,Wednesday\n7,-1.00\n",
        "1,Warmup {9} HEADER ONLY,39.74,-105.18,-7.00,1829.00\n",
    );
    let warmup_bytes = physical(&warmup)?
        .replace(original_body, phase_body)
        .replace(
            "Number of Records Written=           1",
            "Number of Records Written=           4",
        );
    std::fs::write(&eso, &warmup_bytes)?;
    let result = read_completed_source_reports(&eso, &mtr, &warmup)?;
    assert_eq!(result.eso.record_count, 4);
    assert_eq!(result.eso.environments.len(), 4);
    assert_eq!(result.eso.environments[0].fields[0], "Warmup {1} ");
    assert_eq!(
        result.eso.environments[3].fields[0],
        "Warmup {9} HEADER ONLY"
    );
    assert_eq!(
        result
            .eso
            .stamps
            .iter()
            .map(|row| row.environment_ordinal)
            .collect::<Vec<_>>(),
        [1, 2, 3, 3]
    );
    assert_eq!(
        result
            .eso
            .stamps
            .iter()
            .map(|row| row.fields[0].as_str())
            .collect::<Vec<_>>(),
        ["0", "0", "1", "2"]
    );
    let samples = &result.eso.series[0].samples;
    assert_eq!(
        samples
            .iter()
            .map(|row| row.reported_fields[0].as_str())
            .collect::<Vec<_>>(),
        ["-4.50", "-3.250", "-2.0", "-1.00"]
    );
    assert_eq!(
        samples
            .iter()
            .map(|row| row.stream_data_ordinal)
            .collect::<Vec<_>>(),
        [1, 2, 3, 4]
    );
    for (index, sample) in samples.iter().enumerate() {
        assert_eq!(sample.stamp_index, index);
        let raw_sample = format!("7,{}", sample.reported_fields[0]);
        assert_eq!(
            warmup_bytes.lines().nth(sample.physical_line - 1),
            Some(raw_sample.as_str())
        );
    }
    assert!(result.eso.series[1].samples.is_empty());
    write_source_reported_artifacts(&directory.0.join("warmup-export"), &result)?;
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(
        directory
            .0
            .join("warmup-export/source-reported-results.json"),
    )?)?;
    assert_eq!(
        json["schema"],
        "rusted-energyplus.source-reported-streams.v1"
    );
    assert_eq!(
        json["eso"]["environment_tuples"]
            .as_array()
            .ok_or("environment tuples")?
            .len(),
        4
    );
    assert_eq!(json["eso"]["timestamp_tuples"][0]["fields"][0], "0");
    assert_eq!(
        json["eso"]["series"][0]["samples"][3]["stream_data_ordinal"],
        4
    );
    let csv = std::fs::read_to_string(directory.0.join("warmup-export/source-eso.csv"))?;
    assert!(csv.contains("Warmup {1} ") && csv.contains("-4.50") && csv.contains("-1.00"));
    warmup.report_during_warmup = false;
    assert!(read_completed_source_reports(&eso, &mtr, &warmup).is_err());
    warmup.report_during_warmup = true;
    for invalid in [
        warmup_bytes.replace("Warmup {1} ", "Warmup {0} "),
        warmup_bytes.replace("Warmup {1} ", "Warmup {01} "),
        warmup_bytes.replace("Warmup {1} ", "Warmup {+1} "),
        warmup_bytes.replace("Warmup {1} ", "Warmup 1 "),
        warmup_bytes.replace("Warmup {1} ", "warmup {1} "),
        warmup_bytes.replace("2,0,1,1,0,1,", "2,-1,1,1,0,1,"),
        // A title is not a license for an in-place zero/positive transition.
        warmup_bytes.replace("2,2,1,2,0,1,", "2,0,1,2,0,1,"),
        warmup_bytes.replace("7,-4.50\n", "7,-4.50\n2,1,1,1,0,2,0.00,60.00,Tuesday\n"),
        warmup_bytes.replace(
            "Number of Records Written=           4",
            "Number of Records Written=           3",
        ),
    ] {
        std::fs::write(&eso, invalid)?;
        assert!(read_completed_source_reports(&eso, &mtr, &warmup).is_err());
    }
    let regular_prefix = catalog(vec![declaration(7, None)], 1);
    std::fs::write(
        &eso,
        physical(&regular_prefix)?.replace("1,AUTHORED,", "1,Warmup {9} AUTHORED,"),
    )?;
    assert!(read_completed_source_reports(&eso, &mtr, &regular_prefix).is_ok());
    // Native can emit Daily/Monthly zero at warmup end-of-day/month. Annual
    // rows contain only a year: their environment is retained without a phase
    // inference. RunPeriod still requires its original positive cumulative day.
    for (frequency, stamp, operands, accepted) in [
        (
            Frequency::Daily,
            "3,0,12,31,0,Tuesday",
            "1,1,1,0,2,1,0",
            true,
        ),
        (Frequency::Monthly, "4,0,12", "1,1,31,1,0,2,31,1,0", true),
        (
            Frequency::Annual,
            "6,2026",
            "1,1,12,31,1,0,2,12,31,1,0",
            true,
        ),
        (
            Frequency::RunPeriod,
            "5,0",
            "1,1,12,31,1,0,2,12,31,1,0",
            false,
        ),
    ] {
        let mut row = declaration(7, None);
        row.frequency = frequency;
        let mut period = catalog(vec![row], 1);
        period.report_during_warmup = true;
        let period_body = format!(
            "1,Warmup {{1}} ,39.74,-105.18,-7.00,1829.00\n{stamp}\n7,{operands}\n1,UNCLASSIFIED HEADER ONLY,39.74,-105.18,-7.00,1829.00\n"
        );
        let period_bytes = physical(&period)?.replace(original_body, &period_body);
        std::fs::write(&eso, &period_bytes)?;
        let parsed = read_completed_source_reports(&eso, &mtr, &period);
        assert_eq!(parsed.is_ok(), accepted, "{frequency:?}");
        if accepted {
            let parsed = parsed?;
            assert_eq!(parsed.eso.environments.len(), 2);
            assert_eq!(
                parsed.eso.stamps[0].fields.join(","),
                stamp.split_once(',').ok_or("stamp")?.1
            );
            assert_eq!(
                parsed.eso.series[0].samples[0].reported_fields.join(","),
                operands
            );
        }
        period.report_during_warmup = false;
        assert_eq!(
            read_completed_source_reports(&eso, &mtr, &period).is_ok(),
            frequency == Frequency::Annual
        );
    }
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
