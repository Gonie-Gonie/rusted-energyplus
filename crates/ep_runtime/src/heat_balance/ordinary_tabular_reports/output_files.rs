//! Actual OpenOutputTabularFile3027–3155, open_tbl_stream198–210 and five owners.
use super::{Error, SourceTabularReports};
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherSession};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
mod escaped;

/// Native OutputReportTabular.hh1116–1122 owns five distinct default ofstreams.
/// The ordinal pointer array addresses these members; it is not keyed by format.
#[derive(Debug, Default)]
pub(super) struct SourceTabularOutputStreams {
    streams: [SourceTabularOutputStream; 5],
}
#[derive(Debug)]
struct SourceTabularOutputStream {
    file: Option<File>,
    failed: bool,
    fill: u8,
}
impl Default for SourceTabularOutputStream {
    fn default() -> Self {
        Self {
            file: None,
            failed: false,
            fill: b' ',
        }
    }
}
impl SourceTabularOutputStream {
    fn open(&mut self, path: &Path, enabled: bool) -> Result<(), Error> {
        if enabled {
            if self.file.is_some() {
                self.failed = true;
                return Err(Error::Unavailable(
                    "actual repeated ofstream.open failure before ShowFatalError204",
                ));
            }
            match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
            {
                Ok(file) => self.file = Some(file),
                Err(error) => {
                    self.failed = true;
                    // Preserve the actual failed member and lossless OS cause.
                    // Native next calls ShowFatalError204; its live fatal/abort
                    // route is unrepresented, rather than using legacy flags.
                    return Err(Error::Io(error));
                }
            }
        } else {
            self.failed = true; // Actual open_tbl_stream207 badbit, no open.
        }
        Ok(())
    }
    fn bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        if self.failed {
            return Ok(());
        } // Native sentry rejects failed stream.
        let file = self.file.as_mut().ok_or(Error::Unavailable(
            "actual unopened TabularOutputFile insertion",
        ))?;
        #[cfg(windows)]
        let text = {
            let mut text = Vec::with_capacity(bytes.len());
            for &byte in bytes {
                if byte == b'\n' {
                    text.push(b'\r');
                }
                text.push(byte);
            }
            text
        };
        #[cfg(windows)]
        let bytes = text.as_slice();
        if let Err(error) = file.write_all(bytes) {
            self.failed = true;
            // Stop on the actual OS cause; exceptional subsequent Native
            // nonthrowing insertion continuation is explicitly not claimed.
            return Err(Error::Io(error));
        }
        Ok(())
    }
    fn integer(&mut self, value: i32, width: usize) -> Result<(), Error> {
        let value = value.to_string();
        for _ in value.len()..width {
            self.bytes(&[self.fill])?;
        }
        self.bytes(value.as_bytes())
    }
}
struct SourceTabularOpenOwners<'a, 'backend> {
    files: &'a SourceInitializationFiles,
    reports: &'a CtfInitializationReports,
    weather: &'a WeatherSession,
    backend: &'a mut SourceErrorBackend<'backend>,
}
fn display(o: &mut SourceTabularOpenOwners<'_, '_>, text: &str) -> Result<(), Error> {
    let globals = o
        .reports
        .selected_heat_balance_globals
        .as_ref()
        .ok_or(Error::Unavailable(
            "actual Globals before tabular DisplayString",
        ))?;
    let project = o
        .reports
        .selected_project_data
        .as_ref()
        .ok_or(Error::Unavailable(
            "actual SysVars before tabular DisplayString",
        ))?;
    o.backend.display_source_string(
        text,
        &SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &project.system.developer_flag,
            warmup: &o.weather.state.global.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        },
    )
}
fn td(values: &[Option<i32>; 8], index: usize) -> Result<i32, Error> {
    values[index].ok_or(Error::Unavailable(
        "actual date_and_time-written TAB td cell",
    ))
}
fn date(stream: &mut SourceTabularOutputStream, values: &[Option<i32>; 8]) -> Result<(), Error> {
    stream.integer(td(values, 0)?, 4)?;
    stream.bytes(b"-")?;
    stream.fill = b'0';
    stream.integer(td(values, 1)?, 2)?;
    stream.bytes(b"-")?;
    stream.integer(td(values, 2)?, 2)?;
    stream.bytes(b"\n")
}
fn time(stream: &mut SourceTabularOutputStream, values: &[Option<i32>; 8]) -> Result<(), Error> {
    stream.integer(td(values, 4)?, 2)?;
    stream.bytes(b":")?;
    stream.integer(td(values, 5)?, 2)?;
    stream.bytes(b":")?;
    stream.integer(td(values, 6)?, 2)?;
    stream.fill = b' ';
    Ok(())
}
fn environment_parts(
    stream: &mut SourceTabularOutputStream,
    prefix: &[u8],
    delimiter: &[u8],
    environment: &str,
    title: Option<&str>,
    suffix: &[u8],
    html: bool,
) -> Result<(), Error> {
    stream.bytes(prefix)?;
    stream.bytes(delimiter)?;
    if html {
        stream.bytes(&escaped::convert_to_escaped(environment.as_bytes(), false)?)?;
    } else {
        stream.bytes(environment.as_bytes())?;
    }
    if let Some(title) = title {
        stream.bytes(b" ** ")?;
        if html {
            stream.bytes(&escaped::convert_to_escaped(title.as_bytes(), false)?)?;
        } else {
            stream.bytes(title.as_bytes())?;
        }
    }
    stream.bytes(suffix)
}
fn environment_line(
    stream: &mut SourceTabularOutputStream,
    prefix: &[u8],
    delimiter: &[u8],
    environment: &str,
    title: &str,
    suffix: &[u8],
    html: bool,
) -> Result<(), Error> {
    // Each Native if compares the two live strings before the first insertion.
    if environment == title {
        environment_parts(stream, prefix, delimiter, environment, None, suffix, html)
    } else {
        environment_parts(
            stream,
            prefix,
            delimiter,
            environment,
            Some(title),
            suffix,
            html,
        )
    }
}
fn title_line(
    stream: &mut SourceTabularOutputStream,
    building: &str,
    environment: &str,
    title: &str,
) -> Result<(), Error> {
    if environment == title {
        stream.bytes(b"<title> ")?;
        stream.bytes(building.as_bytes())?;
        stream.bytes(b" ")?;
        stream.bytes(environment.as_bytes())?;
        stream.bytes(b"\n")
    } else {
        stream.bytes(b"<title> ")?;
        stream.bytes(building.as_bytes())?;
        stream.bytes(b" ")?;
        stream.bytes(environment.as_bytes())?;
        stream.bytes(b" ** ")?;
        stream.bytes(title.as_bytes())?;
        stream.bytes(b"\n")
    }
}
impl SourceTabularReports {
    fn open_source_tabular_files(
        &mut self,
        o: &mut SourceTabularOpenOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.input_state.write_tabular_files && o.reports.source_output_control.tabular {
            for number in 1..=self.input_state.number_of_styles {
                let at = number
                    .checked_sub(1)
                    .and_then(|n| usize::try_from(n).ok())
                    .ok_or(Error::Unavailable("actual style ordinal3049"))?;
                let cur_del = self
                    .input_state
                    .delimiters
                    .get(at)
                    .ok_or(Error::Unavailable("actual del3049 ordinal domain"))?;
                let style = *self
                    .input_state
                    .table_styles
                    .get(at)
                    .ok_or(Error::Unavailable("actual TableStyle3050 ordinal domain"))?;
                let message = match style {
                    0 => "Writing tabular output file results using comma format.",
                    1 => "Writing tabular output file results using tab format.",
                    3 => "Writing tabular output file results using HTML format.",
                    4 => "Writing tabular output file results using XML format.",
                    _ => "Writing tabular output file results using text format.",
                };
                display(o, message)?;
                let stream = self
                    .output_streams
                    .streams
                    .get_mut(at)
                    .ok_or(Error::Unavailable(
                        "actual TabularOutputFile200 ordinal domain",
                    ))?;
                stream.open(
                    o.files.source_tabular_output_path(style),
                    o.reports.source_output_control.tabular,
                )?;
                let environment = &o.weather.state.environment.environment_name;
                let weather_title = &o.weather.cursor.header().weather_location_title;
                let building = &o.reports.heat_balance_input_state.building_name;
                if style == 0 || style == 1 {
                    stream.bytes(if style == 0 {
                        b"Program Version:"
                    } else {
                        b"Program Version"
                    })?;
                    stream.bytes(cur_del.as_bytes())?;
                    stream.bytes(o.backend.state.version_string.as_bytes())?;
                    stream.bytes(b"\n")?;
                    stream.bytes(b"Tabular Output Report in Format: ")?;
                    stream.bytes(cur_del.as_bytes())?;
                    stream.bytes(if style == 0 { b"Comma\n" } else { b"Tab\n" })?;
                    stream.bytes(b"\n")?;
                    stream.bytes(b"Building:")?;
                    stream.bytes(cur_del.as_bytes())?;
                    stream.bytes(building.as_bytes())?;
                    stream.bytes(b"\n")?;
                    environment_line(
                        stream,
                        b"Environment:",
                        cur_del.as_bytes(),
                        environment,
                        weather_title,
                        b"\n",
                        false,
                    )?;
                    stream.bytes(b"\n")?;
                } else if style == 3 {
                    stream.bytes(b"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\"\"http://www.w3.org/TR/html4/loose.dtd\">\n")?;
                    stream.bytes(b"<html>\n")?;
                    stream.bytes(b"<head>\n")?;
                    title_line(stream, building, environment, weather_title)?;
                    stream.bytes(b"  ")?;
                    date(stream, &self.input_state.td)?;
                    stream.bytes(b"  ")?;
                    time(stream, &self.input_state.td)?;
                    stream.bytes(b"\n")?;
                    stream.bytes(b" - EnergyPlus</title>\n")?;
                    stream.bytes(b"<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">\n")?;
                    stream.bytes(b"</head>\n")?;
                    stream.bytes(b"<body>\n")?;
                    stream.bytes(
                        b"<p><a href=\"#toc\" style=\"float: right\">Table of Contents</a></p>\n",
                    )?;
                    stream.bytes(b"<a name=top></a>\n")?;
                    stream.bytes(b"<p>Program Version:<b>")?;
                    stream.bytes(o.backend.state.version_string.as_bytes())?;
                    stream.bytes(b"</b></p>\n")?;
                    stream.bytes(b"<p>Tabular Output Report in Format: <b>HTML</b></p>\n")?;
                    stream.bytes(b"<p>Building: <b>")?;
                    stream.bytes(building.as_bytes())?;
                    stream.bytes(b"</b></p>\n")?;
                    environment_line(
                        stream,
                        b"<p>Environment: <b>",
                        b"",
                        environment,
                        weather_title,
                        b"</b></p>\n",
                        true,
                    )?;
                    stream.bytes(b"<p>Simulation Timestamp: <b>")?;
                    date(stream, &self.input_state.td)?;
                    stream.bytes(b"  ")?;
                    time(stream, &self.input_state.td)?;
                    stream.bytes(b"</b></p>\n")?;
                } else if style == 4 {
                    stream.bytes(b"<?xml version=\"1.0\"?>\n")?;
                    stream.bytes(b"<EnergyPlusTabularReports>\n")?;
                    stream.bytes(b"  <BuildingName>")?;
                    stream.bytes(&escaped::convert_to_escaped(building.as_bytes(), true)?)?;
                    stream.bytes(b"</BuildingName>\n")?;
                    stream.bytes(b"  <EnvironmentName>")?;
                    stream.bytes(environment.as_bytes())?;
                    stream.bytes(b"</EnvironmentName>\n")?;
                    stream.bytes(b"  <WeatherFileLocationTitle>")?;
                    stream.bytes(weather_title.as_bytes())?;
                    stream.bytes(b"</WeatherFileLocationTitle>\n")?;
                    stream.bytes(b"  <ProgramVersion>")?;
                    stream.bytes(o.backend.state.version_string.as_bytes())?;
                    stream.bytes(b"</ProgramVersion>\n")?;
                    stream.bytes(b"  <SimulationTimestamp>\n")?;
                    stream.bytes(b"    <Date>\n")?;
                    stream.bytes(b"      ")?;
                    date(stream, &self.input_state.td)?;
                    stream.bytes(b"    </Date>\n")?;
                    stream.bytes(b"    <Time>\n")?;
                    stream.bytes(b"      ")?;
                    time(stream, &self.input_state.td)?;
                    stream.bytes(b"\n")?;
                    stream.bytes(b"    </Time>\n")?;
                    stream.bytes(b"  </SimulationTimestamp>\n")?;
                    stream.bytes(b"\n")?;
                } else {
                    stream.bytes(b"Program Version: ")?;
                    stream.bytes(o.backend.state.version_string.as_bytes())?;
                    stream.bytes(b"\n")?;
                    stream.bytes(b"Tabular Output Report in Format: ")?;
                    stream.bytes(cur_del.as_bytes())?;
                    stream.bytes(b"Fixed\n")?;
                    stream.bytes(b"\n")?;
                    stream.bytes(b"Building:        ")?;
                    stream.bytes(building.as_bytes())?;
                    stream.bytes(b"\n")?;
                    environment_line(
                        stream,
                        b"Environment:     ",
                        b"",
                        environment,
                        weather_title,
                        b"\n",
                        false,
                    )?;
                    stream.bytes(b"\n")?;
                }
            }
        }
        Ok(())
    }
}
impl SourceInitializationFiles {
    /// Actual594 entry while the original caller already borrows WeatherSession.
    /// # Errors
    /// Preserves typed causes and all reached stream/header effects.
    pub fn open_source_output_tabular_file_on_session(
        &self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &CtfInitializationReports,
        weather: &WeatherSession,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        let session = session.as_mut().ok_or_else(|| {
            RuntimeError::HeatBalanceInput(Box::new(Error::Unavailable(
                "same actual TAB owner before OpenOutputTabularFile594",
            )))
        })?;
        session
            .tabular
            .open_source_tabular_files(&mut SourceTabularOpenOwners {
                files: self,
                reports,
                weather,
                backend,
            })
            .map_err(hb_input_error)
    }
    /// Actual594 child using the same TAB streams, paths, TD and weather owners.
    /// # Errors
    /// Returns the typed child cause; failed physical open/write states persist.
    pub fn open_source_output_tabular_file(
        &self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &CtfInitializationReports,
        weather: &ProductionWeatherTimestepSeries,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|actual| {
            self.open_source_output_tabular_file_on_session(session, reports, actual, backend)
        })
    }
}
