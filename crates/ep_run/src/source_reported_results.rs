//! Complete own Source ESO/MTR artifact results, separate from legacy hourly stores.
//! This module is HELD and has no pipeline/runtime dispatch activation.
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
}
#[derive(Debug)]
pub(crate) struct SourceReportedStreams {
    pub eso: SourceReportedStream,
    pub mtr: SourceReportedStream,
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
        let other = mtr
            .series
            .iter()
            .find(|m| m.declaration.report_id == e.declaration.report_id)
            .ok_or_else(|| {
                invalid(
                    Stream::Mtr,
                    0,
                    "ESO meter is unpaired with the same actual MTR report ID",
                )
            })?;
        if e.declaration.key != other.declaration.key
            || e.declaration.name != other.declaration.name
            || e.declaration.units != other.declaration.units
            || e.declaration.frequency != other.declaration.frequency
            || e.declaration.store_type != other.declaration.store_type
            || e.declaration.kind != other.declaration.kind
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
    Ok(SourceReportedStreams { eso, mtr })
}
