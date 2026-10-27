// Copyright 2008 The RE2 Authors. All Rights Reserved.
// Rust port of pinned onepass.cc146-188,350-616. BSD: LICENSE.RE2.
//! Constructor OnePass analysis/accounting, without capture matching.
#![cfg(test)]
use crate::native_compiler::{Inst, InstOp};
use crate::{ErrorCode, Status};

const INDEX_SHIFT: u32 = 16;
const EMPTY_SHIFT: u32 = 6;
const REAL_CAP_SHIFT: u32 = EMPTY_SHIFT + 1;
const REAL_MAX_CAP: u32 = (INDEX_SHIFT - REAL_CAP_SHIFT) / 2 * 2;
const CAP_SHIFT: u32 = REAL_CAP_SHIFT - 2;
const MAX_CAP: u32 = REAL_MAX_CAP + 2;
const MATCH_WINS: u32 = 1 << EMPTY_SHIFT;
const IMPOSSIBLE: u32 = (1 << 4) | (1 << 5);

/// Observed SAME-target sizeof scalars. No Rust/default/official layout guess.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OnePassLayout {
    pub one_state_bytes: u32,
    pub uint32_bytes: u32,
}
#[derive(Clone, Copy)]
pub(crate) struct ProgramView<'a> {
    pub instructions: &'a [Inst],
    pub start: u32,
    pub inst_count: [u32; 8],
    pub bytemap: &'a [u8; 256],
    pub bytemap_range: u32,
}
/// Exact logical uint32 core fields; no fabricated target padding bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct OnePassNode {
    pub matchcond: u32,
    pub actions: Vec<u32>,
}
/// None means the source did not reach that local's initialization. Failed
/// analysis retains no OnePass image and never subtracts its scratch memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct OnePassAnalysis {
    pub is_one_pass: bool,
    pub max_nodes: Option<u32>,
    pub state_size: Option<u32>,
    pub stack_size: Option<u32>,
    pub allocated_nodes: Option<u32>,
    pub nodes: Option<Vec<OnePassNode>>,
    pub charged_bytes: i64,
    pub dfa_mem_before: i64,
    pub dfa_mem_after: i64,
}
impl OnePassAnalysis {
    fn initial(dfa_mem: i64) -> Self {
        Self {
            is_one_pass: false,
            max_nodes: None,
            state_size: None,
            stack_size: None,
            allocated_nodes: None,
            nodes: None,
            charged_bytes: 0,
            dfa_mem_before: dfa_mem,
            dfa_mem_after: dfa_mem,
        }
    }
}

/// Per-Prog did_onepass_/onepass_nodes_ retention. This owner must stay bound to
/// one immutable finalized graph and its single remaining-DFA-memory scalar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct OnePassState {
    analysis: Option<OnePassAnalysis>,
}
impl OnePassState {
    /// Cache BOTH successful and rejected analyses. Later reads do not touch
    /// dfa_mem, reproducing the first source did_onepass_ guard.
    pub(crate) fn get_or_analyze(
        &mut self,
        program: ProgramView<'_>,
        dfa_mem: &mut i64,
        layout: OnePassLayout,
    ) -> Result<&OnePassAnalysis, Status> {
        if self.analysis.is_none() {
            let analysis = analyze(program, *dfa_mem, layout)?;
            *dfa_mem = analysis.dfa_mem_after;
            self.analysis = Some(analysis);
        }
        self.analysis.as_ref().ok_or_else(internal)
    }
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
fn filled<T: Clone>(size: usize, value: T) -> Result<Vec<T>, Status> {
    let mut values = Vec::new();
    values.try_reserve_exact(size).map_err(|_| allocation())?;
    values.resize(size, value);
    Ok(values)
}
fn inst(program: ProgramView<'_>, id: u32) -> Result<Inst, Status> {
    program
        .instructions
        .get(id as usize)
        .copied()
        .ok_or_else(internal)
}
// SparseSet dense/sparse membership and first-insertion iteration order.
struct Instq {
    sparse: Vec<usize>,
    dense: Vec<u32>,
}
impl Instq {
    fn new(size: usize) -> Result<Self, Status> {
        let mut dense = Vec::new();
        dense.try_reserve_exact(size).map_err(|_| allocation())?;
        Ok(Self {
            sparse: filled(size, usize::MAX)?,
            dense,
        })
    }
    fn clear(&mut self) {
        self.dense.clear();
    }
    // AddQ pretends to insert id0, including on repeated calls.
    fn add(&mut self, id: u32) -> Result<bool, Status> {
        if id == 0 {
            return Ok(true);
        }
        let position = self.sparse.get_mut(id as usize).ok_or_else(internal)?;
        if self.dense.get(*position).copied() == Some(id) {
            return Ok(false);
        }
        *position = self.dense.len();
        self.dense.push(id); // fixed capacity at program size
        Ok(true)
    }
}
fn zero_node(classes: usize) -> Result<OnePassNode, Status> {
    Ok(OnePassNode {
        matchcond: 0,
        actions: filled(classes, 0)?,
    })
}
// Native contiguous-class skipping may advance past hi: its test is c<255.
fn actions(
    node: &mut OnePassNode,
    bytemap: &[u8; 256],
    mut byte: i32,
    hi: i32,
    action: u32,
) -> Result<bool, Status> {
    while byte <= hi {
        let class = usize::from(*bytemap.get(byte as usize).ok_or_else(internal)?);
        while byte < 255 && usize::from(bytemap[byte as usize + 1]) == class {
            byte += 1;
        }
        let old = node.actions.get_mut(class).ok_or_else(internal)?;
        if *old & IMPOSSIBLE == IMPOSSIBLE {
            *old = action;
        } else if *old != action {
            return Ok(false);
        }
        byte += 1;
    }
    Ok(true)
}
#[derive(Clone, Copy)]
struct InstCond {
    id: u32,
    cond: u32,
}
fn stack_push(stack: &mut Vec<InstCond>, size: u32, value: InstCond) -> Result<(), Status> {
    if stack.len() >= size as usize {
        return Err(internal());
    }
    stack.push(value);
    Ok(())
}

/// Invoke ONCE per compiled expression, after finalization. The constructor
/// caches this result (native did_onepass_) and uses dfa_mem_after thereafter;
/// retrieving it again must not reanalyze or subtract the charge again.
/// Ok(false) is native rejection. Err is host allocation or invalid integration
/// /undefined C++ arithmetic domain, never an invented source rejection branch.
#[expect(
    clippy::manual_is_multiple_of,
    reason = "Literal pinned OnePass divisibility predicate retained from the compiled source unit"
)]
pub(crate) fn analyze(
    program: ProgramView<'_>,
    dfa_mem: i64,
    layout: OnePassLayout,
) -> Result<OnePassAnalysis, Status> {
    let mut result = OnePassAnalysis::initial(dfa_mem);
    if program.start == 0 {
        return Ok(result);
    }
    if program.instructions.is_empty()
        || program.instructions.len() > i32::MAX as usize
        || layout.uint32_bytes != 4
        || layout.one_state_bytes < layout.uint32_bytes
        || layout.one_state_bytes % layout.uint32_bytes != 0
        || !(1..=256).contains(&program.bytemap_range)
        || program
            .bytemap
            .iter()
            .any(|&class| u32::from(class) >= program.bytemap_range)
    {
        return Err(internal());
    }
    let max_nodes = program.inst_count[InstOp::ByteRange as usize]
        .checked_add(2)
        .ok_or_else(internal)?;
    let state_size = program
        .bytemap_range
        .checked_mul(layout.uint32_bytes)
        .and_then(|bytes| bytes.checked_add(layout.one_state_bytes))
        .ok_or_else(internal)?;
    if max_nodes > i32::MAX as u32 || state_size > i32::MAX as u32 {
        return Err(internal());
    }
    result.max_nodes = Some(max_nodes);
    result.state_size = Some(state_size);
    if max_nodes >= 65_000 || dfa_mem / 4 / i64::from(state_size) < i64::from(max_nodes) {
        return Ok(result);
    }
    let stack_size = program.inst_count[InstOp::Capture as usize]
        .checked_add(program.inst_count[InstOp::EmptyWidth as usize])
        .and_then(|count| count.checked_add(program.inst_count[InstOp::Nop as usize]))
        .and_then(|count| count.checked_add(1))
        .ok_or_else(internal)?;
    if stack_size > i32::MAX as u32 {
        return Err(internal());
    }
    result.stack_size = Some(stack_size);
    let size = program.instructions.len();
    let mut node_by_id = filled(size, -1i32)?;
    let mut nodes = Vec::new();
    let mut to_visit = Instq::new(size)?;
    let mut workq = Instq::new(size)?;
    let mut stack = Vec::new();
    stack
        .try_reserve_exact(stack_size as usize)
        .map_err(|_| allocation())?;
    let classes = program.bytemap_range as usize;
    to_visit.add(program.start)?;
    *node_by_id
        .get_mut(program.start as usize)
        .ok_or_else(internal)? = 0;
    let mut allocated = 1u32;
    result.allocated_nodes = Some(allocated);
    push(&mut nodes, zero_node(classes)?)?;
    let mut visit = 0;
    while visit < to_visit.dense.len() {
        let root = to_visit.dense[visit];
        visit += 1;
        let node_index = usize::try_from(*node_by_id.get(root as usize).ok_or_else(internal)?)
            .map_err(|_| internal())?;
        let node = nodes.get_mut(node_index).ok_or_else(internal)?;
        node.actions.fill(IMPOSSIBLE);
        node.matchcond = IMPOSSIBLE;
        workq.clear();
        let mut matched = false;
        stack.clear();
        stack_push(&mut stack, stack_size, InstCond { id: root, cond: 0 })?;
        while let Some(InstCond { mut id, mut cond }) = stack.pop() {
            // Native Loop label. The initial stack id is NOT added to workq.
            loop {
                let ip = inst(program, id)?;
                match ip.opcode() {
                    InstOp::AltMatch => {
                        if ip.last() {
                            return Err(internal());
                        }
                        id = id.checked_add(1).ok_or_else(internal)?;
                        if !workq.add(id)? {
                            return Ok(result);
                        }
                    }
                    InstOp::ByteRange => {
                        let mut next_index =
                            *node_by_id.get(ip.out() as usize).ok_or_else(internal)?;
                        if next_index == -1 {
                            if allocated >= max_nodes {
                                return Ok(result);
                            }
                            next_index = i32::try_from(allocated).map_err(|_| internal())?;
                            to_visit.add(ip.out())?;
                            node_by_id[ip.out() as usize] = next_index;
                            allocated += 1;
                            result.allocated_nodes = Some(allocated);
                            push(&mut nodes, zero_node(classes)?)?;
                        }
                        let mut action = (next_index as u32) << INDEX_SHIFT | cond;
                        if matched {
                            action |= MATCH_WINS;
                        }
                        let node = nodes.get_mut(node_index).ok_or_else(internal)?;
                        if !actions(
                            node,
                            program.bytemap,
                            i32::from(ip.lo()),
                            i32::from(ip.hi()),
                            action,
                        )? {
                            return Ok(result);
                        }
                        if ip.foldcase() {
                            let lo = i32::from(ip.lo()).max(i32::from(b'a')) + i32::from(b'A')
                                - i32::from(b'a');
                            let hi = i32::from(ip.hi()).min(i32::from(b'z')) + i32::from(b'A')
                                - i32::from(b'a');
                            if !actions(node, program.bytemap, lo, hi, action)? {
                                return Ok(result);
                            }
                        }
                        if ip.last() {
                            break;
                        }
                        id = id.checked_add(1).ok_or_else(internal)?;
                        if !workq.add(id)? {
                            return Ok(result);
                        }
                    }
                    InstOp::Capture | InstOp::EmptyWidth | InstOp::Nop => {
                        if !ip.last() {
                            let following = id.checked_add(1).ok_or_else(internal)?;
                            if !workq.add(following)? {
                                return Ok(result);
                            }
                            stack_push(
                                &mut stack,
                                stack_size,
                                InstCond {
                                    id: following,
                                    cond,
                                },
                            )?;
                        }
                        if ip.opcode() == InstOp::Capture {
                            let capture = ip.payload as i32;
                            if capture < 0 {
                                return Err(internal());
                            }
                            if (capture as u32) < MAX_CAP {
                                cond |= (1 << CAP_SHIFT) << capture;
                            }
                        }
                        if ip.opcode() == InstOp::EmptyWidth {
                            cond |= ip.payload;
                        }
                        if !workq.add(ip.out())? {
                            return Ok(result);
                        }
                        id = ip.out();
                    }
                    InstOp::Match => {
                        if matched {
                            return Ok(result);
                        }
                        matched = true;
                        nodes.get_mut(node_index).ok_or_else(internal)?.matchcond = cond;
                        if ip.last() {
                            break;
                        }
                        id = id.checked_add(1).ok_or_else(internal)?;
                        if !workq.add(id)? {
                            return Ok(result);
                        }
                    }
                    InstOp::Fail => break,
                    // Unflattened Alt is an invariant error, not graph rejection.
                    InstOp::Alt => return Err(internal()),
                }
            }
        }
    }
    let charged = allocated.checked_mul(state_size).ok_or_else(internal)?;
    if charged > i32::MAX as u32 {
        return Err(internal());
    }
    result.charged_bytes = i64::from(charged);
    result.dfa_mem_after = dfa_mem
        .checked_sub(result.charged_bytes)
        .ok_or_else(internal)?;
    result.is_one_pass = true;
    result.nodes = Some(nodes);
    Ok(result)
}
