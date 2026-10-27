// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see ../RE2-LICENSE.
//! Pinned ParsePerlFlags and Unicode-category capture-name validation.
use super::{Parser, Status, flags};
use crate::ErrorCode;

impl Parser<'_> {
    pub(super) fn perl_group(&mut self) -> Result<(), Status> {
        let start = self.position;
        self.position += 2; // (?; callers have checked both bytes.
        let tail = &self.pattern[self.position..];
        // This pinned version implements Python's form only. Newer spelling
        // (?<name>...) must not be silently accepted by a newer regex grammar.
        if tail.len() > 2 && tail.starts_with("P<") {
            let Some(end) = tail[2..].find('>').map(|end| end + 2) else {
                return Err(Status::new(
                    ErrorCode::BadNamedCapture,
                    &self.pattern[start..],
                ));
            };
            let name = &tail[2..end];
            let after = self.position + end + 1;
            if !valid_capture_name(name) {
                return Err(self.error(ErrorCode::BadNamedCapture, start, after));
            }
            self.builder.open_group(true, Some(name))?;
            self.position = after;
            return Ok(());
        }

        let mut bits = self.builder.flags();
        let mut negative = false;
        let mut saw_flags = false;
        loop {
            if self.byte().is_none() {
                return Err(self.error(ErrorCode::BadPerlOp, start, self.position));
            }
            let rune = self.rune()?;
            match rune {
                105 => {
                    // i
                    saw_flags = true;
                    if negative {
                        bits &= !flags::FOLD_CASE;
                    } else {
                        bits |= flags::FOLD_CASE;
                    }
                }
                109 => {
                    // m is the opposite of Native OneLine.
                    saw_flags = true;
                    if negative {
                        bits |= flags::ONE_LINE;
                    } else {
                        bits &= !flags::ONE_LINE;
                    }
                }
                115 => {
                    // s
                    saw_flags = true;
                    if negative {
                        bits &= !flags::DOT_NL;
                    } else {
                        bits |= flags::DOT_NL;
                    }
                }
                85 => {
                    // U
                    saw_flags = true;
                    if negative {
                        bits &= !flags::NON_GREEDY;
                    } else {
                        bits |= flags::NON_GREEDY;
                    }
                }
                45 => {
                    // -
                    if negative {
                        return Err(self.error(ErrorCode::BadPerlOp, start, self.position));
                    }
                    negative = true;
                    saw_flags = false;
                }
                58 | 41 => {
                    // : or )
                    if rune == 58 {
                        // Native saves OLD bits before installing scoped edits,
                        // including before the negative-without-flags error.
                        self.builder.open_group(false, None)?;
                    }
                    if negative && !saw_flags {
                        return Err(self.error(ErrorCode::BadPerlOp, start, self.position));
                    }
                    self.builder.set_flags(bits);
                    return Ok(());
                }
                _ => return Err(self.error(ErrorCode::BadPerlOp, start, self.position)),
            }
        }
    }
}

fn valid_capture_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|rune| {
            let rune = u32::from(rune);
            ["Lu", "Ll", "Lt", "Lm", "Lo", "Nl", "Mn", "Mc", "Nd", "Pc"]
                .into_iter()
                .any(|group| {
                    crate::unicode::group(group).is_some_and(|ranges| {
                        ranges.iter().any(|&(lo, hi)| lo <= rune && rune <= hi)
                    })
                })
        })
}
