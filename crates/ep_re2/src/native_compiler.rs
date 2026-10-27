// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Rust port of pinned RE2 compile.cc; BSD license: RE2-LICENSE.
//! Exact forward compilation front end, ending before Prog finalization.
//! This module deliberately exposes no completed Program or matching API.
use crate::native_ast::{Arena, Id, Op, flags};
use crate::{ErrorCode, Status};
use std::collections::HashMap;

mod anchors;
mod graph;
mod range;
mod walk;
pub(crate) use graph::{Inst, InstOp, InstructionGraph};

pub(crate) const DEFAULT_MAX_MEM: i64 = 8 << 20;
pub(crate) const DEFAULT_FORWARD_MAX_MEM: i64 = DEFAULT_MAX_MEM * 2 / 3;
/// Effective default cap inferred from the pinned official DLL's adjacent
/// DataOutputs constructor gates, not a private sizeof(Prog) measurement.
pub(crate) const OFFICIAL_DEFAULT_FORWARD_INSTRUCTION_CAP: u32 = 698_995;

/// Supply actual sizeof values observed from the SAME pinned Native build.
/// In particular, sizeof(Prog) includes platform/library ABI state; this port
/// never substitutes Rust struct size or guesses that Native scalar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NativeLayout {
    pub prog_bytes: u64,
    pub inst_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Limits {
    pub max_mem: i64,
    /// Some only for explicitly observed unit ABI; runtime cap-only input has None.
    pub layout: Option<NativeLayout>,
    pub max_ninst: u32,
    pub initial_visit_budget: i64,
}
impl Limits {
    /// compile.cc1076-1104, with the pinned Inst's two uint32_t words.
    #[cfg(test)]
    pub(crate) fn from_native_layout(max_mem: i64, layout: NativeLayout) -> Result<Self, Status> {
        if layout.prog_bytes == 0 || layout.inst_bytes != 8 {
            return Err(internal());
        }
        let max_ninst = if max_mem <= 0 {
            100_000
        } else if u64::try_from(max_mem).map_err(|_| internal())? <= layout.prog_bytes {
            0
        } else {
            let count = (u64::try_from(max_mem).map_err(|_| internal())? - layout.prog_bytes)
                / layout.inst_bytes;
            count.min(1 << 24).min((1 << 28) - 1) as u32
        };
        Ok(Self {
            max_mem,
            layout: Some(layout),
            max_ninst,
            initial_visit_budget: i64::from(max_ninst) * 2,
        })
    }

    /// Same Setup instruction/visit semantics with an independently established
    /// effective cap. This route supplies NO Native allocation-layout metadata.
    pub(crate) fn from_instruction_cap(max_ninst: u32) -> Result<Self, Status> {
        if max_ninst > 1 << 24 {
            return Err(internal());
        }
        Ok(Self {
            max_mem: DEFAULT_FORWARD_MAX_MEM,
            layout: None,
            max_ninst,
            initial_visit_budget: i64::from(max_ninst) * 2,
        })
    }
}

/// A coherent pre-Optimize graph, NOT a completed Native Prog/constructor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FrontEnd {
    pub graph: InstructionGraph,
    pub limits: Limits,
    pub original_root: Id,
    pub supplied_simplified_root: Id,
    pub anchor_stripped_root: Id,
    pub walk_visits: u64,
    pub walk_remaining_budget: i64,
    pub native_array_slots_before_finalize: usize,
}

fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}
fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}
fn push<T>(values: &mut Vec<T>, value: T) -> Result<(), Status> {
    values.try_reserve(1).map_err(|_| allocation())?;
    values.push(value);
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PatchList {
    head: u32,
    tail: u32,
}
impl PatchList {
    fn single(slot: u32) -> Self {
        Self {
            head: slot,
            tail: slot,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Frag {
    begin: u32,
    end: PatchList,
    nullable: bool,
}
impl Frag {
    fn new(begin: u32, end: PatchList, nullable: bool) -> Self {
        Self {
            begin,
            end,
            nullable,
        }
    }
}

struct Compiler {
    slots: Vec<Inst>,
    used: u32,
    failed: bool,
    latin1: bool,
    limits: Limits,
    rune_cache: HashMap<u64, u32>,
    rune_range: Frag,
    walk_visits: u64,
    walk_remaining_budget: i64,
}
impl Compiler {
    fn new(bits: u16, limits: Limits) -> Result<Self, Status> {
        // Native Constructor first allocates Fail with temporary max_ninst=1,
        // before Setup replaces that limit. Its initial backing array is8.
        let mut slots = Vec::new();
        slots.try_reserve_exact(8).map_err(|_| allocation())?;
        slots.resize(8, Inst::default());
        slots[0] = Inst::fail();
        Ok(Self {
            slots,
            used: 1,
            failed: false,
            latin1: bits & flags::LATIN1 != 0,
            limits,
            rune_cache: HashMap::new(),
            rune_range: Frag::default(),
            walk_visits: 0,
            walk_remaining_budget: limits.initial_visit_budget,
        })
    }
    fn inst(&self, id: u32) -> Result<&Inst, Status> {
        if id >= self.used {
            return Err(internal());
        }
        self.slots.get(id as usize).ok_or_else(internal)
    }
    fn inst_mut(&mut self, id: u32) -> Result<&mut Inst, Status> {
        if id >= self.used {
            return Err(internal());
        }
        self.slots.get_mut(id as usize).ok_or_else(internal)
    }
    fn alloc_inst(&mut self, count: u32) -> Result<Option<u32>, Status> {
        let Some(end) = self.used.checked_add(count) else {
            self.failed = true;
            return Ok(None);
        };
        if self.failed || end > self.limits.max_ninst {
            self.failed = true;
            return Ok(None);
        }
        if end as usize > self.slots.len() {
            let mut capacity = self.slots.len().max(8);
            while end as usize > capacity {
                capacity = capacity.checked_mul(2).ok_or_else(allocation)?;
            }
            self.slots
                .try_reserve_exact(capacity - self.slots.len())
                .map_err(|_| allocation())?;
            self.slots.resize(capacity, Inst::default());
        }
        let id = self.used;
        self.used = end;
        Ok(Some(id))
    }
    fn patch(&mut self, mut list: PatchList, destination: u32) -> Result<(), Status> {
        while list.head != 0 {
            let slot = list.head;
            let inst = self.inst_mut(slot >> 1)?;
            if slot & 1 != 0 {
                list.head = inst.payload;
                inst.payload = destination;
            } else {
                list.head = inst.out();
                inst.set_out(destination);
            }
        }
        Ok(())
    }
    fn append(&mut self, left: PatchList, right: PatchList) -> Result<PatchList, Status> {
        if left.head == 0 {
            return Ok(right);
        }
        if right.head == 0 {
            return Ok(left);
        }
        let inst = self.inst_mut(left.tail >> 1)?;
        if left.tail & 1 != 0 {
            inst.payload = right.head;
        } else {
            inst.set_out(right.head);
        }
        Ok(PatchList {
            head: left.head,
            tail: right.tail,
        })
    }
    fn cat(&mut self, left: Frag, right: Frag) -> Result<Frag, Status> {
        if left.begin == 0 || right.begin == 0 {
            return Ok(Frag::default());
        }
        let first = self.inst(left.begin)?;
        if first.opcode() == InstOp::Nop && left.end.head == left.begin << 1 && first.out() == 0 {
            self.patch(left.end, right.begin)?;
            return Ok(right);
        }
        self.patch(left.end, right.begin)?;
        Ok(Frag::new(
            left.begin,
            right.end,
            left.nullable && right.nullable,
        ))
    }
    fn alt(&mut self, left: Frag, right: Frag) -> Result<Frag, Status> {
        if left.begin == 0 {
            return Ok(right);
        }
        if right.begin == 0 {
            return Ok(left);
        }
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        *self.inst_mut(id)? = Inst::alt(left.begin, right.begin);
        let end = self.append(left.end, right.end)?;
        Ok(Frag::new(id, end, left.nullable || right.nullable))
    }
    fn plus(&mut self, child: Frag, nongreedy: bool) -> Result<Frag, Status> {
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        let end = if nongreedy {
            *self.inst_mut(id)? = Inst::alt(0, child.begin);
            PatchList::single(id << 1)
        } else {
            *self.inst_mut(id)? = Inst::alt(child.begin, 0);
            PatchList::single((id << 1) | 1)
        };
        self.patch(child.end, id)?;
        Ok(Frag::new(child.begin, end, child.nullable))
    }
    fn star(&mut self, child: Frag, nongreedy: bool) -> Result<Frag, Status> {
        if child.nullable {
            let plus = self.plus(child, nongreedy)?;
            return self.quest(plus, nongreedy);
        }
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        let end = if nongreedy {
            *self.inst_mut(id)? = Inst::alt(0, child.begin);
            PatchList::single(id << 1)
        } else {
            *self.inst_mut(id)? = Inst::alt(child.begin, 0);
            PatchList::single((id << 1) | 1)
        };
        self.patch(child.end, id)?;
        Ok(Frag::new(id, end, true))
    }
    fn quest(&mut self, child: Frag, nongreedy: bool) -> Result<Frag, Status> {
        if child.begin == 0 {
            return self.nop();
        }
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        let end = if nongreedy {
            *self.inst_mut(id)? = Inst::alt(0, child.begin);
            PatchList::single(id << 1)
        } else {
            *self.inst_mut(id)? = Inst::alt(child.begin, 0);
            PatchList::single((id << 1) | 1)
        };
        let end = self.append(end, child.end)?;
        Ok(Frag::new(id, end, true))
    }
    fn byte_range(&mut self, lo: u8, hi: u8, foldcase: bool) -> Result<Frag, Status> {
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        *self.inst_mut(id)? = Inst::byte_range(lo, hi, foldcase, 0);
        Ok(Frag::new(id, PatchList::single(id << 1), false))
    }
    fn nop(&mut self) -> Result<Frag, Status> {
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        *self.inst_mut(id)? = Inst::nop();
        Ok(Frag::new(id, PatchList::single(id << 1), true))
    }
    fn match_inst(&mut self, match_id: i32) -> Result<Frag, Status> {
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        *self.inst_mut(id)? = Inst::matched(match_id);
        Ok(Frag::new(id, PatchList::default(), false))
    }
    fn empty_width(&mut self, empty: u32) -> Result<Frag, Status> {
        let Some(id) = self.alloc_inst(1)? else {
            return Ok(Frag::default());
        };
        *self.inst_mut(id)? = Inst::empty_width(empty, 0);
        Ok(Frag::new(id, PatchList::single(id << 1), true))
    }
    fn capture(&mut self, child: Frag, index: u32) -> Result<Frag, Status> {
        if child.begin == 0 {
            return Ok(Frag::default());
        }
        let Some(id) = self.alloc_inst(2)? else {
            return Ok(Frag::default());
        };
        let left = index
            .checked_mul(2)
            .filter(|&value| value < i32::MAX as u32)
            .ok_or_else(internal)?;
        *self.inst_mut(id)? = Inst::capture(left as i32, child.begin);
        *self.inst_mut(id + 1)? = Inst::capture((left + 1) as i32, 0);
        self.patch(child.end, id + 1)?;
        Ok(Frag::new(
            id,
            PatchList::single((id + 1) << 1),
            child.nullable,
        ))
    }
}

/// Compile an already Native-simplified root using the actual default forward
/// budget. Setup uses the ORIGINAL root flags, as Compile() does before Simplify.
/// Caller must retain the original root for later prefix-acceleration analysis.
/// Here original_root means CompileToProg's argument: the constructor-selected
/// suffix if RequiredPrefix succeeded, not necessarily the entire parsed tree.
#[cfg(test)]
pub(crate) fn compile_forward_simplified(
    arena: &mut Arena,
    original_root: Id,
    simplified_root: Id,
    layout: NativeLayout,
) -> Result<FrontEnd, Status> {
    compile_forward_with_budget(
        arena,
        original_root,
        simplified_root,
        layout,
        DEFAULT_FORWARD_MAX_MEM,
    )
}

#[cfg(test)]
pub(crate) fn compile_forward_with_budget(
    arena: &mut Arena,
    original_root: Id,
    simplified_root: Id,
    layout: NativeLayout,
    max_mem: i64,
) -> Result<FrontEnd, Status> {
    let limits = Limits::from_native_layout(max_mem, layout)?;
    compile_with_limits(arena, original_root, simplified_root, limits)
}

/// Exact default forward compilation using a source-defined effective cap,
/// preserving the full AllocInst/failure latch and 2*cap traversal budget.
/// No NativeLayout, sizeof estimate or DFA-memory metadata is invented.
pub(crate) fn compile_forward_with_instruction_cap(
    arena: &mut Arena,
    original_root: Id,
    simplified_root: Id,
    instruction_cap: u32,
) -> Result<FrontEnd, Status> {
    let limits = Limits::from_instruction_cap(instruction_cap)?;
    compile_with_limits(arena, original_root, simplified_root, limits)
}

fn compile_with_limits(
    arena: &mut Arena,
    original_root: Id,
    simplified_root: Id,
    limits: Limits,
) -> Result<FrontEnd, Status> {
    let mut compiler = Compiler::new(arena.node(original_root)?.flags, limits)?;
    let (start_anchor, root) = anchors::remove(arena, simplified_root, false)?;
    let (end_anchor, root) = anchors::remove(arena, root, true)?;
    let mut all = compiler.walk(arena, root)?;
    if compiler.failed {
        return Err(allocation());
    }
    let matched = compiler.match_inst(0)?;
    all = compiler.cat(all, matched)?;
    let start = all.begin;
    if !start_anchor {
        let byte = compiler.byte_range(0, 255, false)?;
        let dot_star = compiler.star(byte, true)?;
        all = compiler.cat(dot_star, all)?;
    }
    let start_unanchored = all.begin;
    // Finish checks failure before trimming an impossible graph.
    if compiler.failed {
        return Err(allocation());
    }
    if start == 0 && start_unanchored == 0 {
        compiler.used = 1;
    }
    let native_array_slots_before_finalize = compiler.slots.len();
    compiler.slots.truncate(compiler.used as usize);
    Ok(FrontEnd {
        graph: InstructionGraph {
            instructions: compiler.slots,
            start,
            start_unanchored,
            anchor_start: start_anchor,
            anchor_end: end_anchor,
            optimized: false,
        },
        limits,
        original_root,
        supplied_simplified_root: simplified_root,
        anchor_stripped_root: root,
        walk_visits: compiler.walk_visits,
        walk_remaining_budget: compiler.walk_remaining_budget,
        native_array_slots_before_finalize,
    })
}
