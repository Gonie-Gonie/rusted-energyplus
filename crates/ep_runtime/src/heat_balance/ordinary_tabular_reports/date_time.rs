//! Actual Objexx date_and_time(_,_,_,td), called only at TAB279.
use super::state::SourceTabularInputState;
use chrono::{Datelike, Local, Timelike};
impl SourceTabularInputState {
    pub(super) fn write_source_current_date_and_time(&mut self) {
        // The first wall-clock sample supplies date/time/milliseconds. Native
        // time_zone_offset_seconds obtains its own current sample afterwards.
        let now = Local::now();
        let milliseconds = now.timestamp_subsec_millis() as i32;
        let day = now.day() as i32;
        let month = now.month() as i32;
        let year = now.year();
        let hour = now.hour() as i32;
        let minute = now.minute() as i32;
        let second = now.second() as i32;
        let zone_seconds = Local::now().offset().local_minus_utc();
        self.td[0] = Some(year);
        self.td[1] = Some(month);
        self.td[2] = Some(day);
        self.td[3] = Some(zone_seconds / 60);
        self.td[4] = Some(hour);
        self.td[5] = Some(minute);
        self.td[6] = Some(second);
        self.td[7] = Some(milliseconds);
    }
}
