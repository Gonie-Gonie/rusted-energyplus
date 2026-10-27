//! Actual pinned-source Native constructor and FullMatch results, not self-pairs.
use ep_re2::Re2;

fn decode_hex(text: &str) -> Result<Vec<u8>, String> {
    if !text.len().is_multiple_of(2) {
        return Err("odd fixture hex length".into());
    }
    text.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => Ok(byte - b'0'),
                b'a'..=b'f' => Ok(byte - b'a' + 10),
                _ => Err("invalid fixture hex digit".to_owned()),
            };
            Ok((digit(pair[0])? << 4) | digit(pair[1])?)
        })
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Writing to String has no fallible external sink.
        let _ = write!(result, "{byte:02x}");
    }
    result
}

#[test]
fn pinned_native_constructor_and_full_match_vectors() -> Result<(), String> {
    let data = include_str!("data/native-default.tsv");
    let mut rows = 0;
    // Adjacent rows reuse one immutable compiled expression. Every observed
    // API field and text match is still checked against the actual Native row.
    let mut previous: Option<(&str, Re2)> = None;
    for (index, line) in data.lines().enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 7 {
            return Err(format!("bad fixture field count at {}", index + 1));
        }
        if previous
            .as_ref()
            .is_none_or(|(encoded, _)| *encoded != fields[0])
        {
            let pattern = decode_hex(fields[0])?;
            let pattern = std::str::from_utf8(&pattern).map_err(|e| e.to_string())?;
            previous = Some((fields[0], Re2::new(pattern)));
        }
        let expression = &previous
            .as_ref()
            .ok_or("missing compiled unit expression")?
            .1;
        let text = decode_hex(fields[1])?;
        let actual = [
            u8::from(expression.ok()).to_string(),
            (expression.status().code as u8).to_string(),
            hex(expression.error().as_bytes()),
            hex(expression.status().arg.as_bytes()),
            u8::from(expression.full_match(&text)).to_string(),
        ];
        let expected: [String; 5] = std::array::from_fn(|at| fields[at + 2].to_owned());
        assert_eq!(
            actual,
            expected,
            "Native vector {}: pattern={} text={}",
            index + 1,
            fields[0],
            fields[1]
        );
        rows += 1;
    }
    assert_eq!(rows, 10_607);

    // Actual Native records include payload LF/NUL and byte-truncated UTF-8.
    // The observed source-unit origin is explicit fixture metadata, so no
    // current checkout path supplies expected diagnostics. The heavy final
    // compile-budget observation is retained outside this default table.
    let mut records = include_str!("data/native-diagnostics.tsv").lines();
    let metadata: Vec<_> = records
        .next()
        .ok_or("missing diagnostic origin")?
        .split('\t')
        .collect();
    if metadata.len() != 3 || metadata[0] != "#origin_hex" {
        return Err("bad diagnostic origin metadata".into());
    }
    let source_file = decode_hex(metadata[1])?;
    let line_ending = decode_hex(metadata[2])?;
    let origin = ep_re2::TargetDiagnosticOrigin {
        source_file: &source_file,
        line_ending: &line_ending,
    };
    let mut diagnostic_rows = 0;
    for (index, line) in records.enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 8 {
            return Err(format!(
                "bad diagnostic fixture field count at {}",
                index + 1
            ));
        }
        let pattern = decode_hex(fields[0])?;
        let pattern = std::str::from_utf8(&pattern).map_err(|error| error.to_string())?;
        let text = decode_hex(fields[1])?;
        let mut diagnostics = Vec::new();
        // Each Native row constructed a fresh expression, including repeats.
        let expression = Re2::new_logged_to(pattern, origin, &mut diagnostics);
        let matched = expression.full_match_logged_to(&text, origin, &mut diagnostics);
        let actual = [
            u8::from(expression.ok()).to_string(),
            (expression.status().code as u8).to_string(),
            hex(expression.error().as_bytes()),
            hex(expression.status().arg.as_bytes()),
            u8::from(matched).to_string(),
        ];
        let expected: [String; 5] = std::array::from_fn(|at| fields[at + 2].to_owned());
        assert_eq!(
            actual,
            expected,
            "Native diagnostic API vector {}",
            index + 1
        );
        assert_eq!(
            diagnostics,
            decode_hex(fields[7])?,
            "Native diagnostic bytes vector {}",
            index + 1
        );
        diagnostic_rows += 1;
    }
    assert_eq!(diagnostic_rows, 15);
    Ok(())
}
