use super::*;

#[test]
fn rewind_and_backspace_use_actual_bytes_and_retain_read_attempts() {
    for bytes in [
        b"a\nbb\n".as_slice(),
        b"a\r\nbb\r\n".as_slice(),
        b"a\nbb".as_slice(),
    ] {
        let mut input = RawEpwInput::new_unopened(bytes.to_vec());
        input.reopen();
        input.read_line().unwrap();
        let second = input.read_line().unwrap();
        input.read_line().unwrap();
        input.backspace().unwrap();
        assert_eq!(input.byte_cursor, second.start_byte);
        assert!(input.snapshot().good);
        assert_eq!(input.read_line().unwrap().data, "bb");
        input.rewind();
        assert_eq!(input.byte_cursor, 0);
        assert_eq!(input.line_reads, 4);
        assert!(input.snapshot().good);
    }
}

#[test]
fn backspace_matches_source_cursor_one_and_closed_owner_edges() {
    let mut input = RawEpwInput::new_unopened(b"x\n".to_vec());
    let closed = input.snapshot();
    input.backspace().unwrap();
    input.rewind();
    assert_eq!(input.snapshot(), closed);
    input.reopen();
    input.byte_cursor = 1;
    input.backspace().unwrap();
    assert_eq!(input.byte_cursor, 1);
    input.byte_cursor = 0;
    input.backspace().unwrap();
    assert_eq!(input.byte_cursor, 0);
    input.byte_cursor = 99;
    assert!(matches!(
        input.backspace(),
        Err(RawEpwHeaderError::OutsideBoundedDomain(_))
    ));
    input.rewind();
    assert_eq!(input.byte_cursor, 0);
}

#[test]
fn read_result_update_retains_data_but_observes_actual_failed_read() {
    let mut input = RawEpwInput::new_unopened(b"retained\n".to_vec());
    input.reopen();
    let mut retained = input.read_line().unwrap();
    let failed = input.read_line().unwrap();
    assert!(failed.data.is_empty());
    retained.update(failed);
    assert_eq!(retained.data, "retained");
    assert!(!retained.read_good && retained.eof);
    assert_eq!(retained.start_byte, 9);
    assert_eq!(retained.end_byte, 9);
}

#[test]
fn closing_discards_stream_flags_and_opening_restarts_without_resetting_attempts() {
    let mut input = RawEpwInput::new_unopened(b"row".to_vec());
    input.reopen();
    input.read_line().unwrap();
    input.read_line().unwrap();
    assert!(input.snapshot().eof && input.snapshot().fail);
    input.close();
    assert_eq!(
        input.snapshot(),
        RawEpwStreamState {
            is_open: false,
            good: false,
            eof: false,
            fail: false,
            bad: true,
            position_byte: None,
        }
    );
    input.rewind();
    assert!(!input.snapshot().is_open);
    input.reopen();
    assert_eq!(input.line_reads, 2);
    assert_eq!(input.byte_cursor, 0);
    assert!(input.snapshot().good);
}
