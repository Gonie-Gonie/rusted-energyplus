// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see RE2-LICENSE.
//! Default UTF8/LikePerl lexical grammar feeding the lossless Native AST builder.
//! The builder alone owns the flags, capture count, parse stack and node arena.
use crate::native_ast::{Ast, Builder, Op, Unary, flags};
use crate::{ErrorCode, Status};

mod character_class;
mod escapes;
mod groups;
mod ranges;
mod repetition;

const RUNE_MAX: u32 = crate::native_ast::MAX_RUNE;

struct Parser<'a> {
    pattern: &'a str,
    position: usize,
    builder: Builder<'a>,
}

/// Parse the pinned default constructor grammar without later constructor stages.
pub(crate) fn parse(pattern: &str) -> Result<Ast, Status> {
    let mut parser = Parser {
        pattern,
        position: 0,
        builder: Builder::new(pattern, flags::LIKE_PERL),
    };
    parser.read()?;
    parser.builder.finish()
}

impl Parser<'_> {
    fn read(&mut self) -> Result<(), Status> {
        // Native resets lastunary after every non-repetition lexer iteration,
        // including an otherwise empty inline flag edit or quoted literal run.
        let mut last_unary: Option<usize> = None;
        while let Some(byte) = self.byte() {
            let start = self.position;
            let mut this_unary = None;
            match byte {
                b'(' => {
                    if self.builder.flags() & flags::PERL_X != 0
                        && self.pattern.as_bytes().get(start + 1) == Some(&b'?')
                    {
                        self.perl_group()?;
                    } else {
                        self.builder
                            .open_group(self.builder.flags() & flags::NEVER_CAPTURE == 0, None)?;
                        self.position += 1;
                    }
                }
                b')' => {
                    self.builder.close_group()?;
                    self.position += 1;
                }
                b'|' => {
                    self.builder.alternate()?;
                    self.position += 1;
                }
                b'^' => {
                    self.builder.push_caret()?;
                    self.position += 1;
                }
                b'$' => {
                    self.builder.push_dollar()?;
                    self.position += 1;
                }
                b'.' => {
                    self.builder.push_dot()?;
                    self.position += 1;
                }
                b'[' => {
                    let class = self.character_class()?;
                    self.push_class(class)?;
                }
                b'*' | b'+' | b'?' => {
                    self.position += 1;
                    let nongreedy = self.consume_question();
                    if let Some(previous) = last_unary {
                        return Err(self.error(ErrorCode::RepeatOp, previous, self.position));
                    }
                    let kind = match byte {
                        b'*' => Unary::Star,
                        b'+' => Unary::Plus,
                        _ => Unary::Quest,
                    };
                    self.builder.push_unary(
                        kind,
                        nongreedy,
                        &self.pattern[start..self.position],
                    )?;
                    this_unary = Some(start);
                }
                b'{' => {
                    if let Some((min, max, end)) = self.maybe_repetition() {
                        self.position = end;
                        let nongreedy = self.consume_question();
                        if let Some(previous) = last_unary {
                            return Err(self.error(ErrorCode::RepeatOp, previous, self.position));
                        }
                        self.builder.push_repetition(
                            min,
                            max,
                            nongreedy,
                            &self.pattern[start..self.position],
                        )?;
                        this_unary = Some(start);
                    } else {
                        self.builder.push_literal(u32::from(b'{'))?;
                        self.position += 1;
                    }
                }
                b'\\' => self.backslash()?,
                _ => {
                    let rune = self.rune()?;
                    self.builder.push_literal(rune)?;
                }
            }
            last_unary = this_unary;
        }
        Ok(())
    }

    fn byte(&self) -> Option<u8> {
        self.pattern.as_bytes().get(self.position).copied()
    }

    fn rune(&mut self) -> Result<u32, Status> {
        let rune = self
            .pattern
            .get(self.position..)
            .and_then(|text| text.chars().next())
            .ok_or_else(|| Status::new(ErrorCode::BadUTF8, ""))?;
        self.position += rune.len_utf8();
        Ok(u32::from(rune))
    }

    fn error(&self, code: ErrorCode, first: usize, end: usize) -> Status {
        Status::new(code, &self.pattern[first..end])
    }

    fn consume_question(&mut self) -> bool {
        if self.builder.flags() & flags::PERL_X != 0 && self.byte() == Some(b'?') {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn push_class(&mut self, ranges: Vec<(u32, u32)>) -> Result<(), Status> {
        // ParseCharClass, Unicode and Perl group paths all clear FoldCase on
        // the already-expanded builder class. Empty classes stay builder nodes.
        let bits = self.builder.flags() & !flags::FOLD_CASE;
        self.builder.push_class_builder(ranges, bits)
    }

    fn backslash(&mut self) -> Result<(), Status> {
        let next = self.pattern.as_bytes().get(self.position + 1).copied();
        match next {
            Some(b'b' | b'B' | b'A' | b'z') => {
                let op = match next {
                    Some(b'b') => Op::WordBoundary,
                    Some(b'B') => Op::NotWordBoundary,
                    Some(b'A') => Op::BeginText,
                    _ => Op::EndText,
                };
                self.builder.push_simple(op)?;
                self.position += 2;
                Ok(())
            }
            Some(b'C') => {
                self.builder.push_simple(Op::AnyByte)?;
                self.position += 2;
                Ok(())
            }
            Some(b'Q') => {
                self.position += 2;
                while self.byte().is_some() {
                    if self
                        .pattern
                        .as_bytes()
                        .get(self.position..self.position + 2)
                        == Some(b"\\E".as_slice())
                    {
                        self.position += 2;
                        break;
                    }
                    let rune = self.rune()?;
                    self.builder.push_literal(rune)?;
                }
                Ok(())
            }
            Some(b'p' | b'P') => {
                let class = self.unicode_group()?;
                self.push_class(class)
            }
            _ => {
                if let Some((positive, source)) = self.perl_class() {
                    let class = self.apply_group(source, positive);
                    self.position += 2;
                    self.push_class(class)
                } else {
                    let rune = self.escape()?;
                    self.builder.push_literal(rune)
                }
            }
        }
    }
}
