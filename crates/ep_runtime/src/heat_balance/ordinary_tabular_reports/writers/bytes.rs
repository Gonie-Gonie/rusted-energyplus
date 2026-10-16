//! Original char-string helpers; lengths, slices and padding count bytes.
use super::Error;
use crate::heat_balance::ordinary_utility_bills::SourceCostEstimateMonetary;

pub(super) fn strip(s: &[u8]) -> &[u8] {
    let start = s.iter().position(|&c| c != b' ').unwrap_or(s.len());
    let end = s.iter().rposition(|&c| c != b' ').map_or(start, |i| i + 1);
    &s[start..end]
}
pub(super) fn trim(s: &[u8]) -> &[u8] {
    &s[..s.iter().rposition(|&c| c != b' ').map_or(0, |i| i + 1)]
}
pub(super) fn right_sized(s: &[u8], width: usize) -> Vec<u8> {
    // Objexx sized first truncates/right-pads, then rjustified moves only
    // trailing spaces to the front. Leading spaces remain part of the text.
    let mut sized = s[..s.len().min(width)].to_vec();
    sized.resize(width, b' ');
    let len_trim = trim(&sized).len();
    let mut result = vec![b' '; sized.len() - len_trim];
    result.extend_from_slice(&sized[..len_trim]);
    result
}
pub(super) fn same(a: &[u8], b: &[u8]) -> bool {
    a.eq_ignore_ascii_case(b)
}
pub(super) fn find(s: &[u8], needle: &[u8]) -> Option<usize> {
    s.windows(needle.len()).position(|w| w == needle)
}
pub(super) fn anchor(report: &[u8], object: &[u8]) -> Vec<u8> {
    const VALID: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_:.";
    let valid = |c: &&u8| VALID.contains(*c);
    let mut result: Vec<u8> = report.iter().filter(valid).copied().collect();
    result.extend_from_slice(b"::");
    result.extend(object.iter().filter(valid).copied());
    result
}
pub(super) fn element(s: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut other = true;
    for &c in s {
        if c.is_ascii_uppercase() {
            result.push(if other { c } else { c + 32 });
            other = false;
        } else if c.is_ascii_lowercase() {
            result.push(if other { c - 32 } else { c });
            other = false;
        } else if c.is_ascii_digit() {
            if result.is_empty() {
                result.push(b't');
            }
            result.push(c);
            other = false;
        } else if c == b'[' {
            break;
        } else {
            other = true;
        }
    }
    result
}
pub(super) fn units(s: &[u8]) -> Vec<u8> {
    if let (Some(left), Some(right)) = (
        s.iter().position(|&c| c == b'['),
        s.iter().position(|&c| c == b']'),
    ) {
        // std::string::size_type subtraction is unsigned. A right bracket
        // preceding a left one produces a large count, which substr clamps.
        if right.wrapping_sub(left) >= 1 {
            let start = left + 1;
            let count = right.wrapping_sub(left).wrapping_sub(1);
            return s[start..start + count.min(s.len() - start)].to_vec();
        }
    }
    Vec::new()
}
pub(super) fn currency(
    input: &[u8],
    html: bool,
    owner: &SourceCostEstimateMonetary,
) -> Result<Vec<u8>, Error> {
    let mut output = trim(input).to_vec();
    while let Some(loc) = find(&output, b"~~$~~") {
        let ordinal = owner
            .selected_monetary_unit
            .checked_sub(1)
            .and_then(|i| usize::try_from(i).ok())
            .ok_or(Error::Unavailable(
                "actual InsertCurrencySymbol selectedMonetaryUnit subscript",
            ))?;
        let unit = owner
            .monetary_unit
            .as_ref()
            .and_then(|rows| rows.get(ordinal))
            .ok_or(Error::Unavailable(
                "same actual InsertCurrencySymbol monetaryUnit row",
            ))?;
        let prefix = input.get(..loc).ok_or(Error::Unavailable(
            "actual InsertCurrencySymbol original-input substr position",
        ))?;
        let mut next = prefix.to_vec();
        next.extend_from_slice(if html {
            unit.html.as_bytes()
        } else {
            unit.txt.as_bytes()
        });
        next.extend_from_slice(&output[loc + 5..]);
        output = next;
    }
    Ok(output)
}
