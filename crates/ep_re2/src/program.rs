//! Explicit-stack compilation and byte-position NFA full matching.
//! Native FullMatchN uses ANCHOR_BOTH; prog.cc::EmptyFlags defines assertions.
//! nfa.cc visits each instruction once per position. No capture answers needed.
use crate::byte_class::{ByteClass, encode_rune};
use crate::{Assertion, ErrorCode, Node, NodeId, Parsed, Status};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Matcher {
    Literal { bytes: [u8; 4], width: usize },
    Ranges(Arc<ByteClass>),
    Byte,
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Match,
    Fail,
    Consume { matcher: Matcher, next: usize },
    Split { first: usize, second: usize },
    Assert { assertion: Assertion, next: usize },
    Capture { slot: u32, next: usize },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Program {
    instructions: Vec<Instruction>,
    start: usize,
}
enum Work {
    Build(NodeId, usize),
    Sequence {
        id: NodeId,
        remaining: usize,
    },
    Alternate {
        id: NodeId,
        remaining: usize,
        tail: usize,
        start: Option<usize>,
    },
    AlternativeDone {
        id: NodeId,
        remaining: usize,
        tail: usize,
        start: Option<usize>,
    },
    Repetitions {
        child: NodeId,
        required: u32,
        optional: u32,
        tail: usize,
    },
    OptionalDone {
        child: NodeId,
        required: u32,
        optional: u32,
        tail: usize,
    },
    RequiredDone {
        child: NodeId,
        required: u32,
    },
    StarDone {
        child: NodeId,
        min: u32,
        split: usize,
    },
    CaptureDone {
        slot: u32,
    },
}
fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}
fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}
fn append<T>(values: &mut Vec<T>, value: T) -> Result<usize, Status> {
    values.try_reserve(1).map_err(|_| allocation())?;
    let index = values.len();
    values.push(value);
    Ok(index)
}

impl Program {
    pub(crate) fn compile(parsed: Parsed) -> Result<Self, Status> {
        let mut instructions = vec![Instruction::Match];
        let mut classes: Vec<Option<Arc<ByteClass>>> = Vec::new();
        classes
            .try_reserve(parsed.nodes.len())
            .map_err(|_| allocation())?;
        classes.resize_with(parsed.nodes.len(), || None);
        let mut work = vec![Work::Build(parsed.root, 0)];
        let mut result = 0;
        while let Some(task) = work.pop() {
            match task {
                Work::Build(id, tail) => match parsed.nodes.get(id).ok_or_else(internal)? {
                    Node::Empty => result = tail,
                    Node::NoMatch => result = append(&mut instructions, Instruction::Fail)?,
                    Node::Literal(rune) => {
                        let (bytes, width) = encode_rune(*rune);
                        result = append(
                            &mut instructions,
                            Instruction::Consume {
                                matcher: Matcher::Literal { bytes, width },
                                next: tail,
                            },
                        )?;
                    }
                    Node::AnyByte => {
                        result = append(
                            &mut instructions,
                            Instruction::Consume {
                                matcher: Matcher::Byte,
                                next: tail,
                            },
                        )?
                    }
                    Node::Class(ranges) => {
                        let cached = classes.get_mut(id).ok_or_else(internal)?;
                        if cached.is_none() {
                            *cached = Some(Arc::new(ByteClass::from_ranges(ranges)?));
                        }
                        let ranges = cached.as_ref().ok_or_else(internal)?.clone();
                        result = append(
                            &mut instructions,
                            Instruction::Consume {
                                matcher: Matcher::Ranges(ranges),
                                next: tail,
                            },
                        )?;
                    }
                    Node::Assert(assertion) => {
                        result = append(
                            &mut instructions,
                            Instruction::Assert {
                                assertion: *assertion,
                                next: tail,
                            },
                        )?
                    }
                    Node::Concat(children) => {
                        result = tail;
                        append(
                            &mut work,
                            Work::Sequence {
                                id,
                                remaining: children.len(),
                            },
                        )?;
                    }
                    Node::Alternate(children) => {
                        append(
                            &mut work,
                            Work::Alternate {
                                id,
                                remaining: children.len(),
                                tail,
                                start: None,
                            },
                        )?;
                    }
                    Node::Repeat {
                        child,
                        min,
                        max: Some(max),
                    } => {
                        let optional = max.checked_sub(*min).ok_or_else(internal)?;
                        append(
                            &mut work,
                            Work::Repetitions {
                                child: *child,
                                required: *min,
                                optional,
                                tail,
                            },
                        )?;
                    }
                    Node::Repeat {
                        child,
                        min,
                        max: None,
                    } => {
                        let split = append(
                            &mut instructions,
                            Instruction::Split {
                                first: 0,
                                second: tail,
                            },
                        )?;
                        append(
                            &mut work,
                            Work::StarDone {
                                child: *child,
                                min: *min,
                                split,
                            },
                        )?;
                        append(&mut work, Work::Build(*child, split))?;
                    }
                    Node::Capture { child, index } => {
                        let slot = index.checked_mul(2).ok_or_else(internal)?;
                        let end = slot.checked_add(1).ok_or_else(internal)?;
                        let next = append(
                            &mut instructions,
                            Instruction::Capture {
                                slot: end,
                                next: tail,
                            },
                        )?;
                        append(&mut work, Work::CaptureDone { slot })?;
                        append(&mut work, Work::Build(*child, next))?;
                    }
                },
                Work::Sequence { id, remaining } => {
                    if remaining != 0 {
                        let Node::Concat(children) = parsed.nodes.get(id).ok_or_else(internal)?
                        else {
                            return Err(internal());
                        };
                        let child = *children.get(remaining - 1).ok_or_else(internal)?;
                        append(
                            &mut work,
                            Work::Sequence {
                                id,
                                remaining: remaining - 1,
                            },
                        )?;
                        append(&mut work, Work::Build(child, result))?;
                    }
                }
                Work::Alternate {
                    id,
                    remaining,
                    tail,
                    start,
                } => {
                    if remaining == 0 {
                        result = match start {
                            Some(start) => start,
                            None => append(&mut instructions, Instruction::Fail)?,
                        };
                    } else {
                        let Node::Alternate(children) =
                            parsed.nodes.get(id).ok_or_else(internal)?
                        else {
                            return Err(internal());
                        };
                        let child = *children.get(remaining - 1).ok_or_else(internal)?;
                        append(
                            &mut work,
                            Work::AlternativeDone {
                                id,
                                remaining: remaining - 1,
                                tail,
                                start,
                            },
                        )?;
                        append(&mut work, Work::Build(child, tail))?;
                    }
                }
                Work::AlternativeDone {
                    id,
                    remaining,
                    tail,
                    start,
                } => {
                    let combined = match start {
                        Some(second) => append(
                            &mut instructions,
                            Instruction::Split {
                                first: result,
                                second,
                            },
                        )?,
                        None => result,
                    };
                    append(
                        &mut work,
                        Work::Alternate {
                            id,
                            remaining,
                            tail,
                            start: Some(combined),
                        },
                    )?;
                }
                Work::Repetitions {
                    child,
                    required,
                    optional,
                    tail,
                } => {
                    if optional != 0 {
                        append(
                            &mut work,
                            Work::OptionalDone {
                                child,
                                required,
                                optional: optional - 1,
                                tail,
                            },
                        )?;
                        append(&mut work, Work::Build(child, tail))?;
                    } else if required != 0 {
                        append(
                            &mut work,
                            Work::RequiredDone {
                                child,
                                required: required - 1,
                            },
                        )?;
                        append(&mut work, Work::Build(child, tail))?;
                    } else {
                        result = tail;
                    }
                }
                Work::OptionalDone {
                    child,
                    required,
                    optional,
                    tail,
                } => {
                    let tail = append(
                        &mut instructions,
                        Instruction::Split {
                            first: result,
                            second: tail,
                        },
                    )?;
                    append(
                        &mut work,
                        Work::Repetitions {
                            child,
                            required,
                            optional,
                            tail,
                        },
                    )?;
                }
                Work::RequiredDone { child, required } => {
                    append(
                        &mut work,
                        Work::Repetitions {
                            child,
                            required,
                            optional: 0,
                            tail: result,
                        },
                    )?;
                }
                Work::StarDone { child, min, split } => {
                    let Some(Instruction::Split { first, .. }) = instructions.get_mut(split) else {
                        return Err(internal());
                    };
                    *first = result;
                    if min == 0 {
                        result = split;
                    } else {
                        append(
                            &mut work,
                            Work::Repetitions {
                                child,
                                required: min - 1,
                                optional: 0,
                                tail: result,
                            },
                        )?;
                    }
                }
                Work::CaptureDone { slot } => {
                    result = append(
                        &mut instructions,
                        Instruction::Capture { slot, next: result },
                    )?;
                }
            }
        }
        Ok(Self {
            instructions,
            start: result,
        })
    }

    pub(crate) fn full_match(&self, text: &[u8]) -> bool {
        // Consumption advances 1..4 bytes. Five rolling queues retain future
        // positions without allocating a queue for every byte of a long key.
        let mut pending: [Vec<usize>; 5] = std::array::from_fn(|_| Vec::new());
        pending[0].push(self.start);
        let mut visited = vec![usize::MAX; self.instructions.len()];
        for position in 0..=text.len() {
            let mut stack = std::mem::take(&mut pending[position % 5]);
            while let Some(index) = stack.pop() {
                let Some(seen) = visited.get_mut(index) else {
                    return false;
                };
                if *seen == position {
                    continue;
                }
                *seen = position;
                let Some(instruction) = self.instructions.get(index) else {
                    return false;
                };
                match instruction {
                    Instruction::Match => {
                        if position == text.len() {
                            return true;
                        }
                    }
                    Instruction::Fail => {}
                    Instruction::Split { first, second } => {
                        stack.push(*second);
                        stack.push(*first);
                    }
                    Instruction::Capture { next, .. } => stack.push(*next),
                    Instruction::Assert { assertion, next } => {
                        if assertion.holds(text, position) {
                            stack.push(*next);
                        }
                    }
                    Instruction::Consume { matcher, next } => {
                        if let Some(width) = matcher.width(text.get(position..).unwrap_or_default())
                        {
                            let target = position + width;
                            if target <= text.len() {
                                pending[target % 5].push(*next);
                            }
                        }
                    }
                }
            }
            // No new unanchored start threads: FullMatch is ANCHOR_BOTH.
            if pending.iter().all(Vec::is_empty) {
                return false;
            }
        }
        false
    }
}

impl Matcher {
    fn width(&self, text: &[u8]) -> Option<usize> {
        match self {
            Self::Literal { bytes, width } => {
                (text.get(..*width) == Some(&bytes[..*width])).then_some(*width)
            }
            Self::Ranges(ranges) => ranges.match_width(text),
            Self::Byte => (!text.is_empty()).then_some(1),
        }
    }
}

impl Assertion {
    fn holds(self, text: &[u8], position: usize) -> bool {
        let previous = position
            .checked_sub(1)
            .and_then(|index| text.get(index))
            .copied();
        let current = text.get(position).copied();
        match self {
            Self::BeginText => position == 0,
            Self::EndText => position == text.len(),
            Self::BeginLine => position == 0 || previous == Some(b'\n'),
            Self::EndLine => position == text.len() || current == Some(b'\n'),
            Self::WordBoundary => previous.is_some_and(word) != current.is_some_and(word),
            Self::NotWordBoundary => previous.is_some_and(word) == current.is_some_and(word),
        }
    }
}
fn word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
