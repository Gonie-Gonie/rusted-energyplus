// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Pinned prog.cc321-520 ByteMapBuilder/ComputeByteMap. BSD: ../RE2-LICENSE.
use super::{Inst, InstOp, Status, internal, push};

struct Builder {
    splits: [bool; 256],
    colors: [u32; 256],
    next_color: u32,
    color_map: Vec<(u32, u32)>,
    ranges: Vec<(u8, u8)>,
}
impl Builder {
    fn new() -> Self {
        let mut splits = [false; 256];
        splits[255] = true;
        let mut colors = [0; 256];
        colors[255] = 256;
        Self {
            splits,
            colors,
            next_color: 257,
            color_map: Vec::new(),
            ranges: Vec::new(),
        }
    }
    fn next_split(&self, begin: usize) -> Result<usize, Status> {
        (begin..256)
            .find(|&at| self.splits[at])
            .ok_or_else(internal)
    }
    fn mark(&mut self, lo: u8, hi: u8) -> Result<(), Status> {
        if lo > hi {
            return Err(internal());
        }
        if lo == 0 && hi == 255 {
            return Ok(());
        }
        push(&mut self.ranges, (lo, hi))
    }
    fn recolor(&mut self, old: u32) -> Result<u32, Status> {
        // Both keys AND values participate, exactly like the primary search.
        // A range already recolored in this batch must not be recolored again.
        if let Some(&(_, value)) = self
            .color_map
            .iter()
            .find(|&&(key, value)| key == old || value == old)
        {
            return Ok(value);
        }
        let new = self.next_color;
        self.next_color = self.next_color.checked_add(1).ok_or_else(internal)?;
        push(&mut self.color_map, (old, new))?;
        Ok(new)
    }
    fn merge(&mut self) -> Result<(), Status> {
        for index in 0..self.ranges.len() {
            let (lo, hi) = self.ranges[index];
            if lo > 0 {
                let low = usize::from(lo) - 1;
                if !self.splits[low] {
                    self.splits[low] = true;
                    self.colors[low] = self.colors[self.next_split(low + 1)?];
                }
            }
            let high = usize::from(hi);
            if !self.splits[high] {
                self.splits[high] = true;
                self.colors[high] = self.colors[self.next_split(high + 1)?];
            }
            let mut current = usize::from(lo);
            loop {
                let next = self.next_split(current)?;
                self.colors[next] = self.recolor(self.colors[next])?;
                if next == high {
                    break;
                }
                current = next + 1;
            }
        }
        self.color_map.clear();
        self.ranges.clear();
        Ok(())
    }
    fn build(mut self) -> Result<([u8; 256], u32), Status> {
        self.next_color = 0;
        let mut bytemap = [0_u8; 256];
        let mut current = 0;
        while current < 256 {
            let next = self.next_split(current)?;
            let byte_class = self.recolor(self.colors[next])?;
            let byte_class = u8::try_from(byte_class).map_err(|_| internal())?;
            while current <= next {
                bytemap[current] = byte_class;
                current += 1;
            }
        }
        Ok((bytemap, self.next_color))
    }
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_uppercase() || byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
}

pub(super) fn compute(instructions: &[Inst]) -> Result<([u8; 256], u32), Status> {
    let mut builder = Builder::new();
    let mut marked_line = false;
    let mut marked_word = false;
    for (id, &instruction) in instructions.iter().enumerate() {
        if instruction.opcode() == InstOp::ByteRange {
            builder.mark(instruction.lo(), instruction.hi())?;
            if instruction.foldcase() && instruction.lo() <= b'z' && instruction.hi() >= b'a' {
                let lo = instruction.lo().max(b'a');
                let hi = instruction.hi().min(b'z');
                if lo <= hi {
                    builder.mark(lo - 32, hi - 32)?;
                }
            }
            if !instruction.last() {
                let next = instructions.get(id + 1).ok_or_else(internal)?;
                if next.opcode() == InstOp::ByteRange && instruction.out() == next.out() {
                    continue;
                }
            }
            builder.merge()?;
        } else if instruction.opcode() == InstOp::EmptyWidth {
            if instruction.payload & ((1 << 0) | (1 << 1)) != 0 && !marked_line {
                builder.mark(b'\n', b'\n')?;
                builder.merge()?;
                marked_line = true;
            }
            if instruction.payload & ((1 << 4) | (1 << 5)) != 0 && !marked_word {
                for selected_word in [true, false] {
                    let mut current: u16 = 0;
                    while current < 256 {
                        let mut next = current + 1;
                        while next < 256 && is_word(current as u8) == is_word(next as u8) {
                            next += 1;
                        }
                        if is_word(current as u8) == selected_word {
                            builder.mark(current as u8, (next - 1) as u8)?;
                        }
                        current = next;
                    }
                    builder.merge()?;
                }
                marked_word = true;
            }
        }
    }
    builder.build()
}
