// Copyright 2006 The RE2 Authors. All Rights Reserved.
// BSD-style license: ../RE2-LICENSE.
//! Pinned parse.cc729-878 leading-prefix inspection and removal.
use super::{Arena, Id, Op, copy, flags, internal};
use crate::Status;

impl Arena {
    /// Native LeadingRegexp checks only the first concatenation level.
    pub(super) fn leading_regexp(&self, id: Id) -> Result<Option<Id>, Status> {
        match &self.node(id)?.op {
            Op::Empty => Ok(None),
            Op::Concat(children) if children.len() >= 2 => {
                let first = children[0];
                if matches!(self.node(first)?.op, Op::Empty) {
                    Ok(None)
                } else {
                    Ok(Some(first))
                }
            }
            _ => Ok(Some(id)),
        }
    }

    /// Consume the logical leading-regexp reference, preserving Native cache
    /// values and returning the existing second child for a two-child concat.
    pub(super) fn remove_leading_regexp(&mut self, id: Id) -> Result<Id, Status> {
        if matches!(self.node(id)?.op, Op::Empty) {
            return Ok(id);
        }
        if let Op::Concat(children) = &self.node(id)?.op
            && children.len() >= 2
        {
            if matches!(self.node(children[0])?.op, Op::Empty) {
                return Ok(id);
            }
            if children.len() == 2 {
                return Ok(children[1]);
            }
            let remaining = copy(&children[1..])?;
            self.node_mut(id)?.op = Op::Concat(remaining);
            return Ok(id);
        }
        // Native constructs a new empty node, with constructor simple=false.
        let bits = self.node(id)?.flags;
        self.new_node(Op::Empty, bits)
    }

    /// Native LeadingString follows nested first concat children and returns
    /// only the FoldCase flag of the final atom, including a nonliteral atom.
    pub(super) fn leading_string(&self, mut id: Id) -> Result<(Vec<u32>, u16), Status> {
        loop {
            match &self.node(id)?.op {
                Op::Concat(children) if !children.is_empty() => id = children[0],
                _ => break,
            }
        }
        let node = self.node(id)?;
        let runes = match &node.op {
            Op::Literal(rune) => vec![*rune],
            Op::LiteralString(runes) => copy(runes)?,
            _ => Vec::new(),
        };
        Ok((runes, node.flags & flags::FOLD_CASE))
    }

    /// Native RemoveLeadingString mutates the existing node and its recorded
    /// concat ancestors without recomputing flags or the simple_ cache.
    pub(super) fn remove_leading_string(&mut self, id: Id, count: usize) -> Result<(), Status> {
        // Native stores only four ancestors, even while chasing farther. Its
        // parser-created concatenations have depth at most two (65535 child
        // grouping), but retain the exact fixed-four behavior rather than
        // silently repairing trees outside that invariant.
        let mut ancestors: [Option<Id>; 4] = [None; 4];
        let mut depth = 0;
        let mut current = id;
        while let Op::Concat(children) = &self.node(current)?.op {
            let child = children.first().copied().ok_or_else(internal)?;
            if depth < ancestors.len() {
                ancestors[depth] = Some(current);
                depth += 1;
            }
            current = child;
        }

        let replacement = match &self.node(current)?.op {
            Op::Literal(_) => Some(Op::Empty),
            Op::LiteralString(runes) => {
                if count >= runes.len() {
                    Some(Op::Empty)
                } else if count == runes.len() - 1 {
                    Some(Op::Literal(*runes.last().ok_or_else(internal)?))
                } else {
                    Some(Op::LiteralString(copy(&runes[count..])?))
                }
            }
            _ => None,
        };
        if let Some(op) = replacement {
            self.node_mut(current)?.op = op;
        }

        while depth > 0 {
            depth -= 1;
            let ancestor = ancestors[depth].ok_or_else(internal)?;
            let children = match &self.node(ancestor)?.op {
                Op::Concat(children) => copy(children)?,
                _ => return Err(internal()),
            };
            let first = children.first().copied().ok_or_else(internal)?;
            if !matches!(self.node(first)?.op, Op::Empty) {
                continue;
            }
            match children.len() {
                0 | 1 => {
                    // Native's DFATAL-only impossible concat branch changes
                    // the opcode without refreshing flags/simple_. The empty
                    // case cannot reach here because sub[0] is required.
                    self.node_mut(ancestor)?.op = Op::Empty;
                }
                2 => {
                    // Regexp::Swap copies the entire payload, flags and cache.
                    let second = self.node(children[1])?.clone();
                    *self.node_mut(ancestor)? = second;
                }
                _ => self.node_mut(ancestor)?.op = Op::Concat(copy(&children[1..])?),
            }
        }
        Ok(())
    }
}
