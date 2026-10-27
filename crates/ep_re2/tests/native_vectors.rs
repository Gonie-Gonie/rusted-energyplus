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
    for (index, line) in data.lines().enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 7 {
            return Err(format!("bad fixture field count at {}", index + 1));
        }
        let pattern = decode_hex(fields[0])?;
        let text = decode_hex(fields[1])?;
        let pattern = std::str::from_utf8(&pattern).map_err(|e| e.to_string())?;
        let expression = Re2::new(pattern);
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
    Ok(())
}
