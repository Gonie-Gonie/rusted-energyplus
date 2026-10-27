// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Rust port of the pinned RE2 AST operations. BSD license: RE2-LICENSE.
//! Lossless Native AST stage, before simplification and matcher-IR lowering.
//! Arena ownership replaces native refcounts; nodes never own child nodes.

use crate::{ErrorCode, Status};
use std::sync::Arc;

mod builder;
mod equality;
mod factor;
mod leading;
mod ranges;
pub(crate) use builder::{Builder, Unary};

pub(crate) type Id = usize;
pub(crate) const MAX_RUNE: u32 = 0x10ffff;
pub(crate) const MAX_NSUB: usize = 65535;

/// Pinned regexp.h ParseFlags bit positions, including internal WasDollar.
pub(crate) mod flags {
    pub const FOLD_CASE: u16 = 1 << 0;
    #[expect(
        dead_code,
        reason = "Pinned flag retained; reporting uses default LikePerl options"
    )]
    pub const LITERAL: u16 = 1 << 1;
    pub const CLASS_NL: u16 = 1 << 2;
    pub const DOT_NL: u16 = 1 << 3;
    pub const ONE_LINE: u16 = 1 << 4;
    pub const LATIN1: u16 = 1 << 5;
    pub const NON_GREEDY: u16 = 1 << 6;
    pub const PERL_CLASSES: u16 = 1 << 7;
    pub const PERL_B: u16 = 1 << 8;
    pub const PERL_X: u16 = 1 << 9;
    pub const UNICODE_GROUPS: u16 = 1 << 10;
    pub const NEVER_NL: u16 = 1 << 11;
    pub const NEVER_CAPTURE: u16 = 1 << 12;
    pub const WAS_DOLLAR: u16 = 1 << 13;
    pub const LIKE_PERL: u16 =
        CLASS_NL | ONE_LINE | PERL_CLASSES | PERL_B | PERL_X | UNICODE_GROUPS;
}

/// Native op identity is retained even where boolean languages are equal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Op {
    NoMatch,
    Empty,
    Literal(u32),
    LiteralString(Vec<u32>),
    Concat(Vec<Id>),
    Alternate(Vec<Id>),
    Star(Id),
    Plus(Id),
    Quest(Id),
    Repeat {
        child: Id,
        min: u32,
        max: Option<u32>,
    },
    Capture {
        child: Id,
        index: u32,
        name: Option<Arc<str>>,
    },
    AnyChar,
    AnyByte,
    BeginLine,
    EndLine,
    WordBoundary,
    NotWordBoundary,
    BeginText,
    EndText,
    ClassBuilder(Vec<(u32, u32)>),
    Class(Vec<(u32, u32)>),
    #[expect(
        dead_code,
        reason = "Pinned Set-only AST operation; reporting does not construct RE2::Set"
    )]
    HaveMatch(i32),
}

impl Op {
    /// Native numeric opcode, with ClassBuilder sharing CharClass's opcode.
    pub(crate) fn opcode(&self) -> u8 {
        match self {
            Self::NoMatch => 1,
            Self::Empty => 2,
            Self::Literal(_) => 3,
            Self::LiteralString(_) => 4,
            Self::Concat(_) => 5,
            Self::Alternate(_) => 6,
            Self::Star(_) => 7,
            Self::Plus(_) => 8,
            Self::Quest(_) => 9,
            Self::Repeat { .. } => 10,
            Self::Capture { .. } => 11,
            Self::AnyChar => 12,
            Self::AnyByte => 13,
            Self::BeginLine => 14,
            Self::EndLine => 15,
            Self::WordBoundary => 16,
            Self::NotWordBoundary => 17,
            Self::BeginText => 18,
            Self::EndText => 19,
            Self::ClassBuilder(_) | Self::Class(_) => 20,
            Self::HaveMatch(_) => 21,
        }
    }

    pub(super) fn is_class(&self) -> bool {
        matches!(self, Self::ClassBuilder(_) | Self::Class(_))
    }
    pub(super) fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_) | Self::LiteralString(_))
    }
    pub(super) fn unary_child(&self) -> Option<Id> {
        match self {
            Self::Star(child) | Self::Plus(child) | Self::Quest(child) => Some(*child),
            Self::Repeat { child, .. } | Self::Capture { child, .. } => Some(*child),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AstNode {
    pub op: Op,
    pub flags: u16,
    /// Native constructor initializes false; only explicit Native writes refresh it.
    pub simple: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Arena {
    pub nodes: Vec<AstNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Ast {
    pub arena: Arena,
    pub root: Id,
    pub captures: u32,
}

pub(super) fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}
pub(super) fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}
pub(super) fn copy<T: Clone>(values: &[T]) -> Result<Vec<T>, Status> {
    let mut out = Vec::new();
    out.try_reserve(values.len()).map_err(|_| allocation())?;
    out.extend_from_slice(values);
    Ok(out)
}
pub(super) fn push<T>(values: &mut Vec<T>, value: T) -> Result<(), Status> {
    values.try_reserve(1).map_err(|_| allocation())?;
    values.push(value);
    Ok(())
}

impl Arena {
    pub(crate) fn node(&self, id: Id) -> Result<&AstNode, Status> {
        self.nodes.get(id).ok_or_else(internal)
    }
    pub(super) fn node_mut(&mut self, id: Id) -> Result<&mut AstNode, Status> {
        self.nodes.get_mut(id).ok_or_else(internal)
    }
    /// Exact Regexp constructor simple=false policy; no eager normalization.
    pub(crate) fn new_node(&mut self, op: Op, flags: u16) -> Result<Id, Status> {
        let id = self.nodes.len();
        push(
            &mut self.nodes,
            AstNode {
                op,
                flags,
                simple: false,
            },
        )?;
        Ok(id)
    }
    pub(crate) fn finish_regexp(&mut self, id: Id) -> Result<Id, Status> {
        if let Op::ClassBuilder(ranges) = &self.node(id)?.op {
            let ranges = copy(ranges)?;
            self.node_mut(id)?.op = Op::Class(ranges);
        }
        Ok(id)
    }
    pub(crate) fn literal_string(&mut self, runes: &[u32], flags: u16) -> Result<Id, Status> {
        match runes {
            [] => self.new_node(Op::Empty, flags),
            [rune] => self.new_node(Op::Literal(*rune), flags),
            _ => self.new_node(Op::LiteralString(copy(runes)?), flags),
        }
    }
    /// Native NewCharClass: finished classes bypass builder PushRegexp rewriting.
    pub(crate) fn new_class(&mut self, ranges: Vec<(u32, u32)>, flags: u16) -> Result<Id, Status> {
        self.new_node(Op::Class(ranges::normalize(ranges)), flags)
    }
    /// ComputeSimple from simplify.cc44-99 using existing child cache values.
    pub(crate) fn refresh_simple(&mut self, id: Id) -> Result<(), Status> {
        let simple = match &self.node(id)?.op {
            Op::NoMatch
            | Op::Empty
            | Op::Literal(_)
            | Op::LiteralString(_)
            | Op::AnyChar
            | Op::AnyByte
            | Op::BeginLine
            | Op::EndLine
            | Op::WordBoundary
            | Op::NotWordBoundary
            | Op::BeginText
            | Op::EndText
            | Op::HaveMatch(_) => true,
            Op::Concat(children) | Op::Alternate(children) => {
                let mut simple = true;
                for &child in children {
                    simple &= self.node(child)?.simple;
                }
                simple
            }
            Op::ClassBuilder(ranges) | Op::Class(ranges) => {
                !ranges.is_empty() && ranges.as_slice() != [(0, MAX_RUNE)]
            }
            Op::Capture { child, .. } => self.node(*child)?.simple,
            Op::Star(child) | Op::Plus(child) | Op::Quest(child) => {
                let child = self.node(*child)?;
                child.simple
                    && !matches!(
                        child.op,
                        Op::Star(_) | Op::Plus(_) | Op::Quest(_) | Op::Empty | Op::NoMatch
                    )
            }
            Op::Repeat { .. } => false,
        };
        self.node_mut(id)?.simple = simple;
        Ok(())
    }
    pub(crate) fn concat(&mut self, children: Vec<Id>, flags: u16) -> Result<Id, Status> {
        self.nary(children, flags, false, false)
    }
    pub(crate) fn alternate(&mut self, children: Vec<Id>, flags: u16) -> Result<Id, Status> {
        self.nary(children, flags, true, true)
    }
    pub(crate) fn alternate_no_factor(
        &mut self,
        children: Vec<Id>,
        flags: u16,
    ) -> Result<Id, Status> {
        self.nary(children, flags, true, false)
    }
    fn nary(
        &mut self,
        mut children: Vec<Id>,
        flags: u16,
        alternate: bool,
        factor: bool,
    ) -> Result<Id, Status> {
        if children.len() == 1 {
            return Ok(children[0]);
        }
        if children.is_empty() {
            return self.new_node(if alternate { Op::NoMatch } else { Op::Empty }, flags);
        }
        if alternate && factor {
            children = self.factor_alternation(children, flags)?;
            if children.len() == 1 {
                return Ok(children[0]);
            }
        }
        if children.len() > MAX_NSUB {
            let mut big = Vec::new();
            big.try_reserve(children.len().div_ceil(MAX_NSUB))
                .map_err(|_| allocation())?;
            for chunk in children.chunks(MAX_NSUB) {
                let id = if chunk.len() == 1 {
                    chunk[0]
                } else {
                    self.new_node(
                        if alternate {
                            Op::Alternate(copy(chunk)?)
                        } else {
                            Op::Concat(copy(chunk)?)
                        },
                        flags,
                    )?
                };
                big.push(id);
            }
            children = big;
        }
        self.new_node(
            if alternate {
                Op::Alternate(children)
            } else {
                Op::Concat(children)
            },
            flags,
        )
    }
}
