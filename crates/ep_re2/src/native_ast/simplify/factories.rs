// Copyright 2006 The RE2 Authors. All Rights Reserved.
// BSD-style license: ../../RE2-LICENSE.
//! Exact regexp.cc196-228 unary factories and plain Repeat/Concat2 constructors.
use super::{Arena, Id, Op, Status, Unary};

impl Arena {
    fn native_unary_factory(&mut self, child: Id, bits: u16, kind: Unary) -> Result<Id, Status> {
        let node = self.node(child)?;
        let same = matches!(
            (&node.op, kind),
            (Op::Star(_), Unary::Star) | (Op::Plus(_), Unary::Plus) | (Op::Quest(_), Unary::Quest)
        );
        if node.flags == bits {
            if same {
                return Ok(child);
            }
            match &node.op {
                Op::Star(_) => return Ok(child),
                Op::Plus(grandchild) | Op::Quest(grandchild) => {
                    return self.new_node(Op::Star(*grandchild), bits);
                }
                _ => {}
            }
        }
        let op = match kind {
            Unary::Star => Op::Star(child),
            Unary::Plus => Op::Plus(child),
            Unary::Quest => Op::Quest(child),
        };
        self.new_node(op, bits)
    }
    pub(crate) fn native_star(&mut self, child: Id, bits: u16) -> Result<Id, Status> {
        self.native_unary_factory(child, bits, Unary::Star)
    }
    pub(crate) fn native_plus(&mut self, child: Id, bits: u16) -> Result<Id, Status> {
        self.native_unary_factory(child, bits, Unary::Plus)
    }
    pub(crate) fn native_quest(&mut self, child: Id, bits: u16) -> Result<Id, Status> {
        self.native_unary_factory(child, bits, Unary::Quest)
    }
    pub(crate) fn native_repeat(
        &mut self,
        child: Id,
        min: u32,
        max: Option<u32>,
        bits: u16,
    ) -> Result<Id, Status> {
        self.new_node(Op::Repeat { child, min, max }, bits)
    }
    pub(super) fn native_concat2(
        &mut self,
        first: Id,
        second: Id,
        bits: u16,
    ) -> Result<Id, Status> {
        // SimplifyWalker::Concat2 does NOT call the normal n-ary constructor.
        self.new_node(Op::Concat(vec![first, second]), bits)
    }
}
