//! Native OutputReportTabular17985–18102 operates on char bytes, not code points.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

fn byte_at(input: &[u8], index: usize) -> Result<u8, Error> {
    // std::string::operator[](size()) is the terminating zero byte. Native's
    // short-circuit lookahead can read exactly this cell for a short sequence.
    if index == input.len() {
        return Ok(0);
    }
    input.get(index).copied().ok_or(Error::Unavailable(
        "actual ConvertToEscaped string subscript beyond size",
    ))
}

fn unicode_to_utf8(value: u64) -> Vec<u8> {
    let mut s = Vec::new();
    if value <= 0x7f {
        s.push(value as u8);
    } else if value <= 0x7ff {
        s.push((0xc0 | ((value >> 6) & 0x1f)) as u8);
        s.push((0x80 | (value & 0x3f)) as u8);
    } else if value <= 0xffff {
        s.push((0xe0 | ((value >> 12) & 0x0f)) as u8);
        s.push((0x80 | ((value >> 6) & 0x3f)) as u8);
        s.push((0x80 | (value & 0x3f)) as u8);
    } else if value <= 0x10ffff {
        s.push((0xf0 | ((value >> 18) & 0x07)) as u8);
        s.push((0x80 | ((value >> 12) & 0x3f)) as u8);
        s.push((0x80 | ((value >> 6) & 0x3f)) as u8);
        s.push((0x80 | (value & 0x3f)) as u8);
    }
    s
}

// std::stoul(base16, nullptr) reads a valid prefix and ignores its suffix.
// At most four bytes are passed here, so positive overflow is impossible.
fn hexadecimal_prefix(bytes: &[u8]) -> Result<u64, Error> {
    let mut i = 0;
    while bytes
        .get(i)
        .is_some_and(|c| matches!(*c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c))
    {
        i += 1;
    }
    let negative = bytes.get(i) == Some(&b'-');
    if negative || bytes.get(i) == Some(&b'+') {
        i += 1;
    }
    if bytes.get(i) == Some(&b'0')
        && bytes.get(i + 1).is_some_and(|c| matches!(*c, b'x' | b'X'))
        && bytes.get(i + 2).is_some_and(u8::is_ascii_hexdigit)
    {
        i += 2;
    }
    let start = i;
    let mut value = 0_u64;
    while let Some(&c) = bytes.get(i) {
        let digit = match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => break,
        };
        value = value * 16 + u64::from(digit);
        i += 1;
    }
    if i == start {
        return Err(Error::Unavailable(
            "actual ConvertToEscaped18082/18085 std::stoul invalid_argument",
        ));
    }
    // Nonzero negative unsigned-long results exceed 0x10ffff on both LLP64
    // Windows and LP64. ConvertUnicodeToUTF8 therefore returns empty there.
    Ok(if negative {
        value.wrapping_neg()
    } else {
        value
    })
}

// Original tests decimal176 and hexadecimal0xb0 in order; preserve that literal compatibility mapping.
#[allow(clippy::ifs_same_cond)]
pub(super) fn convert_to_escaped(input: &[u8], is_xml: bool) -> Result<Vec<u8>, Error> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    let mut s = Vec::with_capacity(input.len());
    let mut index = 0;
    loop {
        if index == input.len() {
            break;
        }
        let mut c = input[index];
        index += 1;
        if c == b'"' && is_xml {
            s.extend_from_slice(b"&quot;");
        } else if c == b'&' {
            s.extend_from_slice(b"&amp;");
        } else if c == b'\'' && is_xml {
            s.extend_from_slice(b"&apos;");
        } else if c == b'<' {
            s.extend_from_slice(b"&lt;");
        } else if c == b'>' {
            s.extend_from_slice(b"&gt;");
        } else if c == 176 && !is_xml {
            s.extend_from_slice(b"&deg;");
        } else if c == 226
            && byte_at(input, index)? == 137
            && byte_at(input, index + 1)? == 164
            && !is_xml
        {
            s.extend_from_slice(b"&le;");
            index += 2;
        } else if c == 226
            && byte_at(input, index)? == 137
            && byte_at(input, index + 1)? == 165
            && !is_xml
        {
            s.extend_from_slice(b"&ge;");
            index += 2;
        } else if c == 0xc2 {
            if index == input.len() {
                s.push(0xc2);
            } else {
                c = input[index];
                index += 1;
                if c == 0xb0 && !is_xml {
                    s.extend_from_slice(b"&deg;");
                } else {
                    s.push(0xc2);
                    s.push(c);
                }
            }
        } else if c == 0xb0 && !is_xml {
            s.extend_from_slice(b"&deg;");
        } else if c == b'\\' {
            if index == input.len() {
                break;
            }
            c = input[index];
            index += 1;
            if c == b'"' && is_xml {
                s.extend_from_slice(b"&quot;");
            } else if c == b'\'' && is_xml {
                s.extend_from_slice(b"&apos;");
            } else if c == b'u' || c == b'x' {
                let remaining = i32::try_from(input.len() - index)
                    .map_err(|_| Error::Unavailable("actual ConvertToEscaped18079 int domain"))?;
                let mut code_point = 0;
                if c == b'u' && remaining > 3 {
                    code_point = hexadecimal_prefix(&input[index..index + 4])?;
                    index += 4;
                } else if c == b'x' && remaining > 1 {
                    code_point = hexadecimal_prefix(&input[index..index + 2])?;
                    index += 2;
                }
                let unicode = unicode_to_utf8(code_point);
                if unicode == b"\xc2\xb0" && !is_xml {
                    s.extend_from_slice(b"&deg;");
                } else {
                    s.extend_from_slice(&unicode);
                }
            } else {
                s.push(c);
            }
        } else {
            s.push(c);
        }
    }
    Ok(s)
}
