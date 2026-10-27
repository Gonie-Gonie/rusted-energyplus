// Copyright 2006 The RE2 Authors. All Rights Reserved.
// BSD-style license: ../RE2-LICENSE.
//! FactorAlternation parse.cc935-1192: exact three rounds and logical recursion.
use super::{Arena, Id, Op, copy, internal, push, ranges};
use crate::Status;

struct Splice {
    prefix: Id,
    first: usize,
    count: usize,
    suffix: Option<Vec<Id>>,
}
struct Frame {
    sub: Vec<Id>,
    round: u8,
    splices: Vec<Splice>,
    splice_index: usize,
}
impl Frame {
    fn new(sub: Vec<Id>) -> Self {
        Self {
            sub,
            round: 0,
            splices: Vec::new(),
            splice_index: 0,
        }
    }
}

impl Arena {
    pub(super) fn factor_alternation(
        &mut self,
        children: Vec<Id>,
        flags: u16,
    ) -> Result<Vec<Id>, Status> {
        let mut stack = vec![Frame::new(children)];
        loop {
            let frame = stack.last_mut().ok_or_else(internal)?;
            if frame.splices.is_empty() {
                frame.round += 1;
            } else if frame.splice_index < frame.splices.len() {
                let splice = &frame.splices[frame.splice_index];
                let end = splice
                    .first
                    .checked_add(splice.count)
                    .ok_or_else(internal)?;
                let suffixes = copy(frame.sub.get(splice.first..end).ok_or_else(internal)?)?;
                push(&mut stack, Frame::new(suffixes))?;
                continue;
            } else {
                // Apply splices only AFTER each logical suffix recursion returns.
                let splices = std::mem::take(&mut frame.splices);
                let mut out = Vec::new();
                out.try_reserve(frame.sub.len())
                    .map_err(|_| super::allocation())?;
                let mut index = 0;
                for splice in splices {
                    for &id in frame.sub.get(index..splice.first).ok_or_else(internal)? {
                        push(&mut out, id)?;
                    }
                    let id = match frame.round {
                        1 | 2 => {
                            let suffix = self
                                .alternate_no_factor(splice.suffix.ok_or_else(internal)?, flags)?;
                            self.concat(vec![splice.prefix, suffix], flags)?
                        }
                        3 => splice.prefix,
                        _ => return Err(internal()),
                    };
                    push(&mut out, id)?;
                    index = splice
                        .first
                        .checked_add(splice.count)
                        .ok_or_else(internal)?;
                }
                for &id in frame.sub.get(index..).ok_or_else(internal)? {
                    push(&mut out, id)?;
                }
                frame.sub = out;
                frame.round += 1;
            }

            match frame.round {
                1 => frame.splices = self.factor_round1(&mut frame.sub)?,
                2 => frame.splices = self.factor_round2(&mut frame.sub)?,
                3 => frame.splices = self.factor_round3(&frame.sub, flags)?,
                4 => {
                    let finished = stack.pop().ok_or_else(internal)?.sub;
                    let Some(parent) = stack.last_mut() else {
                        return Ok(finished);
                    };
                    let splice = parent
                        .splices
                        .get_mut(parent.splice_index)
                        .ok_or_else(internal)?;
                    splice.suffix = Some(finished);
                    parent.splice_index += 1;
                    continue;
                }
                _ => return Err(internal()),
            }
            frame.splice_index = if frame.splices.is_empty() || frame.round == 3 {
                frame.splices.len()
            } else {
                0
            };
        }
    }

    fn factor_round1(&mut self, sub: &mut [Id]) -> Result<Vec<Splice>, Status> {
        let mut splices = Vec::new();
        let mut start = 0;
        let mut runes: Vec<u32> = Vec::new();
        let mut rune_flags = 0;
        for index in 0..=sub.len() {
            let candidate = if index < sub.len() {
                Some(self.leading_string(sub[index])?)
            } else {
                None
            };
            if let Some((current, current_flags)) = &candidate
                && *current_flags == rune_flags
            {
                let same = runes
                    .iter()
                    .zip(current)
                    .take_while(|(a, b)| a == b)
                    .count();
                if same > 0 {
                    runes.truncate(same);
                    continue;
                }
            }
            if index > start + 1 {
                let prefix = self.literal_string(&runes, rune_flags)?;
                for &id in &sub[start..index] {
                    self.remove_leading_string(id, runes.len())?;
                }
                push(
                    &mut splices,
                    Splice {
                        prefix,
                        first: start,
                        count: index - start,
                        suffix: None,
                    },
                )?;
            }
            if let Some((current, current_flags)) = candidate {
                start = index;
                runes = current;
                rune_flags = current_flags;
            }
        }
        Ok(splices)
    }

    fn simple_prefix(&self, id: Id) -> Result<bool, Status> {
        Ok(match &self.node(id)?.op {
            Op::BeginLine
            | Op::EndLine
            | Op::WordBoundary
            | Op::NotWordBoundary
            | Op::BeginText
            | Op::EndText
            | Op::Class(_)
            | Op::AnyChar
            | Op::AnyByte => true,
            Op::Repeat {
                child,
                min,
                max: Some(max),
            } if min == max => {
                matches!(
                    self.node(*child)?.op,
                    Op::Literal(_) | Op::Class(_) | Op::AnyChar | Op::AnyByte
                )
            }
            Op::ClassBuilder(_) => return Err(internal()),
            _ => false,
        })
    }

    fn factor_round2(&mut self, sub: &mut [Id]) -> Result<Vec<Splice>, Status> {
        let mut splices = Vec::new();
        let mut start = 0;
        let mut first: Option<Id> = None;
        for index in 0..=sub.len() {
            let candidate = if index < sub.len() {
                self.leading_regexp(sub[index])?
            } else {
                None
            };
            if index < sub.len()
                && let (Some(previous), Some(current)) = (first, candidate)
                && self.simple_prefix(previous)?
                && self.equal(previous, current)?
            {
                continue;
            }
            if index > start + 1 {
                let prefix = first.ok_or_else(internal)?;
                for id in &mut sub[start..index] {
                    *id = self.remove_leading_regexp(*id)?;
                }
                push(
                    &mut splices,
                    Splice {
                        prefix,
                        first: start,
                        count: index - start,
                        suffix: None,
                    },
                )?;
            }
            if index < sub.len() {
                start = index;
                first = candidate;
            }
        }
        Ok(splices)
    }

    fn factor_round3(&mut self, sub: &[Id], flags: u16) -> Result<Vec<Splice>, Status> {
        let mut splices = Vec::new();
        let mut start = 0;
        let mut first: Option<Id> = None;
        for index in 0..=sub.len() {
            let candidate = sub.get(index).copied();
            if let (Some(previous), Some(current)) = (first, candidate) {
                let eligible = |op: &Op| matches!(op, Op::Literal(_) | Op::Class(_));
                if eligible(&self.node(previous)?.op) && eligible(&self.node(current)?.op) {
                    continue;
                }
            }
            if index > start + 1 {
                let mut cc = Vec::new();
                for &id in &sub[start..index] {
                    let node = self.node(id)?;
                    match &node.op {
                        Op::Class(class) => {
                            for &(lo, hi) in class {
                                ranges::add(&mut cc, lo, hi)?;
                            }
                        }
                        Op::Literal(rune) => ranges::add_literal_flags(&mut cc, *rune, node.flags)?,
                        _ => return Err(internal()),
                    }
                }
                // NewCharClass is FINISHED: do not run builder singleton/[Aa]
                // canonicalization, and do not discard its empty/full state.
                let prefix = self.new_class(cc, flags)?;
                push(
                    &mut splices,
                    Splice {
                        prefix,
                        first: start,
                        count: index - start,
                        suffix: None,
                    },
                )?;
            }
            if index < sub.len() {
                start = index;
                first = candidate;
            }
        }
        Ok(splices)
    }
}
