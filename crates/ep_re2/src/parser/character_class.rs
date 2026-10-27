// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see crate LICENSE.RE2.
//! Native ParseCharClass/ParseUnicodeGroup/AddUGroup, default ClassNL mode.
use super::{Parser, Status, ranges};
use crate::ErrorCode;

impl Parser<'_> {
    pub(super) fn apply_group(&self, source: &[(u32, u32)], positive: bool) -> Vec<(u32, u32)> {
        // Default LikePerl sets ClassNL. Inline i/m/s/U cannot clear it; explicit
        // ranges also force it in Native AddRangeFlags. Fold BEFORE negating.
        let group = if self.flags.fold_case {
            crate::unicode::fold_ranges(source)
        } else {
            source.to_vec()
        };
        if positive {
            ranges::normalize(group)
        } else {
            ranges::complement(group)
        }
    }

    pub(super) fn perl_class(&self) -> Option<(bool, &'static [(u32, u32)])> {
        let text = self.pattern.get(self.position..self.position + 2)?;
        crate::unicode::perl_group(text)
    }

    pub(super) fn unicode_group(&mut self) -> Result<Vec<(u32, u32)>, Status> {
        let start = self.position;
        let positive_escape = self.pattern.as_bytes().get(start + 1) == Some(&b'p');
        self.position += 2;
        let name_start = self.position;
        // Native1754 tests !StringPieceToRune rather than a negative result.
        // At EOF its initialized p/P rune survives the failed read; the empty
        // group lookup then overwrites BadUTF8 with BadCharRange and this span.
        if self.byte().is_none() {
            return Err(self.error(ErrorCode::BadCharRange, start, self.position));
        }
        let first = self.rune()?;
        let mut name = if first == u32::from(b'{') {
            let tail = &self.pattern[self.position..];
            let Some(end) = tail.find('}') else {
                return Err(Status::new(ErrorCode::BadCharRange, &self.pattern[start..]));
            };
            let name = &tail[..end];
            self.position += end + 1;
            name
        } else {
            &self.pattern[name_start..self.position]
        };
        let mut positive = positive_escape;
        if let Some(rest) = name.strip_prefix('^') {
            positive = !positive;
            name = rest;
        }
        let source = crate::unicode::group(name)
            .ok_or_else(|| self.error(ErrorCode::BadCharRange, start, self.position))?;
        Ok(self.apply_group(source, positive))
    }

    pub(super) fn character_class(&mut self) -> Result<Vec<(u32, u32)>, Status> {
        let start = self.position;
        self.position += 1; // [
        let negative = self.byte() == Some(b'^');
        if negative {
            self.position += 1;
        }
        let mut first = true;
        let mut class = Vec::new();
        while self.byte().is_some() && (first || self.byte() != Some(b']')) {
            first = false;
            let tail = &self.pattern[self.position..];
            // ParseCCName does nothing at an unterminated [: prefix; such text
            // is then ordinary characters, possibly ending in MissingBracket.
            if tail.len() > 2
                && tail.starts_with("[:")
                && let Some(end) = tail[2..].find(":]").map(|end| end + 4)
            {
                let spelling = &tail[..end];
                let (positive, source) = crate::unicode::posix_group(spelling)
                    .ok_or_else(|| Status::new(ErrorCode::BadCharRange, spelling))?;
                class.extend(self.apply_group(source, positive));
                self.position += end;
                continue;
            }
            if tail.len() > 2 && (tail.starts_with("\\p") || tail.starts_with("\\P")) {
                class.extend(self.unicode_group()?);
                continue;
            }
            if let Some((positive, source)) = self.perl_class() {
                class.extend(self.apply_group(source, positive));
                self.position += 2;
                continue;
            }
            let range_start = self.position;
            let lo = self.class_character(start)?;
            let hi = if self.byte() == Some(b'-')
                && self
                    .pattern
                    .as_bytes()
                    .get(self.position + 1)
                    .is_some_and(|&next| next != b']')
            {
                self.position += 1;
                self.class_character(start)?
            } else {
                lo
            };
            if hi < lo {
                return Err(self.error(ErrorCode::BadCharRange, range_start, self.position));
            }
            class.extend(self.apply_group(&[(lo, hi)], true));
        }
        if self.byte().is_none() {
            return Err(Status::new(
                ErrorCode::MissingBracket,
                &self.pattern[start..],
            ));
        }
        self.position += 1; // ]
        if negative {
            Ok(ranges::complement(class))
        } else {
            Ok(ranges::normalize(class))
        }
    }

    fn class_character(&mut self, class_start: usize) -> Result<u32, Status> {
        if self.byte().is_none() {
            return Err(Status::new(
                ErrorCode::MissingBracket,
                &self.pattern[class_start..],
            ));
        }
        if self.byte() == Some(b'\\') {
            self.escape()
        } else {
            self.rune()
        }
    }
}
