// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Pinned prog.h Inst and prog.cc32-72,197-285. BSD: ../RE2-LICENSE.
use super::{Status, allocation, internal, push};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum InstOp {
    Alt = 0,
    AltMatch,
    ByteRange,
    Capture,
    EmptyWidth,
    Match,
    Nop,
    Fail,
}

/// Native's actual two-word instruction layout; payload is its uint32 union.
/// Byte-range payload packing is the pinned little-endian Windows layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct Inst {
    pub out_opcode: u32,
    pub payload: u32,
}
impl Inst {
    pub(crate) fn opcode(self) -> InstOp {
        match self.out_opcode & 7 {
            0 => InstOp::Alt,
            1 => InstOp::AltMatch,
            2 => InstOp::ByteRange,
            3 => InstOp::Capture,
            4 => InstOp::EmptyWidth,
            5 => InstOp::Match,
            6 => InstOp::Nop,
            _ => InstOp::Fail,
        }
    }
    pub(crate) fn out(self) -> u32 {
        self.out_opcode >> 4
    }
    pub(crate) fn last(self) -> bool {
        self.out_opcode & 8 != 0
    }
    pub(crate) fn lo(self) -> u8 {
        self.payload as u8
    }
    pub(crate) fn hi(self) -> u8 {
        (self.payload >> 8) as u8
    }
    pub(crate) fn foldcase(self) -> bool {
        self.payload & (1 << 16) != 0
    }
    pub(crate) fn hint(self) -> u16 {
        (self.payload >> 17) as u16
    }
    pub(crate) fn set_out(&mut self, out: u32) {
        self.out_opcode = (out << 4) | (self.out_opcode & 15);
    }
    pub(crate) fn set_opcode(&mut self, op: InstOp) {
        self.out_opcode = (self.out_opcode & !7) | op as u32;
    }
    pub(crate) fn alt(out: u32, out1: u32) -> Self {
        Self {
            out_opcode: out << 4,
            payload: out1,
        }
    }
    pub(crate) fn byte_range(lo: u8, hi: u8, fold: bool, out: u32) -> Self {
        Self {
            out_opcode: (out << 4) | InstOp::ByteRange as u32,
            payload: u32::from(lo) | (u32::from(hi) << 8) | (u32::from(fold) << 16),
        }
    }
    pub(crate) fn capture(index: i32, out: u32) -> Self {
        Self {
            out_opcode: (out << 4) | InstOp::Capture as u32,
            payload: index as u32,
        }
    }
    pub(crate) fn empty_width(empty: u32, out: u32) -> Self {
        Self {
            out_opcode: (out << 4) | InstOp::EmptyWidth as u32,
            payload: empty,
        }
    }
    pub(crate) fn matched(id: i32) -> Self {
        Self {
            out_opcode: InstOp::Match as u32,
            payload: id as u32,
        }
    }
    pub(crate) fn nop() -> Self {
        Self {
            out_opcode: InstOp::Nop as u32,
            payload: 0,
        }
    }
    pub(crate) fn fail() -> Self {
        Self {
            out_opcode: InstOp::Fail as u32,
            payload: 0,
        }
    }
}

/// This is still tree-form bytecode, even after Optimize. It MUST NOT be passed
/// to an execution engine that expects Native flattened lists/hints/bytemap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct InstructionGraph {
    pub instructions: Vec<Inst>,
    pub start: u32,
    pub start_unanchored: u32,
    pub anchor_start: bool,
    pub anchor_end: bool,
    pub optimized: bool,
}
impl InstructionGraph {
    fn inst(&self, id: u32) -> Result<Inst, Status> {
        self.instructions
            .get(id as usize)
            .copied()
            .ok_or_else(internal)
    }
    fn queue(&self, id: u32, work: &mut Vec<u32>, seen: &mut [bool]) -> Result<(), Status> {
        let marked = seen.get_mut(id as usize).ok_or_else(internal)?;
        if id != 0 && !*marked {
            *marked = true;
            push(work, id)?;
        }
        Ok(())
    }
    fn is_match(&self, mut id: u32) -> Result<bool, Status> {
        loop {
            let inst = self.inst(id)?;
            match inst.opcode() {
                InstOp::Match => return Ok(true),
                InstOp::Capture | InstOp::Nop => id = inst.out(),
                _ => return Ok(false),
            }
        }
    }
    /// Exact two-pass reachable queue order of Prog::Optimize. This allocates
    /// scratch only; it does not apply AllocInst budget or remove array entries.
    pub(crate) fn optimize(&mut self) -> Result<(), Status> {
        let mut seen = Vec::new();
        seen.try_reserve_exact(self.instructions.len())
            .map_err(|_| allocation())?;
        seen.resize(self.instructions.len(), false);
        let mut work = Vec::new();
        self.queue(self.start, &mut work, &mut seen)?;
        let mut index = 0;
        while index < work.len() {
            let id = work[index];
            index += 1;
            let mut inst = self.inst(id)?;
            let mut next = inst.out();
            while next != 0 && self.inst(next)?.opcode() == InstOp::Nop {
                next = self.inst(next)?.out();
            }
            inst.set_out(next);
            self.queue(next, &mut work, &mut seen)?;
            if inst.opcode() == InstOp::Alt {
                let mut next = inst.payload;
                while next != 0 && self.inst(next)?.opcode() == InstOp::Nop {
                    next = self.inst(next)?.out();
                }
                inst.payload = next;
                self.queue(next, &mut work, &mut seen)?;
            }
            self.instructions[id as usize] = inst;
        }
        work.clear();
        seen.fill(false);
        self.queue(self.start, &mut work, &mut seen)?;
        let mut index = 0;
        while index < work.len() {
            let id = work[index];
            index += 1;
            let mut inst = self.inst(id)?;
            self.queue(inst.out(), &mut work, &mut seen)?;
            if inst.opcode() == InstOp::Alt {
                self.queue(inst.payload, &mut work, &mut seen)?;
            }
            if inst.opcode() == InstOp::Alt {
                let left = self.inst(inst.out())?;
                let right = self.inst(inst.payload)?;
                if (left.opcode() == InstOp::ByteRange
                    && left.out() == id
                    && left.lo() == 0
                    && left.hi() == 255
                    && self.is_match(inst.payload)?)
                    || (self.is_match(inst.out())?
                        && right.opcode() == InstOp::ByteRange
                        && right.out() == id
                        && right.lo() == 0
                        && right.hi() == 255)
                {
                    inst.set_opcode(InstOp::AltMatch);
                    self.instructions[id as usize] = inst;
                }
            }
        }
        self.optimized = true;
        Ok(())
    }
}
