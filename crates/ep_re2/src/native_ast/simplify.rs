// Copyright 2006 The RE2 Authors. All Rights Reserved.
// BSD-style license: ../RE2-LICENSE.
//! Regexp::Simplify: exact CoalesceWalker, then SimplifyWalker and cache phases.
use super::{Arena, Id, MAX_RUNE, Op, Unary, copy, flags, internal, push};
use crate::{ErrorCode, Status};

mod coalesce;
mod factories;

#[derive(Clone, Copy)]
enum Phase {
    Coalesce,
    Simplify,
}

struct Frame {
    input: Id,
    entered: bool,
    children: Vec<Id>,
    results: Vec<Id>,
}
impl Frame {
    fn new(input: Id) -> Self {
        Self {
            input,
            entered: false,
            children: Vec::new(),
            results: Vec::new(),
        }
    }
}

impl Arena {
    /// Full actual simplify.cc177-195 two-walker path, before program compilation.
    pub(crate) fn simplify(&mut self, root: Id) -> Result<Id, Status> {
        let (coalesced, stopped) = self.walk_simplify_phase(root, Phase::Coalesce)?;
        if stopped {
            return Err(Status::new(ErrorCode::PatternTooLarge, ""));
        }
        let (simple, stopped) = self.walk_simplify_phase(coalesced, Phase::Simplify)?;
        if stopped {
            return Err(Status::new(ErrorCode::PatternTooLarge, ""));
        }
        Ok(simple)
    }

    /// Same input-child order and ONLY the adjacent-identical-child Copy rule
    /// from walker-inl.h. There is deliberately no global DAG memo table.
    fn walk_simplify_phase(&mut self, root: Id, phase: Phase) -> Result<(Id, bool), Status> {
        self.node(root)?;
        let mut stack = vec![Frame::new(root)];
        let mut visits_left = 1_000_000_i64;
        let mut stopped = false;
        loop {
            let frame = stack.last_mut().ok_or_else(internal)?;
            let mut immediate = None;
            if !frame.entered {
                visits_left -= 1;
                if visits_left < 0 {
                    // Native ShortVisit returns the original reference; parent
                    // PostVisits still run. Error is reported AFTER the walk.
                    stopped = true;
                    immediate = Some(frame.input);
                } else if matches!(phase, Phase::Simplify) && self.node(frame.input)?.simple {
                    // Native PreVisit stop: no children and no PostVisit.
                    immediate = Some(frame.input);
                } else {
                    frame.children = self.simplify_children(frame.input)?;
                    frame
                        .results
                        .try_reserve(frame.children.len())
                        .map_err(|_| super::allocation())?;
                    frame.entered = true;
                }
            }
            let result = if let Some(id) = immediate {
                id
            } else {
                let next = frame.results.len();
                if next < frame.children.len() {
                    if next > 0 && frame.children[next - 1] == frame.children[next] {
                        let result = *frame.results.last().ok_or_else(internal)?;
                        frame.results.push(result);
                    } else {
                        let child = frame.children[next];
                        push(&mut stack, Frame::new(child))?;
                    }
                    continue;
                }
                let children = std::mem::take(&mut frame.results);
                match phase {
                    Phase::Coalesce => self.coalesce_post(frame.input, children)?,
                    Phase::Simplify => self.simplify_post(frame.input, children)?,
                }
            };
            stack.pop();
            let Some(parent) = stack.last_mut() else {
                return Ok((result, stopped));
            };
            // Capacity was reserved on the parent's PreVisit.
            parent.results.push(result);
        }
    }

    fn simplify_children(&self, id: Id) -> Result<Vec<Id>, Status> {
        Ok(match &self.node(id)?.op {
            Op::Concat(children) | Op::Alternate(children) => copy(children)?,
            op => op.unary_child().into_iter().collect(),
        })
    }

    fn simple_result(&mut self, id: Id) -> Result<Id, Status> {
        self.node_mut(id)?.simple = true;
        Ok(id)
    }

    /// SimplifyWalker::PostVisit. Pointer identity is the arena ID; do not use
    /// structural equality for ChildArgsChanged or the unary/capture shortcuts.
    fn simplify_post(&mut self, original: Id, children: Vec<Id>) -> Result<Id, Status> {
        let node = self.node(original)?.clone();
        let old_children = self.simplify_children(original)?;
        if old_children.len() != children.len() {
            return Err(internal());
        }
        match node.op {
            Op::NoMatch
            | Op::Empty
            | Op::Literal(_)
            | Op::LiteralString(_)
            | Op::BeginLine
            | Op::EndLine
            | Op::BeginText
            | Op::WordBoundary
            | Op::NotWordBoundary
            | Op::EndText
            | Op::AnyChar
            | Op::AnyByte
            | Op::HaveMatch(_) => self.simple_result(original),
            Op::Concat(_) | Op::Alternate(_) => {
                if old_children == children {
                    return self.simple_result(original);
                }
                let op = if matches!(node.op, Op::Concat(_)) {
                    Op::Concat(children)
                } else {
                    Op::Alternate(children)
                };
                let id = self.new_node(op, node.flags)?;
                self.simple_result(id)
            }
            Op::Capture { child, index, .. } => {
                let newsub = *children.first().ok_or_else(internal)?;
                if newsub == child {
                    return self.simple_result(original);
                }
                // Native copies cap_, but NOT name_, on this new allocation.
                let id = self.new_node(
                    Op::Capture {
                        child: newsub,
                        index,
                        name: None,
                    },
                    node.flags,
                )?;
                self.simple_result(id)
            }
            Op::Star(child) | Op::Plus(child) | Op::Quest(child) => {
                let newsub = *children.first().ok_or_else(internal)?;
                if matches!(self.node(newsub)?.op, Op::Empty) {
                    return Ok(newsub);
                }
                if newsub == child {
                    return self.simple_result(original);
                }
                if self.node(newsub)?.op.opcode() == node.op.opcode()
                    && self.node(newsub)?.flags == node.flags
                {
                    return Ok(newsub);
                }
                let op = match node.op {
                    Op::Star(_) => Op::Star(newsub),
                    Op::Plus(_) => Op::Plus(newsub),
                    Op::Quest(_) => Op::Quest(newsub),
                    _ => return Err(internal()),
                };
                let id = self.new_node(op, node.flags)?;
                self.simple_result(id)
            }
            Op::Repeat { min, max, .. } => {
                let newsub = *children.first().ok_or_else(internal)?;
                if matches!(self.node(newsub)?.op, Op::Empty) {
                    return Ok(newsub);
                }
                let id = self.simplify_repeat(newsub, min, max, node.flags)?;
                self.simple_result(id)
            }
            Op::Class(ranges) => {
                let result = if ranges.is_empty() {
                    self.new_node(Op::NoMatch, node.flags)?
                } else if ranges.as_slice() == [(0, MAX_RUNE)] {
                    self.new_node(Op::AnyChar, node.flags)?
                } else {
                    original
                };
                self.simple_result(result)
            }
            Op::ClassBuilder(_) => Err(internal()),
        }
    }

    fn simplify_repeat(
        &mut self,
        child: Id,
        min: u32,
        max: Option<u32>,
        bits: u16,
    ) -> Result<Id, Status> {
        // Fields model Native nonnegative signed counts. Signed overflow in the
        // source is not assigned a wrapped or clamped successful AST here.
        if min > i32::MAX as u32 || max.is_some_and(|max| max > i32::MAX as u32) {
            return Err(internal());
        }
        if max.is_none() {
            if min == 0 {
                return self.native_star(child, bits);
            }
            if min == 1 {
                return self.native_plus(child, bits);
            }
            let count = usize::try_from(min).map_err(|_| internal())?;
            let mut parts = Vec::new();
            parts.try_reserve(count).map_err(|_| super::allocation())?;
            parts.resize(count - 1, child);
            let last = self.native_plus(child, bits)?;
            parts.push(last);
            return self.concat(parts, bits);
        }
        let max = max.ok_or_else(internal)?;
        if min == 0 && max == 0 {
            return self.new_node(Op::Empty, bits);
        }
        if min == 1 && max == 1 {
            return Ok(child);
        }
        let mut prefix = None;
        if min > 0 {
            let count = usize::try_from(min).map_err(|_| internal())?;
            let mut parts = Vec::new();
            parts.try_reserve(count).map_err(|_| super::allocation())?;
            parts.resize(count, child);
            prefix = Some(self.concat(parts, bits)?);
        }
        if max > min {
            let mut suffix = self.native_quest(child, bits)?;
            // x{n,m}: the tail is nested x(x(x)?)? rather than a flat run.
            for _ in min + 1..max {
                let sequence = self.native_concat2(child, suffix, bits)?;
                suffix = self.native_quest(sequence, bits)?;
            }
            prefix = Some(match prefix {
                None => suffix,
                Some(prefix) => self.native_concat2(prefix, suffix, bits)?,
            });
        }
        match prefix {
            Some(id) => Ok(id),
            // Native's DFATAL-only malformed private-count fallback.
            None => self.new_node(Op::NoMatch, bits),
        }
    }
}
