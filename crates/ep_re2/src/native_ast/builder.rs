// Copyright 2006 The RE2 Authors. All Rights Reserved.
// BSD-style license: ../RE2-LICENSE.
//! Native parse-stack operations; lexical grammar remains the parser adapter.
use super::{Arena, Ast, Id, MAX_RUNE, Op, copy, flags, internal, push, ranges};
use crate::{ErrorCode, Status};
use std::sync::Arc;

#[derive(Clone, Debug)]
enum Entry {
    Node(Id),
    LeftParen {
        flags: u16,
        capture: Option<(u32, Option<Arc<str>>)>,
    },
    VerticalBar,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unary {
    Star,
    Plus,
    Quest,
}
#[derive(Clone, Debug)]
pub(crate) struct Builder<'a> {
    pub arena: Arena,
    pattern: &'a str,
    flags: u16,
    rune_max: u32,
    stack: Vec<Entry>,
    captures: u32,
}
impl<'a> Builder<'a> {
    pub(crate) fn new(pattern: &'a str, flags: u16) -> Self {
        let rune_max = if flags & flags::LATIN1 != 0 {
            255
        } else {
            MAX_RUNE
        };
        Self {
            arena: Arena::default(),
            pattern,
            flags,
            rune_max,
            stack: Vec::new(),
            captures: 0,
        }
    }
    pub(crate) fn flags(&self) -> u16 {
        self.flags
    }
    pub(crate) fn set_flags(&mut self, flags: u16) {
        self.flags = flags;
    }
    fn rune_max(&self) -> u32 {
        self.rune_max
    }
    fn top_node(&self) -> Option<Id> {
        match self.stack.last() {
            Some(Entry::Node(id)) => Some(*id),
            _ => None,
        }
    }
    fn set_top_node(&mut self, id: Id) -> Result<(), Status> {
        let Some(entry @ Entry::Node(_)) = self.stack.last_mut() else {
            return Err(internal());
        };
        *entry = Entry::Node(id);
        Ok(())
    }

    /// PushRegexp canonicalizes ONLY builder classes, then writes ComputeSimple.
    pub(crate) fn push_regexp(&mut self, mut id: Id) -> Result<(), Status> {
        self.maybe_concat_string(None, 0)?;
        if let Op::ClassBuilder(source) = &self.arena.node(id)?.op {
            let max = self.rune_max();
            let clipped = ranges::normalize(
                source
                    .iter()
                    .filter_map(|&(lo, hi)| (lo <= max).then_some((lo, hi.min(max))))
                    .collect(),
            );
            self.arena.node_mut(id)?.op = Op::ClassBuilder(clipped);
            let Op::ClassBuilder(ranges) = &self.arena.node(id)?.op else {
                return Err(internal());
            };
            let size: u64 = ranges
                .iter()
                .map(|&(lo, hi)| u64::from(hi) - u64::from(lo) + 1)
                .sum();
            if size == 1 {
                let rune = ranges.first().ok_or_else(internal)?.0;
                id = self.arena.new_node(Op::Literal(rune), self.flags)?;
            } else if size == 2 {
                let rune = ranges.first().ok_or_else(internal)?.0;
                if (u32::from(b'A')..=u32::from(b'Z')).contains(&rune)
                    && ranges::contains(ranges, rune + 32)
                {
                    id = self
                        .arena
                        .new_node(Op::Literal(rune + 32), self.flags | flags::FOLD_CASE)?;
                }
            }
        }
        self.arena.refresh_simple(id)?;
        push(&mut self.stack, Entry::Node(id))
    }
    pub(crate) fn push_class_builder(
        &mut self,
        ranges: Vec<(u32, u32)>,
        bits: u16,
    ) -> Result<(), Status> {
        let id = self
            .arena
            .new_node(Op::ClassBuilder(ranges::normalize(ranges)), bits)?;
        self.push_regexp(id)
    }
    pub(crate) fn push_simple(&mut self, op: Op) -> Result<(), Status> {
        if !matches!(
            op,
            Op::NoMatch
                | Op::Empty
                | Op::AnyChar
                | Op::AnyByte
                | Op::BeginLine
                | Op::EndLine
                | Op::WordBoundary
                | Op::NotWordBoundary
                | Op::BeginText
                | Op::EndText
                | Op::HaveMatch(_)
        ) {
            return Err(internal());
        }
        let id = self.arena.new_node(op, self.flags)?;
        self.push_regexp(id)
    }
    pub(crate) fn push_literal(&mut self, rune: u32) -> Result<(), Status> {
        if self.flags & flags::FOLD_CASE != 0 && crate::unicode::fold(rune) != rune {
            let mut ranges = Vec::new();
            let mut at = rune;
            loop {
                if self.flags & flags::NEVER_NL == 0 || at != 10 {
                    ranges::add(&mut ranges, at, at)?;
                }
                at = crate::unicode::fold(at);
                if at == rune {
                    break;
                }
            }
            return self.push_class_builder(ranges, self.flags & !flags::FOLD_CASE);
        }
        if self.flags & flags::NEVER_NL != 0 && rune == 10 {
            return self.push_simple(Op::NoMatch);
        }
        if self.maybe_concat_string(Some(rune), self.flags)? {
            return Ok(());
        }
        let id = self.arena.new_node(Op::Literal(rune), self.flags)?;
        self.push_regexp(id)
    }
    pub(crate) fn push_caret(&mut self) -> Result<(), Status> {
        self.push_simple(if self.flags & flags::ONE_LINE != 0 {
            Op::BeginText
        } else {
            Op::BeginLine
        })
    }
    pub(crate) fn push_dollar(&mut self) -> Result<(), Status> {
        if self.flags & flags::ONE_LINE == 0 {
            return self.push_simple(Op::EndLine);
        }
        let old = self.flags;
        self.flags |= flags::WAS_DOLLAR;
        let result = self.push_simple(Op::EndText);
        self.flags = old;
        result
    }
    pub(crate) fn push_dot(&mut self) -> Result<(), Status> {
        if self.flags & flags::DOT_NL != 0 && self.flags & flags::NEVER_NL == 0 {
            self.push_simple(Op::AnyChar)
        } else {
            self.push_class_builder(
                vec![(0, 9), (11, self.rune_max())],
                self.flags & !flags::FOLD_CASE,
            )
        }
    }

    /// PushRepeatOp, preserving distinct op kind and full-flag squash guards.
    pub(crate) fn push_unary(
        &mut self,
        kind: Unary,
        nongreedy: bool,
        argument: &str,
    ) -> Result<(), Status> {
        let child = self
            .top_node()
            .ok_or_else(|| Status::new(ErrorCode::RepeatArgument, argument))?;
        let bits = if nongreedy {
            self.flags ^ flags::NON_GREEDY
        } else {
            self.flags
        };
        let previous = self.arena.node(child)?;
        let same = matches!(
            (&previous.op, kind),
            (Op::Star(_), Unary::Star) | (Op::Plus(_), Unary::Plus) | (Op::Quest(_), Unary::Quest)
        );
        if previous.flags == bits {
            if same {
                return Ok(());
            }
            if let Op::Star(inner) | Op::Plus(inner) | Op::Quest(inner) = previous.op {
                self.arena.node_mut(child)?.op = Op::Star(inner);
                return Ok(());
            }
        }
        let child = self.arena.finish_regexp(child)?;
        let op = match kind {
            Unary::Star => Op::Star(child),
            Unary::Plus => Op::Plus(child),
            Unary::Quest => Op::Quest(child),
        };
        let id = self.arena.new_node(op, bits)?;
        self.arena.refresh_simple(id)?;
        self.set_top_node(id)
    }

    /// PushRepetition's size check precedes its missing-argument check.
    pub(crate) fn push_repetition(
        &mut self,
        min: u32,
        max: Option<u32>,
        nongreedy: bool,
        argument: &str,
    ) -> Result<(), Status> {
        if min > 1000 || max.is_some_and(|max| max > 1000 || max < min) {
            return Err(Status::new(ErrorCode::RepeatSize, argument));
        }
        let child = self
            .top_node()
            .ok_or_else(|| Status::new(ErrorCode::RepeatArgument, argument))?;
        let child = self.arena.finish_regexp(child)?;
        let bits = if nongreedy {
            self.flags ^ flags::NON_GREEDY
        } else {
            self.flags
        };
        let id = self.arena.new_node(Op::Repeat { child, min, max }, bits)?;
        self.arena.refresh_simple(id)?;
        self.set_top_node(id)?;
        if (min >= 2 || max.is_some_and(|max| max >= 2)) && !self.repeat_budget(id)? {
            return Err(Status::new(ErrorCode::RepeatSize, argument));
        }
        Ok(())
    }

    fn repeat_budget(&self, root: Id) -> Result<bool, Status> {
        let mut pending = vec![(root, 1000_u32)];
        while let Some((id, mut budget)) = pending.pop() {
            let op = &self.arena.node(id)?.op;
            if let Op::Repeat { min, max, .. } = op {
                let copies = max.unwrap_or(*min);
                budget = budget.checked_div(copies).unwrap_or(budget);
            }
            if budget == 0 {
                return Ok(false);
            }
            match op {
                Op::Concat(children) | Op::Alternate(children) => {
                    for &child in children {
                        push(&mut pending, (child, budget))?;
                    }
                }
                _ => {
                    if let Some(child) = op.unary_child() {
                        push(&mut pending, (child, budget))?;
                    }
                }
            }
        }
        Ok(true)
    }

    /// Exact pending pair; the top literal is left separate for a quantifier.
    fn maybe_concat_string(&mut self, rune: Option<u32>, bits: u16) -> Result<bool, Status> {
        let n = self.stack.len();
        if n < 2 {
            return Ok(false);
        }
        let (Entry::Node(top), Entry::Node(below)) = (&self.stack[n - 1], &self.stack[n - 2])
        else {
            return Ok(false);
        };
        let (top, below) = (*top, *below);
        let a = self.arena.node(top)?;
        let b = self.arena.node(below)?;
        if !a.op.is_literal() || !b.op.is_literal() || (a.flags ^ b.flags) & flags::FOLD_CASE != 0 {
            return Ok(false);
        }
        let mut merged = match &b.op {
            Op::Literal(r) => vec![*r],
            Op::LiteralString(r) => copy(r)?,
            _ => return Err(internal()),
        };
        match &a.op {
            Op::Literal(r) => push(&mut merged, *r)?,
            Op::LiteralString(runes) => {
                merged
                    .try_reserve(runes.len())
                    .map_err(|_| super::allocation())?;
                merged.extend_from_slice(runes);
            }
            _ => return Err(internal()),
        }
        self.arena.node_mut(below)?.op = Op::LiteralString(merged);
        if let Some(rune) = rune {
            let top = self.arena.node_mut(top)?;
            top.op = Op::Literal(rune);
            top.flags = bits;
            Ok(true)
        } else {
            self.stack.pop();
            Ok(false)
        }
    }

    /// Names are already lexically validated; retain Native name allocation identity.
    pub(crate) fn open_group(&mut self, capturing: bool, name: Option<&str>) -> Result<(), Status> {
        let capture = if capturing {
            self.captures = self.captures.checked_add(1).ok_or_else(super::allocation)?;
            Some((self.captures, name.map(Arc::from)))
        } else {
            None
        };
        self.maybe_concat_string(None, 0)?;
        push(
            &mut self.stack,
            Entry::LeftParen {
                flags: self.flags,
                capture,
            },
        )
    }
    pub(crate) fn close_group(&mut self) -> Result<(), Status> {
        self.do_alternation()?;
        let n = self.stack.len();
        if n < 2 {
            return Err(Status::new(ErrorCode::UnexpectedParen, self.pattern));
        }
        let child = match &self.stack[n - 1] {
            Entry::Node(id) => *id,
            _ => return Err(internal()),
        };
        let Entry::LeftParen {
            flags: saved,
            capture,
        } = self.stack[n - 2].clone()
        else {
            return Err(Status::new(ErrorCode::UnexpectedParen, self.pattern));
        };
        self.stack.truncate(n - 2);
        self.flags = saved;
        let id = if let Some((index, name)) = capture {
            let child = self.arena.finish_regexp(child)?;
            let id = self
                .arena
                .new_node(Op::Capture { child, index, name }, saved)?;
            self.arena.refresh_simple(id)?;
            id
        } else {
            child
        };
        self.push_regexp(id)
    }

    fn do_collapse(&mut self, alternate: bool) -> Result<(), Status> {
        let start = self
            .stack
            .iter()
            .rposition(|entry| !matches!(entry, Entry::Node(_)))
            .map_or(0, |at| at + 1);
        if self.stack.len() - start == 1 {
            return Ok(());
        }
        let mut children = Vec::new();
        // Native finalizes backward while filling the output array from its end.
        let ids: Vec<_> = self.stack[start..]
            .iter()
            .rev()
            .filter_map(|entry| {
                if let Entry::Node(id) = entry {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();
        for id in ids {
            let subs = match &self.arena.node(id)?.op {
                Op::Concat(subs) if !alternate => Some(copy(subs)?),
                Op::Alternate(subs) if alternate => Some(copy(subs)?),
                _ => None,
            };
            if let Some(subs) = subs {
                for child in subs.into_iter().rev() {
                    push(&mut children, child)?;
                }
            } else {
                self.arena.finish_regexp(id)?;
                push(&mut children, id)?;
            }
        }
        children.reverse();
        let id = if alternate {
            self.arena.alternate(children, self.flags)?
        } else {
            self.arena.concat(children, self.flags)?
        };
        self.arena.refresh_simple(id)?;
        self.stack.truncate(start);
        push(&mut self.stack, Entry::Node(id))
    }
    fn do_concatenation(&mut self) -> Result<(), Status> {
        if self.top_node().is_none() {
            self.push_simple(Op::Empty)?;
        }
        self.do_collapse(false)
    }
    /// Native AnyChar absorption and vertical-marker swap, in authored order.
    pub(crate) fn alternate(&mut self) -> Result<(), Status> {
        self.maybe_concat_string(None, 0)?;
        self.do_concatenation()?;
        let n = self.stack.len();
        if n >= 2 && matches!(self.stack[n - 2], Entry::VerticalBar) {
            let current = self.top_node().ok_or_else(internal)?;
            let previous = if n >= 3 {
                match &self.stack[n - 3] {
                    Entry::Node(id) => Some(*id),
                    _ => None,
                }
            } else {
                None
            };
            if let Some(previous) = previous {
                let a = &self.arena.node(current)?.op;
                let b = &self.arena.node(previous)?.op;
                let scalar = |op: &Op| matches!(op, Op::Literal(_) | Op::AnyChar) || op.is_class();
                if matches!(b, Op::AnyChar) && scalar(a) {
                    self.stack.pop();
                    return Ok(());
                }
                if matches!(a, Op::AnyChar) && scalar(b) {
                    self.stack.remove(n - 3);
                    let n = self.stack.len();
                    self.stack.swap(n - 2, n - 1);
                    return Ok(());
                }
            }
            self.stack.swap(n - 2, n - 1);
            Ok(())
        } else {
            self.maybe_concat_string(None, 0)?;
            push(&mut self.stack, Entry::VerticalBar)
        }
    }
    fn do_alternation(&mut self) -> Result<(), Status> {
        self.alternate()?;
        if !matches!(self.stack.pop(), Some(Entry::VerticalBar)) {
            return Err(internal());
        }
        self.do_collapse(true)
    }
    pub(crate) fn finish(mut self) -> Result<Ast, Status> {
        self.do_alternation()?;
        if self.stack.len() != 1 {
            return Err(Status::new(ErrorCode::MissingParen, self.pattern));
        }
        let root = self.top_node().ok_or_else(internal)?;
        self.arena.finish_regexp(root)?;
        Ok(Ast {
            arena: self.arena,
            root,
            captures: self.captures,
        })
    }
}
