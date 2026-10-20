//! Complete own Source ESO/MTR artifact results, separate from legacy hourly stores.
mod export;
mod parser;
use ep_runtime::heat_balance::source_reported_catalog::{
    SourceDeclarationKind, SourceDeclaredFrequency as Frequency, SourceDeclaredStream as Stream,
    SourceReportedCatalog, SourceReportedDeclaration,
};
pub(crate) use export::write_source_reported_artifacts;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) enum SourceReportedError {
    Io(std::io::Error),
    Invalid {
        stream: Stream,
        line: usize,
        message: String,
    },
}
impl std::fmt::Display for SourceReportedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => std::fmt::Display::fmt(e, f),
            Self::Invalid {
                stream,
                line,
                message,
            } => write!(
                f,
                "invalid completed Source {} at line {line}: {message}",
                stream.id()
            ),
        }
    }
}
impl std::error::Error for SourceReportedError {}
impl From<std::io::Error> for SourceReportedError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
fn invalid(stream: Stream, line: usize, message: impl Into<String>) -> SourceReportedError {
    SourceReportedError::Invalid {
        stream,
        line,
        message: message.into(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StampKind {
    Interval,
    Daily,
    Monthly,
    RunPeriod,
    Annual,
}
impl StampKind {
    fn id(&self) -> &'static str {
        match self {
            Self::Interval => "interval",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::RunPeriod => "run-period",
            Self::Annual => "annual",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReportEnvironment {
    pub ordinal: usize,
    pub fields: Vec<String>,
    pub physical_line: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReportStamp {
    pub report_id: i32,
    pub kind: StampKind,
    pub environment_ordinal: usize,
    pub fields: Vec<String>,
    pub physical_line: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReportSample {
    pub reported_fields: Vec<String>,
    pub value: f64,
    pub stamp_index: usize,
    pub physical_line: usize,
    pub stream_data_ordinal: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReportSeries {
    pub declaration: SourceReportedDeclaration,
    pub samples: Vec<ReportSample>,
}
#[derive(Debug)]
pub(crate) struct SourceReportedStream {
    pub stream: Stream,
    pub path: PathBuf,
    pub version_line: Option<String>,
    pub environments: Vec<ReportEnvironment>,
    pub stamps: Vec<ReportStamp>,
    pub series: Vec<ReportSeries>,
    pub record_count: u64,
    pub deleted_empty: bool,
    /// Actual registered metadata retained when CloseOutputFiles deleted all data.
    /// These declarations have no fabricated physical dictionary or samples.
    pub deleted_empty_declarations: Vec<SourceReportedDeclaration>,
}
#[derive(Debug)]
pub(crate) struct SourceReportedStreams {
    pub eso: SourceReportedStream,
    pub mtr: SourceReportedStream,
}

// Empty Annual is not a missing meter sample: its actual year-end report
// child was never reached. Both real stream declarations must still agree.
fn same_meter_declaration(a: &SourceReportedDeclaration, b: &SourceReportedDeclaration) -> bool {
    a.report_id == b.report_id
        && a.key == b.key
        && a.name == b.name
        && a.units == b.units
        && a.frequency == b.frequency
        && a.store_type == b.store_type
        && a.kind == b.kind
        && a.reporting_schedule == b.reporting_schedule
        && a.unreached_annual_reporting_boundary == b.unreached_annual_reporting_boundary
}
fn matching_unreached_annual_meter(
    d: &SourceReportedDeclaration,
    mtr: &SourceReportedStream,
) -> bool {
    if d.stream != Stream::Eso
        || d.frequency != Frequency::Annual
        || !d.unreached_annual_reporting_boundary
        || !matches!(
            d.kind,
            SourceDeclarationKind::Meter | SourceDeclarationKind::CumulativeMeter
        )
    {
        return false;
    }
    let partner = if mtr.deleted_empty {
        if mtr.record_count != 0 || !mtr.series.is_empty() {
            return false;
        }
        mtr.deleted_empty_declarations
            .iter()
            .find(|m| m.report_id == d.report_id)
    } else {
        mtr.series
            .iter()
            .find(|m| m.declaration.report_id == d.report_id && m.samples.is_empty())
            .map(|m| &m.declaration)
    };
    partner.is_some_and(|other| {
        other.stream == Stream::Mtr
            && other.unreached_annual_reporting_boundary
            && same_meter_declaration(d, other)
    })
}

/// Caller must first propagate actual successful ManageSimulation620. Footers
/// independently certify only completed reported streams, not whole simulation.
/// Paths must be exact borrows of the existing SourceInitializationFiles owner.
pub(crate) fn read_completed_source_reports(
    eso_path: &Path,
    mtr_path: &Path,
    catalog: &SourceReportedCatalog,
) -> Result<SourceReportedStreams, SourceReportedError> {
    if catalog.report_during_warmup {
        return Err(invalid(
            Stream::Eso,
            0,
            "warmup-report phase has no unambiguous phase partition in the retained ESO tuples",
        ));
    }
    if !catalog.unresolved_requests.is_empty() {
        return Err(invalid(
            Stream::Eso,
            0,
            format!(
                "{} actual requested variable identities remain unresolved",
                catalog.unresolved_requests.len()
            ),
        ));
    }
    let eso = parser::read_stream(eso_path, Stream::Eso, catalog)?;
    let mtr = parser::read_stream(mtr_path, Stream::Mtr, catalog)?;
    for e in &eso.series {
        if e.declaration.kind == SourceDeclarationKind::Variable {
            continue;
        }
        let Some(other) = mtr
            .series
            .iter()
            .find(|m| m.declaration.report_id == e.declaration.report_id)
        else {
            if e.samples.is_empty() && matching_unreached_annual_meter(&e.declaration, &mtr) {
                continue;
            }
            return Err(invalid(
                Stream::Mtr,
                0,
                "ESO meter is unpaired with the same actual MTR report ID",
            ));
        };
        if !same_meter_declaration(&e.declaration, &other.declaration)
            || e.samples.len() != other.samples.len()
        {
            return Err(invalid(
                Stream::Mtr,
                0,
                "shared ESO/MTR meter declaration or sample count differs",
            ));
        }
        for (a, b) in e.samples.iter().zip(&other.samples) {
            let sa = &eso.stamps[a.stamp_index];
            let sb = &mtr.stamps[b.stamp_index];
            if a.reported_fields != b.reported_fields
                || sa.report_id != sb.report_id
                || sa.kind != sb.kind
                || sa.environment_ordinal != sb.environment_ordinal
                || sa.fields != sb.fields
                || eso.environments[sa.environment_ordinal - 1].fields
                    != mtr.environments[sb.environment_ordinal - 1].fields
            {
                return Err(invalid(
                    Stream::Mtr,
                    b.physical_line,
                    "shared ESO/MTR meter raw value/stamp/environment tuples differ",
                ));
            }
        }
    }
    for declaration in &eso.deleted_empty_declarations {
        if declaration.kind == SourceDeclarationKind::Variable {
            continue;
        }
        if !eso.deleted_empty
            || eso.record_count != 0
            || !eso.series.is_empty()
            || !matching_unreached_annual_meter(declaration, &mtr)
        {
            return Err(invalid(
                Stream::Mtr,
                0,
                "deleted-empty ESO Annual meter lacks the same unreached actual MTR declaration",
            ));
        }
    }
    Ok(SourceReportedStreams { eso, mtr })
}
