//! Exact live-owner warmup diagnostic and summary streams; no bundled legacy Zone copy.
use super::*;
pub(super) fn zone_failure(
    manager: &mut SourceHeatBalanceManager,
    o: &mut SourceWarmupCheckOwners<'_, '_>,
    i: usize,
) -> Result<(), Error> {
    let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
        kickoff: &o.globals.kickoff_simulation,
        developer: &o.project.system.developer_flag,
        warmup: &o.clock.warmup_flag,
        doing_sizing: &o.globals.doing_sizing,
    };
    o.backend.show_source_severe_error(&format!("CheckWarmupConvergence: Loads Initialization, Zone=\"{}\" did not converge after {} warmup days.",zone_name(o.zones,i)?,o.control.max_number_of_warmup_days), &flags)?;
    if !manager.report.warmup_convergence_warning && !o.globals.doing_sizing {
        o.backend
            .show_continue_error("See Warmup Convergence Information in .eio file for details.")?;
        manager.report.warmup_convergence_warning = true;
    } else if !manager.report.sizing_warmup_convergence_warning && o.globals.doing_sizing {
        o.backend
            .show_continue_error("Warmup Convergence failing during sizing.")?;
        manager.report.sizing_warmup_convergence_warning = true;
    }
    let kind = if o.environment.run_period_environment {
        "RunPeriod"
    } else {
        "SizingPeriod"
    };
    o.backend.show_continue_error(&format!(
        "...Environment({kind})=\"{}\"",
        o.environment.environment_name
    ))?;
    let row = manager
        .convergence
        .as_ref()
        .and_then(|a| a.get(i))
        .ok_or(Error::Unavailable(
            "actual diagnostic WarmupConvergenceValues",
        ))?;
    o.backend.show_continue_error(&format!(
        "..Max Temp Comparison = {} vs Temperature Convergence Tolerance={} - {} Convergence",
        round(row.test_values[0], 2)?,
        round(o.control.temperature_convergence_tolerance, 2)?,
        pass_fail(row.pass_flags[0])?
    ))?;
    o.backend.show_continue_error(&format!(
        "..Min Temp Comparison = {} vs Temperature Convergence Tolerance={} - {} Convergence",
        round(row.test_values[1], 2)?,
        round(o.control.temperature_convergence_tolerance, 2)?,
        pass_fail(row.pass_flags[1])?
    ))?;
    o.backend.show_continue_error(&format!(
        "..Max Heat Load Comparison = {} vs Loads Convergence Tolerance={} - {} Convergence",
        round(row.test_values[2], 4)?,
        round(o.control.loads_convergence_tolerance, 2)?,
        pass_fail(row.pass_flags[2])?
    ))?;
    o.backend.show_continue_error(&format!(
        "..Max Cool Load Comparison = {} vs Loads Convergence Tolerance={} - {} Convergence",
        round(row.test_values[3], 4)?,
        round(o.control.loads_convergence_tolerance, 2)?,
        pass_fail(row.pass_flags[3])?
    ))?;
    Ok(())
}
impl SourceHeatBalanceManager {
    pub(crate) fn report_source_warmup_convergence(
        &mut self,
        o: &mut SourceWarmupOutputOwners<'_>,
    ) -> Result<(), Error> {
        if o.clock.warmup_flag {
            return Ok(());
        }
        if self.report.first_summary_write && o.globals.num_of_zones > 0 {
            o.eio.write_all(SUMMARY_HEADER.as_bytes())?;
            self.report.first_summary_write = false;
        }
        report_array(&self.report, 5)?;
        self.report.temperature_stddev.fill(Some(0.0));
        report_array(&self.report, 6)?;
        self.report.load_stddev.fill(Some(0.0));
        self.report.last_local_summaries.clear();
        let header = if o.environment.run_period_environment {
            "RunPeriod:"
        } else {
            "SizingPeriod:"
        };
        for number in 1..=o.globals.num_of_zones {
            let i = index(number)?;
            let count = self.report.count as f64;
            let mut sum = 0.0;
            report_array(&self.report, 2)?;
            for point in 0..self.report.count {
                sum += *self
                    .report
                    .temperature
                    .get(point)
                    .and_then(|a| a.get(i))
                    .ok_or(Error::Unavailable("actual TempZoneRpt summary slice"))?;
            }
            let mean_temperature = sum / count;
            for point in 0..self.report.count {
                report_array(&self.report, 4)?;
                let maximum = *self
                    .report
                    .maximum_load
                    .get(point)
                    .and_then(|a| a.get(i))
                    .ok_or(Error::Unavailable("actual MaxLoadZoneRpt summary cell"))?;
                report_array(&self.report, 3)?;
                let load = self
                    .report
                    .load
                    .get_mut(point)
                    .and_then(|a| a.get_mut(i))
                    .ok_or(Error::Unavailable("actual LoadZoneRpt summary cell"))?;
                if maximum > 1.0e-4 {
                    *load /= maximum;
                } else {
                    *load = 0.0;
                }
            }
            let mut sum = 0.0;
            report_array(&self.report, 3)?;
            for point in 0..self.report.count {
                sum += *self
                    .report
                    .load
                    .get(point)
                    .and_then(|a| a.get(i))
                    .ok_or(Error::Unavailable("actual LoadZoneRpt summary slice"))?;
            }
            let mean_load = sum / count;
            for point in 0..self.report.count {
                let temperature = *self
                    .report
                    .temperature
                    .get(point)
                    .and_then(|a| a.get(i))
                    .ok_or(Error::Unavailable("actual temperature deviation cell"))?
                    - mean_temperature;
                *self
                    .report
                    .temperature_stddev
                    .get_mut(point)
                    .ok_or(Error::Unavailable("actual temperature scratch cell"))? =
                    Some(temperature * temperature);
                let load = *self
                    .report
                    .load
                    .get(point)
                    .and_then(|a| a.get(i))
                    .ok_or(Error::Unavailable("actual load deviation cell"))?
                    - mean_load;
                *self
                    .report
                    .load_stddev
                    .get_mut(point)
                    .ok_or(Error::Unavailable("actual load scratch cell"))? = Some(load * load);
            }
            let mut sum = 0.0;
            for point in 0..self.report.count {
                sum += self
                    .report
                    .temperature_stddev
                    .get(point)
                    .copied()
                    .flatten()
                    .ok_or(Error::Unavailable("actual written temperature scratch"))?;
            }
            let stddev_temperature = (sum / count).sqrt();
            let mut sum = 0.0;
            for point in 0..self.report.count {
                sum += self
                    .report
                    .load_stddev
                    .get(point)
                    .copied()
                    .flatten()
                    .ok_or(Error::Unavailable("actual written load scratch"))?;
            }
            let stddev_load = (sum / count).sqrt();
            // Existing Rust observation only; no scientific value is read back from it.
            self.report.last_local_summaries.push(WarmupReportSummary {
                zone_index: i,
                mean_temperature,
                mean_load,
                stddev_temperature,
                stddev_load,
            });
            let conv = self
                .convergence
                .as_ref()
                .and_then(|a| a.get(i))
                .ok_or(Error::Unavailable("actual summary WarmupConvergenceValues"))?;
            writeln!(
                o.eio,
                " Warmup Convergence Information,{},{},{},{},{},{},{},{},{},{}",
                zone_name(o.zones, i)?,
                format_args!("{header} {}", o.environment.environment_name),
                round(mean_temperature, 10)?,
                round(stddev_temperature, 10)?,
                pass_fail(conv.pass_flags[0])?,
                pass_fail(conv.pass_flags[1])?,
                round(mean_load, 10)?,
                round(stddev_load, 10)?,
                pass_fail(conv.pass_flags[2])?,
                pass_fail(conv.pass_flags[3])?
            )?;
        }
        Ok(())
    }
}
