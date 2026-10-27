// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Pinned prog.cc557-927, ordered roots/lists/hints. BSD: ../RE2-LICENSE.
use super::{Inst, InstOp, InstructionGraph, Status, allocation, filled, inst, internal, push};

const ABSENT: u32 = u32::MAX;

struct Roots {
    positions: Vec<u32>,
    ordered: Vec<u32>,
}
impl Roots {
    fn new(size: usize) -> Result<Self, Status> {
        let mut ordered = Vec::new();
        ordered.try_reserve_exact(size).map_err(|_| allocation())?;
        Ok(Self {
            positions: filled(size, ABSENT)?,
            ordered,
        })
    }
    fn get(&self, id: u32) -> Result<Option<u32>, Status> {
        let value = *self.positions.get(id as usize).ok_or_else(internal)?;
        Ok((value != ABSENT).then_some(value))
    }
    fn existing(&self, id: u32) -> Result<u32, Status> {
        self.get(id)?.ok_or_else(internal)
    }
    fn insert(&mut self, id: u32) -> Result<u32, Status> {
        if let Some(value) = self.get(id)? {
            return Ok(value);
        }
        let value = u32::try_from(self.ordered.len()).map_err(|_| internal())?;
        push(&mut self.ordered, id)?;
        *self.positions.get_mut(id as usize).ok_or_else(internal)? = value;
        Ok(value)
    }
}

// SparseSet's insertion-ordered reachable set and reusable explicit stack.
// Epoch marks make clear O(1), retaining Native's behavior on many list roots.
struct Scratch {
    marks: Vec<u32>,
    epoch: u32,
    ordered: Vec<u32>,
    stack: Vec<u32>,
}
impl Scratch {
    fn new(size: usize) -> Result<Self, Status> {
        let mut ordered = Vec::new();
        ordered.try_reserve_exact(size).map_err(|_| allocation())?;
        let mut stack = Vec::new();
        stack.try_reserve_exact(size).map_err(|_| allocation())?;
        Ok(Self {
            marks: filled(size, 0)?,
            epoch: 1,
            ordered,
            stack,
        })
    }
    fn clear(&mut self) {
        self.epoch = self.epoch.wrapping_add(1);
        if self.epoch == 0 {
            self.marks.fill(0);
            self.epoch = 1;
        }
        self.ordered.clear();
        self.stack.clear();
    }
    fn contains(&self, id: u32) -> Result<bool, Status> {
        Ok(*self.marks.get(id as usize).ok_or_else(internal)? == self.epoch)
    }
    fn visit(&mut self, id: u32) -> Result<bool, Status> {
        if self.contains(id)? {
            return Ok(false);
        }
        *self.marks.get_mut(id as usize).ok_or_else(internal)? = self.epoch;
        push(&mut self.ordered, id)?;
        Ok(true)
    }
}

pub(super) struct Lists {
    pub instructions: Vec<Inst>,
    pub start: u32,
    pub start_unanchored: u32,
    pub list_count: u32,
    pub inst_count: [u32; 8],
    pub list_heads: Option<Vec<u16>>,
}

pub(super) fn flatten(graph: &InstructionGraph) -> Result<Lists, Status> {
    if !graph.optimized {
        return Err(internal());
    }
    let size = graph.instructions.len();
    let mut scratch = Scratch::new(size)?;
    let mut roots = Roots::new(size)?;
    let mut predecessors: Vec<Vec<u32>> = Vec::new();
    let mut predecessor_map = filled(size, ABSENT)?;
    mark_successors(
        graph,
        &mut roots,
        &mut predecessor_map,
        &mut predecessors,
        &mut scratch,
    )?;

    // Sort a COPY of initial successor roots. Newly added dominator roots
    // retain original rootmap insertion order and are not added to this pass.
    let mut sorted = Vec::new();
    sorted
        .try_reserve_exact(roots.ordered.len())
        .map_err(|_| allocation())?;
    sorted.extend_from_slice(&roots.ordered);
    sorted.sort_unstable();
    for &root in sorted.iter().skip(1).rev() {
        if root != graph.start_unanchored && root != graph.start {
            mark_dominator(
                graph,
                root,
                &mut roots,
                &predecessor_map,
                &predecessors,
                &mut scratch,
            )?;
        }
    }

    let mut flatmap = filled(roots.ordered.len(), 0_u32)?;
    let mut flat = Vec::new();
    flat.try_reserve(size).map_err(|_| allocation())?;
    for (root_number, &root) in roots.ordered.iter().enumerate() {
        let begin = flat.len();
        flatmap[root_number] = u32::try_from(begin).map_err(|_| internal())?;
        emit_list(graph, root, &roots, &mut flat, &mut scratch)?;
        if flat.len() == begin {
            return Err(internal());
        }
        flat.last_mut().ok_or_else(internal)?.out_opcode |= 8;
        let end = flat.len();
        compute_hints(&mut flat, begin, end)?;
    }
    let list_count = u32::try_from(flatmap.len()).map_err(|_| internal())?;
    let mut inst_count = [0_u32; 8];
    for instruction in &mut flat {
        if instruction.opcode() != InstOp::AltMatch {
            instruction.set_out(
                *flatmap
                    .get(instruction.out() as usize)
                    .ok_or_else(internal)?,
            );
        }
        let count = &mut inst_count[instruction.opcode() as usize];
        *count = count.checked_add(1).ok_or_else(internal)?;
    }
    let (start_unanchored, start) = if graph.start_unanchored == 0 {
        if graph.start != 0 {
            return Err(internal());
        }
        (0, 0)
    } else if graph.start_unanchored == graph.start {
        let start = *flatmap.get(1).ok_or_else(internal)?;
        (start, start)
    } else {
        (
            *flatmap.get(1).ok_or_else(internal)?,
            *flatmap.get(2).ok_or_else(internal)?,
        )
    };
    let list_heads = if flat.len() <= 512 {
        let mut heads = filled(flat.len(), u16::MAX)?;
        for (number, &head) in flatmap.iter().enumerate() {
            *heads.get_mut(head as usize).ok_or_else(internal)? =
                u16::try_from(number).map_err(|_| internal())?;
        }
        Some(heads)
    } else {
        None
    };
    if list_count == 0 {
        return Err(internal());
    }
    Ok(Lists {
        instructions: flat,
        start,
        start_unanchored,
        list_count,
        inst_count,
        list_heads,
    })
}

fn mark_successors(
    graph: &InstructionGraph,
    roots: &mut Roots,
    predmap: &mut [u32],
    predvec: &mut Vec<Vec<u32>>,
    scratch: &mut Scratch,
) -> Result<(), Status> {
    roots.insert(0)?;
    roots.insert(graph.start_unanchored)?;
    roots.insert(graph.start)?;
    scratch.clear();
    push(&mut scratch.stack, graph.start_unanchored)?;
    while let Some(mut id) = scratch.stack.pop() {
        loop {
            if !scratch.visit(id)? {
                break;
            }
            let instruction = inst(&graph.instructions, id)?;
            match instruction.opcode() {
                InstOp::Alt | InstOp::AltMatch => {
                    for out in [instruction.out(), instruction.payload] {
                        let value = predmap.get_mut(out as usize).ok_or_else(internal)?;
                        if *value == ABSENT {
                            *value = u32::try_from(predvec.len()).map_err(|_| internal())?;
                            push(predvec, Vec::new())?;
                        }
                        push(predvec.get_mut(*value as usize).ok_or_else(internal)?, id)?;
                    }
                    push(&mut scratch.stack, instruction.payload)?;
                    id = instruction.out();
                }
                InstOp::ByteRange | InstOp::Capture | InstOp::EmptyWidth => {
                    roots.insert(instruction.out())?;
                    id = instruction.out();
                }
                InstOp::Nop => id = instruction.out(),
                InstOp::Match | InstOp::Fail => break,
            }
        }
    }
    Ok(())
}

fn mark_dominator(
    graph: &InstructionGraph,
    root: u32,
    roots: &mut Roots,
    predmap: &[u32],
    predvec: &[Vec<u32>],
    scratch: &mut Scratch,
) -> Result<(), Status> {
    scratch.clear();
    push(&mut scratch.stack, root)?;
    while let Some(mut id) = scratch.stack.pop() {
        loop {
            if !scratch.visit(id)? {
                break;
            }
            if id != root && roots.get(id)?.is_some() {
                break;
            }
            let instruction = inst(&graph.instructions, id)?;
            match instruction.opcode() {
                InstOp::Alt | InstOp::AltMatch => {
                    push(&mut scratch.stack, instruction.payload)?;
                    id = instruction.out();
                }
                InstOp::Nop => id = instruction.out(),
                _ => break,
            }
        }
    }
    for &id in &scratch.ordered {
        let at = *predmap.get(id as usize).ok_or_else(internal)?;
        if at != ABSENT {
            for &predecessor in predvec.get(at as usize).ok_or_else(internal)? {
                if !scratch.contains(predecessor)? {
                    roots.insert(id)?;
                }
            }
        }
    }
    Ok(())
}

fn emit_list(
    graph: &InstructionGraph,
    root: u32,
    roots: &Roots,
    flat: &mut Vec<Inst>,
    scratch: &mut Scratch,
) -> Result<(), Status> {
    scratch.clear();
    push(&mut scratch.stack, root)?;
    while let Some(mut id) = scratch.stack.pop() {
        loop {
            if !scratch.visit(id)? {
                break;
            }
            if id != root && roots.get(id)?.is_some() {
                let mut nop = Inst::nop();
                nop.set_out(roots.existing(id)?);
                push(flat, nop)?;
                break;
            }
            let mut instruction = inst(&graph.instructions, id)?;
            match instruction.opcode() {
                InstOp::AltMatch | InstOp::Alt => {
                    if instruction.opcode() == InstOp::AltMatch {
                        let next = u32::try_from(flat.len())
                            .map_err(|_| internal())?
                            .checked_add(1)
                            .ok_or_else(internal)?;
                        let mut alt = Inst::default();
                        alt.set_opcode(InstOp::AltMatch);
                        alt.set_out(next);
                        alt.payload = next.checked_add(1).ok_or_else(internal)?;
                        push(flat, alt)?;
                    }
                    push(&mut scratch.stack, instruction.payload)?;
                    id = instruction.out();
                }
                InstOp::ByteRange | InstOp::Capture | InstOp::EmptyWidth => {
                    instruction.set_out(roots.existing(instruction.out())?);
                    push(flat, instruction)?;
                    break;
                }
                InstOp::Nop => id = instruction.out(),
                InstOp::Match | InstOp::Fail => {
                    push(flat, instruction)?;
                    break;
                }
            }
        }
    }
    Ok(())
}

fn next_split(splits: &[bool; 256], begin: usize) -> Result<usize, Status> {
    (begin..256).find(|&at| splits[at]).ok_or_else(internal)
}
fn recolor_hint(
    splits: &mut [bool; 256],
    colors: &mut [usize; 256],
    lo: u8,
    hi: u8,
    id: usize,
    first: &mut usize,
) -> Result<(), Status> {
    if lo > hi {
        return Err(internal());
    }
    if lo > 0 {
        let low = usize::from(lo) - 1;
        if !splits[low] {
            splits[low] = true;
            colors[low] = colors[next_split(splits, low + 1)?];
        }
    }
    let high = usize::from(hi);
    if !splits[high] {
        splits[high] = true;
        colors[high] = colors[next_split(splits, high + 1)?];
    }
    let mut current = usize::from(lo);
    loop {
        let next = next_split(splits, current)?;
        *first = (*first).min(colors[next]);
        colors[next] = id;
        if next == high {
            break;
        }
        current = next + 1;
    }
    Ok(())
}
fn compute_hints(flat: &mut [Inst], begin: usize, end: usize) -> Result<(), Status> {
    let mut splits = [false; 256];
    let mut colors = [0_usize; 256];
    let mut dirty = false;
    for id in (begin..=end).rev() {
        if id == end || flat.get(id).ok_or_else(internal)?.opcode() != InstOp::ByteRange {
            if dirty {
                dirty = false;
                splits.fill(false);
            }
            splits[255] = true;
            colors[255] = id;
            continue;
        }
        dirty = true;
        let instruction = *flat.get(id).ok_or_else(internal)?;
        let mut first = end;
        recolor_hint(
            &mut splits,
            &mut colors,
            instruction.lo(),
            instruction.hi(),
            id,
            &mut first,
        )?;
        if instruction.foldcase() && instruction.lo() <= b'z' && instruction.hi() >= b'a' {
            let lo = instruction.lo().max(b'a');
            let hi = instruction.hi().min(b'z');
            if lo <= hi {
                recolor_hint(&mut splits, &mut colors, lo - 32, hi - 32, id, &mut first)?;
            }
        }
        if first != end {
            let hint = first.checked_sub(id).ok_or_else(internal)?.min(32767) as u16;
            flat.get_mut(id).ok_or_else(internal)?.payload |= u32::from(hint) << 17;
        }
    }
    Ok(())
}
