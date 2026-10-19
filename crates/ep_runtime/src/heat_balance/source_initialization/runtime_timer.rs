//! Native Timer system-clock lifetime and millisecond/F2 formatting.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceRuntimeTimer {
    start: SystemTime,
    end: SystemTime,
    duration_milliseconds: i64,
}
impl Default for SourceRuntimeTimer {
    fn default() -> Self {
        Self {
            start: UNIX_EPOCH,
            end: UNIX_EPOCH,
            duration_milliseconds: 0,
        }
    }
}
impl SourceRuntimeTimer {
    // Timer.cc57-61: tick does not reset accumulated duration.
    pub(crate) fn tick(&mut self) {
        self.end = UNIX_EPOCH;
        self.start = SystemTime::now();
    }
    pub(crate) fn tock(&mut self) -> Result<(), Error> {
        if self.start == UNIX_EPOCH {
            return Err(Error::Unavailable("actual Timer.tock before tick"));
        }
        self.end = SystemTime::now();
        let delta = match self.end.duration_since(self.start) {
            Ok(v) => i64::try_from(v.as_millis())
                .map_err(|_| Error::Unavailable("actual Timer duration signed milliseconds"))?,
            Err(v) => i64::try_from(v.duration().as_millis())
                .ok()
                .and_then(i64::checked_neg)
                .ok_or(Error::Unavailable(
                    "actual negative Timer duration signed milliseconds",
                ))?,
        };
        self.duration_milliseconds =
            self.duration_milliseconds
                .checked_add(delta)
                .ok_or(Error::Unavailable(
                    "actual Timer accumulated duration signed milliseconds",
                ))?;
        Ok(())
    }
    pub(crate) fn format_as_hour_min_secs(&self) -> Result<String, Error> {
        if self.end == UNIX_EPOCH {
            return Err(Error::Unavailable("actual Timer duration before tock"));
        }
        let mut count = self.duration_milliseconds;
        let hours = count / 3_600_000;
        count -= hours * 3_600_000;
        let minutes = count / 60_000;
        count -= minutes * 60_000;
        let mut seconds = count as f64 / 1000.0;
        if seconds < 0.0 {
            seconds = 0.0;
        }
        let seconds = crate::heat_balance::ctf_reports::formatting::fixed(seconds, 5, 2);
        Ok(format!("{hours:02}hr {minutes:02}min {seconds}sec"))
    }
}
