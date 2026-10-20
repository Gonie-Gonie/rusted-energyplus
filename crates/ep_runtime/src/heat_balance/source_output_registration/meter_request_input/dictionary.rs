//! Genuine WriteMeterDictionaryItem2525-2589 on actual MTR/ESO/SQLite/RF owners.
use super::super::SourceOutputUnits;
use super::super::meters::SourceMeter;
use super::super::report_streams::SourceReportStream;
use super::{Error, SourceMeterReportingHost, SourceReportFrequency};
pub(super) fn write_meter_dictionary(
    frequency: SourceReportFrequency,
    report_id: i32,
    meter: &SourceMeter,
    cumulative: bool,
    meter_file_only: bool,
    host: &mut impl SourceMeterReportingHost,
) -> Result<(bool, bool), Error> {
    let (_, arbitrary_integer, notice, _) = frequency.dictionary_format();
    let short_notice = notice.split_once('[').map_or(notice, |(before, _)| before);
    let mut out = |stream: SourceReportStream| -> Result<bool, Error> {
        if stream.good(host) {
            if meter.units == SourceOutputUnits::Invalid {
                return Err(Error::Unavailable(
                    "defined native meter dictionary unitNames index",
                ));
            }
            let line = if cumulative {
                format!(
                    "{report_id},1,Cumulative {} [{}]{short_notice}\n",
                    meter.name,
                    meter.units.symbol()
                )
            } else {
                format!(
                    "{report_id},{arbitrary_integer},{} [{}]{notice}\n",
                    meter.name,
                    meter.units.symbol()
                )
            };
            stream.write(host, line.as_bytes())?;
            return Ok(true);
        }
        Ok(false)
    };
    let mtr_written = out(SourceReportStream::Mtr)?;
    let eso_written = if !meter_file_only {
        out(SourceReportStream::Eso)?
    } else {
        false
    };
    if host.sqlite_present() {
        return Err(Error::Unavailable(
            "actual SQLite createReportDictionaryRecord meter dictionary",
        ));
    }
    if meter.units == SourceOutputUnits::Invalid {
        return Err(Error::Unavailable(
            "defined native RF report-meter unitNames index",
        ));
    }
    super::super::add_source_report_meter(
        host.results_framework_output_variables(),
        &meter.name,
        meter.units.symbol(),
        frequency,
    );
    Ok((mtr_written, eso_written))
}
