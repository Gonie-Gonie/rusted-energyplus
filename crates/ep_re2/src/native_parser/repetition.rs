// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see ../RE2-LICENSE.
//! Pinned MaybeParseRepetition token recognition; the builder validates sizes.
use super::Parser;

impl Parser<'_> {
    pub(super) fn maybe_repetition(&self) -> Option<(u32, Option<u32>, usize)> {
        let bytes = self.pattern.as_bytes();
        let mut position = self.position + 1;
        let lower = integer(bytes, &mut position)?;
        let upper = if bytes.get(position) == Some(&b',') {
            position += 1;
            if bytes.get(position) == Some(&b'}') {
                None
            } else {
                Some(integer(bytes, &mut position)?)
            }
        } else {
            Some(lower)
        };
        if bytes.get(position) != Some(&b'}') {
            return None;
        }
        Some((lower, upper, position + 1))
    }
}

fn integer(bytes: &[u8], position: &mut usize) -> Option<u32> {
    let initial = *bytes.get(*position)?;
    if !initial.is_ascii_digit()
        || (initial == b'0' && bytes.get(*position + 1).is_some_and(u8::is_ascii_digit))
    {
        return None;
    }
    let mut number = 0_u32;
    while let Some(&digit) = bytes.get(*position).filter(|digit| digit.is_ascii_digit()) {
        if number >= 100_000_000 {
            return None;
        }
        number = number * 10 + u32::from(digit - b'0');
        *position += 1;
    }
    Some(number)
}
