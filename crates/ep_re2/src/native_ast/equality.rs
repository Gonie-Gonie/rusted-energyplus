// Copyright 2006 The RE2 Authors. All Rights Reserved.
// BSD-style license: ../RE2-LICENSE.
//! Regexp::TopEqual/Equal, regexp.cc342-478, without recursive ownership.
use super::{Arena, Id, Op, flags, internal, push};
use crate::Status;
use std::sync::Arc;

impl Arena {
    fn top_equal(&self, a: Id, b: Id) -> Result<bool, Status> {
        let a = self.node(a)?;
        let b = self.node(b)?;
        if a.op.opcode() != b.op.opcode() {
            return Ok(false);
        }
        let equal = match (&a.op, &b.op) {
            (Op::NoMatch, Op::NoMatch)
            | (Op::Empty, Op::Empty)
            | (Op::AnyChar, Op::AnyChar)
            | (Op::AnyByte, Op::AnyByte)
            | (Op::BeginLine, Op::BeginLine)
            | (Op::EndLine, Op::EndLine)
            | (Op::WordBoundary, Op::WordBoundary)
            | (Op::NotWordBoundary, Op::NotWordBoundary)
            | (Op::BeginText, Op::BeginText) => true,
            (Op::EndText, Op::EndText) => (a.flags ^ b.flags) & flags::WAS_DOLLAR == 0,
            (Op::Literal(x), Op::Literal(y)) => {
                x == y && (a.flags ^ b.flags) & flags::FOLD_CASE == 0
            }
            (Op::LiteralString(x), Op::LiteralString(y)) => {
                x == y && (a.flags ^ b.flags) & flags::FOLD_CASE == 0
            }
            (Op::Concat(x), Op::Concat(y)) | (Op::Alternate(x), Op::Alternate(y)) => {
                x.len() == y.len()
            }
            (Op::Star(_), Op::Star(_))
            | (Op::Plus(_), Op::Plus(_))
            | (Op::Quest(_), Op::Quest(_)) => (a.flags ^ b.flags) & flags::NON_GREEDY == 0,
            (
                Op::Repeat {
                    min: x, max: xx, ..
                },
                Op::Repeat {
                    min: y, max: yy, ..
                },
            ) => x == y && xx == yy && (a.flags ^ b.flags) & flags::NON_GREEDY == 0,
            (
                Op::Capture {
                    index: x, name: xn, ..
                },
                Op::Capture {
                    index: y, name: yn, ..
                },
            ) => {
                x == y
                    && match (xn, yn) {
                        (None, None) => true,
                        // Native compares the optional std::string POINTER here.
                        // Arc cloning retains that identity instead of value-equality.
                        (Some(x), Some(y)) => Arc::ptr_eq(x, y),
                        _ => false,
                    }
            }
            (Op::Class(x), Op::Class(y)) => x == y,
            (Op::HaveMatch(x), Op::HaveMatch(y)) => x == y,
            (Op::ClassBuilder(_), _) | (_, Op::ClassBuilder(_)) => return Err(internal()),
            _ => false,
        };
        Ok(equal)
    }

    pub(crate) fn equal(&self, a: Id, b: Id) -> Result<bool, Status> {
        let mut pending = vec![(a, b)];
        while let Some((a, b)) = pending.pop() {
            if !self.top_equal(a, b)? {
                return Ok(false);
            }
            match (&self.node(a)?.op, &self.node(b)?.op) {
                (Op::Concat(x), Op::Concat(y)) | (Op::Alternate(x), Op::Alternate(y)) => {
                    for (&x, &y) in x.iter().zip(y) {
                        push(&mut pending, (x, y))?;
                    }
                }
                (x, y) => {
                    if let (Some(x), Some(y)) = (x.unary_child(), y.unary_child()) {
                        push(&mut pending, (x, y))?;
                    }
                }
            }
        }
        Ok(true)
    }
}
