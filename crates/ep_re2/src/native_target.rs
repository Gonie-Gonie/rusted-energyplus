//! Scalar accounting for the pinned official Windows EnergyPlus RE2 target.
//!
//! Values come from the actual DLL's paired allocation/subtraction and instruction
//! addressing, not Rust sizeof or the separate GCC observer ABI. Source mapping
//! and actual byte/receipt custody are retained in the official DFA binary proof:
//! proposal SHA256 19c6a962435f13b80b049191092468f9d84a2296886a9271b5c670bb44c7e461,
//! actual extracted facts SHA256 2ae2132210144bbe3571e0d1595256a4f46a2eef935a931257812f3e5609425a,
//! DLL SHA256 bbe6f66240d108df9f51a03b9beb968808a2173234253574d27a31605aec2aa2.
use crate::native_dfa::{DfaError, DfaLayout};
use crate::native_finalize::ExecutableProgram;

#[derive(Clone, Copy)]
struct TargetLayout {
    prog_bytes: u64,
    inst_bytes: u64,
    list_head_bytes: u64,
    dfa: DfaLayout,
}

const OFFICIAL: TargetLayout = TargetLayout {
    prog_bytes: 440,
    inst_bytes: 8,
    list_head_bytes: 2,
    dfa: DfaLayout {
        dfa_bytes: 208,
        state_bytes: 16,
        atomic_state_pointer_bytes: 8,
        int_bytes: 4,
    },
};

pub(crate) const DFA_LAYOUT: DfaLayout = OFFICIAL.dfa;

/// compile.cc Finish subtracts ACTUAL flat storage and optional list heads,
/// clamps at zero, then the successful eager OnePass image is charged once.
/// GetDFA's subsequent integer division by two remains in LongestDfa::new.
pub(crate) fn post_onepass_dfa_mem(program: &ExecutableProgram) -> Result<i64, DfaError> {
    if program.instructions.len() != program.finalized_instructions
        || program.list_heads_present != (program.finalized_instructions <= 512)
    {
        return Err(DfaError::InvalidProgram);
    }
    let count =
        i128::try_from(program.finalized_instructions).map_err(|_| DfaError::AccountingOverflow)?;
    let mut remaining = i128::from(crate::native_compiler::DEFAULT_FORWARD_MAX_MEM)
        - i128::from(OFFICIAL.prog_bytes)
        - count * i128::from(OFFICIAL.inst_bytes);
    if program.list_heads_present {
        remaining -= count * i128::from(OFFICIAL.list_head_bytes);
    }
    remaining = remaining.max(0);
    let charged = program
        .onepass
        .as_ref()
        .ok_or(DfaError::SourceInvariant)?
        .charged_bytes;
    if charged < 0 || i128::from(charged) > remaining {
        return Err(DfaError::SourceInvariant);
    }
    i64::try_from(remaining - i128::from(charged)).map_err(|_| DfaError::AccountingOverflow)
}
