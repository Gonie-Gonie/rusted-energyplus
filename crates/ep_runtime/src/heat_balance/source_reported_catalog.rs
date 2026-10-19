//! Read-only metadata projection from the actual Source OutputProcessor owner.
//! No current value, accumulator, history, clock or scientific scalar is copied.
use super::ctf_initialization_context::CtfInitializationReports;
use super::selected_heat_balance_input::HeatBalanceInputError as Error;
use super::source_output_registration::{SourceOutputStoreType, SourceReportFrequency};
use crate::OutputStoreType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Physical Source output stream owning a report declaration.
pub enum SourceDeclaredStream {
    /// Standard ESO variable stream, which may also receive requested meter rows.
    Eso,
    /// Dedicated MTR stream for requested meters.
    Mtr,
}
impl SourceDeclaredStream {
    /// Stable lowercase stream identifier used by reported-output metadata.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Eso => "eso",
            Self::Mtr => "mtr",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Actual OutputProcessor reporting period, without relabeling one frequency as another.
pub enum SourceDeclaredFrequency {
    /// Each actual reporting call.
    EachCall,
    /// Completed zone timestep.
    TimeStep,
    /// Completed hour.
    Hourly,
    /// Completed day.
    Daily,
    /// Completed month.
    Monthly,
    /// Completed simulation environment or run period.
    RunPeriod,
    /// Completed simulation year.
    Annual,
}
impl SourceDeclaredFrequency {
    /// Stable metadata identifier for this exact reporting frequency.
    pub const fn id(self) -> &'static str {
        match self {
            Self::EachCall => "each-call",
            Self::TimeStep => "timestep",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::RunPeriod => "run-period",
            Self::Annual => "annual",
        }
    }
    /// Zero-based slot in the actual seven-frequency OutputProcessor arrays.
    pub const fn index(self) -> usize {
        match self {
            Self::EachCall => 0,
            Self::TimeStep => 1,
            Self::Hourly => 2,
            Self::Daily => 3,
            Self::Monthly => 4,
            Self::RunPeriod => 5,
            Self::Annual => 6,
        }
    }
    /// Native dictionary data-item count and frequency/date-column suffix.
    pub const fn dictionary(self) -> (usize, &'static str) {
        match self {
            Self::EachCall => (1, " !Each Call"),
            Self::TimeStep => (1, " !TimeStep"),
            Self::Hourly => (1, " !Hourly"),
            Self::Daily => (7, " !Daily [Value,Min,Hour,Minute,Max,Hour,Minute]"),
            Self::Monthly => (
                9,
                " !Monthly [Value,Min,Day,Hour,Minute,Max,Day,Hour,Minute]",
            ),
            Self::RunPeriod => (
                11,
                " !RunPeriod [Value,Min,Month,Day,Hour,Minute,Max,Month,Day,Hour,Minute]",
            ),
            Self::Annual => (
                11,
                " !Annual [Value,Min,Month,Day,Hour,Minute,Max,Month,Day,Hour,Minute]",
            ),
        }
    }
}
fn frequency(f: SourceReportFrequency) -> SourceDeclaredFrequency {
    match f {
        SourceReportFrequency::EachCall => SourceDeclaredFrequency::EachCall,
        SourceReportFrequency::TimeStep => SourceDeclaredFrequency::TimeStep,
        SourceReportFrequency::Hour => SourceDeclaredFrequency::Hourly,
        SourceReportFrequency::Day => SourceDeclaredFrequency::Daily,
        SourceReportFrequency::Month => SourceDeclaredFrequency::Monthly,
        SourceReportFrequency::Simulation => SourceDeclaredFrequency::RunPeriod,
        SourceReportFrequency::Year => SourceDeclaredFrequency::Annual,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Native declaration category, independent of its current numeric value.
pub enum SourceDeclarationKind {
    /// Requested report variable.
    Variable,
    /// Requested meter for the reporting period.
    Meter,
    /// Requested cumulative meter total.
    CumulativeMeter,
}
impl SourceDeclarationKind {
    /// Stable metadata identifier for this declaration category.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Variable => "variable",
            Self::Meter => "meter",
            Self::CumulativeMeter => "cumulative-meter",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Read-only declaration metadata borrowed from the completed Source OutputProcessor.
pub struct SourceReportedDeclaration {
    /// Actual physical stream receiving the declaration.
    pub stream: SourceDeclaredStream,
    /// Actual OutputProcessor report identifier.
    pub report_id: i32,
    /// Actual variable key; absent for meter declarations.
    pub key: Option<String>,
    /// Actual variable or meter name.
    pub name: String,
    /// Actual unit symbol supplied by registration.
    pub units: String,
    /// Actual registered reporting frequency.
    pub frequency: SourceDeclaredFrequency,
    /// Actual Average or Sum storage policy.
    pub store_type: OutputStoreType,
    /// Actual variable, meter or cumulative-meter category.
    pub kind: SourceDeclarationKind,
}
impl SourceReportedDeclaration {
    /// Number of value/date items in the native dictionary row, excluding its report ID.
    pub fn data_items(&self) -> usize {
        if self.kind == SourceDeclarationKind::CumulativeMeter {
            1
        } else {
            self.frequency.dictionary().0
        }
    }
    /// Metadata-only dictionary formatter matches the existing genuine writer.
    pub fn dictionary_line(&self) -> Result<String, Error> {
        let (_, notice) = self.frequency.dictionary();
        let line = match self.kind {
            SourceDeclarationKind::Variable => format!(
                "{},{},{},{} [{}]{}",
                self.report_id,
                self.data_items(),
                self.key
                    .as_deref()
                    .ok_or(Error::Unavailable("missing actual variable key metadata"))?,
                self.name,
                self.units,
                notice
            ),
            SourceDeclarationKind::Meter => format!(
                "{},{},{} [{}]{}",
                self.report_id,
                self.data_items(),
                self.name,
                self.units,
                notice
            ),
            SourceDeclarationKind::CumulativeMeter => format!(
                "{},1,Cumulative {} [{}]{}",
                self.report_id,
                self.name,
                self.units,
                notice.split_once('[').map_or(notice, |(left, _)| left)
            ),
        };
        Ok(line)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// A genuine requested variable that no actual registration marked as used.
pub struct SourceUnresolvedRequest {
    /// Requested key or wildcard pattern.
    pub key: String,
    /// Requested variable name.
    pub name: String,
    /// Requested reporting frequency.
    pub frequency: SourceDeclaredFrequency,
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Completed Source report metadata only; contains no copied scientific values.
pub struct SourceReportedCatalog {
    /// Actual enabled variable and meter declarations.
    pub declarations: Vec<SourceReportedDeclaration>,
    /// Actual OutputProcessor timestamp report IDs for its seven frequency slots.
    pub stamp_report_numbers: [i32; 7],
    /// Actual completed standard-output record counter.
    pub eso_record_count: i64,
    /// Actual completed meter-output record counter.
    pub mtr_record_count: i64,
    /// Actual requested variables still unused after registration.
    pub unresolved_requests: Vec<SourceUnresolvedRequest>,
    /// Actual SysVars warmup-reporting setting.
    pub report_during_warmup: bool,
}
impl CtfInitializationReports {
    /// After actual ManageSimulation620, borrow metadata from its same owners.
    /// This accessor neither establishes nor stores a completion flag. Its
    /// caller must already have propagated the genuine successful620 return.
    pub fn source_reported_catalog(&self) -> Result<SourceReportedCatalog, Error> {
        let op = &self.output_registration;
        let project = self
            .selected_project_data
            .as_ref()
            .ok_or(Error::Unavailable(
                "actual same SysVars for completed Source report metadata",
            ))?;
        let mut declarations = Vec::new();
        for var in &op.out_vars {
            if !var.report {
                continue;
            }
            if var.schedule.is_some() {
                return Err(Error::Unavailable(
                    "scheduled Source output history requires its actual reporting schedule contract",
                ));
            }
            declarations.push(SourceReportedDeclaration {
                stream: SourceDeclaredStream::Eso,
                report_id: var.report_id,
                key: Some(var.key.clone()),
                name: var.name.clone(),
                units: var.units.symbol().to_string(),
                frequency: frequency(var.frequency),
                store_type: match var.store_type {
                    SourceOutputStoreType::Average => OutputStoreType::Average,
                    SourceOutputStoreType::Sum => OutputStoreType::Sum,
                },
                kind: SourceDeclarationKind::Variable,
            });
        }
        const FREQUENCIES: [SourceReportFrequency; 7] = [
            SourceReportFrequency::EachCall,
            SourceReportFrequency::TimeStep,
            SourceReportFrequency::Hour,
            SourceReportFrequency::Day,
            SourceReportFrequency::Month,
            SourceReportFrequency::Simulation,
            SourceReportFrequency::Year,
        ];
        for meter in &op.meters.meters {
            for f in FREQUENCIES {
                let period = &meter.periods[f as usize];
                for (enabled, only, id, kind) in [
                    (
                        period.report,
                        period.report_file_only,
                        period.report_number,
                        SourceDeclarationKind::Meter,
                    ),
                    (
                        period.accumulated_report,
                        period.accumulated_file_only,
                        period.accumulated_report_number,
                        SourceDeclarationKind::CumulativeMeter,
                    ),
                ] {
                    if !enabled {
                        continue;
                    }
                    for stream in [SourceDeclaredStream::Mtr, SourceDeclaredStream::Eso] {
                        if stream == SourceDeclaredStream::Eso && only {
                            continue;
                        }
                        // Native meter dictionary declares StoreType::Sum. No
                        // name/unit inference or post-run value read occurs.
                        declarations.push(SourceReportedDeclaration {
                            stream,
                            report_id: id,
                            key: None,
                            name: meter.name.clone(),
                            units: meter.units.symbol().to_string(),
                            frequency: frequency(f),
                            store_type: OutputStoreType::Sum,
                            kind,
                        });
                    }
                }
            }
        }
        let unresolved_requests = op
            .requested_variables
            .iter()
            .filter(|r| !r.used)
            .map(|r| SourceUnresolvedRequest {
                key: r.key.clone(),
                name: r.name.clone(),
                frequency: frequency(r.frequency),
            })
            .collect();
        Ok(SourceReportedCatalog {
            declarations,
            stamp_report_numbers: op.frequency_stamp_report_numbers,
            eso_record_count: self.source_standard_output_record_count,
            mtr_record_count: self.source_standard_meter_record_count,
            unresolved_requests,
            report_during_warmup: project.system.report_during_warmup,
        })
    }
}

/// Immutable caller-scope metadata from the same actual WeatherManager owners.
/// It is not a scientific clock/history or an assertion of recorded frequency.
#[derive(Clone, Debug)]
pub struct SourceReportedEnvironmentScope {
    /// One-based ordinal of the actual WeatherManager environment row.
    pub ordinal: usize,
    /// Actual environment title.
    pub title: String,
    /// Pinned Native environment-kind ordinal.
    pub kind_of_environment: i32,
    /// Actual configured environment duration in days.
    pub total_days: i32,
}
#[derive(Clone, Debug)]
/// Immutable run-selection metadata from the real project and weather owners.
pub struct SourceReportedRunScope {
    /// Actual design-day simulation selection.
    pub do_des_day_sim: bool,
    /// Actual weather-run simulation selection.
    pub do_weath_sim: bool,
    /// Actual DDOnly project option.
    pub dd_only: bool,
    /// Actual FullAnnualRun project option.
    pub full_annual_run: bool,
    /// Actual allocated environment rows in their retained order.
    pub environments: Vec<SourceReportedEnvironmentScope>,
}
impl CtfInitializationReports {
    /// Read after the genuine primary/endrun caller; do not move getters earlier.
    /// Every copied member is immutable result metadata, not simulation state.
    pub fn source_reported_run_scope(
        &self,
        weather: &crate::weather::day::ProductionWeatherTimestepSeries,
    ) -> Result<SourceReportedRunScope, Error> {
        let project = self
            .selected_project_data
            .as_ref()
            .ok_or(Error::Unavailable(
                "actual Project owner for completed Source run scope",
            ))?;
        weather.with_source_weather_session_mut(|session| {
            let rows =
                session
                    .state
                    .weather
                    .source_environments
                    .as_ref()
                    .ok_or(Error::Unavailable(
                        "actual Environment allocation for completed Source run scope",
                    ))?;
            let count = usize::try_from(session.state.weather.num_of_envrn).map_err(|_| {
                Error::Unavailable("actual NumOfEnvrn for completed Source run scope")
            })?;
            if rows.len() != count {
                return Err(Error::Unavailable(
                    "actual Environment allocation/count mismatch for completed Source run scope",
                ));
            }
            Ok(SourceReportedRunScope {
                do_des_day_sim: session.state.global.do_des_day_sim,
                do_weath_sim: session.state.global.do_weath_sim,
                dd_only: project.system.dd_only,
                full_annual_run: project.system.full_annual_run,
                environments: rows
                    .iter()
                    .enumerate()
                    .map(|(index, row)| SourceReportedEnvironmentScope {
                        ordinal: index + 1,
                        title: row.title.clone(),
                        kind_of_environment: row.kind_of_environment,
                        total_days: row.total_days,
                    })
                    .collect(),
            })
        })
    }
}
