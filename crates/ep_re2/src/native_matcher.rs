// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Boolean-only Rust port of pinned nfa.cc194-311 and prog.cc286-319.
// BSD license: RE2-LICENSE.
//! FullMatch over finalized Native lists, using original text assertion context.
//! No capture extraction, unanchored search, DFA cache or resource claim.
use crate::native_compiler::{Inst, InstOp};

pub(crate) struct ProgramView<'a> {
    pub instructions: &'a [Inst],
    pub start: u32,
    pub anchor_start: bool,
}

fn word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Prog::EmptyFlags, with byte offsets replacing pointers into the SAME context.
fn empty_flags(text: &[u8], position: usize) -> u32 {
    let previous = position
        .checked_sub(1)
        .and_then(|index| text.get(index))
        .copied();
    let current = text.get(position).copied();
    let mut flags = 0;
    if position == 0 {
        flags |= (1 << 2) | (1 << 0);
    } else if previous == Some(b'\n') {
        flags |= 1 << 0;
    }
    if position == text.len() {
        flags |= (1 << 3) | (1 << 1);
    } else if current == Some(b'\n') {
        flags |= 1 << 1;
    }
    if previous.is_some_and(word) != current.is_some_and(word) {
        flags |= 1 << 4;
    } else {
        flags |= 1 << 5;
    }
    flags
}

fn matches(instruction: Inst, byte: Option<u8>) -> bool {
    let Some(mut byte) = byte else {
        return false;
    };
    if instruction.foldcase() && byte.is_ascii_uppercase() {
        byte += b'a' - b'A';
    }
    instruction.lo() <= byte && byte <= instruction.hi()
}

#[derive(Clone, Copy)]
struct State {
    id: u32,
    active: bool,
}

/// Same ordered nullable closure and hint branches as NFA::AddToThreadq.
/// At FullMatch's fixed start, all threads have the same whole-match start;
/// user capture registers are not requested. Their copies/refcounts cannot
/// affect boolean acceptance and are omitted. Inactive queue entries remain:
/// they prevent revisits and participate in the AltMatch first-entry guard.
struct Closure<'a> {
    instructions: &'a [Inst],
    seen_at: Vec<usize>,
    states: Vec<State>,
    stack: Vec<u32>,
}

impl<'a> Closure<'a> {
    fn new(instructions: &'a [Inst]) -> Self {
        Self {
            instructions,
            seen_at: vec![usize::MAX; instructions.len()],
            states: Vec::new(),
            stack: Vec::new(),
        }
    }

    /// Return false only for an invalid, non-finalized internal program. Such
    /// graphs are outside this private matcher input contract.
    fn add(&mut self, seed: u32, byte: Option<u8>, flags: u32, position: usize) -> bool {
        if seed == 0 {
            return true;
        }
        self.stack.clear();
        self.stack.push(seed);
        while let Some(mut id) = self.stack.pop() {
            loop {
                if id == 0 {
                    break;
                }
                let Some(seen) = self.seen_at.get_mut(id as usize) else {
                    return false;
                };
                if *seen == position {
                    break;
                }
                *seen = position;
                let Some(&instruction) = self.instructions.get(id as usize) else {
                    return false;
                };
                let slot = self.states.len();
                self.states.push(State { id, active: false });
                match instruction.opcode() {
                    InstOp::Fail => break,
                    InstOp::AltMatch => {
                        self.states[slot].active = true;
                        if instruction.last() {
                            return false;
                        }
                        id += 1;
                    }
                    InstOp::Nop | InstOp::Capture => {
                        if !instruction.last() {
                            self.stack.push(id + 1);
                        }
                        id = instruction.out();
                    }
                    InstOp::EmptyWidth => {
                        if !instruction.last() {
                            self.stack.push(id + 1);
                        }
                        if instruction.payload & !flags != 0 {
                            break;
                        }
                        id = instruction.out();
                    }
                    InstOp::ByteRange => {
                        if matches(instruction, byte) {
                            self.states[slot].active = true;
                            let hint = instruction.hint();
                            if hint == 0 {
                                break;
                            }
                            id += u32::from(hint);
                        } else {
                            if instruction.last() {
                                break;
                            }
                            id += 1;
                        }
                    }
                    InstOp::Match => {
                        self.states[slot].active = true;
                        if instruction.last() {
                            break;
                        }
                        id += 1;
                    }
                    // Flatten removes Alt. Native AddToThreadq also has no
                    // execution case for a remaining tree-form Alt.
                    InstOp::Alt => return false,
                }
            }
        }
        true
    }
}

/// ANCHOR_BOTH, endpos=context.len(), no submatch extraction. Prefix removal
/// changes only the start offset; every assertion sees the original context.
pub(crate) fn full_match(program: ProgramView<'_>, context: &[u8], start: usize) -> bool {
    if start > context.len() || (program.anchor_start && start != 0) || program.start == 0 {
        return false;
    }
    // SearchNFA/OnePass/DFA additionally check anchor_end against the end of
    // the supplied search slice. This FullMatch-only view always retains that
    // original end, so it does not request that redundant metadata operand.
    let end = context.len();
    let mut closure = Closure::new(program.instructions);
    let mut seeds = vec![program.start];
    let mut next = Vec::new();
    for position in start..=end {
        closure.states.clear();
        let flags = empty_flags(context, position);
        let byte = context.get(position).copied();
        for &seed in &seeds {
            if !closure.add(seed, byte, flags, position) {
                return false;
            }
        }
        next.clear();
        for (ordinal, state) in closure.states.iter().enumerate() {
            if !state.active {
                continue;
            }
            let instruction = program.instructions[state.id as usize];
            match instruction.opcode() {
                InstOp::ByteRange => next.push(instruction.out()),
                InstOp::AltMatch if ordinal == 0 => {
                    // FullMatch uses longest mode. Optimize recognizes only
                    // a [00-FF] loop with a Capture/Nop-only path to Match.
                    // Native Step takes its guaranteed full-rest shortcut.
                    return true;
                }
                InstOp::Match if position == end => return true,
                InstOp::AltMatch | InstOp::Match => {}
                _ => return false,
            }
        }
        if next.is_empty() {
            return false;
        }
        std::mem::swap(&mut seeds, &mut next);
    }
    false
}
