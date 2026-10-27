// Copyright 2006 The RE2 Authors. All Rights Reserved.
// BSD-style license: ../RE2-LICENSE.
//! Native CharClassBuilder set operations used by PushRegexp and Round3.
use super::flags;
use crate::Status;

pub(super) fn normalize(mut ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    ranges.retain(|&(lo, hi)| lo <= hi);
    ranges.sort_unstable();
    let mut out: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
    for (lo, hi) in ranges {
        if let Some(last) = out.last_mut()
            && lo <= last.1.saturating_add(1)
        {
            last.1 = last.1.max(hi);
        } else {
            out.push((lo, hi));
        }
    }
    out
}

/// AddRange's exact set result and already-contained early return.
pub(super) fn add(ranges: &mut Vec<(u32, u32)>, lo: u32, hi: u32) -> Result<bool, Status> {
    if lo > hi {
        return Ok(false);
    }
    let first = ranges.partition_point(|&(_, end)| end < lo);
    if ranges
        .get(first)
        .is_some_and(|&(start, end)| start <= lo && hi <= end)
    {
        return Ok(false);
    }
    let begin = if first > 0 && ranges[first - 1].1.saturating_add(1) >= lo {
        first - 1
    } else {
        first
    };
    let mut end = begin;
    let mut merged = (lo, hi);
    while let Some(&(start, finish)) = ranges.get(end) {
        if start > merged.1.saturating_add(1) {
            break;
        }
        merged.0 = merged.0.min(start);
        merged.1 = merged.1.max(finish);
        end += 1;
    }
    if begin == end {
        ranges.try_reserve(1).map_err(|_| super::allocation())?;
    }
    drop(ranges.splice(begin..end, std::iter::once(merged)));
    Ok(true)
}

pub(super) fn contains(ranges: &[(u32, u32)], rune: u32) -> bool {
    let at = ranges.partition_point(|&(_, hi)| hi < rune);
    ranges.get(at).is_some_and(|&(lo, _)| lo <= rune)
}

/// Round3 calls AddRangeFlags only for SINGLE literal runes. Fold against the
/// existing builder: folding a new isolated set and unioning it is different.
pub(super) fn add_literal_flags(
    ranges: &mut Vec<(u32, u32)>,
    rune: u32,
    bits: u16,
) -> Result<(), Status> {
    if rune == 10 && (bits & flags::CLASS_NL == 0 || bits & flags::NEVER_NL != 0) {
        return Ok(());
    }
    if bits & flags::FOLD_CASE == 0 {
        add(ranges, rune, rune)?;
        return Ok(());
    }
    add_folded_range(ranges, rune, rune, 0)?;
    Ok(())
}

// Literal parse.cc345-394. This bounded recursion follows fixed Unicode fold
// cycles (at most four), never user regexp nesting. The depth guard is Native's.
fn add_folded_range(
    ranges: &mut Vec<(u32, u32)>,
    mut lo: u32,
    hi: u32,
    depth: u32,
) -> Result<(), Status> {
    if depth > 10 || !add(ranges, lo, hi)? {
        return Ok(());
    }
    while lo <= hi {
        let Some((start, end, delta)) = crate::unicode::case_fold_row(lo) else {
            break;
        };
        if lo < start {
            lo = start;
            continue;
        }
        let mut lo1 = lo;
        let mut hi1 = hi.min(end);
        match delta {
            1 => {
                if lo1 % 2 == 1 {
                    lo1 -= 1;
                }
                if hi1.is_multiple_of(2) {
                    hi1 += 1;
                }
            }
            -1 => {
                if lo1.is_multiple_of(2) {
                    lo1 -= 1;
                }
                if hi1 % 2 == 1 {
                    hi1 += 1;
                }
            }
            _ => {
                lo1 = lo1.wrapping_add_signed(delta);
                hi1 = hi1.wrapping_add_signed(delta);
            }
        }
        add_folded_range(ranges, lo1, hi1, depth + 1)?;
        lo = end + 1;
    }
    Ok(())
}
