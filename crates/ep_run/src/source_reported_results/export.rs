//! Complete borrowed Source JSON serialization and unchanged raw-token CSV.
use super::*;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use std::io::Write;

// Every adapter borrows an already validated report. Only the serializer's
// current element and bounded output buffer exist in addition to that owner.
// Map keys deliberately match the former Value/BTreeMap lexical order.
struct BundleJson<'a>(&'a SourceReportedStreams);
struct StreamJson<'a>(&'a SourceReportedStream);
struct EnvironmentJson<'a>(&'a ReportEnvironment);
struct StampJson<'a>(&'a ReportStamp);
struct SeriesJson<'a>(&'a ReportSeries);
struct DeclarationJson<'a>(&'a SourceReportedDeclaration);
struct DeclarationsJson<'a>(&'a [SourceReportedDeclaration]);
struct SampleJson<'a>(&'a ReportSample);
struct EnvironmentsJson<'a>(&'a [ReportEnvironment]);
struct StampsJson<'a>(&'a [ReportStamp]);
struct SeriesListJson<'a>(&'a [ReportSeries]);
struct SamplesJson<'a>(&'a [ReportSample]);
struct Binary64Bits(u64);

impl Serialize for Binary64Bits {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&format_args!("{:016x}", self.0))
    }
}
impl Serialize for BundleJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("eso", &StreamJson(&self.0.eso))?;
        map.serialize_entry("mtr", &StreamJson(&self.0.mtr))?;
        map.serialize_entry("schema", "rusted-energyplus.source-reported-streams.v1")?;
        map.serialize_entry("timestamp_boundary", "actual ESO/MTR tuples; no civil year inferred for interval/daily/monthly/run-period records")?;
        map.serialize_entry(
            "value_boundary",
            "exact reported decimal tokens plus parsed binary64; not unformatted internal state",
        )?;
        map.end()
    }
}
impl Serialize for StreamJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0;
        let mut map = serializer.serialize_map(Some(
            8 + usize::from(!s.deleted_empty_declarations.is_empty()),
        ))?;
        map.serialize_entry("deleted_empty", &s.deleted_empty)?;
        if !s.deleted_empty_declarations.is_empty() {
            map.serialize_entry(
                "deleted_empty_declarations",
                &DeclarationsJson(&s.deleted_empty_declarations),
            )?;
        }
        map.serialize_entry("environment_tuples", &EnvironmentsJson(&s.environments))?;
        map.serialize_entry("path", &s.path)?;
        map.serialize_entry("reported_record_count", &s.record_count)?;
        map.serialize_entry("series", &SeriesListJson(&s.series))?;
        map.serialize_entry("stream", s.stream.id())?;
        map.serialize_entry("timestamp_tuples", &StampsJson(&s.stamps))?;
        map.serialize_entry("version_line", &s.version_line)?;
        map.end()
    }
}
impl Serialize for EnvironmentJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let e = self.0;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("fields", &e.fields)?;
        map.serialize_entry("ordinal", &e.ordinal)?;
        map.serialize_entry("physical_line", &e.physical_line)?;
        map.end()
    }
}
impl Serialize for StampJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let t = self.0;
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("environment_ordinal", &t.environment_ordinal)?;
        map.serialize_entry("fields", &t.fields)?;
        map.serialize_entry("kind", t.kind.id())?;
        map.serialize_entry("physical_line", &t.physical_line)?;
        map.serialize_entry("report_id", &t.report_id)?;
        map.end()
    }
}
impl Serialize for SeriesJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let r = self.0;
        let d = &r.declaration;
        let mut map =
            serializer.serialize_map(Some(9 + usize::from(d.reporting_schedule.is_some())))?;
        map.serialize_entry("frequency", d.frequency.id())?;
        map.serialize_entry("key", &d.key)?;
        map.serialize_entry("kind", d.kind.id())?;
        map.serialize_entry("name", &d.name)?;
        map.serialize_entry("report_id", &d.report_id)?;
        if let Some(schedule) = &d.reporting_schedule {
            map.serialize_entry("reporting_schedule", schedule)?;
        }
        map.serialize_entry("sample_count", &r.samples.len())?;
        map.serialize_entry("samples", &SamplesJson(&r.samples))?;
        map.serialize_entry("store_type", d.store_type.id())?;
        map.serialize_entry("units", &d.units)?;
        map.end()
    }
}
impl Serialize for DeclarationJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let d = self.0;
        let mut map =
            serializer.serialize_map(Some(7 + usize::from(d.reporting_schedule.is_some())))?;
        map.serialize_entry("frequency", d.frequency.id())?;
        map.serialize_entry("key", &d.key)?;
        map.serialize_entry("kind", d.kind.id())?;
        map.serialize_entry("name", &d.name)?;
        map.serialize_entry("report_id", &d.report_id)?;
        if let Some(schedule) = &d.reporting_schedule {
            map.serialize_entry("reporting_schedule", schedule)?;
        }
        map.serialize_entry("store_type", d.store_type.id())?;
        map.serialize_entry("units", &d.units)?;
        map.end()
    }
}
impl Serialize for DeclarationsJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for declaration in self.0 {
            seq.serialize_element(&DeclarationJson(declaration))?;
        }
        seq.end()
    }
}
impl Serialize for SampleJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("parsed_binary64_bits", &Binary64Bits(v.value.to_bits()))?;
        map.serialize_entry("physical_line", &v.physical_line)?;
        map.serialize_entry("reported_fields", &v.reported_fields)?;
        map.serialize_entry("stamp_index", &v.stamp_index)?;
        map.serialize_entry("stream_data_ordinal", &v.stream_data_ordinal)?;
        map.serialize_entry("value", &v.value)?;
        map.end()
    }
}
impl Serialize for EnvironmentsJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for row in self.0 {
            seq.serialize_element(&EnvironmentJson(row))?;
        }
        seq.end()
    }
}
impl Serialize for StampsJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for row in self.0 {
            seq.serialize_element(&StampJson(row))?;
        }
        seq.end()
    }
}
impl Serialize for SeriesListJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for row in self.0 {
            seq.serialize_element(&SeriesJson(row))?;
        }
        seq.end()
    }
}
impl Serialize for SamplesJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for row in self.0 {
            seq.serialize_element(&SampleJson(row))?;
        }
        seq.end()
    }
}
fn field(s: &str) -> String {
    if s.contains([',', '"', '\r', '\n']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_owned()
    }
}
fn write_csv(path: &Path, s: &SourceReportedStream) -> Result<(), SourceReportedError> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    writeln!(
        file,
        "stream,report_id,key,name,units,frequency,store_type,kind,sample_index,stream_data_ordinal,environment_ordinal,environment_fields_json,timestamp_report_id,timestamp_kind,timestamp_fields_json,reported_fields_json,reported_decimal,parsed_binary64_bits"
    )?;
    for series in &s.series {
        let d = &series.declaration;
        for (i, v) in series.samples.iter().enumerate() {
            let t = &s.stamps[v.stamp_index];
            let env_json = serde_json::to_string(&s.environments[t.environment_ordinal - 1].fields)
                .map_err(|e| invalid(s.stream, 0, e.to_string()))?;
            let t_json = serde_json::to_string(&t.fields)
                .map_err(|e| invalid(s.stream, 0, e.to_string()))?;
            let values_json = serde_json::to_string(&v.reported_fields)
                .map_err(|e| invalid(s.stream, 0, e.to_string()))?;
            let columns = [
                s.stream.id().to_owned(),
                d.report_id.to_string(),
                d.key.clone().unwrap_or_default(),
                d.name.clone(),
                d.units.clone(),
                d.frequency.id().to_owned(),
                d.store_type.id().to_owned(),
                d.kind.id().to_owned(),
                i.to_string(),
                v.stream_data_ordinal.to_string(),
                t.environment_ordinal.to_string(),
                env_json,
                t.report_id.to_string(),
                t.kind.id().to_owned(),
                t_json,
                values_json,
                v.reported_fields[0].clone(),
                format!("{:016x}", v.value.to_bits()),
            ];
            writeln!(
                file,
                "{}",
                columns
                    .iter()
                    .map(|v| field(v))
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }
    }
    file.flush()?;
    Ok(())
}
pub(crate) fn write_source_reported_artifacts(
    dir: &Path,
    result: &SourceReportedStreams,
) -> Result<(), SourceReportedError> {
    std::fs::create_dir_all(dir)?;
    {
        let mut file = std::io::BufWriter::new(std::fs::File::create(
            dir.join("source-reported-results.json"),
        )?);
        serde_json::to_writer_pretty(&mut file, &BundleJson(result))
            .map_err(|e| invalid(Stream::Eso, 0, e.to_string()))?;
        // Observe the real buffered write error before starting CSV export.
        file.flush()?;
    }
    write_csv(&dir.join("source-eso.csv"), &result.eso)?;
    write_csv(&dir.join("source-mtr.csv"), &result.mtr)
}
