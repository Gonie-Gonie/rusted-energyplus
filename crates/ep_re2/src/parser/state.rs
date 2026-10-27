// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Derived from RE2 under its BSD-style license; see crate LICENSE.RE2.
//! Explicit Native parse-stack equivalent; no recursive nesting traversal.
use super::{Frame, MAX_REPEAT, Node, NodeId, Parser, Status, allocation, internal};
use crate::ErrorCode;

impl Parser<'_> {
    pub(super) fn add_node(&mut self, node: Node) -> Result<NodeId, Status> {
        let factor = match &node {
            Node::Concat(children) | Node::Alternate(children) => children
                .iter()
                .map(|&child| self.repeat_factors[child])
                .max()
                .unwrap_or(1),
            Node::Capture { child, .. } => self.repeat_factors[*child],
            Node::Repeat { child, min, max } => self.repeat_factors[*child]
                .saturating_mul(max.unwrap_or(*min).max(1))
                .min(MAX_REPEAT + 1),
            _ => 1,
        };
        self.nodes.try_reserve(1).map_err(|_| allocation())?;
        self.repeat_factors
            .try_reserve(1)
            .map_err(|_| allocation())?;
        let id = self.nodes.len();
        self.nodes.push(node);
        self.repeat_factors.push(factor);
        Ok(id)
    }

    pub(super) fn append_node(&mut self, node: Node) -> Result<(), Status> {
        // Pinned simplify.cc653-660 turns an empty character class into
        // kRegexpNoMatch. Performing this representation change here preserves
        // its empty language and the repetition walker's factor-one leaf.
        let node = match node {
            Node::Class(ranges) if ranges.is_empty() => Node::NoMatch,
            other => other,
        };
        let id = self.add_node(node)?;
        self.append(id)
    }

    fn append(&mut self, id: NodeId) -> Result<(), Status> {
        let frame = self.frames.last_mut().ok_or_else(internal)?;
        frame.sequence.try_reserve(1).map_err(|_| allocation())?;
        frame.sequence.push(id);
        Ok(())
    }

    fn concatenate(&mut self, sequence: Vec<NodeId>) -> Result<NodeId, Status> {
        match sequence.as_slice() {
            [] => self.add_node(Node::Empty),
            [single] => Ok(*single),
            _ => self.add_node(Node::Concat(sequence)),
        }
    }

    pub(super) fn collapse_frame(&mut self, mut frame: Frame) -> Result<NodeId, Status> {
        let last = self.concatenate(frame.sequence)?;
        let mut result = if frame.alternatives.is_empty() {
            last
        } else {
            frame
                .alternatives
                .try_reserve(1)
                .map_err(|_| allocation())?;
            frame.alternatives.push(last);
            self.add_node(Node::Alternate(frame.alternatives))?
        };
        if let Some(index) = frame.capture {
            result = self.add_node(Node::Capture {
                child: result,
                index,
            })?;
        }
        Ok(result)
    }

    pub(super) fn open_group(&mut self, capturing: bool) -> Result<(), Status> {
        let capture = if capturing {
            self.captures = self.captures.checked_add(1).ok_or_else(allocation)?;
            Some(self.captures)
        } else {
            None
        };
        self.frames.try_reserve(1).map_err(|_| allocation())?;
        self.frames.push(Frame {
            saved_flags: self.flags,
            capture,
            sequence: Vec::new(),
            alternatives: Vec::new(),
        });
        Ok(())
    }

    pub(super) fn close_group(&mut self) -> Result<(), Status> {
        if self.frames.len() == 1 {
            return Err(Status::new(ErrorCode::UnexpectedParen, self.pattern));
        }
        let frame = self.frames.pop().ok_or_else(internal)?;
        self.flags = frame.saved_flags;
        let id = self.collapse_frame(frame)?;
        self.append(id)
    }

    pub(super) fn alternate(&mut self) -> Result<(), Status> {
        let sequence = {
            let frame = self.frames.last_mut().ok_or_else(internal)?;
            std::mem::take(&mut frame.sequence)
        };
        let id = self.concatenate(sequence)?;
        let frame = self.frames.last_mut().ok_or_else(internal)?;
        frame
            .alternatives
            .try_reserve(1)
            .map_err(|_| allocation())?;
        frame.alternatives.push(id);
        Ok(())
    }

    pub(super) fn repeat(
        &mut self,
        min: u32,
        max: Option<u32>,
        start: usize,
        counted: bool,
    ) -> Result<(), Status> {
        // Native checks size before checking whether there is an argument.
        if counted
            && (min > MAX_REPEAT || max.is_some_and(|upper| upper > MAX_REPEAT || upper < min))
        {
            return Err(self.error(ErrorCode::RepeatSize, start, self.position));
        }
        let child = self
            .frames
            .last_mut()
            .ok_or_else(internal)?
            .sequence
            .pop()
            .ok_or_else(|| self.error(ErrorCode::RepeatArgument, start, self.position))?;
        // Equivalent to Native RepetitionWalker dividing a budget by every
        // counted repeat on each AST path. Star/Plus/Quest have factor one.
        let factor = self.repeat_factors[child].saturating_mul(max.unwrap_or(min).max(1));
        if counted && (min >= 2 || max.is_some_and(|upper| upper >= 2)) && factor > MAX_REPEAT {
            return Err(self.error(ErrorCode::RepeatSize, start, self.position));
        }
        let id = self.add_node(Node::Repeat { child, min, max })?;
        self.append(id)
    }

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
