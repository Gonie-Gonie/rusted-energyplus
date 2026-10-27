// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see crate LICENSE.RE2.
//! Default UTF8/LikePerl grammar from pinned RE2 parse.cc.
//!
//! This module ports syntax and boolean-language semantics, using explicit group
//! frames and an arena instead of C++ pointers. It does not implement RE2's AST
//! factoring, required-prefix optimization, or compiled-program memory budget.
//! Capture priorities/nongreediness do not change boolean full-match membership.
use crate::{Assertion, ErrorCode, Node, NodeId, Parsed, Status};

mod character_class;
mod escapes;
mod groups;
mod ranges;
mod state;

const RUNE_MAX: u32 = 0x10ffff;
const MAX_REPEAT: u32 = 1000;

#[derive(Clone, Copy, Default)]
struct Flags {
    fold_case: bool,
    multiline: bool,
    dot_newline: bool,
    non_greedy: bool,
}

struct Frame {
    saved_flags: Flags,
    capture: Option<u32>,
    sequence: Vec<NodeId>,
    alternatives: Vec<NodeId>,
}

struct Parser<'a> {
    pattern: &'a str,
    position: usize,
    flags: Flags,
    frames: Vec<Frame>,
    nodes: Vec<Node>,
    repeat_factors: Vec<u32>,
    captures: u32,
}

fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}

fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}

/// Parse the exact default constructor grammar used by EnergyPlus's RE2 calls.
pub(crate) fn parse(pattern: &str) -> Result<Parsed, Status> {
    let mut parser = Parser {
        pattern,
        position: 0,
        flags: Flags::default(),
        frames: vec![Frame {
            saved_flags: Flags::default(),
            capture: None,
            sequence: Vec::new(),
            alternatives: Vec::new(),
        }],
        nodes: Vec::new(),
        repeat_factors: Vec::new(),
        captures: 0,
    };
    parser.read()?;
    if parser.frames.len() != 1 {
        return Err(Status::new(ErrorCode::MissingParen, pattern));
    }
    let frame = parser.frames.pop().ok_or_else(internal)?;
    let root = parser.collapse_frame(frame)?;
    Ok(Parsed {
        nodes: parser.nodes,
        root,
        captures: parser.captures,
    })
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
                    if self.pattern.as_bytes().get(start + 1) == Some(&b'?') {
                        self.perl_group()?;
                    } else {
                        self.position += 1;
                        self.open_group(true)?;
                    }
                }
                b')' => {
                    self.close_group()?;
                    self.position += 1;
                }
                b'|' => {
                    self.alternate()?;
                    self.position += 1;
                }
                b'^' | b'$' => {
                    self.position += 1;
                    let assertion = match (byte, self.flags.multiline) {
                        (b'^', false) => Assertion::BeginText,
                        (b'^', true) => Assertion::BeginLine,
                        (_, false) => Assertion::EndText,
                        (_, true) => Assertion::EndLine,
                    };
                    self.append_node(Node::Assert(assertion))?;
                }
                b'.' => {
                    self.position += 1;
                    let ranges = if self.flags.dot_newline {
                        vec![(0, RUNE_MAX)]
                    } else {
                        vec![(0, 9), (11, RUNE_MAX)]
                    };
                    self.append_node(Node::Class(ranges))?;
                }
                b'[' => {
                    let class = self.character_class()?;
                    self.append_node(Node::Class(class))?;
                }
                b'*' | b'+' | b'?' => {
                    self.position += 1;
                    let nongreedy = self.consume_question();
                    if let Some(previous) = last_unary {
                        return Err(self.error(ErrorCode::RepeatOp, previous, self.position));
                    }
                    let (min, max) = match byte {
                        b'*' => (0, None),
                        b'+' => (1, None),
                        _ => (0, Some(1)),
                    };
                    let _priority_only = self.flags.non_greedy ^ nongreedy;
                    self.repeat(min, max, start, false)?;
                    this_unary = Some(start);
                }
                b'{' => {
                    if let Some((min, max, end)) = self.maybe_repetition() {
                        self.position = end;
                        let nongreedy = self.consume_question();
                        if let Some(previous) = last_unary {
                            return Err(self.error(ErrorCode::RepeatOp, previous, self.position));
                        }
                        let _priority_only = self.flags.non_greedy ^ nongreedy;
                        self.repeat(min, max, start, true)?;
                        this_unary = Some(start);
                    } else {
                        self.position += 1;
                        self.literal(u32::from(b'{'))?;
                    }
                }
                b'\\' => self.backslash()?,
                _ => {
                    let rune = self.rune()?;
                    self.literal(rune)?;
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
        if self.byte() == Some(b'?') {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn literal(&mut self, rune: u32) -> Result<(), Status> {
        if self.flags.fold_case && crate::unicode::fold(rune) != rune {
            let class = crate::unicode::fold_ranges(&[(rune, rune)]);
            self.append_node(Node::Class(class))
        } else {
            self.append_node(Node::Literal(rune))
        }
    }

    fn backslash(&mut self) -> Result<(), Status> {
        let next = self.pattern.as_bytes().get(self.position + 1).copied();
        match next {
            Some(b'b' | b'B' | b'A' | b'z') => {
                let assertion = match next {
                    Some(b'b') => Assertion::WordBoundary,
                    Some(b'B') => Assertion::NotWordBoundary,
                    Some(b'A') => Assertion::BeginText,
                    _ => Assertion::EndText,
                };
                self.position += 2;
                self.append_node(Node::Assert(assertion))
            }
            Some(b'C') => {
                self.position += 2;
                self.append_node(Node::AnyByte)
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
                    self.literal(rune)?;
                }
                Ok(())
            }
            Some(b'p' | b'P') => {
                let class = self.unicode_group()?;
                self.append_node(Node::Class(class))
            }
            _ => {
                if let Some(group) = self.perl_class() {
                    let class = self.apply_group(group.1, group.0);
                    self.position += 2;
                    self.append_node(Node::Class(class))
                } else {
                    let rune = self.escape()?;
                    self.literal(rune)
                }
            }
        }
    }
}
