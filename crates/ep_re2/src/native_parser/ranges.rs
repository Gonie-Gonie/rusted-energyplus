// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see ../RE2-LICENSE.
//! Character-class union/complement; ranges retain the Native Rune domain.
use super::RUNE_MAX;

pub(super) fn normalize(mut ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    ranges.retain(|&(lo, hi)| lo <= hi && lo <= RUNE_MAX);
    for (_, hi) in &mut ranges {
        *hi = (*hi).min(RUNE_MAX);
    }
    ranges.sort_unstable();
    let mut out: Vec<(u32, u32)> = Vec::new();
    for (lo, hi) in ranges {
        if let Some(last) = out.last_mut()
            && lo <= last.1 + 1
        {
            last.1 = last.1.max(hi);
        } else {
            out.push((lo, hi));
        }
    }
    out
}

pub(super) fn complement(ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let ranges = normalize(ranges);
    let mut out = Vec::new();
    let mut next = 0;
    for (lo, hi) in ranges {
        if next < lo {
            out.push((next, lo - 1));
        }
        next = hi + 1;
    }
    if next <= RUNE_MAX {
        out.push((next, RUNE_MAX));
    }
    out
}
