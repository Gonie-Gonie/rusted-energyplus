//! Strict complete-stream catalog/timestamp/footer parser for own Source outputs.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
#[cfg(test)]
mod scheduled_filter_tests;
// A declared Annual series legitimately has no rows until its real year-end
// report child is reached. This fact is read from the actual completed RF frame;
// it never licenses missing other frequencies or a reached reporting boundary.
fn may_have_no_samples(d: &SourceReportedDeclaration) -> bool {
    d.reporting_schedule.is_some()
        || (d.unreached_annual_reporting_boundary && d.frequency == Frequency::Annual)
}
fn number(s: &str, stream: Stream, line: usize) -> Result<i32, SourceReportedError> {
    s.trim()
        .parse()
        .map_err(|_| invalid(stream, line, "invalid integer field"))
}
fn real(s: &str, stream: Stream, line: usize) -> Result<f64, SourceReportedError> {
    let n = s
        .trim()
        .parse::<f64>()
        .map_err(|_| invalid(stream, line, "invalid reported real field"))?;
    if !n.is_finite() {
        return Err(invalid(
            stream,
            line,
            "nonfinite reported real is unsupported, never replaced with blank",
        ));
    }
    Ok(n)
}
fn clock_rows(
    c: &SourceReportedCatalog,
    stream: Stream,
) -> Result<BTreeMap<i32, (StampKind, String)>, SourceReportedError> {
    let q = &c.stamp_report_numbers;
    if q[1] != q[2] {
        return Err(invalid(
            stream,
            0,
            "actual TimeStep/Hour stamp IDs do not share the Native chained assignment",
        ));
    }
    let suffix = if stream == Stream::Eso {
        "Report Variables Requested"
    } else {
        "Meters Requested"
    };
    let specs=[(q[1],StampKind::Interval,",8,Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],Hour[],StartMinute[],EndMinute[],DayType".to_string()),(q[3],StampKind::Daily,format!(",5,Cumulative Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],DayType  ! When Daily {suffix}")),(q[4],StampKind::Monthly,format!(",2,Cumulative Days of Simulation[],Month[]  ! When Monthly {suffix}")),(q[5],StampKind::RunPeriod,format!(",1,Cumulative Days of Simulation[] ! When Run Period {suffix}")),(q[6],StampKind::Annual,format!(",1,Calendar Year of Simulation[] ! When Annual {suffix}"))];
    let mut result = BTreeMap::new();
    for (id, kind, tail) in specs {
        if id <= 1 || result.insert(id, (kind, format!("{id}{tail}"))).is_some() {
            return Err(invalid(
                stream,
                0,
                "duplicate/invalid actual timestamp report IDs",
            ));
        }
    }
    Ok(result)
}
fn stamp_kind(f: Frequency) -> StampKind {
    match f {
        Frequency::EachCall | Frequency::TimeStep | Frequency::Hourly => StampKind::Interval,
        Frequency::Daily => StampKind::Daily,
        Frequency::Monthly => StampKind::Monthly,
        Frequency::RunPeriod => StampKind::RunPeriod,
        Frequency::Annual => StampKind::Annual,
    }
}
fn stamp_id(c: &SourceReportedCatalog, f: Frequency) -> i32 {
    c.stamp_report_numbers[if f == Frequency::EachCall {
        1
    } else {
        f.index()
    }]
}
fn validate_stamp(
    kind: &StampKind,
    fields: &[String],
    stream: Stream,
    line: usize,
    allow_warmup_day_zero: bool,
) -> Result<Option<bool>, SourceReportedError> {
    let expected = match kind {
        StampKind::Interval => 8,
        StampKind::Daily => 5,
        StampKind::Monthly => 2,
        StampKind::RunPeriod | StampKind::Annual => 1,
    };
    if fields.len() != expected {
        return Err(invalid(
            stream,
            line,
            "timestamp operand count differs from its real dictionary",
        ));
    }
    let integer = |i: usize| number(fields[i].as_str(), stream, line);
    // Only actual day-indexed tuples identify a phase. Annual year-only rows
    // and environment headers remain unclassified, with every raw field kept.
    let phase = match kind {
        StampKind::Annual => None,
        _ => Some(integer(0)? == 0),
    };
    let day_is_valid = || -> Result<bool, SourceReportedError> {
        let day = integer(0)?;
        Ok(day >= 1 || (day == 0 && allow_warmup_day_zero))
    };
    match kind {
        StampKind::Interval | StampKind::Daily => {
            if !day_is_valid()?
                || !(1..=12).contains(&integer(1)?)
                || !(1..=31).contains(&integer(2)?)
                || !(0..=1).contains(&integer(3)?)
            {
                return Err(invalid(
                    stream,
                    line,
                    "invalid reported day/month/date/DST tuple",
                ));
            }
            if *kind == StampKind::Interval {
                let start = real(&fields[5], stream, line)?;
                let end = real(&fields[6], stream, line)?;
                if !(1..=24).contains(&integer(4)?)
                    || start < 0.0
                    || end > 60.0
                    || start > end
                    || fields[7].is_empty()
                {
                    return Err(invalid(stream, line, "invalid reported interval tuple"));
                }
            } else if fields[4].is_empty() {
                return Err(invalid(stream, line, "empty reported day type"));
            }
        }
        StampKind::Monthly => {
            if !day_is_valid()? || !(1..=12).contains(&integer(1)?) {
                return Err(invalid(stream, line, "invalid monthly tuple"));
            }
        }
        StampKind::RunPeriod => {
            if integer(0)? < 1 {
                return Err(invalid(stream, line, "invalid cumulative run-period day"));
            }
        }
        StampKind::Annual => {
            integer(0)?;
        }
    }
    Ok(phase)
}
pub(super) fn read_stream(
    path: &Path,
    stream: Stream,
    c: &SourceReportedCatalog,
) -> Result<SourceReportedStream, SourceReportedError> {
    let file_state = if stream == Stream::Eso {
        c.eso_file_state
    } else {
        c.mtr_file_state
    };
    if !file_state.closed {
        return Err(invalid(
            stream,
            0,
            "actual Source output handle is not successfully closed",
        ));
    }
    let expected_count = u64::try_from(if stream == Stream::Eso {
        c.eso_record_count
    } else {
        c.mtr_record_count
    })
    .map_err(|_| invalid(stream, 0, "negative actual Source record counter"))?;
    let mut declared = BTreeMap::new();
    let mut identities = BTreeSet::new();
    for d in c.declarations.iter().filter(|d| d.stream == stream) {
        if d.report_id <= 0 || declared.insert(d.report_id, d).is_some() {
            return Err(invalid(
                stream,
                0,
                "duplicate or invalid actual series report ID",
            ));
        }
        if (d.kind == SourceDeclarationKind::Variable && d.key.is_none())
            || (d.kind != SourceDeclarationKind::Variable && d.key.is_some())
        {
            return Err(invalid(
                stream,
                0,
                "variable/meter key presence differs from actual declaration kind",
            ));
        }
        if !identities.insert((
            d.key.clone(),
            d.name.clone(),
            d.units.clone(),
            d.frequency,
            d.store_type.id(),
            d.kind.id(),
            d.reporting_schedule.clone(),
        )) {
            return Err(invalid(
                stream,
                0,
                "duplicate full key/name/units/frequency/store/kind identity is not silently collapsed",
            ));
        }
    }
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e)
            if e.kind() == std::io::ErrorKind::NotFound
                && expected_count == 0
                && file_state.deleted_empty
                && declared.values().copied().all(may_have_no_samples) =>
        {
            return Ok(SourceReportedStream {
                stream,
                path: path.to_owned(),
                version_line: None,
                environments: Vec::new(),
                stamps: Vec::new(),
                series: Vec::new(),
                record_count: 0,
                deleted_empty: true,
                deleted_empty_declarations: c
                    .declarations
                    .iter()
                    .filter(|d| d.stream == stream)
                    .cloned()
                    .collect(),
            });
        }
        Err(e) => return Err(e.into()),
    };
    if file_state.deleted_empty {
        return Err(invalid(
            stream,
            0,
            "physical stream exists after its actual owner deleted it",
        ));
    }
    if expected_count == 0 {
        return Err(invalid(
            stream,
            0,
            "zero-record completed Native stream should have been deleted by CloseOutputFiles",
        ));
    }
    if !bytes.ends_with(b"\n") {
        return Err(invalid(stream, 0, "incomplete final physical line"));
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| {
        invalid(
            stream,
            0,
            "nonUTF8 report metadata is unsupported without byte-preserving DTO",
        )
    })?;
    let mut lines = text
        .split_terminator('\n')
        .map(|s| s.strip_suffix('\r').unwrap_or(s))
        .enumerate();
    let (_, version) = lines
        .next()
        .ok_or_else(|| invalid(stream, 1, "missing version header"))?;
    if !version.starts_with("Program Version,") {
        return Err(invalid(stream, 1, "missing actual Program Version header"));
    }
    let clocks = clock_rows(c, stream)?;
    let environment_header =
        "1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]";
    let mut dictionary_ids = BTreeSet::new();
    let mut series = Vec::<ReportSeries>::new();
    let mut series_by_id = BTreeMap::new();
    let mut dictionary_done = false;
    let mut environments = Vec::<ReportEnvironment>::new();
    let mut stamps = Vec::<ReportStamp>::new();
    let mut current_stamp = None;
    // Transient witnesses belong to the current physical environment record,
    // never to a guessed WeatherManager ordinal or a copied scientific owner.
    let mut allow_warmup_day_zero = false;
    let mut environment_phase = None;
    let mut data_count = 0u64;
    let mut end_seen = false;
    let mut footer_seen = false;
    for (index, line) in lines {
        let n = index + 1;
        if footer_seen {
            return Err(invalid(
                stream,
                n,
                "trailing physical content after final record footer",
            ));
        }
        if !dictionary_done {
            if line == "End of Data Dictionary" {
                if dictionary_ids.len() != 1 + clocks.len() + declared.len() {
                    return Err(invalid(
                        stream,
                        n,
                        "dictionary is incomplete or unpaired with the actual Source catalog",
                    ));
                }
                dictionary_done = true;
                continue;
            }
            let (raw_id, _) = line
                .split_once(',')
                .ok_or_else(|| invalid(stream, n, "malformed dictionary record"))?;
            let id = number(raw_id, stream, n)?;
            if !dictionary_ids.insert(id) {
                return Err(invalid(stream, n, "duplicate dictionary report ID"));
            }
            if id == 1 {
                if line != environment_header {
                    return Err(invalid(
                        stream,
                        n,
                        "environment dictionary differs from Native header contract",
                    ));
                }
            } else if let Some((_, header)) = clocks.get(&id) {
                if line != header {
                    return Err(invalid(
                        stream,
                        n,
                        "timestamp dictionary differs from actual Source stamp ID/header",
                    ));
                }
            } else if let Some(d) = declared.get(&id) {
                if line
                    != d.dictionary_line()
                        .map_err(|e| invalid(stream, n, e.to_string()))?
                {
                    return Err(invalid(
                        stream,
                        n,
                        "dictionary identity/units/frequency/operand count differs from actual Source metadata",
                    ));
                }
                series_by_id.insert(id, series.len());
                series.push(ReportSeries {
                    declaration: (**d).clone(),
                    samples: Vec::new(),
                });
            } else {
                return Err(invalid(
                    stream,
                    n,
                    "unpaired dictionary record is not fabricated or discarded",
                ));
            }
            continue;
        }
        if end_seen {
            let tail = line
                .strip_prefix(" Number of Records Written=")
                .ok_or_else(|| invalid(stream, n, "missing exact completed record footer"))?;
            let footer = tail
                .trim()
                .parse::<u64>()
                .map_err(|_| invalid(stream, n, "invalid record footer counter"))?;
            if footer != data_count || footer != expected_count {
                return Err(invalid(
                    stream,
                    n,
                    format!(
                        "actual numeric records {data_count}, footer {footer}, and Source owner count {expected_count} disagree"
                    ),
                ));
            }
            footer_seen = true;
            continue;
        }
        if line == "End of Data" {
            end_seen = true;
            continue;
        }
        let (raw_id, payload) = line
            .split_once(',')
            .ok_or_else(|| invalid(stream, n, "malformed data record"))?;
        let id = number(raw_id, stream, n)?;
        if id == 1 {
            // Title can contain comma. The four final numeric fields cannot.
            let mut tail = payload
                .rsplitn(5, ',')
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if tail.len() != 5 {
                return Err(invalid(stream, n, "malformed environment tuple"));
            }
            tail.reverse();
            for field in &tail[1..] {
                real(field, stream, n)?;
            }
            // Native writes exactly "Warmup {positive decimal N} ", followed
            // by the original environment name (which may itself be empty).
            // A regular authored name may have this prefix too: only a real
            // day-zero tuple under the actual catalog flag is warmup evidence.
            allow_warmup_day_zero = c.report_during_warmup
                && tail[0]
                    .strip_prefix("Warmup {")
                    .and_then(|title| title.split_once("} "))
                    .is_some_and(|(day, _)| {
                        day.as_bytes()
                            .first()
                            .is_some_and(|first| matches!(*first, b'1'..=b'9'))
                            && day.bytes().all(|byte| byte.is_ascii_digit())
                    });
            environment_phase = None;
            environments.push(ReportEnvironment {
                ordinal: environments.len() + 1,
                fields: tail,
                physical_line: n,
            });
            current_stamp = None;
            continue;
        }
        if let Some((kind, _)) = clocks.get(&id) {
            if environments.is_empty() {
                return Err(invalid(
                    stream,
                    n,
                    "timestamp has no actual environment record",
                ));
            }
            let fields = payload.split(',').map(str::to_owned).collect::<Vec<_>>();
            if let Some(phase) = validate_stamp(kind, &fields, stream, n, allow_warmup_day_zero)? {
                if environment_phase.is_some_and(|previous| previous != phase) {
                    return Err(invalid(
                        stream,
                        n,
                        "reported warmup/regular day phase changed without a new physical environment record",
                    ));
                }
                environment_phase = Some(phase);
            }
            current_stamp = Some(stamps.len());
            stamps.push(ReportStamp {
                report_id: id,
                kind: kind.clone(),
                environment_ordinal: environments.len(),
                fields,
                physical_line: n,
            });
            continue;
        }
        let position = *series_by_id
            .get(&id)
            .ok_or_else(|| invalid(stream, n, "unknown/unpaired numeric report ID"))?;
        let declaration = &series[position].declaration;
        let fields = payload.split(',').map(str::to_owned).collect::<Vec<_>>();
        if fields.len() != declaration.data_items() {
            return Err(invalid(
                stream,
                n,
                "reported operand count differs from its real dictionary",
            ));
        }
        let stamp_index = current_stamp.ok_or_else(|| {
            invalid(
                stream,
                n,
                "reported value has no actual current stamp in this environment",
            )
        })?;
        if stamps[stamp_index].report_id != stamp_id(c, declaration.frequency)
            || stamps[stamp_index].kind != stamp_kind(declaration.frequency)
        {
            return Err(invalid(
                stream,
                n,
                "wrong or stale timestamp family for declared frequency",
            ));
        }
        if declaration.frequency == Frequency::Hourly
            && (real(&stamps[stamp_index].fields[5], stream, n)? != 0.0
                || real(&stamps[stamp_index].fields[6], stream, n)? != 60.0)
        {
            return Err(invalid(
                stream,
                n,
                "hourly output paired with a subhour stamp",
            ));
        }
        let value = real(&fields[0], stream, n)?;
        // Retain every emitted operand, including unrounded min/max/date text.
        for operand in &fields[1..] {
            real(operand, stream, n)?;
        }
        data_count = data_count
            .checked_add(1)
            .ok_or_else(|| invalid(stream, n, "record count overflow"))?;
        series[position].samples.push(ReportSample {
            reported_fields: fields,
            value,
            stamp_index,
            physical_line: n,
            stream_data_ordinal: data_count,
        });
    }
    if !dictionary_done || !end_seen || !footer_seen {
        return Err(invalid(
            stream,
            0,
            "partial stream: dictionary/end-of-data/record footer is missing",
        ));
    }
    if series
        .iter()
        .any(|s| s.samples.is_empty() && !may_have_no_samples(&s.declaration))
    {
        return Err(invalid(
            stream,
            0,
            "declared Source report has no paired samples",
        ));
    }
    Ok(SourceReportedStream {
        stream,
        path: path.to_owned(),
        version_line: Some(version.to_owned()),
        environments,
        stamps,
        series,
        record_count: data_count,
        deleted_empty: false,
        deleted_empty_declarations: Vec::new(),
    })
}
