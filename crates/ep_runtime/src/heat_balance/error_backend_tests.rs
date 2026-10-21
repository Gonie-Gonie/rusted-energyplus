use super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "authored version".into(),
        idd_version_string: "authored IDD".into(),
        doing_input_processing: false,
        print_console_output: true,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
            match_counts: [0; 20],
            abort_processing: false,
            total_severe_errors: 7,
            total_severe_errors_during_warmup: 4,
            total_severe_errors_during_sizing: 2,
            last_severe_error: "prior severe".into(),
            total_warning_errors: 3,
            total_warning_errors_during_warmup: 2,
            total_warning_errors_during_sizing: 1,
        },
    }
}

fn backend<'a>(
    state: &'a mut SourceErrorState,
    error: &'a mut dyn Write,
    console: &'a mut dyn Write,
) -> SourceErrorBackend<'a> {
    SourceErrorBackend {
        state,
        error_stream: Some(error),
        console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    }
}

/// Explicit test double for the borrowed database call interface, not SQLite parity.
struct TestSqlite {
    calls: Rc<RefCell<Vec<Vec<u8>>>>,
    transaction: bool,
    fail_record: bool,
}
impl SourceErrorSqlite for TestSqlite {
    fn create_error_record(
        &mut self,
        simulation: i32,
        kind: i32,
        message: &str,
        count: i32,
    ) -> io::Result<()> {
        self.calls
            .borrow_mut()
            .push(format!("create:{simulation}:{kind}:{message}:{count}").into_bytes());
        if self.fail_record {
            return Err(io::Error::other("authored database adapter failure"));
        }
        Ok(())
    }
    fn update_error_record(&mut self, message: &[u8]) -> io::Result<()> {
        let mut call = b"update:".to_vec();
        call.extend_from_slice(message);
        self.calls.borrow_mut().push(call);
        if self.fail_record {
            return Err(io::Error::other("authored database adapter failure"));
        }
        Ok(())
    }
    fn within_transaction(&mut self) -> io::Result<bool> {
        self.calls.borrow_mut().push(b"transaction".to_vec());
        Ok(self.transaction)
    }
    fn commit(&mut self) -> io::Result<()> {
        self.calls.borrow_mut().push(b"commit".to_vec());
        Ok(())
    }
}

#[test]
fn fatal_writes_real_prefix_callbacks_database_then_returns_only_fatal()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let mut optional1 = Vec::new();
    let mut optional2 = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let mut sqlite = TestSqlite {
        calls: calls.clone(),
        transaction: true,
        fail_record: false,
    };
    let f_calls = calls.clone();
    let m_calls = calls.clone();
    let e_calls = calls.clone();
    let mut f = move |_: &mut SourceErrorState, message: &str| {
        f_calls
            .borrow_mut()
            .push(format!("f:{message}").into_bytes())
    };
    let mut m = move |_: &mut SourceErrorState, message: &str| {
        m_calls
            .borrow_mut()
            .push(format!("message:{message}").into_bytes())
    };
    let mut e = move |_: &mut SourceErrorState, level: SourceErrorLevel, message: &[u8]| {
        let mut call = format!("error:{}:", level as i32).into_bytes();
        call.extend_from_slice(message);
        e_calls.borrow_mut().push(call)
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.out_unit1 = Some(&mut optional1);
        actual.out_unit2 = Some(&mut optional2);
        actual.f_message = Some(&mut f);
        actual.message_callback = Some(&mut m);
        actual.error_callback = Some(&mut e);
        actual.sqlite = Some(&mut sqlite);
        assert!(
            matches!(actual.show_fatal_error("stop"), Err(SourceFatalStop::Fatal(SourceFatalError { message })) if message == "stop")
        );
    }
    assert_eq!(
        String::from_utf8(error)?,
        "Program Version,authored version,authored IDD\n   **  Fatal  ** stop\n   ...Summary of Errors that led to program termination:\n   ..... Reference severe error count=7\n   ..... Last severe error=prior severe\n"
    );
    assert_eq!(console, b"**FATAL:stop\n");
    assert_eq!(optional1, b"   **  Fatal  ** stop   ...Summary of Errors that led to program termination:   ..... Reference severe error count=7   ..... Last severe error=prior severe");
    assert_eq!(optional2, optional1);
    assert_eq!(
        calls
            .borrow()
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        [
            "f:**FATAL:stop",
            "message:**FATAL:stop",
            "create:1:2:stop:1",
            "transaction",
            "commit",
            "error:4:stop"
        ]
    );
    assert!(!state.output_error_header);
    assert_eq!(state.tracking.total_severe_errors, 7);
    assert_eq!(state.tracking.total_warning_errors, 3);
    Ok(())
}

#[test]
fn input_processing_keeps_header_in_error_stream_and_routes_messages_to_console()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    state.doing_input_processing = true;
    let mut error = Vec::new();
    let mut console = Vec::new();
    let mut optional = Vec::new();
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.out_unit1 = Some(&mut optional);
        actual.show_error_message("one")?;
        actual.show_error_message("two")?;
    }
    assert_eq!(error, b"Program Version,authored version,authored IDD\n");
    assert_eq!(console, b"one\ntwo\n");
    assert_eq!(optional, b"  one  two");
    Ok(())
}

#[test]
fn absent_error_stream_preserves_header_owner_and_console_off_still_writes_optional()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    state.doing_input_processing = true;
    state.print_console_output = false;
    let mut console = Vec::new();
    let mut optional = Vec::new();
    {
        let mut actual = SourceErrorBackend {
            state: &mut state,
            error_stream: None,
            console: &mut console,
            out_unit1: None,
            out_unit2: Some(&mut optional),
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        actual.show_error_message("reached")?;
    }
    assert!(state.output_error_header);
    assert!(console.is_empty());
    assert_eq!(optional, b"  reached");
    Ok(())
}

#[test]
fn display_callbacks_run_before_kickoff_console_suppression()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    state.kickoff_simulation = true;
    let mut error = Vec::new();
    let mut console = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let f_calls = calls.clone();
    let m_calls = calls.clone();
    let mut f = move |_: &mut SourceErrorState, message: &str| {
        f_calls.borrow_mut().push(format!("first:{message}"))
    };
    let mut m = move |_: &mut SourceErrorState, message: &str| {
        m_calls.borrow_mut().push(format!("second:{message}"))
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.f_message = Some(&mut f);
        actual.message_callback = Some(&mut m);
        actual.display_string("visible to callbacks")?;
    }
    assert_eq!(
        *calls.borrow(),
        ["first:visible to callbacks", "second:visible to callbacks"]
    );
    assert!(console.is_empty());
    Ok(())
}

#[derive(Default)]
struct FlushingConsole {
    bytes: Vec<u8>,
    flushes: usize,
}
impl Write for FlushingConsole {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        Ok(())
    }
}

#[test]
fn developer_display_writes_and_flushes_but_error_message_newline_does_not_flush()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    state.kickoff_simulation = true;
    state.developer_flag = true;
    state.doing_input_processing = true;
    let mut error = Vec::new();
    let mut console = FlushingConsole::default();
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.show_error_message("IP")?;
        actual.display_string("display")?;
    }
    assert_eq!(console.bytes, b"IP\ndisplay\n");
    assert_eq!(console.flushes, 1);
    Ok(())
}

#[test]
fn string_display_retains_full_bytes_except_native_c_string_message_callback()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let f_calls = calls.clone();
    let m_calls = calls.clone();
    let mut f = move |_: &mut SourceErrorState, message: &str| {
        f_calls.borrow_mut().push(message.to_string())
    };
    let mut m = move |_: &mut SourceErrorState, message: &str| {
        m_calls.borrow_mut().push(message.to_string())
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.f_message = Some(&mut f);
        actual.message_callback = Some(&mut m);
        actual.display_string("first\0suffix")?;
    }
    assert_eq!(*calls.borrow(), ["first\0suffix", "first"]);
    assert_eq!(console, b"first\0suffix\n");
    Ok(())
}

#[test]
fn warning_all_search_matches_and_warmup_sizing_abort_guards_are_literal()
-> Result<(), Box<dyn std::error::Error>> {
    for (warmup, sizing, kickoff, abort, warmup_added, sizing_added) in [
        (true, false, false, false, 1, 0),
        (true, true, false, false, 0, 1),
        (true, false, true, false, 0, 0),
        (true, false, false, true, 0, 0),
        (false, false, false, false, 0, 0),
        (false, true, true, true, 0, 1),
    ] {
        let mut state = state();
        state.warmup_flag = warmup;
        state.doing_sizing = sizing;
        state.kickoff_simulation = kickoff;
        state.tracking.abort_processing = abort;
        let mut error = Vec::new();
        let mut console = Vec::new();
        {
            let mut actual = backend(&mut state, &mut error, &mut console);
            actual.show_warning_error("Deprecated; Node Connection Error; deprecated")?;
        }
        assert_eq!(state.tracking.match_counts[2], 1);
        assert_eq!(state.tracking.match_counts[6], 1);
        assert_eq!(state.tracking.match_counts.iter().sum::<i32>(), 2);
        assert_eq!(state.tracking.total_warning_errors, 4);
        assert_eq!(state.tracking.total_severe_errors, 7);
        assert_eq!(
            state.tracking.total_warning_errors_during_warmup,
            2 + warmup_added
        );
        assert_eq!(
            state.tracking.total_warning_errors_during_sizing,
            1 + sizing_added
        );
        assert_eq!(
            String::from_utf8(error)?,
            "Program Version,authored version,authored IDD\n   ** Warning ** Deprecated; Node Connection Error; deprecated\n"
        );
    }
    Ok(())
}

#[test]
fn warning_then_continue_preserve_raw_callback_sqlite_arguments_and_counts()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let e_calls = calls.clone();
    let mut sqlite = TestSqlite {
        calls: calls.clone(),
        transaction: false,
        fail_record: false,
    };
    let mut e = move |_: &mut SourceErrorState, level: SourceErrorLevel, message: &[u8]| {
        let mut call = format!("callback:{}:", level as i32).into_bytes();
        call.extend_from_slice(message);
        e_calls.borrow_mut().push(call)
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut e);
        actual.show_warning_error("warning")?;
        actual.show_continue_error(" details")?;
    }
    assert_eq!(
        calls
            .borrow()
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        [
            "create:1:0:warning:1",
            "callback:2:warning",
            "update: details",
            "callback:0: details"
        ]
    );
    assert_eq!(state.tracking.total_warning_errors, 4);
    assert_eq!(state.tracking.total_severe_errors, 7);
    assert!(String::from_utf8(error.clone())?.ends_with("   **   ~~~   **  details\n"));
    // Message-only aggregate diagnostics retain count0 SQLite and no totals.
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut e);
        actual.show_warning_message("Deprecated summary warning")?;
        actual.show_severe_message("Deprecated summary severe")?;
    }
    assert_eq!(
        calls.borrow()[4..]
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        [
            "create:1:0:Deprecated summary warning:0",
            "callback:2:Deprecated summary warning",
            "create:1:1:Deprecated summary severe:0",
            "callback:3:Deprecated summary severe",
        ]
    );
    assert_eq!(state.tracking.total_warning_errors, 4);
    assert_eq!(state.tracking.total_severe_errors, 7);
    assert_eq!(state.tracking.total_warning_errors_during_warmup, 2);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 4);
    assert_eq!(state.tracking.total_warning_errors_during_sizing, 1);
    assert_eq!(state.tracking.total_severe_errors_during_sizing, 2);
    assert_eq!(state.tracking.match_counts[6], 2);
    assert_eq!(
        state.tracking.last_severe_error,
        "Deprecated summary severe"
    );
    assert!(
        String::from_utf8(error.clone())?.ends_with("   ** Severe  ** Deprecated summary severe\n")
    );
    let retained_tracking = state.tracking.clone();
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut e);
        actual.show_message("Setup volume note")?;
        actual.show_message("")?;
    }
    assert_eq!(
        calls.borrow()[8..]
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        [
            "create:1:-1:Setup volume note:0",
            "callback:1:Setup volume note"
        ]
    );
    assert_eq!(state.tracking, retained_tracking);
    assert!(
        String::from_utf8(error)?
            .ends_with("   ************* Setup volume note\n   *************\n")
    );

    // The byte route retains FF and embedded NUL through ERR/optional streams,
    // the borrowed SQLite update and the final callback, in that exact order.
    let raw = b"raw:\xff\0suffix";
    for (input_processing, print_console) in [(false, true), (true, true), (true, false)] {
        let mut raw_state = state.clone();
        raw_state.doing_input_processing = input_processing;
        raw_state.print_console_output = print_console;
        let prior = raw_state.tracking.clone();
        let (mut raw_error, mut raw_console, mut first, mut second) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        let raw_calls = Rc::new(RefCell::new(Vec::new()));
        let mut raw_sqlite = TestSqlite {
            calls: raw_calls.clone(),
            transaction: false,
            fail_record: false,
        };
        let callback_calls = raw_calls.clone();
        let mut callback =
            move |actual: &mut SourceErrorState, level: SourceErrorLevel, message: &[u8]| {
                assert_eq!(level, SourceErrorLevel::Continue);
                assert_eq!(actual.tracking.total_warning_errors, 4);
                let mut call = b"callback:0:".to_vec();
                call.extend_from_slice(message);
                callback_calls.borrow_mut().push(call);
            };
        {
            let mut actual = backend(&mut raw_state, &mut raw_error, &mut raw_console);
            actual.out_unit1 = Some(&mut first);
            actual.out_unit2 = Some(&mut second);
            actual.sqlite = Some(&mut raw_sqlite);
            actual.error_callback = Some(&mut callback);
            actual.show_continue_error_bytes(raw)?;
        }
        let mut formatted = b"   **   ~~~   ** ".to_vec();
        formatted.extend_from_slice(raw);
        assert_eq!(first, formatted);
        assert_eq!(second, formatted);
        let mut line = formatted.clone();
        line.push(b'\n');
        if input_processing {
            assert!(raw_error.is_empty());
            let expected = if print_console {
                line[2..].to_vec()
            } else {
                Vec::new()
            };
            assert_eq!(raw_console, expected);
        } else {
            assert_eq!(raw_error, line);
            assert!(raw_console.is_empty());
        }
        let mut update = b"update:".to_vec();
        update.extend_from_slice(raw);
        let mut callback = b"callback:0:".to_vec();
        callback.extend_from_slice(raw);
        assert_eq!(*raw_calls.borrow(), [update, callback]);
        assert_eq!(raw_state.tracking, prior);
    }
    Ok(())
}

#[test]
fn fatal_summary_reads_actual_state_after_display_callback()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let mut callback = |state: &mut SourceErrorState, _: &str| {
        state.tracking.total_severe_errors = 9;
        state.tracking.last_severe_error = "changed by callback".into();
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.message_callback = Some(&mut callback);
        assert!(matches!(
            actual.show_fatal_error("stop"),
            Err(SourceFatalStop::Fatal(_))
        ));
    }
    assert!(String::from_utf8(error)?.ends_with(
        "   ..... Reference severe error count=9\n   ..... Last severe error=changed by callback\n"
    ));
    Ok(())
}

struct FailingWriter;
impl Write for FailingWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("authored sink failure"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn first_sink_failure_preserves_state_and_never_claims_source_fatal() {
    let mut state = state();
    let mut error = FailingWriter;
    let mut console = Vec::new();
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        assert!(matches!(
            actual.show_fatal_error("stop"),
            Err(SourceFatalStop::Backend(
                SourceErrorBackendFailure::Effect {
                    operation: "error header",
                    ..
                }
            ))
        ));
    }
    assert!(state.output_error_header);
    assert!(console.is_empty());
    assert_eq!(state.tracking.total_severe_errors, 7);
    // A later optional sink failure preserves the already-written raw ERR
    // bytes, but must stop before output2, SQLite or the callback.
    let mut raw_state = state.clone();
    raw_state.output_error_header = false;
    let prior = raw_state.tracking.clone();
    let (mut raw_error, mut raw_console, mut optional2) = (Vec::new(), Vec::new(), Vec::new());
    let calls = Rc::new(RefCell::new(Vec::new()));
    let mut sqlite = TestSqlite {
        calls: calls.clone(),
        transaction: false,
        fail_record: false,
    };
    let cb_calls = calls.clone();
    let mut callback = move |_: &mut SourceErrorState, _: SourceErrorLevel, _: &[u8]| {
        cb_calls.borrow_mut().push(b"unreached callback".to_vec());
    };
    let mut failed = FailingWriter;
    {
        let mut actual = backend(&mut raw_state, &mut raw_error, &mut raw_console);
        actual.out_unit1 = Some(&mut failed);
        actual.out_unit2 = Some(&mut optional2);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut callback);
        assert!(matches!(
            actual.show_continue_error_bytes(b"raw:\xff\0suffix"),
            Err(SourceErrorBackendFailure::Effect {
                operation: "optional output1",
                ..
            })
        ));
    }
    assert_eq!(raw_error, b"   **   ~~~   ** raw:\xff\0suffix\n");
    assert!(raw_console.is_empty() && optional2.is_empty() && calls.borrow().is_empty());
    assert_eq!(raw_state.tracking, prior);
}

#[test]
fn database_failure_retains_written_prefix_but_not_final_error_callback_or_fatal()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let e_calls = calls.clone();
    let mut sqlite = TestSqlite {
        calls: calls.clone(),
        transaction: true,
        fail_record: true,
    };
    let mut e = move |_: &mut SourceErrorState, _: SourceErrorLevel, _: &[u8]| {
        e_calls.borrow_mut().push(b"unexpected callback".to_vec())
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut e);
        assert!(matches!(
            actual.show_fatal_error("stop"),
            Err(SourceFatalStop::Backend(
                SourceErrorBackendFailure::Effect {
                    operation: "SQLite create fatal",
                    ..
                }
            ))
        ));
    }
    assert!(String::from_utf8(error)?.ends_with("   ..... Last severe error=prior severe\n"));
    assert_eq!(
        calls
            .borrow()
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        ["create:1:2:stop:1"]
    );
    assert_eq!(console, b"**FATAL:stop\n");

    // Database update failure occurs after the complete raw line but before
    // the final callback; a test double records the original unformatted bytes.
    let mut raw_state = state.clone();
    let prior = raw_state.tracking.clone();
    let (mut raw_error, mut raw_console) = (Vec::new(), Vec::new());
    let raw_calls = Rc::new(RefCell::new(Vec::new()));
    let mut sqlite = TestSqlite {
        calls: raw_calls.clone(),
        transaction: false,
        fail_record: true,
    };
    let cb_calls = raw_calls.clone();
    let mut callback = move |_: &mut SourceErrorState, _: SourceErrorLevel, _: &[u8]| {
        cb_calls.borrow_mut().push(b"unreached callback".to_vec());
    };
    {
        let mut actual = backend(&mut raw_state, &mut raw_error, &mut raw_console);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut callback);
        assert!(matches!(
            actual.show_continue_error_bytes(b"raw:\xff\0suffix"),
            Err(SourceErrorBackendFailure::Effect {
                operation: "SQLite update continue",
                ..
            })
        ));
    }
    assert_eq!(raw_error, b"   **   ~~~   ** raw:\xff\0suffix\n");
    assert_eq!(*raw_calls.borrow(), [b"update:raw:\xff\0suffix".to_vec()]);
    assert!(raw_console.is_empty());
    assert_eq!(raw_state.tracking, prior);
    Ok(())
}

#[test]
fn fatal_without_transaction_does_not_commit() {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let mut sqlite = TestSqlite {
        calls: calls.clone(),
        transaction: false,
        fail_record: false,
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.sqlite = Some(&mut sqlite);
        assert!(matches!(
            actual.show_fatal_error("stop"),
            Err(SourceFatalStop::Fatal(_))
        ));
    }
    assert_eq!(
        calls
            .borrow()
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        ["create:1:2:stop:1", "transaction"]
    );
}

#[test]
fn undefined_counter_increment_stops_after_real_prior_match_writes() {
    let mut state = state();
    state.tracking.total_warning_errors = i32::MAX;
    let mut error = Vec::new();
    let mut console = Vec::new();
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        assert!(matches!(
            actual.show_warning_error("Deprecated"),
            Err(SourceErrorBackendFailure::UndefinedCounterIncrement {
                counter: "TotalWarningErrors"
            })
        ));
    }
    assert_eq!(state.tracking.match_counts[6], 1);
    assert_eq!(state.tracking.total_warning_errors, i32::MAX);
    assert!(error.is_empty());
    assert!(console.is_empty());
}

#[test]
fn severe_retains_real_counter_last_text_and_database_before_callback()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    state.warmup_flag = true;
    let mut error = Vec::new();
    let mut console = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let mut sqlite = TestSqlite {
        calls: calls.clone(),
        transaction: false,
        fail_record: false,
    };
    let callback_calls = calls.clone();
    let mut callback = move |state: &mut SourceErrorState, level: SourceErrorLevel, text: &[u8]| {
        assert_eq!(state.tracking.last_severe_error.as_bytes(), text);
        assert_eq!(state.tracking.total_severe_errors, 8);
        let mut call = format!("callback:{}:", level as i32).into_bytes();
        call.extend_from_slice(text);
        callback_calls.borrow_mut().push(call);
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut callback);
        actual.show_severe_error("Deprecated severe")?;
    }
    assert_eq!(state.tracking.match_counts[6], 1);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 5);
    assert_eq!(state.tracking.total_severe_errors_during_sizing, 2);
    assert_eq!(state.tracking.total_warning_errors, 3);
    assert_eq!(
        calls
            .borrow()
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        [
            "create:1:1:Deprecated severe:1",
            "callback:3:Deprecated severe"
        ]
    );
    assert!(String::from_utf8(error)?.ends_with("   ** Severe  ** Deprecated severe\n"));
    assert!(console.is_empty());
    Ok(())
}

#[test]
fn severe_failure_preserves_prior_counts_but_not_unreached_last_text() {
    let mut state = state();
    state.doing_sizing = true;
    state.warmup_flag = true;
    let mut error = FailingWriter;
    let mut console = Vec::new();
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        assert!(matches!(
            actual.show_severe_error("new severe"),
            Err(SourceErrorBackendFailure::Effect {
                operation: "error header",
                ..
            })
        ));
    }
    assert_eq!(state.tracking.total_severe_errors, 8);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 4);
    assert_eq!(state.tracking.total_severe_errors_during_sizing, 3);
    assert_eq!(state.tracking.last_severe_error, "prior severe");
    assert!(state.output_error_header);
}

#[test]
fn severe_database_failure_retains_new_last_text_and_no_callback()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let mut sqlite = TestSqlite {
        calls: calls.clone(),
        transaction: false,
        fail_record: true,
    };
    let callback_calls = calls.clone();
    let mut callback = move |_: &mut SourceErrorState, _: SourceErrorLevel, _: &[u8]| {
        callback_calls
            .borrow_mut()
            .push(b"unreached callback".to_vec());
    };
    {
        let mut actual = backend(&mut state, &mut error, &mut console);
        actual.sqlite = Some(&mut sqlite);
        actual.error_callback = Some(&mut callback);
        assert!(matches!(
            actual.show_severe_error("new severe"),
            Err(SourceErrorBackendFailure::Effect {
                operation: "SQLite create severe",
                ..
            })
        ));
    }
    assert_eq!(state.tracking.last_severe_error, "new severe");
    assert_eq!(
        calls
            .borrow()
            .iter()
            .map(|message| std::str::from_utf8(message)
                .expect("original authored UTF-8 call")
                .to_owned())
            .collect::<Vec<_>>(),
        ["create:1:1:new severe:1"]
    );
    assert!(String::from_utf8(error)?.ends_with("   ** Severe  ** new severe\n"));
    Ok(())
}
