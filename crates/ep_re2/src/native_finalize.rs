// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Rust port of pinned RE2 Prog finalization. BSD license: RE2-LICENSE.
//! Exact flattened-list/bytemap stage; prefix accelerator remains a later stage.
#[cfg(test)]
use crate::native_compiler::NativeLayout;
use crate::native_compiler::{FrontEnd, Inst, InstOp, InstructionGraph};
use crate::{ErrorCode, Status};

mod bytemap;
mod flatten;

/// Actual sizeof scalars from the SAME pinned Native observer/toolchain.
/// No default, inferred Rust layout, or guessed sizeof(Prog) is provided.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FinalizeLayout {
    pub native: NativeLayout,
    pub list_head_bytes: u64,
    pub size_t_bytes: u8,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ListsAndByteMap {
    pub instructions: Vec<Inst>,
    pub start: u32,
    pub start_unanchored: u32,
    pub anchor_start: bool,
    pub anchor_end: bool,
    pub reversed: bool,
    pub list_count: u32,
    pub inst_count: [u32; 8],
    pub list_heads: Option<Vec<u16>>,
    pub bit_state_text_max_size: u64,
    pub bytemap_range: u32,
    pub bytemap: [u8; 256],
    /// Exact compile.cc1183-1194 scalar, before constructor IsOnePass.
    pub dfa_mem: i64,
    pub layout: FinalizeLayout,
    pub max_mem: i64,
    pub original_compile_input_root: crate::native_ast::Id,
}

/// Immutable boolean FullMatch image. Target accounting is supplied separately
/// by native_target; no Rust struct size substitutes for a Native scalar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ExecutableProgram {
    pub instructions: Vec<Inst>,
    pub finalized_instructions: usize,
    pub start: u32,
    pub start_unanchored: u32,
    pub anchor_start: bool,
    pub anchor_end: bool,
    pub list_count: u32,
    pub list_heads_present: bool,
    pub inst_count: [u32; 8],
    pub bytemap_range: u32,
    pub bytemap: [u8; 256],
    /// None before the constructor's eager call; Some retains true or false.
    pub onepass: Option<crate::native_onepass::OnePassAnalysis>,
}

#[cfg(test)]
impl ListsAndByteMap {
    /// Exact public Prog::Inst::greedy predicate on finalized list instructions.
    pub(crate) fn alt_match_greedy(&self, id: u32) -> Result<bool, Status> {
        let alt = inst(&self.instructions, id)?;
        if alt.opcode() != InstOp::AltMatch {
            return Err(internal());
        }
        let out = inst(&self.instructions, alt.out())?;
        Ok(out.opcode() == InstOp::ByteRange
            || (out.opcode() == InstOp::Nop
                && inst(&self.instructions, out.out())?.opcode() == InstOp::ByteRange))
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
fn filled<T: Clone>(count: usize, value: T) -> Result<Vec<T>, Status> {
    let mut values = Vec::new();
    values.try_reserve_exact(count).map_err(|_| allocation())?;
    values.resize(count, value);
    Ok(values)
}
fn inst(instructions: &[Inst], id: u32) -> Result<Inst, Status> {
    instructions.get(id as usize).copied().ok_or_else(internal)
}

/// Consume the sealed pre-Optimize front end. Apply Optimize once, Flatten,
/// ComputeByteMap, then the exact remaining-DFA-memory formula. Prefix accel
/// configuration is EXPLICITLY absent: this is not a completed constructor or
/// execution engine and exposes no invented can_prefix_accel=false field.
#[cfg(test)]
pub(crate) fn finalize(
    mut front: FrontEnd,
    layout: FinalizeLayout,
) -> Result<ListsAndByteMap, Status> {
    if Some(layout.native) != front.limits.layout
        || layout.native.prog_bytes == 0
        || layout.native.inst_bytes != 8
        || layout.list_head_bytes != 2
        || !matches!(layout.size_t_bytes, 4 | 8)
    {
        return Err(internal());
    }
    let (lists, bytemap, bytemap_range) = finalize_lists(&mut front.graph)?;
    let bit_state_text_max_size = bit_state_limit(lists.list_count, layout.size_t_bytes)?;
    let dfa_mem = remaining_dfa_mem(
        front.limits.max_mem,
        lists.instructions.len(),
        lists.list_heads.is_some(),
        layout,
    )?;
    Ok(ListsAndByteMap {
        instructions: lists.instructions,
        start: lists.start,
        start_unanchored: lists.start_unanchored,
        anchor_start: front.graph.anchor_start,
        anchor_end: front.graph.anchor_end,
        reversed: false,
        list_count: lists.list_count,
        inst_count: lists.inst_count,
        list_heads: lists.list_heads,
        bit_state_text_max_size,
        bytemap_range,
        bytemap,
        dfa_mem,
        layout,
        max_mem: front.limits.max_mem,
        original_compile_input_root: front.original_root,
    })
}

/// Share the exact graph passes with metadata finalization, while requiring no
/// unobserved sizeof or size_t width. Both Some(unit ABI) and None(official cap)
/// front ends may produce this private boolean-only image.
pub(crate) fn finalize_executable(mut front: FrontEnd) -> Result<ExecutableProgram, Status> {
    let (lists, bytemap, bytemap_range) = finalize_lists(&mut front.graph)?;
    Ok(ExecutableProgram {
        finalized_instructions: lists.instructions.len(),
        instructions: lists.instructions,
        start: lists.start,
        start_unanchored: lists.start_unanchored,
        anchor_start: front.graph.anchor_start,
        anchor_end: front.graph.anchor_end,
        list_count: lists.list_count,
        list_heads_present: lists.list_heads.is_some(),
        inst_count: lists.inst_count,
        bytemap_range,
        bytemap,
        onepass: None,
    })
}

/// Native Finish's graph pipeline has no dependency on C++ object size or
/// size_t. Scalar ABI/accounting work is reserved for the explicit layout path.
fn finalize_lists(
    graph: &mut InstructionGraph,
) -> Result<(flatten::Lists, [u8; 256], u32), Status> {
    if graph.optimized {
        return Err(internal());
    }
    graph.optimize()?;
    let lists = flatten::flatten(graph)?;
    let (bytemap, bytemap_range) = bytemap::compute(&lists.instructions)?;
    Ok((lists, bytemap, bytemap_range))
}

/// Native size_t division followed by unsigned -1, even if BitState is off.
/// Called only when the same-build Native width was explicitly supplied.
#[cfg(test)]
fn bit_state_limit(list_count: u32, size_t_bytes: u8) -> Result<u64, Status> {
    if list_count == 0 || !matches!(size_t_bytes, 4 | 8) {
        return Err(internal());
    }
    let quotient = 256 * 1024 / u64::from(list_count);
    Ok(if size_t_bytes == 8 {
        quotient.wrapping_sub(1)
    } else {
        u64::from((quotient as u32).wrapping_sub(1))
    })
}

/// Finalization does NOT reapply max_ninst or reject when flattened size has
/// grown. Native instead subtracts the actual flat storage and clamps DFA mem.
#[cfg(test)]
fn remaining_dfa_mem(
    max_mem: i64,
    size: usize,
    bit_state: bool,
    layout: FinalizeLayout,
) -> Result<i64, Status> {
    if max_mem <= 0 {
        return Ok(1 << 20);
    }
    let size = i128::try_from(size).map_err(|_| internal())?;
    let mut remaining = i128::from(max_mem) - i128::from(layout.native.prog_bytes);
    remaining -= size * i128::from(layout.native.inst_bytes);
    if bit_state {
        remaining -= size * i128::from(layout.list_head_bytes);
    }
    i64::try_from(remaining.max(0)).map_err(|_| internal())
}
