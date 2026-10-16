//! Literal OutputProcessor1614?1765 accumulation and warmup reset.
use super::meters::SourceMeterPeriod;
use super::*;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
pub(super) const MAX_SET: f64 = -99999999999999.0;
pub(super) const MIN_SET: f64 = 99999999999999.0;
fn reset_period(period: &mut SourceMeterPeriod) {
    period.value = 0.0;
    period.maximum = MAX_SET;
    period.maximum_date = 0;
    period.minimum = MIN_SET;
    period.minimum_date = 0;
}
impl SourceOutputRegistration {
    pub(crate) fn update_source_meters(
        &mut self,
        timestamp: i32,
        warmup: bool,
    ) -> Result<(), Error> {
        if warmup {
            return Ok(());
        }
        if self.meters.meters.is_empty() || self.meter_values.is_empty() {
            return Ok(());
        }
        for (number, meter) in self.meters.meters.iter_mut().enumerate() {
            meter.periods[SourceReportFrequency::TimeStep as usize].value += *self
                .meter_values
                .get(number)
                .ok_or(Error::Unavailable("actual meterValues index"))?;
            let value = meter.periods[SourceReportFrequency::TimeStep as usize].value;
            meter.periods[SourceReportFrequency::Hour as usize].value += value;
            meter.periods[SourceReportFrequency::Day as usize].value += value;
            meter.periods[SourceReportFrequency::Month as usize].value += value;
            meter.periods[SourceReportFrequency::Year as usize].value += value;
            meter.periods[SourceReportFrequency::Simulation as usize].value += value;
            meter.period_final_year_simulation.value += value;
        }
        for meter in &mut self.meters.meters {
            let value = meter.periods[SourceReportFrequency::TimeStep as usize].value;
            let period = &mut meter.periods[SourceReportFrequency::Day as usize];
            if value <= period.maximum {
                continue;
            }
            period.maximum = value;
            period.maximum_date = timestamp;
            let period = &mut meter.periods[SourceReportFrequency::Month as usize];
            if value <= period.maximum {
                continue;
            }
            period.maximum = value;
            period.maximum_date = timestamp;
            let period = &mut meter.periods[SourceReportFrequency::Year as usize];
            if value > period.maximum {
                period.maximum = value;
                period.maximum_date = timestamp;
            }
            let period = &mut meter.periods[SourceReportFrequency::Simulation as usize];
            if value > period.maximum {
                period.maximum = value;
                period.maximum_date = timestamp;
            }
            if value > meter.period_final_year_simulation.maximum {
                meter.period_final_year_simulation.maximum = value;
                meter.period_final_year_simulation.maximum_date = timestamp;
            }
        }
        for meter in &mut self.meters.meters {
            let value = meter.periods[SourceReportFrequency::TimeStep as usize].value;
            let period = &mut meter.periods[SourceReportFrequency::Day as usize];
            if value >= period.minimum {
                continue;
            }
            period.minimum = value;
            period.minimum_date = timestamp;
            let period = &mut meter.periods[SourceReportFrequency::Month as usize];
            if value >= period.minimum {
                continue;
            }
            period.minimum = value;
            period.minimum_date = timestamp;
            let period = &mut meter.periods[SourceReportFrequency::Year as usize];
            if value < period.minimum {
                period.minimum = value;
                period.minimum_date = timestamp;
            }
            let period = &mut meter.periods[SourceReportFrequency::Simulation as usize];
            if value < period.minimum {
                period.minimum = value;
                period.minimum_date = timestamp;
            }
            if value < meter.period_final_year_simulation.minimum {
                meter.period_final_year_simulation.minimum = value;
                meter.period_final_year_simulation.minimum_date = timestamp;
            }
        }
        for number in 0..self.meters.meters.len() {
            *self
                .meter_values
                .get_mut(number)
                .ok_or(Error::Unavailable("actual meterValues reset index"))? = 0.0;
        }
        Ok(())
    }
    pub(crate) fn reset_source_accumulation_when_warmup_complete(&mut self) -> Result<(), Error> {
        let count = self.meters.meters.len();
        for meter in &mut self.meters.meters {
            for period in &mut meter.periods[SourceReportFrequency::Hour as usize..] {
                reset_period(period);
            }
            reset_period(&mut meter.period_final_year_simulation);
            // Repeated inside each meter, as in Native1750?1752.
            for number in 0..count {
                *self
                    .meter_values
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual warmup meterValues reset index"))? = 0.0;
            }
        }
        for var in &mut self.out_vars {
            if matches!(
                var.frequency,
                SourceReportFrequency::Month
                    | SourceReportFrequency::Year
                    | SourceReportFrequency::Simulation
            ) {
                var.store_value = 0.0;
                var.num_stored = 0.0;
            }
        }
        Ok(())
    }
}
