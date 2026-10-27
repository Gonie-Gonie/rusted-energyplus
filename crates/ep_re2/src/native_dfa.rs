// Copyright 2008 The RE2 Authors. All Rights Reserved.
// Rust port of pinned RE2 dfa.cc; BSD license: RE2-LICENSE.
//! Forward anchored longest-DFA subset for no-capture FullMatch.
//! Persistent cache state is owned; immutable program views are call-scoped.
use crate::native_compiler::{Inst, InstOp};
use std::collections::HashMap;

const BEGIN_LINE: u32 = 1;
const END_LINE: u32 = 2;
const BEGIN_TEXT: u32 = 4;
const END_TEXT: u32 = 8;
const WORD_BOUNDARY: u32 = 16;
const NON_WORD_BOUNDARY: u32 = 32;
const EMPTY_MASK: u32 = 0xff;
const MATCH: u32 = 0x100;
const LAST_WORD: u32 = 0x200;
const NEED_SHIFT: u32 = 16;
const END_BYTE: u16 = 256;
const MARK: i32 = -1;
const CACHE_OVERHEAD: i128 = 40; // dfa.cc751 source constant, NOT target sizeof.

/// Observations from the SAME target build; there are no Rust-size defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DfaLayout {
    pub dfa_bytes: u64,
    pub state_bytes: u64,
    pub atomic_state_pointer_bytes: u64,
    pub int_bytes: u64,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct ProgramView<'a> {
    pub instructions: &'a [Inst],
    pub start: u32,
    pub start_unanchored: u32,
    pub anchor_start: bool,
    pub anchor_end: bool,
    pub list_count: u32,
    pub inst_count: [u32; 8],
    pub bytemap_range: u32,
    pub bytemap: &'a [u8; 256],
}
/// Host/invariant guards remain separate from Native's ordinary DFA failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DfaError {
    InvalidLayout,
    InvalidProgram,
    InvalidRange,
    AccountingOverflow,
    HostAllocation,
    SourceInvariant,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SearchResult {
    pub matched: bool,
    pub failed: bool,
    /// Absolute byte offset in original context, not the prefix-stripped slice.
    pub match_end: Option<usize>,
}
impl SearchResult {
    fn failed() -> Self {
        Self {
            matched: false,
            failed: true,
            match_end: None,
        }
    }
    fn ordinary(matched: bool, match_end: Option<usize>) -> Self {
        Self {
            matched,
            failed: false,
            match_end,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CacheFacts {
    pub init_failed: bool,
    #[cfg(test)]
    pub longest_max_mem: i64,
    /// None iff the source returned BEFORE assigning state_budget_.
    pub state_budget: Option<i64>,
    pub remaining_mem: i64,
    #[cfg(test)]
    pub cached_states: usize,
    #[cfg(test)]
    pub cache_resets: u64,
    #[cfg(test)]
    pub state_computations: u64,
    #[cfg(test)]
    pub slow_bailouts: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StateId {
    Dead,
    Full,
    Live(usize),
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    inst: Vec<i32>,
    flag: u32,
}
struct State {
    key: Key,
    next: Vec<Option<StateId>>,
}
enum Saved {
    Special(StateId),
    Ordinary(Key),
}

// dfa.cc371-421: insertion order, membership, and actual synthetic mark slots.
struct Workq {
    dense: Vec<usize>,
    seen: Vec<bool>,
    n: usize,
    next_mark: usize,
    last_was_mark: bool,
}
impl Workq {
    fn new(n: usize) -> Result<Self, DfaError> {
        let size = n.checked_mul(2).ok_or(DfaError::AccountingOverflow)?;
        let mut dense = Vec::new();
        dense
            .try_reserve_exact(size)
            .map_err(|_| DfaError::HostAllocation)?;
        Ok(Self {
            dense,
            seen: filled(size, false)?,
            n,
            next_mark: n,
            last_was_mark: true,
        })
    }
    fn clear(&mut self) {
        for &id in &self.dense {
            self.seen[id] = false;
        }
        self.dense.clear();
        self.next_mark = self.n;
        // Source clear() does NOT reset last_was_mark_.
    }
    fn is_mark(&self, id: usize) -> bool {
        id >= self.n
    }
    fn insert_new(&mut self, id: usize) -> Result<(), DfaError> {
        let seen = self.seen.get_mut(id).ok_or(DfaError::SourceInvariant)?;
        if *seen {
            return Err(DfaError::SourceInvariant);
        }
        *seen = true;
        self.last_was_mark = false;
        self.dense.push(id); // Full sparse-set capacity already reserved.
        Ok(())
    }
    fn mark(&mut self) -> Result<(), DfaError> {
        if self.last_was_mark {
            return Ok(());
        }
        self.last_was_mark = false; // Deliberately actual pinned spelling.
        let id = self.next_mark;
        self.next_mark = self
            .next_mark
            .checked_add(1)
            .ok_or(DfaError::AccountingOverflow)?;
        self.insert_new(id)
    }
}
struct Scratch {
    q0: Workq,
    q1: Workq,
    stack: Vec<i32>,
    stack_limit: usize,
}

/// Persistent kLongestMatch cache. Construct lazily at source GetDFA, after
/// owner prefix/anchor/OnePass-skip checks. Sequential calls retain this cache.
/// The shared Re2 owner serializes access; there is no retained graph borrow.
#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct ObserverResetEvent {
    pub state_budget: i64,
    pub state_cache_size: usize,
}
pub(crate) struct LongestDfa {
    layout: DfaLayout,
    scratch: Option<Scratch>,
    states: Vec<State>,
    cache: HashMap<Key, usize>,
    /// Anchored forward StartInfos (source start_ indices 1,3,5,7).
    starts: [Option<StateId>; 4],
    facts: CacheFacts,
    #[cfg(test)]
    observer_events: Vec<ObserverResetEvent>,
}
fn filled<T: Clone>(n: usize, value: T) -> Result<Vec<T>, DfaError> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| DfaError::HostAllocation)?;
    v.resize(n, value);
    Ok(v)
}
fn copied<T: Copy>(values: &[T]) -> Result<Vec<T>, DfaError> {
    let mut v = Vec::new();
    v.try_reserve_exact(values.len())
        .map_err(|_| DfaError::HostAllocation)?;
    v.extend_from_slice(values);
    Ok(v)
}
fn word(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}
fn instruction(p: ProgramView<'_>, id: usize) -> Result<Inst, DfaError> {
    p.instructions
        .get(id)
        .copied()
        .ok_or(DfaError::InvalidProgram)
}
fn push_stack(stack: &mut Vec<i32>, limit: usize, id: i32) -> Result<(), DfaError> {
    if stack.len() == limit {
        return Err(DfaError::SourceInvariant);
    }
    stack.push(id);
    Ok(())
}

// dfa.cc816-898. Queue ALL list members/epsilon nodes; only the later cache
// canonicalizer discards everything except list heads and flags.
fn add_to_queue(
    p: ProgramView<'_>,
    stack: &mut Vec<i32>,
    limit: usize,
    q: &mut Workq,
    id: i32,
    flags: u32,
) -> Result<(), DfaError> {
    stack.clear();
    push_stack(stack, limit, id)?;
    while let Some(mut id) = stack.pop() {
        loop {
            if id == MARK {
                q.mark()?;
                break;
            }
            if id == 0 {
                break;
            }
            let at = usize::try_from(id).map_err(|_| DfaError::InvalidProgram)?;
            if *q.seen.get(at).ok_or(DfaError::InvalidProgram)? {
                break;
            }
            q.insert_new(at)?;
            let inst = instruction(p, at)?;
            match inst.opcode() {
                InstOp::ByteRange | InstOp::Match => {
                    if inst.last() {
                        break;
                    }
                    id = id.checked_add(1).ok_or(DfaError::AccountingOverflow)?;
                }
                InstOp::Capture | InstOp::Nop => {
                    if !inst.last() {
                        push_stack(stack, limit, id + 1)?;
                    }
                    if inst.opcode() == InstOp::Nop
                        && at == p.start_unanchored as usize
                        && p.start_unanchored != p.start
                    {
                        push_stack(stack, limit, MARK)?;
                    }
                    id = inst.out() as i32;
                }
                InstOp::AltMatch => {
                    if inst.last() {
                        return Err(DfaError::SourceInvariant);
                    }
                    id = id.checked_add(1).ok_or(DfaError::AccountingOverflow)?;
                }
                InstOp::EmptyWidth => {
                    if !inst.last() {
                        push_stack(stack, limit, id + 1)?;
                    }
                    if inst.payload & !flags != 0 {
                        break;
                    }
                    id = inst.out() as i32;
                }
                // Fail at id0 was skipped. Alt cannot survive Flatten.
                InstOp::Fail | InstOp::Alt => return Err(DfaError::SourceInvariant),
            }
        }
    }
    Ok(())
}
fn run_empty(p: ProgramView<'_>, s: &mut Scratch, flags: u32) -> Result<(), DfaError> {
    s.q1.clear();
    for &id in &s.q0.dense {
        let id = if s.q0.is_mark(id) { MARK } else { id as i32 };
        add_to_queue(p, &mut s.stack, s.stack_limit, &mut s.q1, id, flags)?;
    }
    std::mem::swap(&mut s.q0, &mut s.q1);
    Ok(())
}
fn run_byte(p: ProgramView<'_>, s: &mut Scratch, byte: u16, flags: u32) -> Result<bool, DfaError> {
    s.q1.clear();
    let mut matched = false;
    let mut at = 0;
    while at < s.q0.dense.len() {
        let id = s.q0.dense[at];
        if s.q0.is_mark(id) {
            if matched {
                break;
            }
            s.q1.mark()?;
            at += 1;
            continue;
        }
        let inst = instruction(p, id)?;
        match inst.opcode() {
            InstOp::ByteRange => {
                let mut c = byte;
                if inst.foldcase() && (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
                    c += u16::from(b'a' - b'A');
                }
                if u16::from(inst.lo()) <= c && c <= u16::from(inst.hi()) {
                    add_to_queue(
                        p,
                        &mut s.stack,
                        s.stack_limit,
                        &mut s.q1,
                        inst.out() as i32,
                        flags,
                    )?;
                    // Source advances the QUEUE iterator, not an instruction ID.
                    let step = if inst.hint() != 0 {
                        usize::from(inst.hint())
                    } else {
                        let mut last = id;
                        while !instruction(p, last)?.last() {
                            last = last.checked_add(1).ok_or(DfaError::AccountingOverflow)?;
                        }
                        last - id + 1
                    };
                    at = at.checked_add(step).ok_or(DfaError::AccountingOverflow)?;
                    continue;
                }
            }
            InstOp::Match => {
                if !p.anchor_end || byte == END_BYTE {
                    matched = true;
                }
            }
            InstOp::Alt => return Err(DfaError::SourceInvariant),
            InstOp::Fail
            | InstOp::Capture
            | InstOp::Nop
            | InstOp::AltMatch
            | InstOp::EmptyWidth => {}
        }
        at += 1;
    }
    std::mem::swap(&mut s.q0, &mut s.q1);
    Ok(matched)
}

impl LongestDfa {
    /// dfa.cc423-470,1802-1806: forward longest gets half the ALREADY adjusted
    /// budget. Explicit observations supply every private Native layout scalar.
    pub(crate) fn new(
        p: ProgramView<'_>,
        post_onepass_dfa_mem: i64,
        layout: DfaLayout,
    ) -> Result<Self, DfaError> {
        if layout.dfa_bytes == 0
            || layout.state_bytes == 0
            || layout.atomic_state_pointer_bytes == 0
            || layout.int_bytes != 4
        {
            return Err(DfaError::InvalidLayout);
        }
        let n = p.instructions.len();
        if n == 0
            || n > i32::MAX as usize
            || p.start as usize >= n
            || p.start_unanchored as usize >= n
            || !(1..=256).contains(&p.bytemap_range)
            || p.bytemap.iter().any(|&c| u32::from(c) >= p.bytemap_range)
            || post_onepass_dfa_mem < 0
        {
            return Err(DfaError::InvalidProgram);
        }
        let nstack = u64::from(p.inst_count[InstOp::Capture as usize])
            + u64::from(p.inst_count[InstOp::EmptyWidth as usize])
            + u64::from(p.inst_count[InstOp::Nop as usize])
            + u64::try_from(n).map_err(|_| DfaError::AccountingOverflow)?
            + 1;
        let max_mem = post_onepass_dfa_mem / 2;
        let budget = i128::from(max_mem)
            - i128::from(layout.dfa_bytes)
            - (n as i128 * 2) * (i128::from(layout.int_bytes) * 2) * 2
            - i128::from(nstack) * i128::from(layout.int_bytes);
        let budget = i64::try_from(budget).map_err(|_| DfaError::AccountingOverflow)?;
        let one_state = i128::from(layout.state_bytes)
            + i128::from(p.bytemap_range + 1) * i128::from(layout.atomic_state_pointer_bytes)
            + (i128::from(p.list_count) + n as i128) * i128::from(layout.int_bytes);
        let init_failed = budget < 0 || i128::from(budget) < 20 * one_state;
        let scratch = if init_failed {
            None
        } else {
            let limit = usize::try_from(nstack).map_err(|_| DfaError::AccountingOverflow)?;
            let mut stack = Vec::new();
            stack
                .try_reserve_exact(limit)
                .map_err(|_| DfaError::HostAllocation)?;
            Some(Scratch {
                q0: Workq::new(n)?,
                q1: Workq::new(n)?,
                stack,
                stack_limit: limit,
            })
        };
        Ok(Self {
            layout,
            scratch,
            states: Vec::new(),
            cache: HashMap::new(),
            starts: [None; 4],
            facts: CacheFacts {
                init_failed,
                #[cfg(test)]
                longest_max_mem: max_mem,
                state_budget: (budget >= 0).then_some(budget),
                remaining_mem: budget,
                #[cfg(test)]
                cached_states: 0,
                #[cfg(test)]
                cache_resets: 0,
                #[cfg(test)]
                state_computations: 0,
                #[cfg(test)]
                slow_bailouts: 0,
            },
            #[cfg(test)]
            observer_events: Vec::new(),
        })
    }
    #[cfg(test)]
    pub(crate) fn take_observer_events(&mut self) -> Vec<ObserverResetEvent> {
        std::mem::take(&mut self.observer_events)
    }
    #[cfg(test)]
    #[expect(
        dead_code,
        reason = "Retained source cache accounting is outside the public regression wire"
    )]
    pub(crate) fn facts(&self) -> CacheFacts {
        CacheFacts {
            cached_states: self.states.len(),
            ..self.facts
        }
    }
    fn cache_state(
        &mut self,
        p: ProgramView<'_>,
        inst: Vec<i32>,
        flag: u32,
    ) -> Result<Option<StateId>, DfaError> {
        let key = Key { inst, flag };
        if let Some(&id) = self.cache.get(&key) {
            return Ok(Some(StateId::Live(id)));
        }
        let bytes = i128::from(self.layout.state_bytes)
            + i128::from(p.bytemap_range + 1) * i128::from(self.layout.atomic_state_pointer_bytes)
            + key.inst.len() as i128 * i128::from(self.layout.int_bytes)
            + CACHE_OVERHEAD;
        let bytes = i64::try_from(bytes).map_err(|_| DfaError::AccountingOverflow)?;
        if self.facts.remaining_mem < bytes {
            self.facts.remaining_mem = -1;
            return Ok(None);
        }
        self.facts.remaining_mem -= bytes;
        let next = filled(p.bytemap_range as usize + 1, None)?;
        let stored_key = Key {
            inst: copied(&key.inst)?,
            flag,
        };
        self.states
            .try_reserve(1)
            .map_err(|_| DfaError::HostAllocation)?;
        self.cache
            .try_reserve(1)
            .map_err(|_| DfaError::HostAllocation)?;
        let id = self.states.len();
        self.states.push(State {
            key: stored_key,
            next,
        });
        self.cache.insert(key, id);
        Ok(Some(StateId::Live(id)))
    }
    // dfa.cc595-725 specialized only by fixed kLongestMatch.
    fn cache_queue(
        &mut self,
        p: ProgramView<'_>,
        mut flag: u32,
    ) -> Result<Option<StateId>, DfaError> {
        let q = &self.scratch.as_ref().ok_or(DfaError::SourceInvariant)?.q0;
        let mut inst = Vec::new();
        inst.try_reserve_exact(q.seen.len())
            .map_err(|_| DfaError::HostAllocation)?;
        let mut need = 0;
        let mut saw_match = false;
        let mut saw_mark = false;
        for &id in &q.dense {
            if saw_match && q.is_mark(id) {
                break;
            }
            if q.is_mark(id) {
                if inst.last().is_some_and(|&last| last != MARK) {
                    saw_mark = true;
                    inst.push(MARK);
                }
                continue;
            }
            let ip = instruction(p, id)?;
            if ip.opcode() == InstOp::AltMatch && !saw_mark && flag & MATCH != 0 {
                return Ok(Some(StateId::Full));
            }
            if instruction(p, id.checked_sub(1).ok_or(DfaError::SourceInvariant)?)?.last() {
                inst.push(id as i32);
            }
            if ip.opcode() == InstOp::EmptyWidth {
                need |= ip.payload;
            }
            if ip.opcode() == InstOp::Match && !p.anchor_end {
                saw_match = true;
            }
        }
        if inst.last() == Some(&MARK) {
            inst.pop();
        }
        if need == 0 {
            flag &= MATCH;
        }
        if inst.is_empty() && flag == 0 {
            return Ok(Some(StateId::Dead));
        }
        for group in inst.split_mut(|&id| id == MARK) {
            group.sort_unstable();
        }
        flag |= need << NEED_SHIFT;
        self.cache_state(p, inst, flag)
    }
    fn state_to_queue(&mut self, p: ProgramView<'_>, id: usize) -> Result<(), DfaError> {
        let state = self.states.get(id).ok_or(DfaError::SourceInvariant)?;
        let s = self.scratch.as_mut().ok_or(DfaError::SourceInvariant)?;
        s.q0.clear();
        for &id in &state.key.inst {
            add_to_queue(
                p,
                &mut s.stack,
                s.stack_limit,
                &mut s.q0,
                id,
                state.key.flag & EMPTY_MASK,
            )?;
        }
        Ok(())
    }
    fn byte_class(p: ProgramView<'_>, byte: u16) -> usize {
        if byte == END_BYTE {
            p.bytemap_range as usize
        } else {
            usize::from(p.bytemap[usize::from(byte)])
        }
    }
    fn run_state(
        &mut self,
        p: ProgramView<'_>,
        id: StateId,
        byte: u16,
    ) -> Result<Option<StateId>, DfaError> {
        let StateId::Live(at) = id else {
            return if id == StateId::Full {
                Ok(Some(id))
            } else {
                Err(DfaError::SourceInvariant)
            };
        };
        let class = Self::byte_class(p, byte);
        if let Some(id) = self.states.get(at).ok_or(DfaError::SourceInvariant)?.next[class] {
            return Ok(Some(id));
        }
        #[cfg(test)]
        {
            self.facts.state_computations += 1;
        }
        let state_flag = self.states[at].key.flag;
        self.state_to_queue(p, at)?;
        let need = state_flag >> NEED_SHIFT;
        let mut before = state_flag & EMPTY_MASK;
        let old_before = before;
        let mut after = 0;
        if byte == u16::from(b'\n') {
            before |= END_LINE;
            after |= BEGIN_LINE;
        }
        if byte == END_BYTE {
            before |= END_LINE | END_TEXT;
        }
        let is_word = byte != END_BYTE && word(byte as u8);
        before |= if is_word == (state_flag & LAST_WORD != 0) {
            NON_WORD_BOUNDARY
        } else {
            WORD_BOUNDARY
        };
        let s = self.scratch.as_mut().ok_or(DfaError::SourceInvariant)?;
        if before & !old_before & need != 0 {
            run_empty(p, s, before)?;
        }
        let matched = run_byte(p, s, byte, after)?;
        let mut flag = after;
        if matched {
            flag |= MATCH;
        }
        if is_word {
            flag |= LAST_WORD;
        }
        let next = self.cache_queue(p, flag)?;
        self.states[at].next[class] = next;
        Ok(next)
    }
    fn reset(&mut self) -> Result<(), DfaError> {
        // Public Native hook precedes cache mutation (dfa.cc1170-1173).
        #[cfg(test)]
        {
            let event = ObserverResetEvent {
                state_budget: self.facts.state_budget.ok_or(DfaError::SourceInvariant)?,
                state_cache_size: self.cache.len(),
            };
            self.observer_events
                .try_reserve(1)
                .map_err(|_| DfaError::HostAllocation)?;
            self.observer_events.push(event);
        }
        self.starts.fill(None);
        self.states.clear();
        self.cache.clear();
        self.facts.remaining_mem = self.facts.state_budget.ok_or(DfaError::SourceInvariant)?;
        #[cfg(test)]
        {
            self.facts.cache_resets += 1;
        }
        Ok(())
    }
    fn save(&self, state: StateId) -> Result<Saved, DfaError> {
        match state {
            StateId::Live(id) => {
                let key = &self.states.get(id).ok_or(DfaError::SourceInvariant)?.key;
                Ok(Saved::Ordinary(Key {
                    inst: copied(&key.inst)?,
                    flag: key.flag,
                }))
            }
            special => Ok(Saved::Special(special)),
        }
    }
    fn restore(&mut self, p: ProgramView<'_>, saved: Saved) -> Result<Option<StateId>, DfaError> {
        match saved {
            Saved::Special(id) => Ok(Some(id)),
            Saved::Ordinary(key) => self.cache_state(p, key.inst, key.flag),
        }
    }
    fn analyze(
        &mut self,
        p: ProgramView<'_>,
        index: usize,
        flags: u32,
    ) -> Result<Option<StateId>, DfaError> {
        if let Some(start) = self.starts[index] {
            return Ok(Some(start));
        }
        let s = self.scratch.as_mut().ok_or(DfaError::SourceInvariant)?;
        s.q0.clear();
        add_to_queue(
            p,
            &mut s.stack,
            s.stack_limit,
            &mut s.q0,
            p.start as i32,
            flags,
        )?;
        let result = self.cache_queue(p, flags)?;
        self.starts[index] = result;
        Ok(result)
    }
    fn is_match(&self, state: StateId) -> Result<bool, DfaError> {
        match state {
            StateId::Live(id) => Ok(self
                .states
                .get(id)
                .ok_or(DfaError::SourceInvariant)?
                .key
                .flag
                & MATCH
                != 0),
            _ => Err(DfaError::SourceInvariant),
        }
    }
    /// Prog::SearchDFA(kAnchored,kFullMatch), no submatch/many-match IDs.
    /// Keep original context across required-prefix removal. Only actual source
    /// budget/reset/slow routes set failed; owner emits LOG811 and falls back.
    pub(crate) fn search_full(
        &mut self,
        program: ProgramView<'_>,
        context: &[u8],
        begin: usize,
        end: usize,
    ) -> Result<SearchResult, DfaError> {
        if begin > end || end > context.len() {
            return Err(DfaError::InvalidRange);
        }
        if (program.anchor_start && begin != 0) || (program.anchor_end && end != context.len()) {
            return Ok(SearchResult::ordinary(false, None));
        }
        if self.facts.init_failed {
            return Ok(SearchResult::failed());
        }
        let (index, flags) = if begin == 0 {
            (0, BEGIN_TEXT | BEGIN_LINE)
        } else if context[begin - 1] == b'\n' {
            (1, BEGIN_LINE)
        } else if word(context[begin - 1]) {
            (2, LAST_WORD)
        } else {
            (3, 0)
        };
        let mut start = match self.analyze(program, index, flags)? {
            Some(start) => start,
            None => {
                self.reset()?;
                let Some(start) = self.analyze(program, index, flags)? else {
                    return Ok(SearchResult::failed());
                };
                start
            }
        };
        match start {
            StateId::Dead => return Ok(SearchResult::ordinary(false, None)),
            StateId::Full => return Ok(SearchResult::ordinary(true, Some(end))),
            StateId::Live(_) => {}
        }
        let mut state = start;
        let mut p = begin;
        let mut reset_p = None;
        let mut last_match = self.is_match(state)?.then_some(p);
        while p != end {
            let byte = u16::from(context[p]);
            p += 1;
            let next = match self.run_state(program, state, byte)? {
                Some(next) => next,
                None => {
                    // dfa.cc1418: default bail_when_slow=true. First reset is
                    // allowed; p is already AFTER the current input byte.
                    if reset_p.is_some_and(|prior| p - prior < 10 * self.states.len()) {
                        #[cfg(test)]
                        {
                            self.facts.slow_bailouts += 1;
                        }
                        return Ok(SearchResult::failed());
                    }
                    reset_p = Some(p);
                    let save_start = self.save(start)?;
                    let save_state = self.save(state)?;
                    self.reset()?;
                    let Some(restored_start) = self.restore(program, save_start)? else {
                        return Ok(SearchResult::failed());
                    };
                    start = restored_start;
                    let Some(restored_state) = self.restore(program, save_state)? else {
                        return Ok(SearchResult::failed());
                    };
                    state = restored_state;
                    let Some(next) = self.run_state(program, state, byte)? else {
                        return Ok(SearchResult::failed());
                    };
                    next
                }
            };
            match next {
                StateId::Dead => {
                    return Ok(SearchResult::ordinary(last_match == Some(end), last_match));
                }
                StateId::Full => return Ok(SearchResult::ordinary(true, Some(end))),
                StateId::Live(_) => {
                    state = next;
                    if self.is_match(state)? {
                        last_match = Some(p - 1);
                    } // one-byte delayed match
                }
            }
        }
        // Final lookahead uses a real context byte if the slice ends early;
        // END_BYTE applies only at the ORIGINAL context endpoint.
        let byte = if end == context.len() {
            END_BYTE
        } else {
            u16::from(context[end])
        };
        let next = match self.run_state(program, state, byte)? {
            Some(next) => next,
            None => {
                // The final-byte reset saves s ONLY and has NO slow bailout.
                let saved = self.save(state)?;
                self.reset()?;
                let Some(restored) = self.restore(program, saved)? else {
                    return Ok(SearchResult::failed());
                };
                state = restored;
                let Some(next) = self.run_state(program, state, byte)? else {
                    return Ok(SearchResult::failed());
                };
                next
            }
        };
        match next {
            StateId::Dead => Ok(SearchResult::ordinary(last_match == Some(end), last_match)),
            StateId::Full => Ok(SearchResult::ordinary(true, Some(end))),
            StateId::Live(_) => {
                if self.is_match(next)? {
                    last_match = Some(p);
                }
                // dfa.cc1883: longest success must cover this entire text slice.
                Ok(SearchResult::ordinary(last_match == Some(end), last_match))
            }
        }
    }
}
