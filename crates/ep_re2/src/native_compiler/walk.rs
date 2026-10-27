// Copyright 2006, 2007 The RE2 Authors. All Rights Reserved.
// Pinned walker-inl.h178-275 and compile.cc789-980. BSD: ../RE2-LICENSE.
use super::{Arena, Compiler, Frag, Id, Op, Status, allocation, flags, internal, push};

fn children(op: &Op) -> &[Id] {
    match op {
        Op::Concat(children) | Op::Alternate(children) => children,
        Op::Star(child)
        | Op::Plus(child)
        | Op::Quest(child)
        | Op::Repeat { child, .. }
        | Op::Capture { child, .. } => std::slice::from_ref(child),
        _ => &[],
    }
}
struct Frame {
    id: Id,
    next: Option<usize>,
    args: Vec<Frag>,
}
impl Frame {
    fn new(id: Id) -> Self {
        Self {
            id,
            next: None,
            args: Vec::new(),
        }
    }
}

impl Compiler {
    /// Native WalkExponential: every occurrence is visited, including repeated
    /// DAG identities. No Copy(), memoization or semantic-size substitution.
    pub(super) fn walk(&mut self, arena: &Arena, root: Id) -> Result<Frag, Status> {
        let mut stack = Vec::new();
        push(&mut stack, Frame::new(root))?;
        loop {
            let frame = stack.last_mut().ok_or_else(internal)?;
            let node = arena.node(frame.id)?;
            let mut complete = None;
            if frame.next.is_none() {
                // This decrement precedes PreVisit, including visits after
                // failed_ is set. ShortVisit sets failed_, never compiles kids.
                self.walk_remaining_budget -= 1;
                self.walk_visits += 1;
                if self.walk_remaining_budget < 0 {
                    self.failed = true;
                    complete = Some(Frag::default());
                } else if self.failed {
                    complete = Some(Frag::default());
                } else {
                    frame
                        .args
                        .try_reserve(children(&node.op).len())
                        .map_err(|_| allocation())?;
                    frame.next = Some(0);
                }
            }
            if complete.is_none() {
                let next = frame.next.ok_or_else(internal)?;
                if let Some(&child) = children(&node.op).get(next) {
                    push(&mut stack, Frame::new(child))?;
                    continue;
                }
                complete = Some(self.post_visit(&node.op, node.flags, &frame.args)?);
            }
            let value = complete.ok_or_else(internal)?;
            stack.pop();
            let Some(parent) = stack.last_mut() else {
                return Ok(value);
            };
            push(&mut parent.args, value)?;
            let next = parent.next.as_mut().ok_or_else(internal)?;
            *next += 1;
        }
    }

    fn post_visit(&mut self, op: &Op, bits: u16, args: &[Frag]) -> Result<Frag, Status> {
        if self.failed {
            return Ok(Frag::default());
        }
        let first = || args.first().copied().ok_or_else(internal);
        let nongreedy = bits & flags::NON_GREEDY != 0;
        match op {
            Op::Repeat { .. } | Op::ClassBuilder(_) => {
                // Compile requires Native Simplify before this walk. Native's
                // missing Repeat switch case sets failed_ and returns NoMatch.
                self.failed = true;
                Ok(Frag::default())
            }
            Op::NoMatch => Ok(Frag::default()),
            Op::Empty => self.nop(),
            Op::HaveMatch(id) => self.match_inst(*id), // ordinary Compile is UNANCHORED, not CompileSet
            Op::Concat(_) => {
                let mut result = first()?;
                for &child in &args[1..] {
                    result = self.cat(result, child)?;
                }
                Ok(result)
            }
            Op::Alternate(_) => {
                let mut result = first()?;
                for &child in &args[1..] {
                    result = self.alt(result, child)?;
                }
                Ok(result)
            }
            Op::Star(_) => self.star(first()?, nongreedy),
            Op::Plus(_) => self.plus(first()?, nongreedy),
            Op::Quest(_) => self.quest(first()?, nongreedy),
            Op::Literal(rune) => self.literal(*rune, bits & flags::FOLD_CASE != 0),
            Op::LiteralString(runes) => {
                if runes.is_empty() {
                    return self.nop();
                }
                let mut result = Frag::default();
                for (index, &rune) in runes.iter().enumerate() {
                    let next = self.literal(rune, bits & flags::FOLD_CASE != 0)?;
                    result = if index == 0 {
                        next
                    } else {
                        self.cat(result, next)?
                    };
                }
                Ok(result)
            }
            Op::AnyChar => {
                self.begin_range();
                self.add_rune_range(0, 0x10ffff, false)?;
                Ok(self.end_range())
            }
            Op::AnyByte => self.byte_range(0, 255, false),
            Op::Class(ranges) => {
                if ranges.is_empty() {
                    self.failed = true;
                    return Ok(Frag::default());
                }
                // Same metadata predicate as the retained finished CharClass,
                // not Unicode case folding and not the node's FoldCase bit.
                let contains = |rune| ranges.iter().any(|&(lo, hi)| lo <= rune && rune <= hi);
                let foldascii = (65_u32..=90).all(|upper| contains(upper) == contains(upper + 32));
                self.begin_range();
                for &(lo, hi) in ranges {
                    if foldascii && 65 <= lo && hi <= 90 {
                        continue;
                    }
                    let fold = foldascii
                        && !((lo <= 65 && 122 <= hi)
                            || hi < 65
                            || 122 < lo
                            || (90 < lo && hi < 97));
                    self.add_rune_range(lo, hi, fold)?;
                }
                Ok(self.end_range())
            }
            Op::Capture { index, .. } => self.capture(first()?, *index),
            Op::BeginLine => self.empty_width(1 << 0),
            Op::EndLine => self.empty_width(1 << 1),
            Op::BeginText => self.empty_width(1 << 2),
            Op::EndText => self.empty_width(1 << 3),
            Op::WordBoundary => self.empty_width(1 << 4),
            Op::NotWordBoundary => self.empty_width(1 << 5),
        }
    }
}
