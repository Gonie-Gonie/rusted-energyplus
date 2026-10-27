// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see ../RE2-LICENSE.
//! Native ParseEscape, preserving consumed byte-span error arguments.
use super::{Parser, RUNE_MAX, Status};
use crate::ErrorCode;

impl Parser<'_> {
    pub(super) fn escape(&mut self) -> Result<u32, Status> {
        let start = self.position;
        self.position += 1; // backslash
        if self.byte().is_none() {
            return Err(Status::new(ErrorCode::TrailingBackslash, ""));
        }
        let rune = self.rune()?;
        match rune {
            48..=55 => {
                if rune != 48
                    && !self
                        .byte()
                        .is_some_and(|next| (b'0'..=b'7').contains(&next))
                {
                    return Err(self.error(ErrorCode::BadEscape, start, self.position));
                }
                let mut value = rune - 48;
                for _ in 0..2 {
                    if let Some(next) = self.byte().filter(|next| (b'0'..=b'7').contains(next)) {
                        value = value * 8 + u32::from(next - b'0');
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                Ok(value)
            }
            120 => self.hex_escape(start), // x
            110 => Ok(10),                 // n
            114 => Ok(13),                 // r
            116 => Ok(9),                  // t
            97 => Ok(7),                   // a
            102 => Ok(12),                 // f
            118 => Ok(11),                 // v
            _ if rune < 128 && !(rune as u8).is_ascii_alphanumeric() => Ok(rune),
            _ => Err(self.error(ErrorCode::BadEscape, start, self.position)),
        }
    }

    fn hex_escape(&mut self, start: usize) -> Result<u32, Status> {
        if self.byte().is_none() {
            return Err(self.error(ErrorCode::BadEscape, start, self.position));
        }
        let first = self.rune()?;
        if first == u32::from(b'{') {
            // Native calls StringPieceToRune here even at end-of-input: its
            // invalid-UTF8 status takes precedence over BadEscape at this point.
            let mut current = self.rune()?;
            let mut digits = 0;
            let mut value = 0_u32;
            while let Some(digit) = hexadecimal(current) {
                digits += 1;
                value = value * 16 + digit;
                if value > RUNE_MAX || self.byte().is_none() {
                    return Err(self.error(ErrorCode::BadEscape, start, self.position));
                }
                current = self.rune()?;
            }
            if current != u32::from(b'}') || digits == 0 {
                return Err(self.error(ErrorCode::BadEscape, start, self.position));
            }
            Ok(value)
        } else {
            if self.byte().is_none() {
                return Err(self.error(ErrorCode::BadEscape, start, self.position));
            }
            let second = self.rune()?;
            match (hexadecimal(first), hexadecimal(second)) {
                (Some(first), Some(second)) => Ok(first * 16 + second),
                _ => Err(self.error(ErrorCode::BadEscape, start, self.position)),
            }
        }
    }
}

fn hexadecimal(rune: u32) -> Option<u32> {
    match rune {
        48..=57 => Some(rune - 48),
        65..=70 => Some(rune - 65 + 10),
        97..=102 => Some(rune - 97 + 10),
        _ => None,
    }
}
