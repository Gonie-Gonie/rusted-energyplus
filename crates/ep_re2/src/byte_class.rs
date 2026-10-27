// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from pinned RE2 under its BSD-style license; see crate LICENSE.RE2.
//! Pinned forward UTF8 rune-range byte compiler.
//!
//! Each Native byte-range path is represented directly as a sequence of one to
//! four inclusive byte ranges. Trie/suffix-cache sharing affects representation,
//! not this union. In particular, the primary80..10ffff special case deliberately
//! accepts some noncanonical encodings; no host Unicode decoder is involved.
use crate::{ErrorCode, Status};

#[derive(Clone, Debug, PartialEq, Eq)]
struct ByteSequence {
    ranges: [(u8, u8); 4],
    width: u8,
}

/// The exact byte-path union from the supplied Native character-class ranges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ByteClass {
    sequences: Vec<ByteSequence>,
    width_by_first_byte: [u8; 256],
}

fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}

fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}

impl ByteClass {
    /// Port AddRuneRangeUTF8691-787, retaining the exact common-range branch.
    /// The caller supplies the primary character-class ranges, already in its
    /// own retained form. Merging adjacent input ranges here would be incorrect.
    pub(crate) fn from_ranges(ranges: &[(u32, u32)]) -> Result<Self, Status> {
        let mut result = Self {
            sequences: Vec::new(),
            width_by_first_byte: [0; 256],
        };
        let mut work = Vec::new();
        work.try_reserve(ranges.len()).map_err(|_| allocation())?;
        work.extend(ranges.iter().rev().copied());
        while let Some((lo, hi)) = work.pop() {
            if lo > hi {
                continue;
            }

            // compile.cc695-699 runs before all other range splitting.
            if lo == 0x80 && hi == 0x10ffff {
                result.add_common_non_ascii()?;
                continue;
            }

            if let Some((first, second)) = split_range(lo, hi)? {
                // The primary recursively emits the lower range before the
                // upper range. An explicit LIFO stack preserves that order.
                work.try_reserve(2).map_err(|_| allocation())?;
                work.push(second);
                work.push(first);
                continue;
            }

            if hi < 0x80 {
                result.add_sequence(&[(lo as u8, hi as u8)])?;
                continue;
            }

            // After splitting, every encoded byte varies independently inside
            // these two endpoints, exactly as Native735-786 constructs it.
            let (lower, width) = encode_rune(lo);
            let (upper, upper_width) = encode_rune(hi);
            if width != upper_width {
                // Native DCHECK_EQ(n,m) asserts the same internal association.
                // Actual parser-generated Rune ranges cannot reach this case.
                return Err(internal());
            }
            let mut sequence = [(0_u8, 0_u8); 4];
            for index in 0..width {
                sequence[index] = (lower[index], upper[index]);
            }
            result.add_sequence(&sequence[..width])?;
        }
        Ok(result)
    }

    /// Match one bytecode path; first-byte classes determine its consuming width.
    /// Native noncanonical paths are tested as bytes, never decoded as host chars.
    pub(crate) fn match_width(&self, text: &[u8]) -> Option<usize> {
        let first = *text.first()?;
        let width = usize::from(self.width_by_first_byte[usize::from(first)]);
        if width == 0 {
            return None;
        }
        let prefix = text.get(..width)?;
        self.sequences
            .iter()
            .any(|sequence| {
                usize::from(sequence.width) == width
                    && prefix
                        .iter()
                        .zip(&sequence.ranges[..width])
                        .all(|(&byte, &(lo, hi))| lo <= byte && byte <= hi)
            })
            .then_some(width)
    }

    fn add_common_non_ascii(&mut self) -> Result<(), Status> {
        // Exact forward Add_80_10ffff675-687. In particular do NOT narrow
        // E0's next byte to A0..BF, F0's to90..BF or F4's to80..8F.
        self.add_sequence(&[(0xc2, 0xdf), (0x80, 0xbf)])?;
        self.add_sequence(&[(0xe0, 0xef), (0x80, 0xbf), (0x80, 0xbf)])?;
        self.add_sequence(&[(0xf0, 0xf4), (0x80, 0xbf), (0x80, 0xbf), (0x80, 0xbf)])
    }

    fn add_sequence(&mut self, ranges: &[(u8, u8)]) -> Result<(), Status> {
        if ranges.is_empty() || ranges.len() > 4 || ranges.iter().any(|&(lo, hi)| lo > hi) {
            return Err(internal());
        }
        let width = ranges.len() as u8;
        for first in ranges[0].0..=ranges[0].1 {
            let slot = &mut self.width_by_first_byte[usize::from(first)];
            if *slot != 0 && *slot != width {
                // Forward UTF8 paths have disjoint lead-byte width families;
                // allowing an ambiguous internal class would lose NFA paths.
                return Err(internal());
            }
            *slot = width;
        }
        let mut sequence = ByteSequence {
            ranges: [(0, 0); 4],
            width,
        };
        sequence.ranges[..ranges.len()].copy_from_slice(ranges);
        self.sequences.try_reserve(1).map_err(|_| allocation())?;
        self.sequences.push(sequence);
        Ok(())
    }
}

type Split = Option<((u32, u32), (u32, u32))>;

fn split_range(lo: u32, hi: u32) -> Result<Split, Status> {
    // Native701-709: first split across the MaxRune(1..3) boundaries.
    for boundary in [0x7f_u32, 0x7ff, 0xffff] {
        if lo <= boundary && boundary < hi {
            return Ok(Some(((lo, boundary), (boundary + 1, hi))));
        }
    }
    if hi < 0x80 {
        return Ok(None);
    }
    // Native718-733: align low and high endpoints to whole trailing-byte
    // blocks. Tests run from the smallest six-bit suffix to the largest.
    for count in 1..4 {
        let mask = (1_u32 << (6 * count)) - 1;
        if (lo & !mask) != (hi & !mask) {
            if lo & mask != 0 {
                let boundary = lo | mask;
                let after = boundary.checked_add(1).ok_or_else(internal)?;
                return Ok(Some(((lo, boundary), (after, hi))));
            }
            if hi & mask != mask {
                let boundary = hi & !mask;
                let before = boundary.checked_sub(1).ok_or_else(internal)?;
                return Ok(Some(((lo, before), (boundary, hi))));
            }
        }
    }
    Ok(None)
}

/// Native util/rune.cc126-180 runetochar, including surrogate Rune encodings.
/// Only the Native >Runemax conversion is applied; Rust char is never involved.
pub(crate) fn encode_rune(rune: u32) -> ([u8; 4], usize) {
    let mut output = [0_u8; 4];
    if rune <= 0x7f {
        output[0] = rune as u8;
        return (output, 1);
    }
    if rune <= 0x7ff {
        output[0] = 0xc0 | (rune >> 6) as u8;
        output[1] = 0x80 | (rune & 0x3f) as u8;
        return (output, 2);
    }
    let rune = if rune > 0x10ffff { 0xfffd } else { rune };
    if rune <= 0xffff {
        output[0] = 0xe0 | (rune >> 12) as u8;
        output[1] = 0x80 | ((rune >> 6) & 0x3f) as u8;
        output[2] = 0x80 | (rune & 0x3f) as u8;
        return (output, 3);
    }
    output[0] = 0xf0 | (rune >> 18) as u8;
    output[1] = 0x80 | ((rune >> 12) & 0x3f) as u8;
    output[2] = 0x80 | ((rune >> 6) & 0x3f) as u8;
    output[3] = 0x80 | (rune & 0x3f) as u8;
    (output, 4)
}
