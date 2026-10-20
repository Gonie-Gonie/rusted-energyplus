use super::*;

fn header(period_end: &str) -> String {
    format!(
        "LOCATION,Example,CO,USA,Test,000000,39.74,-105.18,-7,1600\nDESIGN CONDITIONS,0\nTYPICAL/EXTREME PERIODS,0\nGROUND TEMPERATURES,0\nHOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\nCOMMENTS 1,\nCOMMENTS 2,\nDATA PERIODS,1,1,Data,Sunday,1/1,{period_end}\n"
    )
}

fn row(year: i32, day: i32, hour: usize, code: &str) -> String {
    format!(
        "{year},7,{day},{hour},60,A,20,10,50,101325,0,0,300,0,0,0,0,0,0,0,180,2.5,0,0,0,0,0,{code},999,999,999,999,999,0,1\n"
    )
}

fn day(year: i32, date: i32, code: &str) -> String {
    (1..=24).map(|hour| row(year, date, hour, code)).collect()
}

fn owner(text: String) -> RawEpwCursorOwner {
    let initial = RawEpwHeaderState {
        weather_code_missed_count: 41,
        ..Default::default()
    };
    RawEpwCursorOwner::open_from_bytes(
        text.into_bytes(),
        initial,
        HeaderCallerPreparation::default(),
    )
    .unwrap()
}

fn selection(date: i32) -> RawDayReadSelection {
    RawDayReadSelection {
        start_month: 7,
        start_day: date,
        start_year: 2013,
        match_year: false,
        days_in_year: 365,
    }
}

#[derive(Default)]
struct Writes {
    events: Vec<(char, usize)>,
    /// Nonzero four-timestep canaries prove that a failed hourly extraction
    /// clears only timestep one and retains later slots/prior actual writes.
    tomorrow: [[f64; 4]; 24],
}

impl Writes {
    fn canaries() -> Self {
        Self {
            tomorrow: [[77.0, 78.0, 79.0, 80.0]; 24],
            ..Default::default()
        }
    }
}

impl RawDayReadCallbacks for Writes {
    fn first_day_selected(
        &mut self,
        _: &mut RawDayReadState,
        _: &RawWeatherSlot,
    ) -> Result<(), String> {
        self.events.push(('s', 0));
        Ok(())
    }
    fn clear_source_interval(&mut self, hour: usize, interval: usize) -> Result<(), String> {
        assert_eq!(interval, 1);
        self.events.push(('c', hour));
        self.tomorrow[hour - 1][0] = 0.0;
        Ok(())
    }
    fn parsed_hour(
        &mut self,
        state: &mut RawDayReadState,
        hour: usize,
        slot: &RawWeatherSlot,
    ) -> Result<(), String> {
        self.events.push(('p', hour));
        self.tomorrow[hour - 1][0] = slot.raw.mandatory_reals[0];
        if hour == 1 {
            state.current_day_of_week = state.current_day_of_week % 7 + 1;
        }
        Ok(())
    }
    fn complete_day(
        &mut self,
        state: &mut RawDayReadState,
        _: &RawWeatherDay,
    ) -> Result<(), String> {
        self.events.push(('d', 24));
        state.last_hour_set = true;
        Ok(())
    }
}

#[test]
fn selected_day_interprets_only_decoy_first_rows_and_reinterprets_match() {
    let mut text = header("12/31");
    let mut first = day(2001, 1, "9");
    // This row must never reach Interpret during decoy-day skipping.
    let start = first.find('\n').unwrap() + 1;
    let end = start + first[start..].find('\n').unwrap() + 1;
    first.replace_range(start..end, "not a numeric EPW record\n");
    text.push_str(&first);
    text.push_str(&day(2002, 2, "9"));
    let selected_start = text.len();
    text.push_str(&day(2003, 3, "9"));
    let mut cursor = owner(text);
    let mut state = RawDayReadState::default();
    let mut writes = Writes::canaries();
    let selected = cursor
        .read_day(
            selection(3),
            &mut state,
            RawDayReadMode::FirstDay,
            false,
            &mut writes,
        )
        .unwrap();
    assert_eq!(cursor.interpret_count(), 27);
    assert_eq!(cursor.header().weather_code_missed_count, 41 + 27);
    assert_eq!(cursor.parser_state().weather_code_missed_count, 41 + 27);
    assert_eq!(selected.hours.len(), 24);
    assert_eq!(selected.hours[0].provenance.start_byte, selected_start);
    assert_eq!(selected.hours[0].raw.dates, [2003, 7, 3, 1, 60]);
    assert_eq!(selected.hours[23].raw.dates[3], 24);
    assert_eq!(writes.events[0], ('s', 0));
    assert_eq!(
        writes.events[1..5],
        [('c', 1), ('p', 1), ('c', 2), ('p', 2)]
    );
    assert_eq!(writes.events.last(), Some(&('d', 24)));
    assert!(state.last_hour_set);
}

#[test]
fn first_day_search_starts_at_current_cursor_and_ignores_raw_year() {
    let mut text = header("12/31");
    text.push_str(&day(2001, 3, "999999999"));
    let second_start = text.len();
    text.push_str(&day(1999, 3, "999999999"));
    let mut cursor = owner(text);
    let mut state = RawDayReadState::default();
    let mut writes = Writes::default();
    let first = cursor
        .read_day(
            selection(3),
            &mut state,
            RawDayReadMode::FirstDay,
            false,
            &mut writes,
        )
        .unwrap();
    let second = cursor
        .read_day(
            selection(3),
            &mut state,
            RawDayReadMode::FirstDay,
            false,
            &mut writes,
        )
        .unwrap();
    assert_eq!(first.hours[0].raw.dates[0], 2001);
    assert_eq!(second.hours[0].raw.dates[0], 1999);
    assert_eq!(second.hours[0].provenance.start_byte, second_start);
    assert_eq!(cursor.interpret_count(), 50);
}

#[test]
fn missing_start_permits_one_rewind_then_source_fatal_with_actual_counts() {
    let text = header("12/31") + &day(2013, 1, "9");
    let end = text.len();
    let mut cursor = owner(text);
    let mut writes = Writes::default();
    let error = cursor
        .read_day(
            selection(4),
            &mut RawDayReadState::default(),
            RawDayReadMode::FirstDay,
            false,
            &mut writes,
        )
        .unwrap_err();
    assert_eq!(
        error,
        RawDayFailure::SourceFatal(RawDaySourceFatal::SearchRead)
    );
    assert_eq!(cursor.interpret_count(), 2);
    assert_eq!(cursor.line_reads(), 8 + 24 + 1 + 8 + 24 + 1);
    assert_eq!(cursor.byte_cursor(), end);
    assert!(cursor.stream_state().eof && cursor.stream_state().fail);
    assert_eq!(cursor.header().weather_code_missed_count, 43);
    assert!(writes.events.is_empty());
}

#[test]
fn truncated_decoy_uses_retained_last_line_for_date_only_fatal() {
    let text = header("12/31") + &row(2001, 1, 1, "9") + &row(2001, 1, 2, "9");
    let mut cursor = owner(text);
    let error = cursor
        .read_day(
            selection(4),
            &mut RawDayReadState::default(),
            RawDayReadMode::FirstDay,
            false,
            &mut Writes::default(),
        )
        .unwrap_err();
    assert!(error.is_source_fatal());
    assert_eq!(cursor.interpret_count(), 1);
    assert_eq!(cursor.line_reads(), 11);
    assert_eq!(cursor.last_arguments().dates, [2001, 7, 1, 2, 60]);
    assert_eq!(cursor.header().weather_code_missed_count, 42);
}

#[test]
fn next_day_rewinds_only_when_declared_period_is_full_year() {
    for (period_end, wraps) in [("12/31", true), ("1/1", false)] {
        let mut cursor = owner(header(period_end) + &day(2013, 1, "9"));
        let mut state = RawDayReadState::default();
        let mut writes = Writes::default();
        let first = cursor
            .read_day(
                selection(1),
                &mut state,
                RawDayReadMode::FirstDay,
                false,
                &mut writes,
            )
            .unwrap();
        writes.events.clear();
        let result = cursor.read_day(
            selection(1),
            &mut state,
            RawDayReadMode::NextDay,
            false,
            &mut writes,
        );
        if wraps {
            let second = result.unwrap();
            assert_eq!(
                second.hours[0].provenance.start_byte,
                first.hours[0].provenance.start_byte
            );
            assert_eq!(cursor.interpret_count(), 49);
            assert_eq!(writes.events[0], ('c', 1));
            assert_eq!(writes.events[1], ('p', 1));
        } else {
            assert_eq!(
                result.unwrap_err(),
                RawDayFailure::SourceFatal(RawDaySourceFatal::PartialPeriodEof)
            );
            assert_eq!(cursor.interpret_count(), 25);
            assert_eq!(writes.events, [('c', 1)]);
        }
    }
}

#[test]
fn failing_hour_retains_prior_writes_and_only_clears_one_timestep() {
    let text =
        header("12/31") + &row(2013, 1, 1, "9") + &row(2013, 1, 2, "9") + &row(2013, 1, 4, "9");
    let mut cursor = owner(text);
    let mut writes = Writes::canaries();
    let error = cursor
        .read_day(
            selection(1),
            &mut RawDayReadState::default(),
            RawDayReadMode::FirstDay,
            false,
            &mut writes,
        )
        .unwrap_err();
    assert_eq!(
        error,
        RawDayFailure::SourceFatal(RawDaySourceFatal::UnexpectedHour {
            expected: 3,
            actual: 4
        })
    );
    assert_eq!(
        writes.events,
        [('s', 0), ('c', 1), ('p', 1), ('c', 2), ('p', 2), ('c', 3)]
    );
    assert_eq!(writes.tomorrow[0], [20.0, 78.0, 79.0, 80.0]);
    assert_eq!(writes.tomorrow[2], [0.0, 78.0, 79.0, 80.0]);
    assert_eq!(writes.tomorrow[3], [77.0, 78.0, 79.0, 80.0]);
    assert_eq!(cursor.interpret_count(), 4);
    assert_eq!(cursor.header().weather_code_missed_count, 45);
}

#[test]
fn source_backspace_after_day_is_one_record_not_one_day() {
    let mut cursor = owner(header("12/31") + &day(2013, 1, "9"));
    let mut state = RawDayReadState::default();
    let first = cursor
        .read_day(
            selection(1),
            &mut state,
            RawDayReadMode::FirstDay,
            true,
            &mut Writes::default(),
        )
        .unwrap();
    assert_eq!(cursor.byte_cursor(), first.hours[23].provenance.start_byte);
    let mut writes = Writes::default();
    let error = cursor
        .read_day(
            selection(1),
            &mut state,
            RawDayReadMode::FirstDay,
            false,
            &mut writes,
        )
        .unwrap_err();
    assert_eq!(
        error,
        RawDayFailure::SourceFatal(RawDaySourceFatal::UnexpectedHour {
            expected: 1,
            actual: 24
        })
    );
    assert_eq!(writes.events, [('s', 0), ('c', 1)]);
    assert_eq!(cursor.interpret_count(), 27);
}

#[test]
fn skip_header_checks_eof_before_matching_data_periods() {
    let mut cursor = owner(header("12/31"));
    // Supply a real opened byte owner whose matching final line has no LF.
    let mut input = RawEpwInput::new_unopened(b"COMMENTS\nDATA PERIODS,1,1".to_vec());
    input.reopen();
    cursor = RawEpwCursorOwner::from_open_input(
        input,
        cursor.header().clone(),
        false,
        HeaderContextConsumption::default(),
    );
    assert_eq!(
        cursor.skip_header().unwrap_err(),
        RawDayFailure::Header(RawEpwHeaderError::SourceFatal("unexpected-header-EOF"))
    );
    assert_eq!(cursor.interpret_count(), 0);
}

#[test]
fn get_next_style_reopen_skips_without_reprocessing_shared_header() {
    let mut cursor = owner(header("12/31") + &day(2013, 1, "9"));
    let header_end = cursor.byte_cursor();
    let mut state = RawDayReadState::default();
    cursor
        .read_day(
            selection(1),
            &mut state,
            RawDayReadMode::FirstDay,
            false,
            &mut Writes::default(),
        )
        .unwrap();
    let header_before = cursor.header().clone();
    let arguments_before = *cursor.last_arguments();
    let attempts_before = cursor.line_reads();
    cursor.close();
    assert!(!cursor.stream_state().is_open && cursor.stream_state().bad);
    cursor.reopen_without_processing_header().unwrap();
    assert_eq!(cursor.header(), &header_before);
    assert_eq!(cursor.last_arguments(), &arguments_before);
    assert_eq!(cursor.interpret_count(), 25);
    assert_eq!(cursor.line_reads(), attempts_before + 8);
    assert_eq!(cursor.byte_cursor(), header_end);
    assert_eq!(
        cursor.stream_after_header().position_byte,
        Some(header_end as i64)
    );
    cursor.close();
    cursor
        .reopen_header(HeaderCallerPreparation::default())
        .unwrap();
    assert_eq!(
        cursor.header().weather_code_missed_count,
        header_before.weather_code_missed_count
    );
    assert_eq!(cursor.last_arguments(), &arguments_before);
    assert_eq!(cursor.interpret_count(), 25);
}

/// Unit boundary only: borrow genuine weather state for raw-day control checks.
/// Event callbacks observe consumption; they make no physical-processing claim.
struct CalendarRead<'a> {
    state: &'a mut crate::weather::day::WeatherDayState,
    events: Vec<(char, usize)>,
}
impl SourceRawDayReadClock for CalendarRead<'_> {
    fn current_year_is_leap_year(&self) -> Result<bool, RawDayFailure> {
        Ok(self.state.environment.current_year_is_leap_year)
    }
    fn current_day_of_week(&self) -> i32 {
        self.state.weather.cur_day_of_week
    }
    fn set_current_day_of_week(&mut self, value: i32) {
        self.state.weather.cur_day_of_week = value;
    }
    fn set_read_time(&mut self, value: f64) {
        self.state.weather.read_e_plus_weather_cur_time = value;
    }
    fn set_last_hour_set(&mut self, value: bool) {
        self.state.weather.last_hour_set = value;
    }
    fn interpret_record(
        &mut self,
        cursor: &mut RawEpwCursorOwner,
        read: &RawEpwLineRead,
    ) -> Result<(), RawDayFailure> {
        cursor.interpret_with_live_source_counter(read, &mut self.state.missed_counts.weath_codes)
    }
}
impl SourceRawDayReadCallbacks for CalendarRead<'_> {
    type Error = String;
    fn prepare_hour(&mut self, _: &mut RawEpwOutputs) -> Result<(), String> {
        self.events.push(('n', 0));
        Ok(())
    }
    fn prepare_skipped_leap_day(&mut self, header: &mut RawEpwHeaderState) -> Result<(), String> {
        header.month_ends[1] = 28;
        self.state.weather.end_day_of_month_with_leap_day[1] = 28;
        self.events.push(('f', 0));
        Ok(())
    }
    fn report_skipped_leap_day(&mut self, _: &RawEpwHeaderState) -> Result<(), String> {
        self.events.push(('w', 0));
        Ok(())
    }
    fn first_day_selected(
        &mut self,
        _: &RawWeatherSlot,
        _: &mut RawEpwHeaderState,
        _: &mut RawEpwOutputs,
    ) -> Result<(), String> {
        self.events.push(('s', 0));
        Ok(())
    }
    fn clear_source_interval(&mut self, hour: usize, interval: usize) -> Result<(), String> {
        assert_eq!(interval, 1);
        self.events.push(('c', hour));
        Ok(())
    }
    fn parsed_hour(
        &mut self,
        hour: usize,
        slot: &RawWeatherSlot,
        _: &mut RawEpwHeaderState,
        _: &mut RawEpwOutputs,
    ) -> Result<(), String> {
        assert_eq!(slot.raw.dates[3], hour as i32);
        self.events.push(('p', hour));
        Ok(())
    }
    fn complete_day(&mut self, _: &RawWeatherDay, _: &mut RawEpwHeaderState) -> Result<(), String> {
        self.events.push(('d', 24));
        Ok(())
    }
}

fn leap_selection() -> RawDayReadSelection {
    RawDayReadSelection {
        start_month: 2,
        start_day: 29,
        start_year: 2016,
        match_year: false,
        days_in_year: 366,
    }
}
fn leap_raw_text(header_allows: bool) -> String {
    let header = if header_allows {
        header("12/31").replace(
            "HOLIDAYS/DAYLIGHT SAVINGS,No,",
            "HOLIDAYS/DAYLIGHT SAVINGS,Yes,",
        )
    } else {
        header("12/31")
    };
    header + &day(2016, 29, "9").replace("2016,7,29,", "2016,2,29,")
}

#[test]
fn leap_retry_consumes_all_physical_rows_but_only_one_day_of_callbacks() {
    for (calendar_leap, header_allows) in
        [(false, false), (false, true), (true, false), (true, true)]
    {
        let skipped = !calendar_leap || !header_allows;
        let text =
            leap_raw_text(header_allows) + &day(2016, 1, "9").replace("2016,7,1,", "2016,3,1,");
        let mut cursor = owner(text);
        let mut state = crate::weather::day::WeatherDayState {
            environment: crate::weather::day::WeatherEnvironmentState {
                current_year_is_leap_year: calendar_leap,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut sink = CalendarRead {
            state: &mut state,
            events: Vec::new(),
        };
        let read = cursor
            .read_day_with_live_source_state(
                leap_selection(),
                RawDayReadMode::FirstDay,
                false,
                &mut sink,
            )
            .unwrap();
        assert_eq!(read.hours.len(), 24);
        assert_eq!(
            read.hours[0].raw.dates[1..3],
            if skipped { [3, 1] } else { [2, 29] }
        );
        assert_eq!(cursor.interpret_count(), if skipped { 49 } else { 25 });
        for (event, expected) in [
            ('s', 1),
            ('c', if skipped { 48 } else { 24 }),
            ('n', if skipped { 48 } else { 24 }),
            ('p', 24),
            ('f', usize::from(skipped)),
            ('w', usize::from(skipped)),
            ('d', 1),
        ] {
            assert_eq!(
                sink.events.iter().filter(|x| x.0 == event).count(),
                expected,
                "{event}"
            );
        }
        assert_eq!(&sink.events[..3], &[('s', 0), ('c', 1), ('n', 0)]);
        assert_eq!(sink.events.last(), Some(&('d', 24)));
        assert_eq!(
            sink.state.environment.current_year_is_leap_year,
            calendar_leap
        );
        assert_eq!(cursor.header().allows_leap_years, header_allows);
        if skipped {
            assert_eq!(&sink.events[3..6], &[('f', 0), ('w', 0), ('c', 2)]);
            assert_eq!(cursor.header().month_ends[1], 28);
            assert_eq!(sink.state.weather.end_day_of_month_with_leap_day[1], 28);
            assert_eq!(read.hours[0].raw.dates, [2016, 3, 1, 1, 60]);
        }
    }
}

#[test]
fn skipped_day_still_validates_each_hour_before_normalizing_or_processing_it() {
    let text = header("12/31")
        + &row(2016, 29, 1, "9").replace("2016,7,29,", "2016,2,29,")
        + &row(2016, 29, 2, "9").replace("2016,7,29,", "2016,2,29,")
        + &row(2016, 29, 4, "9").replace("2016,7,29,", "2016,2,29,");
    let mut cursor = owner(text);
    let mut state = crate::weather::day::WeatherDayState::default();
    let mut sink = CalendarRead {
        state: &mut state,
        events: Vec::new(),
    };
    let error = cursor
        .read_day_with_live_source_state(
            leap_selection(),
            RawDayReadMode::FirstDay,
            false,
            &mut sink,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        SourceRawDayReadFailure::Native(RawDayFailure::SourceFatal(
            RawDaySourceFatal::UnexpectedHour {
                expected: 3,
                actual: 4
            }
        ))
    ));
    assert_eq!(cursor.interpret_count(), 4);
    assert_eq!(
        sink.events,
        [
            ('s', 0),
            ('c', 1),
            ('n', 0),
            ('f', 0),
            ('w', 0),
            ('c', 2),
            ('n', 0),
            ('c', 3)
        ]
    );
}

#[test]
fn raw_only_reader_keeps_feb29_unavailable_without_a_live_calendar_owner() {
    let mut cursor = owner(leap_raw_text(true));
    let mut writes = Writes::default();
    let error = cursor
        .read_day(
            leap_selection(),
            &mut RawDayReadState::default(),
            RawDayReadMode::FirstDay,
            false,
            &mut writes,
        )
        .unwrap_err();
    assert_eq!(
        error,
        RawDayFailure::OutsideBoundedDomain("February 29 daily retry")
    );
    assert_eq!(writes.events, [('s', 0), ('c', 1)]);
    assert_eq!(cursor.interpret_count(), 2);
}
