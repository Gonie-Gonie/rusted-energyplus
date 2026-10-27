// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Pinned compile.cc450-787 UTF8 suffix cache/trie. BSD: ../RE2-LICENSE.
use super::{Compiler, Frag, Inst, InstOp, PatchList, Status, allocation, internal, push};

fn key(lo: u8, hi: u8, fold: bool, next: u32) -> u64 {
    (u64::from(next) << 17) | (u64::from(lo) << 9) | (u64::from(hi) << 1) | u64::from(fold)
}
impl Compiler {
    pub(super) fn begin_range(&mut self) {
        self.rune_cache.clear();
        self.rune_range.begin = 0;
        self.rune_range.end = PatchList::default();
    }
    pub(super) fn end_range(&self) -> Frag {
        self.rune_range
    }
    fn uncached_suffix(&mut self, lo: u8, hi: u8, fold: bool, next: u32) -> Result<u32, Status> {
        let fragment = self.byte_range(lo, hi, fold)?;
        if next != 0 {
            self.patch(fragment.end, next)?;
        } else {
            self.rune_range.end = self.append(self.rune_range.end, fragment.end)?;
        }
        Ok(fragment.begin)
    }
    fn cached_suffix(&mut self, lo: u8, hi: u8, fold: bool, next: u32) -> Result<u32, Status> {
        let key = key(lo, hi, fold, next);
        if let Some(&id) = self.rune_cache.get(&key) {
            return Ok(id);
        }
        let id = self.uncached_suffix(lo, hi, fold, next)?;
        self.rune_cache.try_reserve(1).map_err(|_| allocation())?;
        self.rune_cache.insert(key, id);
        Ok(id)
    }
    fn is_cached_suffix(&self, id: u32) -> Result<bool, Status> {
        let inst = self.inst(id)?;
        Ok(self
            .rune_cache
            .contains_key(&key(inst.lo(), inst.hi(), inst.foldcase(), inst.out())))
    }
    fn byte_range_equal(&self, a: u32, b: u32) -> Result<bool, Status> {
        let a = self.inst(a)?;
        let b = self.inst(b)?;
        Ok(a.lo() == b.lo() && a.hi() == b.hi() && a.foldcase() == b.foldcase())
    }
    fn find_byte_range(&self, root: u32, id: u32) -> Result<Frag, Status> {
        let node = self.inst(root)?;
        match node.opcode() {
            InstOp::ByteRange => {
                if self.byte_range_equal(root, id)? {
                    Ok(Frag::new(root, PatchList::default(), false))
                } else {
                    Ok(Frag::default())
                }
            }
            InstOp::Alt => {
                // Exact forward shortcut: sorted class input means only the
                // most recent out1 branch is searched. No global trie search.
                if self.byte_range_equal(node.payload, id)? {
                    Ok(Frag::new(root, PatchList::single((root << 1) | 1), false))
                } else {
                    Ok(Frag::default())
                }
            }
            _ => Err(internal()),
        }
    }
    fn add_suffix(&mut self, id: u32) -> Result<(), Status> {
        if self.failed {
            return Ok(());
        }
        if self.rune_range.begin == 0 {
            self.rune_range.begin = id;
            return Ok(());
        }
        if !self.latin1 {
            self.rune_range.begin = self.add_suffix_recursive(self.rune_range.begin, id)?;
        } else {
            let Some(alt) = self.alloc_inst(1)? else {
                self.rune_range.begin = 0;
                return Ok(());
            };
            *self.inst_mut(alt)? = Inst::alt(self.rune_range.begin, id);
            self.rune_range.begin = alt;
        }
        Ok(())
    }
    // Recursion is bounded by a Rune's at-most-four UTF8 bytes, not by user
    // regexp nesting. This preserves Native trie cloning and head reclamation.
    fn add_suffix_recursive(&mut self, mut root: u32, id: u32) -> Result<u32, Status> {
        let found = self.find_byte_range(root, id)?;
        if found.begin == 0 {
            let Some(alt) = self.alloc_inst(1)? else {
                return Ok(0);
            };
            *self.inst_mut(alt)? = Inst::alt(root, id);
            return Ok(alt);
        }
        let mut byte = if found.end.head == 0 {
            root
        } else if found.end.head & 1 != 0 {
            self.inst(found.begin)?.payload
        } else {
            self.inst(found.begin)?.out()
        };
        if self.is_cached_suffix(byte)? {
            let Some(clone) = self.alloc_inst(1)? else {
                return Ok(0);
            };
            let old = *self.inst(byte)?;
            *self.inst_mut(clone)? =
                Inst::byte_range(old.lo(), old.hi(), old.foldcase(), old.out());
            byte = clone;
            if found.end.head == 0 {
                root = byte;
            } else if found.end.head & 1 != 0 {
                self.inst_mut(found.begin)?.payload = byte;
            } else {
                self.inst_mut(found.begin)?.set_out(byte);
            }
        }
        let tail = self.inst(id)?.out();
        if !self.is_cached_suffix(id)? {
            if id != self.used - 1 {
                return Err(internal());
            }
            *self.inst_mut(id)? = Inst::default();
            self.used -= 1;
        }
        let out = self.add_suffix_recursive(self.inst(byte)?.out(), tail)?;
        if out == 0 {
            return Ok(0);
        }
        self.inst_mut(byte)?.set_out(out);
        Ok(root)
    }
    fn add_common_non_ascii(&mut self) -> Result<(), Status> {
        // compile.cc675-687 deliberately permits E0/F0 overlong encodings,
        // surrogates and F4 values above10FFFF. Retain shared continuation IDs.
        let cont1 = self.uncached_suffix(0x80, 0xbf, false, 0)?;
        let id = self.uncached_suffix(0xc2, 0xdf, false, cont1)?;
        self.add_suffix(id)?;
        let cont2 = self.uncached_suffix(0x80, 0xbf, false, cont1)?;
        let id = self.uncached_suffix(0xe0, 0xef, false, cont2)?;
        self.add_suffix(id)?;
        let cont3 = self.uncached_suffix(0x80, 0xbf, false, cont2)?;
        let id = self.uncached_suffix(0xf0, 0xf4, false, cont3)?;
        self.add_suffix(id)
    }
    pub(super) fn add_rune_range(&mut self, lo: u32, hi: u32, fold: bool) -> Result<(), Status> {
        if self.latin1 {
            if lo > hi || lo > 255 {
                return Ok(());
            }
            let id = self.uncached_suffix(lo as u8, hi.min(255) as u8, fold, 0)?;
            return self.add_suffix(id);
        }
        let mut pending = Vec::new();
        push(&mut pending, (lo, hi))?;
        while let Some((lo, hi)) = pending.pop() {
            if lo > hi {
                continue;
            }
            if lo == 0x80 && hi == 0x10ffff {
                self.add_common_non_ascii()?;
                continue;
            }
            if let Some((low, high)) = split(lo, hi)? {
                push(&mut pending, high)?;
                push(&mut pending, low)?;
                continue;
            }
            if hi < 0x80 {
                let id = self.uncached_suffix(lo as u8, hi as u8, fold, 0)?;
                self.add_suffix(id)?;
                continue;
            }
            let (low, count) = crate::byte_class::encode_rune(lo);
            let (high, high_count) = crate::byte_class::encode_rune(hi);
            if count != high_count {
                return Err(internal());
            }
            let mut id = 0;
            for index in (0..count).rev() {
                id = if index == count - 1 || (low[index] < high[index] && index != 0) {
                    self.cached_suffix(low[index], high[index], false, id)?
                } else {
                    self.uncached_suffix(low[index], high[index], false, id)?
                };
            }
            self.add_suffix(id)?;
        }
        Ok(())
    }
    pub(super) fn literal(&mut self, rune: u32, fold: bool) -> Result<Frag, Status> {
        if self.latin1 || rune < 0x80 {
            return self.byte_range(rune as u8, rune as u8, fold);
        }
        let (encoded, count) = crate::byte_class::encode_rune(rune);
        let mut result = self.byte_range(encoded[0], encoded[0], false)?;
        for &byte in &encoded[1..count] {
            let next = self.byte_range(byte, byte, false)?;
            result = self.cat(result, next)?;
        }
        Ok(result)
    }
}

type Split = Option<((u32, u32), (u32, u32))>;
fn split(lo: u32, hi: u32) -> Result<Split, Status> {
    for boundary in [0x7f, 0x7ff, 0xffff] {
        if lo <= boundary && boundary < hi {
            return Ok(Some(((lo, boundary), (boundary + 1, hi))));
        }
    }
    if hi < 0x80 {
        return Ok(None);
    }
    for count in 1..4 {
        let mask = (1_u32 << (6 * count)) - 1;
        if lo & !mask != hi & !mask {
            if lo & mask != 0 {
                let boundary = lo | mask;
                return Ok(Some((
                    (lo, boundary),
                    (boundary.checked_add(1).ok_or_else(internal)?, hi),
                )));
            }
            if hi & mask != mask {
                let boundary = hi & !mask;
                return Ok(Some((
                    (lo, boundary.checked_sub(1).ok_or_else(internal)?),
                    (boundary, hi),
                )));
            }
        }
    }
    Ok(None)
}
