// Copyright 2006, 2008 The RE2 Authors. All Rights Reserved.
// Rust port of the pinned RE2 Unicode lookup/fold operations; see crate LICENSE.RE2.
//! Pinned Unicode 14.0 groups and cyclic simple folding, independent of host Unicode.

#[path = "unicode_data.rs"]
mod data;

const EVEN_ODD: i32 = 1;
const ODD_EVEN: i32 = -1;
const ANY: &[(u32, u32)] = &[(0, 65535), (65536, 0x10ffff)];

/// The exact case-sensitive native property/script lookup, including its fake Any group.
pub(crate) fn group(name: &str) -> Option<&'static [(u32, u32)]> {
    if name == "Any" {
        return Some(ANY);
    }
    data::UNICODE_GROUPS
        .iter()
        .find_map(|&(key, ranges)| (key == name).then_some(ranges))
}

/// The exact native Perl spelling and sign; \s intentionally excludes vertical tab.
pub(crate) fn perl_group(name: &str) -> Option<(bool, &'static [(u32, u32)])> {
    data::PERL_GROUPS
        .iter()
        .find_map(|&(key, sign, ranges)| (key == name).then_some((sign, ranges)))
}

/// The exact native POSIX bracket spelling and sign, including negated entries.
pub(crate) fn posix_group(name: &str) -> Option<(bool, &'static [(u32, u32)])> {
    data::POSIX_GROUPS
        .iter()
        .find_map(|&(key, sign, ranges)| (key == name).then_some((sign, ranges)))
}

// LookupCaseFold: the first row containing rune, or the next row after rune.
fn lookup(rune: u32) -> Option<&'static (u32, u32, i32)> {
    let index = data::CASE_FOLD.partition_point(|&(_, hi, _)| hi < rune);
    data::CASE_FOLD.get(index)
}

// ApplyFold. The fixed cycle table contains no EvenOddSkip/OddEvenSkip deltas;
// those occur only in the separate lowercase optimization table not used here.
fn apply(rune: u32, delta: i32) -> u32 {
    match delta {
        EVEN_ODD => rune ^ 1,
        ODD_EVEN => {
            if rune % 2 == 1 {
                rune + 1
            } else {
                rune - 1
            }
        }
        _ => rune.wrapping_add_signed(delta),
    }
}

/// Native CycleFoldRune: advance once through its pinned equivalence cycle.
pub(crate) fn fold(rune: u32) -> u32 {
    lookup(rune)
        .filter(|&&(lo, _, _)| lo <= rune)
        .map_or(rune, |&(_, _, delta)| apply(rune, delta))
}

// CharClassBuilder::AddRange's observable set result and "anything added" return.
// Normalized disjoint intervals make an existing containing range sufficient.
fn add_range(ranges: &mut Vec<(u32, u32)>, lo: u32, hi: u32) -> bool {
    if lo > hi || ranges.iter().any(|&(a, b)| a <= lo && hi <= b) {
        return false;
    }
    ranges.push((lo, hi));
    ranges.sort_unstable();
    let mut merged: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
    for &(a, b) in ranges.iter() {
        if let Some(last) = merged.last_mut()
            && a <= last.1.saturating_add(1)
        {
            last.1 = last.1.max(b);
            continue;
        }
        merged.push((a, b));
    }
    *ranges = merged;
    true
}

// Literal AddFoldedRange source order. Its recursion depends only on fixed
// folding cycles (at most four); user regex nesting never recurses here.
fn add_folded_range(ranges: &mut Vec<(u32, u32)>, mut lo: u32, hi: u32, depth: u32) {
    if depth > 10 || !add_range(ranges, lo, hi) {
        return;
    }
    while lo <= hi {
        let Some(&(start, end, delta)) = lookup(lo) else {
            break;
        };
        if lo < start {
            lo = start;
            continue;
        }
        let mut lo1 = lo;
        let mut hi1 = hi.min(end);
        match delta {
            EVEN_ODD => {
                if lo1 % 2 == 1 {
                    lo1 -= 1;
                }
                if hi1 % 2 == 0 {
                    hi1 += 1;
                }
            }
            ODD_EVEN => {
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
        add_folded_range(ranges, lo1, hi1, depth + 1);
        lo = end + 1;
    }
}

/// Add each valid rune interval and all fixed-table fold equivalents.
/// The parser owns newline removal, negation and endpoint validation.
pub(crate) fn fold_ranges(input: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut ranges = Vec::new();
    for &(lo, hi) in input {
        add_folded_range(&mut ranges, lo, hi, 0);
    }
    ranges
}

// Add this read-only wrapper inside the existing crate::unicode module.
// It reuses the already pinned private LookupCaseFold port and table.
pub(crate) fn case_fold_row(rune: u32) -> Option<(u32, u32, i32)> {
    lookup(rune).copied()
}
